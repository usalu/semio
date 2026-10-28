"""📏 Shared sheet vocabulary of the orthographic Villa Savoye drawings: pens, selection tones, cached labels, poché, dimensions, grid bubbles, plane cuts and the ground enclosure."""

from __future__ import annotations

import importlib
import math

import numpy as np

_PREFIX = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_engine = importlib.import_module("manim")

P = _d.P
INK, MUTED, DATA, GOOD, WARN, GLASS, BACKGROUND = _d.INK, _d.MUTED, _d.DATA, _d.GOOD, _d.WARN, _d.GLASS, _d.BACKGROUND
CONCRETE, GREEN_GROUND, GROUND_PLANE, SERV = _d.CONCRETE, _d.GREEN_GROUND, _d.GROUND_PLANE, _d.SERV
S_SMALL, S_LABEL = _d.S_SMALL, _d.S_LABEL
W_CUT, W_EDGE, W_THIN, W_HAIR = _d.W_CUT, _d.W_EDGE, _d.W_THIN, _d.W_HAIR

WHITE_BOX = _d.mix(INK, BACKGROUND, 0.1)
SPACING = 0.075
MIN_HATCH = 0.1
MIN_PILOTI = 0.05
WALL_T = 0.2
BEYOND_FILL = _d.mix(INK, BACKGROUND, 0.88)
EPS = 1e-9
_TEXTS: dict = {}
_CARS: dict = {}
_WALLS: dict = {}


def font_text(scene, string: str, point=(0, 0, 0), size: float = S_SMALL, color: str = INK, anchor: str = "c"):
    """🗄️ A label typeset once per string, size and colour; every call returns a fresh copy placed at ``point``."""
    key = (string, round(size, 2), str(color))
    proto = _TEXTS.get(key)
    if proto is None:
        proto = _TEXTS[key] = scene.text(string, (0, 0, 0), size=size, color=color)
        if len(_TEXTS) > 900:
            _TEXTS.pop(next(iter(_TEXTS)))
    return _d.place(proto.copy(), point, anchor)


def dim(scene, start, end, text: str | None = None, *, offset: float = 0.3, color: str = DATA, size: float = S_SMALL, tick: float = 0.07,
        text_at: float = 0.5, flip: bool = False, opacity: float = 1.0):
    """↔️ Architectural dimension with cached label: extension lines, line with oblique ticks, horizontal text; positive offset is left of start → end."""
    a, b = P(start), P(end)
    direction = (b - a) / max(np.linalg.norm(b - a), 1e-9)
    normal = np.array([-direction[1], direction[0], 0.0]) * (-1 if flip else 1)
    sign = 1.0 if offset >= 0 else -1.0
    p, q = a + normal * offset, b + normal * offset
    over = normal * sign * 0.07
    shape = _engine.VGroup(
        _d.segments([(a + normal * sign * 0.05, p + over), (b + normal * sign * 0.05, q + over)], color, W_HAIR, 0.9 * opacity),
        _d.segments([(p, q)], color, W_THIN, opacity),
        _d.segments([(p + (direction + normal) * tick, p - (direction + normal) * tick), (q + (direction + normal) * tick, q - (direction + normal) * tick)], color, W_EDGE, opacity))
    if text:
        label = font_text(scene, text, (0, 0, 0), size, color)
        label.set_opacity(opacity)
        centre = p + (q - p) * text_at
        shift = abs(normal[0]) * label.width / 2 + abs(normal[1]) * label.height / 2 + 0.07
        shape.add(label.move_to(centre + normal * sign * shift))
        shape.label = label
    shape.line = shape[1]
    return shape


def level_mark(scene, point, text: str, *, color: str = INK, size: float = S_SMALL, length: float = 0.6, side: int = 1):
    """🔻 Level mark: inverted triangle on a rule with the elevation text; side 1 extends right, -1 left."""
    x, y, _ = P(point)
    s = 0.09
    tri = _d.polygon([(x, y), (x - s, y + 1.6 * s), (x + s, y + 1.6 * s)], None, stroke=color, width=W_EDGE)
    rule = _d.segments([(P((x, y)), P((x + side * length, y)))], color, W_THIN)
    label = font_text(scene, text, (x + side * (length + 0.05), y + 0.02), size, color, "l" if side > 0 else "r")
    return _engine.VGroup(tri, rule, label)


def bubble(scene, text: str, centre, *, color: str = MUTED, radius: float = 0.13):
    """⭕ Grid bubble: circle with its axis label."""
    ring = _d.polygon(_d.circle_points(centre, radius, 20), None, stroke=color, width=W_THIN)
    return _engine.VGroup(ring, font_text(scene, text, centre, S_SMALL, INK))


def level_text(scene, value: float) -> str:
    """🔢 Elevation text in language punctuation: ±0.00, +3.50."""
    return "±" + scene.number(0.0, 2) if abs(value) < 1e-9 else ("+" if value > 0 else "") + scene.number(value, 2)


def short_name(name: dict, language: str) -> str:
    """✂️ The first phrase of a model name: cut at 'with', 'mit', a comma or a bracket."""
    text = name[language]
    for separator in (" with ", " mit ", " (", ","):
        text = text.split(separator)[0]
    return text


def dash_dot(a, b, long: float = 0.2, gap: float = 0.05, dot: float = 0.02) -> list:
    """➖ Segment pairs of a dash-dot line between two screen points."""
    a, b = P(a), P(b)
    length = float(np.linalg.norm(b - a))
    if length < 1e-6:
        return []
    direction = (b - a) / length
    pairs, position, pattern = [], 0.0, (long, gap, dot, gap)
    index = 0
    while position < length - 1e-9:
        span = min(pattern[index % 4], length - position)
        if index % 2 == 0:
            pairs.append((a + direction * position, a + direction * (position + span)))
        position += span
        index += 1
    return pairs


def poche(rings, color: str, *, fill: float = 0.3, hatch: bool = True, cross: bool = False, spacing: float = SPACING, angle: float = 45.0,
          width: float = W_CUT, stroke: str | None = None, opacity: float = 1.0, solid_below: float = MIN_HATCH):
    """⬛ A cut area on screen: translucent fill, hatch lines and a heavy outline; areas thinner than ``solid_below`` become solid."""
    rings = [[P(p)[:2] for p in ring] for ring in rings]
    pts = np.asarray([p for ring in rings for p in ring], float)
    extent = float(min(np.ptp(pts[:, 0]), np.ptp(pts[:, 1])))
    group = _engine.VGroup()
    line = stroke or color
    if extent < solid_below or not hatch:
        group.add(_d.region(rings, color, (0.85 if extent < solid_below else fill) * opacity, line, min(width, max(W_THIN, extent * 60)), opacity))
        return group
    group.add(_d.region(rings, color, fill * opacity, line, width, opacity))
    group.add(_d.hatch(rings, color, spacing, angle, W_HAIR, 0.9 * opacity))
    if cross:
        group.add(_d.hatch(rings, color, spacing, angle + 90.0, W_HAIR, 0.9 * opacity))
    return group


def zigzag(a, b, amplitude: float = 0.06, teeth: int = 3, color: str = INK, width: float = W_THIN):
    """〰️ A break line between two screen points."""
    a, b = P(a), P(b)
    direction = (b - a) / max(np.linalg.norm(b - a), 1e-9)
    normal = np.array([-direction[1], direction[0], 0.0])
    points = [a]
    for index in range(teeth * 2):
        t = (index + 0.5) / (teeth * 2)
        points.append(a + (b - a) * t + normal * amplitude * (1 if index % 2 == 0 else -1))
    points.append(b)
    return _d.polyline(points, color, width)


def ground_strip(scene, to, h0: float, h1: float, depth: float = 0.45, *, color: str = MUTED, breaks: bool = False):
    """🌱 Ground below the ±0.00 line: tinted strip, diagonal hatch and a firm ground line; ``to(h, z)`` maps a drawing coordinate to the screen."""
    ring = [to(h0, 0.0), to(h1, 0.0), to(h1, -depth), to(h0, -depth)]
    group = _engine.VGroup(_d.polygon(ring, GROUND_PLANE, 1.0), _d.hatch(ring, color, 0.1, 45.0, W_HAIR, 0.5),
                           _d.polyline([to(h0, 0.0), to(h1, 0.0)], INK, W_EDGE))
    if breaks:
        group.add(zigzag(ring[0], ring[3], color=color), zigzag(ring[1], ring[2], color=color))
    return group


def circle(centre, radius: float, count: int = 20) -> list:
    """⭕ Screen polygon of a circle."""
    return _d.circle_points(centre, radius, count)


def offset_path(path, half: float) -> tuple:
    """↔️ Left and right offset curves of a plan polyline at ``half`` metres."""
    pts = np.asarray(path, float)
    normals = []
    for index in range(len(pts)):
        segments = []
        if index > 0:
            segments.append(pts[index] - pts[index - 1])
        if index < len(pts) - 1:
            segments.append(pts[index + 1] - pts[index])
        d = sum(s / max(np.linalg.norm(s), 1e-12) for s in segments)
        d = d / max(np.linalg.norm(d), 1e-12)
        normals.append(np.array([-d[1], d[0]]))
    normals = np.asarray(normals)
    return pts + normals * half, pts - normals * half


def centroid(ring) -> np.ndarray:
    """🎯 Area centroid of a plan polygon."""
    pts = np.asarray(ring, float)
    nxt = np.roll(pts, -1, axis=0)
    cross = pts[:, 0] * nxt[:, 1] - nxt[:, 0] * pts[:, 1]
    area = cross.sum() / 2
    if abs(area) < 1e-9:
        return pts.mean(axis=0)
    return np.array([((pts[:, 0] + nxt[:, 0]) * cross).sum(), ((pts[:, 1] + nxt[:, 1]) * cross).sum()]) / (6 * area)


def quantise(value: float, step: float, full: float | None = None) -> float:
    """🪜 Round a live label value to a coarse step so animated numbers reuse cached labels; the full value stays exact."""
    if full is not None and abs(value - full) < 1e-9:
        return full
    return round(value / step) * step


def subtract(base: list, cut: list) -> list:
    """➖ Interval list minus interval list."""
    out = []
    for lo, hi in base:
        pieces = [(lo, hi)]
        for c0, c1 in cut:
            nxt = []
            for a, b in pieces:
                if c1 <= a or c0 >= b:
                    nxt.append((a, b))
                    continue
                if c0 > a:
                    nxt.append((a, c0))
                if c1 < b:
                    nxt.append((c1, b))
            pieces = nxt
        out += pieces
    return [(a, b) for a, b in out if b - a > 1e-6]


def intersect(base: list, other: list) -> list:
    """∩ Interval list intersected with interval list."""
    out = []
    for a0, a1 in base:
        for b0, b1 in other:
            lo, hi = max(a0, b0), min(a1, b1)
            if hi - lo > 1e-6:
                out.append((lo, hi))
    return out


class Sheet(_d.Drawing):
    """📄 A drawing sheet: ``at(x, y, z)`` projects a model point, ``at(h, z)`` a point of a section or elevation, ``to_screen`` is the plan alias."""

    def __init__(self, view, kind: str = "plan"):
        super().__init__(view)
        self.kind, self.plane, self.cut_z = kind, None, None

    def to_screen(self, x: float, y: float, z: float = 0.0) -> np.ndarray:
        """📍 Screen position of a plan coordinate in metres."""
        return self.view(x, y, z)

    def hz(self, h: float, z: float) -> np.ndarray:
        """📍 Screen position of horizontal drawing coordinate ``h`` (y of an x-cut, x of a y-cut, x or y of an elevation) and height ``z``."""
        if self.view.kind == "plan":
            return self.view(h, z, 0.0)
        if self.view.kind == "section":
            axis, value = self.plane
            return self.view(value, h, z) if axis == "x" else self.view(h, value, z)
        return self.view(h, 0.0, z) if self.view.face in ("south", "north") else self.view(0.0, h, z)

    def at(self, *args) -> np.ndarray:
        """📍 ``at("B3")`` is a part's centre, ``at(x, y, z)`` a model point, ``at(h, z)`` a section or elevation point."""
        if len(args) == 1 and isinstance(args[0], str):
            return super().at(*args)
        if len(args) == 2:
            return self.hz(*args)
        return self.view(*args)

    def register(self, tag: str, group, anchor=None, attach: bool = True):
        """🏷️ Add a group once to the drawing and make it addressable; a repeated tag merges its children into the first group."""
        if tag in self.parts:
            self.parts[tag].add(*group.submobjects)
            return self.parts[tag]
        self.parts[tag] = group
        if anchor is not None:
            self.anchors[tag] = P(anchor)
        if attach:
            self.add(group)
        return group

    def alias(self, name: str, tag: str):
        """🔗 Make a second key address the same part."""
        self.parts[name] = self.parts[tag]
        return self


class Look:
    """🎨 Highlight, ghost and dim state of every element, with one highlight colour per tag (default DATA)."""

    def __init__(self, highlight=(), ghost=(), dim: float = 0.45, colors=None, mode: str = "material"):
        self.sel = _c.Selection(highlight, ghost, dim)
        self.dim, self.colors, self.mode = dim, dict(colors or {}), mode

    def state(self, item) -> str:
        """🎯 normal, highlight, dim or ghost."""
        return self.sel.state(item)

    def highlight_color(self, item) -> str:
        """🖍️ The colour a highlighted item takes."""
        for key in (item["tag"], item["cls"], item["kind"], item["guid"]):
            if key in self.colors:
                return self.colors[key]
        return DATA

    def tone(self, item, base: str) -> tuple:
        """🌗 ``(colour, opacity, extra width)`` of an item whose natural colour is ``base``."""
        state = self.sel.state(item)
        if state == "highlight":
            return self.highlight_color(item), 1.0, 0.8
        if state == "ghost":
            return base, 0.22, 0.0
        if state == "dim":
            return _d.mix(base, BACKGROUND, self.dim), 1.0, 0.0
        return base, 1.0, 0.0

    def base(self, item, material: str) -> str:
        """🧱 Material colour, or the discipline colour (architecture before structure before services) in discipline mode."""
        if self.mode == "discipline" and item.get("disciplines"):
            return _d.DISCIPLINES[next(name for name in ("architecture", "structure", "services") if name in item["disciplines"])]
        return material


def ground_walls(g: dict) -> dict:
    """🧱 Walls of the set-back ground floor derived from the ground spaces: ``outer`` and ``inner`` straight edges ``(a, b, space)`` and the ``glass`` edges of the curtain wall; ``ring`` is the outer boundary."""
    hit = _WALLS.get(id(g))
    if hit is not None and hit[0] is g:
        return hit[1]
    curtain = g["curtain_wall"]["path"]
    glass_edges = {frozenset((tuple(np.round(a, 4)), tuple(np.round(b, 4)))) for a, b in zip(curtain[:-1], curtain[1:])}
    spaces = [s for s in g["spaces"] if s["storey"] == "ground"]
    vertices = [tuple(np.round(p, 4)) for s in spaces for p in s["outline"]]
    edges = []
    for space in spaces:
        ring = [tuple(np.round(p, 4)) for p in space["outline"]]
        for a, b in zip(ring, ring[1:] + ring[:1]):
            a_, b_ = np.array(a), np.array(b)
            length = float(np.linalg.norm(b_ - a_))
            cuts = [0.0, 1.0]
            for v in vertices:
                t = float((np.array(v) - a_) @ (b_ - a_) / length ** 2)
                offset = np.array(v) - a_
                if 1e-6 < t < 1 - 1e-6 and abs(float((b_ - a_)[0] * offset[1] - (b_ - a_)[1] * offset[0])) / length < 1e-6:
                    cuts.append(t)
            cuts = sorted(set(cuts))
            for t0, t1 in zip(cuts[:-1], cuts[1:]):
                edges.append((tuple(np.round(a_ + (b_ - a_) * t0, 4)), tuple(np.round(a_ + (b_ - a_) * t1, 4)), space["tag"]))
    outer, inner, glass, seen = [], [], [], {}
    for a, b, tag in edges:
        seen.setdefault(frozenset((a, b)), []).append((a, b, tag))
    for pair, items in seen.items():
        if pair in glass_edges:
            glass.append(items[0])
        elif len(items) == 1:
            outer.append(items[0])
        else:
            inner.append(items[0])
    result = {"outer": outer, "inner": inner, "glass": glass, "height": g["curtain_wall"]["top"], "thickness": WALL_T}
    _WALLS[id(g)] = (g, result)
    if len(_WALLS) > 8:
        _WALLS.pop(next(iter(_WALLS)))
    return result


class Plane:
    """✂️ A vertical cutting plane ``x = value`` or ``y = value``: ``h`` is the horizontal drawing coordinate and ``s`` the distance toward the viewer."""

    def __init__(self, axis: str, value: float):
        self.axis, self.value, self.index = axis, float(value), 0 if axis == "x" else 1

    def h(self, p) -> float:
        """↔️ Horizontal coordinate of a plan point: y for an x-cut (south left), x for a y-cut (west left)."""
        return float(p[1] if self.index == 0 else p[0])

    def s(self, p) -> float:
        """👁️ Signed distance toward the viewer: positive means in front of the cut (removed)."""
        return float(p[0] - self.value) if self.index == 0 else float(self.value - p[1])

    def intervals(self, rings) -> list:
        """📏 Horizontal intervals where the plane cuts a polygon with holes."""
        return _c.scan(rings, self.value, self.index)

    def classify(self, s0: float, s1: float) -> str:
        """🔎 cut, beyond or removed for an element whose depth extent along the view axis is [s0, s1]."""
        lo, hi = min(s0, s1), max(s0, s1)
        if lo < -EPS and hi > EPS:
            return "cut"
        return "beyond" if hi <= EPS else "removed"

    def extent(self, ring) -> tuple:
        """📐 Depth extent of a plan ring."""
        values = [self.s(p) for p in ring]
        return min(values), max(values)

    def hrange(self, ring) -> tuple:
        """📐 Horizontal extent of a plan ring."""
        values = [self.h(p) for p in ring]
        return min(values), max(values)

    @property
    def spec(self) -> tuple:
        """🏷️ ``("x"|"y", value)``."""
        return self.axis, self.value


def parse_cut(cut, g: dict) -> Plane:
    """🧭 Resolve a named cut (salon, ramp, terrace), ``"row:3"`` (piloti column), ``"row:C"`` (piloti row), ``("x"|"y", value)`` or a number (x) into a plane."""
    if isinstance(cut, Plane):
        return cut
    if isinstance(cut, (int, float)):
        return Plane("x", cut)
    if isinstance(cut, (tuple, list)):
        return Plane(cut[0], cut[1])
    xs, ys = g["grid"]["x"], g["grid"]["y"]
    if cut == "salon":
        return Plane("x", xs[1])
    if cut == "ramp":
        return Plane("x", g["ramp"]["flights"][0]["start"][0])
    if cut == "terrace":
        return Plane("x", xs[3])
    if isinstance(cut, str) and cut.startswith("row:"):
        key = cut[4:]
        if key in g["grid"]["labels_x"]:
            return Plane("x", xs[g["grid"]["labels_x"].index(key)])
        return Plane("y", ys[g["grid"]["labels_y"].index(key)])
    raise ValueError(f"Unknown cut: {cut}")


def car_dims(model=None) -> dict:
    """🚗 Length and width of the assumed 1929 car in metres, read from the model."""
    key = id(model)
    hit = _CARS.get(key)
    if hit is None or hit[0] is not model:
        data = (model if model is not None else _model.load_model())["car"]
        hit = _CARS[key] = (model, {"length": data["length"]["value"], "width": data["width"]["value"], "turning_radius": data["turning_radius"]["value"]})
        if len(_CARS) > 8:
            _CARS.pop(next(iter(_CARS)))
    return hit[1]


def storey_bands(g: dict) -> dict:
    """🏢 ``{storey: (elevation, next elevation or box height)}``."""
    names = sorted(g["storeys"], key=g["storeys"].get)
    tops = [g["storeys"][n] for n in names[1:]] + [g["box"]["height"]]
    return {name: (g["storeys"][name], top) for name, top in zip(names, tops)}


def cloud(root, samples: int = 7) -> np.ndarray:
    """☁️ Screen points sampled along every visible outline of a drawing; label placement keeps clear of them."""
    chunks = []
    for m in root.family_members_with_points():
        pts = m.points
        if len(pts) < 4 or (m.get_stroke_opacity() < 0.05 and m.get_fill_opacity() < 0.05):
            continue
        n = len(pts) // 4
        p = pts[: n * 4].reshape(n, 4, 3)
        t = np.linspace(0, 1, samples)[None, :, None]
        b = (1 - t) ** 3 * p[:, 0:1] + 3 * (1 - t) ** 2 * t * p[:, 1:2] + 3 * (1 - t) * t**2 * p[:, 2:3] + t**3 * p[:, 3:4]
        chunks.append(b.reshape(-1, 3)[:, :2])
    return np.vstack(chunks) if chunks else np.zeros((0, 2))


def inside_polygon(points: np.ndarray, ring) -> np.ndarray:
    """🎯 Even-odd point-in-polygon test for an ``(n, 2)`` array."""
    ring = np.asarray([p[:2] for p in ring], float)
    x, y = points[:, 0], points[:, 1]
    result = np.zeros(len(points), bool)
    for a, b in zip(ring, np.roll(ring, -1, axis=0)):
        crossing = (a[1] > y) != (b[1] > y)
        with np.errstate(divide="ignore", invalid="ignore"):
            at = a[0] + (y - a[1]) * (b[0] - a[0]) / (b[1] - a[1])
        result ^= crossing & (x < at)
    return result


def find_spot(points: np.ndarray, width: float, height: float, ring, prefer, pad: float = 0.04, step: float = 0.05):
    """🔍 Centre of a ``width × height`` label rectangle inside a screen polygon that touches none of the sampled outlines, nearest to ``prefer``; ``None`` when nothing fits."""
    ring = np.asarray([p[:2] for p in ring], float)
    lo, hi = ring.min(axis=0) + (width / 2, height / 2), ring.max(axis=0) - (width / 2, height / 2)
    if hi[0] < lo[0] or hi[1] < lo[1]:
        return None
    xs, ys = np.arange(lo[0], hi[0] + 1e-9, step), np.arange(lo[1], hi[1] + 1e-9, step)
    cand = np.array([(x, y) for x in xs for y in ys])
    prefer = np.asarray(prefer[:2], float)
    cand = cand[np.argsort(np.linalg.norm(cand - prefer, axis=1))]
    near = points[(points[:, 0] > lo[0] - width) & (points[:, 0] < hi[0] + width) & (points[:, 1] > lo[1] - height) & (points[:, 1] < hi[1] + height)]
    hw, hh = width / 2 + pad, height / 2 + pad
    for c in cand:
        if len(near) and ((np.abs(near[:, 0] - c[0]) < hw) & (np.abs(near[:, 1] - c[1]) < hh)).any():
            continue
        corners = np.array([(c[0] + sx * width / 2, c[1] + sy * height / 2) for sx in (-1, 1) for sy in (-1, 1)])
        if inside_polygon(corners, ring).all():
            return c
    return None


def items(model=None) -> list:
    """📚 The drawable catalogue with windows keyed by their IfcWindow tag (``RW-S``) and GlobalId; ``opening`` keeps the IfcOpeningElement tag."""
    out = []
    for item in _c.catalogue(model):
        if item["kind"] == "windows":
            data = item["data"]
            item = {**item, "tag": data["window_tag"], "guid": data["window_guid"], "opening": data["tag"]}
        out.append(item)
    return out


def expand_names(names, g: dict) -> set:
    """🔗 A window tag stands for its opening and the other way round."""
    names = {names} if isinstance(names, str) else set(names or ())
    pairs = {o["tag"]: o["window_tag"] for w in g["walls"] for o in w["openings"]}
    pairs.update({v: k for k, v in list(pairs.items())})
    return names | {pairs[n] for n in names if n in pairs}


def pseudo(tag: str, cls: str, kind: str, guid: str = "", disciplines=("architecture",)) -> dict:
    """🧩 A selection record for a drawn thing that is not a model element (a space, a derived ground wall)."""
    return {"tag": tag, "cls": cls, "kind": kind, "guid": guid or tag, "disciplines": disciplines}


class SheetMixin:
    """🧰 Scene helpers shared by every orthographic widget module: cached labels, dimensions and level marks."""

    def _t(self, string: str, point=(0, 0, 0), size: float = S_SMALL, color: str = INK, anchor: str = "c"):
        """🏷️ Cached label."""
        return font_text(self, string, point, size, color, anchor)

    def _dim(self, start, end, text=None, **kwargs):
        """↔️ Cached-label dimension."""
        return dim(self, start, end, text, **kwargs)

    def _level(self, point, value: float, **kwargs):
        """🔻 Level mark with the localized elevation text."""
        return level_mark(self, point, level_text(self, value), **kwargs)

    def _look(self, highlight, ghost, dim_amount, colors, mode) -> Look:
        """🎨 Selection state for a drawing."""
        return Look(highlight, ghost, dim_amount, colors, mode)
