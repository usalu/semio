"""🔀 Morph toolkit: identity-preserving Manim morphs between heterogeneous shapes (concrete building parts to abstract tokens and back).

Domain-neutral; Manim only. Reference: https://docs.manim.community/en/stable/reference/manim.animation.transform.Transform.html
"""

from __future__ import annotations

import importlib

import numpy as np

_engine = importlib.import_module("manim")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")

INK, MUTED, DATA, GOOD, WARN, DARK = _art.P_WHITE, _art.PASTEL_GREY, _art.P_YELLOW, _art.P_GREEN, _art.P_ORANGE, _art.P_DEEP_DARK
KINDS = ("item", "cell", "path", "number", "plane", "text")
BASE = 0.4
SAMPLES = (0.0, 0.25, 0.5, 0.75)


def _members(mob):
    """🧩 Every family member that owns points."""
    return [m for m in mob.get_family() if isinstance(m, _engine.VMobject) and len(m.points) and np.isfinite(m.points).all()]


def _rgb(color):
    """🎨 Colour as an RGB triple."""
    return np.array(_engine.ManimColor(color).to_rgb())


def _style(mob):
    """🎨 Mean fill, first stroke and strongest opacity of a family: (fill, fill_opacity, stroke, stroke_width, stroke_opacity)."""
    parts = _members(mob)
    weights = np.array([max(m.get_fill_opacity(), 1e-6) * max(len(m.points), 1) for m in parts])
    fills = np.array([_rgb(m.get_fill_color()) for m in parts])
    fill = _engine.ManimColor.from_rgb(tuple(np.clip((fills * weights[:, None]).sum(0) / weights.sum(), 0, 1)))
    opacity = max(m.get_fill_opacity() for m in parts)
    lead = max(parts, key=lambda m: m.get_stroke_width() * m.get_stroke_opacity())
    return fill, opacity, lead.get_stroke_color(), lead.get_stroke_width(), lead.get_stroke_opacity()


def _outline(mob):
    """📍 Sampled outline points of every curve in a family, shape (n, 3)."""
    chunks = []
    for m in _members(mob):
        curves = m.points[: len(m.points) // 4 * 4].reshape(-1, 4, 3)
        if not len(curves):
            chunks.append(m.points)
            continue
        for t in SAMPLES:
            chunks.append(((1 - t) ** 3 * curves[:, 0] + 3 * (1 - t) ** 2 * t * curves[:, 1] + 3 * (1 - t) * t**2 * curves[:, 2] + t**3 * curves[:, 3]))
    return np.concatenate(chunks)


def _hull(points):
    """🔺 Convex hull (monotone chain) of planar points, counter-clockwise."""
    unique = sorted({(round(float(x), 5), round(float(y), 5)) for x, y, _ in points})
    if len(unique) < 3:
        return unique

    def half(sequence):
        out = []
        for p in sequence:
            while len(out) >= 2 and (out[-1][0] - out[-2][0]) * (p[1] - out[-2][1]) - (out[-1][1] - out[-2][1]) * (p[0] - out[-2][0]) <= 0:
                out.pop()
            out.append(p)
        return out[:-1]

    return half(unique) + half(unique[::-1])


def _restyle(shape, style, color):
    """🖌 Apply a captured style (and optional colour override) to a shape."""
    fill, opacity, stroke, width, stroke_opacity = style
    shape.set_fill(color or fill, opacity=opacity)
    shape.set_stroke(color or stroke, width=width, opacity=stroke_opacity)
    return shape


def _nothing():
    """⚪ Invisible zero-size shape standing in for geometry without points."""
    shape = _engine.VMobject()
    point = np.zeros(3)
    shape.set_points_as_corners([point, point + 1e-6])
    return shape.set_fill(opacity=0).set_stroke(opacity=0)


def _centre_of(mob):
    """🎯 Centre of the pointed parts of a family (origin when there are none)."""
    parts = _members(mob)
    return _outline(mob).mean(axis=0) if parts else np.zeros(3)


def silhouette(mob, *, color=None):
    """🫥 One closed outline for a multi-part group (convex hull of its curves, mean fill) as a clean morph source; a single shape is copied; parts without points are skipped and a pointless input gives an invisible stand-in."""
    parts = _members(mob)
    if not parts:
        return _nothing()
    if len(parts) == 1 and len(mob.get_family()) == 1:
        shape = mob.copy()
        return shape.set_color(color) if color else shape
    hull = _hull(_outline(mob))
    shape = _engine.VMobject()
    shape.set_points_as_corners([np.array([x, y, 0.0]) for x, y in (*hull, hull[0])])
    return _restyle(shape, _style(mob), color)


def proxy(mob, *, color=None):
    """🧱 Cheap stand-in for heavy groups: one bounding rectangle with the group's mean colour, never the thousands of faces; pointless input gives an invisible stand-in."""
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

    def __init__(self, src, dst, *, ghost=0.0, keep_source=False, consume=False, path_arc=0.0, run_time=1.4, dip=0.0, rate_func=_engine.smooth, **kwargs):
        self.source, self.final, self.ghost_level, self.keep_source, self.consume, self.dip = src, dst, ghost, keep_source, consume, dip
        self.ghost_mobject, self._leaving, self._arriving = None, [], []
        self.detailed_target = not consume and len(_members(dst)) != 1
        start = silhouette(src) if _members(src) else _nothing().move_to(_centre_of(dst))
        self.detailed_source = len(_members(src)) > 1 or len(src.get_family()) > 1
        super().__init__(start, (silhouette(dst) if _members(dst) else _nothing().move_to(start.get_center())) if self.detailed_target else dst, path_arc=path_arc, run_time=run_time, rate_func=rate_func, remover=consume, replace_mobject_with_target_in_scene=not (consume or self.detailed_target), **kwargs)

    def _setup_scene(self, scene):
        """🎬 Take the source out of the scene (details or a ghost stay for a moment) and prepare the arriving target."""
        super()._setup_scene(scene)
        if not self.keep_source:
            if self.ghost_level > 0 or self.detailed_source:
                self.ghost_mobject = self.source.copy()
                self._leaving = _opacities(self.ghost_mobject)
                scene.add(self.ghost_mobject.add_updater(_live))
                scene.bring_to_front(self.mobject)
            scene.remove(self.source)
        if self.detailed_target:
            self._arriving = _opacities(self.final)
            _scale_opacity(self._arriving, 0.0)
            scene.add(self.final.add_updater(_live))

    def _apply(self, a):
        """📈 Leaving details fade, arriving details appear, the silhouette hands over."""
        if self.ghost_mobject is not None:
            _scale_opacity(self._leaving, 1 - (1 - self.ghost_level) * _ramp(a, 0.0, self.LEAVE))
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
            self.final.remove_updater(_live)
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
    low, high = _outline(part).min(axis=0), _outline(part).max(axis=0)
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
    rest = [p for p in ranked[1:]]
    named = [p for p in rest if getattr(p, "name", "") == "window"]
    window = (named or rest or [None])[0]
    targets = [t for t in tok.submobjects if _members(t)] or [tok]
    pairs = [(shell, targets[0])]
    if window is not None:
        pairs.append((window, targets[1] if len(targets) > 1 else targets[0]))
    return pairs


HEAVY = 200


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
    """🧩 Parts to match: every pointed shape, or per-part proxy outlines (one per top-level part) above `HEAVY` shapes; one proxy of the whole group when even those are too many."""
    leaves = _members(mob)
    if len(leaves) <= HEAVY:
        return [_solo(m) for m in leaves]
    tops = _tops(mob)
    return [proxy(t) for t in tops] if 1 < len(tops) <= HEAVY else [proxy(mob)]


def _middle(part):
    """🎯 Centre of the bounding box of a part's outline."""
    outline = _outline(part)
    return (outline.min(axis=0) + outline.max(axis=0)) / 2


def _plan(src, dst):
    """🧭 Automatic part matching: parts sorted by area (then position) are paired greedily; extra source parts shrink towards the nearest partner's target, extra target parts grow from the nearest partner's source. None when either side has no parts."""
    def ordered(parts):
        keyed = [(-round(_extent(p), 3), *np.round(_middle(p)[:2], 2), i) for i, p in enumerate(parts)]
        return [parts[k[-1]] for k in sorted(keyed)]

    sp, dp = ordered(_parts(src)), ordered(_parts(dst))
    count = min(len(sp), len(dp))
    if not count:
        return None
    near = lambda point, group: int(np.argmin([np.linalg.norm(point - _middle(g)) for g in group[:count]]))
    pairs = list(zip(sp[:count], dp[:count]))
    shrinks = [(s, _middle(dp[near(_middle(s), sp)])) for s in sp[count:]]
    grows = [(d, _middle(sp[near(_middle(d), dp)])) for d in dp[count:]]
    return pairs, shrinks, grows


def morph(src, dst, *, pairs=None, ghost=0.0, path_arc=0.0, run_time=1.4, keep_source=False, dip=0.0, mode="parts"):
    """🔀 Concrete to abstract (or back): `src` leaves the scene, `dst` is in the scene afterwards, colours interpolate; `ghost` leaves a faint copy of the source (`anim.ghost_mobject`).

    Default `mode="parts"`: the parts of source and target are matched automatically (by area, then position) and fly to each other keeping their real shapes; extra source parts shrink towards their nearest partner's target, extra target parts grow from their nearest partner. Above 200 shapes per-part proxy outlines travel. `mode="hull"` opts into one convex-hull silhouette (a blob) instead.
    `pairs=[(src_part, dst_part), ...]` overrides the matching with your own pairs (window to dot, body to outline); unpaired parts dim and arrive with the whole. `dip` shrinks the flight slightly mid-way (0.1 to 0.2)."""
    if mode not in ("parts", "hull"):
        raise ValueError("mode is parts or hull")
    pairs = _sound(src, dst, pairs)
    if pairs:
        if keep_source:
            raise ValueError("pairs cannot be combined with keep_source")
        return PartMorph(src, dst, pairs, ghost=ghost, path_arc=path_arc, run_time=run_time, dip=dip)
    plan = _plan(src, dst) if mode == "parts" and (len(_members(src)) > 1 or len(_members(dst)) > 1) else None
    if plan:
        return PartMorph(src, dst, plan[0], shrinks=plan[1], grows=plan[2], ghost=ghost, path_arc=path_arc, run_time=run_time, dip=dip, keep_source=keep_source)
    return Morph(src, dst, ghost=ghost, path_arc=path_arc, run_time=run_time, keep_source=keep_source, dip=dip)
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

    def __init__(self, src, dst, pairs, *, shrinks=(), grows=(), ghost=0.0, path_arc=0.0, run_time=1.4, dip=0.0, keep_source=False):
        self.source, self.final, self.ghost_level, self.keep_source = src, dst, ghost, keep_source
        self.ghost_mobject, self._arriving = (None if keep_source else src.copy().add_updater(_live)), []
        fly = dict(dip=dip, path_arc=path_arc, run_time=run_time)
        self.flights = [Flight(silhouette(s), silhouette(d) if len(_members(d)) > 1 else d, **fly) for s, d in pairs]
        self.vanishing = [Flight(start := silhouette(s), start.copy().scale(0.15).move_to(point), **fly) for s, point in shrinks]
        self.growing = [Flight(_small(d, point), d, **fly) for d, point in grows]
        extra = [] if self.ghost_mobject is None else [Dim(self.ghost_mobject, ghost)]
        super().__init__(*extra, *self.flights, *self.vanishing, *self.growing, lag_ratio=0.0, run_time=run_time)

    def _setup_scene(self, scene):
        """🎬 Swap the source for its dimming copy and the flying parts; prepare the arriving target."""
        super()._setup_scene(scene)
        if not self.keep_source:
            scene.remove(self.source)
        if self.final is not None:
            self._arriving = _opacities(self.final)
            _scale_opacity(self._arriving, 0.0)
            scene.add(self.final.add_updater(_live))

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
            _scale_opacity(self._arriving, 1.0)
            self.final.remove_updater(_live)
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
    lag = min(lag, max(limit - run_time, 0.0) / max(count - 1, 1))
    return Swarm(*animations, lag_ratio=lag / run_time if run_time else 0.0)


def scatter(srcs, dsts, *, lag=0.04, ghost=0.0, path_arc=0.6, run_time=1.2, limit=4.0, pairs_fn=None, dip=0.0):
    """🎆 Many to many in order (140 capsules to 140 leaves); every source flies to its own target. `pairs_fn(src, dst)` returns the part pairs of `morph(pairs=...)` for each couple."""
    srcs, dsts = list(srcs), list(dsts)
    if len(srcs) != len(dsts):
        raise ValueError("scatter needs as many targets as sources")
    return _staggered([morph(s, d, pairs=pairs_fn(s, d) if pairs_fn else None, ghost=ghost, path_arc=path_arc, run_time=run_time, dip=dip) for s, d in zip(srcs, dsts)], run_time, lag, limit)


def gather(srcs, dst, *, lag=0.02, path_arc=0.4, run_time=1.0, limit=3.0):
    """🧲 Many to one: sources collapse into the target's place while it appears (a count, a list, a number)."""
    srcs = list(srcs)
    sink = silhouette(dst)
    sink.set_fill(opacity=sink.get_fill_opacity() * 0.5).set_stroke(opacity=sink.get_stroke_opacity() * 0.5)
    point = _middle(dst) if _members(dst) else dst.get_center()
    absorbed = [PartMorph(s, None, [], shrinks=[(p, point) for p in _parts(s)], path_arc=path_arc, run_time=run_time) if len(_members(s)) > 1 else Morph(s, sink.copy(), consume=True, path_arc=path_arc, run_time=run_time) for s in srcs]
    return _staggered([*absorbed, _engine.FadeIn(dst, scale=0.85, run_time=run_time)], run_time, lag, limit)


def split(src, dsts, *, lag=0.06, path_arc=0.3, ghost=0.0, run_time=1.2, limit=3.0):
    """🌱 One to many: the source sprouts the targets (a list into branches, a value into cells)."""
    dsts = list(dsts)
    return _staggered([morph(src, d, keep_source=i > 0, ghost=ghost if i == 0 else 0.0, path_arc=path_arc, run_time=run_time) for i, d in enumerate(dsts)], run_time, lag, limit)


def _text(label, size, color, mono=False):
    """🔤 Token label in the bundled sans or mono face."""
    if mono:
        return _engine.Text(label, font=_theme.FONT_MONO, font_size=size, color=color, disable_ligatures=True)
    return _fonts.body_text(label, font_size=size, color=color)


def _pill(text, color, corner, fill, text_color=None):
    """💊 Rounded box around a label; returns (box, text)."""
    height = BASE
    width = max(height, text.width + 0.26)
    box = _engine.RoundedRectangle(width=width, height=height, corner_radius=min(corner, height / 2, width / 2), stroke_color=color, stroke_width=2.0, fill_color=fill, fill_opacity=1.0 if text_color else 0.18)
    return box, text.move_to(box)


def token(label, *, kind="item", color=DATA, size=1.0):
    """🪙 Abstract stand-in with one visual language: item (dot and value), cell (chip), path (`{0;1;3;1}` pill), number (filled badge), plane (square with axes), text (glyph)."""
    if kind not in KINDS:
        raise ValueError(f"unknown token kind {kind!r}")
    label = str(label)
    small = 16.2 * size
    if kind == "item":
        dot = _engine.Dot(radius=0.1 * size, color=color)
        ring = _engine.Circle(radius=0.19 * size, stroke_color=color, stroke_width=1.5, stroke_opacity=0.6, fill_opacity=0).move_to(dot)
        parts = [ring, dot] + ([_text(label, small, INK).next_to(ring, _engine.RIGHT, buff=0.08 * size)] if label else [])
    elif kind == "cell":
        parts = list(_pill(_text(label, small, INK), color, 0.05, color))
    elif kind == "path":
        parts = list(_pill(_text(label, small, color, mono=True), color, 1.0, color))
    elif kind == "number":
        parts = list(_pill(_text(label, small, DARK), color, 1.0, color, text_color=DARK))
    elif kind == "plane":
        side = BASE * 1.1 * size
        square = _engine.Square(side_length=side, stroke_color=color, stroke_width=2.0, fill_color=color, fill_opacity=0.18)
        origin = square.get_corner(_engine.DL)
        x = _engine.Arrow(origin, origin + _engine.RIGHT * side * 0.8, buff=0, stroke_width=3, color=_art.P_RED, max_tip_length_to_length_ratio=0.35)
        y = _engine.Arrow(origin, origin + _engine.UP * side * 0.8, buff=0, stroke_width=3, color=GOOD, max_tip_length_to_length_ratio=0.35)
        parts = [square, x, y] + ([_text(label, small, INK).next_to(square, _engine.RIGHT, buff=0.1 * size)] if label else [])
    else:
        parts = [_text(label, small * 1.15, color)]
    group = _engine.VGroup(*parts)
    group.kind = kind
    return group.move_to(_engine.ORIGIN)


def travel(tok, path, *, run_time=1.2, rate_func=_engine.smooth):
    """🚚 Run a token along a wire or leader (a path mobject or a list of points); it rests at the end."""
    if not isinstance(path, _engine.Mobject):
        path = _engine.VMobject().set_points_smoothly([np.array(p, dtype=float) for p in path])
    return _engine.MoveAlongPath(tok, path, run_time=run_time, rate_func=rate_func)


def _centre(x):
    """🎯 Centre of a mobject or a point."""
    return np.array(x.get_center() if isinstance(x, _engine.Mobject) else x, dtype=float)


def _edge(x, toward):
    """🧭 Point where the ray from the centre of `x` towards `toward` leaves its bounding box."""
    centre = _centre(x)
    if not isinstance(x, _engine.Mobject):
        return centre
    delta = toward - centre
    scale = min((x.width / 2) / abs(delta[0]) if abs(delta[0]) > 1e-9 else np.inf, (x.height / 2) / abs(delta[1]) if abs(delta[1]) > 1e-9 else np.inf, 1.0)
    return centre + delta * scale


def tether(a, b, *, color=DATA, dashed=True, width=2.0):
    """🪢 Leader between two mobjects (or points) that follows both ends while they move."""

    def build():
        ca, cb = _centre(a), _centre(b)
        p, q = _edge(a, cb), _edge(b, ca)
        if np.dot(cb - ca, q - p) <= 1e-4:
            return _engine.VMobject()
        if dashed:
            return _engine.DashedLine(p, q, color=color, stroke_width=width, dash_length=0.08, dashed_ratio=0.55)
        return _engine.Line(p, q, color=color, stroke_width=width)

    return _engine.always_redraw(build)


def unfold(faces, layout, *, lag=0.08, path_arc=0.0, run_time=1.4, limit=3.0):
    """📖 Faces of a box unfold into a net or into rows: `layout` holds one target mobject or point per face."""
    faces, layout = list(faces), list(layout)
    if len(faces) != len(layout):
        raise ValueError("unfold needs one layout entry per face")

    def target(face, spot):
        return spot if isinstance(spot, _engine.Mobject) else face.copy().move_to(spot)

    return _staggered([morph(f, target(f, s), path_arc=path_arc, run_time=run_time) for f, s in zip(faces, layout)], run_time, lag, limit)


def retext(a, b, *, run_time=1.2, path_arc=0.0):
    """✏ Formula or text morph: matching glyph shapes slide, the rest transforms."""
    return _engine.TransformMatchingShapes(a, b, transform_mismatches=True, run_time=run_time, path_arc=path_arc)
