"""🎞️ Course-level exports of one profile: chaptered preview/final joiner and per-lesson delivery.

Every input is a lesson master with `.export.json` provenance of exactly the requested profile whose authored sources are still current; a master of the other profile, a stale one or one without provenance stops the export.

@see ../🐍️.py — provenance, profile guards and work counters
@see ../🖼️stills/🐍️.py — poster and thumbnail of the joined movie
"""

from __future__ import annotations

import importlib
import json
import tempfile
from pathlib import Path

_PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
_export = importlib.import_module(_PACKAGE + ".📤️export.🐍️")
_stills = importlib.import_module(_PACKAGE + ".📤️export.🖼️stills.🐍️")
_profile = importlib.import_module(_PACKAGE + ".🎚️profile.🐍️")
_runtime = importlib.import_module(_PACKAGE + ".⚙️runtime.🐍️")
_output = importlib.import_module(_PACKAGE + ".📁️output.🐍️")
_hierarchy = importlib.import_module(_PACKAGE + ".🪟️presentation.🥞️hierarchy.🐍️")

COURSE = "Architecture_course"
SCHEMA = "semio.teaching.delivery/v2"


def titles(videos: list[dict], language: str) -> dict[str, str]:
    """🏷️ Localized lesson titles from the canonical presentation hierarchy; missing, duplicate or empty titles are errors."""
    if language not in ("en", "de"):
        raise ValueError(f"unsupported language: {language}")
    entries = [(topic, video) for topic in _hierarchy.hierarchy_contract()["topics"] for video in topic["videos"]]
    result = {}
    for video in videos:
        found = [(topic, entry) for topic, entry in entries if entry["id"] == video["id"]]
        if len(found) != 1 or any(set(item.get("title", {})) != {"en", "de"} or any(not isinstance(value, str) or not value.strip() for value in item["title"].values()) for item in found[0]):
            raise ValueError(f"missing, duplicate or invalid localized title: {video['id']}")
        topic, entry = found[0]
        heading, label = topic["title"][language], entry["title"][language]
        result[video["id"]] = label if heading.casefold() in label.casefold() else f"{heading} · {label}"
    if len({value.casefold() for value in result.values()}) != len(result):
        raise ValueError(f"duplicate localized chapter titles: {language}")
    return result


def metadata_text(value: str) -> str:
    """🧾 Escape one FFMETADATA value."""
    if not isinstance(value, str) or not value.strip():
        raise ValueError("empty chapter title")
    return value.replace("\\", "\\\\").replace("=", "\\=").replace(";", "\\;").replace("#", "\\#").replace("\n", "\\\n").replace("\r", "\\\r")


def folder(language: str, audience: str, profile: str, output_root: Path | None = None) -> Path:
    """📁 The generated-media folder of one profile, language and audience."""
    return Path(output_root or _output.output_root()).resolve() / profile / language / audience


def master_path(video: dict, language: str, audience: str, profile: str, record_render: dict | None = None, output_root: Path | None = None) -> Path:
    """🎬 Where `compose_videos` publishes the lesson master of one profile."""
    chosen = _profile.profile(profile)
    resolution = (record_render or {}).get("resolutionOverride")
    height = resolution.replace("x", ",").split(",")[1] if resolution else str(chosen.height)
    rate = (record_render or {}).get("fpsOverride") or chosen.fps
    return folder(language, audience, profile, output_root) / "rendered" / f"{video['output']}_{audience}_{language}_{height}p{rate:g}.mp4"


def current_master(video: dict, master: Path, language: str, audience: str, profile: str) -> dict:
    """♻️ The provenance of a lesson master, after proving its profile and that every authored source is still current."""
    record = _export.require_record_profile(master, profile)
    saved = _export.sidecar_state(master)
    _export.require_state_profile(saved, profile, label=str(master))
    settings = {"language": language, "profile": profile, "resolution": record["render"]["resolutionOverride"], "fps": record["render"]["fpsOverride"], "audience": audience}
    expected = [_runtime.segment_state(job["segment"], settings) for job in _runtime.segment_plan() if job["video_id"] == video["id"]]
    recorded = [segment for chapter in saved["content"]["chapters"] for segment in chapter["segments"]]
    if recorded != expected or saved.get("presentation") != _runtime.presentation_state(video["id"], settings, language) or not _runtime.state_inputs_current(saved):
        raise ValueError(f"{master} is stale: its authored sources changed after the {profile} export; render it again with --profile {profile}")
    return record


def current_masters(language: str, audience: str, profile: str, output_root: Path | None = None) -> list[tuple[dict, Path, dict]]:
    """📚 Every lesson master of the curriculum, in order, or one error naming all missing or stale ones."""
    parts, problems = [], []
    for video in _runtime.load_manifest():
        base = master_path(video, language, audience, profile, None, output_root)
        try:
            parts.append((video, base, current_master(video, base, language, audience, profile)))
        except (OSError, ValueError, KeyError) as error:
            problems.append(f"{video['id']}: {error}")
    if problems:
        raise ValueError(f"stale, missing or foreign lesson masters for {profile}:\n" + "\n".join(problems))
    return parts


def normalize(source: Path, target: Path, profile: str) -> float:
    """🧱 Re-encode one master to the profile's frame size and rate with stereo 48 kHz audio; returns its seconds."""
    render = _export.effective_render(profile)
    seconds = _runtime.clip_duration(source)
    with tempfile.TemporaryDirectory(prefix="join-") as directory:
        log = Path(directory) / "audio.log"
        _runtime.run_command([*_runtime.ffmpeg_command(), "-hide_banner", "-nostdin", "-i", str(source), "-t", "0.001", "-f", "null", "-"], log=log)
        has_audio = "Audio:" in log.read_text(encoding="utf-8")
    command = [*_runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i", str(source)]
    if not has_audio:
        command += ["-f", "lavfi", "-i", "anullsrc=channel_layout=stereo:sample_rate=48000"]
    command += ["-map", "0:v:0", "-map", "0:a:0" if has_audio else "1:a:0",
                "-vf", f"scale={render['width']}:{render['height']}:force_original_aspect_ratio=decrease,pad={render['width']}:{render['height']}:(ow-iw)/2:(oh-ih)/2,fps={render['fps']:g},format=yuv420p",
                "-af", "aresample=48000,apad", "-ac", "2", "-ar", "48000", "-c:a", "aac", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-g", "30",
                "-t", f"{seconds:.9f}", str(target)]
    _runtime.run_command(command)
    return _runtime.clip_duration(target)


def join_course(language: str, audience: str, profile: str, *, output_root: Path | None = None, work_root: Path | None = None) -> Path:
    """🌍 Join every current lesson master of `profile` into one chaptered movie; publish only after the masters stayed current."""
    videos = _runtime.load_manifest()
    names = titles(videos, language)
    parts = current_masters(language, audience, profile, output_root)
    render = _export.effective_render(profile)
    destination = folder(language, audience, profile, output_root) / "course"
    destination.mkdir(parents=True, exist_ok=True)
    output = destination / f"{COURSE}_{audience}_{language}_{render['height']}p{render['fps']:g}.mp4"
    with tempfile.TemporaryDirectory(prefix="course-", dir=work_root) as temporary:
        work = Path(temporary)
        normalized = []
        for index, (video, master, _) in enumerate(parts):
            target = work / f"{index:02d}-{video['id']}.mp4"
            normalized.append((video, target, normalize(master, target, profile)))
        listing = work / "list.txt"
        listing.write_text("".join(_runtime.concat_entry(target) + "\n" for _, target, _ in normalized), encoding="utf-8")
        chapters, start = [";FFMETADATA1\n"], 0.0
        for video, _, seconds in normalized:
            chapters.append(f"[CHAPTER]\nTIMEBASE=1/1000\nSTART={round(start * 1000)}\nEND={round((start + seconds) * 1000)}\ntitle={metadata_text(names[video['id']])}\n")
            start += seconds
        (work / "chapters.txt").write_text("".join(chapters), encoding="utf-8")
        partial = destination / f".{output.stem}.partial.mp4"
        try:
            _runtime.run_command([*_runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-f", "concat", "-safe", "0", "-i", str(listing),
                                  "-i", str(work / "chapters.txt"), "-map", "0:v:0", "-map", "0:a:0", "-map_metadata", "1", "-map_chapters", "1", "-c", "copy",
                                  "-movflags", "+faststart", str(partial)])
            current_masters(language, audience, profile, output_root)
            partial.replace(output)
        finally:
            partial.unlink(missing_ok=True)
    certificate = {"settings": {"language": language, "profile": profile, "audience": audience, "resolution": None, "fps": None}, "profile": _profile.fingerprint(profile),
                   "lessons": [{"id": video["id"], "title": names[video["id"]], "master": master.name, "sha256": record["artifact"]["sha256"], "seconds": round(seconds, 3)}
                               for (video, master, record), (_, _, seconds) in zip(parts, normalized, strict=True)], "sources": {}}
    _runtime._write_state(output.with_suffix(".render.json"), certificate)
    made = _stills.make_stills(output, destination / "posters", output.stem, profile=profile, policy=_export.poster_policy("course"))
    record = _export.export_record("course", output, profile=profile, language=language, audience=audience, work=_export.merge_work(item["work"] for _, _, item in parts),
                                   inputs=[master for _, master, _ in parts], stills=made)
    _export.publish_record(output, record)
    return output


def deliver(language: str, audience: str, profile: str, *, output_root: Path | None = None) -> Path:
    """🚚 Re-encode every current lesson master of `profile` for delivery and publish `delivery.json` naming the profile of each file."""
    parts = current_masters(language, audience, profile, output_root)
    render = _export.effective_render(profile)
    destination = folder(language, audience, profile, output_root) / "delivery"
    destination.mkdir(parents=True, exist_ok=True)
    entries = []
    for video, master, record in parts:
        target = destination / master.name
        partial = target.with_name(f".{target.stem}.partial.mp4")
        try:
            _runtime.run_command([*_runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i", str(master), "-map", "0:v:0", "-map", "0:a?",
                                  "-c:v", "libx264", "-preset", "fast", "-crf", "18", "-pix_fmt", "yuv420p", "-c:a", "copy", "-movflags", "+faststart", str(partial)])
            partial.replace(target)
        finally:
            partial.unlink(missing_ok=True)
        _runtime._write_state(target.with_suffix(".render.json"), {"settings": {"language": language, "profile": profile, "audience": audience, "resolution": None, "fps": None},
                                                                    "profile": _profile.fingerprint(profile), "master": master.name, "sources": {}})
        provenance = _export.export_record("delivery", target, profile=profile, language=language, audience=audience, work=record["work"], inputs=[master])
        _export.publish_record(target, provenance)
        entries.append({"id": video["id"], "file": target.name, "sha256": provenance["artifact"]["sha256"], "bytes": provenance["artifact"]["bytes"],
                        "master": f"../rendered/{master.name}", "masterSha256": record["artifact"]["sha256"]})
    document = {"schema": SCHEMA, "language": language, "audience": audience, "profile": _export.profile_ref(profile), "width": render["width"], "height": render["height"],
                "fps": render["fps"], "videos": entries}
    target = destination / "delivery.json"
    target.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return target
