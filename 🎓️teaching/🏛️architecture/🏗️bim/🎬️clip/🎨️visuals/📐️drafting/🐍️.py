"""📐 Drafting vocabulary of the BIM clips: palette, line weights, projections, hatches, dimensions and live drawings."""

from __future__ import annotations

import importlib
import math

import numpy as np

_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
_engine = importlib.import_module("manim")


def mix(first: str, second: str, amount: float) -> str:
    """🎨 Blend two palette colours; amount 0 is the first, 1 the second."""
    return _engine.interpolate_color(_engine.ManimColor(first), _engine.ManimColor(second), float(amount)).to_hex()


INK = _art.P_WHITE
MUTED = _art.PASTEL_GREY
ARCH = _art.P_CYAN
STRUCT = _art.PASTEL_PURPLE
SERV = _art.P_RED
DATA = _art.P_YELLOW
GOOD = _art.P_GREEN
WARN = _art.P_ORANGE
GLASS = _art.P_CYAN
BACKGROUND = _art.P_DEEP_DARK
CONCRETE = _theme.color("light-gray")
GREEN_GROUND = mix(_art.P_GREEN, BACKGROUND, 0.52)
GROUND_PLANE = mix(_art.P_GREEN, BACKGROUND, 0.82)
DISCIPLINES = {"architecture": ARCH, "structure": STRUCT, "services": SERV}

S_SMALL, S_LABEL, S_BODY = 16.2, 19.2, 21.6
W_CUT, W_EDGE, W_THIN, W_HAIR = 3.2, 1.9, 1.2, 0.7
NARROW = " "
DEFAULT_AZIMUTH, DEFAULT_ELEVATION = -40.0, 28.0


def P(point) -> np.ndarray:
    """📍 Coerce a 2D or 3D point to a float 3-vector."""
    values = np.zeros(3)
    for index, value in enumerate(point):
        if index < 3:
            values[index] = value
    return values


def circle_points(centre, radius: float, count: int = 28, start: float = 0.0, stop: float = 360.0) -> list:
    """⭕ Sample a circle or arc counter-clockwise in degrees."""
    angles = np.radians(np.linspace(start, stop, count + 1))
    return [P((centre[0] + radius * math.cos(a), centre[1] + radius * math.sin(a))) for a in angles]


def segments(pairs, color=INK, width=W_THIN, opacity=1.0):
    """〰️ Many disjoint straight segments as one shape; the cheapest way to draw hatches, grids and dashes."""
    shape = _engine.VMobject(stroke_color=color, stroke_width=width, stroke_opacity=opacity, fill_opacity=0)
    array = np.asarray([[P(a), P(b)] for a, b in pairs], float).reshape(-1, 2, 3) if len(pairs) else np.zeros((0, 2, 3))
    array = array[np.linalg.norm(array[:, 1] - array[:, 0], axis=1) > 1e-6]
    if len(array):
        first, last = array[:, 0], array[:, 1]
        shape.set_points(np.stack([first, first + (last - first) / 3, first + 2 * (last - first) / 3, last], axis=1).reshape(-1, 3))
    return shape


def dashes(points, dash: float = 0.12, gap: float = 0.08, closed: bool = False) -> list:
    """┄ Break a polyline into dash segments of fixed screen length."""
    pts = [P(p) for p in points] + ([P(points[0])] if closed else [])
    pairs, carry, drawing = [], 0.0, True
    for a, b in zip(pts, pts[1:]):
        length = float(np.linalg.norm(b - a))
        if length < 1e-9:
            continue
        direction, position = (b - a) / length, 0.0
        while position < length - 1e-9:
            span = (dash if drawing else gap) - carry
            step = min(span, length - position)
            if drawing:
                pairs.append((a + direction * position, a + direction * (position + step)))
            position += step
            carry = carry + step if step < span - 1e-9 else 0.0
            if carry == 0.0:
                drawing = not drawing
    return pairs


def polyline(points, color=INK, width=W_EDGE, opacity=1.0, closed=False, dash: float | None = None):
    """➰ An open or closed polyline, optionally dashed."""
    pts = [P(p) for p in points]
    if dash:
        return segments(dashes(pts, dash, dash * 0.65, closed), color, width, opacity)
    shape = _engine.VMobject(stroke_color=color, stroke_width=width, stroke_opacity=opacity, fill_opacity=0)
    shape.set_points_as_corners(pts + ([pts[0]] if closed else []))
    return shape


def polygon(points, fill=None, opacity=1.0, stroke=None, width=W_EDGE, stroke_opacity=1.0):
    """⬟ A closed polygon with optional fill and outline."""
    pts = [P(p) for p in points]
    shape = _engine.VMobject()
    shape.set_points_as_corners(pts + [pts[0]])
    shape.set_fill(fill or INK, opacity if fill else 0)
    shape.set_stroke(stroke or fill or INK, width if stroke else 0, stroke_opacity if stroke else 0)
    return shape


def signed_area(ring) -> float:
    """📐 Shoelace area; positive when counter-clockwise."""
    pts = np.asarray([P(p)[:2] for p in ring])
    return 0.5 * float(np.sum(pts[:, 0] * np.roll(pts[:, 1], -1) - np.roll(pts[:, 0], -1) * pts[:, 1]))


def region(rings, fill=None, opacity=1.0, stroke=None, width=W_EDGE, stroke_opacity=1.0):
    """🕳️ A filled outline with holes; holes wind against the outer ring so every fill rule agrees."""
    shape = _engine.VMobject()
    for index, ring in enumerate(rings):
        pts = [P(p) for p in ring]
        if (signed_area(pts) > 0) != (index == 0):
            pts.reverse()
        shape.start_new_path(pts[0])
        for point in pts[1:] + [pts[0]]:
            shape.add_line_to(point)
    shape.set_fill(fill or INK, opacity if fill else 0)
    shape.set_stroke(stroke or fill or INK, width if stroke else 0, stroke_opacity if stroke else 0)
    return shape


def hatch_pairs(rings, spacing: float = 0.09, angle: float = 45.0) -> list:
    """▨ Parallel lines clipped to a polygon with holes (even-odd rule); coordinates are screen units."""
    rings = [np.asarray([P(p)[:2] for p in ring]) for ring in rings]
    theta = math.radians(angle)
    along, across = np.array([math.cos(theta), math.sin(theta)]), np.array([-math.sin(theta), math.cos(theta)])
    offsets = np.concatenate([ring @ across for ring in rings])
    pairs, level = [], float(offsets.min()) + spacing / 2
    while level < float(offsets.max()):
        hits = []
        for ring in rings:
            height = ring @ across - level
            for index in range(len(ring)):
                a, b = ring[index], ring[(index + 1) % len(ring)]
                ha, hb = height[index], height[(index + 1) % len(ring)]
                if (ha < 0) != (hb < 0):
                    hits.append(float((a + (b - a) * (ha / (ha - hb))) @ along))
        hits.sort()
        for first, second in zip(hits[0::2], hits[1::2]):
            pairs.append((along * first + across * level, along * second + across * level))
        level += spacing
    return pairs


def hatch(rings, color=MUTED, spacing: float = 0.09, angle: float = 45.0, width=W_HAIR, opacity=0.9):
    """▨ A hatch over a screen polygon or polygon with holes."""
    rings = [rings] if len(rings) and np.ndim(rings[0]) == 1 else rings
    return segments(hatch_pairs(rings, spacing, angle), color, width, opacity)


def shade(colour: str, factor: float) -> str:
    """🌗 Darken a face colour toward the background."""
    return mix(BACKGROUND, colour, factor)


def place(shape, point, anchor: str = "c"):
    """📌 Move a shape so the chosen anchor sits on a point: l r u d combinations, b puts the text baseline on the point's y, c is the centre."""
    centre = np.array(shape.get_center())
    dx = shape.width / 2 if "l" in anchor else -shape.width / 2 if "r" in anchor else 0.0
    dy = -shape.height / 2 if "u" in anchor else shape.height / 2 if "d" in anchor else 0.0
    if "b" in anchor:
        dy = centre[1] - (shape.get_bottom()[1] + getattr(shape, "descent", 0.0))
    return shape.shift(P(point) - centre + np.array([dx, dy, 0.0]))


DESCENDERS = frozenset("gjpqyQ,;()[]{}$|_@/")


def descent(size: float, mono: bool = False) -> float:
    """🔻 Depth of descenders below the baseline for one type size."""
    return _descent(size, mono)


def _descent(size: float, mono: bool) -> float:
    key = ("descent", size, mono)
    if key not in _CACHE:
        if mono:
            _fonts.register_fonts(_theme.FONT_MONO)
            tall = _engine.Text("H", font=_theme.FONT_MONO, font_size=size).height
            deep = _engine.Text("Hg", font=_theme.FONT_MONO, font_size=size).height
        else:
            tall = _fonts.body_text("H", font_size=size, color=INK).height
            deep = _fonts.body_text("Hg", font_size=size, color=INK).height
        _CACHE[key] = deep - tall
    return _CACHE[key]


_CACHE: dict = {}


class View:
    """🔭 One projection for every drawing: plan, axonometric, elevation and section share ``view(x, y, z)`` and ``depth``."""

    def __init__(self, kind: str, point=(0, 0, 0), scale: float = 0.3, centre=(0, 0, 0), *, azimuth=DEFAULT_AZIMUTH,
                 elevation=DEFAULT_ELEVATION, face: str = "south", cut: float = 0.0, axis: str = "x"):
        self.kind, self.point, self.scale, self.centre = kind, P(point), float(scale), P(centre)
        self.azimuth, self.elevation, self.face, self.cut, self.axis = azimuth, elevation, face, cut, axis
        phi, eps = math.radians(azimuth), math.radians(elevation)
        self.right = np.array([-math.sin(phi), math.cos(phi), 0.0])
        self.up = np.array([-math.sin(eps) * math.cos(phi), -math.sin(eps) * math.sin(phi), math.cos(eps)])
        self.towards = np.array([math.cos(phi) * math.cos(eps), math.sin(phi) * math.cos(eps), math.sin(eps)])

    def __call__(self, x, y=None, z=0.0) -> np.ndarray:
        if y is None:
            x, y, z = (list(x) + [0.0])[:3]
        d = np.array([x, y, z], float) - self.centre
        if self.kind == "plan":
            local = (d[0], d[1])
        elif self.kind == "axon":
            local = (d @ self.right, d @ self.up)
        elif self.kind == "section":
            local = (d[0], d[2]) if self.axis == "x" else (d[1], d[2])
        else:
            local = {"south": (d[0], d[2]), "north": (-d[0], d[2]), "east": (d[1], d[2]), "west": (-d[1], d[2])}[self.face]
        return self.point + self.scale * np.array([local[0], local[1], 0.0])

    def depth(self, x, y, z=0.0) -> float:
        """👁️ Larger is closer to the viewer."""
        if self.kind == "axon":
            return float(np.array([x, y, z]) @ self.towards)
        if self.kind == "plan":
            return float(z)
        if self.kind == "section":
            return float(-y) if self.axis == "x" else float(x)
        return float({"south": -y, "north": y, "east": x, "west": -x}[self.face])

    def facing(self, normal) -> float:
        """🧭 Positive when a face with this outward normal looks at the viewer."""
        n = np.array(normal, float)
        if self.kind == "axon":
            return float(n @ self.towards)
        if self.kind == "plan":
            return float(n[2])
        if self.kind == "section":
            return float(-n[1]) if self.axis == "x" else float(n[0])
        return float({"south": -n[1], "north": n[1], "east": n[0], "west": -n[0]}[self.face])


class Drawing(_engine.VGroup):
    """🏛️ A drawing group: ``parts[tag]`` sub-drawings, ``layers[name]``, ``anchors`` and ``at(...)`` back to screen space."""

    def __init__(self, view: View | None = None):
        super().__init__()
        self.view, self.parts, self.layers, self.anchors = view, {}, {}, {}

    def at(self, *args) -> np.ndarray:
        """📍 ``at("B3")`` is a part's centre; ``at(x, y, z)`` projects a model coordinate with this drawing's view."""
        if len(args) == 1 and isinstance(args[0], str):
            return self.anchors[args[0]] if args[0] in self.anchors else np.array(self.parts[args[0]].get_center())
        return self.view(*args)

    def part(self, tag: str, shape, anchor=None):
        """🏷️ Register an addressable sub-drawing."""
        if tag in self.parts and self.parts[tag] is not shape:
            self.parts[tag].add(shape)
        else:
            self.parts[tag] = shape if isinstance(shape, _engine.VGroup) else _engine.VGroup(shape)
        if anchor is not None:
            self.anchors[tag] = P(anchor)
        return self.parts[tag]


_STALE = object()


class Live(_engine.VGroup):
    """♻️ A drawing rebuilt from its builder each frame, or only when ``key()`` changes; animations see frozen snapshots."""

    def __init__(self, builder, key=None):
        built = builder()
        super().__init__(*built.submobjects)
        self._builder, self._key, self._seen, self._deferred, self.current = builder, key, _STALE, False, built
        self.add_updater(lambda mob: mob.refresh())
        if key is not None:
            self._seen = key()

    def __getattr__(self, name):
        if name in ("current", "_builder"):
            raise AttributeError(name)
        return getattr(self.current, name)

    def refresh(self):
        """🔄 Rebuild from the current inputs."""
        if self._builder is None or self._deferred:
            return self
        if self._key is not None:
            seen = self._key()
            if seen == self._seen:
                return self
            self._seen = seen
        self.current = self._builder()
        self.submobjects = self.current.submobjects
        return self

    def resume_updating(self, recursive=True):
        self._deferred = True
        try:
            return super().resume_updating(recursive)
        finally:
            self._deferred = False
            self._seen = _STALE

    def __deepcopy__(self, memo):
        self.refresh()
        twin = super().__deepcopy__(memo)
        twin.updaters, twin._builder, twin._key = [], None, None
        return twin


class Drafting:
    """✏️ Scene mixin: labels, dimensions, level marks, leaders, hatches, scale bars and live redraws."""

    def label(self, en: str, de: str | None = None, point=(0, 0, 0), *, size=S_LABEL, color=INK, width=None, anchor="c"):
        """🏷️ A localized label; ``de`` omitted means the string is already final."""
        return self.text(en if de is None else self.tr(en, de), point, size=size, color=color, width=width, anchor=anchor)

    def text(self, string: str, point=(0, 0, 0), *, size=S_LABEL, color=INK, width=None, anchor="c"):
        """🔤 One string in the bundled sans face, optionally squeezed to a width."""
        shape = _fonts.body_text(string, font_size=size, color=color)
        shape.descent = descent(size) if DESCENDERS & set(string) else 0.0
        if width and shape.width > width:
            shape.descent *= width / shape.width
            shape.scale_to_fit_width(width)
        return place(shape, point, anchor)

    def mono(self, string: str, point=(0, 0, 0), *, size=S_SMALL, color=INK, anchor="c"):
        """⌨️ One string in the bundled monospace face."""
        _fonts.register_fonts(_theme.FONT_MONO)
        shape = _engine.Text(string, font=_theme.FONT_MONO, font_size=size, color=color, disable_ligatures=True)
        shape.descent = descent(size, True) if DESCENDERS & set(string) else 0.0
        return place(shape, point, anchor)

    def quantity(self, value: float, unit: str = "", digits: int = 2) -> str:
        """📏 A number with its unit, language punctuation and a narrow no-break space."""
        return self.number(value, digits) + (NARROW + unit if unit else "")

    def live(self, builder, key=None):
        """♻️ Redraw ``builder()`` every frame (or when ``key()`` changes) while keeping fade and transform animations safe."""
        return Live(builder, key)

    def live_label(self, fn, point=(0, 0, 0), *, size=S_LABEL, color=INK, anchor="c"):
        """🔢 A label whose string comes from ``fn()``; re-typeset only when the string changes."""
        return Live(lambda: _engine.VGroup(self.text(fn(), point, size=size, color=color, anchor=anchor)), fn)

    def dimension(self, start, end, text: str | None = None, *, offset: float = 0.3, color=DATA, size=S_SMALL, tick: float = 0.07,
                  text_at: float = 0.5, flip: bool = False):
        """↔️ Architectural dimension: extension lines, a line with oblique ticks and a horizontal label. Positive offset is to the left of start → end."""
        a, b = P(start), P(end)
        direction = (b - a) / max(np.linalg.norm(b - a), 1e-9)
        normal = np.array([-direction[1], direction[0], 0.0]) * (-1 if flip else 1)
        sign = 1.0 if offset >= 0 else -1.0
        p, q = a + normal * offset, b + normal * offset
        over = normal * sign * 0.07
        shape = _engine.VGroup(
            segments([(a + normal * sign * 0.05, p + over), (b + normal * sign * 0.05, q + over)], color, W_HAIR, 0.9),
            segments([(p, q)], color, W_THIN),
            segments([(p + (direction + normal) * tick, p - (direction + normal) * tick),
                      (q + (direction + normal) * tick, q - (direction + normal) * tick)], color, W_EDGE))
        if text:
            label = self.text(text, (0, 0, 0), size=size, color=color)
            centre = p + (q - p) * text_at
            shift = abs(normal[0]) * label.width / 2 + abs(normal[1]) * label.height / 2 + 0.07
            shape.add(label.move_to(centre + normal * sign * shift))
            shape.label = label
        shape.line = shape[1]
        return shape

    def level_mark(self, point, text: str, *, color=INK, size=S_SMALL, length: float = 0.9, side: int = 1):
        """🔻 Level mark: inverted triangle on a line with its elevation text; side 1 extends right, -1 left."""
        x, y, _ = P(point)
        s = 0.09
        tri = polygon([(x, y), (x - s, y + 1.6 * s), (x + s, y + 1.6 * s)], None, stroke=color, width=W_EDGE)
        rule = segments([(P((x, y)), P((x + side * length, y)))], color, W_THIN)
        label = self.text(text, (x + side * (length + 0.05), y), size=size, color=color, anchor="l" if side > 0 else "r")
        return _engine.VGroup(tri, rule, label)

    def hatch(self, rings, color=MUTED, spacing: float = 0.09, angle: float = 45.0, width=W_HAIR, opacity=0.9):
        """▨ A hatch clipped to a polygon (or polygon with holes) in screen coordinates."""
        return hatch(rings, color, spacing, angle, width, opacity)

    def leader(self, source, target, text: str | None = None, *, color=DATA, size=S_SMALL, dot: bool = True, anchor: str | None = None):
        """➖ A leader from a mobject (or point) to a target point, with a dot at the target and an optional label at the start."""
        t = P(target)
        if hasattr(source, "get_center"):
            c = np.array(source.get_center())
            start = np.array([min(max(t[0], source.get_left()[0]), source.get_right()[0]), min(max(t[1], source.get_bottom()[1]), source.get_top()[1]), 0.0])
            if np.linalg.norm(start - t) < 1e-6:
                start = c
        else:
            start = P(source)
        shape = _engine.VGroup(segments([(start, t)], color, W_THIN))
        if dot:
            shape.add(_engine.Dot(t, radius=0.045, color=color))
        if text:
            direction = (start - t) / max(np.linalg.norm(start - t), 1e-9)
            label = self.text(text, start, size=size, color=color, anchor=anchor or ("l" if direction[0] >= 0 else "r"))
            label.shift(np.array([0.06 if direction[0] >= 0 else -0.06, 0.0, 0.0]) + np.array([0, 0.1 * (1 if direction[1] >= 0 else -1), 0]))
            shape.add(label)
            shape.label = label
        return shape

    def badge(self, text: str, point=(0, 0, 0), *, color=GOOD, size=S_SMALL, filled: bool = True, anchor="c"):
        """🔖 A small rounded status badge."""
        label = self.text(text, (0, 0, 0), size=size, color=BACKGROUND if filled else color)
        frame = _engine.RoundedRectangle(corner_radius=0.07, width=label.width + 0.24, height=label.height + 0.16, stroke_color=color,
                                         stroke_width=W_THIN, fill_color=color, fill_opacity=0.95 if filled else 0.0)
        group = _engine.VGroup(frame, label.move_to(frame))
        group.frame, group.label = frame, label
        return place(group, point, anchor)

    def north_arrow(self, point, size: float = 0.5, *, color=INK, label: str = "N"):
        """🧭 North arrow: circle, half-filled needle and N."""
        c = P(point)
        ring = _engine.Circle(radius=size / 2, stroke_color=color, stroke_width=W_THIN).move_to(c)
        needle = polygon([c + (0, size * 0.42, 0), c + (-size * 0.16, -size * 0.3, 0), c + (0, -size * 0.12, 0), c + (size * 0.16, -size * 0.3, 0)], color, 0.9, color, W_HAIR)
        letter = self.text(label, c + (0, size * 0.5 + 0.14, 0), size=S_SMALL, color=color)
        return _engine.VGroup(ring, needle, letter)

    def scale_bar(self, point, scale: float, metres: float = 10.0, *, color=INK, steps: int = 2):
        """📏 Alternating scale bar of ``metres`` metres starting at ``point`` (left end)."""
        c = P(point)
        step = metres / steps * scale
        shape = _engine.VGroup()
        for index in range(steps):
            x0 = c[0] + index * step
            shape.add(polygon([(x0, c[1]), (x0 + step, c[1]), (x0 + step, c[1] + 0.06), (x0, c[1] + 0.06)],
                              color if index % 2 == 0 else None, 0.95, color, W_HAIR))
        for index in range(steps + 1):
            value = metres * index / steps
            shape.add(self.text(self.number(value, 0) + (NARROW + "m" if index == steps else ""), (c[0] + index * step, c[1] - 0.04), size=S_SMALL, color=color, anchor="u"))
        return shape
