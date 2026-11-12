"""🎭 Morph stage: per-scene entity registry and the single entry point that turns a declared transition into one Manim animation.

Clip code binds entities and plays `stage.transition(id)`; the active render profile decides between the preview swap and the full morph. No other code path branches on the profile. Reference: https://docs.manim.community/en/stable/reference/manim.animation.animation.Animation.html
"""

from __future__ import annotations

import importlib
import json
import os
import weakref
from collections import OrderedDict
from collections.abc import Mapping, Sequence
from dataclasses import dataclass, field
from pathlib import Path

import numpy as np

_engine = importlib.import_module("manim")
_spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
_timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")

PROFILE_MODULE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️"
PREVIEW_MODULE = "🎓️teaching.🏛️architecture.🔀️morph.🖼️preview.🐍️"
FULL_MODULE = "🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️"
MORPHS = ("swap", "full")
TRANSITIONS_FILE = "transitions.json"
_REPORTED: OrderedDict = OrderedDict()
_REPORTED_LIMIT = 4096


@dataclass(frozen=True)
class Subject:
    """🧍 A bound entity resolved at one level of detail: representation, named parts, frozen logical bounds and anchors."""

    id: str
    mobject: object
    parts: dict
    bounds: np.ndarray
    anchors: dict
    lod: object
    revision: int
    part_bounds: dict


def _frozen(value):
    out = np.array(value, dtype=float)
    out.setflags(write=False)
    return out


def _bounds_of(mobject):
    if len(mobject.get_all_points()) == 0:
        return _frozen([[0, 0, 0], [0, 0, 0]])
    return _frozen([mobject.get_critical_point(-np.ones(3)), mobject.get_critical_point(np.ones(3))])


def _anchors_of(bounds, given):
    low, high = bounds
    centre = (low + high) / 2
    out = {"center": centre, "origin": centre, "bottom": np.array([centre[0], low[1], centre[2]]), "top": np.array([centre[0], high[1], centre[2]]), "left": np.array([low[0], centre[1], centre[2]]), "right": np.array([high[0], centre[1], centre[2]])}
    out.update({name: np.array(point, dtype=float) for name, point in (given or {}).items()})
    return {name: _frozen(point) for name, point in out.items()}


def _is_factory(value):
    return callable(value) and not isinstance(value, _engine.Mobject)


@dataclass
class Bound:
    """🔌 One bind call: representation or factories, frozen logical bounds/anchors, revision and provided selectors; resolves lazily per lod."""

    id: str
    value: object
    bounds: np.ndarray
    anchors: dict
    revision: int
    provides: dict
    part_bounds: dict = field(default_factory=dict)
    frame: object = None
    cache: dict = field(default_factory=dict)

    def part_names(self):
        """🏷 Part names without resolving factories (None when unknown)."""
        if isinstance(self.value, Mapping):
            return tuple(self.value)
        if isinstance(self.value, Sequence):
            return tuple(str(i) for i in range(len(self.value)))
        if isinstance(self.value, _engine.Mobject):
            return tuple(str(i) for i in range(len(self.value.submobjects)))
        return None

    def resolve(self, lod):
        """🪄 Resolve at `lod`; factories run once per lod and are cached."""
        key = lod if isinstance(lod, (str, int, type(None))) else repr(lod)
        if key not in self.cache:
            self.cache[key] = self._build(lod)
        return self.cache[key]

    def _build(self, lod):
        value = self.value
        if isinstance(value, Mapping):
            parts = {str(k): v(lod) if _is_factory(v) else v for k, v in value.items()}
            mobject = _engine.VGroup(*parts.values())
        elif isinstance(value, _engine.Mobject):
            mobject, parts = value, {str(i): m for i, m in enumerate(value.submobjects)}
        elif _is_factory(value):
            mobject = value(lod)
            parts = {str(i): m for i, m in enumerate(mobject.submobjects)}
        else:
            mobject = _engine.VGroup(*value)
            parts = {str(i): m for i, m in enumerate(value)}
        return Subject(self.id, mobject, parts, self.bounds, self.anchors, lod, self.revision, self.part_bounds)


def _module(name):
    return importlib.import_module(name)


def _profile():
    return _module(PROFILE_MODULE).active()


class Stage:
    """🎬 Entity registry plus `transition`/`cut` for one scene and one validated transition document."""

    binders = {}

    def __init__(self, scene, document):
        self.scene = scene
        self.document = document
        self._bound = {}
        self._played = []
        self._animations = weakref.WeakSet()
        self._closed = False

    def close(self):
        """🔒 Idempotent disposal: release every animation this stage handed out that is still suspended (a cancelled or interrupted play), drop the cached representations and the scene reference. Safe to call twice and after a normal finish."""
        if self._closed:
            return
        self._closed = True
        for animation in list(self._animations):
            release = getattr(animation, "release", None)
            if release is not None:
                release()
        self._animations = weakref.WeakSet()
        for held in self._bound.values():
            held.cache.clear()
        self._bound.clear()
        self.scene = None

    def __enter__(self):
        return self

    def __exit__(self, *_exc):
        self.close()

    @property
    def played(self):
        """🧾 Ids of transitions requested on this stage, in call order (curriculum checks)."""
        return tuple(self._played)

    @property
    def correspondences(self):
        """🔗 Registered recipe correspondence names from the full-morph module, or None while it declares none."""
        try:
            names = getattr(_module(FULL_MODULE), "CORRESPONDENCES", None)
        except ModuleNotFoundError:
            return None
        return None if names is None else frozenset(names)

    def bound(self, entity_id):
        """🔎 The `Bound` registered under `entity_id` or None."""
        return self._bound.get(entity_id)

    @classmethod
    def register_binder(cls, kind, binder):
        """🧩 Staffage extension point: register `binder(stage, entity_id, *args, **kwargs)` for `bind_with(kind, ...)`; it must end in `stage.bind(...)`."""
        cls.binders[kind] = binder

    def bind_with(self, kind, entity_id, *args, **kwargs):
        """🧩 Bind an entity through a registered binder (for example canonical staffage references)."""
        if kind not in self.binders:
            raise KeyError(f"no binder registered for {kind!r}; registered: {sorted(self.binders)}. Import the module that calls Stage.register_binder({kind!r}, ...) first")
        return self.binders[kind](self, entity_id, *args, **kwargs)

    def bind(self, entity_id, value, *, bounds=None, anchors=None, revision=1, provides=None, part_bounds=None, frame=None):
        """🔌 Bind `value` (Mobject, mapping, sequence or `factory(lod)`) to a stable entity id; logical bounds, anchors and per-part bounds freeze now (`part_bounds` is required for parts of factories that lanes should measure; `frame` is the entity's `placement.Frame` for nested placement spaces)."""
        if entity_id in self._bound:
            raise ValueError(f"entity {entity_id!r} is already bound on this stage. Fix: bind every entity once per scene (bind a mapping to expose several parts)")
        if _is_factory(value) and bounds is None:
            raise ValueError(f"factory for {entity_id!r} needs explicit logical bounds ((xmin, ymin, zmin), (xmax, ymax, zmax)). Fix: pass bounds=...; they are never derived from reduced-detail geometry")
        if bounds is not None:
            frozen = _frozen(bounds)
        else:
            parts = list(value.values()) if isinstance(value, Mapping) else None
            whole = value if isinstance(value, _engine.Mobject) else _engine.VGroup(*(parts if parts is not None else list(value)))
            frozen = _bounds_of(whole)
        if frozen.shape != (2, 3):
            raise ValueError(f"bounds of {entity_id!r} must have shape (2, 3) (min and max corner), got {frozen.shape}")
        measured = self._measure(value)
        measured.update({str(k): _frozen(v) for k, v in (part_bounds or {}).items()})
        self._bound[entity_id] = Bound(entity_id, value, frozen, _anchors_of(frozen, anchors), int(revision), dict(provides or {}), measured, frame)
        return self

    @staticmethod
    def _measure(value):
        if isinstance(value, Mapping):
            items = value.items()
        elif isinstance(value, _engine.Mobject):
            items = enumerate(value.submobjects)
        elif _is_factory(value):
            return {}
        else:
            items = enumerate(value)
        return {str(k): _bounds_of(m) for k, m in items if isinstance(m, _engine.Mobject)}

    def _frames(self):
        return {entity: item.frame for entity, item in self._bound.items() if item.frame is not None}

    def _subjects(self, transition_id):
        item = self.document.transition(transition_id)
        problems = _spec.validate(self.document, self, only=transition_id)
        if problems:
            raise _spec.SpecError(problems)
        self._played.append(transition_id)
        lod = _profile().lod
        return item, *(None if end is None else self._bound[end.entity].resolve(lod) for end in (item.source, item.target))

    def mobject(self, entity_id):
        """🧱 The representation of a bound entity at the active profile lod (adds nothing to the scene)."""
        if entity_id not in self._bound:
            raise KeyError(f"entity {entity_id!r} is not bound; bound: {sorted(self._bound)}. Fix: call stage.bind({entity_id!r}, ...)")
        return self._bound[entity_id].resolve(_profile().lod).mobject

    def _attach(self, transition_id, source, target):
        item = self.document.transition(transition_id)
        for end, value in ((item.source, source), (item.target, target)):
            if value is None:
                continue
            if end is None:
                raise ValueError(f"transition {transition_id!r} declares no entity on this side but a value was passed. Fix: pass source/target only for non-null endpoints")
            held = self._bound.get(end.entity)
            if held is None or held.value is not value:
                self._bound.pop(end.entity, None)
                self.bind(end.entity, value, revision=end.revision)
        return item

    def _report(self, item):
        """🧾 Append `{id, start, duration, f0, f1}` on the scene clock to `transitions.json` beside the work record, so posters and tools resolve the real terminal frame."""
        path = os.environ.get("SEMIO_TEACHING_WORK")
        if not path:
            return
        fps = float(_engine.config.frame_rate)
        start = float(self.scene.renderer.time)
        window, schedule = _timeline.Window(start, item.timing.duration), _timeline.Schedule(fps)
        span = _timeline.frames(window, schedule)
        f1 = _timeline.terminal(window, schedule)
        _REPORTED[(item.id, start)] = {"id": item.id, "start": start, "duration": item.timing.duration, "f0": f1 if span is None else span[0], "f1": f1}
        while len(_REPORTED) > _REPORTED_LIMIT:
            _REPORTED.popitem(last=False)
        target = Path(path).with_name(TRANSITIONS_FILE)
        temporary = target.with_name(f".{target.stem}.{os.getpid()}.tmp")
        try:
            temporary.write_text(json.dumps({"transitions": list(_REPORTED.values())}, indent=2), encoding="utf-8")
            temporary.replace(target)
        finally:
            temporary.unlink(missing_ok=True)

    def transition(self, transition_id, source=None, target=None):
        """🔀 Return ONE Manim animation for `transition_id`: preview swap or full morph by the active profile. `source`/`target` bind the declared entities inline (rebinding replaces an entity bound to another object)."""
        item = self._attach(transition_id, source, target)
        if item.timing.duration == 0:
            raise ValueError(f"transition {transition_id!r} has duration 0, which Manim cannot play. Fix: call stage.cut({transition_id!r}) for an atomic switch or give it a positive duration")
        profile = _profile()
        if profile.morph not in MORPHS:
            raise ValueError(f"profile morph {profile.morph!r} is not one of {MORPHS}")
        item, source, target = self._subjects(transition_id)
        self._report(item)
        fps = float(_engine.config.frame_rate)
        animation = (_module(PREVIEW_MODULE).PreviewSwap(source, target, transition=item, fps=fps, frames=self._frames() or None) if profile.morph == "swap"
                     else _module(FULL_MODULE).execute(source, target, transition=item, fps=fps))
        self._animations.add(animation)
        return animation

    def cut(self, transition_id, source=None, target=None):
        """✂ Atomic switch at the current scene time for both profiles: remove the source (unless `copy`), add the exact target as bound; an absent endpoint (null) only removes or only adds (no frames, no full-morph work)."""
        self._attach(transition_id, source, target)
        item, source, target = self._subjects(transition_id)
        if item.timing.duration != 0:
            self._played.pop()
            raise ValueError(f"transition {transition_id!r} has duration {item.timing.duration}; cut only switches atomically. Fix: use stage.transition({transition_id!r}) or set duration 0")
        self._report(item)
        if _profile().morph == "swap":
            _module(PREVIEW_MODULE).PreviewSwap(source, target, transition=item, fps=float(_engine.config.frame_rate), frames=self._frames() or None).swap_now(self.scene)
            return
        if source is not None and not item.copy:
            self.scene.remove(source.mobject)
        if target is not None:
            self.scene.add(target.mobject)


def stage(scene, document_path):
    """🎭 Load and validate the transition document at `document_path` once and return the `Stage` of `scene`."""
    return Stage(scene, _spec.load(Path(document_path)))
