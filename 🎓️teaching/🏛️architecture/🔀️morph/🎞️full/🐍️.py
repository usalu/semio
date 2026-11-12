"""🎞 Full morphs: the authored final-profile execution of declarative transitions (parts, hull, scatter, gather, split, unfold, retext, flight, move, replace, fade, vanish, appear).

Identity-preserving Manim morphs between heterogeneous shapes; the legacy toolkit helpers (`morph`, `scatter`, ...) live here too. The bound source and target definitions are never mutated while a morph is evaluated: arriving details are shadow copies and the exact bound target joins the scene at clean-up. Manim only. Reference: https://docs.manim.community/en/stable/reference/manim.animation.transform.Transform.html
"""

from __future__ import annotations

import importlib

import numpy as np

_engine = importlib.import_module("manim")
_clock = importlib.import_module("🎓️teaching.🧰️interop.🕐️clock.🐍️")
_timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")
_instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
_spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")

_sil = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🫥️silhouette.🐍️")

SAMPLES, _members, _rgb, _style, _outline, _inside, _hull, _restyle, _nothing, _centre_of, silhouette = (getattr(_sil, name) for name in ("SAMPLES", "_members", "_rgb", "_style", "_outline", "_inside", "_hull", "_restyle", "_nothing", "_centre_of", "silhouette"))
HEAVY = 200
RECIPES = ("parts", "hull", "scatter", "gather", "split", "unfold", "retext", "flight", "move", "replace", "fade", "vanish", "appear")


def proxy(mob, *, color=None):
    """🧱 Cheap stand-in for heavy groups: one bounding rectangle with the group's mean colour, never the thousands of faces; pointless input gives an invisible stand-in."""
    _instrument.count("full.prepare")
    parts = _members(mob)
    if not parts:
        return _nothing()
    outline = _outline(mob)
    low, high = outline.min(axis=0), outline.max(axis=0)
    shape = _engine.Rectangle(width=max(high[0] - low[0], 1e-3), height=max(high[1] - low[1], 1e-3)).move_to((low + high) / 2)
    return _restyle(shape, _style(mob), color)


def _opacities(mob):
    """🔆 Per-member (fill, stroke) opacities of a family."""
    return [(m, m.get_fill_opacity(), m.get_stroke_opacity()) for m in _members(mob)]


def _scale_opacity(bases, level):
    """🔆 Set every member to its base opacity times `level`."""
    for m, fill, stroke in bases:
        m.set_fill(opacity=fill * level)
        m.set_stroke(opacity=stroke * level)


def _ramp(alpha, start, end):
    """📐 Clamped linear ramp of `alpha` between `start` and `end`."""
    return float(np.clip((alpha - start) / (end - start), 0.0, 1.0))


def _live(mob):
    """🎥 No-op updater that marks a mobject as moving, so the renderer redraws it every frame instead of caching it."""


def _dip(anim, alpha, dip):
    """🛫 Arc easing: shrink a flying shape slightly mid-flight, more for long distances (full effect from 6 units)."""
    if dip:
        span = float(np.linalg.norm(anim.target_copy.get_center() - anim.starting_mobject.get_center()))
        anim.mobject.scale(1 - dip * np.sin(np.pi * anim.rate_func(alpha)) * min(1.0, span / 6.0), about_point=anim.mobject.get_center())


class Morph(_engine.Transform):
    """🔀 A silhouette of the source flows into the target: multi-part sources fade their details out first, multi-part targets fade their details in last; optional ghost, or consumed target for gathers."""

    LEAVE, ARRIVE = 0.4, 0.5

    def __init__(self, src, dst, *, ghost=0.0, keep_source=False, consume=False, path_arc=0.0, run_time=1.4, dip=0.0, rate_func=_engine.smooth, tail=0.0, **kwargs):
        self.source, self.final, self.ghost_level, self.keep_source, self.consume, self.dip, self.tail = src, dst, ghost, keep_source, consume, dip, tail
        self.ghost_mobject, self.arrival, self._leaving, self._arriving = None, None, [], []
        self.detailed_target = not consume and len(_members(dst)) != 1
        start = silhouette(src) if _members(src) else _nothing().move_to(_centre_of(dst))
        self.detailed_source = len(_members(src)) > 1 or len(src.get_family()) > 1
        super().__init__(start, (silhouette(dst) if _members(dst) else _nothing().move_to(start.get_center())) if self.detailed_target else dst, path_arc=path_arc, run_time=run_time, rate_func=rate_func, remover=consume, replace_mobject_with_target_in_scene=not (consume or self.detailed_target), **kwargs)

    def _setup_scene(self, scene):
        """🎬 Take the source out of the scene (details or a ghost stay for a moment) and prepare the arriving target shadow."""
        super()._setup_scene(scene)
        if not self.keep_source:
            if self.ghost_level > 0 or self.detailed_source:
                self.ghost_mobject = self.source.copy()
                self._leaving = _opacities(self.ghost_mobject)
                scene.add(self.ghost_mobject.add_updater(_live))
                scene.bring_to_front(self.mobject)
            scene.remove(self.source)
        if self.detailed_target:
            self.arrival = self.final.copy()
            self._arriving = _opacities(self.arrival)
            _scale_opacity(self._arriving, 0.0)
            scene.add(self.arrival.add_updater(_live))

    def _apply(self, a):
        """📈 Leaving details fade, arriving details appear, the silhouette hands over."""
        if self.ghost_mobject is not None:
            _scale_opacity(self._leaving, 1 - (1 - self.ghost_level) * _ramp(a, 0.0, self.LEAVE))
        if self.tail:
            level = 1 - _ramp(a, 1 - self.tail, 1.0)
            for m in _members(self.mobject):
                m.set_fill(opacity=m.get_fill_opacity() * level)
                m.set_stroke(opacity=m.get_stroke_opacity() * level)
        if self.detailed_target:
            reveal = _ramp(a, 1 - self.ARRIVE, 1.0)
            _scale_opacity(self._arriving, reveal)
            for m in _members(self.mobject):
                m.set_fill(opacity=m.get_fill_opacity() * (1 - reveal))
                m.set_stroke(opacity=m.get_stroke_opacity() * (1 - reveal))

    def interpolate_mobject(self, alpha):
        """📈 Morph the silhouette, then apply the detail fades."""
        super().interpolate_mobject(alpha)
        self._apply(self.rate_func(alpha))
        _dip(self, alpha, self.dip)

    def clean_up_from_scene(self, scene):
        """🧹 Leave exactly the target (and the ghost, when asked for) in the scene."""
        if self.detailed_target:
            _engine.Animation.clean_up_from_scene(self, scene)
            scene.remove(self.mobject)
        else:
            super().clean_up_from_scene(scene)
        self._apply(1.0)
        if self.detailed_target:
            scene.remove(self.arrival)
            scene.add(self.final)
        if self.ghost_mobject is not None:
            self.ghost_mobject.remove_updater(_live)
            if self.ghost_level <= 0:
                scene.remove(self.ghost_mobject)
                self.ghost_mobject = None


def _sound(src, dst, pairs):
    """🩹 Pairs without empty parts: a paired part without points is replaced by the whole source or target; nothing is left when either side has no points at all."""
    if not pairs or not _members(src) or not _members(dst):
        return []
    fixed = [(s if _members(s) else src, d if _members(d) else dst) for s, d in pairs]
    return list(dict.fromkeys((s, d) for s, d in fixed))


def _extent(part):
    """📏 Bounding-box area of a part's outline (0 without points)."""
    if not _members(part):
        return 0.0
    outline = _outline(part)
    low, high = outline.min(axis=0), outline.max(axis=0)
    return float((high[0] - low[0]) * (high[1] - low[1]))


def capsule_pairs(capsule, tok):
    """💊 Part pairs for a capsule drawing and an `item` token: the shell (largest outline) to the ring and the window (a part named `window`, else the second largest) to the dot; parts without points are ignored."""
    parts = [p for p in capsule.submobjects if _members(p)]
    while len(parts) == 1 and parts[0].submobjects:
        parts = [p for p in parts[0].submobjects if _members(p)]
    if not parts or not _members(tok):
        return []
    ranked = sorted(parts, key=_extent, reverse=True)
    shell = ranked[0]
    rest = ranked[1:]
    named = [p for p in rest if getattr(p, "name", "") == "window"]
    window = (named or rest or [None])[0]
    targets = [t for t in tok.submobjects if _members(t)] or [tok]
    pairs = [(shell, targets[0])]
    if window is not None:
        pairs.append((window, targets[1] if len(targets) > 1 else targets[0]))
    return pairs


def _tops(mob):
    """🧱 Top-level parts of a group (single wrapper groups unwrapped, empty parts dropped)."""
    parts = [p for p in mob.submobjects if _members(p)]
    while len(parts) == 1 and parts[0].submobjects:
        parts = [p for p in parts[0].submobjects if _members(p)]
    return parts


def _solo(part):
    """🔹 Copy of a shape without its submobjects (an arrow's tip travels as its own part)."""
    shape = part.copy()
    shape.submobjects = []
    return shape


def _parts(mob):
    """🧩 Parts to match: every pointed shape, or the real silhouette of every top-level part above `HEAVY` shapes; the silhouette of the whole group when it has no usable top-level parts. Never a bounding rectangle."""
    leaves = _members(mob)
    if len(leaves) <= HEAVY:
        return [_solo(m) for m in leaves]
    tops = _tops(mob)
    if 1 < len(tops) <= HEAVY:
        return [silhouette(t) for t in tops]
    if len(tops) > HEAVY:
        raise ValueError(f"a group of {len(leaves)} shapes in {len(tops)} top-level parts is too large for a real per-part morph (limit {HEAVY} parts); declare correspondence pairs on at most {HEAVY} parts")
    return [silhouette(mob)]


def _middle(part):
    """🎯 Centre of the bounding box of a part's outline."""
    outline = _outline(part)
    return (outline.min(axis=0) + outline.max(axis=0)) / 2


def _plan(src, dst):
    """🧭 Automatic part matching: parts sorted by area (then position) are paired greedily; extra source parts shrink towards the nearest partner's target, extra target parts grow from the nearest partner's source. None when either side has no parts. Outlines are measured once per part."""
    _instrument.count("full.prepare")

    def ordered(parts):
        middles = [_middle(p) for p in parts]
        extents = [_extent(p) for p in parts]
        keyed = [(-round(extents[i], 3), *np.round(middles[i][:2], 2), i) for i in range(len(parts))]
        order = [k[-1] for k in sorted(keyed)]
        return [parts[i] for i in order], np.array([middles[i] for i in order]).reshape(-1, 3)

    (sp, sm), (dp, dm) = ordered(_parts(src)), ordered(_parts(dst))
    count = min(len(sp), len(dp))
    if not count:
        return None
    near = lambda point, group: int(np.argmin(np.linalg.norm(group[:count] - point, axis=1)))
    pairs = list(zip(sp[:count], dp[:count]))
    shrinks = [(s, dm[near(sm[count + i], sm)]) for i, s in enumerate(sp[count:])]
    grows = [(d, sm[near(dm[count + i], dm)]) for i, d in enumerate(dp[count:])]
    return pairs, shrinks, grows


def _build(src, dst, *, pairs=None, ghost=0.0, path_arc=0.0, run_time=1.4, keep_source=False, dip=0.0, mode="parts", rate_func=_engine.smooth):
    """🔀 One morph animation without frame instrumentation (see `morph`)."""
    if mode not in ("parts", "hull"):
        raise ValueError("mode is parts or hull")
    pairs = _sound(src, dst, pairs)
    if pairs:
        return PartMorph(src, dst, pairs, ghost=ghost, path_arc=path_arc, run_time=run_time, dip=dip, keep_source=keep_source, rate_func=rate_func)
    plan = _plan(src, dst) if mode == "parts" and (len(_members(src)) > 1 or len(_members(dst)) > 1) else None
    if plan:
        return PartMorph(src, dst, plan[0], shrinks=plan[1], grows=plan[2], ghost=ghost, path_arc=path_arc, run_time=run_time, dip=dip, keep_source=keep_source, rate_func=rate_func)
    return Morph(src, dst, ghost=ghost, path_arc=path_arc, run_time=run_time, keep_source=keep_source, dip=dip, rate_func=rate_func)


def _counted(animation, remap=None):
    """📊 Count `full.evaluate` per interpolated frame; `remap(alpha)` optionally replaces the raw alpha handed to the animation."""
    inner = animation.interpolate

    def interpolate(alpha):
        _instrument.count("full.evaluate")
        inner(alpha if remap is None else remap(alpha))

    animation.interpolate = interpolate
    return animation


def morph(src, dst, *, pairs=None, ghost=0.0, path_arc=0.0, run_time=1.4, keep_source=False, dip=0.0, mode="parts"):
    """🔀 Concrete to abstract (or back): `src` leaves the scene, `dst` is in the scene afterwards, colours interpolate; `ghost` leaves a faint copy of the source (`anim.ghost_mobject`).

    Default `mode="parts"`: the parts of source and target are matched automatically (by area, then position) and fly to each other keeping their real shapes; extra source parts shrink towards their nearest partner's target, extra target parts grow from their nearest partner. Above 200 shapes the real silhouettes of the top-level parts travel. `mode="hull"` opts into one convex-hull silhouette (a blob) instead.
    `pairs=[(src_part, dst_part), ...]` overrides the matching with your own pairs (window to dot, body to outline); unpaired parts dim and arrive with the whole. `dip` shrinks the flight slightly mid-way (0.1 to 0.2)."""
    return _counted(_build(src, dst, pairs=pairs, ghost=ghost, path_arc=path_arc, run_time=run_time, keep_source=keep_source, dip=dip, mode=mode))


class Swarm(_engine.LaggedStart):
    """🐝 Staggered group whose members live in the scene individually, so nothing but the targets remains afterwards."""

    def __init__(self, *animations, **kwargs):
        super().__init__(*animations, **kwargs)
        self.group.swarm = True

    def _setup_scene(self, scene):
        """🎬 Dissolve the animation's wrapper group (and nested swarm groups) inside the scene."""
        super()._setup_scene(scene)
        if self.group in scene.mobjects:
            at = scene.mobjects.index(self.group)
            scene.mobjects[at : at + 1] = _flatten(self.group)


def _flatten(group):
    """🥞 Members of a swarm group with nested swarm groups expanded."""
    return [leaf for m in group.submobjects for leaf in (_flatten(m) if getattr(m, "swarm", False) else [m])]


class Lazy(_engine.Animation):
    """💤 Placeholder for one element of a staggered group: the real sub-morph (correspondence, outlines, silhouettes) is built when the group starts and dropped when it ends."""

    def __init__(self, factory, run_time):
        self.factory, self.real = factory, None
        super().__init__(_engine.VGroup(), introducer=True, run_time=run_time)

    def _setup_scene(self, scene):
        """🎬 Build the real animation and set it up like the scene does for a direct play."""
        self.real = self.factory()
        if not self.real.is_introducer():
            scene.add(self.real.mobject)
        self.real._setup_scene(scene)

    def begin(self):
        """▶ Begin the real animation."""
        self.real.begin()

    def interpolate(self, alpha):
        """📈 Evaluate the real animation."""
        self.real.interpolate(alpha)

    def update_mobjects(self, dt):
        """⏱ Forward updaters."""
        self.real.update_mobjects(dt)

    def finish(self):
        """🏁 Finish the real animation."""
        self.real.finish()

    def clean_up_from_scene(self, scene):
        """🧹 Clean up and release the real animation."""
        self.real.clean_up_from_scene(scene)
        self.real = None

    def get_all_mobjects(self):
        """🧩 Mobjects of the real animation."""
        return [] if self.real is None else self.real.get_all_mobjects()


def _lazy(factory, run_time):
    """💤 Lazy element of a staggered group."""
    return Lazy(factory, run_time)


class Dim(_engine.Animation):
    """🌫 Fade a copy to a resting opacity during the first part of the morph; removed when the level is zero."""

    def __init__(self, mob, level=0.0, **kwargs):
        self.level, self._bases = level, _opacities(mob)
        super().__init__(mob, remover=level <= 0, rate_func=_engine.linear, **kwargs)

    def interpolate_mobject(self, alpha):
        """📈 Apply the fade."""
        _scale_opacity(self._bases, 1 - (1 - self.level) * _ramp(alpha, 0.0, Morph.LEAVE))


class Flight(_engine.Transform):
    """🕊 One part flying to its partner; removed afterwards, the arriving target takes over."""

    def __init__(self, start, target, *, dip=0.0, **kwargs):
        self.dip = dip
        super().__init__(start, target, remover=True, replace_mobject_with_target_in_scene=False, **kwargs)

    def interpolate_mobject(self, alpha):
        """📈 Fly, with the optional dip."""
        super().interpolate_mobject(alpha)
        _dip(self, alpha, self.dip)


class PartMorph(Swarm):
    """🧩 Part-wise morph: paired parts fly to their partners keeping their own shapes; `shrinks` (part, point) parts shrink towards a point and fade out last, `grows` (part, point) parts grow from a point; the whole source dims and the whole target (if any) arrives."""

    def __init__(self, src, dst, pairs, *, shrinks=(), grows=(), ghost=0.0, path_arc=0.0, run_time=1.4, dip=0.0, keep_source=False, rate_func=_engine.smooth):
        self.source, self.final, self.ghost_level, self.keep_source = src, dst, ghost, keep_source
        self.ghost_mobject, self.arrival, self._arriving = (None if keep_source else src.copy().add_updater(_live)), None, []
        fly = dict(dip=dip, path_arc=path_arc, run_time=run_time, rate_func=rate_func)
        self.flights = [Flight(silhouette(s), silhouette(d) if len(_members(d)) > 1 else d, **fly) for s, d in pairs]
        self.vanishing = [Flight(start := silhouette(s), start.copy().scale(0.15).move_to(point), **fly) for s, point in shrinks]
        self.growing = [Flight(_small(d, point), d, **fly) for d, point in grows]
        extra = [] if self.ghost_mobject is None else [Dim(self.ghost_mobject, ghost, run_time=min(1.0, run_time))]
        super().__init__(*extra, *self.flights, *self.vanishing, *self.growing, lag_ratio=0.0, run_time=run_time)

    def _setup_scene(self, scene):
        """🎬 Swap the source for its dimming copy and the flying parts; prepare the arriving target shadow."""
        super()._setup_scene(scene)
        if not self.keep_source:
            scene.remove(self.source)
        if self.final is not None:
            self.arrival = self.final.copy()
            self._arriving = _opacities(self.arrival)
            _scale_opacity(self._arriving, 0.0)
            scene.add(self.arrival.add_updater(_live))

    def interpolate(self, alpha):
        """📈 Fly the parts, then hand them over to the arriving target."""
        super().interpolate(alpha)
        smooth = _engine.smooth(alpha)
        reveal = _ramp(smooth, 1 - Morph.ARRIVE, 1.0) if self.final is not None else 0.0
        _scale_opacity(self._arriving, reveal)
        for flight in (*self.flights, *self.growing):
            _fade_members(flight.mobject, reveal)
        for flight in self.vanishing:
            _fade_members(flight.mobject, _ramp(smooth, 0.75, 1.0))

    def clean_up_from_scene(self, scene):
        """🧹 Leave exactly the target (and the ghost, when asked for) in the scene."""
        super().clean_up_from_scene(scene)
        if self.final is not None:
            scene.remove(self.arrival)
            scene.add(self.final)
        if self.ghost_mobject is not None:
            self.ghost_mobject.remove_updater(_live)
            if self.ghost_level <= 0:
                self.ghost_mobject = None


def _fade_members(mob, level):
    """🌫 Multiply the opacities of a family by `1 - level`."""
    for m in _members(mob):
        m.set_fill(opacity=m.get_fill_opacity() * (1 - level))
        m.set_stroke(opacity=m.get_stroke_opacity() * (1 - level))


def _small(part, point):
    """🌱 Invisible shrunken copy of a part at a point: the seed it grows from."""
    seed = part.copy().scale(0.15).move_to(point)
    for m in _members(seed):
        m.set_fill(opacity=0)
        m.set_stroke(opacity=0)
    return seed


def _staggered(animations, run_time, lag, limit):
    """⏱ Swarm with a total duration capped at `limit` seconds."""
    count = len(animations)
    lag = _timeline.stagger_lag(run_time, count, lag, limit)
    return Swarm(*animations, lag_ratio=lag / run_time if run_time else 0.0)


def _absorbed(part, sink, point, *, path_arc, run_time, tail=0.0):
    """🧲 A source part collapses into a point and disappears."""
    if len(_members(part)) > 1:
        return PartMorph(part, None, [], shrinks=[(p, point) for p in _parts(part)], path_arc=path_arc, run_time=run_time)
    return Morph(part, sink.copy(), consume=True, path_arc=path_arc, run_time=run_time, tail=tail)


def _vanish(part, point, *, path_arc, run_time):
    """🫥 A leftover source part shrinks into a point while fading."""
    sink = silhouette(part).scale(0.15).move_to(point)
    for m in _members(sink):
        m.set_fill(opacity=0)
        m.set_stroke(opacity=0)
    return _absorbed(part, sink, point, path_arc=path_arc, run_time=run_time)


def scatter(srcs, dsts, *, lag=0.04, ghost=0.0, path_arc=0.6, run_time=1.2, limit=4.0, pairs_fn=None, dip=0.0):
    """🎆 Many to many in order (140 capsules to 140 leaves); every source flies to its own target. `pairs_fn(src, dst)` returns the part pairs of `morph(pairs=...)` for each couple."""
    srcs, dsts = list(srcs), list(dsts)
    if len(srcs) != len(dsts):
        raise ValueError("scatter needs as many targets as sources")
    return _counted(_staggered([_lazy(lambda s=s, d=d: _build(s, d, pairs=pairs_fn(s, d) if pairs_fn else None, ghost=ghost, path_arc=path_arc, run_time=run_time, dip=dip), run_time) for s, d in zip(srcs, dsts)], run_time, lag, limit))


def _gathered(srcs, dst, *, path_arc, run_time, shadow):
    """🧲 Animations of a gather: every source collapses into the target's place while the target (or its shadow) appears."""
    sink = silhouette(dst)
    sink.set_fill(opacity=sink.get_fill_opacity() * 0.5).set_stroke(opacity=sink.get_stroke_opacity() * 0.5)
    point = _middle(dst) if _members(dst) else dst.get_center()
    absorbed = [_lazy(lambda s=s: _absorbed(s, sink, point, path_arc=path_arc, run_time=run_time, tail=0.15 if shadow else 0.0), run_time) for s in srcs]
    arriving = _engine.FadeIn(dst.copy(), scale=0.85, remover=True, run_time=run_time) if shadow else _engine.FadeIn(dst, scale=0.85, run_time=run_time)
    return [*absorbed, arriving]


def gather(srcs, dst, *, lag=0.02, path_arc=0.4, run_time=1.0, limit=3.0):
    """🧲 Many to one: sources collapse into the target's place while it appears (a count, a list, a number)."""
    return _counted(_staggered(_gathered(list(srcs), dst, path_arc=path_arc, run_time=run_time, shadow=False), run_time, lag, limit))


def split(src, dsts, *, lag=0.06, path_arc=0.3, ghost=0.0, run_time=1.2, limit=3.0):
    """🌱 One to many: the source sprouts the targets (a list into branches, a value into cells)."""
    dsts = list(dsts)
    return _counted(_staggered([_lazy(lambda i=i, d=d: _build(src, d, keep_source=i > 0, ghost=ghost if i == 0 else 0.0, path_arc=path_arc, run_time=run_time), run_time) for i, d in enumerate(dsts)], run_time, lag, limit))


def unfold(faces, layout, *, lag=0.08, path_arc=0.0, run_time=1.4, limit=3.0):
    """📖 Faces of a box unfold into a net or into rows: `layout` holds one target mobject or point per face."""
    faces, layout = list(faces), list(layout)
    if len(faces) != len(layout):
        raise ValueError("unfold needs one layout entry per face")

    def target(face, spot):
        return spot if isinstance(spot, _engine.Mobject) else face.copy().move_to(spot)

    return _counted(_staggered([_lazy(lambda f=f, s=s: _build(f, target(f, s), path_arc=path_arc, run_time=run_time), run_time) for f, s in zip(faces, layout)], run_time, lag, limit))


def retext(a, b, *, run_time=1.2, path_arc=0.0):
    """✏ Formula or text morph: matching glyph shapes slide, the rest transforms."""
    _instrument.count("full.prepare")
    return _counted(_engine.TransformMatchingShapes(a, b, transform_mismatches=True, run_time=run_time, path_arc=path_arc))


class Trip(_engine.Animation):
    """✈️ A real object (a capsule copy, a chip, a digit, a leaf, a swatch; a plain dot without one) flying on a curved path, optionally round a waypoint, scaled from its first to its last width and visible only while in flight; `carry` blends a real object's colours towards the partner's."""

    def __init__(self, start, end, colors, widths=(0.26, 0.1), arc=0.45, mob=None, via=None, carry=False, **kwargs):
        _instrument.count("full.prepare")
        flier = mob if mob is not None else _engine.Dot(start, radius=0.05, color=colors[0])
        super().__init__(flier, introducer=True, remover=True, **kwargs)
        self.a, self.b, self.arc, self.colors, self.widths, self.real, self.carry = np.array(start, dtype=float), np.array(end, dtype=float), arc, colors, widths, mob is not None, carry and mob is not None
        self.via = None if via is None else np.array([via[0], via[1], 0.0], dtype=float)
        self.base = [(m, m.get_fill_opacity(), m.get_stroke_opacity()) for m in flier.get_family()]
        self.hues = [(m, _rgb(m.get_fill_color()), _rgb(m.get_stroke_color())) for m in flier.get_family() if isinstance(m, _engine.VMobject)]

    def interpolate_mobject(self, alpha):
        """📈 Fly along the arc, rescale, fade at both ends."""
        t = float(self.rate_func(alpha))
        chord = self.b - self.a
        if self.via is None:
            side = np.array([-chord[1], chord[0], 0.0]) * self.arc * float(np.sin(np.pi * t))
            point = self.a + chord * t + side
        else:
            control = 2.0 * self.via - (self.a + self.b) / 2.0
            point = (1 - t) ** 2 * self.a + 2 * (1 - t) * t * control + t**2 * self.b
        wide = self.widths[0] + (self.widths[1] - self.widths[0]) * t
        if self.mobject.width > 1e-6 and wide > 0:
            self.mobject.scale(wide / self.mobject.width)
        self.mobject.move_to(point)
        level = min(1.0, 8.0 * alpha, 8.0 * (1.0 - alpha))
        if self.real:
            for member, fill, stroke in self.base:
                member.set_fill(opacity=fill * level)
                member.set_stroke(opacity=stroke * level)
            if self.carry:
                goal = _rgb(self.colors[1])
                for member, fill, stroke in self.hues:
                    member.set_fill(_engine.ManimColor.from_rgb(tuple(fill + (goal - fill) * t)))
                    member.set_stroke(_engine.ManimColor.from_rgb(tuple(stroke + (goal - stroke) * t)))
        else:
            self.mobject.set_color(_engine.interpolate_color(_engine.ManimColor(self.colors[0]), _engine.ManimColor(self.colors[1]), t))
            self.mobject.set_opacity(level)


def flight(starts, ends, colors, *, widths=(0.26, 0.1), lag=0.015, arc=0.45, mobs=None, via=None, carry=False, run_time=1.4, limit=None):
    """✈️ Staggered fliers from `starts` to `ends`: the real objects `mobs` (copies) when given, else dots with a `(from, to)` colour pair; `widths` may be a function of the flier. Sources and targets stay as authored; only fliers are drawn."""
    trips = [Trip(s, e, c, widths(m) if callable(widths) else widths, arc, mob=m, via=via, carry=carry, run_time=run_time) for s, e, c, m in zip(starts, ends, colors, mobs or [None] * len(starts))]
    return _counted(_staggered(trips, run_time, lag, limit)) if trips else _engine.Wait(0.01)


class Flock(Swarm):
    """🕊 Real copies of source parts fly to their partners while the whole source stays; at the terminal progress the whole target takes the place."""

    def __init__(self, src, dst, trips, *, lag_ratio, run_time):
        self.source, self.final = src, dst
        self.stay, self.arrival, self._stay, self._arrive = src.copy(), dst.copy(), [], []
        super().__init__(*trips, lag_ratio=lag_ratio, run_time=run_time)

    def _setup_scene(self, scene):
        """🎬 Show shadows of source and target instead of the bound objects."""
        super()._setup_scene(scene)
        scene.remove(self.source)
        self._stay, self._arrive = _opacities(self.stay), _opacities(self.arrival)
        _scale_opacity(self._arrive, 0.0)
        scene.add(self.stay.add_updater(_live), self.arrival.add_updater(_live))

    def interpolate(self, alpha):
        """📈 Fly the copies; the terminal progress swaps source for target."""
        super().interpolate(alpha)
        landed = alpha >= 1.0
        _scale_opacity(self._stay, 0.0 if landed else 1.0)
        _scale_opacity(self._arrive, 1.0 if landed else 0.0)

    def clean_up_from_scene(self, scene):
        """🧹 Leave exactly the bound target in the scene."""
        super().clean_up_from_scene(scene)
        scene.remove(self.stay, self.arrival)
        scene.add(self.final)


class Move(_engine.Transform):
    """🚚 One identity moves, scales and restyles into the target (ReplacementTransform semantics on a copy, so the bound source is never mutated); `keep_source` leaves the source in place like TransformFromCopy."""

    def __init__(self, src, dst, *, keep_source=False, path_arc=0.0, dip=0.0, **kwargs):
        _instrument.count("full.prepare")
        self.source, self.final, self.keep_source, self.dip = src, dst, keep_source, dip
        super().__init__(src.copy(), dst, path_arc=path_arc, introducer=True, replace_mobject_with_target_in_scene=True, **kwargs)

    def _setup_scene(self, scene):
        """🎬 The moving copy replaces the source unless the source stays."""
        super()._setup_scene(scene)
        if not self.keep_source:
            scene.remove(self.source)

    def interpolate_mobject(self, alpha):
        """📈 Move, with the optional dip."""
        super().interpolate_mobject(alpha)
        _dip(self, alpha, self.dip)


class Dissolve(_engine.Animation):
    """🌫 FadeTransform on copies: the source copy stretches to the target's box while fading out, the target copy grows from the source's box while fading in; the bound source and target are never mutated."""

    def __init__(self, src, dst, *, stretch=True, keep_source=False, **kwargs):
        _instrument.count("full.prepare")
        self.source, self.final, self.keep_source = src, dst, keep_source
        self.out, self.into = src.copy(), dst.copy()
        self.out_start, self.into_end = self.out.copy(), self.into.copy()
        self.out_end, self.into_start = self._fit(self.out.copy(), dst, stretch), self._fit(self.into.copy(), src, stretch)
        self._out, self._into = _opacities(self.out), _opacities(self.into)
        super().__init__(_engine.VGroup(self.out, self.into), introducer=True, remover=True, **kwargs)

    @staticmethod
    def _fit(mob, box, stretch):
        """📐 Place a copy in the box of another mobject."""
        if mob.width > 1e-6 and mob.height > 1e-6 and box.width > 1e-6 and box.height > 1e-6:
            return mob.replace(box, stretch=stretch)
        return mob.move_to(box.get_center())

    def _setup_scene(self, scene):
        """🎬 The source copy takes the source's place."""
        super()._setup_scene(scene)
        if not self.keep_source:
            scene.remove(self.source)

    def interpolate_mobject(self, alpha):
        """📈 Cross-fade while both copies travel."""
        self.out.interpolate(self.out_start, self.out_end, alpha)
        self.into.interpolate(self.into_start, self.into_end, alpha)
        _scale_opacity(self._out, 1.0 - alpha)
        _scale_opacity(self._into, alpha)

    def clean_up_from_scene(self, scene):
        """🧹 Leave exactly the bound target in the scene."""
        super().clean_up_from_scene(scene)
        scene.add(self.final)


def _lookup(subject, name):
    """🔎 Part of a bound entity by name."""
    if name not in subject.parts:
        raise ValueError(f"entity {subject.id!r} has no part {name!r}; its parts are {sorted(subject.parts)}")
    return subject.parts[name]


def _by_key(source, target):
    """🗝 Parts that share a key, in source order."""
    return [(source.parts[k], target.parts[k], []) for k in source.parts if k in target.parts]


def _by_order(source, target):
    """🔢 Parts paired in list order."""
    return [(s, d, []) for s, d in zip(source.parts.values(), target.parts.values())]


def _capsule_item(source, target):
    """💊 Capsule drawings to item tokens by shared key (else list order), each with its shell-to-ring and window-to-dot part pairs."""
    couples = _by_key(source, target) or _by_order(source, target)
    return [(s, d, capsule_pairs(s, d)) for s, d, _ in couples]


def _auto(source, target):
    """🧭 Automatic part matching of the whole source with the whole target (by area, then position)."""
    return [(source.mobject, target.mobject, [])]


def _fan_in(source, target):
    """🧲 Every source part to the whole target."""
    return [(s, target.mobject, []) for s in source.parts.values()]


def _fan_out(source, target):
    """🌱 The whole source to every target part."""
    return [(source.mobject, d, []) for d in target.parts.values()]


def _capsule_whole(source, target):
    """💊 One capsule drawing to one item token: shell to ring, window to dot."""
    return [(source.mobject, target.mobject, capsule_pairs(source.mobject, target.mobject))]


def _item_capsule(source, target):
    """💊 One item token back to one capsule drawing: ring to shell, dot to window."""
    return [(source.mobject, target.mobject, [(t, s) for s, t in capsule_pairs(target.mobject, source.mobject)])]


def _body(source, target):
    """🫙 List order couples whose whole source shape flies into the first child (the body) of its target token."""
    return [(s, d, [(s, d.submobjects[0])] if d.submobjects else []) for s, d, _ in _by_order(source, target)]


def _body_whole(source, target):
    """🫙 One whole source shape flying into the first child (the body) of one whole target token."""
    return [(source.mobject, target.mobject, [(source.mobject, target.mobject.submobjects[0])] if target.mobject.submobjects else [])]


_CORRESPONDENCE = {"body": _body, "body-whole": _body_whole, "auto": _auto, "fan-in": _fan_in, "fan-out": _fan_out, "capsule-item": _capsule_whole, "item-capsule": _item_capsule, "capsule-items": _capsule_item}
CORRESPONDENCES = frozenset(_CORRESPONDENCE)


def _couples(transition, source, target):
    """🔗 (source part, target part, part pairs) triples of a transition's correspondence."""
    _instrument.count("full.prepare")
    c = transition.correspondence
    if c.kind == "identity":
        return [(source.mobject, target.mobject, [])]
    if c.kind == "key":
        return _by_key(source, target)
    if c.kind == "order":
        return _by_order(source, target)
    if c.kind == "pairs":
        return [(_lookup(source, s), _lookup(target, d), []) for s, d in c.pairs]
    if c.name not in _CORRESPONDENCE:
        raise ValueError(f"unknown correspondence {c.name!r}; registered are {sorted(_CORRESPONDENCE)}")
    return _CORRESPONDENCE[c.name](source, target)


def _unique(mobs):
    """🧮 Mobjects without repeats, in order."""
    return list({id(m): m for m in mobs}.values())


def _rest(parts, used):
    """🧩 Parts of a mapping that no couple uses."""
    taken = {id(m) for m in used}
    return [p for p in parts.values() if id(p) not in taken and _members(p)]


def _nearest(point, mobs, fallback):
    """🎯 Centre of the pointed mobject nearest to a point (the fallback point without any)."""
    centres = [_middle(m) for m in mobs if _members(m)]
    return min(centres, key=lambda c: np.linalg.norm(c - point)) if centres else fallback


def _grouped(animations, c):
    """🐝 Swarm of per-element animations (each runs the full duration) with the shared stagger windows; the group spans `timeline.span`."""
    windows = _timeline.expand(_timeline.Window(0.0, c.duration), len(animations), c.lag, c.limit)
    lag = _timeline.stagger_lag(c.duration, len(animations), c.lag, c.limit)
    return Swarm(*animations, lag_ratio=lag / c.duration, run_time=_timeline.span(windows).duration)


def _b_parts(source, target, couples, c):
    """🧩 `parts` and `hull`: one part-wise (or hull) morph of the whole source into the whole target."""
    whole = [(s, d, p) for s, d, p in couples if p or not (s is source.mobject and d is target.mobject)]
    pairs = [x for s, d, sub in whole for x in (sub or [(s, d)])]
    return _build(source.mobject, target.mobject, pairs=pairs or None, ghost=c.ghost, path_arc=c.arc, run_time=c.duration, dip=c.dip, mode="hull" if c.recipe == "hull" else "parts", rate_func=c.ease, keep_source=c.copy), c.copy


def _b_scatter(source, target, couples, c):
    """🎆 `scatter` and `unfold`: every couple flies on its own; leftovers vanish into or grow out of their nearest partner."""
    srcs, dsts = [s for s, _, _ in couples], [d for _, d, _ in couples]
    animations = [_lazy(lambda s=s, d=d, p=p: _build(s, d, pairs=p or None, ghost=c.ghost, path_arc=c.arc, run_time=c.duration, dip=c.dip, rate_func=c.ease), c.duration) for s, d, p in couples]
    if not any(s is source.mobject for s in srcs):
        animations += [_lazy(lambda p=p: _vanish(p, _nearest(_middle(p), dsts, target.anchors["center"]), path_arc=c.arc, run_time=c.duration), c.duration) for p in _rest(source.parts, srcs)]
    if not any(d is target.mobject for d in dsts):
        animations += [_engine.FadeIn(p.copy(), target_position=_nearest(_middle(p), srcs, source.anchors["center"]), scale=0.15, remover=True, run_time=c.duration) for p in _rest(target.parts, dsts)]
    return _grouped(animations, c), False


def _b_gather(source, target, couples, c):
    """🧲 `gather`: every source part collapses into the target while it fades in."""
    srcs = _unique([s for s, _, _ in couples if s is not source.mobject])
    srcs = [*srcs, *_rest(source.parts, srcs)] or [source.mobject]
    return _grouped(_gathered(srcs, target.mobject, path_arc=c.arc, run_time=c.duration, shadow=True), c), False


def _b_split(source, target, couples, c):
    """🌱 `split`: the source sprouts the targets; it stays visible for branches 2..n (and for all with the transition flag `copy`)."""
    dsts = _unique([d for _, d, _ in couples if d is not target.mobject])
    dsts = [*dsts, *_rest(target.parts, dsts)] or [target.mobject]
    return _grouped([_lazy(lambda i=i, d=d: _build(source.mobject, d, keep_source=c.copy or i > 0, ghost=c.ghost if i == 0 else 0.0, path_arc=c.arc, run_time=c.duration, rate_func=c.ease), c.duration) for i, d in enumerate(dsts)], c), c.copy


def _b_retext(source, target, couples, c):
    """✏ `retext`: matching glyph shapes slide, the rest transforms."""
    return _engine.TransformMatchingShapes(source.mobject, target.mobject, transform_mismatches=True, run_time=c.duration, path_arc=c.arc), False


def _tone(mob, fallback):
    """🎨 Mean fill colour of a mobject (the fallback without points)."""
    return _style(mob)[0] if _members(mob) else fallback


def _b_flight(source, target, couples, c):
    """✈️ `flight`: real copies of the source parts arc to their partners with colour carry-over while source and target stay; the target takes over at the terminal progress."""
    trips = [Trip(s.get_center(), d.get_center(), (_tone(s, "#ffffff"), _tone(d, "#ffffff")), (s.width, d.width), c.arc, mob=s.copy(), carry=True, run_time=c.duration, rate_func=c.ease) for s, d, _ in couples]
    windows = _timeline.expand(_timeline.Window(0.0, c.duration), len(trips), c.lag, c.limit)
    lag = _timeline.stagger_lag(c.duration, len(trips), c.lag, c.limit)
    return Flock(source.mobject, target.mobject, trips, lag_ratio=lag / c.duration, run_time=_timeline.span(windows).duration), False


def _b_move(source, target, couples, c):
    """🚚 `move` and `replace`: one identity moves, scales and restyles; the transition flag `copy` leaves the source (TransformFromCopy); a many-to-many transition with `lag` moves every couple on its own, staggered."""
    if couples:
        return _grouped([_lazy(lambda s=s, d=d: Move(s, d, keep_source=c.copy, path_arc=c.arc, dip=c.dip, run_time=c.duration, rate_func=c.ease), c.duration) for s, d, _ in couples], c), c.copy
    return Move(source.mobject, target.mobject, keep_source=c.copy, path_arc=c.arc, dip=c.dip, run_time=c.duration, rate_func=c.ease), c.copy


def _b_fade(source, target, couples, c):
    """🌫 `fade`: cross-fade of source and target copies."""
    if couples:
        return _grouped([_lazy(lambda s=s, d=d: Dissolve(s, d, keep_source=c.copy, run_time=c.duration, rate_func=c.ease), c.duration) for s, d, _ in couples], c), c.copy
    return Dissolve(source.mobject, target.mobject, keep_source=c.copy, run_time=c.duration, rate_func=c.ease), c.copy


_BUILDERS = {"parts": _b_parts, "hull": _b_parts, "scatter": _b_scatter, "unfold": _b_scatter, "gather": _b_gather, "split": _b_split, "retext": _b_retext, "flight": _b_flight, "move": _b_move, "replace": _b_move, "fade": _b_fade}


class _Settings:
    """⚙ Recipe parameters of one transition."""

    def __init__(self, transition):
        self.recipe, self.duration, self.copy = transition.recipe, float(transition.timing.duration), bool(getattr(transition, "copy", False))
        self.ease, self.lag, self.limit = (lambda p, name=transition.timing.easing: _timeline.ease(name, p)), float(transition.timing.lag), transition.timing.limit
        self.ghost, self.arc, self.dip = float(transition.final.ghost), float(transition.final.path_arc), float(transition.final.dip)


def _vanish_whole(source, c):
    """🫥 `vanish`: a source that morphs into nothing collapses into a point while fading out."""
    nothing = _engine.VGroup()
    return _build(source.mobject, nothing, ghost=c.ghost, path_arc=c.arc, run_time=c.duration, rate_func=c.ease), nothing


def _appear(target, c):
    """🌱 `appear`: a target without source grows into view."""
    return _engine.FadeIn(target.mobject.copy(), scale=0.85, remover=True, run_time=c.duration, rate_func=c.ease)


def execute(source, target, *, transition, fps):
    """🎞 The authored full morph of a transition as ONE Manim animation: `source`/`target` are stage subjects (`source` is None for `appear`, `target` None for `vanish`), the recipe, correspondence, easing, stagger and final parameters come from `transition`. Frames are driven by the timeline owner's progress, so the terminal frame f1 shows the exact bound target and the duration is unchanged (a staggered group spans `timeline.span`)."""
    if transition.recipe not in _BUILDERS and transition.recipe not in ("vanish", "appear"):
        raise ValueError(f"recipe {transition.recipe!r} is not supported; use one of {list(RECIPES)}")
    c = _Settings(transition)
    if c.duration <= 0:
        raise ValueError("a zero-duration transition is an atomic switch; it never executes a full morph")
    nothing, keep = None, c.copy
    if transition.recipe == "vanish":
        animation, nothing = _vanish_whole(source, c)
    elif transition.recipe == "appear":
        animation = _appear(target, c)
    else:
        animation, keep = _BUILDERS[transition.recipe](source, target, _couples(transition, source, target) if _spec.uses_parts(transition) else [], c)
    if c.copy and transition.recipe not in ("parts", "hull", "move", "replace", "fade"):
        raise ValueError(f"copy keeps the source and is only supported by parts, hull, move, replace and fade, not {transition.recipe!r}")
    inner_clean = animation.clean_up_from_scene

    def clean_up(scene):
        inner_clean(scene)
        if nothing is not None:
            scene.remove(nothing)
        if source is not None and not keep and (target is None or source.mobject is not target.mobject):
            scene.remove(source.mobject)
        if target is not None:
            scene.add(target.mobject)

    animation.clean_up_from_scene = clean_up
    return _counted(animation, lambda alpha: _timeline.sample_alpha(alpha, animation.run_time, fps, _clock.SCENE_CLOCK.phase).progress if alpha < 1 else 1.0)
