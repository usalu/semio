"""🧊 Shaded 30° isometric axonometric of the Villa Savoye: pilotis, white box, dark-green ground floor, per discipline, explodable.

Projection: ``point + scale·(x·ux + y·uy + z·uz)`` with ux = (0.866, 0.5), uy = (−0.866, 0.5), uz = (0, 1), the same as ``_cube_faces`` of the energy clips.
The view names describe model corners; x and y follow the source plan, with geographic north approximately (-1, -1)."""

from __future__ import annotations

import importlib
import math

import numpy as np

_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_engine = importlib.import_module("manim")
_primitives = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
ring_ccw = _primitives.ring_ccw
Prim = _primitives.Prim
_poly = _primitives._poly
_ring3 = _primitives._ring3
_solid = _primitives._solid
_box = _primitives._box
_prism = _primitives._prism
_part_faces = _primitives._part_faces
_discipline_of = _primitives._discipline_of
_item = _primitives._item
_wall_corners = _primitives._wall_corners
_tiles = _primitives._tiles
_both_sides = _primitives._both_sides
_window_faces = _primitives._window_faces
_wall_cells = _primitives._wall_cells
_wall_solids = _primitives._wall_solids
_stage = _primitives._stage
_column_prims = _primitives._column_prims
_ground_index = _primitives._ground_index
_slab_prims = _primitives._slab_prims
_service_prims = _primitives._service_prims
prims = _primitives.prims
_build = _primitives._build
UP = _primitives.UP
STAGE_PLANE = _primitives.STAGE_PLANE
STAGE_UNDER = _primitives.STAGE_UNDER
STAGE_SLAB = _primitives.STAGE_SLAB
STAGE_FIRST = _primitives.STAGE_FIRST
STAGE_WALLS = _primitives.STAGE_WALLS
STAGE_ROOF = _primitives.STAGE_ROOF
STAGE_TOP = _primitives.STAGE_TOP
STAGE_OVERLAY = _primitives.STAGE_OVERLAY
STAGE_CELLAR_SLAB = _primitives.STAGE_CELLAR_SLAB
STAGE_CELLAR = _primitives.STAGE_CELLAR

AXON = {"ux": (0.866, 0.5), "uy": (-0.866, 0.5), "uz": (0.0, 1.0)}
_UX, _UY, _UZ = (np.array(AXON[key], float) for key in ("ux", "uy", "uz"))


def axon_point(x, y, z, point, scale):
    """📍 Screen position of the model coordinate ``(x, y, z)`` metres in the 30° isometric axonometric whose model origin sits at ``point``: ``point + scale·(x·ux + y·uy + z·uz)``.

    The stable projection of every axonometric drawing (``villa_axon``, ``element_axon``, analysis views); the same as ``_cube_faces`` of the energy clips."""
    return np.array([point[0] + scale * (x * _UX[0] + y * _UY[0] + z * _UZ[0]), point[1] + scale * (x * _UX[1] + y * _UY[1] + z * _UZ[1]), 0.0])


def axon_origin(point, scale, model=None):
    """🧭 Screen position of the model origin (grid corner A/1 at ±0.00) of a ``villa_axon`` whose box centre sits at ``point``; feed it to ``axon_point`` as ``point``."""
    low, high = _bounds(prims(model))
    centre = (low + high) / 2
    return axon_point(*(-centre), point, scale)


VIEWS = {"sw": 0, "se": -1, "ne": 2, "nw": 1}
EXPLODE_METRES = 8.0
LAYER_ORDER = ("services", "structure", "architecture")
PRIMARY = ("structure", "architecture", "services")
LIGHT = np.array([-0.28, -0.54, 0.78])
MIN_RADIUS = 0.03
REACH = 2.2
LAYER_GAP = 1.2
SORTED_STAGES = (STAGE_CELLAR, STAGE_UNDER, STAGE_FIRST, STAGE_WALLS, STAGE_TOP, STAGE_OVERLAY)
ROLES = {"wall": _d.INK, "slab": _d.mix(_d.INK, _d.CONCRETE, 0.3), "post": _d.CONCRETE, "ramp": _d.CONCRETE, "ground": _d.GREEN_GROUND,
         "glass": _d.GLASS, "service": _d.SERV, "garden": _d.mix(_d.GOOD, _d.BACKGROUND, 0.3), "soil": _d.mix(_d.INK, _d.GOOD, 0.1),
         "paving": _d.mix(_d.INK, _d.CONCRETE, 0.7), "wood": "#A48869", "tile": _d.CONCRETE, "metal": _d.MUTED, "concrete": _d.CONCRETE, "hall": _d.mix(_d.GREEN_GROUND, _d.BACKGROUND, 0.5), "frame": _d.INK, "void": _d.MUTED, "foot": _d.MUTED}
SMOOTH_ROLES = ("ground",)
P = _d.P


def brightness(normal, turns: int = 0) -> float:
    """☀️ Face brightness from a fixed light at the viewer's left: top 1.0, south 0.76, west 0.5, hidden sides 0.3 (``turns`` rotates the view by quarter turns)."""
    n = np.array(normal, float)
    if turns % 4:
        c, s = round(math.cos(math.pi / 2 * turns)), round(math.sin(math.pi / 2 * turns))
        n = np.array([c * n[0] - s * n[1], s * n[0] + c * n[1], n[2]])
    return float(min(1.0, max(0.3, 0.22 + float(n @ LIGHT))))


class AxonView:
    """🔭 The 30° isometric projection above the model’s negative x/y corner; ``view(x, y, z)`` is the screen point, ``depth`` larger is closer, ``facing(normal)`` positive looks at the viewer.

    ``point`` is the screen position of ``centre`` (the box centre for ``villa_axon``); ``turns`` rotates the building by quarter turns about ``centre`` to look from another corner."""

    kind = "axon"
    elevation = 35.264
    azimuth = 225.0

    def __init__(self, point=(0, 0, 0), scale: float = 0.26, centre=(0, 0, 0), turns: int = 0):
        self.point, self.scale, self.centre, self.turns = np.asarray(P(point), float), float(scale), np.asarray(centre, float), turns % 4
        c, s = round(math.cos(math.pi / 2 * self.turns)), round(math.sin(math.pi / 2 * self.turns))
        rotation = np.array([[c, -s], [s, c]], float)
        self.M = np.zeros((3, 3))
        for index in range(2):
            self.M[index, :2] = _UX * rotation[0, index] + _UY * rotation[1, index]
        self.M[2, :2] = _UZ
        self.T = np.array([-(rotation[0, 0] + rotation[1, 0]), -(rotation[0, 1] + rotation[1, 1]), 1.0])

    def __call__(self, x, y=None, z=0.0) -> np.ndarray:
        if y is None:
            x, y, z = (list(x) + [0.0])[:3]
        return self.point + self.scale * ((np.array([x, y, z], float) - self.centre) @ self.M)

    def project(self, array, dz: float = 0.0) -> np.ndarray:
        """📐 Project an ``(n, 3)`` array of model points at once, optionally lifted by ``dz`` metres."""
        out = self.point + self.scale * ((np.asarray(array, float).reshape(-1, 3) - self.centre) @ self.M)
        if dz:
            out[:, 1] += self.scale * dz
        return out

    def depth(self, x, y=None, z=0.0) -> float:
        """👁️ Larger is closer to the viewer."""
        if y is None:
            x, y, z = (list(x) + [0.0])[:3]
        return float((np.array([x, y, z], float) - self.centre) @ self.T)

    def key(self, x: float, y: float) -> float:
        """🔑 Painter's key of a plan position: larger is closer."""
        return float((x - self.centre[0]) * self.T[0] + (y - self.centre[1]) * self.T[1])

    def facing(self, normal) -> float:
        """🧭 Positive when a face with this outward normal looks at the viewer."""
        return float(np.asarray(normal, float) @ self.T)


class AxonDrawing(_d.Drawing):
    """🧊 Axonometric drawing: ``parts[tag]``, ``layers[discipline]``, ``layer_parts[discipline][tag]``, ``anchors`` and ``at`` for model coordinates and tags."""

    def __init__(self, view):
        super().__init__(view)
        self.layer_parts, self.layer_dz = {}, {}

    def at(self, *args, layer: str | None = None) -> np.ndarray:
        """📍 ``at("B3")`` the centre of a part, ``at(x, y, z)`` a model coordinate; ``layer`` adds that discipline's explode lift."""
        dz = self.layer_dz.get(layer, 0.0)
        if len(args) == 1 and isinstance(args[0], str):
            tag = args[0]
            if layer and tag in self.layer_parts.get(layer, {}):
                return np.array(self.layer_parts[layer][tag].get_center())
            return super().at(tag)
        x, y, z = (list(args) + [0.0, 0.0])[:3]
        return self.view(x, y, z + dz)

    def layer_part(self, layer: str, tag: str, shape):
        """🏷️ Register a part inside one discipline layer."""
        bucket = self.layer_parts.setdefault(layer, {})
        if tag in bucket and bucket[tag] is not shape:
            bucket[tag].add(shape)
        else:
            bucket[tag] = shape if isinstance(shape, _engine.VGroup) else _engine.VGroup(shape)


class _Selection(_c.Selection):
    """🎯 Selection by tag, class, kind, GlobalId and the aliases of each drawn piece (a ramp name lights its flights, an opening its window)."""

    def state(self, item) -> str:
        names = item["names"]
        if names & self.ghost:
            return "ghost"
        if self.highlight:
            return "highlight" if names & self.highlight else "dim"
        return "normal"


class _Painter:
    """🖌️ Turns pieces into mobjects for one view: culling, three face shades, highlight / ghost / dim / x-ray styling, merged buckets."""

    def __init__(self, view, selection, discipline_mode: bool, fades: dict | None = None):
        self.view, self.selection, self.discipline_mode, self.fades = view, selection, discipline_mode, fades or {}

    def base(self, prim, role, layer):
        if prim.base is not None:
            return prim.base
        if role in ("void", "foot"):
            return ROLES[role]
        if self.discipline_mode:
            colour = _d.DISCIPLINES[layer]
            return _d.mix(colour, _d.BACKGROUND, 0.35) if role in ("glass", "frame") else colour
        return ROLES[role]

    def factor(self, prim, layer, state):
        if state in ("highlight", "ghost") or prim.item["kind"] == "pilotis":
            return 1.0
        return self.fades.get(layer, self.fades.get(None, 1.0))

    def style(self, prim, role, base, bright, layer):
        state = self.selection.state(prim.item)
        fill, opacity, stroke, stroke_opacity, width = self.selection.colors(prim.item, base, bright)
        if state == "normal" and role == "glass":
            opacity, stroke_opacity = 0.62, 1.0
        if state == "normal" and role == "garden":
            opacity = 0.92
        factor = self.factor(prim, layer, state)
        if factor < 1.0:
            opacity, stroke_opacity = opacity * factor, max(stroke_opacity * factor, 0.2)
        return state, fill, opacity, stroke, stroke_opacity, width

    def render(self, prim, layer, dz):
        """🧱 The mobjects of one piece, in painter's order."""
        group, buckets = _engine.VGroup(), {}
        for face in prim.faces:
            kind = face[0]
            if kind == "poly":
                self._poly(prim, face, layer, dz, group, buckets)
            elif kind == "cyl":
                group.add(self._cylinder(prim, face[1], layer, dz))
            elif kind == "line":
                group.add(self._line(prim, face, layer, dz))
            elif kind == "segs":
                group.add(self._segs(prim, face, layer, dz))
        for (fill, opacity, stroke, stroke_opacity, width), rings in buckets.items():
            group.add(_multi(rings, fill, opacity, stroke, width, stroke_opacity))
        return group

    def _poly(self, prim, face, layer, dz, group, buckets):
        rings, normal, role = face[1], face[2], face[3]
        if normal is not None and self.view.facing(normal) <= 1e-6:
            return
        bright = 0.8 if normal is None else brightness(normal, self.view.turns)
        screen = [self.view.project(ring, dz) for ring in rings]
        if role == "void":
            state = self.selection.state(prim.item)
            colour = _d.DATA if state == "highlight" else _d.MUTED
            group.add(_d.polygon(screen[0], colour, 0.2, None))
            group.add(_d.polyline(screen[0], colour, 1.2, 0.9, True, 0.08))
            return
        state, fill, opacity, stroke, stroke_opacity, width = self.style(prim, role, self.base(prim, role, layer), bright, layer)
        smooth = role in SMOOTH_ROLES or (prim.merge and normal is not None and normal[2] < 0.5)
        if smooth and state in ("normal", "dim"):
            stroke, width = fill, 0.5
        if prim.merge and len(rings) == 1:
            buckets.setdefault((fill, round(opacity, 3), stroke, round(stroke_opacity, 3), width), []).append(screen[0])
        elif len(rings) == 1:
            group.add(_d.polygon(screen[0], fill, opacity, stroke, width, stroke_opacity))
        else:
            group.add(_d.region(screen, fill, opacity, stroke, width, stroke_opacity))
        if state == "ghost":
            for ring in screen:
                group.add(_d.polyline(ring, stroke, 1.0, 0.8, True, 0.07))

    def _cylinder(self, prim, spec, layer, dz):
        x, y, z0, z1, r = spec
        view = self.view
        base = self.base(prim, "post", layer)
        state = self.selection.state(prim.item)
        centre0, centre1 = view(x, y, z0 + dz), view(x, y, z1 + dz)
        a = max(1.2247 * r * view.scale, MIN_RADIUS)
        b = a * 0.5774
        angles = np.linspace(0.0, math.pi, 12)
        top = [centre1 + np.array([a * math.cos(t), b * math.sin(t), 0.0]) for t in angles]
        bottom = [centre0 + np.array([-a * math.cos(t), -b * math.sin(t), 0.0]) for t in angles]
        outline = top + bottom
        fill, opacity, stroke, stroke_opacity, width = self.selection.colors(prim.item, base, 0.68)
        shape = _engine.VGroup(_d.polygon(outline, fill, opacity, stroke, width, stroke_opacity))
        if state != "ghost" and a > 0.034:
            lit = self.selection.colors(prim.item, base, 1.0)[0]
            dark = self.selection.colors(prim.item, base, 0.34)[0]
            shape.add(_d.polygon([centre0 + (-a * 0.92, 0, 0), centre0 + (-a * 0.42, 0, 0), centre1 + (-a * 0.42, 0, 0), centre1 + (-a * 0.92, 0, 0)], lit, opacity, None))
            shape.add(_d.polygon([centre0 + (a * 0.38, 0, 0), centre0 + (a * 0.95, 0, 0), centre1 + (a * 0.95, 0, 0), centre1 + (a * 0.38, 0, 0)], dark, opacity, None))
        elif state == "ghost":
            shape.add(_d.polyline(outline, stroke, 1.0, 0.8, True, 0.07))
        return shape

    def _line(self, prim, face, layer, dz):
        pts, width, role = face[1], face[2], face[3]
        dash = face[4] if len(face) > 4 else None
        colour, opacity = self._line_colour(prim, role, layer)
        return _d.polyline(self.view.project(pts, dz), colour, width, opacity, False, dash)

    def _segs(self, prim, face, layer, dz):
        pairs, width, role = face[1], face[2], face[3]
        colour, opacity = self._line_colour(prim, role, layer)
        first = self.view.project([a for a, _ in pairs], dz)
        last = self.view.project([b for _, b in pairs], dz)
        return _d.segments(list(zip(first, last)), colour, width, opacity)

    def _line_colour(self, prim, role, layer):
        state = self.selection.state(prim.item)
        base = self.base(prim, role, layer)
        if state == "highlight":
            return _d.DATA, 1.0
        if state == "ghost":
            return base, 0.4
        colour = _d.shade(base, 0.3) if role == "frame" else base if role == "foot" else _d.shade(base, 0.85)
        if state == "dim":
            colour = _d.mix(colour, _d.BACKGROUND, self.selection.dim)
        return colour, 0.95 * self.factor(prim, layer, state)


def _multi(rings, fill, opacity, stroke, width, stroke_opacity):
    """🧩 Several disjoint polygons as one shape."""
    shape = _engine.VMobject()
    for ring in rings:
        pts = [P(p) for p in ring]
        shape.start_new_path(pts[0])
        for point in pts[1:] + [pts[0]]:
            shape.add_line_to(point)
    shape.set_fill(fill, opacity)
    shape.set_stroke(stroke, width if stroke else 0, stroke_opacity if stroke else 0)
    return shape


def _bounds(pieces):
    """🔲 Model-space bounding box ``(low, high)`` of all faces of some pieces."""
    pts = []
    for prim in pieces:
        for face in prim.faces:
            if face[0] == "poly":
                pts.extend(face[1])
            elif face[0] == "cyl":
                x, y, z0, z1, r = face[1]
                pts.append(np.array([[x - r, y - r, z0], [x + r, y + r, z1]]))
            elif face[0] == "line":
                pts.append(face[1])
            elif face[0] == "segs":
                pts.append(np.array([p for pair in face[1] for p in pair]))
    stack = np.vstack(pts)
    return stack.min(axis=0), stack.max(axis=0)


def _lift(prim, zb: float, factor: float):
    """📏 A copy of a piece whose heights above ``zb`` are scaled by ``factor`` (live ``extrude``)."""
    def z(value):
        return zb + (value - zb) * factor

    faces = []
    for face in prim.faces:
        if face[0] == "poly":
            normal = None if face[2] is None else np.asarray(face[2])*np.array([factor,factor,1.])
            if normal is not None:
                length = np.linalg.norm(normal)
                normal = normal/length if length > 1e-12 else face[2]
            rings = [np.asarray(points, float) for points in face[1]]
            faces.append(("poly",[np.column_stack([ring[:,:2],z(ring[:,2])]) for ring in rings],normal,face[3]))
        elif face[0] == "cyl":
            x, y, z0, z1, r = face[1]
            faces.append(("cyl", (x, y, z(z0), z(z1), r), None, face[3]))
        elif face[0] == "line":
            points = np.asarray(face[1], float)
            faces.append(("line", np.column_stack([points[:, :2], z(points[:, 2])]), *face[2:]))
        else:
            faces.append(("segs", [((a[0], a[1], z(a[2])), (b[0], b[1], z(b[2]))) for a, b in face[1]], *face[2:]))
    return Prim(prim.tag, prim.item, prim.stage, prim.foot, faces, merge=prim.merge, alias=prim.alias, sub=prim.sub, z=prim.z, base=prim.base)


def _wall_pieces(g, model, wall, e: float, o: float, w: float) -> list:
    """🧱 Animate every authored cut and filling independently from a wall's footprint."""
    origin, along, normal, length = _wall_corners(wall, g)
    base, thickness = wall["base"], wall["thickness"]
    mid = origin + along * length / 2 + normal * thickness / 2
    info = _item(wall["tag"], wall["guid"], "walls", "IfcWall", model, ("architecture",))
    pieces = [Prim(wall["tag"], info, STAGE_WALLS, mid, faces) for _, faces in _wall_solids(wall, g, extrude=e, cut=o > 0.001 and e > 0.02)]

    def at(s, t, z, shift):
        p = origin + along * s + normal * t - normal * shift
        return (p[0], p[1], base + e * (z - base))

    for opening in wall.get("openings", ()):
        a, b, c, d = (opening[key] for key in ("start", "end", "sill_z", "head_z"))
        if o > 0.001 and e > 0.02:
            vertices = [at(s, t, z, o * REACH) for z in (c, d) for s, t in ((a, -0.03), (b, -0.03), (b, thickness + 0.03), (a, thickness + 0.03))]
            info = _item(opening["tag"], opening["guid"], "openings", "IfcOpeningElement", model, ("architecture",))
            pieces.append(Prim(opening["tag"], info, STAGE_WALLS, mid, _solid(vertices, "void"), sub=1))
        tag, guid, cls = opening.get("fill_tag"), opening.get("fill_guid"), opening.get("fill_class")
        if not tag:
            continue
        info = _item(tag, guid, "doors" if cls == "IfcDoor" else "windows", cls, model, ("architecture",), (opening["tag"], opening["guid"]))
        filling = next(f for f in g["fillings"] if f["tag"]==tag)
        faces = []
        for part in filling["parts"]:
            ring = [np.asarray(p)-normal*((1-w)*REACH) for p in part["outline"]]
            faces.extend(_box(ring,base+e*(part["z0"]-base),base+e*(part["z1"]-base),"glass" if part["role"]=="glass" else "frame"))
        pieces.append(Prim(tag, info, STAGE_WALLS, mid, faces, sub=2))
    ring = [at(s, t, base, 0) for s, t in ((0, 0), (length, 0), (length, thickness), (0, thickness))]
    pieces.append(Prim("footprint", _item("footprint", "footprint", "footprints", "IfcWall", model, ("architecture",)), STAGE_PLANE, mid,
                       [("line", np.array(ring + [ring[0]], float), 1.4, "foot", 0.1)], sub=-1))
    return pieces


def _slab_pieces(g, model, slab, e: float, o: float) -> list:
    """🧱 Keep measured structural depths and level roof finishes when exploded."""
    info = _item(slab["tag"], slab["guid"], "slabs", "IfcSlab", model, ("architecture", "structure"))
    pieces = []
    for part in slab["parts"]:
        z0, z1 = part["z0"], part["z0"] + e * (part["z1"] - part["z0"])
        pieces.append(Prim(slab["tag"], info, STAGE_ROOF, (0, 0), _prism([part["outline"]], z0, z1, "slab"), alias=("structural",)))
    for part in slab.get("finish_parts", ()):
        base = min(p["z0"] for p in slab["parts"])
        z0, z1 = (base + e * (part[key] - base) + o * LAYER_GAP for key in ("z0", "z1"))
        pieces.append(Prim(part["layer"], info, STAGE_ROOF, (0, 0), _prism([part["outline"]], z0, z1, "paving"), alias=(slab["tag"],), sub=1))
    return pieces


class Axon:
    """🧊 Scene mixin: ``axon_view``, ``axon_point``, ``villa_axon``, ``element_axon``."""

    def axon_view(self, point=(0, 0, 0), scale: float = 0.26, *, model=None, view: str = "sw") -> AxonView:
        """🔭 The projection every axonometric widget shares; ``point`` is the screen position of the box centre."""
        low, high = _bounds(prims(model))
        return AxonView(point, scale, (low + high) / 2, VIEWS[view])

    def axon_point(self, x, y, z, point=(0, 0, 0), scale: float = 0.26, *, discipline=None, explode: float = 0.0, model=None, view: str = "sw"):
        """📍 Screen position of a model coordinate in ``villa_axon`` with the same ``point`` (box centre), ``scale`` and ``explode``; ``discipline`` adds that layer's lift.

        The module function ``axon_point(x, y, z, point, scale)`` is the same projection with ``point`` at the model origin."""
        dz = {"architecture": explode * EXPLODE_METRES, "services": -explode * EXPLODE_METRES}.get(discipline, 0.0)
        return self.axon_view(point, scale, model=model, view=view)(x, y, z + dz)

    def villa_axon(self, point=(0, 0, 0), scale: float = 0.26, *, model=None, highlight=(), disciplines=("architecture", "structure", "services"), explode: float = 0.0,
                   ghost=(), grid: bool = False, labels: bool = False, colors: str | None = None, shell: float | None = None, view: str = "sw", dim: float = 0.62,
                   ground: bool | None = None):
        """🧊 Shaded axonometric of the villa: white box on pilotis, dark-green set-back ground floor with the curved glass wall, ribbon windows, ramp, roof garden and curved solarium screens.

        ``point`` is the screen position of the box centre, ``scale`` screen units per metre (0.17 fills the stage height, 0.26 is larger than the stage).
        ``highlight`` / ``ghost`` take tags (``"B3"``, ``"RW-S"``), IFC classes (``"IfcColumn"``), kinds (``"pilotis"``, ``"walls"``) or GlobalIds; highlighted pieces are DATA yellow, the rest dims, ghosts are dashed outlines at 12 %.
        ``disciplines`` selects partial models; ``colors`` is ``"material"`` (white box, green ground floor) or ``"discipline"`` (default when fewer than three disciplines or exploded).
        ``explode`` lifts architecture by ``explode × 8 m`` and lowers services by the same, each shared element is drawn once per layer. ``shell`` is the opacity of everything that is not a piloti
        (automatic 0.22 when pilotis are highlighted or ghosted, so every piloti shows through; 0.55 for the structure layer of partial or exploded drawings). ``view`` is ``"sw"`` (default), ``"se"``, ``"ne"`` or ``"nw"``.
        Occlusion follows the authored solids; ``shell`` reveals internal supports and circulation. Returns an ``AxonDrawing`` with ``parts[tag]``, ``layers[discipline]``, ``layer_parts``, ``at(...)``."""
        g = _c.geometry(model)
        projection = self.axon_view(point, scale, model=model, view=view)
        selection = _Selection(highlight, ghost, dim)
        layered = explode > 0
        discipline_mode = (colors == "discipline") if colors else (len(disciplines) < 3 or layered)
        pieces = prims(model)
        lit = any(selection.state(p.item) in ("highlight", "ghost") and p.item["kind"] == "pilotis" for p in pieces)
        fades = {}
        if shell is not None:
            fades[None] = shell
        elif lit:
            fades[None] = 0.22
        elif discipline_mode and (layered or "architecture" not in disciplines):
            fades["structure"] = 0.55
        painter = _Painter(projection, selection, discipline_mode, fades)
        overlay = lit and fades.get(None, 1.0) < 1.0
        drawing = AxonDrawing(projection)
        lifts = {"architecture": explode * EXPLODE_METRES, "structure": 0.0, "services": -explode * EXPLODE_METRES}
        drawing.layer_dz = lifts
        for name in disciplines:
            drawing.layers[name] = _engine.VGroup()
        entries = []
        for index, prim in enumerate(pieces):
            present = [name for name in prim.item["disciplines"] if name in disciplines]
            if not present:
                continue
            stage = STAGE_OVERLAY if overlay and prim.item["kind"] == "pilotis" and selection.state(prim.item) == "highlight" else prim.stage
            stage_key = projection.key(*prim.foot) if stage in SORTED_STAGES else 0.0
            for layer in (present if layered else [next(name for name in PRIMARY if name in present)]):
                entries.append((LAYER_ORDER.index(layer) if layered else 0, stage, stage_key, prim.sub, prim.z if stage in SORTED_STAGES else 0.0, index, prim, layer, present))
        entries.sort(key=lambda row: row[:6])
        if ground is None:
            ground = not discipline_mode
        ground_index = _ground_index(entries) if ground else -1
        for entry_index, entry in enumerate(entries):
            if entry_index == ground_index:
                self._ground(drawing, g, projection)
            _, _, _, _, _, _, prim, layer, present = entry
            group = painter.render(prim, layer, lifts[layer] if layered else 0.0)
            if not len(group.submobjects):
                continue
            drawing.add(group)
            for name in ([layer] if layered else present):
                drawing.layers[name].add(group)
                drawing.layer_part(name, prim.tag, group)
            for tag in (prim.tag, *prim.alias):
                drawing.part(tag, group)
        if ground_index == len(entries):
            self._ground(drawing, g, projection)
        for tag in {p.tag for p in pieces if p.item["kind"] == "pilotis"}:
            members = [p for p in pieces if p.tag == tag]
            layer = next((name for name in PRIMARY if name in members[0].item["disciplines"] and name in disciplines), None)
            if layer:
                low, high = _bounds(members)
                centre = (low + high) / 2
                centre[2] += lifts[layer] if layered else 0.0
                drawing.anchors[tag] = P(projection(*centre))
        if grid:
            drawing.grid = self._axon_grid(projection, g)
            drawing.add(drawing.grid)
        if labels:
            drawing.tags = self._axon_labels(drawing, selection, pieces)
            drawing.add(drawing.tags)
        return drawing

    def element_axon(self, tag, point=(0, 0, 0), scale: float = 0.26, *, model=None, extrude: float = 1.0, opening: float = 1.0, window: float = 1.0, highlight=(),
                     colors: str | None = None, view: str = "sw", dim: float = 0.62):
        """🧱 One isolated element at its model coordinates with live parameters; ``point`` is the screen position of the centre of its bounding box.

        Walls (``"W-S"``, or the tag of its opening ``"OP-S"`` / window ``"RW-S"``): ``extrude`` raises the solid from its dashed footprint, ``opening`` cuts the ribbon void and lifts the translucent void block out
        of the wall, ``window`` slides the glass into the hole. Pilotis: ``extrude`` is the height. Slabs: ``extrude`` is the thickness, ``opening`` spreads the roof build-up layers apart (``parts["paving"]`` …).
        Any other tag (``"RM-1"``, ``"CW-1"``, ``"SC-1"``, services): its pieces as in ``villa_axon`` with heights scaled by ``extrude``. ``colors`` is ``"discipline"`` (default) or ``"material"``.
        Returns an ``AxonDrawing`` with ``parts[tag]`` (and ``parts["wall"]``, ``["void"]``, ``["glass"]``, ``["footprint"]`` for walls) and ``at(x, y, z)`` in model metres."""
        g = _c.geometry(model)
        e, o, w = (min(max(float(v), 0.0), 1.0) for v in (extrude, opening, window))
        wall = next((x for x in g["walls"] if tag in (x["tag"], *(y for op in x.get("openings", ()) for y in (op["tag"], op.get("fill_tag"))))), None)
        slab = next((x for x in g["slabs"] if x["tag"] == tag), None)
        if wall:
            pieces = _wall_pieces(g, model, wall, e, o, w)
        elif slab:
            pieces = _slab_pieces(g, model, slab, e, o)
        else:
            pieces = [p for p in prims(model) if tag in (p.tag, *p.alias)]
            if not pieces:
                raise KeyError(f"No axonometric element {tag!r}")
            zb = float(_bounds(pieces)[0][2])
            pieces = [_lift(p, zb, max(e, 1e-3)) for p in pieces]
            pilotis = [p for p in pieces if p.item["kind"] == "pilotis"]
            for p in pilotis:
                low, high = _bounds([p])
                if p.faces[0][0] == "cyl":
                    x, y, z0, z1, r = p.faces[0][1]
                    ring = [(x + r * math.cos(t), y + r * math.sin(t), z0) for t in np.linspace(0, 2 * math.pi, 73)]
                else:
                    top = next(face for face in p.faces if face[0] == "poly" and face[2] is not None and face[2][2] > .9)
                    ring = [(v[0], v[1], low[2]) for v in top[1][0]]
                pieces.append(Prim("footprint", _item("footprint", "footprint", "footprints", "IfcColumn", model, ("structure",)), STAGE_PLANE, tuple((low[:2] + high[:2]) / 2), [("line", np.array(ring + [ring[0]], float), 1.4, "foot", 0.08)], sub=-1))
        low, high = _bounds(pieces)
        projection = AxonView(point, scale, (low + high) / 2, VIEWS[view])
        selection = _Selection(highlight, (), dim)
        discipline_mode = colors != "material"
        painter = _Painter(projection, selection, discipline_mode)
        drawing = AxonDrawing(projection)
        entries = []
        for index, prim in enumerate(pieces):
            layer = next((name for name in PRIMARY if name in prim.item["disciplines"]), "architecture")
            stage_key = projection.key(*prim.foot) if prim.stage in SORTED_STAGES else 0.0
            entries.append((prim.stage, stage_key, prim.sub, index, prim, layer))
        entries.sort(key=lambda row: row[:4])
        for _, _, _, _, prim, layer in entries:
            group = painter.render(prim, layer, 0.0)
            if not len(group.submobjects):
                continue
            drawing.add(group)
            drawing.layers.setdefault(layer, _engine.VGroup()).add(group)
            for name in (prim.tag, *prim.alias):
                drawing.part(name, group)
        if wall:
            drawing.parts["wall"] = drawing.parts.get(wall["tag"], _engine.VGroup())
            for alias, key in (("void", "tag"), ("glass", "fill_tag")):
                group = _engine.VGroup(*(drawing.parts[op[key]] for op in wall.get("openings", ()) if op.get(key) in drawing.parts))
                drawing.parts[alias] = group
        return drawing

    def _ground(self, drawing, g, view):
        """🟩 Ground plane with the shadow of the box on it."""
        box = g["box"]
        pad = 3.0
        plane = _d.polygon([view(x, y, 0.0) for x, y in ((-pad, -pad), (box["length"] + pad, -pad), (box["length"] + pad, box["width"] + pad), (-pad, box["width"] + pad))],
                           _d.GROUND_PLANE, 1.0, None)
        shadow = _d.polygon([view(x + 1.2, y + 2.2, 0.0) for x, y in ((0, 0), (box["length"], 0), (box["length"], box["width"]), (0, box["width"]))], _d.BACKGROUND, 0.38, None)
        group = _engine.VGroup(plane, shadow)
        drawing.add(group)
        drawing.ground = group
        drawing.part("ground", group)

    def _axon_labels(self, drawing, selection, pieces):
        """🏷️ Tag labels above the highlighted pieces (at most ten)."""
        group, seen = _engine.VGroup(), set()
        for prim in pieces:
            if prim.tag in seen or selection.state(prim.item) != "highlight" or prim.tag not in drawing.parts:
                continue
            seen.add(prim.tag)
            group.add(self.text(prim.tag, np.array(drawing.parts[prim.tag].get_top()) + np.array([0.0, 0.17, 0.0]), size=_d.S_SMALL, color=_d.DATA))
        return group if len(seen) <= 10 else _engine.VGroup()

    def _axon_grid(self, view, g):
        """🔢 Ground-plane grid with bubbles 1…5 / A…E and the ±0.00 origin marker; it never lifts with ``explode``."""
        gx, gy = g["grid"]["x"], g["grid"]["y"]
        lx, ly = g["box"]["length"], g["box"]["width"]
        group = _engine.VGroup()
        pad = 1.4
        lines = [(view(x, -pad, 0), view(x, ly + pad, 0)) for x in gx] + [(view(-pad, y, 0), view(lx + pad, y, 0)) for y in gy]
        group.add(_d.segments([pair for a, b in lines for pair in _d.dashes([a, b], 0.14, 0.1)], _d.MUTED, 0.8, 0.55))
        south = view.facing((0, -1, 0)) > 0
        east = view.facing((1, 0, 0)) > 0
        y_num = -pad - 0.9 if south else ly + pad + 0.9
        x_let = lx + pad + 0.9 if east else -pad - 0.9
        for x, label in zip(gx, g["grid"]["labels_x"]):
            group.add(self._bubble(label, view(x, y_num, 0)))
        for y, label in zip(gy, g["grid"]["labels_y"]):
            group.add(self._bubble(label, view(x_let, y, 0)))
        origin = view(gx[0], gy[0], 0)
        cross = _d.segments([(origin + (-0.07, 0, 0), origin + (0.07, 0, 0)), (origin + (0, -0.07, 0), origin + (0, 0.07, 0))], _d.DATA, 1.8)
        group.add(cross, self.text("±0.00", origin + (-0.1, 0.1, 0), size=_d.S_SMALL, color=_d.DATA, anchor="r"))
        return group

    def _bubble(self, label, centre):
        """⭕ Grid bubble with its label."""
        ring = _engine.Circle(radius=0.12, stroke_color=_d.MUTED, stroke_width=1.2).move_to(centre)
        return _engine.VGroup(ring, self.text(label, centre, size=_d.S_SMALL, color=_d.INK))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             