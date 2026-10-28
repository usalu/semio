"""🎬 Portable interfaces for rendering the authored teaching curriculum."""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

import json
import ast
import hashlib
import math
import re
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
_output = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️")
_audience = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.👥️audience.🐍️")
_presentation_hierarchy = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🐍️")


def teaching_root(path: Path) -> Path:
    """🧭 Find teaching sources independently of package managers and virtual environments."""
    source = Path(path).resolve()
    for candidate in (source, *source.parents):
        if (candidate / "🏛️architecture" / "⚡️energy").is_dir():
            return candidate
    raise FileNotFoundError(f"Teaching root not found from {path}")


def load_manifest(*, source_ids: list[str] | None = None) -> list[dict]:
    """📚 Read the language-independent curriculum contract."""
    root = teaching_root(Path(__file__))
    videos = json.loads((root / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))["videos"]
    if not videos or len({video["id"] for video in videos}) != len(videos):
        raise ValueError("The curriculum must contain distinct videos")
    for video in videos:
        if set(video) != {"id", "script", "chapters", "output"} or not video["chapters"]:
            raise ValueError(f"Invalid video contract: {video.get('id')}")
        validate_source = source_ids is None or video["id"] in source_ids
        wrapper = (root / video["script"]).resolve()
        if "\\" in video["script"] or not wrapper.is_relative_to(root) or (validate_source and not wrapper.is_file()):
            raise ValueError(f"Invalid lesson entry: {video['id']}")
        segments = [segment for chapter in video["chapters"] for segment in chapter["segments"]]
        if len({chapter["scene"] for chapter in video["chapters"]}) != len(video["chapters"]):
            raise ValueError(f"Duplicate logical chapter: {video['id']}")
        for chapter in video["chapters"]:
            if set(chapter) != {"scene", "segments"} or not chapter["segments"]:
                raise ValueError(f"Invalid chapter contract: {video['id']}")
        for segment in segments:
            script = (root / segment["script"]).resolve()
            if not {"script", "scene"} <= set(segment) or set(segment) - {"script", "scene", "clauses"} or "\\" in segment["script"] or not script.is_relative_to(root) or (validate_source and not script.is_file()) or not segment["scene"].isascii() or not segment["scene"].isidentifier():
                raise ValueError(f"Invalid curriculum segment: {segment}")
            if "clauses" in segment:
                selection = segment["clauses"]
                if not isinstance(selection, dict) or set(selection) != {"first", "last"} or not all(isinstance(value, str) and value for value in selection.values()):
                    raise ValueError(f"Invalid clause selection: {segment}")
                if not validate_source:
                    continue
                scene = next((node for node in ast.parse(script.read_text(encoding="utf-8")).body if isinstance(node, ast.ClassDef) and node.name == segment["scene"]), None)
                narration = next((ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets)), []) if scene else []
                keys = [clause[0] for clause in narration]
                if selection["first"] not in keys or selection["last"] not in keys or keys.index(selection["first"]) > keys.index(selection["last"]):
                    raise ValueError(f"Unknown or reversed clause selection: {segment}")
    return videos


def segment_plan(videos: list[dict] | None = None) -> list[dict]:
    """🧩 Assign stable global identities to every declared intro and authored beat."""
    plan, sources = [], {}
    for index, video in enumerate(load_manifest() if videos is None else videos, 1):
        chapters = video["chapters"]
        for chapter_index, chapter in enumerate(chapters):
            for segment in chapter["segments"]:
                identity = (segment["script"], segment["scene"])
                source_ordinal = sources.setdefault(identity, len(plan) + 1)
                plan.append({"source_ordinal": source_ordinal, "ordinal": len(plan) + 1, "video_index": index, "video_id": video["id"],
                             "chapter_index": chapter_index, "chapter_scene": chapter["scene"], "segment": segment})
    return plan


def source_plan(plan: list[dict]) -> list[dict]:
    """♻️ Render one canonical source for every repeated complete or partial occurrence."""
    sources = {}
    for job in plan:
        sources.setdefault(job["source_ordinal"], job)
    return list(sources.values())


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


def clip_duration(clip: Path) -> float:
    """⏱️ Measure media through the existing speech-duration interface."""
    speech = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️")
    return speech.probe_duration(Path(clip))


def _normalized_cursor_intervals(intervals: list[dict]) -> list[dict]:
    """📏 Validate and merge ordered half-open cursor activity ranges."""
    if not isinstance(intervals, list):
        raise TypeError("Cursor intervals must be a list")
    ranges = []
    for interval in intervals:
        if not isinstance(interval, dict) or set(interval) != {"start", "end"}:
            raise ValueError("Cursor intervals require start and end")
        start, end = interval["start"], interval["end"]
        if any(isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value)
               for value in (start, end)) or not 0 <= start < end:
            raise ValueError("Invalid cursor activity interval")
        ranges.append((float(start), float(end)))
    merged = []
    for start, end in sorted(ranges):
        if merged and start <= merged[-1]["end"] + 1e-9:
            merged[-1]["end"] = max(merged[-1]["end"], end)
        else:
            merged.append({"start": start, "end": end})
    return merged


def cursor_intervals(clip: Path) -> list[dict]:
    """👆 Read normalized cursor activity independently of movie decoding."""
    sidecar = Path(clip).with_suffix(".cursor.json")
    if not sidecar.is_file():
        return []
    record = json.loads(sidecar.read_text(encoding="utf-8"))
    if not isinstance(record, dict) or record.get("version") != 1:
        raise ValueError(f"Invalid cursor activity metadata: {sidecar}")
    return _normalized_cursor_intervals(record["intervals"])


def publish_cursor_intervals(clip: Path, intervals: list[dict]) -> None:
    """🧾 Atomically publish normalized activity beside its completed movie."""
    _write_state(Path(clip).with_suffix(".cursor.json"), {"version": 1, "intervals": _normalized_cursor_intervals(intervals)})


def clip_cursor_intervals(intervals: list[dict], start: float, end: float) -> list[dict]:
    """✂️ Intersect cursor activity with an excerpt and rebase it to zero."""
    if any(isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value)
           for value in (start, end)) or not 0 <= start < end:
        raise ValueError("Invalid cursor excerpt range")
    clipped = [{"start": max(part["start"], start) - start, "end": min(part["end"], end) - start}
               for part in _normalized_cursor_intervals(intervals)
               if min(part["end"], end) > max(part["start"], start)]
    return _normalized_cursor_intervals(clipped)


def propagate_cursor_intervals(clips: list[Path], output: Path) -> None:
    """🔀 Offset activity through concatenation while avoiding empty-input media probes."""
    ranges = [cursor_intervals(clip) for clip in clips]
    if not any(ranges):
        publish_cursor_intervals(output, [])
        return
    result, offset = [], 0.0
    for clip, intervals in zip(clips, ranges, strict=True):
        duration = clip_duration(clip)
        if isinstance(duration, bool) or not math.isfinite(duration) or duration <= 0:
            raise ValueError(f"Invalid clip duration for cursor activity: {clip}")
        for part in intervals:
            if part["start"] >= duration or part["end"] > duration + 1e-9:
                raise ValueError(f"Cursor activity exceeds its source movie: {clip}")
            result.append({"start": offset + part["start"], "end": offset + min(part["end"], duration)})
        offset += duration
    publish_cursor_intervals(output, result)


def _cursor_hash(clip: Path) -> str | None:
    """🔐 Fingerprint activity metadata that changes enclosing parent hover."""
    sidecar = Path(clip).with_suffix(".cursor.json")
    return hashlib.sha256(sidecar.read_bytes()).hexdigest() if sidecar.is_file() else None


def concat_videos(clips: list[Path], output: Path, list_path: Path, *, keep_audio: bool = True) -> None:
    """🔗 Join ordered clips with stream copy and atomically publish the result."""
    if not clips or any(not clip.is_file() for clip in clips):
        raise FileNotFoundError("Every clip must exist before concatenation")
    output.parent.mkdir(parents=True, exist_ok=True)
    list_path.parent.mkdir(parents=True, exist_ok=True)
    list_path.write_text("\n".join(concat_entry(clip) for clip in clips) + "\n", encoding="utf-8")
    temporary = output.with_suffix(".partial.mp4")
    command = [*ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-f", "concat", "-safe", "0", "-i", str(list_path), "-c:v", "copy"]
    command.extend(["-c:a", "copy"] if keep_audio else ["-an"])
    run_command([*command, str(temporary)])
    temporary.replace(output)
    propagate_cursor_intervals(clips, output)


def render_config(media_dir: Path) -> Path:
    """📁 Prepare short render paths and discard malformed generated text SVGs."""
    media_dir = media_dir.resolve()
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
        + "".join(f"{key} = {value}\n" for key, value in _output.MANIM_DIRECTORIES.items())
        + "progress_bar = display\nverbosity = WARNING\n",
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


def source_dependencies(script: Path) -> list[Path]:
    """🔗 Follow repository-owned literal module imports without loading rendering libraries."""
    root = teaching_root(Path(__file__))
    sources, pending = set(), [script]
    while pending:
        source = pending.pop()
        if source in sources or not source.is_file():
            continue
        sources.add(source)
        try:
            tree = ast.parse(source.read_text(encoding="utf-8"))
        except SyntaxError:
            continue
        for node in ast.walk(tree):
            if isinstance(node, ast.Call) and node.args and isinstance(node.args[0], ast.Constant):
                function = node.func
                name = function.id if isinstance(function, ast.Name) else function.attr if isinstance(function, ast.Attribute) else ""
                module = node.args[0].value
                if name == "import_module" and isinstance(module, str) and module.startswith("🎓️teaching."):
                    pending.append(root / (module.removeprefix("🎓️teaching.").replace(".", "/") + ".py"))
    return list(sources)


def segment_state(segment: dict, settings: dict) -> dict:
    """🧬 Fingerprint source imports, authored assets, narration and shared rendering profiles."""
    root = teaching_root(Path(__file__))
    script = root / segment["script"]
    shared = root / "🏛️architecture/⚡️energy/🏗️authoring"
    interaction = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🐍️")
    helper = shared / "🎬️clip/🪟️presentation/🖱️interaction/🎬️scene"
    sources = [Path(__file__), *source_dependencies(script), shared / "🔤️fonts/🐍️.py", shared / "🎨️visuals/🐍️.py",
               shared / "🔊️speech/🐍️.py", *interaction.source_files(), helper / "🐍️.py", helper / "🔣️.schema.json"]
    assets = [path for owner in {source.parent for source in sources if source.suffix == ".py"} for path in owner.rglob("*")
              if "🧪️tests" not in path.relative_to(owner).parts and path.suffix.lower() in
              (".json", ".pdf", ".svg", ".png", ".jpg", ".jpeg", ".webp", ".ttf", ".otf", ".mp3", ".wav", ".m4a", ".flac", ".ogg")]
    theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
    timing = [source_speech_root(source, language=settings["language"]) / "timing.json" for source in sources]
    recordings = [path for _, path in source_narration(segment, language=settings["language"])]
    return {**clip_state(settings, [*sources, *assets, *timing, *recordings, *theme.source_files(), *_audience.source_files()]), "segment": segment}


def source_speech_root(script: Path, *, language: str) -> Path:
    """🌐 Isolate canonical speech by source identity and explicit content language."""
    if language not in {"en", "de"}:
        raise ValueError("An explicit supported speech language is required")
    owner = _output.speech_root(script.parent)
    return owner.parent / language / owner.name


def source_narration(segment: dict, *, language: str) -> list[tuple[str, Path]]:
    """🗣️ Resolve existing canonical clause recordings without generating speech."""
    script = teaching_root(Path(__file__)) / segment["script"]
    try:
        scene = next(node for node in ast.parse(script.read_text(encoding="utf-8")).body if isinstance(node, ast.ClassDef) and node.name == segment["scene"])
        narration = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
    except (SyntaxError, StopIteration):
        return []
    number = re.match(r"Beat(\d+)", segment["scene"])
    owner = f"beat_{number.group(1)}" if number else segment["scene"]
    directory = source_speech_root(script, language=language) / f"{owner}_clauses"
    clauses = [(clause[0], next((directory / f"clause_{index:02d}{suffix}" for suffix in (".mp3", ".wav", ".m4a", ".flac", ".ogg")
                                if (directory / f"clause_{index:02d}{suffix}").is_file()), None))
               for index, clause in enumerate(narration) if clause[1 if language == "en" else 2].strip()]
    if not any(path for _, path in clauses):
        return []
    if any(path is None for _, path in clauses):
        raise ValueError(f"Incomplete canonical clause audio: {directory}")
    return clauses


def attach_source_narration(clip: Path, clauses: list[tuple[str, Path]], marks: dict, *, session: ToolSession | None = None) -> None:
    """🔊 Bind existing clause recordings to the source's measured subtitle clock."""
    speech = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️")
    placements = []
    for key, path in clauses:
        mark = marks.get(key)
        if not mark or speech.probe_duration(path) > float(mark["window"]) + 0.02:
            raise ValueError(f"Narration does not fit its measured clause: {key}")
        placements.append((path, float(mark["start"])))
    audio = clip.with_suffix(".narration.mp3")
    duration = speech.probe_duration(clip)
    speech.assemble_aligned_track(placements, audio, total_duration=duration, min_gap=0.0, session=session)
    temporary = clip.with_suffix(".narrated.mp4")
    print(f"[narration] Aligning canonical clauses for {clip.stem}", flush=True)
    run_command([*ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i", str(clip), "-i", str(audio),
                 "-map", "0:v:0", "-map", "1:a:0", "-c:v", "copy", "-c:a", "aac", "-af", f"apad=whole_dur={duration:.9f}", "-t", f"{duration:.9f}", str(temporary)],
                session=session)
    if not temporary.is_file() or temporary.stat().st_size <= 1000:
        raise FileNotFoundError(f"Missing narrated source output {temporary}")
    temporary.replace(clip)


def render_segment(segment: dict, media_dir: Path, settings: dict, *, force: bool = False,
                   concat_only: bool = False, session: ToolSession | None = None,
                   output_root: Path | None = None, log_output: bool = False, require_trace: bool = False) -> Path:
    """🎞️ Render one canonical source beat with measured narration and success-only provenance."""
    _audience.subtitles_visible(settings["audience"])
    root = teaching_root(Path(__file__))
    media_dir = media_dir.resolve()
    clip = media_dir / "v" / f"{segment['scene']}.mp4"
    segment = {key: segment[key] for key in ("script", "scene")}
    narration = source_narration(segment, language=settings["language"])
    require_trace = require_trace or bool(narration)
    state = segment_state(segment, settings)
    sidecar = clip.with_suffix(".render.json")
    trace = source_trace_path(segment, Path(output_root or media_dir.parent)) if require_trace else None
    cursor_trace = source_trace_path(segment, Path(output_root or media_dir.parent)).with_name("👆️.json")
    if not force and reusable_clip(clip, state) and clip.with_suffix(".cursor.json").is_file() and (not require_trace or source_marks(clip, state) is not None):
        return clip
    if concat_only:
        raise FileNotFoundError(f"Missing or stale rendered segment {clip}")
    config = render_config(media_dir)
    sidecar.unlink(missing_ok=True)
    cursor_trace.unlink(missing_ok=True)
    if require_trace:
        trace.unlink(missing_ok=True)
        clip.with_suffix(".trace.json").unlink(missing_ok=True)
    command = [*manim_command(), "--config_file", str(config), settings["quality"], "--media_dir", str(media_dir)]
    if settings["resolution"]:
        command.extend(["--resolution", settings["resolution"]])
    if settings["fps"]:
        command.extend(["--fps", str(settings["fps"])])
    command.extend(["--disable_caching", str(root / segment["script"]), segment["scene"]])
    print(f"[segment] Rendering {segment['scene']}", flush=True)
    log = media_dir / f"attempt-{time.time_ns()}.log" if log_output else None
    environment = {"SEMIO_TEACHING_OUTPUT_ROOT": str(output_root or media_dir.parent), "VO_TRACE": "1",
                   "SEMIO_TEACHING_SPEECH_ROOT": str(source_speech_root(root / segment["script"], language=settings["language"]).parent),
                   "SEMIO_TEACHING_LANGUAGE": settings["language"],
                   "SEMIO_TEACHING_AUDIENCE": settings["audience"]}
    run_command(command, cwd=root, env=environment, session=session, log=log)
    if segment_state(segment, settings) != state:
        raise RuntimeError(f"Authored sources changed while rendering {segment['scene']}")
    if not clip.is_file() or clip.stat().st_size <= 1000:
        raise FileNotFoundError(f"Manim did not create a complete segment {clip}")
    if require_trace:
        if not trace.is_file():
            raise FileNotFoundError(f"Missing narration trace for {segment['scene']}")
        marks = json.loads(trace.read_text(encoding="utf-8"))["beats"][segment["scene"]]
        if narration:
            attach_source_narration(clip, narration, marks, session=session)
            if segment_state(segment, settings) != state:
                raise RuntimeError(f"Authored inputs changed while attaching narration for {segment['scene']}")
        _write_state(clip.with_suffix(".trace.json"), {"source_state": state, "marks": marks})
    activity = json.loads(cursor_trace.read_text(encoding="utf-8")) if cursor_trace.is_file() else {"version": 1, "intervals": []}
    if not isinstance(activity, dict) or activity.get("version") != 1:
        raise ValueError(f"Invalid source cursor trace: {cursor_trace}")
    publish_cursor_intervals(clip, activity["intervals"])
    _write_state(sidecar, state)
    print(f"[segment] Complete {segment['scene']}", flush=True)
    return clip



def source_trace_path(segment: dict, output_root: Path) -> Path:
    """🗺️ Locate the canonical source scene's measured narration boundaries."""
    owner = (teaching_root(Path(__file__)) / segment["script"]).resolve().parent
    identity = hashlib.sha256(owner.relative_to(_WORKSPACE_ROOT).as_posix().encode()).hexdigest()[:10]
    return output_root / "trace" / identity / segment["scene"] / "🔣️.json"


def source_marks(clip: Path, state: dict) -> dict | None:
    """📌 Bind measured narration to the exact rendered source and quality."""
    try:
        trace = json.loads(clip.with_suffix(".trace.json").read_text(encoding="utf-8"))
        return trace["marks"] if trace["source_state"] == state and isinstance(trace["marks"], dict) else None
    except (OSError, ValueError, KeyError):
        return None


def clause_interval(marks: dict, *, first: str, last: str) -> tuple[float, float]:
    """✂️ Resolve inclusive semantic clause boundaries without fixed media timestamps."""
    try:
        ordered = sorted(marks, key=lambda key: float(marks[key]["start"]))
        bounds = [(float(marks[key]["start"]), float(marks[key]["window"])) for key in ordered]
        if any(not math.isfinite(start) or not math.isfinite(window) or start < 0 or window <= 0 for start, window in bounds):
            raise ValueError("Invalid narration timing")
        if any(start + window > next_start + 0.001 for (start, window), (next_start, _) in zip(bounds, bounds[1:])):
            raise ValueError("Overlapping narration timings")
        begin, end = ordered.index(first), ordered.index(last)
        if begin > end:
            raise ValueError("Reversed narration range")
        return bounds[begin][0], round(sum(bounds[end]), 9)
    except (KeyError, TypeError, OverflowError) as error:
        raise ValueError("Invalid narration range") from error


def cut_clip(source: Path, output: Path, start: float, end: float, *, session: ToolSession | None = None) -> None:
    """🎞️ Cut video and optional audio on one measured clock and publish only complete output."""
    if not all(math.isfinite(value) for value in (start, end)) or not 0 <= start < end:
        raise ValueError("Invalid clip interval")
    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = output.with_suffix(".partial.mp4")
    run_command([*ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i", str(source), "-ss", f"{start:.9f}", "-t", f"{end - start:.9f}",
                 "-map", "0:v:0", "-map", "0:a?", "-c:v", "libx264", "-preset", "fast", "-crf", "18",
                 "-threads", "1", "-pix_fmt", "yuv420p", "-c:a", "aac", "-movflags", "+faststart", str(temporary)],
                session=session)
    if not temporary.is_file() or temporary.stat().st_size <= 1000:
        raise FileNotFoundError(f"FFmpeg did not create a complete excerpt {temporary}")
    temporary.replace(output)
    publish_cursor_intervals(output, clip_cursor_intervals(cursor_intervals(source), start, end))


def excerpt_segment(source: Path, segment: dict, settings: dict, *,
                    force: bool = False, concat_only: bool = False) -> Path:
    """🎯 Share a selected canonical sequence across every lesson that references it."""
    if "clauses" not in segment:
        return source
    trace = source.with_suffix(".trace.json")
    source_state = segment_state({key: segment[key] for key in ("script", "scene")}, settings)
    marks = source_marks(source, source_state)
    if marks is None:
        raise ValueError(f"Missing or stale measured narration for {source}")
    interval = clause_interval(marks, **segment["clauses"])
    identity = hashlib.sha256(json.dumps(segment["clauses"], sort_keys=True).encode()).hexdigest()[:10]
    output = source.with_name(f"{segment['scene']}.{identity}.mp4")
    state = {**segment_state(segment, settings), "interval": list(interval), "trace": hashlib.sha256(trace.read_bytes()).hexdigest(),
             "cursor": _cursor_hash(source)}
    if not force and reusable_clip(output, state) and output.with_suffix(".cursor.json").is_file():
        return output
    if concat_only:
        raise FileNotFoundError(f"Missing or stale rendered excerpt {output}")
    sidecar = output.with_suffix(".render.json")
    sidecar.unlink(missing_ok=True)
    print(f"[excerpt] Selecting {segment['scene']} {segment['clauses']}", flush=True)
    cut_clip(source, output, *interval)
    if hashlib.sha256(trace.read_bytes()).hexdigest() != state["trace"] or _cursor_hash(source) != state["cursor"] or segment_state(segment, settings) != {key: state[key] for key in ("settings", "sources", "segment")}:
        raise RuntimeError(f"Authored inputs changed while selecting {segment['scene']}")
    _write_state(sidecar, state)
    return output


def _logical_state(scene: str, segments: list[dict], settings: dict, cursor_sources: list[Path] | None = None) -> dict:
    """📒 Describe a chapter by ordered sources and the activity metadata it projects."""
    state = {"scene": scene, "settings": settings, "segments": [segment_state(segment, settings) for segment in segments]}
    if cursor_sources is not None:
        state["cursor"] = [_cursor_hash(source) for source in cursor_sources]
    return state


def presentation_state(video_id: str, settings: dict, language: str) -> dict:
    """🪟 Bind a lesson envelope to its explicit locale and complete presentation sources."""
    presentation = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️")
    catalog_path = Path(_presentation_hierarchy.__file__).resolve().parent.parent / "🔣️.json"
    sources = tuple(path for path in presentation.source_files() if path.resolve() != catalog_path)
    return {"video": video_id, "language": language, "catalog": _presentation_hierarchy.hierarchy_contract(),
            **clip_state(settings, sources)}


def render_presentation(video_id: str, content: Path, output: Path, settings: dict, *, session: ToolSession | None = None) -> Path:
    """🖥️ Route every user-facing movie through the virtual lesson window compositor."""
    presentation = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️")
    return presentation.render_presentation(video_id, content, output, settings, session=session)


def state_inputs_current(value) -> bool:
    """🔒 Recheck nested authored dependency hashes before publishing a presentation."""
    if isinstance(value, list):
        return all(state_inputs_current(item) for item in value)
    if not isinstance(value, dict):
        return True
    try:
        if any(hashlib.sha256(Path(path).read_bytes()).hexdigest() != digest for path, digest in value.get("sources", {}).items()):
            return False
    except OSError:
        return False
    return all(state_inputs_current(item) for key, item in value.items() if key != "sources")


@_presentation_hierarchy.capture_catalog()
def publish_presentation(video_id: str, content: Path, output: Path, content_state: dict,
                         settings: dict, language: str, *, force: bool = False) -> Path:
    """📺 Validate a candidate before replacing the published movie and its certificate."""
    state = {"content": content_state, "cursor": _cursor_hash(content), "presentation": presentation_state(video_id, settings, language)}
    if not force and reusable_clip(output, state) and state_inputs_current(state):
        return output
    sidecar = output.with_suffix(".render.json")
    candidate = output.parent / f".vui-{time.time_ns():x}.mp4"
    session = ToolSession()
    try:
        render_presentation(video_id, content, candidate, {**settings, "language": language}, session=session)
        if presentation_state(video_id, settings, language) != state["presentation"] or _cursor_hash(content) != state["cursor"] or not reusable_clip(content, content_state) or not state_inputs_current(state):
            raise RuntimeError(f"Presentation inputs changed while rendering {video_id}")
        if not candidate.is_file() or candidate.stat().st_size <= 1000:
            raise FileNotFoundError(f"Missing complete presentation {candidate}")
        sidecar.unlink(missing_ok=True)
        candidate.replace(output)
        _write_state(sidecar, state)
    except BaseException:
        session.cancel()
        raise
    finally:
        candidate.unlink(missing_ok=True)
    return output


@_presentation_hierarchy.capture_catalog()
def compose_videos(video_ids: list[str] | None = None, *, language: str, audience: str, quality_flag: str = "-ql", play: bool = False,
                   output_root: Path | None = None, media_dir: Path | None = None,
                   output_dir: Path | None = None, force: bool = False, concat_only: bool = False,
                   resolution: str | None = None, fps: float | None = None, jobs: int = 1,
                   log_output: bool = False, chapter_scenes: list[str] | None = None) -> dict[str, Path]:
    """🎥 Render a globally bounded segment plan, then assemble ordered chapters and lessons."""
    _audience.subtitles_visible(audience)
    if language not in {"en", "de"} or jobs < 1 or quality_flag not in QUALITY_FOLDERS or (force and concat_only):
        raise ValueError("Invalid render settings")
    if fps is not None and (not math.isfinite(fps) or fps <= 0):
        raise ValueError("Render frame rate must be finite and positive")
    if resolution:
        parts = resolution.split(",")
        if len(parts) != 2 or not all(part.isdigit() and int(part) > 0 and int(part) % 2 == 0 for part in parts) or abs(int(parts[0]) / int(parts[1]) - 16 / 9) > 0.01:
            raise ValueError("Render resolution must be two positive even 16:9 dimensions")
    root = teaching_root(Path(__file__))
    videos = load_manifest(source_ids=video_ids or None)
    selected = set(video_ids or [video["id"] for video in videos])
    if selected - {video["id"] for video in videos} or (media_dir and len(selected) != 1):
        raise ValueError("Invalid selected videos or explicit media directory")
    output_root = Path(output_root or _output.output_root()).resolve()
    output_root = output_root / language / audience
    media_dir = Path(media_dir).resolve() / language / audience if media_dir else None
    segment_root = media_dir / "s" if media_dir else output_root / "s"
    destination = Path(output_dir or (Path(media_dir) / "rendered" if media_dir else output_root / "rendered")).resolve()
    settings = {"language": language, "quality": quality_flag, "resolution": resolution, "fps": fps, "audience": audience}
    selected_chapters = set(chapter_scenes or [])
    available_chapters = {chapter["scene"] for video in videos if video["id"] in selected for chapter in video["chapters"]}
    if selected_chapters - available_chapters:
        raise ValueError("Unknown chapters for selected lessons")
    plan = segment_plan(videos)
    groups, tasks = [], []
    for index, video in enumerate(videos, 1):
        if video["id"] not in selected:
            continue
        logical_dir = Path(media_dir).resolve() if media_dir else output_root / str(index)
        _write_state(logical_dir / "settings.json", settings)
        chapters = video["chapters"]
        for chapter_index, chapter in enumerate(chapters):
            if selected_chapters and chapter["scene"] not in selected_chapters:
                continue
            clip = logical_dir / "v" / f"{chapter['scene']}.mp4"
            entries = [job for job in plan if job["video_index"] == index and job["chapter_index"] == chapter_index]
            sources = [segment_root / str(job["source_ordinal"]) / "v" / f"{job['segment']['scene']}.mp4" for job in entries]
            state = _logical_state(chapter["scene"], chapter["segments"], settings, sources)
            groups.append({"video": video, "directory": logical_dir, "clip": clip, "sources": sources,
                           "chapter": chapter, "state": state, "entries": entries})
            if force or not reusable_clip(clip, state) or not clip.with_suffix(".cursor.json").is_file():
                tasks.extend(entries)

    tasks = source_plan(tasks)
    trace_root = Path(media_dir or output_root)
    partial_sources = {job["source_ordinal"] for job in plan if "clauses" in job["segment"]}

    def render(job, session):
        return render_segment(job["segment"], segment_root / str(job["source_ordinal"]), settings, force=force,
                              concat_only=concat_only, session=session, output_root=trace_root,
                              log_output=log_output, require_trace=job["source_ordinal"] in partial_sources)

    _, failures = execute_jobs(tasks, render, max_workers=jobs)
    errors = [f"{tasks[index]['segment']['scene']}: {error}" for index, error in failures.items()]
    excerpts = {}

    def selected_clip(source, segment):
        identity = json.dumps(segment, sort_keys=True)
        if identity not in excerpts:
            excerpts[identity] = excerpt_segment(source, segment, settings, force=force, concat_only=concat_only)
        return excerpts[identity]

    for group in groups:
        authored = _logical_state(group["chapter"]["scene"], group["chapter"]["segments"], settings)
        if authored != {key: value for key, value in group["state"].items() if key != "cursor"}:
            errors.append(f"Authored inputs changed before assembling {group['chapter']['scene']}")
            continue
        group["state"] = _logical_state(group["chapter"]["scene"], group["chapter"]["segments"], settings, group["sources"])
        if not force and reusable_clip(group["clip"], group["state"]) and group["clip"].with_suffix(".cursor.json").is_file():
            continue
        sources = group["sources"]
        if not all(reusable_clip(clip, segment_state({key: job["segment"][key] for key in ("script", "scene")}, settings)) for clip, job in zip(sources, group["entries"], strict=True)):
            continue
        try:
            clips = [selected_clip(source, job["segment"])
                     for source, job in zip(sources, group["entries"], strict=True)]
        except (OSError, ValueError, KeyError, RuntimeError, subprocess.SubprocessError) as error:
            errors.append(f"{group['chapter']['scene']}: {error}")
            continue
        sidecar = group["clip"].with_suffix(".render.json")
        sidecar.unlink(missing_ok=True)
        if len(clips) == 1:
            group["clip"].parent.mkdir(parents=True, exist_ok=True)
            temporary = group["clip"].with_suffix(".partial.mp4")
            shutil.copy2(clips[0], temporary)
            temporary.replace(group["clip"])
            publish_cursor_intervals(group["clip"], cursor_intervals(clips[0]))
        else:
            concat_videos(clips, group["clip"], group["directory"] / f"{group['chapter']['scene']}.txt")
        if _logical_state(group["chapter"]["scene"], group["chapter"]["segments"], settings, group["sources"]) != group["state"]:
            errors.append(f"Authored inputs changed while assembling {group['chapter']['scene']}")
            continue
        _write_state(sidecar, group["state"])
        print(f"[chapter] Complete {group['chapter']['scene']}", flush=True)

    if selected_chapters:
        if errors:
            raise RuntimeError("Render failures; successful sources remain reusable:\n" + "\n".join(errors))
        if not all(reusable_clip(group["clip"], group["state"]) for group in groups):
            raise RuntimeError("Incomplete selected chapters")
        return {group["chapter"]["scene"]: publish_presentation(
            group["video"]["id"], group["clip"], destination / f"{group['chapter']['scene']}_{audience}_{language}.mp4",
            group["state"], settings, language, force=force,
        ) for group in groups}
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
        content = chapters[0]["directory"] / "content.mp4"
        output = destination / f"{video['output']}_{audience}_{language}_{dimensions}p{rate:g}.mp4"
        state = {"video": video["id"], "settings": settings, "chapters": [group["state"] for group in chapters],
                 "cursor": [_cursor_hash(group["clip"]) for group in chapters]}
        if force or not reusable_clip(content, state) or not content.with_suffix(".cursor.json").is_file():
            content.with_suffix(".render.json").unlink(missing_ok=True)
            concat_videos([group["clip"] for group in chapters], content, chapters[0]["directory"] / "concat.txt")
            if any(_logical_state(group["chapter"]["scene"], group["chapter"]["segments"], settings, group["sources"]) != group["state"] for group in chapters) or [_cursor_hash(group["clip"]) for group in chapters] != state["cursor"]:
                errors.append(f"Authored inputs changed while assembling {video['id']}")
                continue
            _write_state(content.with_suffix(".render.json"), state)
        publish_presentation(video["id"], content, output, state, settings, language, force=force)
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
