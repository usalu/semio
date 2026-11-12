"""🎞️ Playback of rendered teaching media: explicit profile selection from export provenance and exact frame access by index.

A player never guesses a path or a profile. It names the profile it wants, `open_media` proves from the `.export.json` provenance that the movie was produced under exactly that profile and fingerprint, and `Media.label` is what the player must show so preview media is never presented as final. Frame access is addressed by integer output frame index; every seek strategy (forward, reverse, random) resolves to the same decoded frame, and identity at a frame comes from the shared timeline evaluator.

@see ../📤️export/🐍️.py — provenance (`require_record_profile`) and stills
@see ../🎚️profile/🐍️.py — profile registry and fingerprints
@see ../../../../🔀️morph/⏱️timeline/🐍️.py — frame ownership at the terminal sample
"""

from __future__ import annotations

import importlib
import random
from dataclasses import dataclass
from pathlib import Path

_PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
_profile = importlib.import_module(_PACKAGE + ".🎚️profile.🐍️")
_export = importlib.import_module(_PACKAGE + ".📤️export.🐍️")
_timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")

LABELS = {"en": {"preview": "Preview", "final": "Final"}, "de": {"preview": "Vorschau", "final": "Final"}}
ORDERS = ("forward", "reverse", "random")


@dataclass(frozen=True)
class Media:
    """🎬 One rendered movie proven to belong to `profile`; `poster` and `thumbnail` come from the same provenance."""

    path: Path
    profile: str
    fingerprint: str
    kind: str
    language: str
    audience: str
    width: int
    height: int
    fps: float
    poster: Path | None
    thumbnail: Path | None

    def label(self, language: str) -> str:
        """🏷️ The visible profile badge in an explicitly chosen language."""
        if language not in LABELS:
            raise ValueError(f"A badge language is required: {sorted(LABELS)}")
        return LABELS[language][self.profile]


def locate(root: Path, profile: str, language: str, audience: str, *parts: str) -> Path:
    """📍 Resolve a path in the profile-first output layout `<root>/<profile>/<language>/<audience>/...`; unknown profiles fail, nothing falls back."""
    _profile.profile(profile)
    return Path(root, profile, language, audience, *parts)


def open_media(artifact: Path, profile: str) -> Media:
    """🔓 Open a movie for exactly `profile`, refusing missing provenance, another profile, a stale fingerprint or a modified file."""
    record = _export.require_record_profile(Path(artifact), profile)
    render, stills = record["render"], record.get("stills")
    folder = Path(artifact).parent
    pick = lambda key: folder / stills[key]["file"] if stills else None
    return Media(Path(artifact), profile, record["profile"]["fingerprint"], record["kind"], record["language"], record["audience"],
                 int(render["width"]), int(render["height"]), float(render["fps"]), pick("poster"), pick("thumbnail"))


def frame_index(pts: int, time_base, fps: float) -> int:
    """🔢 Output frame index of a presentation timestamp."""
    return _timeline.Schedule(fps).frame(float(pts * time_base))


def _decode(path: Path, start: int, stop: int):
    """🧵 Decode frames `start..stop` in stream order starting from the keyframe at or before `start`."""
    import av

    with av.open(str(path)) as container:
        stream = container.streams.video[0]
        fps = float(stream.average_rate)
        if start > 0:
            container.seek(int(max(0, (start - 0.5) / fps) / stream.time_base), stream=stream, backward=True, any_frame=False)
        for frame in container.decode(stream):
            index = frame_index(frame.pts, stream.time_base, fps)
            if index < start:
                continue
            if index > stop:
                return
            yield index, frame.to_ndarray(format="rgb24")


def frames(path: Path) -> dict[int, "numpy.ndarray"]:
    """▶️ Every frame by index from one forward pass."""
    return dict(_decode(Path(path), 0, 2**31))


def frame_at(path: Path, index: int) -> "numpy.ndarray":
    """🎯 One frame by index through a fresh seek, with no state from earlier calls."""
    for found, pixels in _decode(Path(path), index, index):
        return pixels
    raise IndexError(f"{path} has no frame {index}")


def sample(path: Path, indices, order: str = "forward", *, seed: int = 0) -> dict[int, "numpy.ndarray"]:
    """🔀 Decode `indices` forward (one pass), reverse (a seek per frame, descending) or random (a seek per frame, seeded shuffle)."""
    if order not in ORDERS:
        raise ValueError(f"Unknown order {order!r}; expected one of {list(ORDERS)}")
    wanted = sorted(set(indices))
    if order == "forward":
        everything = frames(path)
        return {index: everything[index] for index in wanted}
    sequence = list(reversed(wanted))
    if order == "random":
        random.Random(seed).shuffle(sequence)
    return {index: frame_at(path, index) for index in sequence}


def owner_at(index: int, *, start: int, count: int, fps: float) -> str:
    """🧭 Which identity the shared evaluator shows at output frame `index` of a transition occupying `count` frames from `start`."""
    window = _timeline.Window(start / fps, count / fps)
    return _timeline.owner(window, _timeline.Schedule(fps), index)
