"""🎬 Portable interfaces for rendering the authored teaching curriculum."""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

import json
import hashlib
from concurrent.futures import ThreadPoolExecutor, as_completed
from contextlib import nullcontext
import os
import shutil
import signal
import subprocess
import threading
import time
import xml.etree.ElementTree as ElementTree

sys.dont_write_bytecode = True

QUALITY_FOLDERS = {"-ql": "480p15", "-qm": "720p30", "-qh": "1080p60"}


def teaching_root(path: Path) -> Path:
    """🧭 Find teaching sources independently of package managers and virtual environments."""
    source = Path(path).resolve()
    for candidate in (source, *source.parents):
        if (candidate / "🏛️architecture" / "⚡️energy").is_dir():
            return candidate
    raise FileNotFoundError(f"Teaching root not found from {path}")


def load_manifest() -> list[dict]:
    """📚 Read the language-independent curriculum contract."""
    root = teaching_root(Path(__file__))
    videos = json.loads((root / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))["videos"]
    if len(videos) != 5 or len({video["id"] for video in videos}) != 5:
        raise ValueError("The curriculum must contain five distinct videos")
    for video in videos:
        if set(video) != {"id", "script", "playlist", "intro", "chapters", "output"} or not video["chapters"]:
            raise ValueError(f"Invalid video contract: {video.get('id')}")
        segments = [video["intro"], *(segment for chapter in video["chapters"] for segment in chapter["segments"])]
        if len({chapter["scene"] for chapter in video["chapters"]}) != len(video["chapters"]):
            raise ValueError(f"Duplicate logical chapter: {video['id']}")
        for chapter in video["chapters"]:
            if set(chapter) != {"scene", "segments"} or not chapter["segments"]:
                raise ValueError(f"Invalid chapter contract: {video['id']}")
        for segment in [*segments, {"script": video["script"], "scene": video["playlist"]}]:
            script = (root / segment["script"]).resolve()
            if set(segment) != {"script", "scene"} or "\\" in segment["script"] or not script.is_relative_to(root) or not script.is_file() or not segment["scene"].isascii() or not segment["scene"].isidentifier():
                raise ValueError(f"Invalid curriculum segment: {segment}")
    return videos


def segment_plan(videos: list[dict] | None = None) -> list[dict]:
    """🧩 Assign stable global identities to every declared intro and authored beat."""
    plan = []
    for index, video in enumerate(load_manifest() if videos is None else videos, 1):
        chapters = [{"scene": video["intro"]["scene"], "segments": [video["intro"]]}, *video["chapters"]]
        for chapter_index, chapter in enumerate(chapters):
            for segment in chapter["segments"]:
                plan.append({"ordinal": len(plan) + 1, "video_index": index, "video_id": video["id"],
                             "chapter_index": chapter_index, "chapter_scene": chapter["scene"], "segment": segment})
    return plan


def prepare_output(argv: list[str]) -> None:
    """📦 Configure output before authored scene imports can discover fonts."""
    import argparse

    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--output-root", type=Path)
    args, _ = parser.parse_known_args(argv)
    if args.output_root:
        os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(args.output_root.resolve())


def manim_command() -> list[str]:
    """🐍 Invoke Manim through the active Python environment on every platform."""
    return [sys.executable, "-m", "manim"]


def ffmpeg_command() -> list[str]:
    """🎞️ Resolve FFmpeg through its provisioned executable interface."""
    executable = os.environ.get("SEMIO_FFMPEG_BINARY") or shutil.which("ffmpeg")
    if not executable:
        try:
            from imageio_ffmpeg import get_ffmpeg_exe

            executable = get_ffmpeg_exe()
        except ImportError:
            executable = None
    if executable:
        return [executable]
    raise FileNotFoundError("FFmpeg is unavailable; run the teaching bootstrap first")


def _stop_process(process: subprocess.Popen) -> None:
    """🛑 Terminate an owned tool and its complete descendant group."""
    if os.name == "nt":
        subprocess.run(["taskkill", "/PID", str(process.pid), "/T", "/F"], capture_output=True,
                       creationflags=subprocess.CREATE_NO_WINDOW, check=False, timeout=10)
    else:
        for sig in (signal.SIGTERM, signal.SIGKILL):
            try:
                os.killpg(process.pid, sig)
            except ProcessLookupError:
                pass
            if sig == signal.SIGTERM:
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    pass
    process.wait(timeout=5)


class ToolSession:
    """🚦 Own concurrent tool lifetimes and cooperative cancellation for one render run."""

    def __init__(self):
        self.cancelled = threading.Event()
        self._lock = threading.Lock()
        self._active = set()

    def cancel(self) -> None:
        """⛔ Stop every active process tree before releasing queued worker shutdown."""
        self.cancelled.set()
        with self._lock:
            active = list(self._active)
        if active:
            with ThreadPoolExecutor(max_workers=len(active)) as pool:
                list(pool.map(_stop_process, active))

    def run(self, command: list[str], *, cwd: Path | None = None, env: dict[str, str] | None = None, log: Path | None = None) -> None:
        """⏳ Execute a tool with UTF-8 output and optional immutable attempt diagnostics."""
        environment = {**os.environ, **(env or {}), "PYTHONUTF8": "1", "PYTHONIOENCODING": "utf-8", "PYTHONDONTWRITEBYTECODE": "1"}
        environment.pop("PYTHONPYCACHEPREFIX", None)
        if log:
            log.parent.mkdir(parents=True, exist_ok=True)
        with log.open("x", encoding="utf-8") if log else nullcontext(None) as stream:
            if stream:
                stream.write("[DEBUG] " + json.dumps({"command": command, "cwd": str(cwd)}) + "\n")
                stream.flush()
            with self._lock:
                if self.cancelled.is_set():
                    raise InterruptedError("Render cancelled")
                process = subprocess.Popen(command, cwd=cwd, env=environment, start_new_session=os.name != "nt",
                                           stdout=stream, stderr=subprocess.STDOUT if stream else None)
                self._active.add(process)
            record = {"command": command, "pid": process.pid, "started": time.time()}
            if log:
                log.with_suffix(".json").write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
            try:
                code = process.wait()
            except BaseException:
                _stop_process(process)
                raise
            finally:
                with self._lock:
                    self._active.discard(process)
            if log:
                log.with_suffix(".json").write_text(json.dumps({**record, "exit_code": code, "finished": time.time()}, indent=2) + "\n", encoding="utf-8")
            if code:
                raise subprocess.CalledProcessError(code, command)


def run_command(command: list[str], *, cwd: Path | None = None, env: dict[str, str] | None = None,
                session: ToolSession | None = None, log: Path | None = None) -> None:
    """🔧 Invoke the shared process interface for standalone or grouped tool operations."""
    (session or ToolSession()).run(command, cwd=cwd, env=env, log=log)


def execute_jobs(tasks: list, worker, *, max_workers: int = 1) -> tuple[dict, dict]:
    """⚙️ Bound global work, retain successful results, and isolate individual failures."""
    if max_workers < 1:
        raise ValueError("Worker count must be positive")
    session = ToolSession()
    pool = ThreadPoolExecutor(max_workers=max_workers)
    futures = {}
    results, failures = {}, {}
    try:
        for index, task in enumerate(tasks):
            futures[pool.submit(worker, task, session)] = index
        for future in as_completed(futures):
            index = futures[future]
            try:
                results[index] = future.result()
            except Exception as error:
                failures[index] = str(error)
    except BaseException:
        session.cancelled.set()
        for future in futures:
            future.cancel()
        pool.shutdown(wait=False, cancel_futures=True)
        session.cancel()
        raise
    pool.shutdown()
    return results, failures


def concat_entry(path: Path) -> str:
    """🪟 Write an FFmpeg concat entry with portable separators and quote escaping."""
    escaped = path.resolve().as_posix().replace("'", r"'\''")
    return f"file '{escaped}'"


def concat_videos(clips: list[Path], output: Path, list_path: Path, *, keep_audio: bool = False) -> None:
    """🔗 Join ordered clips with stream copy and atomically publish the result."""
    if not clips or any(not clip.is_file() for clip in clips):
        raise FileNotFoundError("Every clip must exist before concatenation")
    output.parent.mkdir(parents=True, exist_ok=True)
    list_path.parent.mkdir(parents=True, exist_ok=True)
    list_path.write_text("\n".join(concat_entry(clip) for clip in clips) + "\n", encoding="utf-8")
    temporary = output.with_suffix(".partial.mp4")
    command = [*ffmpeg_command(), "-y", "-f", "concat", "-safe", "0", "-i", str(list_path), "-c:v", "copy"]
    command.extend(["-c:a", "copy"] if keep_audio else ["-an"])
    run_command([*command, str(temporary)])
    temporary.replace(output)


def render_config(media_dir: Path) -> Path:
    """📁 Prepare short render paths and discard malformed generated text SVGs."""
    media_dir.mkdir(parents=True, exist_ok=True)
    text_dir = media_dir / "t"
    linked = text_dir.is_symlink() or getattr(text_dir, "is_junction", lambda: False)()
    if text_dir.is_dir() and not linked:
        for svg in text_dir.glob("*.svg"):
            if svg.is_symlink() or not svg.is_file():
                continue
            try:
                ElementTree.parse(svg)
            except FileNotFoundError:
                continue
            except ElementTree.ParseError:
                if not svg.is_symlink():
                    svg.unlink(missing_ok=True)
    config = media_dir / "render.cfg"
    config.write_text(
        "[CLI]\nmedia_dir = " + media_dir.as_posix() + "\n"
        "video_dir = {media_dir}/v\npartial_movie_dir = {media_dir}/p/{scene_name}\n"
        "images_dir = {media_dir}/i\ntext_dir = {media_dir}/t\ntex_dir = {media_dir}/x\n"
        "log_dir = {media_dir}/logs\nprogress_bar = display\nverbosity = WARNING\n",
        encoding="utf-8",
    )
    return config


def clip_state(settings: dict, sources: list[Path]) -> dict:
    """🔎 Fingerprint render settings and every authored dependency, including assets."""
    return {"settings": settings, "sources": {path.resolve().as_posix(): hashlib.sha256(path.read_bytes()).hexdigest()
                                             for path in sorted(set(sources)) if path.is_file()}}


def reusable_clip(clip: Path, state: dict) -> bool:
    """♻️ Reuse only a complete clip with matching settings and source fingerprints."""
    sidecar = clip.with_suffix(".render.json")
    if not clip.is_file() or clip.stat().st_size <= 1000 or not sidecar.is_file():
        return False
    try:
        return json.loads(sidecar.read_text(encoding="utf-8")) == state
    except (OSError, ValueError):
        return False


def _write_state(path: Path, state: dict) -> None:
    """💾 Publish successful provenance atomically beside its generated artifact."""
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)


def segment_state(segment: dict, settings: dict) -> dict:
    """🧬 Fingerprint the declared scene, authored timings/assets, and shared visual helpers."""
    root = teaching_root(Path(__file__))
    script = root / segment["script"]
    assets = [path for path in script.parent.rglob("*") if path.suffix.lower() in
              (".json", ".pdf", ".svg", ".png", ".jpg", ".jpeg", ".webp", ".ttf", ".otf")]
    font_root = root / "🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts"
    fonts = [path for path in font_root.rglob("*") if path.is_file() and path.suffix.lower() in (".ttf", ".otf")]
    sources = [script, root / "🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py", root / "🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py", *assets, *fonts]
    return {**clip_state(settings, sources), "segment": segment}


def render_segment(segment: dict, media_dir: Path, settings: dict, *, force: bool = False,
                   concat_only: bool = False, session: ToolSession | None = None,
                   output_root: Path | None = None, log_output: bool = False) -> Path:
    """🎞️ Render one entire authored beat with a private cache and success-only provenance."""
    root = teaching_root(Path(__file__))
    media_dir = media_dir.resolve()
    clip = media_dir / "v" / f"{segment['scene']}.mp4"
    state = segment_state(segment, settings)
    sidecar = clip.with_suffix(".render.json")
    if not force and reusable_clip(clip, state):
        return clip
    if concat_only:
        raise FileNotFoundError(f"Missing or stale rendered segment {clip}")
    config = render_config(media_dir)
    sidecar.unlink(missing_ok=True)
    command = [*manim_command(), "--config_file", str(config), settings["quality"], "--media_dir", str(media_dir)]
    if settings["resolution"]:
        command.extend(["--resolution", settings["resolution"]])
    if settings["fps"]:
        command.extend(["--fps", str(settings["fps"])])
    command.extend(["--disable_caching", str(root / segment["script"]), segment["scene"]])
    print(f"[segment] Rendering {segment['scene']}", flush=True)
    log = media_dir / f"attempt-{time.time_ns()}.log" if log_output else None
    environment = {"SEMIO_TEACHING_OUTPUT_ROOT": str(output_root or media_dir.parent)}
    run_command(command, cwd=root, env=environment, session=session, log=log)
    if segment_state(segment, settings) != state:
        raise RuntimeError(f"Authored sources changed while rendering {segment['scene']}")
    if not clip.is_file() or clip.stat().st_size <= 1000:
        raise FileNotFoundError(f"Manim did not create a complete segment {clip}")
    _write_state(sidecar, state)
    print(f"[segment] Complete {segment['scene']}", flush=True)
    return clip


def _logical_state(scene: str, segments: list[dict], settings: dict) -> dict:
    """📒 Describe a logical chapter by its ordered complete segment inputs."""
    return {"scene": scene, "settings": settings, "segments": [segment_state(segment, settings) for segment in segments]}


def compose_videos(video_ids: list[str] | None = None, *, quality_flag: str = "-ql", play: bool = False,
                   output_root: Path | None = None, media_dir: Path | None = None,
                   output_dir: Path | None = None, force: bool = False, concat_only: bool = False,
                   resolution: str | None = None, fps: float | None = None, jobs: int = 1,
                   log_output: bool = False) -> dict[str, Path]:
    """🎥 Render a globally bounded segment plan, then assemble ordered chapters and lessons."""
    if jobs < 1 or quality_flag not in QUALITY_FOLDERS or (force and concat_only):
        raise ValueError("Invalid render settings")
    root = teaching_root(Path(__file__))
    videos = load_manifest()
    selected = set(video_ids or [video["id"] for video in videos])
    if selected - {video["id"] for video in videos} or (media_dir and len(selected) != 1):
        raise ValueError("Invalid selected videos or explicit media directory")
    output_root = Path(output_root or os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", _WORKSPACE_ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos")).resolve()
    segment_root = Path(media_dir).resolve() / "s" if media_dir else output_root / "s"
    destination = Path(output_dir or (Path(media_dir) / "rendered" if media_dir else output_root / "rendered")).resolve()
    settings = {"quality": quality_flag, "resolution": resolution, "fps": fps}
    plan = segment_plan(videos)
    groups, tasks = [], []
    for index, video in enumerate(videos, 1):
        if video["id"] not in selected:
            continue
        logical_dir = Path(media_dir).resolve() if media_dir else output_root / str(index)
        _write_state(logical_dir / "settings.json", settings)
        chapters = [{"scene": video["intro"]["scene"], "segments": [video["intro"]]}, *video["chapters"]]
        for chapter_index, chapter in enumerate(chapters):
            state = _logical_state(chapter["scene"], chapter["segments"], settings)
            clip = logical_dir / "v" / f"{chapter['scene']}.mp4"
            entries = [job for job in plan if job["video_index"] == index and job["chapter_index"] == chapter_index]
            groups.append({"video": video, "directory": logical_dir, "clip": clip,
                           "chapter": chapter, "state": state, "entries": entries})
            if force or not reusable_clip(clip, state):
                tasks.extend(entries)

    def render(job, session):
        return render_segment(job["segment"], segment_root / str(job["ordinal"]), settings, force=force,
                              concat_only=concat_only, session=session, output_root=Path(media_dir or output_root),
                              log_output=log_output)

    _, failures = execute_jobs(tasks, render, max_workers=jobs)
    errors = [f"{tasks[index]['segment']['scene']}: {error}" for index, error in failures.items()]
    for group in groups:
        if not force and reusable_clip(group["clip"], group["state"]):
            continue
        clips = [segment_root / str(job["ordinal"]) / "v" / f"{job['segment']['scene']}.mp4" for job in group["entries"]]
        if not all(reusable_clip(clip, segment_state(job["segment"], settings)) for clip, job in zip(clips, group["entries"], strict=True)):
            continue
        sidecar = group["clip"].with_suffix(".render.json")
        sidecar.unlink(missing_ok=True)
        if len(clips) == 1:
            group["clip"].parent.mkdir(parents=True, exist_ok=True)
            temporary = group["clip"].with_suffix(".partial.mp4")
            shutil.copy2(clips[0], temporary)
            temporary.replace(group["clip"])
        else:
            concat_videos(clips, group["clip"], group["directory"] / f"{group['chapter']['scene']}.txt")
        if _logical_state(group["chapter"]["scene"], group["chapter"]["segments"], settings) != group["state"]:
            errors.append(f"Authored inputs changed while assembling {group['chapter']['scene']}")
            continue
        _write_state(sidecar, group["state"])
        print(f"[chapter] Complete {group['chapter']['scene']}", flush=True)

    completed = {}
    rate = fps if fps is not None else {"-ql": 15, "-qm": 30, "-qh": 60}[quality_flag]
    dimensions = resolution.replace(",", "x") if resolution else QUALITY_FOLDERS[quality_flag].split("p")[0]
    for video in videos:
        if video["id"] not in selected:
            continue
        chapters = [group for group in groups if group["video"]["id"] == video["id"]]
        if not all(reusable_clip(group["clip"], group["state"]) for group in chapters):
            errors.append(f"Incomplete chapter inputs for {video['id']}")
            continue
        output = destination / f"{video['output']}_{dimensions}p{rate:g}.mp4"
        state = {"video": video["id"], "settings": settings, "chapters": [group["state"] for group in chapters]}
        if force or not reusable_clip(output, state):
            output.with_suffix(".render.json").unlink(missing_ok=True)
            concat_videos([group["clip"] for group in chapters], output, chapters[0]["directory"] / "concat.txt")
            _write_state(output.with_suffix(".render.json"), state)
        completed[video["id"]] = output
        print(f"[{video['id']}] Ready: {output}", flush=True)
        if play:
            if sys.platform == "win32":
                os.startfile(output)
            else:
                run_command(["open" if sys.platform == "darwin" else "xdg-open", str(output)])
    if errors:
        raise RuntimeError("Render failures; successful segments remain reusable:\n" + "\n".join(errors))
    return completed


def compose_video(video_id: str, **kwargs) -> Path:
    """🎬 Render a single lesson through the same declared curriculum execution plan."""
    return compose_videos([video_id], **kwargs)[video_id]


def video_main(video_id: str, argv: list[str] | None = None) -> int:
    """▶️ Expose the same portable command options for every authored wrapper."""
    _render_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🖌️render.🐍️")
    main = _render_module.main

    return main(["--video", video_id, *(sys.argv[1:] if argv is None else argv)])
