"""📤 Export jobs: explicit profile, profile isolation of inputs, work counters and provenance beside every exported movie.

@see 🧬️schema/🔣️.schema.json — the provenance contract
@see ../🎚️profile/🐍️.py — profiles, fingerprints and jobs
@see ../../../../🔀️morph/📊️instrument/🐍️.py — the `work.json` writer
"""

from __future__ import annotations

import hashlib
import importlib
import json
from pathlib import Path

_PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
_profile = importlib.import_module(_PACKAGE + ".🎚️profile.🐍️")

RECORD_SUFFIX = ".export.json"
RENDER_SUFFIX = ".render.json"
WORK_FILE = "work.json"
FULL_WORK = ("full.prepare", "full.evaluate")
KINDS = ("chapter", "master", "course", "delivery")


class ProfileMismatch(ValueError):
    """🚫 An export input was produced under another render profile than the job asks for."""


class WorkMismatch(ValueError):
    """🚫 The recorded work counters contradict the profile that was exported."""


def poster_policy(video_id: str) -> dict:
    """🎯 The declared canonical poster policy of one lesson, else the default."""
    posters = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))["posters"]
    return dict(posters["videos"].get(video_id, posters["default"]))


def digest(path: Path) -> str:
    """🔏 SHA-256 of a file, read in chunks."""
    sha = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            sha.update(chunk)
    return sha.hexdigest()


def profile_ref(name: str) -> dict:
    """🎚️ Name, fingerprint and execution policy of one profile, as recorded in provenance."""
    chosen = _profile.profile(name)
    return {"name": chosen.name, "fingerprint": chosen.fingerprint, "morph": chosen.morph, "lod": chosen.lod}


def effective_render(name: str, *, resolution: str | None = None, fps: float | None = None, seed: int = 0) -> dict:
    """⚙️ Effective output and effect settings: the profile's own values with explicit overrides applied."""
    chosen = _profile.profile(name)
    width, height = (int(part) for part in resolution.replace("x", ",").split(",")) if resolution else (chosen.width, chosen.height)
    budgets = {"curveSamples": chosen.budgets.curve_samples, "heatmapResolution": chosen.budgets.heatmap_resolution,
               "particles": chosen.budgets.particles, "trails": chosen.budgets.trails}
    return {"width": width, "height": height, "fps": float(fps if fps is not None else chosen.fps), "antialias": chosen.antialias,
            "shadows": chosen.shadows, "budgets": budgets, "resolutionOverride": resolution, "fpsOverride": None if fps is None else float(fps), "seed": int(seed)}


def profiles_in(state) -> set[tuple[str, str | None]]:
    """🔎 Every `(profile name, fingerprint or None)` a nested certificate or sidecar state declares."""
    found = set()
    if isinstance(state, dict):
        settings = state.get("settings")
        if isinstance(settings, dict) and isinstance(settings.get("profile"), str):
            found.add((settings["profile"], state["profile"] if isinstance(state.get("profile"), str) else None))
        for key, value in state.items():
            if key != "sources":
                found |= profiles_in(value)
    elif isinstance(state, list):
        for item in state:
            found |= profiles_in(item)
    return found


def sidecar_state(artifact: Path) -> dict:
    """📄 Read the success-only `.render.json` certificate beside a generated movie."""
    sidecar = Path(artifact).with_suffix(RENDER_SUFFIX)
    if not sidecar.is_file():
        raise ProfileMismatch(f"{artifact} has no render certificate, so its profile is unknown; render it again with an explicit --profile")
    return json.loads(sidecar.read_text(encoding="utf-8"))


def require_state_profile(state: dict, profile: str, *, label: str = "input") -> None:
    """🛑 Refuse a state that is not exactly the requested profile, fingerprint included."""
    fingerprint = _profile.fingerprint(profile)
    found = profiles_in(state)
    if not found:
        raise ProfileMismatch(f"{label} declares no render profile; it cannot be used for a {profile} job")
    foreign = sorted(name + (" (stale fingerprint)" if name == profile else "") for name, seen in found if name != profile or seen not in (None, fingerprint))
    if foreign:
        raise ProfileMismatch(f"{label} was produced under {', '.join(foreign)} but this is a {profile} job; "
                              f"a {profile} export never consumes other profiles' partials or outputs. Render it with --profile {profile}")


def require_profile(artifact: Path, profile: str) -> None:
    """🛑 Refuse a movie, partial or still whose certificate belongs to another profile."""
    require_state_profile(sidecar_state(Path(artifact)), profile, label=str(artifact))


def require_profiles(artifacts, profile: str) -> None:
    """🛑 Apply `require_profile` to every input of a join, excerpt or presentation."""
    for artifact in artifacts:
        require_profile(Path(artifact), profile)


def read_work(path: Path) -> dict:
    """📊 Read one `work.json` written by the scene process."""
    record = json.loads(Path(path).read_text(encoding="utf-8"))
    if not isinstance(record, dict) or not {"job", "profile", "fingerprint", "counts"} <= set(record) or not isinstance(record["counts"], dict):
        raise WorkMismatch(f"{path} is not a work record")
    return record


def collect_work(directories, profile: str) -> dict:
    """➕ Sum the work of every scene output; a foreign profile or fingerprint is a contamination error."""
    expected = _profile.fingerprint(profile)
    total, sources, jobs = {}, [], set()
    for directory in dict.fromkeys(Path(item) for item in directories):
        path = directory / WORK_FILE
        if not path.is_file():
            raise WorkMismatch(f"Missing {WORK_FILE} in {directory}; the scene did not report its work")
        record = read_work(path)
        if record["profile"] != profile or record["fingerprint"] != expected:
            raise ProfileMismatch(f"{path} reports profile {record['profile']} but this is a {profile} job")
        jobs.add(record["job"])
        for name, value in record["counts"].items():
            total[name] = total.get(name, 0) + int(value)
        sources.append({"file": path.as_posix(), "job": record["job"], "counts": {key: int(value) for key, value in record["counts"].items()}})
    return {"jobs": len(jobs), "counts": dict(sorted(total.items())), "sources": sources}


def declared_transitions(script: Path) -> int:
    """🔀 Number of declarative transitions of the clip folder that owns `script`."""
    document = Path(script).resolve().parent / "🔀️transitions" / "🔣️.json"
    if not document.is_file():
        return 0
    return len(json.loads(document.read_text(encoding="utf-8")).get("transitions", []))


def verify_work(profile: str, work: dict, *, transitions: int) -> None:
    """⚖️ Preview jobs did zero full-morph work; final jobs that declare transitions did real full-morph work."""
    counts = work["counts"]
    if _profile.profile(profile).morph == "swap":
        if any(counts.get(name, 0) for name in FULL_WORK):
            raise WorkMismatch(f"{profile} job performed full-morph work: " + ", ".join(f"{name}={counts.get(name, 0)}" for name in FULL_WORK))
    elif transitions and not all(counts.get(name, 0) > 0 for name in FULL_WORK):
        raise WorkMismatch(f"{profile} job declares {transitions} transition(s) but ran no full morph: " + ", ".join(f"{name}={counts.get(name, 0)}" for name in FULL_WORK))


def export_record(kind: str, artifact: Path, *, profile: str, language: str, audience: str, work: dict, inputs=(),
                  resolution: str | None = None, fps: float | None = None, seed: int = 0, stills: dict | None = None) -> dict:
    """🧾 Provenance of one export: effective profile, fingerprint, render settings, work counters and the inputs it consumed."""
    if kind not in KINDS:
        raise ValueError(f"Unknown export kind {kind}; expected one of {list(KINDS)}")
    artifact = Path(artifact)
    record = {"version": 1, "kind": kind, "artifact": {"file": artifact.name, "sha256": digest(artifact), "bytes": artifact.stat().st_size},
              "language": language, "audience": audience, "profile": profile_ref(profile),
              "render": effective_render(profile, resolution=resolution, fps=fps, seed=seed), "work": work,
              "inputs": [{"file": Path(item).name, "profile": profile, "fingerprint": _profile.fingerprint(profile)} for item in inputs]}
    if stills:
        record["stills"] = stills
    return record


def publish_record(artifact: Path, record: dict) -> Path:
    """💾 Atomically publish `<movie>.export.json` after the record validates."""
    problems = validate_record(record)
    if problems:
        raise ValueError("Invalid export record: " + "; ".join(problems))
    target = Path(artifact).with_suffix(RECORD_SUFFIX)
    temporary = target.with_suffix(target.suffix + ".tmp")
    temporary.write_text(json.dumps(record, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    temporary.replace(target)
    return target


def validate_record(record: dict) -> list[str]:
    """✅ Structural validation of a record against the schema's required shape and the profile registry."""
    problems = []
    required = ("version", "kind", "artifact", "language", "audience", "profile", "render", "work", "inputs")
    for key in required:
        if key not in record:
            problems.append(f"missing {key}")
    if problems:
        return problems
    if record["version"] != 1 or record["kind"] not in KINDS:
        problems.append("unknown version or kind")
    reference = record["profile"]
    try:
        current = profile_ref(reference.get("name", ""))
    except _profile.ProfileError as error:
        return [str(error)]
    if reference != current:
        problems.append(f"profile reference differs from the registry entry {current['name']} (stale fingerprint)")
    if not isinstance(record["inputs"], list) or any(item.get("profile") != reference["name"] or item.get("fingerprint") != reference["fingerprint"] for item in record["inputs"]):
        problems.append("inputs must share the exported profile and fingerprint")
    if not isinstance(record["work"], dict) or not {"jobs", "counts", "sources"} <= set(record["work"]):
        problems.append("work needs jobs, counts and sources")
    return problems


def write_work(movie: Path, work: dict, profile: str) -> Path:
    """💾 Keep the summed work of the scenes behind an assembled movie next to it, so reuse never needs the scene media again."""
    target = Path(movie).with_suffix(".work.json")
    temporary = target.with_suffix(".json.tmp")
    temporary.write_text(json.dumps({"profile": profile, "fingerprint": _profile.fingerprint(profile), **work}, indent=2) + "\n", encoding="utf-8")
    temporary.replace(target)
    return target


def read_assembled_work(movie: Path, profile: str) -> dict:
    """📖 The retained work of an assembled movie; another profile or a missing file is an error."""
    path = Path(movie).with_suffix(".work.json")
    if not path.is_file():
        raise WorkMismatch(f"Missing {path.name}; assemble {movie} again so its scenes' work is recorded")
    record = json.loads(path.read_text(encoding="utf-8"))
    if record.get("profile") != profile or record.get("fingerprint") != _profile.fingerprint(profile):
        raise ProfileMismatch(f"{path} reports profile {record.get('profile')} but this is a {profile} job")
    return {key: record[key] for key in ("jobs", "counts", "sources")}


def merge_work(items) -> dict:
    """➕ Sum several work records."""
    counts, sources, jobs = {}, [], 0
    for item in items:
        jobs += item["jobs"]
        sources.extend(item["sources"])
        for name, value in item["counts"].items():
            counts[name] = counts.get(name, 0) + int(value)
    return {"jobs": jobs, "counts": dict(sorted(counts.items())), "sources": sources}


def publish_export(kind: str, artifact: Path, *, settings: dict, movies, scripts, inputs=(), stills: dict | None = None, windows: dict | None = None, scene_clips: dict | None = None) -> dict:
    """📦 Verify the retained work of every assembled input against the profile, then publish provenance (and, for `stills`, the poster and thumbnail) beside the movie.

    `movies` are the assembled chapter clips whose `.work.json` is summed, `scripts` the scene scripts they came from; `stills` is `{"folder": Path, "name": str, "policy": dict | None}`; a policy with a `scene` takes the poster from that scene clip (`scene_clips`) on its own clock.
    """
    profile = settings["profile"]
    work = merge_work(read_assembled_work(movie, profile) for movie in movies)
    verify_work(profile, work, transitions=sum(declared_transitions(script) for script in dict.fromkeys(Path(item) for item in scripts)))
    made = None
    if stills:
        module = importlib.import_module(_PACKAGE + ".📤️export.🖼️stills.🐍️")
        render = effective_render(profile, resolution=settings.get("resolution"))
        policy, source = stills.get("policy"), artifact
        if policy and policy.get("scene"):
            source = Path((scene_clips or {}).get(policy["scene"], ""))
            if not source.is_file():
                raise ValueError(f"The poster policy names scene {policy['scene']} but its clip is not retained; render that scene again under the {profile} profile")
            windows = module.read_transitions(source.parent.parent)
        made = module.make_stills(source, stills["folder"], stills["name"], profile=profile, policy=policy, windows=windows, size=(render["width"], render["height"]))
    record = export_record(kind, artifact, profile=profile, language=settings["language"], audience=settings["audience"], work=work, inputs=inputs,
                           resolution=settings.get("resolution"), fps=settings.get("fps"), stills=made)
    publish_record(artifact, record)
    return record


def read_record(artifact: Path) -> dict:
    """📖 Read and validate the provenance of an exported movie."""
    path = Path(artifact).with_suffix(RECORD_SUFFIX)
    if not path.is_file():
        raise ProfileMismatch(f"{artifact} has no {RECORD_SUFFIX} provenance; export it again")
    record = json.loads(path.read_text(encoding="utf-8"))
    problems = validate_record(record)
    if problems:
        raise ValueError(f"Invalid export record {path}: " + "; ".join(problems))
    if record["artifact"]["sha256"] != digest(Path(artifact)):
        raise ProfileMismatch(f"{artifact} changed after its provenance was written")
    return record


def require_record_profile(artifact: Path, profile: str) -> dict:
    """🛑 Read provenance and refuse an artifact exported under another profile."""
    record = read_record(artifact)
    if record["profile"] != profile_ref(profile):
        raise ProfileMismatch(f"{artifact} is a {record['profile']['name']} export but this is a {profile} job")
    return record
