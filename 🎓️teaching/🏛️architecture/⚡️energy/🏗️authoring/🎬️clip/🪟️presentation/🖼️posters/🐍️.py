"""🖼️ Poster frames of the teaching overview: every card shows a stored, card-sized frame of its authored chapter, never a placeholder and never a render of a foreign lesson.

A chapter's poster is the end state of its last segment (the summary state), or the authored `poster` of the manifest chapter (`segment`, `animation`). The asset is a 640x360 PNG with a fingerprint sidecar, stored beside the lesson's clip sources, one folder per language (`<lesson clip folder>/🖼️posters/<language>/<chapter>.png|json`), and checked into the repository. A poster never crosses languages: a render in one language reads only that language's posters.
The overview reads only these assets: a missing one (also one not yet captured in the render language) fails loudly, a stale one (an authored source changed since the capture) is listed in one warning line and used as is.
Posters are captured as a by-product: rendering a chapter takes its poster from the last frame of the segment clip it produced anyway. `script.ts posters` (re)captures them from an up-to-date clip of the output root, else by one single Manim frame of the scene.
A lesson card shows the poster of its designated chapter (the manifest lesson's `poster.chapter`, else its first chapter); topic and course cards show the first poster below them; a topic without lessons shows an explicit empty card.

@see ../../🧬️schema/🔣️.schema.json — `poster` of a lesson and of a chapter
@see https://docs.manim.community/en/stable/guides/configuration.html — `--save_last_frame` and `--from_animation_number`
"""

from __future__ import annotations

import hashlib
import importlib
import json
import shutil
import tempfile
import time
from pathlib import Path

_PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
_CHILDREN = ("topics", "videos", "chapters")
FOLDER = "🖼️posters"
SIZE = (640, 360)
CONTENT_AREA = (0.05, 0.15, 0.95, 0.85)
MINIMUM_COVERAGE = 0.03
BACKGROUND_TOLERANCE = 24


def _runtime():
    """🔌 Resolve the render runtime lazily, because it imports this package's hierarchy."""
    return importlib.import_module(_PACKAGE + ".⚙️runtime.🐍️")


def _hierarchy():
    """🥞 Resolve the hierarchy port lazily."""
    return importlib.import_module(_PACKAGE + ".🪟️presentation.🥞️hierarchy.🐍️")


def designated(video: dict) -> str:
    """🎯 The chapter whose poster a lesson card shows: the authored `poster.chapter`, else the first chapter."""
    scenes = [chapter["scene"] for chapter in video["chapters"]]
    chosen = video.get("poster", {}).get("chapter", scenes[0])
    if chosen not in scenes:
        raise ValueError(f"The poster chapter {chosen} does not belong to lesson {video['id']}")
    return chosen


def declaration(chapter: dict) -> dict:
    """📍 The authored moment of one chapter poster: a segment index and an optional Manim animation number up to which the scene runs."""
    segments = chapter["segments"]
    authored = chapter.get("poster", {})
    index = authored.get("segment", len(segments) - 1)
    animation = authored.get("animation")
    if not 0 <= index < len(segments):
        raise ValueError(f"The poster segment {index} of chapter {chapter['scene']} does not exist")
    if "clauses" in segments[index] and animation is None:
        raise ValueError(f"The poster of chapter {chapter['scene']} lies in a partial segment; declare the `animation` of its poster")
    return {"segment": index, "animation": animation}


def chapters() -> dict[str, dict]:
    """📚 Every authored chapter by scene id with its lesson, manifest position, poster declaration, poster segment and asset folder."""
    runtime, result = _runtime(), {}
    teaching = runtime.teaching_root(Path(runtime.__file__))
    for video in runtime.load_manifest(source_ids=[]):
        for position, chapter in enumerate(video["chapters"]):
            moment = declaration(chapter)
            if "Fahrplan" in chapter["segments"][moment["segment"]]["scene"]:
                raise ValueError(f"The poster of chapter {chapter['scene']} would be the roadmap; declare another `poster` segment")
            result[chapter["scene"]] = {"video": video, "position": position, "ordinal": len(result) + 1, "declaration": moment,
                                        "segment": {key: chapter["segments"][moment["segment"]][key] for key in ("script", "scene")},
                                        "folder": (teaching / video["script"]).parent / FOLDER}
    return result


def required(branch: list[str]) -> list[str]:
    """📋 The chapter scenes whose posters the overview of one branch shows: every card of the course, topic, lesson and chapter grid on it."""
    entries, scenes = _hierarchy().hierarchy_contract()["courses"], []
    for depth in range(4):
        scenes.extend(entry["poster"] for entry in entries if entry["poster"] is not None)
        if depth >= len(branch):
            break
        found = next((entry for entry in entries if entry["id"] == branch[depth]), None)
        if found is None:
            raise ValueError(f"Unknown hierarchy branch {'/'.join(branch)}")
        if depth < 3:
            entries = found[_CHILDREN[depth]]
    return list(dict.fromkeys(scenes))


def _relative(path: str | Path) -> str:
    """🧭 Name a source relative to the workspace, so a stored sidecar does not depend on the machine that captured it."""
    root = _runtime()._WORKSPACE_ROOT
    resolved = Path(path)
    return resolved.relative_to(root).as_posix() if resolved.is_relative_to(root) else resolved.as_posix()


def sources(scene: str, settings: dict) -> dict[str, str]:
    """🧬 The authored source fingerprints a poster of this chapter depends on: every file of its poster segment's dependency closure."""
    state = _runtime().segment_state(chapters()[scene]["segment"], settings)
    return {_relative(path): digest for path, digest in state["sources"].items()}


def _digest(path: Path) -> str:
    """🔏 SHA-256 of one file."""
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def asset(scene: str, language: str) -> tuple[Path, Path]:
    """📦 The stored poster image and its fingerprint sidecar of one render language."""
    folder = chapters()[scene]["folder"] / language
    return folder / f"{scene}.png", folder / f"{scene}.json"


def stored(scene: str, language: str) -> dict | None:
    """📄 The sidecar of a stored poster of this language whose image is intact, or `None` when it is missing or damaged."""
    image, sidecar = asset(scene, language)
    try:
        record = json.loads(sidecar.read_text(encoding="utf-8"))
        return record if image.is_file() and record["sha256"] == _digest(image) else None
    except (OSError, ValueError, KeyError):
        return None


def stale(scene: str, language: str) -> bool:
    """⌛ A poster is stale when none is stored for this language, its moment changed, or any authored source it was captured from changed or vanished."""
    record, root = stored(scene, language), _runtime()._WORKSPACE_ROOT
    if record is None or record.get("moment") != chapters()[scene]["declaration"]:
        return True
    try:
        return any(_digest(root / path) != digest for path, digest in record["sources"].items())
    except OSError:
        return True


def coverage(image) -> float:
    """🫥 The share of non-background pixels inside the content area (title chips, references and captions excluded); the background is the most frequent colour of that area."""
    import numpy as np

    pixels = np.asarray(image.convert("RGB"), dtype=int)
    height, width = pixels.shape[:2]
    left, top, right, bottom = CONTENT_AREA
    flat = pixels[int(height * top):int(height * bottom), int(width * left):int(width * right)].reshape(-1, 3)
    keys, counts = np.unique(flat // 8, axis=0, return_counts=True)
    background = keys[counts.argmax()] * 8 + 4
    return float((np.abs(flat - background).max(axis=1) > BACKGROUND_TOLERANCE).mean())


def store(scene: str, frame: Path, settings: dict, *, sources_state: dict[str, str] | None = None) -> Path:
    """💾 Atomically publish a captured frame as the card-sized poster asset and its sidecar in the language of `settings`."""
    from PIL import Image

    runtime = _runtime()
    image, sidecar = asset(scene, settings["language"])
    image.parent.mkdir(parents=True, exist_ok=True)
    temporary = image.with_name(f".{scene}.{time.time_ns():x}.png")
    try:
        with Image.open(frame) as source:
            card = source.convert("RGB").resize(SIZE, Image.Resampling.LANCZOS)
        share = coverage(card)
        if share < MINIMUM_COVERAGE:
            raise ValueError(f"The poster of chapter {scene} is almost empty ({share:.1%} content, at least {MINIMUM_COVERAGE:.1%} required); declare a `poster` moment (segment, animation) in the manifest that shows the chapter's key picture")
        card.save(temporary, "PNG", optimize=True)
        sidecar.unlink(missing_ok=True)
        temporary.replace(image)
    finally:
        temporary.unlink(missing_ok=True)
    runtime._write_state(sidecar, {"scene": scene, "moment": chapters()[scene]["declaration"], "sources": sources_state or sources(scene, settings),
                                   "captured": {key: settings.get(key) for key in ("profile", "audience")}, "size": list(SIZE), "sha256": _digest(image)})
    return image


def last_frame(clip: Path, output: Path) -> Path:
    """📸 The exact last scheduled frame of a clip, taken from the frames a render produced anyway."""
    stills = importlib.import_module(_PACKAGE + ".📤️export.🖼️stills.🐍️")
    frames, _ = stills.probe(Path(clip))
    return stills.extract(Path(clip), frames - 1, output)


def capture_clip(scene: str, clip: Path, settings: dict) -> Path:
    """🎞️ Take the poster from the last frame of an up-to-date poster-segment clip: zero extra scene renders."""
    if chapters()[scene]["declaration"]["animation"] is not None:
        raise ValueError(f"The poster of {scene} stops after an animation; render its single frame instead")
    with tempfile.TemporaryDirectory() as directory:
        return store(scene, last_frame(clip, Path(directory) / "frame.png"), settings)


def render_frame(scene: str, settings: dict, work: Path, session) -> Path:
    """📸 Render one poster frame as a single Manim frame of its authored moment (`--save_last_frame`) and store it; never a lesson."""
    from PIL import Image

    runtime = _runtime()
    profile = importlib.import_module(_PACKAGE + ".🎚️profile.🐍️")
    info, work = chapters()[scene], Path(work).resolve()
    teaching = runtime.teaching_root(Path(runtime.__file__))
    segment, animation = info["segment"], info["declaration"]["animation"]
    before = sources(scene, settings)
    media = work / "m" / str(info["ordinal"])
    shutil.rmtree(media / "i", ignore_errors=True)
    chosen = profile.profile(settings["profile"])
    resolution = (settings.get("resolution") or f"{chosen.width},{chosen.height}").replace("x", ",")
    width, height = (int(part) for part in resolution.split(","))
    fps = settings.get("fps") or chosen.fps
    command = [*runtime.manim_command(), "--config_file", str(runtime.render_config(media)), "--media_dir", str(media), "--resolution", resolution, "--fps", f"{fps:g}",
               "--disable_caching", "--save_last_frame", *([] if animation is None else ["--from_animation_number", f"0,{animation}"]),
               str(teaching / segment["script"]), segment["scene"]]
    environment = {"SEMIO_TEACHING_OUTPUT_ROOT": str(work),
                   "SEMIO_TEACHING_SPEECH_ROOT": str(runtime.source_speech_root(teaching / segment["script"], language=settings["language"]).parent),
                   "SEMIO_TEACHING_LANGUAGE": settings["language"], "SEMIO_TEACHING_AUDIENCE": settings["audience"],
                   **profile.environment(settings["profile"], job_id=f"{settings['profile']}-poster-{scene}-{time.time_ns():x}", fps=fps, resolution=resolution),
                   "SEMIO_TEACHING_WORK": str(media / "work.json")}
    runtime.run_command(command, cwd=teaching, env=environment, session=session)
    images = sorted((media / "i").glob("*.png"))
    if len(images) != 1:
        raise FileNotFoundError(f"Manim did not create exactly one poster frame for {scene}: {len(images)} in {media / 'i'}")
    with Image.open(images[0]) as image:
        if image.size != (width, height):
            raise RuntimeError(f"The poster of {scene} is {image.size[0]}x{image.size[1]} but this render is {width}x{height}")
    if sources(scene, settings) != before:
        raise RuntimeError(f"Authored sources changed while rendering the poster of {scene}")
    if session.cancelled.is_set():
        raise InterruptedError("Poster rendering cancelled")
    published = store(scene, images[0], settings, sources_state=before)
    shutil.rmtree(media / "i", ignore_errors=True)
    return published


def _segment_clip(scene: str, settings: dict, output_root: Path) -> Path | None:
    """🔎 The up-to-date source clip of a chapter's poster segment in one output root, if a render left it there."""
    runtime, info = _runtime(), chapters()[scene]
    job = next(job for job in runtime.segment_plan() if job["video_id"] == info["video"]["id"] and job["chapter_scene"] == scene and job["segment"]["scene"] == info["segment"]["scene"] and job["segment"]["script"] == info["segment"]["script"])
    clip = Path(output_root) / "s" / str(job["source_ordinal"]) / "v" / f"{info['segment']['scene']}.mp4"
    return clip if runtime.reusable_clip(clip, runtime.segment_state(info["segment"], settings)) else None


def capture(scenes, settings: dict, output_root: Path, *, jobs: int = 1, force: bool = False) -> dict[str, str]:
    """📦 (Re)capture the named posters with progress and cancellation: from an up-to-date clip of the output root, else by one single-frame render; returns `{scene: how}`."""
    runtime, known = _runtime(), chapters()
    scenes = list(dict.fromkeys(scenes))
    if any(scene not in known for scene in scenes):
        raise ValueError("Unknown poster chapters: " + ", ".join(scene for scene in scenes if scene not in known))
    root, how = Path(output_root).resolve(), {}

    def work(scene, session):
        if not force and not stale(scene, settings["language"]):
            how[scene] = "current"
            return
        clip = _segment_clip(scene, settings, root) if known[scene]["declaration"]["animation"] is None else None
        print(f"[poster] {'Taking the last frame of the rendered clip for' if clip else 'Rendering one frame of'} {scene} ({len(how) + 1}/{len(scenes)})", flush=True)
        if clip:
            capture_clip(scene, clip, settings)
        else:
            render_frame(scene, settings, root / "posters", session)
        how[scene] = "clip" if clip else "frame"
        print(f"[poster] Stored {scene}", flush=True)

    _, failures = runtime.execute_jobs(scenes, work, max_workers=jobs)
    if failures:
        raise RuntimeError("Poster failures:\n" + "\n".join(f"{scenes[index]}: {error}" for index, error in failures.items()))
    return how


def resolve(scenes, language: str) -> dict[str, dict]:
    """📚 Read only the stored assets of one language: a missing one fails loudly, stale ones are listed once and used as is; returns `{scene: {file, sha256}}`."""
    scenes = list(dict.fromkeys(scenes))
    known = chapters()
    missing = [scene for scene in scenes if scene not in known or stored(scene, language) is None]
    if missing:
        raise FileNotFoundError(f"Missing stored chapter posters (capture them with `script.ts posters --language {language} --profile final`): " + ", ".join(missing))
    from PIL import Image

    empty = []
    for scene in scenes:
        with Image.open(asset(scene, language)[0]) as image:
            if coverage(image) < MINIMUM_COVERAGE:
                empty.append(scene)
    if empty:
        raise ValueError("Almost empty stored chapter posters; declare a `poster` moment (segment, animation) for: " + ", ".join(empty))
    outdated = [scene for scene in scenes if stale(scene, language)]
    if outdated:
        print("[poster] Warning: stale stored posters, not re-rendered: " + ", ".join(outdated), flush=True)
    return {scene: {"file": str(asset(scene, language)[0]), "sha256": stored(scene, language)["sha256"]} for scene in scenes}


def prepare(rendered, settings: dict) -> dict[str, dict]:
    """🧩 Refresh the posters of the chapters this render produced from their segment clips, then read every poster their overviews show."""
    hierarchy, info = _hierarchy(), chapters()
    for scene, clips in rendered:
        moment = info[scene]["declaration"]
        if moment["animation"] is None and clips and Path(clips[moment["segment"]]).is_file() and stale(scene, settings["language"]):
            capture_clip(scene, clips[moment["segment"]], settings)
    return resolve((shown for scene, _ in rendered for shown in required(hierarchy.path(scene))), settings["language"])
