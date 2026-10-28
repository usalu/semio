"""🧊 Shaded 30° isometric axonometric of the Villa Savoye: pilotis, white box, dark-green ground floor, per discipline, explodable.

Projection: ``point + scale·(x·ux + y·uy + z·uz)`` with ux = (0.866, 0.5), uy = (−0.866, 0.5), uz = (0, 1), the same as ``_cube_faces`` of the energy clips.
The viewer stands south-west above the building (x east, y north, z up), so the south and west façades and the top are visible."""

from __future__ import annotations

import importlib
import math

import numpy as np

_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_engine = importlib.import_module("manim")

AXON = {"ux": (0.866, 0.5), "uy": (-0.866, 0.5), "uz": (0.0, 1.0)}
_UX, _UY, _UZ = (np.array(AXON[key], float) for key in ("ux", "uy", "uz"))


def axon_point(x, y, z, point, scale):
    """📍 Screen position of the model coordinate ``(x, y, z)`` metres in the 30° isometric axonometric whose model origin sits at ``point``: ``point + scale·(x·ux + y·uy + z·uz)``.

    The stable projection of every axonometric drawing (``villa_axon``, ``element_axon``, analysis views); the same as ``_cube_faces`` of the energy clips."""
    return np.array([point[0] + scale * (x * _UX[0] + y * _UY[0] + z * _UZ[0]), point[1] + scale * (x * _UX[1] + y * _UY[1] + z * _UZ[1]), 0.0])


def axon_origin(point, scale, model=None):
    """🧭 Screen position of the model origin (grid corner A/1 at ±0.00) of a ``villa_axon`` whose box centre sits at ``point``; feed it to ``axon_point`` as ``point``."""
    box = _c.geometry(model)["box"]
    return axon_point(-box["length"] / 2, -box["width"] / 2, -box["height"] / 2, point, scale)


VIEWS = {"sw": 0, "se": -1, "ne": 2, "nw": 1}
EXPLODE_METRES = 8.0
LAYER_ORDER = ("services", "structure", "architecture")
PRIMARY = ("structure", "architecture", "services")
LIGHT = np.array([-0.28, -0.54, 0.78])
UP = np.array([0.0, 0.0, 1.0])
MIN_RADIUS = 0.03
REACH = 2.2
LAYER_GAP = 1.2
STAGE_PLANE, STAGE_UNDER, STAGE_SLAB, STAGE_FIRST, STAGE_WALLS, STAGE_ROOF, STAGE_TOP, STAGE_OVERLAY = range(8)
SORTED_STAGES = (STAGE_UNDER, STAGE_FIRST, STAGE_WALLS, STAGE_TOP, STAGE_OVERLAY)
ROLES = {"wall": _d.INK, "slab": _d.mix(_d.INK, _d.CONCRETE, 0.3), "post": _d.CONCRETE, "ramp": _d.CONCRETE, "ground": _d.GREEN_GROUND,
         "glass": _d.GLASS, "service": _d.SERV, "garden": _d.mix(_d.GOOD, _d.BACKGROUND, 0.3), "soil": _d.mix(_d.INK, _d.GOOD, 0.1),
         "paving": _d.mix(_d.INK, _d.DATA, 0.2), "hall": _d.mix(_d.GREEN_GROUND, _d.BACKGROUND, 0.5), "frame": _d.INK, "void": _d.MUTED, "foot": _d.MUTED}
SMOOTH_ROLES = ("ground",)
P = _d.P


def brightness(normal, turns: int = 0) -> float:
    """☀️ Face brightness from a fixed light at the viewer's left: top 1.0, south 0.76, west 0.5, hidden sides 0.3 (``turns`` rotates the view by quarter turns)."""
    n = np.array(normal, float)
    if turns % 4:
        c, s = round(math.cos(math.pi / 2 * turns)), round(math.sin(math.pi / 2 * turns))
        n = np.array([c * n[0] - s * n[1], s * n[0] + c * n[1], n[2]])
    return float(min(1.0, max(0.3, 0.22 + float(n @ LIGHT))))


def ring_ccw(ring, ccw: bool = True) -> list:
    """🔁 Orient a 2D ring."""
    pts = [list(map(float, p[:2])) for p in ring]
    return pts if (_d.signed_area(pts) > 0) == ccw else pts[::-1]


class AxonView:
    """🔭 The 30° isometric projection with the viewer south-west above; ``view(x, y, z)`` is the screen point, ``depth`` larger is closer, ``facing(normal)`` positive looks at the viewer.

    ``point`` is the screen position of ``centre`` (the box centre for ``villa_axon``); ``turns`` rotates the building by quarter turns about ``centre`` to look from another corner."""

    kind = "axon"
    elevation = 35.264
    azimuth = 225.0

    def __init__(self, point=(0, 0, 0), scale: float = 0.26, centre=(0, 0, 0), turns: int = 0):
        self.point, self.scale, self.centre, self.turns = P(point), float(scale), np.asarray(centre, float), turns % 4
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


class Prim:
    """🧩 One drawable element piece in model coordinates: faces, painter's stage and plan position for depth sorting; ``base`` overrides the colour."""

    __slots__ = ("tag", "item", "stage", "foot", "faces", "merge", "alias", "sub", "z", "base")

    def __init__(self, tag, item, stage, foot, faces, *, merge=False, alias=(), sub=0, z=0.0, base=None):
        self.tag, self.item, self.stage, self.foot, self.faces = tag, item, stage, tuple(foot), faces
        self.merge, self.alias, self.sub, self.z, self.base = merge, tuple(alias), sub, z, base


def _poly(rings, normal, role):
    return ("poly", [np.asarray(ring, float).reshape(-1, 3) for ring in rings], None if normal is None else np.asarray(normal, float), role)


def _ring3(ring, z: float) -> np.ndarray:
    pts = np.asarray([[p[0], p[1]] for p in ring], float)
    return np.column_stack([pts, np.full(len(pts), float(z))])


_BOX = ((0, 1, 2, 3), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7))


def _solid(vertices, role) -> list:
    """🧱 Six faces of a hexahedron (bottom ring 0–3, top ring 4–7) with outward normals."""
    v = np.asarray(vertices, float)
    centre, faces = v.mean(axis=0), []
    for index in _BOX:
        pts = v[list(index)]
        n = np.cross(pts[1] - pts[0], pts[2] - pts[0])
        length = float(np.linalg.norm(n))
        if length < 1e-9:
            continue
        n = n / length
        if n @ (pts.mean(axis=0) - centre) < 0:
            n = -n
        faces.append(_poly([pts], n, role))
    return faces


def _box(ring4, z0: float, z1: float, role: str) -> list:
    ring = [list(map(float, p[:2])) for p in ring4]
    return _solid([[*p, z0] for p in ring] + [[*p, z1] for p in ring], role)


def _prism(rings, z0: float, z1: float, role: str, *, top: bool = True) -> list:
    """🧊 Side faces and top of an extruded polygon with holes, normals pointing away from the material."""
    faces = []
    for index, ring in enumerate(rings):
        pts = ring_ccw(ring, index == 0)
        for a, b in zip(pts, pts[1:] + pts[:1]):
            d = np.array(b) - np.array(a)
            length = float(np.linalg.norm(d))
            if length < 1e-9:
                continue
            faces.append(_poly([[(a[0], a[1], z0), (b[0], b[1], z0), (b[0], b[1], z1), (a[0], a[1], z1)]], (d[1] / length, -d[0] / length, 0.0), role))
    if top:
        faces.append(_poly([_ring3(ring, z1) for ring in rings], UP, role))
    return faces


def _discipline_of(guid, default, model):
    found = _c.membership(model).get(guid)
    return tuple(name for name in _d.DISCIPLINES if name in found) if found else default


def _item(tag, guid, kind, cls, model, default, extra=()):
    return {"tag": tag, "guid": guid, "kind": kind, "cls": cls, "disciplines": _discipline_of(guid, default, model),
            "names": frozenset({tag, guid, kind, cls, *extra})}


def _wall_corners(wall, g):
    a, along, normal, length = _c.wall_frame(wall, g)
    return a - normal * wall["thickness"] / 2, along, normal, length


def _tiles(ring, z0, z1, role, width: float = 1.2):
    """🧱 Vertical faces of a ring cut into narrow tiles ``(foot, face)`` so painter's sorting stays local."""
    out = []
    pts = ring_ccw(ring, True)
    for a, b in zip(pts, pts[1:] + pts[:1]):
        a, b = np.array(a), np.array(b)
        length = float(np.linalg.norm(b - a))
        if length < 1e-9:
            continue
        d = (b - a) / length
        normal = (d[1], -d[0], 0.0)
        count = max(1, math.ceil(length / width))
        for index in range(count):
            p, q = a + d * length * index / count, a + d * length * (index + 1) / count
            out.append((((p + q) / 2), _poly([[(p[0], p[1], z0), (q[0], q[1], z0), (q[0], q[1], z1), (p[0], p[1], z1)]], normal, role)))
    return out


def _both_sides(face):
    """👁️ The same face without a normal, so it is drawn from both sides (glass)."""
    return ("poly", face[1], None, face[3])


def _window_faces(wall, opening, g, shift: float = 0.01, panes_per: float = 2.0) -> list:
    origin, along, normal, length = _wall_corners(wall, g)
    out_normal = -normal
    s0, s1, z0, z1 = opening["start"], opening["end"], opening["sill_z"], opening["head_z"]
    offset = out_normal * shift

    def at(s, z):
        p = origin + along * s + offset
        return (p[0], p[1], z)

    quad = [at(s0, z0), at(s1, z0), at(s1, z1), at(s0, z1)]
    panes = max(2, round((s1 - s0) / panes_per))
    mullions = [(at(s0 + (s1 - s0) * k / panes, z0), at(s0 + (s1 - s0) * k / panes, z1)) for k in range(1, panes)]
    return [_poly([quad], (out_normal[0], out_normal[1], 0.0), "glass"), ("line", np.array(quad + [quad[0]], float), 1.1, "frame", None), ("segs", mullions, 0.8, "frame")]


def _service_prims(service, info, z_under, z_first) -> list:
    pts = [np.array(p, float) for p in service["points"]]
    kind = service["kind"]
    if kind in ("boiler", "radiator") and service.get("size"):
        sx, sy, sz = service["size"]
        if kind == "boiler":
            centre = pts[0]
            lo, hi = np.array([centre[0] - sx / 2, centre[1] - sy / 2]), np.array([centre[0] + sx / 2, centre[1] + sy / 2])
        else:
            a, b = pts[0], pts[1]
            lo, hi = np.array([min(a[0], b[0]), min(a[1], b[1]) - sy / 2]), np.array([max(a[0], b[0]), max(a[1], b[1]) + sy / 2])
        z0 = pts[0][2]
        ring = [(lo[0], lo[1]), (hi[0], lo[1]), (hi[0], hi[1]), (lo[0], hi[1])]
        stage = STAGE_UNDER if z0 < z_under else STAGE_FIRST
        return [Prim(service["tag"], info, stage, ((lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2), _box(ring, z0, z0 + sz, "service"), z=z0)]
    if kind == "drain":
        c = pts[0]
        r = (service.get("size") or [0.3])[0] / 2
        ring = [(c[0] + r * math.cos(t), c[1] + r * math.sin(t), c[2] + 0.02) for t in np.linspace(0, 2 * math.pi, 14)[:-1]]
        return [Prim(service["tag"], info, STAGE_TOP, (c[0], c[1]), [_poly([ring], UP, "service")], z=c[2])]
    width = max(1.6, 26 * (service.get("diameter") or 0.05))
    if abs(pts[0][0] - pts[-1][0]) < 1e-6 and abs(pts[0][1] - pts[-1][1]) < 1e-6:
        x, y = pts[0][0], pts[0][1]
        lo, hi = sorted((pts[0][2], pts[-1][2]))
        pieces = []
        for z_a, z_b, stage in ((lo, min(hi, z_under), STAGE_UNDER), (max(lo, z_first), hi, STAGE_FIRST)):
            if z_b - z_a > 1e-6:
                pieces.append(Prim(service["tag"], info, stage, (x, y), [("line", np.array([[x, y, z_a], [x, y, z_b]], float), width, "service", None)], z=z_a))
        return pieces
    stage = STAGE_UNDER if max(p[2] for p in pts) <= z_under else STAGE_FIRST
    centre = np.mean(pts, axis=0)
    return [Prim(service["tag"], info, stage, (centre[0], centre[1]), [("line", np.array(pts, float), width, "service", None)], z=float(centre[2]))]


_PRIMS: dict = {}


def prims(model=None) -> list:
    """🧩 Every drawable piece of the villa in model coordinates, memoized per model object (``building_geometry`` of the model)."""
    g = _c.geometry(model)
    hit = _PRIMS.get(id(g))
    if hit is not None and hit[0] is g:
        return hit[1]
    built = _build(g, model)
    _PRIMS[id(g)] = (g, built)
    if len(_PRIMS) > 12:
        _PRIMS.pop(next(iter(_PRIMS)))
    return built


def _build(g, model) -> list:
    out = []
    floor = next(s for s in g["slabs"] if s["kind"] != "ROOF")
    roof = next(s for s in g["slabs"] if s["kind"] == "ROOF")
    z_under, z_first = floor["top"] - floor["thickness"], floor["top"]

    for item in g["pilotis"]:
        info = _item(item["tag"], item["guid"], "pilotis", "IfcColumn", model, ("structure",))
        out.append(Prim(item["tag"], info, STAGE_UNDER, (item["x"], item["y"]), [("cyl", (item["x"], item["y"], item["base"], item["top"], item["radius"]), None, "post")]))

    info = _item(floor["tag"], floor["guid"], "slabs", "IfcSlab", model, ("architecture", "structure"))
    rings = [floor["outline"]] + [list(v) for v in floor["voids"]]
    out.append(Prim(floor["tag"], info, STAGE_SLAB, (0, 0), _prism(rings, z_under, z_first, "slab")))

    roof_info = _item(roof["tag"], roof["guid"], "slabs", "IfcSlab", model, ("architecture", "structure"))
    roof_rings = [roof["outline"]] + [list(v) for v in roof["voids"]]
    out.append(Prim(roof["tag"], roof_info, STAGE_ROOF, (0, 0), _prism(roof_rings, roof["top"] - roof["thickness"], roof["top"], "slab"), sub=0))
    solarium = (roof.get("zones") or {}).get("solarium")
    layers = [layer for layer in roof.get("layers", ()) if layer["id"] != "concrete"]
    cover = {zone: sum(layer["thickness"] for layer in layers if zone in layer.get("zones", ())) for zone in ("planted", "solarium")}
    if solarium and cover["planted"] > 0:
        out.append(Prim(roof["tag"], roof_info, STAGE_ROOF, (0, 0), _prism(roof_rings + [list(solarium)], roof["top"], roof["top"] + cover["planted"], "soil"), sub=1))
    if solarium:
        space = next((s for s in g["spaces"] if s["storey"] == "roof"), None)
        sol_info = _item(space["tag"], space["guid"], "spaces", "IfcSpace", model, ("architecture",), (roof["tag"], roof["guid"])) if space else roof_info
        out.append(Prim(sol_info["tag"], sol_info, STAGE_ROOF, (0, 0), _prism([list(solarium)], roof["top"], roof["top"] + cover["solarium"], "paving"), sub=2))

    curtain = g.get("curtain_wall")
    path_set = {tuple(round(v, 3) for v in p) for p in curtain["path"]} if curtain else set()
    for space in g["spaces"]:
        info = _item(space["tag"], space["guid"], "spaces", "IfcSpace", model, ("architecture",))
        ring = space["outline"]
        if space["storey"] == "ground":
            if path_set and path_set <= {tuple(round(v, 3) for v in p) for p in ring}:
                out.append(Prim(space["tag"], info, STAGE_PLANE, (0, 0), [_poly([_ring3(ring, 0.0)], UP, "hall")], sub=2))
                continue
            for centre, face in _tiles(ring, 0.0, z_under, "ground"):
                out.append(Prim(space["tag"], info, STAGE_UNDER, (centre[0], centre[1]), [face]))
        elif space["storey"] == "first" and space.get("predefined_type") == "EXTERNAL":
            out.append(Prim(space["tag"], info, STAGE_FIRST, (0, 0), [_poly([_ring3(ring, z_first + 0.004)], UP, "garden")]))

    for wall in g["walls"]:
        info = _item(wall["tag"], wall["guid"], "walls", "IfcWall", model, ("architecture",))
        origin, along, normal, length = _wall_corners(wall, g)
        ring = [origin, origin + along * length, origin + along * length + normal * wall["thickness"], origin + normal * wall["thickness"]]
        mid = origin + along * length / 2 + normal * wall["thickness"] / 2
        out.append(Prim(wall["tag"], info, STAGE_WALLS, (mid[0], mid[1]), _box(ring, wall["base"], wall["top"], "wall")))
        for opening in wall.get("openings", ()):
            w_info = _item(opening["window_tag"], opening["window_guid"], "windows", "IfcWindow", model, ("architecture",), (opening["tag"], opening["guid"]))
            out.append(Prim(opening["window_tag"], w_info, STAGE_WALLS, (mid[0], mid[1]), _window_faces(wall, opening, g), alias=(opening["tag"],), sub=1))

    if curtain:
        info = _item(curtain["tag"], curtain["guid"], "curtain", "IfcCurtainWall", model, ("architecture",))
        cx, cy = curtain["centre"]
        fronts, backs = [], []
        pts = np.asarray(curtain["path"], float)
        for a, b in zip(pts[:-1], pts[1:]):
            mid, d = (a + b) / 2, b - a
            n = np.array([d[1], -d[0], 0.0])
            n /= max(np.linalg.norm(n), 1e-9)
            if (mid - np.array([cx, min(mid[1], cy)])) @ n[:2] < 0:
                n = -n
            face = _poly([[(a[0], a[1], curtain["base"]), (b[0], b[1], curtain["base"]), (b[0], b[1], curtain["top"]), (a[0], a[1], curtain["top"])]], n, "glass")
            (fronts if n[0] + n[1] < 0 else backs).append(_both_sides(face))
        lines = [("segs", [((p[0], p[1], curtain["base"]), (p[0], p[1], curtain["top"])) for p in pts[::2]], 0.7, "frame")]
        out.append(Prim(curtain["tag"], info, STAGE_UNDER, (float(pts[:, 0].max()), float(pts[:, 1].max())), backs, merge=True, sub=-1))
        out.append(Prim(curtain["tag"], info, STAGE_UNDER, (float(pts[:, 0].min()), float(pts[:, 1].min()) + 0.5), fronts + lines, merge=True, sub=5))

    ramp_g = g.get("ramp")
    if ramp_g:
        for ramp in ramp_g.get("ramps") or [ramp_g]:
            ramp_info = _item(ramp["tag"], ramp["guid"], "ramp", "IfcRamp", model, ("architecture", "structure"))
            for flight in ramp["flights"]:
                info = _item(flight["tag"], flight["guid"], "ramp", "IfcRampFlight", model, ("architecture", "structure"), (ramp["tag"], ramp["guid"], "IfcRamp"))
                s0, s1 = np.array(flight["start"], float), np.array(flight["end"], float)
                d = (s1 - s0) / np.linalg.norm(s1 - s0)
                nrm = np.array([-d[1], d[0]])
                half = flight["width"] / 2
                top = [(*(s0 - nrm * half), flight["z0"]), (*(s0 + nrm * half), flight["z0"]), (*(s1 + nrm * half), flight["z1"]), (*(s1 - nrm * half), flight["z1"])]
                vertices = [[p[0], p[1], p[2] - 0.25] for p in top] + [list(p) for p in top]
                stage = STAGE_UNDER if flight["z1"] <= z_first + 1e-6 else STAGE_WALLS
                mid = (s0 + s1) / 2
                out.append(Prim(flight["tag"], info, stage, (mid[0], mid[1]), _solid(vertices, "ramp"), alias=(ramp["tag"],), z=flight["z0"]))
            for landing in ramp.get("landings", ()):
                z = landing["z"]
                centre = np.mean(np.asarray(landing["outline"], float), axis=0)
                stage = STAGE_UNDER if z < z_first - 1e-6 else STAGE_FIRST if z <= z_first + 1e-6 else STAGE_WALLS
                faces = _prism([landing["outline"]], max(z - 0.25, 0.0), z, "ramp") if z > 0.01 else [_poly([_ring3(landing["outline"], 0.003)], UP, "ramp")]
                out.append(Prim(ramp["tag"], ramp_info, stage, (centre[0], centre[1]), faces, sub=2, z=z))

    for screen in g.get("screens", ()):
        info = _item(screen["tag"], screen["guid"], "screens", "IfcWall", model, ("architecture",))
        path = np.asarray(screen["path"], float)
        tangent = np.gradient(path, axis=0)
        tangent /= np.maximum(np.linalg.norm(tangent, axis=1, keepdims=True), 1e-9)
        normals = np.column_stack([-tangent[:, 1], tangent[:, 0]]) * screen.get("thickness", 0.15) / 2
        ring = [list(p) for p in path + normals] + [list(p) for p in (path - normals)[::-1]]
        out.append(Prim(screen["tag"], info, STAGE_TOP, (path[:, 0].mean(), path[:, 1].mean()), _prism([ring], screen["base"], screen["top"], "wall"), merge=True))

    for service in g.get("services", ()):
        info = _item(service["tag"], service["guid"], "services", service["class"], model, ("services",), (service["kind"], service.get("system", "")))
        out.extend(_service_prims(service, info, z_under, z_first))
    return out


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
            faces.append(("poly", [np.column_stack([ring[:, :2], z(ring[:, 2])]) for ring in face[1]], face[2], face[3]))
        elif face[0] == "cyl":
            x, y, z0, z1, r = face[1]
            faces.append(("cyl", (x, y, z(z0), z(z1), r), None, face[3]))
        elif face[0] == "line":
            faces.append(("line", np.column_stack([face[1][:, :2], z(face[1][:, 2])]), *face[2:]))
        else:
            faces.append(("segs", [((a[0], a[1], z(a[2])), (b[0], b[1], z(b[2]))) for a, b in face[1]], *face[2:]))
    return Prim(prim.tag, prim.item, prim.stage, prim.foot, faces, merge=prim.merge, alias=prim.alias, sub=prim.sub, z=prim.z, base=prim.base)


def _wall_pieces(g, model, wall, e: float, o: float, w: float) -> list:
    """🧱 One wall as a solid with its ribbon opening: ``extrude`` raises it from the footprint, ``opening`` cuts the void and lifts the void block out, ``window`` slides the glass into the hole."""
    opening = wall["openings"][0]
    origin, along, normal, length = _wall_corners(wall, g)
    out = -normal
    th, zb = wall["thickness"], wall["base"]
    zt = zb + e * (wall["top"] - zb)
    s0, s1 = opening["start"], opening["end"]
    zs, zh = zb + e * (opening["sill_z"] - zb), zb + e * (opening["head_z"] - zb)
    hole = o > 0.001 and e > 0.02

    def at(s, t, z):
        p = origin + along * s + normal * t
        return (p[0], p[1], z)

    def rect(t, a, b, za, zb_):
        return [at(a, t, za), at(b, t, za), at(b, t, zb_), at(a, t, zb_)]

    out_n, in_n = (out[0], out[1], 0.0), (normal[0], normal[1], 0.0)
    along_n = (along[0], along[1], 0.0)
    faces = [_poly([rect(0, 0, length, zb, zt)] + ([rect(0, s0, s1, zs, zh)] if hole else []), out_n, "wall"),
             _poly([rect(th, 0, length, zb, zt)] + ([rect(th, s0, s1, zs, zh)] if hole else []), in_n, "wall"),
             _poly([[at(0, 0, zt), at(length, 0, zt), at(length, th, zt), at(0, th, zt)]], UP, "wall"),
             _poly([[at(0, 0, zb), at(0, th, zb), at(0, th, zt), at(0, 0, zt)]], (-along[0], -along[1], 0.0), "wall"),
             _poly([[at(length, 0, zb), at(length, th, zb), at(length, th, zt), at(length, 0, zt)]], along_n, "wall")]
    if hole:
        faces += [_poly([[at(s0, 0, zs), at(s1, 0, zs), at(s1, th, zs), at(s0, th, zs)]], UP, "wall"),
                  _poly([[at(s0, 0, zh), at(s1, 0, zh), at(s1, th, zh), at(s0, th, zh)]], -UP, "wall"),
                  _poly([[at(s0, 0, zs), at(s0, th, zs), at(s0, th, zh), at(s0, 0, zh)]], along_n, "wall"),
                  _poly([[at(s1, 0, zs), at(s1, th, zs), at(s1, th, zh), at(s1, 0, zh)]], (-along[0], -along[1], 0.0), "wall")]
    mid = origin + along * length / 2
    foot = (mid[0], mid[1])
    wall_item = _item(wall["tag"], wall["guid"], "walls", "IfcWall", model, ("architecture",))
    pieces = [Prim(wall["tag"], wall_item, STAGE_WALLS, foot, faces)]
    lift = out * (o * REACH)
    block = [(*(np.array(at(s, t, z)[:2]) + lift), z) for z in (zs, zh) for s, t in ((s0, -0.03), (s1, -0.03), (s1, th + 0.03), (s0, th + 0.03))]
    void_item = _item(opening["tag"], opening["guid"], "openings", "IfcOpeningElement", model, ("architecture",))
    pieces.append(Prim(opening["tag"], void_item, STAGE_WALLS, foot, _solid(block, "void"), sub=1))
    shift = out * ((1.0 - w) * REACH)
    quad = [(*(np.array(at(s, th / 2, z)[:2]) + shift), z) for s, z in ((s0, zs), (s1, zs), (s1, zh), (s0, zh))]
    panes = max(2, round((s1 - s0) / 2.0))
    mullions = [((*(np.array(at(s0 + (s1 - s0) * k / panes, th / 2, 0)[:2]) + shift), zs), (*(np.array(at(s0 + (s1 - s0) * k / panes, th / 2, 0)[:2]) + shift), zh)) for k in range(1, panes)]
    glass_item = _item(opening["window_tag"], opening["window_guid"], "windows", "IfcWindow", model, ("architecture",), (opening["tag"], opening["guid"]))
    glass = [_poly([quad], None, "glass"), ("line", np.array(quad + [quad[0]], float), 1.1, "frame", None), ("segs", mullions, 0.8, "frame")]
    pieces.append(Prim(opening["window_tag"], glass_item, STAGE_WALLS, foot, glass, sub=2))
    ring = [at(0, 0, zb), at(length, 0, zb), at(length, th, zb), at(0, th, zb)]
    pieces.append(Prim("footprint", _item("footprint", "footprint", "footprints", "IfcWall", model, ("architecture",)), STAGE_PLANE, foot,
                       [("line", np.array(ring + [ring[0]], float), 1.4, "foot", 0.1)], sub=-1))
    return pieces


def _slab_pieces(g, model, slab, e: float, o: float) -> list:
    """🧱 A slab as one plate, or the roof as its build-up of layers that spread apart with ``opening``."""
    disc = ("architecture", "structure")
    info = _item(slab["tag"], slab["guid"], "slabs", "IfcSlab", model, disc)
    z0 = slab["top"] - slab["thickness"]
    rings = [slab["outline"]] + [list(v) for v in slab["voids"]]
    layers = slab.get("layers")
    if not layers:
        return [Prim(slab["tag"], info, STAGE_ROOF, (0, 0), _prism(rings, z0, z0 + slab["thickness"] * e, "slab"))]
    solarium = (slab.get("zones") or {}).get("solarium")
    out, z = [], z0
    for index, layer in enumerate(layers):
        zones = set(layer.get("zones", ()))
        chosen = rings if (len(zones) > 1 or not solarium) else rings + [list(solarium)] if "planted" in zones else [list(solarium)]
        thickness = layer["thickness"] * e
        item = _item(slab["tag"], slab["guid"], "slabs", "IfcSlab", model, disc, (layer["id"],))
        out.append(Prim(layer["id"], item, STAGE_ROOF, (0, 0), _prism(chosen, z, z + thickness, "slab"), alias=(slab["tag"],), sub=index, base=_c.layer_color(layer["id"])))
        z += thickness + o * LAYER_GAP
    return out


class Axon:
    """🧊 Scene mixin: ``axon_view``, ``axon_point``, ``villa_axon``, ``element_axon``."""

    def axon_view(self, point=(0, 0, 0), scale: float = 0.26, *, model=None, view: str = "sw") -> AxonView:
        """🔭 The projection every axonometric widget shares; ``point`` is the screen position of the box centre."""
        box = _c.geometry(model)["box"]
        return AxonView(point, scale, (box["length"] / 2, box["width"] / 2, box["height"] / 2), VIEWS[view])

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
        Visible without ``shell``: pilotis A1–A5 and the lower 2 m of B1–E1; with it all 25. Returns an ``AxonDrawing`` with ``parts[tag]``, ``layers[discipline]``, ``layer_parts``, ``at(...)``."""
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
        if ground:
            self._ground(drawing, g, projection)
        for _, _, _, _, _, _, prim, layer, present in entries:
            group = painter.render(prim, layer, lifts[layer] if layered else 0.0)
            if not len(group.submobjects):
                continue
            drawing.add(group)
            for name in ([layer] if layered else present):
                drawing.layers[name].add(group)
                drawing.layer_part(name, prim.tag, group)
            for tag in (prim.tag, *prim.alias):
                drawing.part(tag, group)
        for prim in pieces:
            if prim.item["kind"] == "pilotis":
                x, y, z0, z1, _ = prim.faces[0][1]
                layer = next((name for name in PRIMARY if name in prim.item["disciplines"] and name in disciplines), None)
                if layer:
                    drawing.anchors[prim.tag] = P(projection(x, y, (z0 + z1) / 2 + (lifts[layer] if layered else 0.0)))
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
        of the wall, ``window`` slides the glass into the hole. Pilotis: ``extrude`` is the height. Slabs: ``extrude`` is the thickness, ``opening`` spreads the roof build-up layers apart (``parts["soil"]`` …).
        Any other tag (``"RM-1"``, ``"CW-1"``, ``"SC-1"``, services): its pieces as in ``villa_axon`` with heights scaled by ``extrude``. ``colors`` is ``"discipline"`` (default) or ``"material"``.
        Returns an ``AxonDrawing`` with ``parts[tag]`` (and ``parts["wall"]``, ``["void"]``, ``["glass"]``, ``["footprint"]`` for walls) and ``at(x, y, z)`` in model metres."""
        g = _c.geometry(model)
        e, o, w = (min(max(float(v), 0.0), 1.0) for v in (extrude, opening, window))
        wall = next((x for x in g["walls"] if tag in (x["tag"], *(y for op in x.get("openings", ()) for y in (op["tag"], op["window_tag"])))), None)
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
                x, y, z0, z1, r = p.faces[0][1]
                ring = [(x + r * math.cos(t), y + r * math.sin(t), z0) for t in np.linspace(0, 2 * math.pi, 25)]
                pieces.append(Prim("footprint", _item("footprint", "footprint", "footprints", "IfcColumn", model, ("structure",)), STAGE_PLANE, (x, y), [("line", np.array(ring, float), 1.4, "foot", 0.08)], sub=-1))
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
            drawing.parts.update({"wall": drawing.parts[wall["tag"]], "void": drawing.parts[wall["openings"][0]["tag"]], "glass": drawing.parts[wall["openings"][0]["window_tag"]],
                                  "footprint": drawing.parts["footprint"]})
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
