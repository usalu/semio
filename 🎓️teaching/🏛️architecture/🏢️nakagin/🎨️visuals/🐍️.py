"""🏢 Manim drawings of the Nakagin teaching model: capsule, orthographic views, plane axes, core, capital, podium, tower axonometric and elevation.

Every widget is a pure function of its parameters and returns a drawing with named parts, so ``always_redraw`` and ``Drafting.live`` can rebuild it every frame.
Related: [Kisho Kurokawa, Nakagin Capsule Tower](https://en.wikipedia.org/wiki/Nakagin_Capsule_Tower)."""

from __future__ import annotations

import importlib
import math
import sys
from functools import lru_cache
from pathlib import Path
from typing import NamedTuple

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, DATA, GOOD, WARN, ARCH, BACKGROUND, CONCRETE = _d.INK, _d.MUTED, _d.DATA, _d.GOOD, _d.WARN, _d.ARCH, _d.BACKGROUND, _d.CONCRETE
W_CUT, W_EDGE, W_THIN, W_HAIR, S_SMALL = _d.W_CUT, _d.W_EDGE, _d.W_THIN, _d.W_HAIR, _d.S_SMALL
CAPSULE = _theme.color("light-8-9")
CAPITAL = _theme.color("light-gray")
GLASS_DARK = _d.mix(BACKGROUND, ARCH, 0.14)
AXIS_X, AXIS_Y, AXIS_Z = _art.P_RED, _theme.color("diff-added"), _d.mix(_art.P_CYAN, "#2f55ff", 0.62)
TYPE_COLORS = (_art.P_CYAN, _art.P_ORANGE, _art.P_GREEN, _art.P_RED)
VIEWS = {"axon": (55.0, 24.0), "axon_back": (-55.0, 24.0), "axon_left": (125.0, 24.0), "axon_high": (55.0, 42.0)}
ORTHO = {"plan": "plan", "north": "north", "south": "south", "east": "east", "west": "west"}
LIGHT = np.array([-0.3, 0.45, 0.85]) / math.sqrt(0.09 + 0.2025 + 0.7225)

_TEXTS: dict = {}


def P(point) -> np.ndarray:
    """🧭 Drawing point as a float 3-vector, padded with zeros, so projections and offsets work on it directly."""
    return np.array([*(float(value) for value in tuple(point)[:3]), 0.0, 0.0, 0.0][:3])


def type_color(key: str) -> str:
    """🎨 The fixed colour of a capsule type: A teal, B orange, C green, D red."""
    keys = [t.key for t in _model.capsule_types()]
    return TYPE_COLORS[keys.index(key) % len(TYPE_COLORS)]


@lru_cache(maxsize=8192)
def _blend(first: str, second: str, amount: float) -> str:
    return _d.mix(first, second, amount)


def shade_of(base: str, tone: float) -> str:
    """🌗 A face colour: the base lit by a tone in 0…1 against the background."""
    return _blend(BACKGROUND, base, round(float(tone), 3))


def tone_of(normal) -> float:
    """☀️ Face tone from a fixed light at the upper left front: roof 0.95, front 0.76, left 0.7, right 0.58."""
    return 0.58 + 0.4 * float(min(1.0, max(0.0, np.asarray(normal, float) @ LIGHT)))


class Mesh(NamedTuple):
    """🧊 Polyhedral mesh: vertices ``V``, outward unit normals ``N``, faces as index arrays and a name per face."""

    V: np.ndarray
    N: np.ndarray
    faces: tuple
    names: tuple


class Item(NamedTuple):
    """🧩 One depth-sorted drawing unit: tier, centre, vertices for the bounding box and the drawing function."""

    tier: int
    centre: np.ndarray
    verts: np.ndarray
    draw: object


class NakaginDrawing3D(_engine.VGroup):
    """🏢 A Nakagin drawing: ``view``, named ``parts``, ``anchors`` and ``at(x, y, z)`` back to screen space."""

    def __init__(self, view=None):
        super().__init__()
        self.view, self.parts, self.anchors = view, {}, {}

    def at(self, *args) -> np.ndarray:
        """📍 Screen position of a model coordinate (or of a part tag)."""
        if len(args) == 1 and isinstance(args[0], str):
            return self.anchors[args[0]] if args[0] in self.anchors else np.array(self.parts[args[0]].get_center())
        return self.view(*args)


def make_view(view, at, scale: float, centre) -> _d.View:
    """🔭 The projection of a drawing: an axon name, ``(azimuth, elevation)``, ``plan``, ``north|south|east|west`` or a ready view; ``centre`` is the model point that sits at ``at``."""
    if isinstance(view, _d.View):
        return _d.View(view.kind, at, scale, centre, azimuth=view.azimuth, elevation=view.elevation, face=view.face)
    if isinstance(view, str) and view in ORTHO:
        return _d.View("plan" if view == "plan" else "elevation", at, scale, centre, face="south" if view == "plan" else view)
    azimuth, elevation = VIEWS[view] if isinstance(view, str) else view
    return _d.View("axon", at, scale, centre, azimuth=azimuth, elevation=elevation)


def project(point, view: _d.View) -> np.ndarray:
    """📍 Screen position of a model point in a view."""
    return view(*P(point))


def _basis(view: _d.View):
    if view.kind == "axon":
        return np.array([view.right, view.up]), view.towards
    if view.kind == "plan":
        return np.array([[1.0, 0, 0], [0, 1.0, 0]]), np.array([0.0, 0, 1.0])
    table = {"south": ((1, 0, 0), (0, 0, 1), (0, -1, 0)), "north": ((-1, 0, 0), (0, 0, 1), (0, 1, 0)),
             "east": ((0, 1, 0), (0, 0, 1), (1, 0, 0)), "west": ((0, -1, 0), (0, 0, 1), (-1, 0, 0))}
    a, b, t = table[view.face]
    return np.array([a, b], float), np.array(t, float)


def _screen(view: _d.View, points) -> np.ndarray:
    B, _ = _basis(view)
    local = (np.asarray(points, float).reshape(-1, 3) - view.centre) @ B.T
    out = np.zeros((len(local), 3))
    out[:, :2] = view.point[:2] + view.scale * local
    return out


def _order(view: _d.View, centre) -> float:
    _, T = _basis(view)
    offset = np.asarray(centre, float) - view.centre
    return float(offset[:2] @ T[:2]) if view.kind == "axon" else float(offset @ T)


def _poly_points(polys) -> np.ndarray:
    chunks = []
    for poly in polys:
        ring = np.vstack([poly, poly[:1]])
        a, b = ring[:-1], ring[1:]
        chunks.append(np.stack([a, a + (b - a) / 3, a + 2 * (b - a) / 3, b], 1).reshape(-1, 3))
    return np.vstack(chunks)


def _shape(polys, fill, opacity: float = 1.0, stroke=None, width: float = 0.0, stroke_opacity: float = 1.0):
    shape = _engine.VMobject()
    shape.set_points(_poly_points(polys))
    shape.set_fill(fill, opacity)
    shape.set_stroke(stroke or fill, width, stroke_opacity)
    return shape


def _circle(centre, u, v, radius: float, count: int = 28, start: float = 0.0, stop: float = 360.0) -> np.ndarray:
    angles = np.radians(np.linspace(start, stop, count, endpoint=stop - start < 360))
    return np.asarray(centre, float) + radius * (np.cos(angles)[:, None] * np.asarray(u, float) + np.sin(angles)[:, None] * np.asarray(v, float))


def _profile(w: float, h: float, r: float, segments: int) -> np.ndarray:
    r = max(min(r, w / 2 - 1e-3, h / 2 - 1e-3), 1e-3)
    points = []
    for (cx, cz), start in (((w / 2 - r, r), -90), ((w / 2 - r, h - r), 0), ((-w / 2 + r, h - r), 90), ((-w / 2 + r, r), 180)):
        for index in range(segments + 1):
            angle = math.radians(start + 90 * index / segments)
            points.append((cx + r * math.cos(angle), cz + r * math.sin(angle)))
    return np.array(points)


@lru_cache(maxsize=512)
def capsule_mesh(w: float, l: float, h: float, r: float, segments: int) -> Mesh:
    """💊 The rounded-profile capsule mesh in its local frame: x centred, back at y=0, front at y=l, base at z=0; face names front, back, top, bottom, left, right."""
    profile = _profile(w, h, r, segments)
    n = len(profile)
    V = np.vstack([np.column_stack([profile[:, 0], np.zeros(n), profile[:, 1]]), np.column_stack([profile[:, 0], np.full(n, l), profile[:, 1]])])
    edge = np.roll(profile, -1, axis=0) - profile
    normal = np.column_stack([edge[:, 1], np.zeros(n), -edge[:, 0]])
    normal /= np.linalg.norm(normal, axis=1)[:, None]
    faces = [np.arange(n)[::-1], np.arange(n, 2 * n)] + [np.array([i, (i + 1) % n, n + (i + 1) % n, n + i]) for i in range(n)]
    names = ["back", "front"] + [("top" if nz > 0 else "bottom") if abs(nz) > abs(nx) else ("right" if nx > 0 else "left") for nx, _, nz in normal]
    return Mesh(V, np.vstack([[0, -1.0, 0], [0, 1.0, 0], normal]), tuple(faces), tuple(names))


def box_mesh(x0: float, x1: float, y0: float, y1: float, z0: float, z1: float) -> Mesh:
    """📦 An axis-aligned box mesh; face names bottom, top, back (−y), right (+x), front (+y), left (−x)."""
    V = np.array([(x0, y0, z0), (x1, y0, z0), (x1, y1, z0), (x0, y1, z0), (x0, y0, z1), (x1, y0, z1), (x1, y1, z1), (x0, y1, z1)], float)
    faces = ((0, 3, 2, 1), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7))
    N = np.array([(0, 0, -1), (0, 0, 1), (0, -1, 0), (1, 0, 0), (0, 1, 0), (-1, 0, 0)], float)
    return Mesh(V, N, tuple(np.array(f) for f in faces), ("bottom", "top", "back", "right", "front", "left"))


def solid_mesh(solid) -> Mesh:
    """🧊 A mesh from a model ``Solid`` (outward-wound vertex loops)."""
    V = np.array(solid.vertices, float)
    faces, normals = [], []
    for loop in solid.faces:
        ring = V[list(loop)]
        normal = np.cross(ring[1] - ring[0], ring[2] - ring[0]) if len(ring) > 2 else np.array([0, 0, 1.0])
        total = np.zeros(3)
        for a, b in zip(ring, np.roll(ring, -1, axis=0)):
            total += np.cross(a, b)
        normal = total if np.linalg.norm(total) > 1e-9 else normal
        faces.append(np.array(loop))
        normals.append(normal / max(np.linalg.norm(normal), 1e-12))
    return Mesh(V, np.array(normals), tuple(faces), tuple(f"face{i}" for i in range(len(faces))))


def _world(mesh: Mesh, plane) -> Mesh:
    R = np.array([plane.x, plane.y, plane.z], float)
    return Mesh(np.asarray(plane.origin, float) + mesh.V @ R, mesh.N @ R, mesh.faces, mesh.names)


def _cap(t) -> dict:
    base = _model.capsule_type(t["key"]) if isinstance(t, dict) and "key" in t else (_model.capsule_types()[0] if isinstance(t, dict) else t)
    get = lambda name: float(t[name] if isinstance(t, dict) and name in t else getattr(base, name))
    return {"w": get("width"), "l": get("length"), "h": get("height"), "r": get("corner_radius"), "seg": int(get("corner_segments")),
            "d": get("window_diameter"), "u": get("window_u"), "v": get("window_v"), "key": getattr(base, "key", None)}


def _window_rings(cap: dict, frame: float, count: int = 28):
    centre = np.array([cap["u"], cap["l"], cap["v"]])
    ex, ez = np.array([1.0, 0, 0]), np.array([0, 0, 1.0])
    return centre, ex, ez, _circle(centre, ex, ez, cap["d"] / 2, count), _circle(centre, ex, ez, cap["d"] / 2 + frame, count)


def _visible(mesh: Mesh, view: _d.View, minimum: float = 1e-6):
    _, T = _basis(view)
    return [i for i, n in enumerate(mesh.N @ T) if n > minimum]


def _tone_buckets(mesh: Mesh, view: _d.View, base: str, levels: int, edge: float, width: float, opacity: float = 1.0, stroke=None):
    S = _screen(view, mesh.V)
    buckets = {}
    for i in _visible(mesh, view):
        key = round(tone_of(mesh.N[i]) * levels) / levels
        buckets.setdefault(key, []).append(S[mesh.faces[i]])
    return [_shape(polys, shade_of(base, tone), opacity, stroke or shade_of(base, tone * edge), width) for tone, polys in sorted(buckets.items())]


def _capsule_lite(cap: dict, plane, view: _d.View, base: str, *, window: bool = True, highlight=None, levels: int = 8):
    mesh = _world(capsule_mesh(cap["w"], cap["l"], cap["h"], cap["r"], cap["seg"]), plane)
    body = _tone_buckets(mesh, view, base, levels, 0.7, 0.45 if highlight is None else 2.0, 1.0, highlight)
    group = _engine.VGroup(*body)
    if window and "front" in [mesh.names[i] for i in _visible(mesh, view)]:
        centre, ex, ez, glass, frame = _window_rings(cap, 0.07, 18)
        R = np.array([plane.x, plane.y, plane.z], float)
        origin = np.asarray(plane.origin, float)
        S = _screen(view, origin + np.vstack([frame, glass]) @ R)
        group.add(_shape([S[:len(frame)]], shade_of(base, 0.95)), _shape([S[len(frame):]], shade_of(GLASS_DARK, 1.0)))
    return group


def _detail(mesh: Mesh, view: _d.View, base: str, colors: dict):
    S = _screen(view, mesh.V)
    faces, rings = {}, {}
    for i in _visible(mesh, view):
        name = mesh.names[i]
        fill = colors.get(name, base)
        color = shade_of(fill, tone_of(mesh.N[i]))
        faces.setdefault(name, _engine.VGroup()).add(_shape([S[mesh.faces[i]]], color, 1.0, color, 0.7))
        rings.setdefault(name, []).append(S[mesh.faces[i]][:, :2])
    return faces, rings


class NakaginDrawing:
    """🏢 Scene mixin: Nakagin capsule, views, axes, core, capital, podium and tower drawings. Combine with ``Drafting`` for text and dimensions."""

    def _text(self, string: str, point=(0, 0, 0), *, size: float = S_SMALL, color=DATA, anchor: str = "c"):
        key = (string, size, color)
        if key not in _TEXTS:
            _TEXTS[key] = self.text(string, (0, 0, 0), size=size, color=color)
        return _d.place(_TEXTS[key].copy(), point, anchor)

    def _away(self, start, end, text, centre, offset: float, color=DATA):
        start, end = P(start), P(end)
        delta = end - start
        normal = np.array([-delta[1], delta[0], 0.0])
        normal /= max(np.linalg.norm(normal), 1e-9)
        sign = 1.0 if float(normal @ ((start + end) / 2 - P(centre))) >= 0 else -1.0
        shape = self.dimension(start, end, None, offset=sign * abs(offset), color=color)
        label = self._text(text, (0, 0, 0), color=color)
        shift = abs(normal[0]) * label.width / 2 + abs(normal[1]) * label.height / 2 + 0.07
        shape.add(label.move_to((start + end) / 2 + normal * sign * (abs(offset) + shift)))
        shape.label = label
        return shape

    def _chip(self, string: str, point, color=DATA):
        label = self._text(string, point, color=color)
        chip = _engine.RoundedRectangle(corner_radius=0.05, width=label.width + 0.14, height=label.height + 0.1, stroke_width=0, fill_color=BACKGROUND, fill_opacity=0.82).move_to(label)
        return _engine.VGroup(chip, label)

    def capsule_axon(self, t, *, at=(0, 0, 0), scale: float = 0.62, view="axon", window: bool = True, bolts: bool = False, dims=(),
                     highlight=(), hatch=(), base: str = CAPSULE, segments: int | None = None, dim_offset: float = 0.42):
        """💊 One shaded capsule in axonometric with its round window, optional bolt points, dimension lines and hatched faces.

        ``t`` is a ``CapsuleType`` or a dict of parameters (e.g. live width/length/height/window_diameter); ``at`` is the screen position of the capsule centre.
        ``dims`` picks from ``width length height window``; ``hatch`` picks ``exposed``, ``attached`` or face names (front back top bottom left right);
        ``highlight`` is a tuple of face names or a dict of face name to colour. Parts: ``body``, ``faces[name]``, ``window``, ``bolts``, ``dims[name]``, ``hatches``."""
        cap = _cap(t)
        v = make_view(view, at, scale, (0.0, cap["l"] / 2, cap["h"] / 2))
        mesh = capsule_mesh(cap["w"], cap["l"], cap["h"], cap["r"], cap["seg"] if segments is None else segments)
        colors = highlight if isinstance(highlight, dict) else {name: DATA for name in highlight}
        colors = {name: _blend(base, color, 0.55) for name, color in colors.items()}
        faces, rings = _detail(mesh, v, base, colors)
        drawing = NakaginDrawing3D(v)
        drawing.faces, drawing.dims, drawing.hatches = faces, {}, {}
        drawing.body = _engine.VGroup(*faces.values())
        drawing.add(drawing.body)
        S = _screen(v, mesh.V)
        n = len(mesh.V) // 2
        for name in ("front", "back"):
            if name in faces:
                edge = S[n:] if name == "front" else S[:n]
                drawing.body.add(_d.polyline(edge[:, :2], shade_of(base, 0.32), W_THIN, 0.9, closed=True))
        drawing.window = _engine.VGroup()
        hole = None
        if window and "front" in faces:
            centre, ex, ez, glass, frame = _window_rings(cap, 0.07)
            W = _screen(v, np.vstack([frame, glass]))
            frame_pts, glass_pts = W[:len(frame)], W[len(frame):]
            hole = frame_pts[:, :2]
            front_color = colors.get("front", base)
            drawing.window.add(_shape([frame_pts], shade_of(front_color, 0.96), 1.0, shade_of(front_color, 0.4), W_THIN),
                               _shape([glass_pts], GLASS_DARK, 1.0, shade_of(front_color, 0.32), W_THIN))
            arc = np.vstack([_circle(centre, ex, ez, cap["d"] * 0.43, 12, 105, 195), _circle(centre, ex, ez, cap["d"] * 0.33, 12, 195, 105)])
            drawing.window.add(_shape([_screen(v, arc)], INK, 0.2, INK, 0))
            drawing.window.add(_shape([_screen(v, _circle(centre, ex, ez, cap["d"] * 0.5, 28))], BACKGROUND, 0.0, shade_of(front_color, 0.55), W_HAIR))
            drawing.add(drawing.window)
        names = {"exposed": ("front", "top", "bottom", "left", "right"), "attached": ("back",)}
        for tag in hatch:
            group = _engine.VGroup()
            members = names.get(tag, (tag,))
            polys = [ring for name in members for ring in rings.get(name, [])]
            if hole is not None and "front" in members:
                polys = polys + [hole]
            if polys:
                color = {"exposed": WARN, "attached": GOOD}.get(tag, DATA)
                group.add(_d.hatch(polys, color, spacing=0.1, angle=40, width=W_THIN, opacity=0.95))
            drawing.hatches[tag] = group
            drawing.add(group)
        drawing.bolts = _engine.VGroup()
        if bolts:
            u, lo, hi, radius = _model.value("capsule.bolt_u").value, _model.value("capsule.bolt_v_low").value, _model.value("capsule.bolt_v_high").value, 0.075
            ex, ez = np.array([1.0, 0, 0]), np.array([0, 0, 1.0])
            front_back = "back" in faces
            for x, z in ((-u, lo), (u, lo), (u, hi), (-u, hi)):
                ring = _screen(v, _circle((x, 0.0, z), ex, ez, radius, 14))
                if front_back:
                    drawing.bolts.add(_shape([ring], _d.mix(BACKGROUND, INK, 0.2), 1.0, DATA, W_EDGE), _engine.Dot(P(v(x, 0.0, z)), radius=0.02, color=DATA))
                else:
                    drawing.bolts.add(_shape([ring], DATA, 0.0, DATA, W_THIN, 0.55))
            drawing.add(drawing.bolts)
        centre2 = v(0.0, cap["l"] / 2, cap["h"] / 2)
        w, l, h = cap["w"], cap["l"], cap["h"]
        specs = {"width": ((-w / 2, l, 0), (w / 2, l, 0), w), "length": ((w / 2, 0, 0), (w / 2, l, 0), l), "height": ((-w / 2, l, 0), (-w / 2, l, h), h)}
        for name in dims:
            if name in specs:
                a, b, value = specs[name]
                drawing.dims[name] = self._away(v(*a), v(*b), self.quantity(value, "m", 2), centre2, dim_offset)
            elif name == "window" and "front" in faces and window:
                u0, v0, radius = cap["u"], cap["v"], cap["d"] / 2
                a, b = v(u0 - radius, l, v0), v(u0 + radius, l, v0)
                group = _engine.VGroup(_d.segments([(a, b)], DATA, W_THIN))
                direction = (b - a) / max(np.linalg.norm(b - a), 1e-9)
                normal = np.array([-direction[1], direction[0], 0.0])
                group.add(_d.segments([(p + normal * 0.07, p - normal * 0.07) for p in (a, b)], DATA, W_EDGE))
                label = self._chip("Ø " + self.quantity(cap["d"], "m", 2), v(u0, l, (v0 + radius + h) / 2))
                group.add(label)
                group.label = label
                drawing.dims[name] = group
            if name in drawing.dims:
                drawing.add(drawing.dims[name])
        drawing.parts.update({"body": drawing.body, "window": drawing.window, "bolts": drawing.bolts})
        return drawing

    def capsule_views(self, t, *, at=(0, 0, 0), scale: float = 0.58, dims: bool = True, base: str = CAPSULE, gap: float = 1.0):
        """📐 Plan (front down, hatched walls), front elevation and side elevation of a capsule with dimension lines.

        The plan sits above the front elevation, the side elevation right of it; ``at`` is the screen position of the middle of the whole sheet.
        Parts: ``plan``, ``front``, ``side``, each with ``.dims`` and the live ``.window`` where drawn."""
        cap = _cap(t)
        w, l, h, r, d, u, vv = (cap[k] for k in ("w", "l", "h", "r", "d", "u", "v"))
        wall = max((w - _model.capsule_types()[0].interior_width) / 2, 0.06)
        sheet_w, sheet_h = (w + l) * scale + gap, (l + h) * scale + gap
        fx, fy = at[0] - sheet_w / 2 + w * scale / 2, at[1] - sheet_h / 2
        px, py = fx, fy + h * scale + gap
        sx, sy = fx + w * scale / 2 + gap, fy
        front_pt = lambda x, z: P((fx - x * scale, fy + z * scale))
        side_pt = lambda y, z: P((sx + y * scale, sy + z * scale))
        plan_pt = lambda x, y: P((px - x * scale, py + (l - y) * scale))
        drawing = NakaginDrawing3D()
        front, side, plan = _engine.VGroup(), _engine.VGroup(), _engine.VGroup()
        profile = _profile(w, h, r, cap["seg"])
        front.add(_shape([np.array([front_pt(x, z) for x, z in profile])], shade_of(base, 0.76), 1.0, INK, W_EDGE))
        centre = front_pt(u, vv)
        front.window = _engine.VGroup(_engine.Circle(radius=(d / 2 + 0.07) * scale, stroke_color=INK, stroke_width=W_THIN, fill_color=shade_of(base, 0.96), fill_opacity=1).move_to(centre),
                                      _engine.Circle(radius=d / 2 * scale, stroke_color=INK, stroke_width=W_THIN, fill_color=GLASS_DARK, fill_opacity=1).move_to(centre))
        front.add(front.window)
        side.add(_shape([np.array([side_pt(0, 0), side_pt(l, 0), side_pt(l, h), side_pt(0, h)])], shade_of(base, 0.7), 1.0, INK, W_EDGE))
        side.add(_d.segments([(side_pt(0, r), side_pt(l, r)), (side_pt(0, h - r), side_pt(l, h - r))], shade_of(base, 0.4), W_HAIR, 0.9))
        side.window = _d.polyline([side_pt(l, vv - d / 2), side_pt(l - 0.14, vv - d / 2), side_pt(l - 0.14, vv + d / 2), side_pt(l, vv + d / 2)], DATA, W_THIN, 0.95)
        side.add(side.window)
        box = np.array([plan_pt(-w / 2, 0), plan_pt(w / 2, 0), plan_pt(w / 2, l), plan_pt(-w / 2, l)])
        inner = np.array([plan_pt(-w / 2 + wall, wall), plan_pt(w / 2 - wall, wall), plan_pt(w / 2 - wall, l - wall), plan_pt(-w / 2 + wall, l - wall)])
        plan.add(_shape([box], shade_of(base, 0.9), 1.0, INK, W_CUT), _shape([inner], BACKGROUND, 0.7, INK, W_THIN),
                 _d.hatch([box[:, :2], inner[:, :2]], MUTED, spacing=0.05, angle=45, width=W_HAIR, opacity=0.9))
        plan.window = _shape([np.array([plan_pt(u - d / 2, l - wall), plan_pt(u + d / 2, l - wall), plan_pt(u + d / 2, l), plan_pt(u - d / 2, l)])], BACKGROUND, 1.0, DATA, W_THIN)
        plan.add(plan.window)
        for name, group in (("plan", plan), ("front", front), ("side", side)):
            group.dims = {}
            drawing.parts[name] = group
            drawing.add(group)
        if dims:
            q = lambda value: self.quantity(value, "m", 2)
            dm = lambda a, b, text, c: self._away(a, b, text, c, 0.3)
            pc, fc, sc = plan_pt(0, l / 2), front_pt(0, h / 2), side_pt(l / 2, h / 2)
            plan.dims["width"] = dm(plan_pt(w / 2, 0), plan_pt(-w / 2, 0), q(w), pc)
            plan.dims["length"] = dm(plan_pt(w / 2, 0), plan_pt(w / 2, l), q(l), pc)
            front.dims["width"] = dm(front_pt(w / 2, 0), front_pt(-w / 2, 0), q(w), fc)
            front.dims["height"] = dm(front_pt(w / 2, 0), front_pt(w / 2, h), q(h), fc)
            front.dims["window"] = _engine.VGroup(_d.segments([(front_pt(u + d / 2, vv), front_pt(u - d / 2, vv))], DATA, W_THIN), self._chip("Ø " + q(d), front_pt(u, (vv - d / 2) / 2)))
            if abs(u) > 1e-6:
                front.dims["offset"] = _engine.VGroup(_d.segments([(front_pt(0, vv - d / 2 - 0.15), front_pt(0, vv + d / 2 + 0.15))], MUTED, W_HAIR),
                                                      _d.segments([(front_pt(0, vv + d / 2 + 0.1), front_pt(u, vv + d / 2 + 0.1))], DATA, W_THIN),
                                                      self._text(q(abs(u)), front_pt(u / 2, vv + d / 2 + 0.1) + np.array([0, 0.16, 0])))
            side.dims["length"] = dm(side_pt(0, 0), side_pt(l, 0), q(l), sc)
            side.dims["height"] = dm(side_pt(l, 0), side_pt(l, h), q(h), sc)
            for group in (plan, front, side):
                group.add(*group.dims.values())
        return drawing

    def plane_axes(self, plane, *, at=(0, 0, 0), scale: float = 0.62, view="axon", length: float = 2.0, labels: bool = True, square: bool = True):
        """🧭 A Rhino plane: red X, green Y, blue Z arrows from the origin dot with an optional faint XY square; ``at`` is the screen position of the plane origin.

        Parts: ``axes["x"|"y"|"z"]`` (line, head, label), ``origin``, ``square``."""
        v = make_view(view, at, scale, plane.origin)
        origin = np.asarray(plane.origin, float)
        drawing = NakaginDrawing3D(v)
        drawing.axes = {}
        if square:
            corners = [origin + length * (a * np.asarray(plane.x) + b * np.asarray(plane.y)) for a, b in ((0, 0), (1, 0), (1, 1), (0, 1))]
            drawing.square = _shape([_screen(v, np.array(corners))], AXIS_Z, 0.12, AXIS_Z, W_HAIR, 0.5)
            drawing.add(drawing.square)
        for key, axis, color in (("x", plane.x, AXIS_X), ("y", plane.y, AXIS_Y), ("z", plane.z, AXIS_Z)):
            tip = v(*(origin + length * np.asarray(axis)))
            start = v(*origin)
            delta = tip - start
            norm = max(float(np.linalg.norm(delta)), 1e-9)
            direction = delta / norm
            normal = np.array([-direction[1], direction[0], 0.0])
            head = min(0.2, norm * 0.4)
            arrow = _engine.VGroup(_d.segments([(start, tip - direction * head * 0.6)], color, 2.6),
                                   _d.polygon([tip, tip - direction * head + normal * head * 0.38, tip - direction * head - normal * head * 0.38], color, 1.0, color, 0.5))
            if labels:
                arrow.add(self._text(key.upper(), tip + direction * 0.2, size=_d.S_LABEL, color=color))
            drawing.axes[key] = arrow
            drawing.add(arrow)
        drawing.origin = _engine.Dot(P(v(*origin)), radius=0.06, color=INK)
        drawing.add(drawing.origin)
        drawing.parts.update({"origin": drawing.origin, **{k: m for k, m in drawing.axes.items()}})
        return drawing

    def _compose(self, items, view, at, scale, drawing=None):
        verts = np.vstack([item.verts for item in items])
        centre = (verts.min(axis=0) + verts.max(axis=0)) / 2
        v = make_view(view, at, scale, centre)
        drawing = NakaginDrawing3D() if drawing is None else drawing
        drawing.view = v
        drawing.add(*[item.draw(v) for item in sorted(items, key=lambda item: (item.tier, _order(v, item.centre)))])
        return drawing

    def _box_item(self, tag, mesh: Mesh, color: str, *, tier: int, lines=None, drawing=None, levels: int = 8):
        def draw(v):
            group = _engine.VGroup(*_tone_buckets(mesh, v, color, levels, 0.62, 0.6))
            if lines is not None:
                group.add(lines(v, mesh))
            if drawing is not None:
                drawing(tag, group)
            return group

        return Item(tier, (mesh.V.min(axis=0) + mesh.V.max(axis=0)) / 2, mesh.V, draw)

    def _core_item(self, tower, *, storeys, side, pitch, z0, tag, drawing=None, color=None):
        cx, cy = tower.center
        s = side / 2
        top = storeys * pitch
        mesh = box_mesh(cx - s, cx + s, cy - s, cy + s, z0, top)
        door = 1.2

        def lines(v, m):
            pairs = []
            faces = {m.names[i]: i for i in _visible(m, v)}
            level = np.arange(1, storeys + 1) * pitch
            level = level[level > z0 + 1e-6]
            for name, (a, b) in (("front", ((cx - s, cy + s), (cx + s, cy + s))), ("right", ((cx + s, cy - s), (cx + s, cy + s))),
                                 ("back", ((cx - s, cy - s), (cx + s, cy - s))), ("left", ((cx - s, cy - s), (cx - s, cy + s)))):
                if name in faces:
                    pairs += [(v(a[0], a[1], z), v(b[0], b[1], z)) for z in level]
            group = _engine.VGroup(_d.segments(pairs, shade_of(CONCRETE, 0.3), W_HAIR, 0.8))
            if z0 < 1e-6 and "front" in faces:
                mid = ((cx - door / 2, cy + s), (cx + door / 2, cy + s))
                group.add(_shape([_screen(v, np.array([(mid[0][0], mid[0][1], 0), (mid[1][0], mid[1][1], 0), (mid[1][0], mid[1][1], 2.1), (mid[0][0], mid[0][1], 2.1)]))], GLASS_DARK, 1.0, MUTED, W_HAIR))
            return group

        return self._box_item(tag, mesh, color or _blend(CONCRETE, BACKGROUND, 0.3), tier=1, lines=lines, drawing=drawing, levels=6)

    def _capital_item(self, tower, *, drawing=None, tag=None, z0=None):
        mesh = solid_mesh(_model.capital_solid(tower))
        if z0 is not None:
            mesh = Mesh(mesh.V + np.array([0, 0, z0 - mesh.V[:, 2].min()]), mesh.N, mesh.faces, mesh.names)
        return self._box_item(tag, mesh, CAPITAL, tier=2, drawing=drawing, levels=6)

    def _podium_item(self, *, drawing=None):
        mesh = solid_mesh(_model.podium_solid())
        storeys = _model.value("podium.storeys").value
        pitch = _model.value("core.storey_height").value

        def lines(v, m):
            shapes = []
            for index in _visible(m, v):
                if abs(m.N[index][2]) > 0.5:
                    continue
                ring = m.V[m.faces[index]]
                low = ring[ring[:, 2] < ring[:, 2].min() + 1e-6]
                a, b = low[0], low[1]
                for storey in range(int(storeys)):
                    za, zb = storey * pitch + 0.8, storey * pitch + 2.0
                    p0, p1 = a + (b - a) * 0.04, a + (b - a) * 0.96
                    shapes.append(_screen(v, np.array([(p0[0], p0[1], za), (p1[0], p1[1], za), (p1[0], p1[1], zb), (p0[0], p0[1], zb)])))
            return _shape(shapes, shade_of(GLASS_DARK, 1.0), 1.0, MUTED, 0.0) if shapes else _engine.VGroup()

        return self._box_item("podium", mesh, _blend(CONCRETE, BACKGROUND, 0.5), tier=0, lines=lines, drawing=drawing, levels=6)

    def core_axon(self, tower, *, at=(0, 0, 0), scale: float = 0.2, view="axon", storeys: int | None = None, side: float | None = None, storey_height: float | None = None):
        """🧱 The concrete core with storey lines and the ground door; ``storeys``, ``side`` and ``storey_height`` override the model for live parametric cores. ``at`` is the screen position of the core centre. Parts: ``core``."""
        t = _tower(tower)
        drawing = NakaginDrawing3D()
        item = self._core_item(t, storeys=t.storeys if storeys is None else storeys, side=t.core_side if side is None else side,
                               pitch=t.storey_height if storey_height is None else storey_height, z0=0.0, tag="core")
        self._compose([item], view, at, scale, drawing)
        drawing.parts["core"] = drawing
        return drawing

    def capital_axon(self, tower, *, at=(0, 0, 0), scale: float = 0.2, view="axon"):
        """👑 The capital (roof head) frustum of a tower; ``at`` is the screen position of its centre. Parts: ``capital``."""
        drawing = NakaginDrawing3D()
        self._compose([self._capital_item(_tower(tower))], view, at, scale, drawing)
        drawing.parts["capital"] = drawing
        return drawing

    def podium_axon(self, *, at=(0, 0, 0), scale: float = 0.1, view="axon"):
        """🏬 The lower building as a simple massing with a ribbon of windows on every storey. Parts: ``podium``."""
        drawing = NakaginDrawing3D()
        self._compose([self._podium_item()], view, at, scale, drawing)
        drawing.parts["podium"] = drawing
        return drawing

    def _tower_items(self, *, lit, tint, types, capital, podium, where, core, ground, segments, capsules, cores, capitals, base, towers, storeys):
        items = []
        tower_list = _model.towers()
        pitch = _model.value("core.storey_height").value
        z0 = pitch * _model.value("core.podium_storeys").value if podium else 0.0
        if podium:
            items.append(self._podium_item(drawing=lambda tag, group: capitals.setdefault("podium", group)))
        for t in (tower_list[i] for i in towers):
            count = t.storeys if storeys is None else min(t.storeys, storeys)
            if core:
                items.append(self._core_item(t, storeys=count, side=t.core_side, pitch=t.storey_height, z0=z0, tag=t.index,
                                             drawing=lambda tag, group: cores.setdefault(tag, group)))
            if capital:
                items.append(self._capital_item(t, tag=t.index, z0=count * t.storey_height, drawing=lambda tag, group: capitals.setdefault(tag, group)))
        types_by_key = {c.key: _cap(c) for c in _model.capsule_types()}
        for slot in _model.slots():
            key = (slot.building, slot.tower, slot.storey, slot.facade, slot.item)
            if slot.tower not in towers or (where is not None and not where(slot)):
                continue
            cap = dict(types_by_key[slot.type], seg=segments)
            mesh = _world(capsule_mesh(cap["w"], cap["l"], cap["h"], cap["r"], segments), slot.plane)
            color = _blend(base, type_color(slot.type), 0.5) if types else base
            chosen = _match(tint, key)
            color = chosen if chosen else color
            on = lit is None or _match(lit, key, True)
            tone = 1.0 if on else 0.4
            colour = color if on else _blend(BACKGROUND, color, tone)
            ring = DATA if (lit is not None and on) else None
            centre = (mesh.V.min(axis=0) + mesh.V.max(axis=0)) / 2

            def draw(v, cap=cap, slot=slot, colour=colour, ring=ring, key=key):
                group = _capsule_lite(cap, slot.plane, v, colour, highlight=ring)
                capsules[key] = group
                return group

            items.append(Item(3, centre, mesh.V, draw))
        if ground:
            verts = np.vstack([item.verts for item in items])
            lo, hi = verts.min(axis=0) - 3.0, verts.max(axis=0) + 3.0

            def draw_ground(v):
                S = _screen(v, np.array([(lo[0], lo[1], 0), (hi[0], lo[1], 0), (hi[0], hi[1], 0), (lo[0], hi[1], 0)]))
                return _shape([S], _d.GROUND_PLANE, 1.0, _d.GREEN_GROUND, W_HAIR, 0.6)

            items.append(Item(-1, np.array([(lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2, 0.0]), np.array([(lo[0], lo[1], 0), (hi[0], hi[1], 0)]), draw_ground))
        return items

    def tower_axon(self, *, at=(0, 0, 0), scale: float = 0.1, view="axon", lit=None, tint=None, types: bool = True, capital: bool = True, podium: bool = True,
                   core: bool = True, ground: bool = False, where=None, towers=(0, 1), storeys: int | None = None, segments: int = 2, base: str = CAPSULE):
        """🗼 The whole building in axonometric: podium, both cores, 140 capsules, both capitals, depth-sorted by the painter's algorithm.

        ``lit`` (None, a collection of paths or a predicate) lights those capsules and dims the others; a path is a prefix of ``(b, t, s, f, i)`` so ``(0, 1, 4, 2)`` lights a facade branch.
        ``tint`` maps paths (or a predicate) to colours; ``types`` colours capsules by type; ``where`` filters slots, ``towers`` picks tower indices, ``storeys`` caps the core height. ``at`` is the screen position of the bounding-box centre.
        Parts: ``cores[tower]``, ``capsules[(b, t, s, f, i)]``, ``capitals[tower]`` and ``podium``."""
        drawing = NakaginDrawing3D()
        drawing.capsules, drawing.cores, drawing.capitals = {}, {}, {}
        items = self._tower_items(lit=lit, tint=tint, types=types, capital=capital, podium=podium, where=where, core=core, ground=ground, segments=segments,
                                  capsules=drawing.capsules, cores=drawing.cores, capitals=drawing.capitals, base=base, towers=towers, storeys=storeys)
        self._compose(items, view, at, scale, drawing)
        drawing.podium = drawing.capitals.pop("podium", _engine.VGroup())
        drawing.parts.update({"cores": drawing.cores, "capsules": drawing.capsules, "capitals": drawing.capitals, "podium": drawing.podium})
        return drawing

    def tower_elevation(self, *, at=(0, 0, 0), scale: float = 0.1, face: str = "north", lit=None, tint=None, types: bool = True, capital: bool = True,
                        podium: bool = True, where=None, towers=(0, 1), storeys: int | None = None, segments: int = 2, base: str = CAPSULE):
        """🏙 The building as an orthographic elevation seen from ``face`` (north: capsule windows face the viewer), same parts and options as ``tower_axon``."""
        return self.tower_axon(at=at, scale=scale, view=face, lit=lit, tint=tint, types=types, capital=capital, podium=podium, where=where, towers=towers, storeys=storeys, segments=segments, base=base)


def _tower(tower):
    if isinstance(tower, _model.Tower):
        return tower
    towers = _model.towers()
    return towers[tower] if isinstance(tower, int) else next(t for t in towers if t.key == tower)


def _match(spec, key, boolean: bool = False):
    if spec is None:
        return None if not boolean else True
    if callable(spec):
        return spec(key)
    if isinstance(spec, dict):
        best = max((len(p) for p in spec if tuple(key[:len(p)]) == tuple(p)), default=None)
        return None if best is None else spec[next(p for p in spec if len(p) == best and tuple(key[:len(p)]) == tuple(p))]
    return any(tuple(key[:len(p)]) == tuple(p) for p in spec)
