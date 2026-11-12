"""🖼️ Posters and thumbnails taken from the canonical output frame schedule of one profile.

The poster frame is an integer index derived from the schedule (frame, time, fraction of the frame count, or the terminal sample of a declared transition), never the decoder's last delivered frame.

@see ../🧬️schema/🔣️.schema.json — `posterPolicy` and `stills`
@see ../../../../../🔀️morph/⏱️timeline/🐍️.py — `terminal`, the shared terminal-sample definition
"""

from __future__ import annotations

import importlib
import json
import math
import re
import tempfile
from pathlib import Path

_PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
_export = importlib.import_module(_PACKAGE + ".📤️export.🐍️")
_timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")

THUMBNAIL = (320, 180)
DEFAULT_POLICY = {"kind": "fraction", "value": 0.5}
TRANSITIONS_FILE = "transitions.json"


class StillError(ValueError):
    """🚫 A poster frame cannot be derived or extracted exactly."""


def poster_frame(policy: dict, *, frames: int, fps: float, windows: dict | None = None) -> int:
    """🎯 The frame index a poster policy selects on a schedule of `frames` frames at `fps`; windows are `{id: (start seconds, duration seconds)}` on the movie's own clock."""
    if isinstance(frames, bool) or not isinstance(frames, int) or frames < 1 or not math.isfinite(fps) or fps <= 0:
        raise StillError("A poster needs a positive frame count and frame rate")
    kind = policy.get("kind")
    schedule = _timeline.Schedule(fps)
    if kind == "frame":
        index = policy["index"]
    elif kind == "time":
        index = schedule.frame(policy["seconds"])
    elif kind == "fraction":
        value = policy["value"]
        if not 0 <= value <= 1:
            raise StillError("A poster fraction must lie in [0, 1]")
        index = min(frames - 1, math.floor(value * frames))
    elif kind == "transition":
        if not windows or policy["id"] not in windows:
            raise StillError(f"Transition {policy['id']} has no window on this movie's clock; export the scene's {TRANSITIONS_FILE} next to its work record")
        start, duration = windows[policy["id"]]
        index = _timeline.terminal(_timeline.Window(start, duration), schedule)
    else:
        raise StillError(f"Unknown poster policy {kind!r}; expected frame, time, fraction or transition")
    if not 0 <= index < frames:
        raise StillError(f"Poster frame {index} lies outside the {frames}-frame movie")
    return int(index)


def probe(video: Path) -> tuple[int, float]:
    """🔢 Frame count and frame rate from the video packets, without decoding."""
    runtime = importlib.import_module(_PACKAGE + ".⚙️runtime.🐍️")
    with tempfile.TemporaryDirectory(prefix="probe-") as directory:
        log = Path(directory) / "probe.log"
        runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-nostdin", "-i", str(video), "-map", "0:v:0", "-an", "-c", "copy", "-f", "null", "-"], log=log)
        text = log.read_text(encoding="utf-8")
    frames = re.findall(r"frame=\s*(\d+)", text)
    rate = re.search(r"Video:.*?([\d.]+) fps", text) or re.search(r"Video:.*?([\d.]+) tbr", text)
    if not frames or not rate or float(rate.group(1)) <= 0:
        raise StillError(f"Cannot measure the frame clock of {video}")
    return int(frames[-1]), float(rate.group(1))


def extract(video: Path, index: int, output: Path, *, size: tuple[int, int] | None = None) -> Path:
    """📸 Write frame `index` of `video` as a PNG through an accurate input seek, optionally scaled."""
    runtime = importlib.import_module(_PACKAGE + ".⚙️runtime.🐍️")
    frames, fps = probe(video)
    if not 0 <= index < frames:
        raise StillError(f"Frame {index} lies outside the {frames}-frame movie {video}")
    output = Path(output)
    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = output.with_name(output.stem + ".partial.png")
    filters = ["-vf", f"scale={size[0]}:{size[1]}:flags=lanczos"] if size else []
    seek = ["-ss", f"{max(index - 0.5, 0) / fps:.9f}"] if index else []
    try:
        runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", *seek, "-i", str(video), "-map", "0:v:0", "-an",
                             *filters, "-frames:v", "1", "-update", "1", str(temporary)])
        if not temporary.is_file() or temporary.stat().st_size <= 0:
            raise StillError(f"FFmpeg produced no frame {index} for {video}")
        temporary.replace(output)
    finally:
        temporary.unlink(missing_ok=True)
    return output


def scale(image: Path, output: Path, size: tuple[int, int]) -> Path:
    """📐 Deterministically resize one still."""
    runtime = importlib.import_module(_PACKAGE + ".⚙️runtime.🐍️")
    temporary = Path(output).with_name(Path(output).stem + ".partial.png")
    try:
        runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i", str(image),
                             "-vf", f"scale={size[0]}:{size[1]}:flags=lanczos", "-frames:v", "1", "-update", "1", str(temporary)])
        temporary.replace(output)
    finally:
        temporary.unlink(missing_ok=True)
    return Path(output)


def read_transitions(media_dir: Path) -> dict:
    """🔀 Transition windows `{id: (start, duration)}` in scene seconds a scene reported next to its work record."""
    path = Path(media_dir) / TRANSITIONS_FILE
    if not path.is_file():
        return {}
    return {item["id"]: (float(item["start"]), float(item["duration"])) for item in json.loads(path.read_text(encoding="utf-8"))["transitions"]}


def make_stills(video: Path, folder: Path, name: str, *, profile: str, policy: dict | None = None, windows: dict | None = None,
                size: tuple[int, int] | None = None) -> dict:
    """🖼️ Poster at the profile's output size and a thumbnail of the same frame, with their frame index, policy, profile and hashes."""
    video = Path(video)
    _export.require_profile(video, profile)
    chosen = _export.profile_ref(profile)
    policy = dict(policy or DEFAULT_POLICY)
    frames, fps = probe(video)
    index = poster_frame(policy, frames=frames, fps=fps, windows=windows)
    effective = _export.effective_render(profile)
    size = size or (effective["width"], effective["height"])
    folder = Path(folder)
    poster = extract(video, index, folder / f"{name}.poster.png", size=size)
    thumbnail = scale(poster, folder / f"{name}.thumb.png", THUMBNAIL)
    return {"profile": chosen, "policy": policy, "frame": index, "frames": frames, "fps": fps, "seconds": index / fps,
            "poster": {"file": poster.name, "width": size[0], "height": size[1], "sha256": _export.digest(poster)},
            "thumbnail": {"file": thumbnail.name, "width": THUMBNAIL[0], "height": THUMBNAIL[1], "sha256": _export.digest(thumbnail)}}


def stills_current(record: dict, video: Path, folder: Path, *, profile: str, policy: dict | None = None) -> bool:
    """♻️ A recorded pair is reusable only for the same profile, policy, files and an unchanged movie frame clock."""
    try:
        if record["profile"] != _export.profile_ref(profile) or record["policy"] != (policy or DEFAULT_POLICY):
            return False
        frames, fps = probe(Path(video))
        return (record["frames"], record["fps"]) == (frames, fps) and all(
            (Path(folder) / record[key]["file"]).is_file() and _export.digest(Path(folder) / record[key]["file"]) == record[key]["sha256"] for key in ("poster", "thumbnail"))
    except (KeyError, OSError, StillError):
        return False
