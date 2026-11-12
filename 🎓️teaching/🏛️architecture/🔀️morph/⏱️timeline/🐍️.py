"""⏱ Canonical transition timeline: frame ranges, terminal samples, identity ownership and easing, pure Python (NumPy only, no Manim).

Spec: ``teaching-morph-preview-launch-kit/acceptance.md`` (terminal sample definition), ticket ``CONTRACT.md`` section 4.
Oracle: ``manim.Scene.get_time_progression`` (``numpy.arange(0, run_time, 1 / fps)``) and ``manim.utils.rate_functions``.
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from fractions import Fraction
from itertools import accumulate
from typing import Callable

import numpy as np

TOLERANCE = 1e-9
BEFORE, SOURCE, TARGET = "before", "source", "target"


@dataclass(frozen=True)
class Schedule:
    """🎞 Output frame grid: frame `f` is shown at scene time `origin + f / fps`."""

    fps: float
    origin: float = 0.0

    def __post_init__(self):
        if not self.fps > 0:
            raise ValueError(f"fps must be positive, got {self.fps!r}")

    def frame(self, t):
        """🎯 Nearest frame index of scene time `t` (half up, robust to float noise)."""
        return math.floor((t - self.origin) * self.fps + 0.5 + TOLERANCE)

    def time(self, frame):
        """🕒 Scene time of `frame`."""
        return self.origin + frame / self.fps


@dataclass(frozen=True)
class Window:
    """🪟 Transition interval in scene seconds: `start` and canonical `duration` (0 is an atomic switch)."""

    start: float
    duration: float

    def __post_init__(self):
        if self.duration < 0:
            raise ValueError(f"duration must not be negative, got {self.duration!r}")

    @property
    def end(self):
        """🏁 Half-open end time `start + duration`; never sampled by a frame schedule."""
        return self.start + self.duration


@dataclass(frozen=True)
class Sample:
    """🔬 Identity owner (`before`, `source`, `target`) and clamped linear progress in 0..1."""

    owner: str
    progress: float


@dataclass(frozen=True)
class Link:
    """⛓ One window inside a chain: first frame `f0`, terminal frame `f1`, last owned frame `until` (None: open end)."""

    index: int
    f0: int
    f1: int
    until: int | None

    @property
    def terminal_owned(self):
        """🎯 False when the next window takes over the terminal frame (its source is this target)."""
        return self.until is None or self.f1 <= self.until


@dataclass(frozen=True)
class Resolved:
    """🧭 Chain evaluation: owning window `index` and its `sample`."""

    index: int
    sample: Sample


def exact(seconds):
    """🧮 Authored seconds as an exact fraction, snapped to a microsecond so float noise never moves a grid point."""
    return Fraction(seconds).limit_denominator(10**6)


def rate(fps):
    """🎞 The exact frame rate (29.97 stays 2997/100)."""
    return Fraction(float(fps)).limit_denominator(1001)


def grid(seconds, schedule):
    """📍 `ceil((seconds - origin) · fps)` in exact arithmetic: the first frame on the absolute grid at or after `seconds`."""
    return math.ceil((exact(seconds) - exact(schedule.origin)) * rate(schedule.fps))


def emitted(end, done, schedule):
    """📼 Frames emitted after a play or wait that ends at authored time `end` when `done` frames came before: `max(done + 1, ceil(end · fps))`. A sub-frame interval that would get 0 frames borrows one and the next interval gets that much fewer, so a run of plays totals `ceil(total · fps)` unless more plays than frames occur."""
    return max(done + 1, grid(end, schedule))


def count(window, schedule):
    """🔢 Frames a window occupies on the absolute grid: `ceil((start + T) · fps) - ceil(start · fps)`, the grid points in `[start, start + T)`, so any split of a clip adds up to `ceil(total · fps)`; at least one for `T > 0` (the borrowed frame is absorbed by the next interval, see `🧰️interop/🕐️clock`); 0 for T == 0."""
    if window.duration <= 0:
        return 0
    return max(grid(window.end, schedule) - grid(window.start, schedule), 1)


def first(window, schedule):
    """🥇 First grid frame at or after `window.start` (`ceil(start · fps)`), the atomic switch frame for zero duration."""
    return grid(window.start, schedule)


def frames(window, schedule):
    """📐 Inclusive `(f0, f1)` with `f1 = f0 + n - 1`; None for a zero-duration window."""
    n = count(window, schedule)
    if n == 0:
        return None
    f0 = first(window, schedule)
    return f0, f0 + n - 1


def terminal(window, schedule):
    """🎯 Frame that shows the target first: `f1`, or the atomic frame for zero duration."""
    span = frames(window, schedule)
    return first(window, schedule) if span is None else span[1]


def boundary(window, schedule):
    """⏳ Continuous boundary time `tb = origin + f1 / fps`; `t >= tb` is target."""
    return schedule.time(terminal(window, schedule))


def _clamp(value):
    return min(max(value, 0.0), 1.0)


def owner(window, schedule, frame):
    """👁 `before` (frame < f0), `source` (f0 <= frame < f1) or `target` (frame >= f1, one-frame and zero-duration included)."""
    if frame < first(window, schedule):
        return BEFORE
    return SOURCE if frame < terminal(window, schedule) else TARGET


def progress(window, schedule, frame):
    """📈 Linear `clamp((frame - f0) / (f1 - f0), 0, 1)`; 1 for one-frame and zero-duration windows from f0 on."""
    f0 = first(window, schedule)
    f1 = terminal(window, schedule)
    if frame < f0:
        return 0.0
    if f1 == f0:
        return 1.0
    return _clamp((frame - f0) / (f1 - f0))


def sample(window, schedule, frame):
    """🔬 `Sample(owner, progress)` of one frame; stateless, so trimmed, reversed and random-order evaluation agree."""
    return Sample(owner(window, schedule, frame), progress(window, schedule, frame))


def at(window, t, schedule=None):
    """⏲ Continuous seek: the sample at scene time `t`.

    With a `schedule` the window is snapped to its frame range, so `at(w, schedule.time(f), schedule) == sample(w, schedule, f)`
    and a player that skipped the terminal frame shows the target at the next evaluation after `tb`.
    Without one the boundary is the plain end `start + duration`.
    """
    if schedule is None:
        t0, tb = window.start, window.end
    else:
        t0, tb = schedule.time(first(window, schedule)), boundary(window, schedule)
    if t < t0 - TOLERANCE:
        return Sample(BEFORE, 0.0)
    if t >= tb - TOLERANCE:
        return Sample(TARGET, 1.0)
    return Sample(SOURCE, _clamp((t - t0) / (tb - t0)))


def frame_of_alpha(alpha, run_time, fps, phase=0.0):
    """🧮 Frame index inside a Manim animation from its raw `alpha`: `round(alpha * run_time * fps - phase)`, never from callback counts. `phase` is the play's offset in frames from its authored start to its first grid frame (`SCENE_CLOCK.phase`, 0 when aligned)."""
    return math.floor(alpha * run_time * fps - phase + 0.5 + TOLERANCE)


def sample_alpha(alpha, run_time, fps, phase=0.0):
    """🎬 Sample of a Manim `Animation.interpolate(alpha)` call (before `rate_func`); `alpha >= 1` from `finish()` is target. The window starts `phase` frames before the first sampled frame, so the last grid point inside the authored interval is the terminal sample."""
    return sample(Window(-phase / fps, run_time), Schedule(fps), frame_of_alpha(alpha, run_time, fps, phase))


def stagger_lag(run_time, size, lag, limit=None):
    """🐝 Effective per-element lag: `min(lag, max(limit - run_time, 0) / max(size - 1, 1))`, the cap of `morph._staggered`."""
    if limit is None:
        return lag
    return min(lag, max(limit - run_time, 0.0) / max(size - 1, 1))


def expand(window, size, lag, limit=None):
    """🐝 Deterministic staggered windows of `size` elements, each of the full `window.duration`.

    Element `i` starts `i * lag'` after `window.start` with `lag'` from `stagger_lag`; identical to the
    `LaggedStart(lag_ratio=lag' / T)` timings of Manim, so the group lasts `T + (size - 1) * lag'`.
    """
    if size < 1:
        return ()
    lag = stagger_lag(window.duration, size, lag, limit)
    ratio = lag / window.duration if window.duration else 0.0
    steps = [window.duration * ratio] * (size - 1)
    starts = [0.0, *accumulate(steps)]
    return tuple(Window(window.start + s, window.duration) for s in starts)


def span(windows):
    """📏 Enclosing window of staggered element windows: first start to latest end."""
    start = min(w.start for w in windows)
    return Window(start, max(w.end for w in windows) - start)


def element_progress(windows, group_progress):
    """🧩 Per-element linear progress for a group progress `p` in 0..1 (remapped so `p = 1` finishes every element exactly)."""
    group = span(windows)
    now = _clamp(group_progress) * group.duration
    return tuple(1.0 if w.duration == 0 else _clamp((now - (w.start - group.start)) / w.duration) for w in windows)


def chain(windows, schedule):
    """⛓ Links of adjacent transitions ordered by start: the later window owns every frame from its own `f0`."""
    starts = [first(w, schedule) for w in windows]
    if any(b < a for a, b in zip(starts, starts[1:])):
        raise ValueError("chained windows must be ordered by start")
    return tuple(Link(i, starts[i], terminal(w, schedule), starts[i + 1] - 1 if i + 1 < len(windows) else None) for i, w in enumerate(windows))


def resolve(windows, schedule, frame):
    """🧭 Owning window and sample of `frame` in a chain; None before the first window. Exactly one owner per frame."""
    owned = [link for link in chain(windows, schedule) if link.f0 <= frame]
    if not owned:
        return None
    index = owned[-1].index
    return Resolved(index, sample(windows[index], schedule, frame))


def resolve_at(windows, t, schedule=None):
    """🧭 Continuous-time `resolve`: the latest window whose start is at or before `t` owns it."""
    def begin(w):
        return w.start if schedule is None else schedule.time(first(w, schedule))

    if schedule is None and any(b.start < a.start for a, b in zip(windows, windows[1:])):
        raise ValueError("chained windows must be ordered by start")
    owned = [i for i, w in enumerate(windows) if begin(w) - TOLERANCE <= t]
    if not owned:
        return None
    return Resolved(owned[-1], at(windows[owned[-1]], t, schedule))


def _sigmoid(x):
    return 1.0 / (1 + np.exp(-x))


def _unit(function):
    def run(t):
        return 0.0 if t < 0 else 1.0 if t > 1 else function(t)

    return run


def _smooth(t, inflection=10.0):
    error = _sigmoid(-inflection / 2)
    return min(max((_sigmoid(inflection * (t - 0.5)) - error) / (1 - 2 * error), 0), 1)


def _there_and_back(t):
    return _smooth(2 * t if t < 0.5 else 2 * (1 - t))


def _there_and_back_with_pause(t, pause_ratio=1.0 / 3):
    a = 2.0 / (1.0 - pause_ratio)
    if t < 0.5 - pause_ratio / 2:
        return _smooth(a * t)
    if t < 0.5 + pause_ratio / 2:
        return 1
    return _smooth(a - a * t)


def _double_smooth(t):
    return 0.5 * _smooth(2 * t) if t < 0.5 else 0.5 * (1 + _smooth(2 * t - 1))


def _running_start(t, pull_factor=-0.5):
    mt = 1 - t
    return 15 * t**2 * mt**4 * pull_factor + 20 * t**3 * mt**3 * pull_factor + 15 * t**4 * mt**2 + 6 * t**5 * mt + t**6


def _ease_out_back(t):
    c1 = 1.70158
    return 1 + (c1 + 1) * pow(t - 1, 3) + c1 * pow(t - 1, 2)


def _ease_in_back(t):
    c1 = 1.70158
    return (c1 + 1) * t * t * t - c1 * t * t


EASINGS: dict[str, Callable[[float], float]] = {
    "linear": lambda t: t,
    "smooth": _smooth,
    "smoothstep": lambda t: 3 * t**2 - 2 * t**3,
    "smootherstep": lambda t: 6 * t**5 - 15 * t**4 + 10 * t**3,
    "double-smooth": _double_smooth,
    "there-and-back": _there_and_back,
    "there-and-back-with-pause": _there_and_back_with_pause,
    "rush-into": lambda t: 2 * _smooth(t / 2.0),
    "rush-from": lambda t: 2 * _smooth(t / 2.0 + 0.5) - 1,
    "slow-into": lambda t: float(np.sqrt(1 - (1 - t) * (1 - t))),
    "ease-in": lambda t: t**3,
    "ease-out": lambda t: 1 - pow(1 - t, 3),
    "ease-in-out": lambda t: 4 * t**3 if t < 0.5 else 1 - pow(-2 * t + 2, 3) / 2,
    "ease-in-quad": lambda t: t * t,
    "ease-out-quad": lambda t: 1 - (1 - t) * (1 - t),
    "overshoot": _ease_out_back,
    "ease-out-back": _ease_out_back,
    "ease-in-back": _ease_in_back,
    "running-start": _running_start,
}


def ease(name, p):
    """🌊 Eased placement progress for `p` clamped to 0..1; hyphen or underscore names (Manim `rate_func` names included).

    Easing only shapes translation/scale; identity ownership comes from `owner`, so an overshooting or early-1.0 curve never switches early.
    """
    key = name.replace("_", "-")
    if key not in EASINGS:
        raise ValueError(f"unknown easing {name!r}; known: {', '.join(sorted(EASINGS))}")
    return float(_unit(EASINGS[key])(_clamp(p)))
