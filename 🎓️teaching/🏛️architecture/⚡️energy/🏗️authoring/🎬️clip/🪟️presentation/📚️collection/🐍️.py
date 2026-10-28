"""📚 Resolve authored chapter metadata independently of source rendering."""
from __future__ import annotations

import copy
import importlib
import math
import re
import time
import uuid
import json
from pathlib import Path

_ROOT = Path(__file__).resolve().parent
_TEACHING = next(path for path in _ROOT.parents if path.name == "🎓️teaching")


def collection_contract() -> dict:
    """🧾 Read explicitly localized chapter identities and curriculum references."""
    return json.loads((_ROOT / "🔣️.json").read_text(encoding="utf-8"))


def enrich(catalog: dict) -> dict:
    """🌳 Join authored chapters and the real course to existing topic and clip cards."""
    result = copy.deepcopy(catalog)
    contract = collection_contract()
    course = json.loads((_TEACHING / contract["course"]).read_text(encoding="utf-8"))
    manifest = json.loads((_TEACHING / contract["manifest"]).read_text(encoding="utf-8"))
    chapters = {video["id"]: video["chapters"] for video in manifest["videos"]}
    declared = {entry["scene"] for entries in chapters.values() for entry in entries}
    if declared != set(contract["chapters"]):
        raise ValueError("Chapter navigation metadata must match the entire authored curriculum")
    actual = set()
    for topic in result["topics"]:
        for clip in topic["videos"]:
            actual.add(clip["id"])
            clip["chapters"] = [{"id": entry["scene"], "title": contract["chapters"][entry["scene"]],
                "icon": clip["icon"], "symbol": clip["symbol"], "cell": {"column": index % 3, "row": index // 3}}
                for index, entry in enumerate(chapters[clip["id"]])]
    if actual != set(chapters):
        raise ValueError("Clip navigation must cover every authored lesson exactly once")
    result["labels"].update(contract["labels"])
    result["courses"] = [{"id": course["id"], "title": course["title"], "icon": "🏛️",
        "symbol": "bim", "cell": {"column": 0, "row": 0}, "topics": copy.deepcopy(result["topics"])}]
    return result


def source_files() -> tuple[Path, ...]:
    """🧬 Bind chapter navigation to its schema, labels, manifest and actual course."""
    contract = collection_contract()
    return (Path(__file__), _ROOT / "🔣️.json", _ROOT / "🧬️schema.json",
        _TEACHING / contract["course"], _TEACHING / contract["manifest"])


def _ports():
    """🔌 Resolve rendering through the repository's existing tool-independent runtime port."""
    package = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
    return (importlib.import_module(package + ".⚙️runtime.🐍️"),
        importlib.import_module(package + ".🪟️presentation.🐍️"),
        importlib.import_module(package + ".🪟️presentation.🥞️hierarchy.🐍️"))


def chapter_output(directory: Path, scene: str, settings: dict) -> Path:
    """📁 Give each chapter an explicit locale, audience and inspection profile."""
    _, presentation, hierarchy = _ports()
    hierarchy.chapter(scene)
    width, height, fps, language = presentation._settings(settings)
    return Path(directory) / "chapters" / f"{scene}_{settings['audience']}_{language}_{width}x{height}p{fps:g}.mp4"


def _decoded_clock(content: Path, work: Path, session) -> tuple[int, float]:
    """🔎 Decode the complete native video clock through the existing FFmpeg port."""
    runtime, _, _ = _ports()
    log = work / ("body-" + uuid.uuid4().hex[:8] + ".log")
    runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-i", str(content), "-map", "0:v:0", "-an", "-f", "null", "-"], session=session, log=log)
    text = log.read_text(encoding="utf-8")
    frames = re.findall(r"frame=\s*(\d+)", text)
    rate = re.search(r"Video:.*?([\d.]+) tbr", text)
    if not frames or not rate or float(rate.group(1)) <= 0:
        raise RuntimeError("Cannot measure the authored chapter frame clock")
    return int(frames[-1]), float(rate.group(1))


def body_frames(content: Path, settings: dict, work: Path, session) -> int:
    """🔢 Measure the authored body clock at the explicit presentation frame rate."""
    frames, source_fps = _decoded_clock(content, work, session)
    _, presentation, _ = _ports()
    return max(1, math.ceil(frames * presentation._settings(settings)[2] / source_fps - 1e-6))


def require_frames(content: Path, expected: int, settings: dict, work: Path, session) -> None:
    """🛑 Reject a healthy container when its decoded finite frame clock is incomplete."""
    frames, source_fps = _decoded_clock(content, work, session)
    _, presentation, _ = _ports()
    fps = presentation._settings(settings)[2]
    if frames != expected or abs(source_fps - fps) > 1e-6:
        raise RuntimeError(f"Encoded frame clock mismatch: {frames} frames at {source_fps:g} fps; expected {expected} at {fps:g} fps")


def chapter_clock(scene: str, frames: int, settings: dict) -> dict:
    """⏲️ Project finite chapter body and both context-sensitive camera sequences."""
    _, presentation, hierarchy = _ports()
    fps = presentation._settings(settings)[2]
    focus = max(3, round(presentation.transition_duration("focus", scene) * fps))
    restore = max(3, round(presentation.transition_duration("restore", scene) * fps))
    neighbors = hierarchy.chapter_neighbors(scene)
    return {"chapter": scene, "path": hierarchy.path(scene), "focus_frames": focus,
        "body_frames": frames, "restore_frames": restore, "frames": focus + frames + restore,
        "fps": fps, "before": hierarchy.boundary(neighbors["previous"], scene),
        "after": hierarchy.boundary(scene, neighbors["next"])}


def _chapter_cursor(runtime, raw: Path, output: Path, clock: dict) -> None:
    """👆 Preserve body pointer activity and the navigation cursor ranges."""
    fps = clock["fps"]
    padding, end = clock["focus_frames"] / fps, (clock["focus_frames"] + clock["body_frames"]) / fps
    intervals = [{"start": 0, "end": padding}, {"start": end, "end": clock["frames"] / fps}]
    intervals.extend({"start": padding + part["start"], "end": min(end, padding + part["end"])}
        for part in runtime.cursor_intervals(raw) if part["start"] < clock["body_frames"] / fps)
    runtime.publish_cursor_intervals(output, intervals)


def publish_chapter(video_id: str, raw: Path, output: Path, authored: dict, settings: dict, session) -> Path:
    """🛑 Publish atomically while sharing cancellation with the enclosing lesson operation."""
    if session.cancelled.is_set():
        raise InterruptedError("Chapter publication cancelled")
    runtime, presentation, hierarchy = _ports()
    if hierarchy.chapter(authored["scene"])["clip_id"] != video_id:
        raise ValueError("The chapter does not belong to the selected clip")
    expected = {"content": authored, "cursor": runtime._cursor_hash(raw),
        "presentation": runtime.presentation_state(video_id, settings, settings["language"])}
    if runtime.reusable_clip(output, expected) and runtime.state_inputs_current(expected):
        return output
    output.parent.mkdir(parents=True, exist_ok=True)
    candidate = output.parent / f".chapter-{time.time_ns():x}.mp4"
    try:
        presentation.render_presentation(video_id, raw, candidate, settings, session=session)
        if session.cancelled.is_set():
            raise InterruptedError("Chapter publication cancelled")
        current = {"content": authored, "cursor": runtime._cursor_hash(raw),
            "presentation": runtime.presentation_state(video_id, settings, settings["language"])}
        if current != expected or not runtime.reusable_clip(raw, authored) or not runtime.state_inputs_current(expected):
            raise RuntimeError(f"Chapter inputs changed while rendering {authored['scene']}")
        if not candidate.is_file() or candidate.stat().st_size <= 1000:
            raise RuntimeError("Chapter renderer produced no complete movie")
        output.with_suffix(".render.json").unlink(missing_ok=True)
        candidate.replace(output)
        runtime._write_state(output.with_suffix(".render.json"), expected)
    finally:
        candidate.unlink(missing_ok=True)
    return output


def compose_lesson(video_id: str, content: Path, output: Path, settings: dict, session, work: Path) -> Path:
    """🎞️ Publish individual chapter windows and join their exact finite video clocks."""
    runtime, presentation, hierarchy = _ports()
    state = json.loads(content.with_suffix(".render.json").read_text(encoding="utf-8"))
    if state.get("video") != video_id or not state.get("chapters"):
        raise ValueError("A lesson requires its complete authored chapter certificate")
    clips, states, clocks = [], [], []
    for authored in state["chapters"]:
        if session.cancelled.is_set():
            raise InterruptedError("Chapter publication cancelled")
        scene = authored["scene"]
        if hierarchy.chapter(scene)["clip_id"] != video_id:
            raise ValueError("A chapter does not belong to the selected clip")
        raw = content.parent / "v" / (scene + ".mp4")
        if not runtime.reusable_clip(raw, authored) or not runtime.state_inputs_current(authored):
            raise RuntimeError(f"Chapter inputs changed before publication: {scene}")
        destination = chapter_output(output.parent, scene, settings)
        publish_chapter(video_id, raw, destination, authored, settings, session)
        clock = chapter_clock(scene, body_frames(raw, settings, work, session), settings)
        _chapter_cursor(runtime, raw, destination, clock)
        expected = {"content": authored, "cursor": runtime._cursor_hash(raw),
            "presentation": runtime.presentation_state(video_id, settings, settings["language"])}
        if not runtime.reusable_clip(destination, expected) or not runtime.state_inputs_current(expected):
            raise RuntimeError(f"Chapter inputs changed after publication: {scene}")
        runtime._write_state(destination.with_suffix(".chapter.json"), clock)
        clips.append(destination)
        states.append(expected)
        clocks.append(clock)
    fps = presentation._settings(settings)[2]
    args, filters = [], []
    for index, (clip, clock) in enumerate(zip(clips, clocks, strict=True)):
        args.extend(["-i", str(clip)])
        duration = clock["frames"] / fps
        filters.extend([f"[{index}:v]setpts=N/({fps}*TB),setsar=1[v{index}]",
            f"[{index}:a]asetpts=PTS-STARTPTS,apad=whole_dur={duration},atrim=duration={duration}[a{index}]"])
    filters.append("".join(f"[v{index}][a{index}]" for index in range(len(clips))) + f"concat=n={len(clips)}:v=1:a=1[video][audio]")
    total = sum(clock["frames"] for clock in clocks)
    runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-y", *args,
        "-filter_complex", ";".join(filters), "-map", "[video]", "-map", "[audio]",
        "-r", str(fps), "-fps_mode", "cfr", "-frames:v", str(total), "-t", str(total / fps),
        "-c:v", "libx264", "-preset", "fast", "-crf", "0", "-pix_fmt", "yuv420p",
        "-c:a", "aac", "-b:a", "192k", "-movflags", "+faststart", str(output)],
        session=session, log=work / "collection-compose.log")
    require_frames(output, total, settings, work, session)
    if any(not runtime.reusable_clip(clip, expected) or not runtime.state_inputs_current(expected)
            for clip, expected in zip(clips, states, strict=True)):
        raise RuntimeError("Chapter inputs changed while assembling the lesson")
    print(f"[DEBUG] Published lesson {video_id}: {len(clips)} chapter windows, {total} frames", flush=True)
    return output
