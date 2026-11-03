"""🧩 Native material primitives for axonometric projection without a rendering runtime."""

from __future__ import annotations

import importlib
import math

import numpy as np

_c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.📐️geometry")
_surfaces = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧩️surfaces.🐍️")
_measure = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.📏️geometry")

UP = (0.0, 0.0, 1.0)


STAGE_PLANE, STAGE_UNDER, STAGE_SLAB, STAGE_FIRST, STAGE_WALLS, STAGE_ROOF, STAGE_TOP, STAGE_OVERLAY = range(8)


STAGE_CELLAR_SLAB, STAGE_CELLAR = -2, -1


def ring_ccw(ring, ccw: bool = True) -> list:
    """🔁 Orient a 2D ring."""
    pts = [list(map(float, p[:2])) for p in ring]
    return pts if (_measure.signed_area(pts) > 0) == ccw else pts[::-1]


class Prim:
    """🧩 One drawable element piece in model coordinates: faces, painter's stage and plan position for depth sorting; ``base`` overrides the colour."""

    __slots__ = ("tag", "item", "stage", "foot", "faces", "merge", "alias", "sub", "z", "base", "skin", "outlines")

    def __init__(self, tag, item, stage, foot, faces, *, merge=False, alias=(), sub=0, z=0.0, base=None, skin=None):
        self.tag, self.item, self.stage = tag, item, stage
        self.foot = tuple(map(float, foot))
        self.faces = [_face(face) for face in faces]
        self.merge, self.alias, self.sub, self.z, self.base = merge, tuple(alias), sub, z, base
        self.skin = skin
        self.outlines = {}


def _face(face):
    kind, coordinates, value, role, *extra = face
    if kind == "poly":
        rings = [[tuple(map(float, point)) for point in ring] for ring in coordinates]
        return kind, rings, None if value is None else tuple(map(float, value)), role
    if kind == "cyl":
        return kind, tuple(map(float, coordinates)), None, role
    if kind == "line":
        points = [tuple(map(float, point)) for point in coordinates]
    elif kind == "segs":
        points = [(tuple(map(float, a)), tuple(map(float, b))) for a, b in coordinates]
    else:
        raise ValueError("Unknown native primitive face: " + kind)
    return kind, points, float(value), role, *(None if item is None else float(item) for item in extra)


def _poly(rings, normal, role):
    return _face(("poly", rings, normal, role))


def _ring3(ring, z: float) -> np.ndarray:
    pts = np.asarray([[p[0], p[1]] for p in ring], float)
    return np.column_stack([pts, np.full(len(pts), float(z))])


def _solid(vertices, role) -> list:
    """🧱 Caps and outward side faces of corresponding native polygon rings."""
    v = np.asarray(vertices, float)
    centre, faces = v.mean(axis=0), []
    count = len(v) // 2
    indices = (tuple(range(count)), tuple(range(count, 2 * count)), *((i, (i + 1) % count, (i + 1) % count + count, i + count) for i in range(count)))
    signed = _measure.signed_area(v[:count])
    for face_index, index in enumerate(indices):
        pts = v[list(index)]
        n = np.cross(pts[1] - pts[0], pts[2] - pts[0])
        length = float(np.linalg.norm(n))
        if length < 1e-9:
            continue
        n = n / length
        reverse = n @ (pts.mean(axis=0) - centre) < 0 if count == 4 else (n[2] > 0 if face_index == 0 else n[2] < 0 if face_index == 1 else signed < 0)
        if reverse:
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


def _part_faces(part, role):
    """🔻 Shade actual tapered sides and upper cap with their real outward normals."""
    faces=_prism([part["outline"],*part["holes"]],part["z0"],part["z1"],role) if part.get("holes") else [_poly([vertices],normal,role) for vertices,normal in _c.part_faces(part)]
    if part.get("surface_appearance") or part.get("surface_finish"):
        return [(*face[:3],_c.face_appearance(part,face[1][0],face[3])) for face in faces]
    return faces


def _discipline_of(guid, default, model):
    found = _c.membership(model).get(guid)
    return tuple(name for name in _c.DISCIPLINES if name in found) if found else default


def _item(tag, guid, kind, cls, model, default, extra=()):
    return {"tag": tag, "guid": guid, "kind": kind, "cls": cls, "disciplines": _discipline_of(guid, default, model),
            "names": frozenset({tag, guid, kind, cls, *extra})}


def _wall_corners(wall, g):
    a, along, normal, length = _c.wall_frame(wall, g)
    return np.asarray(a, float), np.asarray(along, float), np.asarray(normal, float), length


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


def _window_faces(wall, opening, g, shift: float = 0.0) -> list:
    """🪟 Render shared physical frame and panel parts without invented pane divisions."""
    filling = next((f for f in g["fillings"] if f["tag"]==opening.get("fill_tag")),None)
    if filling is None:
        return []
    return [face for part in filling["parts"] for face in _part_faces(part,"glass" if part["role"]=="glass" else "frame")]


def _wall_cells(length, base, top, openings):
    """🪚 Disjoint solid rectangles in wall coordinates after subtracting the union of every opening."""
    cuts = [(max(0.0, op["start"]), min(length, op["end"]), max(base, op["sill_z"]), min(top, op["head_z"])) for op in openings]
    cuts = [(a, b, c, d) for a, b, c, d in cuts if b > a and d > c]
    xs = sorted({0.0, float(length), *(v for a, b, _, _ in cuts for v in (a, b))})
    zs = sorted({float(base), float(top), *(v for _, _, c, d in cuts for v in (c, d))})
    cells = []
    for c, d in zip(zs, zs[1:]):
        start = None
        for a, b in zip(xs, xs[1:]):
            solid = not any(lo < (a + b) / 2 < hi and bottom < (c + d) / 2 < head for lo, hi, bottom, head in cuts)
            if solid and start is None:
                start = a
            if start is not None and (not solid or b == xs[-1]):
                cells.append((start, b if solid else a, c, d))
                start = None
    return cells


def _wall_solids(wall, g, *, extrude=1.0, cut=True):
    """🧱 Project the shared occupied wall leaves, preserving cavities and through openings."""
    base = wall["base"]
    role = "ground" if wall.get("storey") == "ground" and wall.get("is_external") else "wall"
    parts = wall["parts"]
    if not cut:
        a,b = np.asarray(wall["start"],float),np.asarray(wall["end"],float)
        direction = (b-a)/wall["length"]
        normal = np.array([-direction[1],direction[0]])
        parts = [{"outline":[a+normal*lo,b+normal*lo,b+normal*hi,a+normal*hi],"z0":layer["z0"],"z1":layer["z1"]}
                 for layer in wall["layers"] if not layer["is_void"] for lo,hi in [layer["offsets"]]]
    result=[]
    for part in parts:
        faces=_box(part["outline"],base+extrude*(part["z0"]-base),base+extrude*(part["z1"]-base),role)
        if part.get("surface_finish") and extrude==1:
            faces=[(*face[:3],_c.face_appearance(part,face[1][0],role)) for face in faces]
        result.append((np.mean(part["outline"],axis=0),faces))
    return result


def _ground_context(g, model=None):
    """🌱 Bind schematic context to a qualified local terrain datum without extending an estate surface."""
    source = _c._model.load_model() if model is None else model
    candidates = [obj for obj in source.get("elements", ()) if obj.get("class")=="IfcCovering" and obj.get("predefined_type")=="FLOORING" and "Pset_TerrainReconstruction" in obj.get("psets",{}) and "reference_surface_elevation" in obj.get("attributes",{})]
    elevations = {obj["attributes"]["reference_surface_elevation"]["value"] for obj in candidates}
    if len(elevations)!=1:
        return None
    box = g["box"]
    return {"elevation":float(next(iter(elevations))),"outline":((0.,0.),(box["length"],0.),(box["length"],box["width"]),(0.,box["width"])),"source_tags":tuple(obj["tag"] for obj in candidates),"surveyed":False,"uniform_surface":all(obj["psets"]["Pset_TerrainReconstruction"].get("UniformExtension",{}).get("value") is True for obj in candidates),"scope":"bounded local reference annotation; grading not surveyed"}


def _stage(z, first, roof, ground):
    """🪜 Painter stage relative to the actual local terrain, independent of the ±0 annotation."""
    return STAGE_CELLAR if z < ground-1e-6 else STAGE_UNDER if z < first-.25 else STAGE_WALLS if z < roof-.25 else STAGE_TOP


def _surface_stage(item, part, first, roof, ground):
    """🪵 Place native storey wearing surfaces after their structural plane at their true height."""
    if item["kind"]!="ramp_finish" and (item.get("class")=="IfcCovering" and item.get("predefined_type")=="FLOORING" or item["kind"] in ("parquet","tile","paving","grass_joint")):
        return {"cellar":STAGE_CELLAR_SLAB+.5,"ground":STAGE_PLANE+.5,"first":STAGE_SLAB+.5,"roof":STAGE_ROOF+.5}[item["storey"]]
    return _stage(part["z0"],first,roof,ground)


def _surface_prims(item, info, first, roof, ground):
    """🧩 Draw exact exposed same-product material instead of computational cell boundaries."""
    groups={}
    for part in item["parts"]:
        stage=_surface_stage(item,part,first,roof,ground)
        role=_c.surface_role(item,part)
        for face in _part_faces(part,role):
            groups.setdefault(stage,[]).append({"rings":face[1],"normal":face[2],"material":{"appearance":face[3],"material":part.get("material",item.get("material"))}})
    out=[]
    for stage,facets in groups.items():
        skin=_surfaces.material_skin(facets)
        if not skin:
            continue
        points=[p for facet in skin for p in facet["rings"][0]];centre=np.mean(np.asarray(points),axis=0)
        faces=[_poly(facet["rings"],facet["normal"],facet["material"]["appearance"])for facet in skin]
        out.append(Prim(item["tag"],info,stage,centre[:2],faces,z=min(p[2]for p in points),skin=tuple(facet["material"]for facet in skin)))
    return out


def _column_band(source, z0, z1):
    """🪚 Split only the displayed height band, preserving real native caps and holes."""
    if z0==source["z0"] and z1==source["z1"]:
        return [source]
    if not any(key in source for key in ("mesh","section","top_outline")):
        return [{**source,"z0":z0,"z1":z1}]
    api = _c._model
    result = []
    for cell in api._native_material_cells(source):
        stock = {"material":source.get("material"),"role":source.get("role","post"),"mesh":{"vertices":api.part_vertices(cell),"faces":api.part_faces(cell)}}
        clipped = api.surface_finishes().clip_stock(stock,[{"normal":[0,0,-1],"offset":-z0,"point":[0,0,z0]},{"normal":[0,0,1],"offset":z1,"point":[0,0,z1]}],exact=True)
        if clipped is not None:
            metadata = {key:value for key,value in source.items() if key not in ("mesh","section","top_outline","outline","holes","z0","z1")}
            result.append({**metadata,**api.surface_finishes().publish_stock(clipped)})
    return result


def _column_prims(item, model, first, roof, ground):
    """🏛️ Preserve every authored column section rather than a bounding cylinder."""
    info = _item(item["tag"], item["guid"], "pilotis", "IfcColumn", model, ("structure",))
    pieces = []
    for source in item["parts"]:
        cuts = [source["z0"],*sorted({level for level in (ground,first,roof) if source["z0"]<level<source["z1"]}),source["z1"]]
        bands = list(zip(cuts,cuts[1:]))
        for z0,z1 in bands:
            for part in _column_band(source,z0,z1):
                faces = [("cyl",(item["x"],item["y"],part["z0"],part["z1"],part["radius"]),None,"post")] if part["profile"]["shape"]=="circle" and not part.get("holes") and not any(key in part for key in ("mesh","section","top_outline")) else _part_faces(part,"post")
                pieces.append(Prim(item["tag"],info,_stage(part["z0"],first,roof,ground),(item["x"],item["y"]),faces,z=part["z0"]))
    return pieces


def _ground_index(entries):
    """🌱 Insert opaque terrain after buried pieces and before above-ground pieces."""
    return next((i for i,entry in enumerate(entries) if entry[1] >= STAGE_PLANE), len(entries))


def _slab_prims(slab, model, stage):
    """🪨 Draw the actual disjoint slab solids with their individual depths."""
    info = _item(slab["tag"], slab["guid"], "slabs", "IfcSlab", model, ("architecture", "structure"))
    return [Prim(slab["tag"], info, stage, tuple(np.mean(np.asarray(part["outline"]), axis=0)), _prism([part["outline"]], part["z0"], part["z1"], "slab"), z=part["z0"]) for part in slab["parts"]]


def _service_prims(service, info, z_under, z_first) -> list:
    """🔧 Project the same authored service envelopes used by plans and sections."""
    pieces = []
    for part in service["parts"]:
        role = _c.service_surface_role(service,part)
        ring, lo, hi = part["outline"], part["z0"], part["z1"]
        centre = np.mean(np.asarray(ring), axis=0)
        bands = [(lo, hi, STAGE_TOP)] if service["kind"] == "drain" else [(lo, min(hi, z_first), STAGE_UNDER), (max(lo, z_first), hi, STAGE_FIRST)]
        if "section" in part or "mesh" in part:
            pieces.append(Prim(service["tag"],info,STAGE_UNDER if (lo+hi)/2<z_first else STAGE_FIRST,tuple(centre),_part_faces(part,role),z=lo))
            continue
        for z0, z1, stage in bands:
            if z1 - z0 > 1e-9:
                if "top_outline" in part:
                    band = {**part,"outline":_c._model.part_outline_at(part,z0),"top_outline":_c._model.part_outline_at(part,z1),"z0":z0,"z1":z1}
                    faces = _part_faces(band,role)
                elif part.get("surface_appearance") or part.get("surface_finish"):
                    faces = _part_faces({**part,"z0":z0,"z1":z1},role)
                else:
                    faces = _prism([ring],z0,z1,role)
                pieces.append(Prim(service["tag"],info,stage,tuple(centre),faces,z=z0))
    return pieces


_PRIMS: dict = {}


def prims(model=None) -> list:
    """🧩 Every drawable piece of the villa in model coordinates, bound to the shared source/producer snapshot."""
    model,g = _c.geometry_context(model)
    context = _ground_context(g,model)
    terrain_key = None if context is None else (context["elevation"],context["outline"],context["source_tags"],context["uniform_surface"])
    key = (id(g),terrain_key,_build,_ground_context,_stage,_surface_stage,_column_prims,_column_band,_part_faces,_c.part_faces,_c.face_appearance,_surface_prims,_surfaces.producer_token())
    hit = _PRIMS.get(key)
    if hit is not None and hit[0] is g:
        return hit[1]
    built = _build(g, model)
    _PRIMS[key] = (g, built)
    if len(_PRIMS) > 2:
        _PRIMS.pop(next(iter(_PRIMS)))
    return built


def _build(g, model) -> list:
    out = []
    floor = next(s for s in g["slabs"] if s["tag"] == _c.role("floor", model))
    roof = next(s for s in g["slabs"] if s["tag"] == _c.role("roof", model))
    z_under, z_first = min(p["z0"] for p in floor["parts"]), g["storeys"][floor["level"]]
    roof_level = g["storeys"][roof["level"]]
    context = _ground_context(g,model)
    ground = context["elevation"] if context else g["storeys"].get("ground",0.)

    for item in g["pilotis"]:
        out.extend(_column_prims(item, model, z_first, roof_level, ground))

    for slab in g["slabs"]:
        stage = {"cellar": STAGE_CELLAR_SLAB, "ground": STAGE_PLANE, "first": STAGE_SLAB, "roof": STAGE_ROOF}[slab["storey"]]
        out.extend(_slab_prims(slab, model, stage))
        info = _item(slab["tag"], slab["guid"], "slabs", "IfcSlab", model, ("architecture",))
        for part in slab.get("finish_parts", ()):
            out.append(Prim(slab["tag"], info, stage + .25, tuple(np.mean(np.asarray(part["outline"]), axis=0)), _part_faces(part, "paving"), alias=(part["layer"],), z=part["z0"]))

    curtain = g.get("curtain_wall")
    for wall in g["walls"]:
        info = _item(wall["tag"], wall["guid"], "walls", "IfcWall", model, ("architecture",))
        stage = _stage(wall["base"], z_first, roof_level, ground)
        for centre, faces in _wall_solids(wall, g):
            out.append(Prim(wall["tag"], info, stage, centre, faces, z=wall["base"]))
        for opening in wall.get("openings", ()):
            tag, guid, cls = opening.get("fill_tag"), opening.get("fill_guid"), opening.get("fill_class")
            filled = bool(tag)
            tag, guid, cls = (tag, guid, cls) if filled else (opening["tag"], opening["guid"], "IfcOpeningElement")
            kind = "doors" if cls == "IfcDoor" else "windows" if filled else "openings"
            w_info = _item(tag, guid, kind, cls, model, ("architecture",), (opening["tag"], opening["guid"]))
            mid = (np.asarray(wall["start"]) + np.asarray(wall["end"])) / 2
            faces = _window_faces(wall, opening, g)
            out.append(Prim(tag, w_info, stage, mid, faces if filled else faces[1:2], alias=(opening["tag"],) if filled else (), sub=1, z=opening["sill_z"]))

    if curtain:
        info = _item(curtain["tag"],curtain["guid"],"curtain","IfcCurtainWall",model,("architecture",))
        for part in curtain["parts"]:
            out.append(Prim(curtain["tag"],info,STAGE_UNDER,tuple(np.mean(part["outline"],axis=0)),_part_faces(part,"glass" if part["role"]=="glass" else "frame"),merge=True,z=part["z0"]))

    for filling in g["fillings"]:
        if filling["host"] in {wall["tag"] for wall in g["walls"]}:
            continue
        info=_item(filling["tag"],filling["guid"],"doors" if filling["class"]=="IfcDoor" else "windows",filling["class"],model,("architecture",),(filling["opening"],))
        for part in filling["parts"]:
            out.append(Prim(filling["tag"],info,_stage(part["z0"],z_first,roof_level,ground),tuple(np.mean(part["outline"],axis=0)),_part_faces(part,"glass" if part["role"]=="glass" else "frame"),alias=(filling["opening"],),merge=True,z=part["z0"]))

    ramp_g = g.get("ramp")
    if ramp_g:
        for ramp in ramp_g.get("ramps") or [ramp_g]:
            ramp_info = _item(ramp["tag"], ramp["guid"], "ramp", "IfcRamp", model, ("architecture", "structure"))
            for flight in ramp["flights"]:
                info = _item(flight["tag"], flight["guid"], "ramp", "IfcRampFlight", model, ("architecture", "structure"), (ramp["tag"], ramp["guid"], "IfcRamp"))
                stage = STAGE_UNDER if flight["z1"] <= z_first + 1e-6 else STAGE_WALLS
                for part in flight["parts"]:
                    mid = np.mean(np.asarray(_c._model.part_footprint(part)),axis=0)
                    out.append(Prim(flight["tag"],info,stage,tuple(mid),_part_faces(part,"ramp"),alias=(ramp["tag"],),z=part["z0"]))
            for landing in ramp.get("landings", ()):
                if landing["owner"]!=ramp["tag"]:
                    continue
                z = landing["z"]
                stage = STAGE_UNDER if z < z_first - 1e-6 else STAGE_FIRST if z <= z_first + 1e-6 else STAGE_WALLS
                aliases = tuple(other["tag"] for other in ramp_g["ramps"] if other["tag"]!=ramp["tag"] and any(p["tag"]==landing["tag"] for p in other["landings"]))
                for part in landing["parts"]:
                    centre = np.mean(np.asarray(_c._model.part_footprint(part)),axis=0)
                    out.append(Prim(ramp["tag"],ramp_info,stage,tuple(centre),_part_faces(part,"ramp"),alias=aliases,sub=2,z=z))

    for screen in g.get("screens", ()):
        info = _item(screen["tag"], screen["guid"], "screens", "IfcWall", model, ("architecture",))
        for part in screen["parts"]:
            centre = tuple(np.mean(part["outline"],axis=0))
            out.append(Prim(screen["tag"],info,STAGE_TOP,centre,_part_faces(part,"wall"),merge=True,z=part["z0"]))

    for kind in ("stairs", "railings", "details", "furnishings", "beams", "foundations"):
        for item in g.get(kind, ()):
            info = _item(item["tag"], item["guid"], kind, item["class"], model, tuple(item["domains"]), (item["kind"],))
            out.extend(_surface_prims(item,info,z_first,roof_level,ground))

    for service in g.get("services", ()):
        info = _item(service["tag"], service["guid"], "services", service["class"], model, ("services",), (service["kind"], service.get("system", "")))
        out.extend(_service_prims(service, info, z_under, z_first))
    return out
     