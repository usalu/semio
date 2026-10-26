"""🔊 Teaching TTS — speech generated per subtitle clause and placed on the video's own clock.

The voiceover reads the German subtitles that are already on screen, so the two
can only drift if the audio is treated as one opaque block. Every clause is
therefore synthesized, measured and positioned separately:

1. ``synthesize_clause_audio`` renders one file per NARRATION clause.
2. ``write_vo_timing_manifest`` feeds the measured lengths back to ``hold_for``.
3. Rendering with ``VO_TRACE=1`` records when each subtitle appears.
4. ``assemble_aligned_track`` lays each clause's speech on that exact timestamp.
"""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

import json
import os
import re
import shutil
import subprocess
import sys
import time
import urllib.error
import urllib.request
import wave
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Any, Optional

_video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
concat_videos = _video_module.concat_videos
ffmpeg_command = _video_module.ffmpeg_command
manim_command = _video_module.manim_command
run_command = _video_module.run_command

Clause = tuple[str, str, str]

_GEMINI_MODEL = "google/gemini-3.1-flash-tts-preview"
_ENV_FILE = _WORKSPACE_ROOT / "🎓️teaching" / ".env"  # teaching/.env — gitignored


def _clause_workers() -> int:
    """🧵 Parallel clause requests. Keep low for OpenAI TTS (RPM caps); override with TTS_CLAUSE_WORKERS."""
    try:
        return max(1, int(os.getenv("TTS_CLAUSE_WORKERS", "2")))
    except ValueError:
        return 2


def _load_dotenv_file(path: Path) -> None:
    """🔑 Minimal ``KEY=VALUE`` loader for ``teaching/.env`` — no python-dotenv dependency.

    Blank lines and ``#`` comments are skipped; surrounding quotes are stripped;
    a real shell environment variable always wins (``setdefault``).
    """
    if not path.is_file():
        return
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, _, value = line.partition("=")
        key = key.strip()
        if key.startswith("export "):
            key = key[len("export "):].strip()
        value = value.strip().strip('"').strip("'")
        if key and value:
            os.environ.setdefault(key, value)


def configure_tutorial_tts() -> None:
    """⚙️ Load teaching/.env while preserving explicit shell settings."""
    _load_dotenv_file(_ENV_FILE)
    os.environ["TTS_MODEL"] = os.getenv("TUTORIAL_TTS_MODEL") or os.getenv("TTS_MODEL") or _GEMINI_MODEL
    os.environ["TTS_BASE_URL"] = os.getenv("TUTORIAL_TTS_BASE_URL") or os.getenv("TTS_BASE_URL") or "https://openrouter.ai/api/v1"
    if not (os.getenv("TTS_API_KEY") or "").strip() and os.getenv("OPENROUTER_API_KEY"):
        os.environ["TTS_API_KEY"] = os.environ["OPENROUTER_API_KEY"]


def subtitle_clauses(narration: list[Clause]) -> list[str]:
    """💬 One German subtitle line per clause — empty entries stay silent staging."""
    return [text_de.strip() for _, _, text_de in narration]


def subtitle_narration_text(narration: list[Clause], key: str | None = None) -> str:
    """📝 Join subtitle clauses into one spoken string."""
    return " ".join(
        text_de for section, _, text_de in narration
        if text_de.strip() and (key is None or section == key)
    )


def probe_duration(path: Path) -> float:
    """⏱️ Measure WAV with stdlib and other media through provisioned FFmpeg tools."""
    path = Path(path)
    if not path.is_file():
        return 0.0
    if path.suffix.lower() == ".wav":
        try:
            with wave.open(str(path), "rb") as audio:
                return audio.getnframes() / float(audio.getframerate())
        except (wave.Error, EOFError, ZeroDivisionError):
            pass
    probe = os.getenv("SEMIO_FFPROBE_BINARY") or shutil.which("ffprobe")
    if probe:
        process = subprocess.run(
            [probe, "-v", "error", "-show_entries", "format=duration", "-of", "default=nw=1:nk=1", str(path)],
            capture_output=True, text=True, timeout=30, check=False,
        )
        try:
            return float(process.stdout.strip())
        except ValueError:
            pass
    process = subprocess.run(
        [*ffmpeg_command(), "-hide_banner", "-i", str(path)],
        capture_output=True, text=True, timeout=30, check=False,
    )
    match = re.search(r"Duration:\s*(\d+):(\d+):(\d+(?:\.\d+)?)", process.stderr)
    if not match:
        raise RuntimeError(f"Unable to measure media duration: {path}")
    hours, minutes, seconds = match.groups()
    return int(hours) * 3600 + int(minutes) * 60 + float(seconds)


def assemble_aligned_track(
    placements: list[tuple[Path, float]],
    output_path: Path,
    *,
    total_duration: float,
    min_gap: float = 0.08,
) -> Optional[Path]:
    """🧩 Mix each clause's speech in at its own start time, padded to ``total_duration``.

    Starts are sorted and clamped so one clause never overlays the previous
    one's speech (bad ``hold_for(..., used=...)`` backdating used to cause
    double-VO). Clamped speech may begin slightly after its subtitle; that is
    preferable to two voices at once.
    """
    usable = [(Path(p), max(0.0, float(t))) for p, t in placements if Path(p).is_file()]
    if not usable:
        return None
    usable.sort(key=lambda item: item[1])
    clamped: list[tuple[Path, float]] = []
    cursor = 0.0
    for path, start in usable:
        start = max(start, cursor)
        clamped.append((path, start))
        cursor = start + probe_duration(path) + min_gap
    usable = clamped

    output_path.parent.mkdir(parents=True, exist_ok=True)
    total = max(total_duration, max(start for _, start in usable) + 0.5, cursor)

    inputs: list[str] = []
    chains: list[str] = []
    labels: list[str] = []
    for i, (path, start) in enumerate(usable):
        inputs += ["-i", str(path)]
        delay_ms = int(round(start * 1000))
        chains.append(
            f"[{i}:a]aresample=44100,aformat=sample_fmts=fltp:channel_layouts=mono,"
            f"adelay={delay_ms}|{delay_ms}[a{i}]"
        )
        labels.append(f"[a{i}]")
    mix = (
        f"{''.join(labels)}amix=inputs={len(usable)}:normalize=0:dropout_transition=0[m];"
        f"[m]apad=whole_dur={total:.3f},atrim=0:{total:.3f}[out]"
    )
    filter_complex = ";".join(chains + [mix])
    cmd = [
        *ffmpeg_command(), "-y", *inputs,
        "-filter_complex", filter_complex,
        "-map", "[out]", "-c:a", "libmp3lame", "-q:a", "3",
        str(output_path),
    ]
    run_command(cmd)
    if not output_path.is_file():
        raise RuntimeError("ffmpeg did not create the aligned track")
    return output_path


class _SpeechBackend:
    """🌐 Optional speech synthesis through an explicitly configured HTTP endpoint."""

    def __init__(self) -> None:
        self.api_key = (os.getenv("TTS_API_KEY") or os.getenv("OPENROUTER_API_KEY") or "").strip()
        self.base_url = (os.getenv("TTS_BASE_URL") or "https://openrouter.ai/api/v1").rstrip("/")
        self.model = os.getenv("TTS_MODEL") or _GEMINI_MODEL
        self.voice = os.getenv("TTS_VOICE") or "Kore"

    def synthesize(self, text: str, output_path: Path, *, voice: str | None = None) -> tuple[Optional[str], bool]:
        if not self.api_key or not text.strip():
            return None, True
        use_pcm = "gemini" in self.model.lower() and "tts" in self.model.lower()
        payload: dict[str, Any] = {
            "model": self.model,
            "input": text.strip(),
            "voice": voice or self.voice,
            "response_format": "pcm" if use_pcm else "mp3",
        }
        instructions = (os.getenv("TTS_INSTRUCTIONS") or "").strip()
        if instructions and "gpt-4o" in self.model.lower():
            payload["instructions"] = instructions
        speed = (os.getenv("TTS_SPEED") or "").strip()
        if speed:
            payload["speed"] = float(speed)
        headers = {"Authorization": f"Bearer {self.api_key}", "Content-Type": "application/json"}
        for variable, header in (("OPENROUTER_SITE_URL", "HTTP-Referer"), ("OPENROUTER_APP_NAME", "X-Title")):
            if os.getenv(variable):
                headers[header] = os.environ[variable]
        request = urllib.request.Request(
            f"{self.base_url}/audio/speech", data=json.dumps(payload).encode("utf-8"), headers=headers, method="POST",
        )
        for attempt in range(6):
            try:
                with urllib.request.urlopen(request, timeout=120) as response:
                    content = response.read()
                    content_type = response.headers.get("Content-Type", "")
                break
            except urllib.error.HTTPError as error:
                if attempt == 5 or (error.code != 429 and error.code < 500):
                    raise RuntimeError(f"TTS request failed with HTTP {error.code}") from error
                wait = error.headers.get("Retry-After", "")
                delay = float(wait) if wait.replace(".", "", 1).isdigit() else 2.0 * (attempt + 1)
                time.sleep(min(delay, 30.0))
        if not content:
            raise RuntimeError("TTS endpoint returned no audio")
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        if use_pcm:
            output_path = output_path.with_suffix(".wav")
            rate = re.search(r"rate=(\d+)", content_type, re.I)
            channels = re.search(r"channels=(\d+)", content_type, re.I)
            with wave.open(str(output_path), "wb") as audio:
                audio.setnchannels(int(channels.group(1)) if channels else 1)
                audio.setsampwidth(2)
                audio.setframerate(int(rate.group(1)) if rate else 24000)
                audio.writeframes(content)
        else:
            output_path = output_path.with_suffix(".mp3")
            output_path.write_bytes(content)
        return str(output_path), False


def resolve_backends() -> list[_SpeechBackend]:
    """🔌 Resolve the optional endpoint without importing another project."""
    configure_tutorial_tts()
    return [_SpeechBackend()]


def synthesize_narration(text: str, output_path: Path, *, voice: str | None = None) -> tuple[Optional[str], bool]:
    """🎤 Synthesize one narration, or skip when credentials or text are absent."""
    return resolve_backends()[0].synthesize(text, output_path, voice=voice)


def synthesize_clause_audio(
    narration: list[Clause],
    clause_dir: Path,
    *,
    voice: str | None = None,
) -> list[tuple[str, Optional[Path], float]]:
    """🎙️ One audio file per subtitle clause, returned with its measured length."""
    backends = resolve_backends()
    if not any(backend.api_key for backend in backends):
        return [(section, None, 0.0) for section, _, _ in narration]
    clause_dir.mkdir(parents=True, exist_ok=True)
    texts = subtitle_clauses(narration)

    def _render(item: tuple[int, str]) -> tuple[int, Optional[Path]]:
        index, text = item
        if not text:
            return index, None
        target = clause_dir / f"clause_{index:02d}"
        last_error: Exception | None = None
        for backend in backends:
            try:
                path, skipped = backend.synthesize(text, target, voice=voice)
            except Exception as exc:  # noqa: BLE001
                last_error = exc
                continue
            if not skipped and path:
                return index, Path(path)
        if last_error:
            print(f"  ! clause {index}: {last_error}", file=sys.stderr)
        return index, None

    spoken = [(i, t) for i, t in enumerate(texts) if t]
    with ThreadPoolExecutor(max_workers=_clause_workers()) as pool:
        rendered = dict(pool.map(_render, spoken))

    results: list[tuple[str, Optional[Path], float]] = []
    for index, (section, _, _) in enumerate(narration):
        path = rendered.get(index)
        results.append((section, path, probe_duration(path) if path else 0.0))
    return results


def write_vo_timing_manifest(
    beats: list[tuple[str, list[tuple[str, Optional[Path], float]]]],
    path: Path,
    *,
    merge_existing: bool = True,
) -> None:
    """📋 Beat class → clause key → measured seconds, read back by ``hold_for``."""
    payload: dict[str, Any] = {"source": "subtitle", "beats": {}}
    if merge_existing and path.is_file():
        try:
            existing = json.loads(path.read_text(encoding="utf-8"))
            if isinstance(existing.get("beats"), dict):
                payload["beats"] = dict(existing["beats"])
        except Exception:  # noqa: BLE001
            pass
    for beat_name, clauses in beats:
        payload["beats"][beat_name] = {
            section: round(seconds, 3) for section, _, seconds in clauses if seconds > 0
        }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def read_vo_trace(path: Path) -> dict[str, dict[str, dict[str, float]]]:
    """📍 Subtitle appearance times captured by rendering with ``VO_TRACE=1``."""
    path = Path(path)
    if path.is_dir():
        return {beat: marks for entry in sorted(path.glob("*/🔣️.json")) for beat, marks in read_vo_trace(entry).items()}
    if not path.is_file():
        return {}
    data = json.loads(path.read_text(encoding="utf-8"))
    beats = data.get("beats") if isinstance(data, dict) else None
    return beats if isinstance(beats, dict) else {}


def mux_scene_audio(video_path: Path | str, audio_path: Path | str | None, output_path: Path, *, fps: int = 60) -> str:
    """🔊 Normalize a scene and optional narration without truncating either stream."""
    video = Path(video_path)
    if not video.is_file():
        raise FileNotFoundError(video)
    audio = Path(audio_path) if audio_path else None
    if audio is not None and not audio.is_file():
        audio = None
    video_seconds = probe_duration(video)
    target = max(video_seconds, probe_duration(audio) if audio else 0.0, 0.5) + (0.15 if audio else 0.0)
    video_filter = f"[0:v]fps={fps},tpad=stop_mode=clone:stop_duration={max(0.0, target - video_seconds):.3f},format=yuv420p[v]"
    if audio:
        inputs = ["-i", str(video), "-i", str(audio)]
        audio_filter = f"[1:a]aformat=sample_fmts=fltp:sample_rates=44100:channel_layouts=stereo,apad=whole_dur={target:.3f},atrim=0:{target:.3f}[a]"
    else:
        inputs = ["-i", str(video)]
        audio_filter = f"anullsrc=channel_layout=stereo:sample_rate=44100,atrim=0:{target:.3f}[a]"
    output_path = Path(output_path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    run_command([
        *ffmpeg_command(), "-y", *inputs, "-filter_complex", f"{video_filter};{audio_filter}",
        "-map", "[v]", "-map", "[a]", "-c:v", "libx264", "-preset", "veryfast", "-crf", "18",
        "-c:a", "aac", "-b:a", "192k", "-ar", "44100", "-movflags", "+faststart", str(output_path),
    ])
    return str(output_path)


def compose_final_video(clips: list[Path | str], output_path: Path) -> str:
    """🎞️ Join normalized chapter clips in their authored order."""
    output_path = Path(output_path)
    concat_videos([Path(clip) for clip in clips], output_path, output_path.with_suffix(".concat.txt"), keep_audio=True)
    return str(output_path)


def build_section_video(base_dir: Path, scenes: list[tuple[Path, str, Path]], output_name: str) -> Path:
    """🎬 Render every section beat and preserve its complete optional narration."""
    from hashlib import sha256
    _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
    render_config = _video_module.render_config
    teaching_root = _video_module.teaching_root

    root = teaching_root(base_dir)
    section_id = sha256(base_dir.relative_to(root).as_posix().encode("utf-8")).hexdigest()[:10]
    output_root = os.getenv("SEMIO_TEACHING_OUTPUT_ROOT")
    output_dir = Path(output_root).resolve() / f"b{section_id}" if output_root else base_dir / "rendered" / "hq"
    media_dir = output_dir / "m"
    config = render_config(media_dir)
    clips = []
    for index, (script, scene, audio) in enumerate(scenes, 1):
        print(f"Rendering {scene} ({index}/{len(scenes)})", flush=True)
        name = f"b{index:02d}"
        run_command([
            *manim_command(), "-qh", "--disable_caching", "--config_file", str(config), "-o", name, str(script), scene,
        ], cwd=base_dir)
        rendered = media_dir / "v" / f"{name}.mp4"
        if not rendered.is_file():
            raise FileNotFoundError(f"Rendered clip not found: {rendered}")
        audio_path = next((path for path in (audio, audio.with_suffix(".wav")) if path.is_file()), None)
        clips.append(mux_scene_audio(rendered, audio_path, output_dir / f"{name}.mp4"))
    output = output_dir / output_name
    compose_final_video(clips, output)
    print(f"Composed {output}", flush=True)
    return output
