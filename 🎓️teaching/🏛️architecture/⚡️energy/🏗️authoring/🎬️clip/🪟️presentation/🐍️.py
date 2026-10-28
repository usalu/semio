"""🪟 Present authored lessons through the shared layered topic and video hierarchy."""

from __future__ import annotations

import importlib
import json
import math
import re
from pathlib import Path

_PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
_ROOT = Path(__file__).resolve().parent
_theme = importlib.import_module(_PACKAGE + ".🎨️theme.🐍️")
_UNIT = 8 / 900
_browser = importlib.import_module(_PACKAGE + ".🎬️clip.🪟️presentation.🌐️browser.🐍️")
_geometry = importlib.import_module(_PACKAGE + ".🎬️clip.🪟️presentation.📐️geometry.🐍️")
_typography = importlib.import_module(_PACKAGE + ".🎬️clip.🪟️presentation.🔤️typography.🐍️")
_interaction = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🐍️")
_interaction_render = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🖌️render.🐍️")
_hierarchy = importlib.import_module(_PACKAGE + ".🎬️clip.🪟️presentation.🥞️hierarchy.🐍️")
_hierarchy_render = importlib.import_module(_PACKAGE + ".🎬️clip.🪟️presentation.🥞️hierarchy.🖌️render.🐍️")


def presentation_contract() -> dict:
    """📐 Read the explicitly localized topic and video presentation contract."""
    return _hierarchy.hierarchy_contract()


def source_files() -> tuple[Path, ...]:
    """🧬 Expose every authored input that changes presentation cache identity."""
    runtime = importlib.import_module(_PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
    authoring = _ROOT.parents[1]
    visuals = authoring / "🎨️visuals"
    helpers = (visuals / "🐍️.py", authoring / "🔤️fonts/🐍️.py", Path(runtime.__file__))
    paths = {Path(__file__), _ROOT / "🔣️.json", _ROOT / "🧬️schema.json", *_theme.source_files(), *_interaction.source_files(), *_geometry.source_files(), *_typography.source_files(), *_browser.source_files(), *_hierarchy.source_files(), Path(_hierarchy_render.__file__),
             Path(_interaction_render.__file__), _ROOT / "🖱️interaction/🎬️scene/🐍️.py", _ROOT / "🖱️interaction/🎬️scene/🔣️.schema.json",
             visuals / "🔣️.json", visuals / "🫟️silhouettes/🔣️.json"}
    for helper in helpers:
        paths.update(runtime.source_dependencies(helper))
    return tuple(sorted(paths))


def lesson_window(video_id: str) -> dict:
    """🏷️ Resolve one lesson by its canonical curriculum identity."""
    return _hierarchy.video(video_id)


def _scaled(rect: dict, width: int, height: int) -> dict:
    """📏 Project a canonical top-left rectangle to output pixels."""
    return {key: value * (width / 1600 if key in ("x", "width") else height / 900)
            for key, value in rect.items()}


def focus_viewport(width: int, height: int) -> dict:
    """🔍 Return the unobstructed focused lesson viewport in output pixels."""
    if width <= 0 or height <= 0:
        raise ValueError("Presentation dimensions must be positive")
    return _scaled(presentation_contract()["focus"]["viewport"], width, height)


def transition_duration(phase: str, chapter_id: str) -> float:
    """⏱️ Use each chapter's exact ancestor route and content camera duration."""
    return _hierarchy_render.transition_duration(chapter_id, phase)


def _settings(settings: dict) -> tuple[int, int, float, str]:
    """⚙️ Validate explicitly localized presentation output settings."""
    language = settings.get("language")
    if language not in presentation_contract()["locales"]:
        raise ValueError("An explicit presentation language is required: en or de")
    quality = {"-ql": ("854,480", 15), "-qm": ("1280,720", 30), "-qh": ("1920,1080", 60)}
    default_resolution, default_fps = quality.get(settings.get("quality", "-ql"), quality["-ql"])
    resolution = settings.get("resolution") or default_resolution
    width, height = (int(part) for part in resolution.replace("x", ",").split(","))
    fps = float(settings["fps"]) if settings.get("fps") is not None else float(default_fps)
    if width < 2 or height < 2 or width % 2 or height % 2 or not math.isfinite(fps) or fps <= 0:
        raise ValueError("Presentation needs positive even dimensions and a positive frame rate")
    if abs(width / height - 16 / 9) > 0.01:
        raise ValueError("Presentation resolution must have a 16:9 aspect ratio")
    return width, height, fps, language


def _point(x: float, y: float):
    """📍 Translate canonical screen pixels to native Manim scene coordinates."""
    import numpy as np

    return np.array([(x - 800) * _UNIT, (450 - y) * _UNIT, 0.0])


def _text(value: str, x: float, y: float, *, font_size: float = 11.2, line_height: float = 11.2, color: str | None = None):
    """🔤 Preserve actual CSS em size, advance and baseline in a centered line box."""
    return _typography._native_text(value, x, y, _UNIT, font_size=font_size, line_height=line_height,
                                    color=color or _theme.color("light"))


def _box(rect: dict, *, color: str, fill: str | None = None, opacity: float = 1, stroke: float = 1):
    """🔲 Build a square rectangle at a screen-space location."""
    from manim import Rectangle

    return Rectangle(width=rect["width"] * _UNIT, height=rect["height"] * _UNIT, color=color,
                     stroke_width=stroke, fill_color=fill or color, fill_opacity=opacity).move_to(
                         _point(rect["x"] + rect["width"] / 2, rect["y"] + rect["height"] / 2))


def render_interaction_frame(video_id: str, phase: str, t: float, output: Path, settings: dict, *,
                             point=None, pressed=None, menu_open: bool = False, hierarchy=None,
                             snapshot=None, transition_elapsed=None) -> dict:
    """🖱️ Render a real cursor state and expose its actual hit geometry for independent verification."""
    return _hierarchy_render.frame(video_id, phase, t, output, settings, point=point, pressed=pressed,
                                   menu_open=menu_open, hierarchy=hierarchy, snapshot=snapshot, transition_elapsed=transition_elapsed)


def render_overview(output: Path, settings: dict) -> Path:
    """🗺️ Render the same hovered canvas and parked cursor at every lesson boundary."""
    first = _hierarchy.ordered_chapters()[0]
    return render_interaction_frame(first["id"], "focus", 0, output, settings)["path"]


def _transition(video_id: str, snapshot: Path, output: Path, settings: dict, session, *, phase: str = "focus") -> tuple[Path, int]:
    """🎞️ Replay a distinct committed fullscreen or restore click before moving its window."""
    _, _, fps, _ = _settings(settings)
    count = max(3, round(transition_duration(phase, video_id) * fps))
    frames = output / ("i" if phase == "focus" else "o")
    requests = [{"video_id": video_id, "phase": phase, "t": index / (count - 1),
                 "output": frames / f"{index:04d}.png", "snapshot": snapshot} for index in range(count)]
    if session.cancelled.is_set():
        raise InterruptedError("Presentation cancelled")
    print(f"[DEBUG] Presentation {video_id} {phase}: actual UI batch of {count} frames", flush=True)
    _hierarchy_render.frame_batch(requests, settings, session=session)
    return frames, count


def _clean_attempt(work: Path, parent: Path, *, keep_logs: bool) -> None:
    """🧹 Remove only verified owned presentation outputs, retaining compact failure diagnostics."""
    import shutil

    resolved = work.resolve()
    if resolved.parent != parent.resolve() or not re.fullmatch(r"p-[0-9a-f]{8}", resolved.name):
        raise RuntimeError("Presentation cleanup target is outside its owned output directory")
    if not keep_logs:
        shutil.rmtree(resolved)
        return
    for child in resolved.iterdir():
        if child.is_symlink() or getattr(child, "is_junction", lambda: False)():
            child.unlink()
        elif child.is_file() and child.suffix in (".log", ".json"):
            if child.stat().st_size > 1024 * 1024:
                child.write_bytes(child.read_bytes()[-1024 * 1024:])
        else:
            target = child.resolve()
            if not target.is_relative_to(resolved):
                raise RuntimeError("Presentation cleanup child escapes its owned attempt")
            if child.is_dir():
                shutil.rmtree(target)
            else:
                child.unlink()


def _normalize_transition_frames(source: Path, output: Path, count: int, settings: dict, session) -> Path:
    """🖼️ Publish uniform RGB PNG copies without mutating browser assets or compositing alpha."""
    from PIL import Image

    width, height, _, _ = _settings(settings)
    output.mkdir(parents=True, exist_ok=True)
    for index in range(count):
        if session.cancelled.is_set():
            raise InterruptedError("Transition normalization cancelled")
        destination = output / f"{index:04d}.png"
        temporary = destination.with_suffix(".tmp.png")
        try:
            with Image.open(source / destination.name) as image:
                if image.format != "PNG" or image.size != (width, height):
                    raise RuntimeError("Transition PNG does not match its declared viewport")
                pixels = image.convert("RGB").tobytes()
            Image.frombytes("RGB", (width, height), pixels).save(temporary, "PNG")
            if session.cancelled.is_set():
                raise InterruptedError("Transition normalization cancelled")
            temporary.replace(destination)
        finally:
            temporary.unlink(missing_ok=True)
    return output


def _render_chapter(video_id: str, content: Path, output: Path, settings: dict, session, work: Path) -> Path:
    """🎥 Compose finite video and audio streams without unbounded filter scheduling."""
    import time

    runtime = importlib.import_module(_PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
    width, height, fps, _ = _settings(settings)
    ffmpeg = runtime.ffmpeg_command()
    run = lambda args, name: runtime.run_command(ffmpeg + ["-hide_banner", "-y", *args], session=session,
                                                log=work / name)
    run(["-i", str(content), "-map", "0:v:0", "-an", "-f", "null", "-"], "probe.log")
    metadata = (work / "probe.log").read_text(encoding="utf-8")
    has_audio = bool(re.search(r"Stream #.*Audio:", metadata))
    frames = re.findall(r"frame=\s*(\d+)", metadata)
    rate = re.search(r"Video:.*?([\d.]+) tbr", metadata)
    if not frames or not rate or float(rate.group(1)) <= 0:
        raise RuntimeError("Cannot determine the authored video's frame count and frame rate")
    source_fps = float(rate.group(1))
    body_count = max(1, math.ceil(int(frames[-1]) * fps / source_fps - 1e-6))
    first, last = work / "first.png", work / "last.png"
    dark = _theme.color("dark").replace("#", "0x")
    fit = (f"scale={width}:{height}:force_original_aspect_ratio=decrease,"
           f"pad={width}:{height}:(ow-iw)/2:(oh-ih)/2:color={dark},setsar=1")
    normalized = (f"setpts=N/({source_fps}*TB),tpad=stop_mode=clone:stop_duration={1 / source_fps},"
                  f"fps={fps}:eof_action=pass,trim=end_frame={body_count},{fit}")
    run(["-i", str(content), "-vf", normalized, "-frames:v", "1", "-update", "1", str(first)], "first.log")
    run(["-i", str(content), "-vf", normalized + f",select=eq(n\\,{body_count - 1})", "-frames:v", "1", "-update", "1", str(last)], "last.log")
    inward, count = _transition(video_id, first, work, settings, session, phase="focus")
    outward, restore_count = _transition(video_id, last, work, settings, session, phase="restore")
    inward = _normalize_transition_frames(inward, work / "normalized" / "i", count, settings, session)
    outward = _normalize_transition_frames(outward, work / "normalized" / "o", restore_count, settings, session)
    padding = count / fps
    total_count = body_count + count + restore_count
    total_duration = total_count / fps
    filters = (
        f"[0:v]{normalized},format=yuv420p[body];"
        "[1:v]setpts=PTS-STARTPTS,format=yuv420p[intro];"
        "[2:v]setpts=PTS-STARTPTS,format=yuv420p[outro];"
        f"[intro][body][outro]concat=n=3:v=1:a=0,setpts=N/({fps}*TB),"
        f"tpad=stop_mode=clone:stop_duration={1 / fps},fps={fps}:eof_action=pass,"
        f"trim=end_frame={total_count}[video];"
    )
    filters += (f"[0:a]asetpts=PTS-STARTPTS,adelay={round(padding * 1000)}:all=1,"
                f"apad=whole_dur={total_duration},atrim=duration={total_duration}[audio]"
                if has_audio else f"anullsrc=r=48000:cl=stereo,atrim=duration={total_duration}[audio]")
    partial = output.with_name(work.name + ".partial" + output.suffix)
    args = ["-i", str(content), "-framerate", str(fps), "-i", str(inward / "%04d.png"),
            "-framerate", str(fps), "-i", str(outward / "%04d.png"),
            "-filter_complex", filters, "-map", "[video]", "-map", "[audio]",
            "-r", str(fps), "-fps_mode", "cfr", "-frames:v", str(total_count), "-t", str(total_duration),
            "-c:v", "libx264", "-preset", "fast", "-crf", "0", "-pix_fmt", "yuv420p",
            "-c:a", "aac", "-b:a", "192k", "-movflags", "+faststart", str(partial)]
    started = time.monotonic()
    print(f"[DEBUG] Presentation {video_id}: composing {total_count} frames", flush=True)
    try:
        run(args, "compose.log")
        if not partial.is_file() or partial.stat().st_size == 0:
            raise RuntimeError("Presentation encoder produced no output")
        collection = importlib.import_module(_PACKAGE + ".🎬️clip.🪟️presentation.📚️collection.🐍️")
        collection.require_frames(partial, total_count, settings, work, session)
        if session.cancelled.is_set():
            raise InterruptedError("Presentation cancelled before publication")
        partial.replace(output)
    finally:
        partial.unlink(missing_ok=True)
    print(f"[DEBUG] Presentation {video_id}: composed in {time.monotonic() - started:.1f}s", flush=True)
    return output


@_hierarchy.capture_catalog()
def render_presentation(video_id: str, content: Path, output: Path, settings: dict, session=None) -> Path:
    """🎬 Wrap one complete lesson and reclaim its isolated generated work on every outcome."""
    import uuid

    runtime = importlib.import_module(_PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
    lesson_window(video_id)
    _settings(settings)
    content, output = Path(content).resolve(), Path(output).resolve()
    if not content.is_file():
        raise FileNotFoundError(content)
    if content == output:
        raise ValueError("Presentation output must differ from its authored content")
    output.parent.mkdir(parents=True, exist_ok=True)
    work = output.parent / ("p-" + uuid.uuid4().hex[:8])
    work.mkdir(exist_ok=False)
    success = False
    try:
        state = json.loads(content.with_suffix(".render.json").read_text(encoding="utf-8"))
        owned_session = session or runtime.ToolSession()
        if "scene" in state:
            scene = state["scene"]
            if _hierarchy.chapter(scene)["clip_id"] != video_id:
                raise ValueError("The chapter does not belong to the selected clip")
            result = _render_chapter(scene, content, output, settings, owned_session, work)
        else:
            collection = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.📚️collection.🐍️")
            result = collection.compose_lesson(video_id, content, output, settings, owned_session, work)
        success = True
        return result
    finally:
        _clean_attempt(work, output.parent, keep_logs=not success)
