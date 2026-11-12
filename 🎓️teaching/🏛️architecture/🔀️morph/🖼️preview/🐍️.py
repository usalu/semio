"""🖼 Preview swap: the cheap execution of a declared transition. The source is the sole visible representation, translated and scaled from an immutable snapshot toward the declared placement; at the terminal scheduled sample it is replaced atomically by the exact target.

No crossfade, overlap, blank frame, rotation, deformation or colour blending, and no full-morph work: this module never builds silhouettes, plans, part morphs, flights or hulls.
Identity and progress come only from the timeline owner (`sample_alpha`/`sample`); easing shapes translation and scale, never the identity boundary.
Reference: https://docs.manim.community/en/stable/reference/manim.animation.animation.Animation.html
"""

from __future__ import annotations

import importlib
from dataclasses import dataclass, field

import numpy as np

_engine = importlib.import_module("manim")
_timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")
_clock = importlib.import_module("🎓️teaching.🧰️interop.🕐️clock.🐍️")
_spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
_placement = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📍️placement.🐍️")
_instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")

TARGET = _timeline.TARGET


@dataclass
class Lane:
    """🛤 One source-to-target movement: source mobjects (direct children of the source root), the targets that replace them, a frozen affine and a window."""

    sources: list
    targets: list
    affine: object
    window: object
    natural: list = field(default_factory=list)


def _bounds(subject):
    """📦 Logical bounds of a subject, frozen at bind time (never measured from LoD geometry)."""
    return _placement.Bounds.of(subject.bounds, getattr(subject, "anchors", {}).get("origin"))


def _part_bounds(subject, name):
    """📦 Logical bounds of one named part, frozen at bind time (identical across LoDs); a missing entry is an error, never measured from LoD geometry."""
    frozen = getattr(subject, "part_bounds", None)
    if not frozen or name not in frozen:
        raise _placement.PlacementError(f"entity {subject.id!r} has no frozen logical bounds for part {name!r}; bind it with part_bounds={{{name!r}: ((xmin, ymin, zmin), (xmax, ymax, zmax))}} so placement is identical for every LoD")
    return _placement.Bounds.of(frozen[name])


def _pairs(transition, source, target):
    """🔗 Explicit (source part, target part) pairs of an n:n transition; None when preview must move the group as one lane."""
    correspondence = transition.correspondence
    kind = correspondence.kind
    if transition.relation != "many-to-many" or not _spec.uses_parts(transition):
        return None
    if kind == "recipe":
        keyed = [(k, k) for k in source.parts if k in target.parts]
        kind = "key" if keyed and len(keyed) == len(source.parts) == len(target.parts) else "order" if len(source.parts) == len(target.parts) else "group"
    if kind == "key":
        pairs = [(k, k) for k in source.parts if k in target.parts]
    elif kind == "order":
        pairs = list(zip(source.parts, target.parts))
    elif kind == "pairs":
        pairs = [tuple(pair) for pair in correspondence.pairs]
    else:
        return None
    for s, t in pairs:
        if s not in source.parts or t not in target.parts:
            raise _placement.PlacementError(f"transition {transition.id!r}: pair ({s!r}, {t!r}) names a part that is not bound; source parts {sorted(source.parts)}, target parts {sorted(target.parts)}")
    direct = [id(c) for c in source.mobject.submobjects]
    sources = [id(source.parts[s]) for s, _ in pairs]
    if not pairs or len(set(sources)) != len(sources) or any(i not in direct for i in sources):
        return None
    return pairs


def _lanes(source, target, transition, frames):
    """🛤 Lanes of one transition: per-element lanes for key/order/pairs n:n, otherwise one group lane; leftover source parts ride one rest lane."""
    placement, timing = transition.placement, transition.timing
    whole = _timeline.Window(0.0, timing.duration)
    if source is None or target is None or transition.copy:
        return ([] if source is None else [Lane([source.mobject], [], _placement.IDENTITY, whole)]), whole
    group = _placement.affine(_bounds(source), _bounds(target), placement, frames=frames)
    pairs = _pairs(transition, source, target)
    if pairs is None:
        return [Lane([source.mobject], [target.mobject], group, whole)], whole
    windows = _timeline.expand(whole, len(pairs), timing.lag, timing.limit)
    lanes = [
        Lane([source.parts[s]], [target.parts[t]], _placement.affine(_part_bounds(source, s), _part_bounds(target, t), placement, frames=frames), w)
        for (s, t), w in zip(pairs, windows)
    ]
    paired = {id(source.parts[s]) for s, _ in pairs}
    rest = [c for c in source.mobject.submobjects if id(c) not in paired]
    group_window = _timeline.span(windows)
    if rest:
        lanes.append(Lane(rest, [], group, group_window))
    return lanes, group_window


def _members(mob):
    """🧩 Family members that own points."""
    return [m for m in mob.get_family() if len(m.points)]


class PreviewSwap(_engine.Animation):
    """🖼 Source-only translation/scale toward the declared placement, then an atomic exact-target swap at the terminal sample.

    Construct from the stage subjects (`source`/`target` duck-typed: `mobject`, `parts`, `bounds`, `anchors`) and the spec transition.
    `ghost_mobject` is always None: a preview leaves no ghost.
    Appear (`source` None), vanish (`target` None) and copy transitions do not move anything: the source stays (and `copy` keeps it afterwards) until the terminal sample, where the target appears and a non-copy source disappears.
    A zero-duration transition cannot be played by Manim (`run_time <= 0`); call `swap_now(scene)` for the atomic switch.
    """

    def __init__(self, source, target, *, transition, fps, frames=None):
        self.source, self.target, self.transition = source, target, transition
        self.fps = float(fps)
        self.sampling = transition.source.sampling
        self.target_sampling = transition.target.sampling
        self.easing = transition.timing.easing
        self.lanes, self.window = _lanes(source, target, transition, frames)
        self.nominal = self.window.duration
        self.schedule = _timeline.Schedule(self.fps)
        self.root = _engine.Group() if source is None else source.mobject
        self.keep = bool(transition.copy)
        self.ghost_mobject = None
        self.stage = _engine.Group()
        super().__init__(self.stage, run_time=self.window.duration, rate_func=_engine.linear, suspend_mobject_updating=False)
        self._children = list(self.root.submobjects)
        self._gates = []
        self._target_points = []
        self._state = None
        self._drawn = None
        self._scene = None
        self._released = False

    def get_all_mobjects(self):
        return (self.stage,)

    def _gate(self, suspended):
        """🚪 Suspend or release the source's own updaters for the whole family; flags are remembered once."""
        family = self.root.get_family()
        if suspended and not self._gates:
            self._gates = [(m, m.updating_suspended) for m in family]
        for m in family:
            m.updating_suspended = suspended

    def _capture(self):
        """📸 Natural pose of every lane source: references to the point arrays, which placement replaces and never mutates."""
        for lane in self.lanes:
            lane.natural = [(m, m.points) for s in lane.sources for m in _members(s)]

    def _restore(self):
        """↩ Put every lane source back to its natural pose."""
        for lane in self.lanes:
            for m, points in lane.natural:
                m.points = points.copy()

    def _setup_scene(self, scene):
        self._scene = scene
        if self.stage in scene.mobjects:
            scene.mobjects.remove(self.stage)
        mobjects = scene.mobjects
        members = {id(m) for m in (self.root, *self.root.submobjects)}
        index = next((i for i, top in enumerate(mobjects) if any(id(m) in members for m in top.get_family())), len(mobjects))
        scene.remove(self.root, *self.root.submobjects)
        scene.mobjects.insert(min(index, len(scene.mobjects)), self.stage)
        self.stage.submobjects = [self.root]

    def begin(self):
        self._released = False
        self._children = list(self.root.submobjects)
        self._target_points = [(m, m.points.copy()) for m in _members(self.target.mobject)] if self.target is not None and self.target_sampling == "snapshot" else []
        _instrument.count("preview.swap")
        self._capture()
        self._gate(True)
        self._state = None
        self._drawn = None
        try:
            self.interpolate(0)
        except BaseException:
            self.release()
            raise

    def update_mobjects(self, dt):
        if self.sampling != "live" or self._state == "target":
            return
        self._restore()
        for m in self.root.get_family():
            m.updating_suspended = False
        self.root.update(dt)
        self._gate(True)
        self._capture()

    def _show_target(self):
        if self._state == "target":
            return
        for m, points in self._target_points:
            m.points = points.copy()
        self.root.submobjects = list(self._children)
        self.stage.submobjects = self._final()
        self._sync(("target",))
        self._state = "target"

    def _sync(self, key):
        """🖌 Manim's Cairo renderer draws the flat moving-mobject list captured when the play call began, never the stage group's current children. The list is therefore made equal, in place, to what the stage shows: managed mobjects (the source and target families) that are hidden leave it, visible ones missing from it join it (appended in stage family order). The result depends only on the current stage state, never on earlier calls, so seeking and a premature evaluation at `begin` cannot leave a visible mobject undrawn."""
        if key == self._drawn:
            return
        self._drawn = key
        moving = getattr(self._scene, "moving_mobjects", None)
        if moving is None:
            return
        managed = {id(m) for part in (self.root, *([] if self.target is None else [self.target.mobject])) for m in part.get_family()}
        shown = list(self.stage.get_family())
        visible = {id(m) for m in shown}
        moving[:] = [m for m in moving if id(m) not in managed or id(m) in visible]
        present = {id(m) for m in moving}
        moving.extend(m for m in shown if id(m) in managed and id(m) not in present)

    def interpolate(self, alpha):
        try:
            self._interpolate(alpha)
        except BaseException:
            self.release()
            raise

    def _interpolate(self, alpha):
        phase = _clock.SCENE_CLOCK.phase
        schedule = _timeline.Schedule(self.fps, phase / self.fps)
        frame = _timeline.frame_of_alpha(alpha, self.run_time, self.fps, phase)
        k = self.run_time / self.nominal if self.nominal else 1.0
        if _timeline.sample(self._scaled(self.window, k), schedule, frame).owner == TARGET:
            return self._show_target()
        swapped, shown = [], []
        for lane in self.lanes:
            s = _timeline.sample(self._scaled(lane.window, k), schedule, frame)
            if s.owner == TARGET and lane.targets:
                swapped.append(lane)
                shown.extend(lane.targets)
                continue
            moved = _placement.at(lane.affine, _timeline.ease(self.easing, s.progress))
            for m, points in lane.natural:
                m.points = _placement.apply(moved, points)
        gone = {id(s) for lane in swapped for s in lane.sources}
        keep = [c for c in self._children if id(c) not in gone]
        if len(keep) != len(self.root.submobjects):
            self.root.submobjects = keep
        self.stage.submobjects = [self.root, *shown]
        self._sync(("source", *(i for i, lane in enumerate(self.lanes) if any(lane is done for done in swapped))))
        self._state = "source"

    def finish(self):
        self.interpolate(1)
        self.release()

    def release(self):
        """🔓 Idempotently restore the source's pre-window pose, children and updater state."""
        if self._released:
            return
        self._released = True
        self._restore()
        self.root.submobjects = list(self._children)
        for m, flag in self._gates:
            m.updating_suspended = flag
        self._gates = []

    @staticmethod
    def _scaled(window, k):
        """↔ A window stretched uniformly when the caller (`scene.play(..., run_time=...)`) overrides the run time: the schedule always follows the time that actually plays."""
        return window if k == 1.0 else _timeline.Window(window.start * k, window.duration * k)

    def _final(self):
        """🎯 What the scene holds after the terminal sample: the exact target, plus the source for a copy."""
        return ([self.root] if self.keep and self.source is not None else []) + ([] if self.target is None else [self.target.mobject])

    def clean_up_from_scene(self, scene):
        self.release()
        mobjects = scene.mobjects
        final = self._final()
        for m in final:
            if m in mobjects:
                mobjects.remove(m)
        at = mobjects.index(self.stage) if self.stage in mobjects else len(mobjects)
        if self.stage in mobjects:
            mobjects.pop(at)
        mobjects[at:at] = final

    def swap_now(self, scene):
        """⚡ Atomic zero-duration switch: the scene holds exactly the target afterwards (plus the source for a copy), no frame is played."""
        self._setup_scene(scene)
        self.begin()
        self.finish()
        self.clean_up_from_scene(scene)
        return self
