"""🕐 The canonical scene clock: authored time is an exact fraction sum of durations, output frames sit on the absolute frame grid, and rendering, skipping, caching and trimming all share both."""
from __future__ import annotations

import importlib
import platform
from collections.abc import Callable
from fractions import Fraction

from manim import Scene, ValueTracker, Wait, config
from manim.renderer.cairo_renderer import CairoRenderer
from tqdm import tqdm

_timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")


class SceneClock:
    """⏱️ The scene time of the frame being drawn, in seconds: `now` is the frame being drawn, `origin` the first frame of the play that draws it (`None` between plays) and `phase` the frames from the play's authored start to that first frame (0 between plays); setting `now` directly serves tests and tools that drive a motion without a scene."""

    __slots__ = ("now", "origin", "phase")

    def __init__(self) -> None:
        self.now = 0.0
        self.origin = None
        self.phase = 0.0


SCENE_CLOCK = SceneClock()


class _State:
    """🧾 One scene's clock: `t` authored seconds (exact), `done` frames emitted, `now` the frame being drawn, `calls` frames drawn by the current play and `planned` its frame count."""

    __slots__ = ("t", "done", "now", "calls", "planned", "phase")

    def __init__(self) -> None:
        self.t, self.done, self.now, self.calls, self.planned, self.phase = Fraction(0), 0, 0, 0, None, 0.0

    def plan(self, duration: float, fps: float) -> int:
        if self.planned is None:
            end = self.t + _timeline.exact(duration)
            self.planned = _timeline.emitted(end, self.done, _timeline.Schedule(fps)) - self.done
        return self.planned


def frame_count(duration: float, fps: float, start: float = 0.0, done: int | None = None) -> int:
    """🧮 The frames of an interval of `duration` seconds that starts at authored `start` after `done` frames (default: `ceil(start · fps)`, no debt): `max(1, ceil((start + duration) · fps) - done)`. A clip's total is `ceil(total · fps)` however it is split."""
    schedule = _timeline.Schedule(fps)
    before = _timeline.grid(start, schedule) if done is None else done
    return _timeline.emitted(_timeline.exact(start) + _timeline.exact(duration), before, schedule) - before


def stopwatch(rate: float = 1.0) -> Callable[[], float]:
    """⏲️ A reader of `rate` times the canonical scene seconds since the play of its first reading began; it replaces `dt` accumulators, whose sum depends on how many frames a play rendered or skipped."""
    start = []

    def elapsed() -> float:
        now = SCENE_CLOCK.now
        if not start:
            start.append(now if SCENE_CLOCK.origin is None else SCENE_CLOCK.origin)
        return rate * (now - start[0])

    return elapsed


def scene_timer(rate: float = 1.0) -> ValueTracker:
    """⏱️ A tracker that follows a `stopwatch(rate)` while it is in the scene and freezes when its updaters are cleared."""
    tracker, watch = ValueTracker(0.0), stopwatch(rate)
    tracker.add_updater(lambda mob: mob.set_value(watch()))
    return tracker


def rate_integral(speed: float = 1.0) -> Callable[[float], float]:
    """📈 The phase `speed · ∫ rate dt` of a stream whose `rate` changes between plays, on a piecewise-linear rate track: within a play the rate runs linearly from the rate observed at the end of the previous play (the knot; a stream starts at rest, rate 0) to the current rate, so the phase at a play's end is `phase₀ + speed · (rate₀ + rate₁)/2 · duration` whether the play rendered every frame or was skipped; call it with the current rate from an updater."""
    track = {"play": None, "t0": 0.0, "r0": 0.0, "ph0": 0.0, "ph": 0.0, "r": 0.0}

    def phase(rate: float) -> float:
        play = SCENE_CLOCK.origin
        if play is None:
            return track["ph"]
        if track["play"] != play:
            track.update(play=play, t0=play, ph0=track["ph"], r0=track["r"])
        track["r"] = rate
        track["ph"] = track["ph0"] + speed * (track["r0"] + rate) / 2 * (SCENE_CLOCK.now - track["t0"])
        return track["ph"]

    return phase


def _state(scene: Scene) -> _State:
    return scene.__dict__.setdefault("_clock_state", _State())


def _seconds(scene: Scene, frames: int) -> float:
    return float(Fraction(frames) / _timeline.rate(scene.camera.frame_rate))


def scene_seconds(scene: Scene) -> float:
    """📍 The canonical scene time in seconds of the frame being drawn (`frame / fps`, rounded once)."""
    return _seconds(scene, _state(scene).now)


def _stop_wait(scene: Scene) -> bool:
    animations = scene.animations or []
    return len(animations) == 1 and isinstance(animations[0], Wait) and animations[0].stop_condition is not None


def _publish(scene: Scene) -> None:
    now = scene_seconds(scene)
    SCENE_CLOCK.now = now
    scene.renderer.time = now


def install() -> None:
    """🔌 Make every Manim scene draw, skip and cache its plays on the canonical clock; safe to call repeatedly."""
    if getattr(Scene, "_canonical_clock", False):
        return
    update_mobjects, update_to_time = Scene.update_mobjects, Scene.update_to_time
    play, freeze = CairoRenderer.play, CairoRenderer.freeze_current_frame
    progression = Scene.get_time_progression

    def clocked_update_mobjects(self, dt):
        _publish(self)
        return update_mobjects(self, dt)

    def clocked_update_to_time(self, t):
        state, fps = _state(self), self.camera.frame_rate
        whole = self.renderer.skip_animations and not _stop_wait(self)
        state.now = state.done + (state.plan(t, fps) if whole else state.calls)
        state.calls += 0 if whole else 1
        _publish(self)
        return update_to_time(self, t)

    def clocked_time_progression(self, run_time, description, n_iterations=None, override_skip_animations=False):
        if self.renderer.skip_animations and not override_skip_animations:
            return progression(self, run_time, description, n_iterations, override_skip_animations)
        state, fps = _state(self), self.camera.frame_rate
        count, rate, length = state.plan(run_time, fps), _timeline.rate(fps), _timeline.exact(run_time)

        def times():
            for k in range(count):
                yield float(min(Fraction(state.done + k) / rate - state.t, length))
            state.now = state.done + count

        return tqdm(times(), desc=description, total=count if n_iterations is None else n_iterations,
                    leave=config["progress_bar"] == "leave", ascii=True if platform.system() == "Windows" else None,
                    disable=config["progress_bar"] == "none")

    def clocked_play(self, scene, *args, **kwargs):
        state = _state(scene)
        state.now, state.calls, state.planned = state.done, 0, None
        state.phase = float(state.done - state.t * _timeline.rate(scene.camera.frame_rate))
        SCENE_CLOCK.origin, SCENE_CLOCK.phase = _seconds(scene, state.done), state.phase
        self._clock_state = state
        try:
            result = play(self, scene, *args, **kwargs)
            frames = state.calls if _stop_wait(scene) else state.plan(scene.duration, scene.camera.frame_rate)
            state.t = Fraction(state.done + frames) / _timeline.rate(scene.camera.frame_rate) if _stop_wait(scene) else state.t + _timeline.exact(scene.duration)
            state.done += frames
            return result
        finally:
            SCENE_CLOCK.origin, SCENE_CLOCK.phase = None, 0.0
            state.now, state.planned = state.done, None
            _publish(scene)

    def clocked_freeze(self, duration):
        fps = self.camera.frame_rate
        self.add_frame(self.get_frame(), num_frames=self._clock_state.plan(duration, fps) if hasattr(self, "_clock_state") else frame_count(duration, fps))

    Scene.update_mobjects = clocked_update_mobjects
    Scene.update_to_time = clocked_update_to_time
    Scene.get_time_progression = clocked_time_progression
    CairoRenderer.play = clocked_play
    CairoRenderer.freeze_current_frame = clocked_freeze
    Scene._canonical_clock = True


install()
