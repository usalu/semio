"""🏢 Schema-first Nakagin Capsule Tower teaching model: one JSON source, stdlib-only geometry, program and energy derivations."""

from __future__ import annotations

from dataclasses import dataclass, field
from functools import lru_cache
import json
import math
from pathlib import Path

LANGUAGES = ("en", "de")
BASES = ("sourced", "assumed", "derived")
FACADE_KEYS = ("N", "E", "S", "W")


@dataclass(frozen=True)
class Value:
    """🏷 A model value with unit, basis (sourced, assumed, derived) and optional source key."""
    value: object
    unit: str
    basis: str
    source: str | None = None


@dataclass(frozen=True)
class Plane:
    """📐 Orthonormal right-handed plane: origin and the X, Y, Z axes in world coordinates."""
    origin: tuple
    x: tuple
    y: tuple
    z: tuple


@dataclass(frozen=True)
class Circle:
    """⭕ Circle by centre, normal and radius."""
    center: tuple
    normal: tuple
    radius: float


@dataclass(frozen=True)
class Solid:
    """🧊 Polyhedral solid: vertices, faces (vertex index loops, outward winding), unique edges and attached features."""
    vertices: tuple
    faces: tuple
    edges: tuple
    window: Circle | None = None
    bolts: tuple = ()
    meta: dict = field(default_factory=dict)


@dataclass(frozen=True)
class CapsuleType:
    """💊 One parameter variant of the parametric capsule."""
    key: str
    name: dict
    use: dict
    width: float
    length: float
    height: float
    corner_radius: float
    corner_segments: int
    window_diameter: float
    window_u: float
    window_v: float
    rotation: float
    occupants: int
    fittings: tuple
    interior_width: float
    interior_length: float
    interior_height: float
    count: int


@dataclass(frozen=True)
class WindowUnitState:
    """🪭 An inspectable sash pose and independent radial fan coverage/orientation."""
    sash_angle: float
    blind_fraction: float
    blind_rotation: float


@dataclass(frozen=True)
class CasingDimensions:
    """📦 A physical rectangular extent expressed in the nominal capsule reference frame."""
    width: float
    length: float
    height: float
    left: float
    right: float
    back: float
    front: float
    bottom: float
    top: float


@dataclass(frozen=True)
class Tower:
    """🗼 One tower: core, storeys, capsule facades and capital."""
    key: str
    name: dict
    index: int
    center: tuple
    storeys: int
    capsule_storeys: int
    first_capsule_storey: int
    core_side: float
    shaft_side: float
    storey_height: float
    base_elevation: float
    height: float
    capital_height: float
    capital_overhang: float
    roof_clearance: float


@dataclass(frozen=True)
class Slot:
    """📍 One capsule position: tree path, item, type and placement plane (with the type rotation) beside the unrotated facade plane."""
    building: int
    tower: int
    storey: int
    facade: int
    item: int
    type: str
    plane: Plane
    facade_plane: Plane
    rotation: float
    sublevel: int = 0
    layout: str = "Slash"
    floor: int = 0

    @property
    def path(self) -> tuple:
        return (self.building, self.tower, self.storey, self.facade)


@lru_cache(maxsize=1)
def _source() -> str:
    return Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8")


def _leaves(node, trail=()):
    if isinstance(node, dict):
        if "value" in node and "unit" in node:
            yield trail, node
            return
        for key, item in node.items():
            yield from _leaves(item, trail + (key,))
    elif isinstance(node, list):
        for index, item in enumerate(node):
            yield from _leaves(item, trail + (index,))


def _validate(data: dict) -> dict:
    required = ("version", "languages", "revision", "notes", "sources", "capsule", "types", "assignment", "facades",
                "items", "core", "towers", "capital", "podium", "energy", "climate", "variants", "detail", "foundation", "families")
    missing = [key for key in required if key not in data]
    if missing:
        raise ValueError(f"Missing keys: {missing}")
    for trail, leaf in _leaves({key: data[key] for key in required[4:]}):
        if leaf["basis"] not in BASES:
            raise ValueError(f"Unknown basis at {trail}")
        if leaf["basis"] == "sourced" and "source" not in leaf:
            raise ValueError(f"Sourced value without source at {trail}")
        if "source" in leaf and leaf["source"] not in data["sources"]:
            raise ValueError(f"Unknown source {leaf['source']} at {trail}")
    keys = [t["key"] for t in data["types"]]
    if len(set(keys)) != len(keys) or sorted(data["assignment"]["order"]) != sorted(keys):
        raise ValueError("Type keys must be unique and match the assignment order")
    c = data["capsule"]
    for t in data["types"]:
        r, u, v = t["window_diameter"]["value"] / 2, t["window_u"]["value"], t["window_v"]["value"]
        if abs(u) + r > c["width"]["value"] / 2 or v - r < 0 or v + r > c["height"]["value"]:
            raise ValueError(f"Window of type {t['key']} does not fit the front face")
    detail = data["detail"]
    if not 0 < detail["shell_thickness"]["value"] < min(c["width"]["value"], c["height"]["value"]) / 4:
        raise ValueError("Shell thickness must leave an interior cavity")
    if detail["segments"]["value"] < 12 or int(detail["segments"]["value"]) != detail["segments"]["value"]:
        raise ValueError("Detailed curves require at least twelve integral segments")
    for tower in data["towers"]:
        floors = tower["floors"]
        if len(floors) != tower["capsule_storeys"]["value"]:
            raise ValueError("Authored capsule floor count must match tower metadata")
        total = 0
        for floor in floors:
            count = sum(len(g["items"]) for g in floor["groups"])
            if count != floor["count"]["value"]:
                raise ValueError("Each authored floor must match its sourced capsule count")
            total += count
            if len({g["facade"] for g in floor["groups"]}) != len(floor["groups"]):
                raise ValueError("Facade groups must be unique within each floor")
        if total != tower["capsule_count"]["value"]:
            raise ValueError("Authored slots must match the measured tower capsule total")
        crown = max(item["center"][2]["value"] + c["height"]["value"] for floor in floors for group in floor["groups"] for item in group["items"])
        if tower["roof_elevation"]["value"] <= crown:
            raise ValueError("Concrete roof datum must clear the highest authored capsule crown")
        if abs(tower["roof_elevation"]["value"] - crown - tower["roof_clearance"]["value"]) > 1e-6:
            raise ValueError("Each tower roof clearance must derive from its own authored datum and crown")
    return data


def load() -> dict:
    """📦 Load a fresh, validated copy of the raw Nakagin teaching data."""
    return _validate(json.loads(_source()))


@lru_cache(maxsize=1)
def _data() -> dict:
    return load()


def _walk(path: str):
    node = _data()
    for step in path.split("."):
        if isinstance(node, list):
            keyed = next((i for i in node if isinstance(i, dict) and i.get("key") == step), None)
            node = keyed if keyed is not None else (node[int(step)] if step.isdigit() and int(step) < len(node) else None)
        else:
            node = node.get(step) if isinstance(node, dict) else None
        if node is None:
            raise KeyError(f"Unknown path: {path}")
    return node


def value(path: str) -> Value:
    """🔎 Resolve a dotted path such as `capsule.width` or `types.A.window_diameter` to a Value."""
    leaf = _walk(path)
    if not (isinstance(leaf, dict) and "value" in leaf and "unit" in leaf):
        raise KeyError(f"Not a value: {path}")
    return Value(leaf["value"], leaf["unit"], leaf["basis"], leaf.get("source"))


def sources() -> dict:
    """📚 Source registry: key to title and url or repo path."""
    return dict(_data()["sources"])


def _v(path: str):
    return value(path).value


def _rad(degrees: float) -> float:
    return math.radians(degrees)


@lru_cache(maxsize=1)
def capsule_types() -> tuple:
    """💊 The capsule types with resolved dimensions and counts derived from the slots."""
    d = _data()
    counts = {}
    for slot in slots():
        counts[slot.type] = counts.get(slot.type, 0) + 1
    c = d["capsule"]
    out = []
    for t in d["types"]:
        out.append(CapsuleType(
            key=t["key"], name=t["name"], use=t["use"],
            width=c["width"]["value"], length=c["length"]["value"], height=c["height"]["value"],
            corner_radius=c["corner_radius"]["value"], corner_segments=c["corner_segments"]["value"],
            window_diameter=t["window_diameter"]["value"], window_u=t["window_u"]["value"], window_v=t["window_v"]["value"],
            rotation=t["rotation"]["value"], occupants=t["occupants"]["value"],
            fittings=tuple({"key": f["key"], "name": f["name"], "width": f["width"]["value"], "length": f["length"]["value"], "count": f["count"]["value"]} for f in t["fittings"]),
            interior_width=c["interior"]["width"]["value"], interior_length=c["interior"]["length"]["value"], interior_height=c["interior"]["height"]["value"],
            count=counts.get(t["key"], 0)))
    return tuple(out)


def capsule_type(key: str) -> CapsuleType:
    """🔑 Look up a capsule type by key."""
    for t in capsule_types():
        if t.key == key:
            return t
    raise KeyError(f"Unknown capsule type: {key}")


def _get(t, name):
    return t[name] if isinstance(t, dict) else getattr(t, name)


def _param(t, name):
    if isinstance(t, dict) and name in t:
        return t[name]
    if not isinstance(t, dict):
        return getattr(t, name)
    base = capsule_type(t["key"]) if "key" in t else capsule_types()[0]
    return getattr(base, name)


def _edges(faces) -> tuple:
    seen = {}
    for face in faces:
        for a, b in zip(face, face[1:] + face[:1]):
            seen.setdefault((min(a, b), max(a, b)), None)
    return tuple(seen)


def _profile(w, h, r, segments) -> list:
    if r<=1e-12:
        return [(-w/2,0),(w/2,0),(w/2,h),(-w/2,h)]
    pts = []
    for (cx, cz), start in (((w / 2 - r, r), -90), ((w / 2 - r, h - r), 0), ((-w / 2 + r, h - r), 90), ((-w / 2 + r, r), 180)):
        for i in range(segments + 1):
            a = _rad(start + 90 * i / segments)
            pts.append((cx + r * math.cos(a), cz + r * math.sin(a)))
    return pts


def _casing_profile(w, h, r, count=48):
    """🛡️ Intersect analytic support tangents to conservatively enclose the entire rounded body."""
    if r<=1e-12:
        return ((-w/2,0),(w/2,0),(w/2,h),(-w/2,h))
    points = []
    for i in range(count):
        a,b = 2*math.pi*i/count,2*math.pi*(i+1)/count
        nx,nz,mx,mz = math.cos(a),math.sin(a),math.cos(b),math.sin(b)
        d = abs(nx)*(w/2-r)+abs(nz)*(h/2-r)+r+nz*h/2
        e = abs(mx)*(w/2-r)+abs(mz)*(h/2-r)+r+mz*h/2
        determinant = nx*mz-nz*mx
        points.append(((d*mz-e*nz)/determinant,(nx*e-mx*d)/determinant))
    return tuple(points)


def capsule_solid(t) -> Solid:
    """💊 Nominal analysis/study envelope: base on World XY, back at y=0 and front at the nominal length."""
    w, l, h = _param(t, "width"), _param(t, "length"), _param(t, "height")
    r, seg = _param(t, "corner_radius"), _param(t, "corner_segments")
    prof = _profile(w, h, r, seg)
    n = len(prof)
    vertices = tuple((x, 0.0, z) for x, z in prof) + tuple((x, l, z) for x, z in prof)
    faces = [tuple(range(n)), tuple(range(2 * n - 1, n - 1, -1))]
    for i in range(n):
        j = (i + 1) % n
        faces.append((i, n + i, n + j, j))
    d = _param(t, "window_diameter")
    window = Circle((_param(t, "window_u"), l, _param(t, "window_v")), (0.0, 1.0, 0.0), d / 2)
    u, lo, hi = _v("capsule.bolt_u"), _v("capsule.bolt_v_low"), _v("capsule.bolt_v_high")
    bolts = ((-u, 0.0, lo), (u, 0.0, lo), (u, 0.0, hi), (-u, 0.0, hi))
    return Solid(vertices, tuple(faces), _edges(faces), window, bolts, {"profile": n, "bolt_radius": _v("capsule.bolt_radius")})


def _box(x0, x1, y0, y1, z0, z1, meta=None) -> Solid:
    vertices = ((x0, y0, z0), (x1, y0, z0), (x1, y1, z0), (x0, y1, z0), (x0, y0, z1), (x1, y0, z1), (x1, y1, z1), (x0, y1, z1))
    faces = ((0, 3, 2, 1), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7))
    return Solid(vertices, faces, _edges(faces), None, (), meta or {})


@lru_cache(maxsize=1)
def towers() -> tuple:
    """🗼 Both authored tower datums, capsule floors, separately derived clearances and roof-head proportions."""
    d = _data()
    core, cap = d["core"], d["capital"]
    pitch, podium = core["storey_height"]["value"], core["podium_storeys"]["value"]
    out = []
    for i, t in enumerate(d["towers"]):
        cs = t["capsule_storeys"]["value"]
        out.append(Tower(
            key=t["key"], name=t["name"], index=i, center=(t["center"]["x"]["value"], t["center"]["y"]["value"]),
            storeys=podium + cs, capsule_storeys=cs, first_capsule_storey=podium,
            core_side=core["side"]["value"], shaft_side=core["shaft_side"]["value"], storey_height=pitch,
            base_elevation=min(item["center"][2]["value"] for group in t["floors"][0]["groups"] for item in group["items"]),
            height=t["roof_elevation"]["value"], roof_clearance=t["roof_clearance"]["value"],
            capital_height=t["capital_height"]["value"],
            capital_overhang=cap["overhang"]["value"]))
    return tuple(out)


def _tower(tower) -> Tower:
    return tower if isinstance(tower, Tower) else towers()[tower if isinstance(tower, int) else [t.key for t in towers()].index(tower)]


def core_solid(tower) -> Solid:
    """🧱 Square core box of a tower from ground to core top; meta holds the shaft side and storey elevations."""
    t = _tower(tower)
    cx, cy = t.center
    s = t.core_side / 2
    levels = tuple(z for z in (*(_data()["podium"]["levels"][i]["value"] for i in range(2)), *(podium_height() + i * t.storey_height for i in range(t.capsule_storeys)), t.height) if z <= t.height)
    return _box(cx - s, cx + s, cy - s, cy + s, 0.0, t.height, {"shaft_side": t.shaft_side, "levels": levels})


def _capital_profile_height() -> float:
    """🏔️ Maximum of the owned native roof profile, independent of either tower’s live head parameter."""
    return max(q["value"] for q in _data()["capital"]["roof_corners"])


def capital_solid(tower) -> Solid:
    """🏭 Diagonally cut sheet-metal machinery enclosure envelope, scaled from original drawings and repository CAD."""
    t = _tower(tower)
    cx, cy = t.center
    s = t.core_side / 2 + t.capital_overhang
    z0 = capital_base(t)
    scale = t.capital_height / _capital_profile_height()
    heights = tuple(v["value"] * scale for v in _data()["capital"]["roof_corners"])
    corners = ((s, s), (s, -s), (-s, -s), (-s, s))
    vertices = tuple((cx + x, cy + y, z0) for x, y in corners) + tuple((cx + x, cy + y, z0 + h) for (x, y), h in zip(corners, heights))
    faces = ((0, 1, 2, 3), (4, 7, 6), (4, 6, 5), (0, 4, 5, 1), (1, 5, 6, 2), (2, 6, 7, 3), (3, 7, 4, 0))
    return _closed(vertices, faces, {"base_side": 2 * s, "roof_side": 2 * s, "roof_corners": heights})


def podium_solid() -> Solid:
    """🏬 Irregular podium roof-extent envelope, distinct from its recessed ground-floor construction."""
    points = tuple(tuple(v["value"] for v in p) for p in _data()["podium"]["footprint"])
    return _polygon_prism(points, 0, podium_height())


def podium_height() -> float:
    """📏 Authored flat podium roof datum, distinct from the raised perimeter coping."""
    return _data()["podium"]["levels"][-1]["value"]


def facade_storeys(tower) -> tuple:
    """🏛 Authored attachment facades per actual capsule floor; groups may contain one or two slots."""
    t = _tower(tower)
    return tuple(tuple(g["facade"] for g in f["groups"]) for f in _data()["towers"][t.index]["floors"])


def facade_items(tower, storey, facade) -> tuple:
    """🔢 Authored item indices within one facade branch, including ragged single-capsule branches."""
    t = _tower(tower)
    floor = _data()["towers"][t.index]["floors"][storey]
    group = next((g for g in floor["groups"] if g["facade"] == facade), None)
    return tuple(i["item"] for i in group["items"]) if group else ()


def slot_spec(tower, storey, facade, item) -> dict:
    """📄 Authored family, centre, orientation and door port for one independently addressable capsule."""
    t = _tower(tower)
    floor = _data()["towers"][t.index]["floors"][storey]
    group = next(g for g in floor["groups"] if g["facade"] == facade)
    return next(i for i in group["items"] if i["item"] == item)


def _type_key(tower, storey, facade, item) -> str:
    a = _data()["assignment"]
    n = (a["tower_weight"]["value"] * tower + a["storey_weight"]["value"] * storey + a["facade_weight"]["value"] * facade
         + a["item_weight"]["value"] * item + a["offset"]["value"])
    return a["order"][n % len(a["order"])]


def _facade_plane(t: Tower, facade: int, item: int, storey: int) -> Plane:
    spec = slot_spec(t, storey, facade, item)
    a = _rad(spec["rotation"]["value"])
    c, s = math.cos(a), math.sin(a)
    center = tuple(v["value"] for v in spec["center"])
    door = tuple(v["value"] for v in spec["door"])
    origin = (center[0] + c * door[0] - s * door[1], center[1] + s * door[0] + c * door[1], center[2] + .2)
    nx, ny = _data()["facades"][facade]["normal"]
    return Plane(origin, (ny, -nx, 0.0), (float(nx), float(ny), 0.0), (0.0, 0.0, 1.0))


@lru_cache(maxsize=1)
def slots() -> tuple:
    """📍 Authored compact capsule frames grouped by building/tower/storey/attachment facade; sublevel preserves three-phase staggering."""
    out = []
    length = _v("capsule.length")
    for t in towers():
        for storey, floor in enumerate(_data()["towers"][t.index]["floors"]):
            for group in floor["groups"]:
                facade = group["facade"]
                for spec in group["items"]:
                    item = spec["item"]
                    angle = spec["rotation"]["value"]
                    c, s = math.cos(_rad(angle)), math.sin(_rad(angle))
                    center = tuple(v["value"] for v in spec["center"])
                    plane = Plane((center[0] + s * length / 2, center[1] - c * length / 2, center[2]), (c, s, 0.0), (-s, c, 0.0), (0.0, 0.0, 1.0))
                    key = _type_key(t.index, storey, facade, item)
                    out.append(Slot(0, t.index, storey, facade, item, key, plane, _facade_plane(t, facade, item, storey), angle, spec["sublevel"], spec["family"], floor["floor"]))
    return tuple(out)


def tree() -> dict:
    """🌳 Aggregated tree {BUILDING;TOWER;STORY;FACADE} to each occupied one-or-two-slot attachment branch."""
    branches = {}
    for slot in slots():
        branches.setdefault(slot.path, []).append(slot)
    return {path: tuple(items) for path, items in branches.items()}


def place(solid: Solid, plane: Plane) -> Solid:
    """🧭 Map a solid from its local frame onto a plane (capsule base plane to slot plane)."""
    def at(p):
        return tuple(plane.origin[k] + p[0] * plane.x[k] + p[1] * plane.y[k] + p[2] * plane.z[k] for k in range(3))
    def vec(p):
        return tuple(p[0] * plane.x[k] + p[1] * plane.y[k] + p[2] * plane.z[k] for k in range(3))
    window = None if solid.window is None else Circle(at(solid.window.center), vec(solid.window.normal), solid.window.radius)
    x, y, z = plane.x, plane.y, plane.z
    determinant = x[0] * (y[1] * z[2] - y[2] * z[1]) - x[1] * (y[0] * z[2] - y[2] * z[0]) + x[2] * (y[0] * z[1] - y[1] * z[0])
    faces = tuple(tuple(reversed(face)) for face in solid.faces) if determinant < 0 else solid.faces
    return Solid(tuple(at(p) for p in solid.vertices), faces, solid.edges, window, tuple(at(p) for p in solid.bolts), solid.meta)


def capsule_casing(t) -> CasingDimensions:
    """📦 Physical fabrication envelope with source-bearing allowances over nominal study controls."""
    t = capsule_type(t) if isinstance(t, str) else t
    values = tuple(_v("capsule.casing." + key) + (_param(t,key) - _v("capsule." + key)) for key in ("width","length","height"))
    w, l, h = values
    centre = _param(t,"length") / 2
    return CasingDimensions(w,l,h,-w/2,w/2,centre-l/2,centre+l/2,0,h)


def capsule_parameters_from_casing(physical: dict) -> dict:
    """🎚️ Convert explicit physical casing controls once into the nominal reference parameter domain."""
    if "dimensional_basis" in physical and physical["dimensional_basis"]!="physical":
        raise ValueError("Casing controls must explicitly describe physical dimensions")
    if any(key not in physical for key in ("width","length","height")):
        raise ValueError("Physical casing controls require width, length and height")
    result={**physical,"dimensional_basis":"nominal"}
    for key in ("width","length","height"):
        result[key]=physical[key]-(_v("capsule.casing."+key)-_v("capsule."+key))
        if result[key]<=0:
            raise ValueError("Physical casing controls must leave positive nominal dimensions")
    return result


def capsule_interior(t) -> CasingDimensions:
    """🏠 Usable finished bounds with explicit interior overrides or nominal-control clearances."""
    t = capsule_type(t) if isinstance(t,str) else t
    values = []
    for key in ("width","length","height"):
        reference = _v("capsule.interior." + key)
        override = t.get("interior_"+key) if isinstance(t,dict) else getattr(t,"interior_"+key,None)
        explicit = override is not None
        values.append(override if explicit else reference + _param(t,key) - _v("capsule."+key))
    w,l,h = values
    centre,finish = _param(t,"length")/2,_detail_value("floor_finish_height")
    return CasingDimensions(w,l,h,-w/2,w/2,centre-l/2,centre+l/2,finish,finish+h)


def _capsule_quarters(t, count):
    return max(1,int(t["corner_segments"])) if isinstance(t,dict) and "corner_segments" in t else max(2,count//12)


def _interior_radius(t):
    usable=capsule_interior(t)
    radius=t.get("interior_corner_radius",_v("capsule.interior.corner_radius")) if isinstance(t,dict) else getattr(t,"interior_corner_radius",_v("capsule.interior.corner_radius"))
    if radius<.001 or radius>=min(usable.width,usable.height)/2:
        raise ValueError("Interior corner radius must be positive and fit inside the usable profile")
    return radius


def capsule_interior_profile(t, detail="full") -> tuple:
    """📐 Actual clear X/Z lining polygon, including finish elevation and the same physical quarter facets."""
    t=capsule_type(t) if isinstance(t,str) else t
    if detail not in ("full","exterior"):
        raise ValueError(f"Unknown detail level: {detail}")
    usable=capsule_interior(t)
    count=int(_detail_value("segments")) if detail=="full" else 16
    return tuple((x,z+usable.bottom) for x,z in _profile(usable.width,usable.height,_interior_radius(t),_capsule_quarters(t,count)))


def capsule_entry(t, layout="Slash") -> Plane:
    """🚪 Physical doorway frame at casing base; finished sill is the explicit floor-finish height."""
    t = capsule_type(t) if isinstance(t,str) else t
    casing,family = capsule_casing(t),_family(layout)
    coordinate = family["entry_coordinate"]["value"]
    if family["entry_wall"] == "left":
        return Plane((casing.left,coordinate,0),(0,-1,0),(1,0,0),(0,0,1))
    if family["entry_wall"] == "right":
        return Plane((casing.right,coordinate,0),(0,1,0),(-1,0,0),(0,0,1))
    return Plane((coordinate,casing.back,0),(1,0,0),(0,1,0),(0,0,1))


def capsule_window(t, layout="Slash") -> Circle:
    """🪟 Actual local glazing midplane and outward normal for a physical capsule family."""
    t = capsule_type(t) if isinstance(t, str) else t
    family = _family(layout)
    casing = capsule_casing(t)
    w, l, u, v, diameter = casing.width, _param(t,"length"), _param(t,"window_u"), _param(t,"window_v"), _param(t,"window_diameter")
    inset = _v("capsule.casing.glazing_recess_side") if family["window_wall"] == "right" else _v("capsule.casing.glazing_recess_front")
    if family["window_wall"] == "right":
        center = (w / 2 - inset, l / 2 + family["window_longitudinal_offset"]["value"] + u, v)
        normal = (1.0, 0.0, 0.0)
    else:
        center = (u, casing.front - inset, v)
        normal = (0.0, 1.0, 0.0)
    return Circle(center, normal, diameter / 2)


def slot_solid(slot: Slot) -> Solid:
    """💊 Conservative physical casing envelope, actual glazing midplane and physical-wall attachment bolts."""
    t = capsule_type(slot.type)
    casing = capsule_casing(t)
    profile = _casing_profile(casing.width,casing.height,t.corner_radius)
    n = len(profile)
    vertices = tuple((x,y,z) for y in (casing.back,casing.front) for x,z in profile)
    faces = [tuple(range(n)),tuple(range(2*n-1,n-1,-1))] + [(i,n+i,n+(i+1)%n,(i+1)%n) for i in range(n)]
    nominal = capsule_solid(t)
    envelope = Solid(vertices,tuple(faces),_edges(faces),None,nominal.bolts,{"physical_casing":True})
    family = _family(slot.layout)
    window = capsule_window(t, slot.layout)
    bolts = []
    for x, y, z in envelope.bolts:
        if family["entry_wall"] != "back":
            x = math.copysign(.4, x)
            lateral = family["entry_coordinate"]["value"]
            point = (casing.left + y, lateral - x, z) if family["entry_wall"] == "left" else (casing.right - y, lateral + x, z)
        else:
            point = (x, casing.back + y, z)
        bolts.append(point)
    meta = {**envelope.meta, "family": slot.layout, "window_reference": "glazing_midplane"}
    solid = Solid(envelope.vertices, envelope.faces, envelope.edges, window, tuple(bolts), meta)
    return place(solid, slot.plane)


def program(t) -> dict:
    """🛏 Room program of a type: interior net floor area and volume, fittings, free floor area, occupants."""
    t = capsule_type(t) if isinstance(t, str) else t
    fittings = [{"key": f["key"], "name": f["name"], "count": f["count"], "area": f["width"] * f["length"] * f["count"]} for f in t.fittings]
    net = t.interior_width * t.interior_length
    used = sum(f["area"] for f in fittings)
    return {"key": t.key, "net_floor_area": net, "volume": net * t.interior_height, "occupants": t.occupants,
            "fittings": fittings, "fitting_area": used, "free_area": net - used, "area_per_person": net / t.occupants}


def variants() -> tuple:
    """🧪 Keys of the named design variants."""
    return tuple(v["key"] for v in _data()["variants"])


def variant(name) -> dict:
    """🧪 A named design variant: key, bilingual name and description, and the overriding Values by energy key."""
    raw = next((v for v in _data()["variants"] if v["key"] == name), None)
    if raw is None:
        raise KeyError(f"Unknown variant: {name}")
    return {"key": raw["key"], "name": raw["name"], "description": raw["description"],
            "overrides": {k: Value(o["value"], o["unit"], o["basis"], o.get("source")) for k, o in raw["overrides"].items()}}


def _energy(variant_names=None) -> dict:
    e = dict(_data()["energy"])
    names = (variant_names,) if isinstance(variant_names, str) else tuple(variant_names or ())
    for name in names:
        for key, o in variant(name)["overrides"].items():
            e[key] = {**e[key], "value": o.value, "unit": o.unit, "basis": o.basis}
    return e


def envelope(t, variant=None) -> dict:
    """🧥 Envelope of a type, optionally under a design variant name or tuple of names: five exposed faces plus the back face attached to the core, window cut from the front."""
    t = capsule_type(t) if isinstance(t, str) else t
    e = _energy(variant)
    window = math.pi * t.window_diameter ** 2 / 4
    names = {
        "roof": ({"en": "Roof", "de": "Dach"}, t.width * t.length, e["u_roof"]["value"], True),
        "floor": ({"en": "Floor", "de": "Boden"}, t.width * t.length, e["u_floor"]["value"], True),
        "wall_left": ({"en": "Left side wall", "de": "Linke Seitenwand"}, t.height * t.length, e["u_wall"]["value"], True),
        "wall_right": ({"en": "Right side wall", "de": "Rechte Seitenwand"}, t.height * t.length, e["u_wall"]["value"], True),
        "front": ({"en": "Front wall", "de": "Frontwand"}, t.width * t.height - window, e["u_wall"]["value"], True),
        "window": ({"en": "Window", "de": "Fenster"}, window, e["u_window"]["value"], True),
        "back": ({"en": "Back wall to core", "de": "Rückwand zum Kern"}, t.width * t.height, e["u_wall"]["value"], False)}
    surfaces = []
    for key, (name, area, u, exposed) in names.items():
        factor = 1.0 if exposed else e["attached_factor"]["value"]
        surfaces.append({"key": key, "name": name, "area": area, "u_value": u, "exposed": exposed, "factor": factor, "ua": u * area * factor})
    pr = program(t)
    h_t = sum(s["ua"] for s in surfaces)
    h_v = e["air_heat_capacity"]["value"] * e["air_change"]["value"] * pr["volume"]
    return {"key": t.key, "variant": variant, "surfaces": surfaces, "window_area": window,
            "exposed_area": sum(s["area"] for s in surfaces if s["exposed"]), "attached_area": sum(s["area"] for s in surfaces if not s["exposed"]),
            "air_change": e["air_change"]["value"], "volume": pr["volume"], "h_t": h_t, "h_v": h_v, "h": h_t + h_v}


def climate() -> dict:
    """🌡 Climate: indoor and design temperature, heating base, monthly means with degree days, annual heating degree days."""
    d = _data()
    base = d["energy"]["heating_base"]["value"]
    months = [{"key": m["key"], "name": m["name"], "mean": m["mean"]["value"], "days": m["days"]["value"],
               "hdd": max(0.0, base - m["mean"]["value"]) * m["days"]["value"]} for m in d["climate"]["months"]]
    return {"station": d["climate"]["station"], "indoor_temperature": d["energy"]["indoor_temperature"]["value"],
            "design_temperature": d["energy"]["design_temperature"]["value"], "heating_base": base,
            "months": months, "hdd": sum(m["hdd"] for m in months),
            "source": d["climate"]["source"], "period": d["climate"]["period"], "method": d["climate"]["method"],
            "hdd_value": value("climate.heating_degree_days"), "sources": [d["sources"][d["climate"]["source"]]]}


def heating(t, variant=None) -> dict:
    """🔥 Heating of one capsule, optionally under a design variant (name or tuple of names): design load in W, annual demand in kWh/a and in kWh/(m²·a) of net floor area."""
    t = capsule_type(t) if isinstance(t, str) else t
    c, env, pr = climate(), envelope(t, variant), program(t)
    load = env["h"] * (c["indoor_temperature"] - c["design_temperature"])
    annual = env["h"] * c["hdd"] * 24 / 1000
    return {"key": t.key, "h_t": env["h_t"], "h_v": env["h_v"], "h": env["h"], "load": load, "hdd": c["hdd"],
            "annual": annual, "annual_per_area": annual / pr["net_floor_area"], "variant": variant, "net_floor_area": pr["net_floor_area"], "count": t.count}


def totals() -> dict:
    """🏙 Building totals: capsule count, floor area and annual heating demand, per type and per tower."""
    per_type = {t.key: heating(t) for t in capsule_types()}
    per_tower = {}
    for slot in slots():
        h = per_type[slot.type]
        e = per_tower.setdefault(towers()[slot.tower].key, {"count": 0, "area": 0.0, "annual": 0.0})
        e["count"] += 1
        e["area"] += h["net_floor_area"]
        e["annual"] += h["annual"]
    count = sum(h["count"] for h in per_type.values())
    area = sum(h["count"] * h["net_floor_area"] for h in per_type.values())
    annual = sum(h["count"] * h["annual"] for h in per_type.values())
    return {"count": count, "area": area, "annual": annual, "annual_per_area": annual / area, "per_type": per_type, "per_tower": per_tower}


@dataclass(frozen=True)
class NamedPart:
    """🧩 Named reconstruction component with semantic role, geometry and renderer-neutral material."""
    key: str
    role: str
    solid: Solid
    material: str


@dataclass(frozen=True)
class Assembly:
    """🏗 Immutable collection of named model components in one coordinate frame."""
    parts: tuple


def _detail_value(key):
    return _v("detail." + key)


def _closed(vertices, faces, meta=None):
    return Solid(tuple(vertices), tuple(faces), _edges(faces), meta=meta or {})


def _rings(loops, closed=True, meta=None):
    n = len(loops[0])
    vertices = tuple(p for loop in loops for p in loop)
    faces = []
    for k in range(len(loops) if closed else len(loops) - 1):
        j = (k + 1) % len(loops)
        for i in range(n):
            q = (i + 1) % n
            faces.append((k * n + i, j * n + i, j * n + q, k * n + q))
    if not closed:
        faces.extend((tuple(range(n)), tuple(range(len(vertices) - 1, len(vertices) - n - 1, -1))))
    return _closed(vertices, faces, meta)


def _circle_loop(cx, cz, radius, count):
    return tuple((cx + radius * math.cos(2 * math.pi * i / count), cz + radius * math.sin(2 * math.pi * i / count)) for i in range(count))


def _at_depth(profile, depth):
    return tuple((x, depth, z) for x, z in profile)


def _cylinder(center, radius, depth, axis="y", segments=None):
    n = segments or int(_detail_value("segments"))
    x, y, z = center
    loops = [_at_depth(_circle_loop(x, z, radius, n), y), _at_depth(_circle_loop(x, z, radius, n), y + depth)]
    solid = _rings(loops, False, {"radius": radius, "depth": depth})
    if axis == "z":
        solid = _closed(tuple((a, y + c - z, z + b - y) for a, b, c in solid.vertices), tuple(tuple(reversed(f)) for f in solid.faces), solid.meta)
    if axis == "x":
        solid = _closed(tuple((x + b - y, y + a - x, c) for a, b, c in solid.vertices), tuple(tuple(reversed(f)) for f in solid.faces), solid.meta)
    return solid


def _annulus(cx, cz, outer, inner, y0, y1, count):
    a, b = _circle_loop(cx, cz, outer, count), _circle_loop(cx, cz, inner, count)
    return _rings((_at_depth(a, y0), _at_depth(a, y1), _at_depth(b, y1), _at_depth(b, y0)), meta={"outer_radius": outer, "inner_radius": inner})


def _capsule_hardware(t, layout):
    """🔩 Native shell edge seams and recessed entry hardware, with inferred construction dimensions."""
    casing = capsule_casing(t)
    w,l,h,r = casing.width,_param(t,"length"),casing.height,_param(t,"corner_radius")
    dw = _detail_value("door_width")
    dx = 0 if _family(layout)["entry_wall"] != "back" else _family(layout)["entry_coordinate"]["value"]
    parts = []
    def add(key,role,solid,material="metal",door=False):
        parts.append(NamedPart(key,role,_layout_door(solid,layout,t) if door else solid,material))
    for i,z in enumerate((.45,1.15,1.86)):
        x = dx-dw/2+.012
        add(f"entry-hinge-{i}","door_hinge",_cylinder((x,.012,z),_v("capsule.detail.door_hinge_radius"),.085,"z",12),door=True)
        add(f"entry-hinge-leaf-{i}","door_hinge",_box(x-.02,x+.025,.015,.022,z+.01,z+.075),door=True)
        for j in range(2):
            add(f"entry-hinge-screw-{i}-{j}","door_hinge",_cylinder((x+.012,.01,z+.02+j*.045),.003,.005,segments=8),door=True)
    lockx = dx+dw/2-.085
    for key,z,radius in (("lock",.91,.021),("handle-rose",.965,.027),("viewer",1.66,.012)):
        add("entry-"+key,"door_lock",_cylinder((lockx,-.018,z),radius,.05,segments=20),door=True)
    add("entry-key-slot","door_lock",_box(lockx-.002,lockx+.002,-.019,-.018,.9,.922),"dark",True)
    add("entry-latch","door_lock",_box(dx+dw/2-.004,dx+dw/2+.004,.037,.055,.92,1.0),door=True)
    for side in (-1,1):
        for level,z in enumerate((max(r,_v("capsule.detail.seam_width")),h-max(r,_v("capsule.detail.seam_width")))):
            x = side*(w/2-_v("capsule.detail.seam_width"))
            add(f"cladding-seal-{side}-{level}","cladding_joint",_fixture_tube(((x,casing.back+r+.01,z),(x,casing.front-r-.01,z)),_v("capsule.detail.seam_width")/2),"dark")
    return tuple(parts)


def _floor_build_up(t):
    """🧱 Support the full usable floor from the real flat inner steel underside without corner protrusions."""
    c,usable = capsule_casing(t),capsule_interior(t)
    r,skin = _param(t,"corner_radius"),_detail_value("steel_skin_thickness")
    inset=max(r,skin)
    base=_box(c.left+inset,c.right-inset,c.back+inset,c.front-inset,skin,usable.bottom)
    top=((usable.left,usable.back,usable.bottom),(usable.right,usable.back,usable.bottom),(usable.right,usable.front,usable.bottom),(usable.left,usable.front,usable.bottom))
    return _positive_material(base.vertices[:4]+top,base.faces,{"support":"inner_steel_floor","finish_height":usable.bottom})


def _window_value(key):
    return _v("capsule.window_unit."+key)


def _unit_sector(cx,cz,outer,inner,y0,y1,angles):
    """🧲 Closed annular guide material on explicit shared angular contact facets."""
    profile=tuple((cx+outer*math.cos(a),cz+outer*math.sin(a)) for a in angles)+tuple((cx+inner*math.cos(a),cz+inner*math.sin(a)) for a in reversed(angles))
    solid=_rings((_at_depth(profile,y0),_at_depth(profile,y1)),False)
    return _positive_material(solid.vertices,solid.faces)


def _unit_torus(cx,cz,radius,tube,depth,angles,sections):
    """🛞 A real closed circular guide rod with planar triangular surface facets."""
    vertices=tuple((cx+(radius+tube*math.cos(2*math.pi*j/sections))*math.cos(a),depth+tube*math.sin(2*math.pi*j/sections),cz+(radius+tube*math.cos(2*math.pi*j/sections))*math.sin(a)) for a in angles for j in range(sections+1))
    faces=[]
    stride=sections+1
    for i in range(len(angles)-1):
        for j in range(sections):
            a=i*stride+j
            faces.extend(((a,a+stride,a+stride+1),(a,a+stride+1,a+1)))
    return _positive_material(vertices,tuple(faces))


def _unit_angle(start,end,leg,thickness,side=1):
    """📏 A finite bent-sheet L section, not a solid square bar."""
    delta=tuple(b-a for a,b in zip(start,end))
    length=math.sqrt(sum(v*v for v in delta))
    axis=tuple(v/length for v in delta)
    normal=(0,float(side),0)
    tangent=(normal[1]*axis[2],normal[2]*axis[0]-normal[0]*axis[2],-normal[1]*axis[0])
    profile=((0,0),(leg,0),(leg,thickness),(thickness,thickness),(thickness,leg),(0,leg))
    solid=_polygon_prism(profile,0,length)
    return place(_positive_material(solid.vertices,solid.faces),Plane(start,tangent,normal,axis))


def _unit_chord(radius,count,height,sign):
    """🧭 Exact horizontal intersection of the actual faceted annulus, not its analytic circle."""
    points=_circle_loop(0,0,radius,count)
    hits=[]
    for a,b in zip(points,points[1:]+points[:1]):
        if min(a[1],b[1])-1e-12<=height<=max(a[1],b[1])+1e-12 and abs(b[1]-a[1])>1e-12:
            hits.append(a[0]+(height-a[1])*(b[0]-a[0])/(b[1]-a[1]))
    if not hits:
        raise ValueError("Hinge must meet the actual sash frame height")
    return max(hits) if sign>0 else min(hits)


def _paper_triangle(points,thickness):
    """📄 Finite paper thickness follows the actual folded facet normal."""
    a,b,c=points
    ab,ac=tuple(b[k]-a[k] for k in range(3)),tuple(c[k]-a[k] for k in range(3))
    normal=(ab[1]*ac[2]-ab[2]*ac[1],ab[2]*ac[0]-ab[0]*ac[2],ab[0]*ac[1]-ab[1]*ac[0])
    magnitude=math.sqrt(sum(v*v for v in normal))
    normal=tuple(v/magnitude*thickness/2 for v in normal)
    vertices=tuple(tuple(p[k]+side*normal[k] for k in range(3)) for side in (-1,1) for p in points)
    return _positive_material(vertices,((0,2,1),(3,4,5),(0,1,4,3),(1,2,5,4),(2,0,3,5)))


def capsule_window_unit(t, layout="Slash", state=None, detail="full") -> Assembly:
    """🪟 Primary fixed/fan hardware plus the observed A904 inner-sash reference; all depths are physical."""
    t=capsule_type(t) if isinstance(t,str) else t
    if detail not in ("full","exterior"):
        raise ValueError(f"Unknown detail level: {detail}")
    keys=("sash_angle","blind_fraction","blind_rotation")
    if state is None:
        values={key:t.get("window_"+key,_window_value("default_"+key)) if isinstance(t,dict) else _window_value("default_"+key) for key in keys}
    else:
        values={key:getattr(state,key) for key in keys} if isinstance(state,WindowUnitState) else dict(state)
    if set(values)!=set(keys):
        raise ValueError("Window state requires sash angle, fan coverage and rotation")
    values={key:float(value) for key,value in values.items()}
    if not all(math.isfinite(value) for value in values.values()) or not 0<=values["sash_angle"]<=_window_value("sash_limit") or not 0<=values["blind_fraction"]<=1:
        raise ValueError("Window state requires a valid sash angle, fan coverage and rotation")
    c,inside,family=capsule_casing(t),capsule_interior(t),_family(layout)
    u,v,radius=_param(t,"window_u"),_param(t,"window_v"),_param(t,"window_diameter")/2
    side=family["window_wall"]=="right"
    skin,fixed,lining=_detail_value("steel_skin_thickness"),_detail_value("glazing_thickness"),_detail_value("lining_thickness")
    reveal=_v("capsule.casing.reveal_depth")
    recess=_v("capsule.casing.glazing_recess_side") if side else _v("capsule.casing.glazing_recess_front")
    fixed_y=c.front-recess
    inner_face=c.front-(c.right-inside.right) if side else inside.front
    inner_y=inner_face-_window_value("inner_glass_inset")
    inner_thickness=_window_value("inner_glass_thickness")
    fan_y=(fixed_y-fixed/2+inner_y+inner_thickness/2)/2
    n=int(_detail_value("segments")) if detail=="full" else 16
    count=int(_window_value("fold_count")) if detail=="full" else int(_window_value("exterior_fold_count"))
    parts=[]
    def add(key,role,solid,material="metal",moving=False):
        if moving:
            sign=-1 if side else 1
            pivot=u+sign*radius
            angle=_rad(sign*values["sash_angle"])
            co,si=math.cos(angle),math.sin(angle)
            solid=_closed(tuple((pivot+co*(x-pivot)-si*(y-inner_y),inner_y+si*(x-pivot)+co*(y-inner_y),z) for x,y,z in solid.vertices),solid.faces,solid.meta)
        solid=_layout_window(solid,layout,t)
        meta={**solid.meta,"window_unit":True,"window_component":role,"window_state":dict(values),"reference":"observed-a904" if moving or role in ("window_hinge","window_lock","window_stop","window_mount_frame") else "detail-1972"}
        parts.append(NamedPart(key,role,_closed(solid.vertices,solid.faces,meta),material))
    def ring(key,role,outer,inner,y0,y1,material="metal",moving=False):
        add(key,role,_annulus(u,v,outer,inner,y0,y1,n),material,moving)
    ring("window-rim","window_rim",radius+skin,radius,c.front-skin,c.front)
    ring("window-frame","window_frame",radius+skin,radius,c.front-reveal,c.front-skin)
    add("window-glazing","glazing",_cylinder((u,fixed_y-fixed/2,v),radius,fixed,segments=n),"glass")
    sheet=_window_value("closure_sheet")
    trim=_detail_value("frame_width")
    rear=inner_face+lining
    ring("window-reveal-collar","window_closure",radius+trim,radius+trim-sheet,inner_face,rear)
    ring("window-reveal-flange","window_closure",radius+trim+_window_value("angle_side"),radius+trim-sheet,rear,rear+sheet)
    closure=_rings((_at_depth(_circle_loop(u,v,radius+trim,n),rear+sheet),_at_depth(_circle_loop(u,v,radius+sheet,n),c.front-reveal),_at_depth(_circle_loop(u,v,radius,n),c.front-reveal),_at_depth(_circle_loop(u,v,radius+trim-sheet,n),rear+sheet)))
    add("window-reveal-closure","window_closure",_positive_material(closure.vertices,closure.faces))
    back=fixed_y-fixed/2
    outer=radius+(radius+trim-sheet-radius)*(c.front-reveal-back)/(c.front-reveal-rear-sheet)
    gasket=_rings((_at_depth(_circle_loop(u,v,outer,n),back),_at_depth(_circle_loop(u,v,radius,n),c.front-reveal),_at_depth(_circle_loop(u,v,radius,n),back)))
    add("window-gasket","window_gasket",_positive_material(gasket.vertices,gasket.faces),"rubber")
    mount_outer=radius+trim+.01
    ring("window-inner-mount","window_mount_frame",mount_outer,radius,inner_face-sheet,inner_face)
    for i in range(4):
        angle=i*math.pi/2
        radial=(math.cos(angle),0,math.sin(angle))
        tangent=(-math.sin(angle),0,math.cos(angle))
        length,leg,thick=_window_value("angle_length"),_window_value("angle_side"),_window_value("angle_thickness")
        origin=(u+(radius+trim)*radial[0],rear+sheet,v+(radius+trim)*radial[2])
        profile=((0,0),(leg,0),(leg,thick),(thick,thick),(thick,leg),(0,leg))
        solid=_polygon_prism(profile,-length/2,length/2)
        add(f"window-angle-{i}","window_angle_support",place(_positive_material(solid.vertices,solid.faces),Plane(origin,radial,(0,1,0),tangent)))
    base_r,cover_r=_window_value("base_diameter")/2,_window_value("cover_diameter")/2
    sleeve_r=_window_value("sleeve_diameter")/2
    base_thick,divider=_window_value("base_thickness"),_window_value("divider_thickness")
    guide_r,guide_w,guide_depth=_window_value("inner_guide_radius"),_window_value("inner_guide_width"),_window_value("inner_guide_depth")
    add("window-base-plate","window_base_plate",_cylinder((u,back-base_thick,v),base_r,base_thick,segments=n),"metal")
    ring("window-divider","window_divider",base_r,sleeve_r,fan_y-guide_depth/2-divider,fan_y-guide_depth/2)
    cover_y=fan_y-guide_depth/2-divider-_window_value("cover_thickness")
    ring("window-centre-cover","window_cover",cover_r,sleeve_r,cover_y,cover_y+_window_value("cover_thickness"))
    ring("window-threaded-sleeve","window_sleeve",sleeve_r,sleeve_r-_window_value("sleeve_wall"),fan_y-guide_depth/2-divider,back-base_thick)
    add("window-hub-cap","window_cover",_cylinder((u,cover_y-.002,v),_window_value("hub_fastener_radius"),.002,segments=n))
    paper_r=radius-_window_value("paper_margin")
    hub_r=base_r+_window_value("hub_clearance")
    span=min(2*math.pi-.02,max(_rad(_window_value("stowed_span")),2*math.pi*values["blind_fraction"]))
    centre=-math.pi/2+_rad(math.fmod(values["blind_rotation"],360))
    leading=(centre-span/2,centre+span/2)
    clip_half=_rad(_window_value("clip_span"))/2
    angles=sorted(set([2*math.pi*i/n for i in range(n+1)]+[math.pi]+[(a+d)%(2*math.pi) for a in leading for d in (-clip_half,0,clip_half)]))
    if angles[-1]<2*math.pi:
        angles.append(2*math.pi)
    for i,(lo,hi) in enumerate(((0,math.pi),(math.pi,2*math.pi))):
        arc=[a for a in angles if lo-1e-10<=a<=hi+1e-10]
        add(f"window-inner-guide-{i}","window_inner_guide",_unit_sector(u,v,guide_r+guide_w,guide_r-guide_w,fan_y-guide_depth/2,fan_y+guide_depth/2,arc),"brass")
    outer_r=radius-_window_value("guide_inset")
    tube=_window_value("guide_radius")
    add("window-outer-guide","window_outer_guide",_unit_torus(u,v,outer_r,tube,fan_y,angles,8))
    for i,a in enumerate(leading):
        arc=sorted(set([a-clip_half,a,a+clip_half]+[q+k*2*math.pi for q in angles for k in (-1,0,1) if a-clip_half<q+k*2*math.pi<a+clip_half]))
        grids=[]
        for cross_radius in (tube+_window_value("guide_clip_thickness"),tube):
            grids.append(tuple((u+(outer_r+cross_radius*math.cos(-math.pi/2+j*math.pi/4))*math.cos(theta),fan_y+cross_radius*math.sin(-math.pi/2+j*math.pi/4),v+(outer_r+cross_radius*math.cos(-math.pi/2+j*math.pi/4))*math.sin(theta)) for theta in arc for j in range(5)))
        add(f"window-guide-clip-{i}","window_guide_clip",_skin_patch(grids[0],grids[1],len(arc)-1,4))
        inner_clip=_unit_sector(u,v,guide_r+guide_w+_window_value("guide_clip_thickness"),guide_r+guide_w,fan_y-guide_depth/2,fan_y+guide_depth/2,arc)
        add(f"window-inner-clip-{i}","window_carrier",inner_clip)
        start=(u+(guide_r+guide_w+.001)*math.cos(a),fan_y,v+(guide_r+guide_w+.001)*math.sin(a))
        end=(u+(outer_r+tube+.001)*math.cos(a),fan_y,v+(outer_r+tube+.001)*math.sin(a))
        add(f"window-carrier-{i}","window_carrier",_unit_angle(start,end,_window_value("carrier_leg"),_window_value("carrier_sheet"),-1 if i==0 else 1))
    for i in range(4):
        a=i*math.pi/2
        start=(u+(radius+trim/2)*math.cos(a),inner_face,v+(radius+trim/2)*math.sin(a))
        end=(u+(outer_r+tube)*math.cos(a),fan_y,v+(outer_r+tube)*math.sin(a))
        add(f"window-guide-fixing-{i}","window_guide_fixing",_beam(start,end,_window_value("guide_fixing_side")))
    actual=int(_window_value("fold_count"))
    rest=2*paper_r*math.sin(math.pi/actual)
    projected=2*paper_r*math.sin(span/2/actual)
    amplitude=math.sqrt(max(0,rest*rest-projected*projected))/2
    paper_vertices,paper_faces=[],[]
    for i in range(count):
        points=[]
        for index,r in ((i,hub_r),(i+1,hub_r),(i+1,paper_r),(i,paper_r)):
            a=leading[0]+span*index/count
            depth=0 if index in (0,count) else (-1 if index%2 else 1)*amplitude*(r-hub_r)/(paper_r-hub_r)
            points.append((u+r*math.cos(a),fan_y+depth,v+r*math.sin(a)))
        for tri in ((points[0],points[1],points[2]),(points[0],points[2],points[3])):
            solid=_paper_triangle(tri,_window_value("paper_thickness"))
            offset=len(paper_vertices)
            paper_vertices.extend(solid.vertices)
            paper_faces.extend(tuple(offset+j for j in face) for face in solid.faces)
    add("window-paper-fan","window_blind",_closed(tuple(paper_vertices),tuple(paper_faces),{"folds":count,"radial":True}),"paper")
    sash_r=radius-_window_value("sash_clearance")
    clear_r=sash_r-_window_value("sash_radial_width")
    wall,depth=_window_value("sash_wall"),_window_value("sash_depth")
    ring("window-sash-frame","window_sash_frame",sash_r,sash_r-wall,inner_y-depth/2,inner_y+depth/2,moving=True)
    ring("window-sash-front-flange","window_sash_frame",sash_r-wall,clear_r,inner_y-depth/2,inner_y-depth/2+wall,moving=True)
    for i,sign in enumerate((-1,1)):
        glass_face=inner_y+sign*inner_thickness/2
        seal_face=glass_face+sign*_window_value("sash_gasket_thickness")
        seat_face=seal_face+sign*wall
        ring(f"window-sash-seat-{i}","window_sash_frame",sash_r-wall,clear_r,min(seal_face,seat_face),max(seal_face,seat_face),moving=True)
        ring(f"window-sash-seal-{i}","window_gasket",clear_r+_window_value("sash_rebate"),clear_r,min(glass_face,seal_face),max(glass_face,seal_face),"rubber",True)
    add("window-sash-glazing","window_sash_glazing",_cylinder((u,inner_y-inner_thickness/2,v),clear_r+_window_value("sash_rebate"),inner_thickness,segments=n),"glass",True)
    hinge_sign=-1 if side else 1
    pivot=u+hinge_sign*radius
    hinge_r,pin_r,clearance=_window_value("hinge_radius"),_window_value("hinge_pin_radius"),_window_value("hinge_clearance")
    half=_window_value("hinge_span")/2
    for i,sign in enumerate((-1,1)):
        z=v+sign*_window_value("hinge_spacing")/2
        add(f"window-hinge-{i}-pin","window_hinge",_cylinder((pivot,inner_y,z-half-.002),pin_r,_window_value("hinge_span")+.004,"z",16))
        for j,(lo,hi) in enumerate(((z-half,z-half/3),(z+half/3,z+half))):
            ann=_annulus(pivot,inner_y,hinge_r,pin_r+clearance,lo,hi,16)
            add(f"window-hinge-{i}-fixed-{j}","window_hinge",_closed(tuple((x,c,b) for x,b,c in ann.vertices),tuple(tuple(reversed(f)) for f in ann.faces)))
        ann=_annulus(pivot,inner_y,hinge_r,pin_r+clearance,z-half/3,z+half/3,16)
        add(f"window-hinge-{i}-moving","window_hinge",_closed(tuple((x,c,b) for x,b,c in ann.vertices),tuple(tuple(reversed(f)) for f in ann.faces)),moving=True)
        for j,z0 in enumerate((z-half-.002,z+half)):
            add(f"window-hinge-{i}-collar-{j}","window_hinge",_cylinder((pivot,inner_y,z0),hinge_r,.002,"z",16))
        x=u+(_unit_chord(sash_r,n,z-v,hinge_sign)+_unit_chord(sash_r-wall,n,z-v,hinge_sign))/2
        add(f"window-hinge-{i}-leaf","window_hinge",_beam((x,inner_y,z),(pivot,inner_y,z),wall),moving=True)
        add(f"window-hinge-{i}-mount","window_hinge",_box(pivot-.01,pivot+.01,min(inner_face,inner_y+hinge_r),max(inner_face,inner_y+hinge_r),z-.012,z+.012))
    for key,sign in (("upper",1),("lower",-1)):
        x=u-hinge_sign*.8*(sash_r-_window_value("sash_radial_width")/2)
        z=v+sign*.6*(sash_r-_window_value("sash_radial_width")/2)
        add("window-lock-"+key,"window_lock",_cylinder((x,inner_y-depth/2-.008,z),_window_value("lock_radius"),.008,segments=12),moving=True)
        add("window-lock-lever-"+key,"window_lock",_beam((x,inner_y-depth/2-.004,z),(x+hinge_sign*_window_value("lock_length"),inner_y-depth/2-.025,z),wall*2),moving=True)
    stop_z=v+_window_value("hinge_spacing")/2
    x0=pivot+hinge_sign*.001
    add("window-sash-stop","window_stop",_box(min(x0,x0+hinge_sign*.005),max(x0,x0+hinge_sign*.005),inner_y-.045,inner_y-.035,stop_z-.006,stop_z+.006))
    add("window-stop-mount","window_stop",_beam((pivot+hinge_sign*.03,inner_face,stop_z),(pivot+hinge_sign*.03,inner_y-.04,stop_z),wall*2))
    add("window-stop-return","window_stop",_beam((pivot+hinge_sign*.03,inner_y-.04,stop_z),(x0,inner_y-.04,stop_z),wall*2))
    return Assembly(tuple(parts))


def _long_wall_luminaire(t, layout, reference):
    """💡 Source-visible elongated cabinet-wall light; precise housing dimensions are inferred."""
    usable=capsule_interior(t)
    cabinet=next(p.solid for p in reference if p.key=="media-wall")
    low=tuple(min(p[k] for p in cabinet.vertices) for k in range(3))
    high=tuple(max(p[k] for p in cabinet.vertices) for k in range(3))
    sign=-1 if (low[0]+high[0])/2>0 else 1
    face=low[0] if sign<0 else high[0]
    sx,sy,sz=usable.width/2.3,usable.length/3.8,usable.height/2.1
    length,depth,height=_v("capsule.luminaire.reference_length")*sy,_v("capsule.luminaire.depth")*sx,_v("capsule.luminaire.height")*sz
    thickness=_v("capsule.luminaire.housing_thickness")
    grille=tuple(v for p in reference if p.key.startswith("vent-grille-") for v in p.solid.vertices)
    middle=(min(v[1] for v in grille)+max(v[1] for v in grille))/2
    y0,y1=middle-length/2,middle+length/2
    centre=usable.bottom+_v("capsule.luminaire.mount_height")*sz
    z0,z1=centre-height/2,centre+height/2
    parts=[]
    def box(key,bounds,role="wall_luminaire",material="metal"):
        solid=_box(*bounds)
        parts.append(NamedPart("long-wall-luminaire-"+key,role,_closed(solid.vertices,solid.faces,{"reference_device":"long-wall-luminaire","basis":"assumed","source":"domus"}),material))
    def interval(a,b):
        return min(a,b),max(a,b)
    x0,x1=interval(face,face+sign*depth)
    a,b=interval(face,face+sign*thickness)
    box("housing",(a,b,y0,y1,z0,z1))
    for key,bounds in (("top",(x0,x1,y0,y1,z1-thickness,z1)),("bottom",(x0,x1,y0,y1,z0,z0+thickness)),("end-a",(x0,x1,y0,y0+thickness,z0+thickness,z1-thickness)),("end-b",(x0,x1,y1-thickness,y1,z0+thickness,z1-thickness))):
        box(key,bounds)
    a,b=interval(face+sign*(depth-_v("capsule.luminaire.diffuser_thickness")),face+sign*depth)
    box("diffuser",(a,b,y0+thickness,y1-thickness,z0+thickness,z1-thickness),"luminaire_diffuser","diffuser")
    tube=_cylinder((face+sign*depth/2,y0+thickness,centre),_v("capsule.luminaire.tube_radius"),length-2*thickness,segments=16)
    parts.append(NamedPart("long-wall-luminaire-tube","luminaire_tube",_closed(tube.vertices,tube.faces,{**tube.meta,"reference_device":"long-wall-luminaire","basis":"assumed","source":"domus"}),"diffuser"))
    return tuple(parts)


def capsule_assembly(t, detail="full", layout="Slash", interior="reference") -> Assembly:
    """💊 Detailed reconstruction: a hollow shell with an actual porthole, hardware and inferred interior fittings; detail='exterior' omits furnishings."""
    t = capsule_type(t) if isinstance(t, str) else t
    casing = capsule_casing(t)
    usable = capsule_interior(t)
    w,l,h,r = casing.width,_param(t,"length"),casing.height,_param(t,"corner_radius")
    u, v, radius = _param(t, "window_u"), _param(t, "window_v"), _param(t, "window_diameter") / 2
    th, fw, fd, glass = (_detail_value(k) for k in ("shell_thickness", "frame_width", "frame_depth", "glazing_thickness"))
    if detail not in ("full", "exterior"):
        raise ValueError(f"Unknown detail level: {detail}")
    n = int(_detail_value("segments")) if detail == "full" else 16
    skin = _detail_value("steel_skin_thickness")
    if interior not in ("reference", "study"):
        raise ValueError(f"Unknown interior mode: {interior}")
    if layout not in capsule_families():
        raise ValueError(f"Unknown capsule family: {layout}")
    parts = _shell_parts(t, layout, 0, skin, n, "shell", "shell")
    def add(key, role, solid, material="shell"):
        parts.append(NamedPart(key, role, solid, material))
    def box(key, role, bounds, material="shell"):
        add(key, role, _box(*bounds), material)
    parts.extend(capsule_window_unit(t,layout,detail=detail).parts)
    dw, dh = (_detail_value(k) for k in ("door_width", "door_height"))
    finish = _detail_value("floor_finish_height")
    dx = 0 if _family(layout)["entry_wall"] != "back" else _family(layout)["entry_coordinate"]["value"]
    door_start = len(parts)
    box("entry-door", "door", (dx - dw / 2, dx + dw / 2, .03, .07, finish, dh + finish), "metal")
    for key, bounds in (("left", (dx - dw / 2 - .035, dx - dw / 2, -.015, .1, finish, dh + finish + .035)), ("right", (dx + dw / 2, dx + dw / 2 + .035, -.015, .1, finish, dh + finish + .035)), ("head", (dx - dw / 2, dx + dw / 2, -.015, .1, dh + finish, dh + finish + .035))):
        box("entry-frame-" + key, "door_frame", bounds, "metal")
    add("entry-handle", "door_handle", _fixture_tube(((dx+dw/2-.085,.015,.965),(dx+dw/2-.085,-.035,.965),(dx+dw/2-.16,-.035,.965)),.007), "metal")
    parts[door_start:] = [NamedPart(p.key, p.role, _layout_door(p.solid, layout, t), p.material) for p in parts[door_start:]]
    parts.extend(_capsule_hardware(t, layout))
    for i, (x, y, z) in enumerate(capsule_solid(t).bolts):
        x = math.copysign(.4, x) if layout in ("J", "L", "S") else x
        add(f"bolt-{i}-shaft", "bolt_shaft", _layout_door(_cylinder((x, -_detail_value("bolt_length"), z), _v("capsule.bolt_radius"), _detail_value("bolt_length") + .035), layout, t), "metal")
        add(f"bolt-{i}-head", "bolt_head", _layout_door(_cylinder((x, -.035, z), _detail_value("bolt_head_radius"), .035, segments=6), layout, t), "metal")
    for i, x in enumerate((-.65, .65)):
        box(f"service-rail-{i}", "service_rail", (x - .025, x + .025, .12, l - .12, .035, .085), "metal")
        add(f"service-pipe-{i}", "service_pipe", _cylinder((x, casing.back+max(r,skin), .04), _detail_value("service_radius"), casing.length-2*max(r,skin)), "metal")
    depth = usable.back-casing.back if _family(layout)["entry_wall"] == "back" else (casing.width-usable.width)/2
    threshold = _box(dx - dw / 2, dx + dw / 2, 0, depth + .02, .02, finish)
    add("entry-threshold", "entry_threshold", _layout_door(threshold, layout, t), "metal")
    if detail == "full":
        add("floor-finish","floor_finish",_floor_build_up(t),"fabric")
        lining = _detail_value("lining_thickness")
        parts.extend(_shell_parts(t, layout, th - lining, lining, n, "lining", "shell"))
        member = _detail_value("truss_member_side")
        cover=max(r/2,member/2+skin)
        end_cover=max(r,member/2+skin)
        for side, x in enumerate((-w/2+cover,w/2-cover)):
            for level, z in enumerate((cover,h-cover)):
                add(f"truss-longitudinal-{side}-{level}", "steel_truss", _beam((x, casing.back+end_cover, z), (x, casing.front-end_cover, z), member), "metal")
            for end, y in enumerate((casing.back+end_cover, casing.front-end_cover)):
                add(f"truss-upright-{side}-{end}", "steel_truss", _beam((x,y,cover),(x,y,h-cover), member), "metal")
            opening = _family(layout)["entry_wall"] == ("left" if side == 0 else "right") or _family(layout)["window_wall"] == "right" and side == 1
            if not opening:
                for diagonal, ends in enumerate((((x,casing.back+end_cover,cover),(x,casing.front-end_cover,h-cover)), ((x,casing.back+end_cover,h-cover),(x,casing.front-end_cover,cover)))):
                    add(f"truss-diagonal-{side}-{diagonal}", "steel_truss", _beam(*ends, member), "metal")
        for end, y in enumerate((casing.back+end_cover, casing.front-end_cover)):
            for level, z in enumerate((cover,h-cover)):
                add(f"truss-cross-{end}-{level}", "steel_truss", _beam((-w/2+cover,y,z),(w/2-cover,y,z), member), "metal")
        if interior == "reference":
            reference=_reference_interior(t,layout)
            parts.extend(reference)
            parts.extend(_long_wall_luminaire(t,layout,reference))
            return Assembly(tuple(parts))
        parts.extend(_study_interior(t, layout))
    return Assembly(tuple(parts))


def place_assembly(assembly: Assembly, plane: Plane) -> Assembly:
    """🧭 Place every semantic component with the same orthonormal transformation."""
    return Assembly(tuple(NamedPart(p.key, p.role, place(p.solid, plane), p.material) for p in assembly.parts))


def core_assembly(tower, detail="full", cancelled=None) -> Assembly:
    """🗼 Reconstructed core (``detail="exterior"`` omits the interior stairs, lift compound and risers that no exterior view shows) with local staggered landings, ground/office floors, turning stairs and enclosed central lift; detailed dimensions are assumptions."""
    t = _tower(tower)
    cx, cy = t.center
    s, wall, slab = t.core_side / 2, _detail_value("wall_thickness"), _detail_value("floor_thickness")
    parts = []
    def box(key, role, bounds, material="concrete"):
        if role == "lift_shaft":
            x0, x1, y0, y1, _, _ = bounds
            for level in range(t.storeys):
                parts.append(NamedPart(f"{key}-{level}", role, _box(x0, x1, y0, y1, level * t.height / t.storeys, (level + 1) * t.height / t.storeys), material))
        else:
            parts.append(NamedPart(key, role, _box(*bounds), material))
    dw, dh, dx = (_detail_value(k) for k in ("door_width", "door_height", "door_offset"))
    for facade in range(4):
        _check_cancel(cancelled)
        nx, ny = _data()["facades"][facade]["normal"]
        axes = Plane((cx + nx * s, cy + ny * s, 0), (ny, -nx, 0), (nx, ny, 0), (0, 0, 1))
        openings = []
        ground = _data()["core"]["ground_entry"]
        if facade == ground["facade"]:
            u, width, head = (ground[k]["value"] for k in ("lateral", "width", "height"))
            openings.append((u - width / 2, u + width / 2, 0, head))
            threshold = _box(u - width / 2, u + width / 2, -wall - .02, .1, -slab, 0, {"walking_axis": "y"})
            parts.append(NamedPart("ground-threshold", "ground_threshold", place(threshold, axes), "concrete"))
            office = _data()["podium"]["levels"][1]["value"]
            openings.append((u - width / 2, u + width / 2, office, office + head))
            threshold = _box(u - width / 2, u + width / 2, -wall - .02, .1, office - slab, office, {"walking_axis": "y"})
            parts.append(NamedPart("office-threshold", "office_threshold", place(threshold, axes), "concrete"))
        for slot in slots():
            if slot.tower == t.index and slot.facade == facade and slot.plane.origin[2] + .2 < t.height:
                u = sum((slot.facade_plane.origin[k] - axes.origin[k]) * axes.x[k] for k in range(3))
                z = slot.plane.origin[2]
                openings.append((u - dw / 2, u + dw / 2, z + .2, z + dh + .2))
                landing = _box(u - dw / 2 - .12, u + dw / 2 + .12, -_detail_value("capsule_landing_depth"), 0, z + .02, z + .2, {"walking_axis": "y" if facade in (0, 2) else "x"})
                parts.append(NamedPart(f"capsule-landing-{slot.storey}-{facade}-{slot.item}", "landing", place(landing, axes), "concrete"))
        bridge_facade = 1 if t.key == "A" else 3
        if facade == bridge_facade:
            for level in _data()["detail"]["bridge_storeys"]:
                z = bridge_elevation(level)
                if z >= t.height:
                    continue
                lateral = -_v("detail.bridge_geometry.lateral_offset") if facade == 1 else _v("detail.bridge_geometry.lateral_offset")
                openings.append((lateral - _detail_value("bridge_width") / 2, lateral + _detail_value("bridge_width") / 2, z, z + 2.1))
                width = _detail_value("bridge_width")
                passage = _v("detail.bridge_geometry.passage_width")
                portal_y = max(_v("detail.bridge_geometry.lateral_offset"), -(s - wall - _v("core.profile.pier_leg")) + passage / 2 + _v("detail.bridge_geometry.portal_clearance"))
                route = ((cx + nx * (s - wall - .35), cy + portal_y), (cx + nx * (s + .02), cy + portal_y), (cx + nx * (s + .02 + _v("detail.bridge_geometry.portal_transition_length")), cy + _v("detail.bridge_geometry.lateral_offset")))
                approach = _box(lateral - width / 2, lateral + width / 2, -wall - .35, .02, z - slab, z, {"walking_axis": "x", "walking_route": route})
                parts.append(NamedPart(f"bridge-landing-{level}", "bridge_approach", place(approach, axes), "concrete"))
        cuts = sorted({-s, s, *(max(-s, min(s, x)) for opening in openings for x in opening[:2])})
        cell = 0
        for x0, x1 in zip(cuts, cuts[1:]):
            if x1 - x0 < 1e-8:
                continue
            middle = (x0 + x1) / 2
            intervals = sorted((max(0, z0), min(t.height, z1)) for a, b, z0, z1 in openings if a < middle < b and z0 < t.height and z1 > 0)
            cursor = 0
            for z0, z1 in [*intervals, (t.height, t.height)]:
                if z0 - cursor > 1e-8:
                    levels = [cursor, *(i * t.storey_height for i in range(1, t.storeys + 1) if cursor < i * t.storey_height < z0), z0]
                    for bottom, top in zip(levels, levels[1:]):
                        solid = place(_box(x0, x1, -wall, 0, bottom, top), axes)
                        parts.append(NamedPart(f"wall-{facade}-{cell}", "core_wall", solid, "concrete"))
                        cell += 1
                cursor = max(cursor, z1)
    leg = _v("core.profile.pier_leg")
    inside = s - wall
    for index, (sx, sy) in enumerate(((1, 1), (-1, 1), (-1, -1), (1, -1))):
        points = ((sx * inside, sy * inside), (sx * (inside - leg), sy * inside), (sx * inside, sy * (inside - leg)))
        parts.append(NamedPart(f"corner-pier-{index}", "core_corner_pier", place(_polygon_prism(points, 0, t.height), _core_frame(t)), "concrete"))
    if detail != "exterior":
        parts.extend(_circulation_parts(t, detail, cancelled))
        for j, y in enumerate((-.45, .45)):
            for level in range(t.storeys):
                bottom = level * t.height / t.storeys
                top = (level + 1) * t.height / t.storeys
                solid = _cylinder((-.79, y, bottom), _detail_value("core_service_radius"), top - bottom, "z", 12)
                parts.append(NamedPart(f"service-riser-{j}-{level}", "service_riser", place(solid, _core_frame(t)), "metal"))
    opening = _v("core.circulation.flight_inner") + _v("core.circulation.flight_width")
    roof_bounds = ((cx - s + wall, cx - opening, cy - s + wall, cy + s - wall), (cx + opening, cx + s - wall, cy - s + wall, cy + s - wall), (cx - opening, cx + opening, cy - s + wall, cy - opening), (cx - opening, cx + opening, cy + opening, cy + s - wall))
    for i, bounds in enumerate(roof_bounds):
        box(f"roof-slab-{i}", "roof", (*bounds, t.height - slab, t.height))
    shaft_x = _v("core.profile.compound_width") / 2
    shaft_y = _v("core.profile.compound_depth") / 2
    vx, vy = _detail_value("roof_vent_aperture_width") / 2, _detail_value("roof_vent_aperture_length") / 2
    curbs = ((-shaft_x, -vx, -shaft_y, shaft_y), (vx, shaft_x, -shaft_y, shaft_y), (-vx, vx, -shaft_y, -vy), (-vx, vx, vy, shaft_y))
    for i, bounds in enumerate(curbs):
        a, b, c, d = bounds
        box(f"roof-vent-curb-{i}", "vent_support", (cx + a, cx + b, cy + c, cy + d, t.height, t.height + slab), "metal")
    box("roof-vent", "vent", (cx - .5, cx + .5, cy - .4, cy + .4, t.height + slab, t.height + .7), "metal")
    return Assembly(tuple(parts))


def _capital_cladding(t, envelope):
    """🧷 Panelized sheet-metal shell follows the actual two-plane roof; seam dimensions are inferred from Domus imagery."""
    data = _data()["capital"]["cladding"]
    value = lambda key: data[key]["value"]
    pitch, sheet, joint = (value(k) for k in ("panel_pitch", "sheet_thickness", "horizontal_pitch"))
    seam, projection, flashing, flash_projection = (value(k) for k in ("seam_width", "seam_projection", "flashing_width", "flashing_projection"))
    base, half = capital_base(t), t.core_side / 2 + t.capital_overhang
    parts = []
    def add(key, role, points, normal, origin, surface, offset=0):
        shifted = tuple(tuple(point[k] + normal[k] * offset for k in range(3)) for point in points)
        solid = _face_slab(shifted, sheet + offset)
        solid = _closed(solid.vertices, solid.faces, {"surface": surface, "normal": normal, "origin": origin, "projection": offset, "source": data["source"]})
        parts.append(NamedPart(key, role, solid, "dark" if role == "capital" else "metal"))
    for side, surface in enumerate(("east", "south", "west", "north")):
        following = (side + 1) % 4
        a, b = envelope.vertices[side], envelope.vertices[following]
        top_a, top_b = envelope.vertices[side + 4][2], envelope.vertices[following + 4][2]
        length = math.dist(a, b)
        normal = ((a[1] - b[1]) / length, (b[0] - a[0]) / length, 0)
        count = max(1, math.ceil(length / pitch))
        vent_lo, vent_hi = (half - value("louvre_width") / 2) / length, (half + value("louvre_width") / 2) / length
        vent_bottom, vent_top = base + value("louvre_bottom"), base + value("louvre_bottom") + value("louvre_height")
        def wall_patch(key, u0, u1, lo, hi, role="capital", offset=0):
            cuts = sorted({u0, u1, *(u for u in (vent_lo, vent_hi) if surface == "south" and u0 < u < u1)})
            for segment, (left, right) in enumerate(zip(cuts, cuts[1:])):
                regions = ((lo, min(hi, vent_bottom)), (max(lo, vent_top), hi)) if surface == "south" and vent_lo < (left + right) / 2 < vent_hi else ((lo, hi),)
                for region, (low, high) in enumerate(regions):
                    if high - low <= 1e-9:
                        continue
                    polygon = ((left, base), (left, top_a + left * (top_b - top_a)), (right, top_a + right * (top_b - top_a)), (right, base))
                    polygon = _clip_polygon(_clip_polygon(polygon, 1, low, True), 1, high, False)
                    clean = []
                    for point in polygon:
                        if not clean or math.dist(point, clean[-1]) > 1e-9:
                            clean.append(point)
                    if len(clean) > 1 and math.dist(clean[0], clean[-1]) < 1e-9:
                        clean.pop()
                    area = abs(sum(p[0] * q[1] - q[0] * p[1] for p, q in zip(clean, clean[1:] + clean[:1]))) if len(clean) >= 3 else 0
                    if area <= 1e-10:
                        continue
                    points = tuple((a[0] + u * (b[0] - a[0]), a[1] + u * (b[1] - a[1]), z) for u, z in clean)
                    add(f"{key}-{segment}-{region}", role, points, normal, a, surface, offset)
        rows = max(1, math.ceil((max(top_a, top_b) - base) / joint))
        for panel in range(count):
            for row in range(rows):
                wall_patch(f"capital-panel-{surface}-{panel}-{row}", panel / count, (panel + 1) / count, base + row * joint, base + (row + 1) * joint)
        for index in range(1, count):
            u = index / count
            wall_patch(f"capital-seam-{surface}-vertical-{index}", u - seam / (2 * length), u + seam / (2 * length), base, max(top_a, top_b), "capital_seam", projection)
        for row in range(1, rows):
            z = base + row * joint
            wall_patch(f"capital-seam-{surface}-horizontal-{row}", 0, 1, z - seam / 2, z + seam / 2, "capital_seam", projection)
        for index, (lo, hi) in enumerate(((0, flashing / length), (1 - flashing / length, 1))):
            wall_patch(f"capital-corner-flash-{surface}-{index}", lo, hi, base, max(top_a, top_b), "capital_flashing", flash_projection)
        for panel in range(count):
            u0, u1 = panel / count, (panel + 1) / count
            z0, z1 = top_a + u0 * (top_b - top_a), top_a + u1 * (top_b - top_a)
            points = tuple((a[0] + u * (b[0] - a[0]), a[1] + u * (b[1] - a[1]), z) for u, z in ((u0, max(base, z0 - flashing)), (u0, z0), (u1, z1), (u1, max(base, z1 - flashing))))
            add(f"capital-head-flash-{surface}-{panel}", "capital_flashing", points, normal, a, surface, flash_projection)
    x0, x1, y0, y1 = t.center[0] - half, t.center[0] + half, t.center[1] - half, t.center[1] + half
    count = max(1, math.ceil(2 * half / pitch))
    for triangle, indices in enumerate(((4, 7, 6), (4, 6, 5))):
        points = tuple(envelope.vertices[i] for i in indices)
        a, b, c = points
        u, v = tuple(b[k] - a[k] for k in range(3)), tuple(c[k] - a[k] for k in range(3))
        normal = (u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0])
        magnitude = math.sqrt(sum(q * q for q in normal))
        normal = tuple(q / magnitude for q in normal)
        def roof_patch(key, rect, role="capital", offset=0):
            polygon = tuple((p[0], p[1]) for p in points)
            for axis, limit, above in ((0, rect[0], True), (0, rect[1], False), (1, rect[2], True), (1, rect[3], False)):
                polygon = _clip_polygon(polygon, axis, limit, above) if polygon else ()
            clean = []
            for point in polygon:
                if not clean or math.dist(point, clean[-1]) > 1e-9:
                    clean.append(point)
            if len(clean) > 1 and math.dist(clean[0], clean[-1]) < 1e-9:
                clean.pop()
            polygon = tuple(clean)
            if len(polygon) < 3:
                return
            area = abs(sum(p[0] * q[1] - q[0] * p[1] for p, q in zip(polygon, polygon[1:] + polygon[:1])))
            if area <= 1e-10:
                return
            lifted = tuple((x, y, a[2] - ((x - a[0]) * normal[0] + (y - a[1]) * normal[1]) / normal[2]) for x, y in polygon)
            offset = min(offset, max(0, (capital_base(t) + t.capital_height - max(p[2] for p in lifted)) / normal[2] / 2))
            add(key, role, lifted, normal, a, f"roof-{triangle}", offset)
        for panel in range(count):
            roof_patch(f"capital-panel-roof-{triangle}-{panel}", (x0 + panel * 2 * half / count, x0 + (panel + 1) * 2 * half / count, y0, y1))
        for index in range(1, count):
            x = x0 + index * 2 * half / count
            roof_patch(f"capital-seam-roof-{triangle}-{index}", (x - seam / 2, x + seam / 2, y0 + flashing, y1 - flashing), "capital_seam", projection)
        for edge, rect in enumerate(((x0, x0 + flashing, y0, y1), (x1 - flashing, x1, y0, y1), (x0, x1, y0, y0 + flashing), (x0, x1, y1 - flashing, y1))):
            roof_patch(f"capital-edge-flash-roof-{triangle}-{edge}", rect, "capital_flashing")
    return tuple(parts)


def capital_assembly(tower) -> Assembly:
    """🏭 Closed diagonal machinery head with internal bracing, panelized metal skin, louvres and supported water tank."""
    t = _tower(tower)
    cx, cy = t.center
    base, s = capital_base(t), t.core_side / 2 + t.capital_overhang
    envelope = capital_solid(t)
    parts = list(_capital_cladding(t, envelope))
    scale = t.capital_height / _capital_profile_height()
    framing = _data()["capital"]["framing"]
    post_side, brace_side, clearance = (framing[k]["value"] for k in ("post_side", "brace_side", "sheet_clearance"))
    radius = max(post_side, brace_side) / math.sqrt(2)
    margin = _v("capital.cladding.sheet_thickness") + clearance + radius
    x, ymin, ymax = cx - s + margin, cy - s + margin, cy + s - margin
    a, b, c = (envelope.vertices[i] for i in (4, 7, 6))
    gx, gy = (a[2] - b[2]) / (2 * s), (b[2] - c[2]) / (2 * s)
    normal_length = math.sqrt(gx * gx + gy * gy + 1)
    def roof_limit(y):
        """📐 Interior west roof plane offset by sheet, clearance and finite-section radius."""
        return b[2] + gx * (x - b[0]) + gy * (y - b[1]) - margin * normal_length
    def start_y(z):
        """📏 First safe centreline position below the inset sloping roof plane."""
        return max(ymin, b[1] + (z + margin * normal_length - b[2] - gx * (x - b[0])) / gy)
    for i, y in enumerate((ymin, cy, ymax)):
        parts.append(NamedPart(f"capital-post-{i}", "steel_truss", _beam((x, y, base), (x, y, roof_limit(y)), post_side), "metal"))
    levels = [base + z * scale for z in (1, 3, 5, 7, 9, 11)]
    for i, z in enumerate(levels):
        first = start_y(z)
        if first >= ymax:
            continue
        parts.append(NamedPart(f"capital-cross-{i}", "steel_truss", _beam((x, first, z), (x, ymax, z), brace_side), "metal"))
        bottom = base + radius if i == 0 else levels[i - 1]
        parts.append(NamedPart(f"capital-brace-{i}", "steel_truss", _beam((x, start_y(bottom), bottom), (x, ymax, z), brace_side), "metal"))
    for i in range(6):
        parts.append(NamedPart(f"capital-louvre-{i}", "vent", _box(cx - .8, cx + .8, cy - s - .04, cy - s - .005, base + .4 + i * .12, base + .45 + i * .12), "metal"))
    side = -1 if t.key == "A" else 1
    support = _data()["capital"]["tank_support"]
    offset, post, width, depth, cw, cl, ct = (support[k]["value"] for k in ("post_offset", "post_width", "beam_width", "beam_depth", "cradle_width", "cradle_length", "cradle_thickness"))
    equipment = base + _v("capital.tank_support.clearance_lift")
    inner, outer = .8, .8 + cw
    x0, x1 = sorted((cx + side * inner, cx + side * outer))
    parts.append(NamedPart("tank-cradle", "tank_support", _box(x0, x1, cy + .2 - cl / 2, cy + .2 + cl / 2, equipment + .2 - ct, equipment + .2), "metal"))
    for i, y in enumerate((cy + .2 - cl / 2 + width / 2, cy + .2 + cl / 2 - width / 2)):
        px = cx + side * offset
        bx0, bx1 = sorted((px, cx + side * inner))
        parts.append(NamedPart(f"tank-post-{i}", "tank_support", _box(px - post / 2, px + post / 2, y - width / 2, y + width / 2, base, equipment + .2 - ct - depth), "metal"))
        parts.append(NamedPart(f"tank-beam-{i}", "tank_support", _box(bx0, bx1, y - width / 2, y + width / 2, equipment + .2 - ct - depth, equipment + .2 - ct), "metal"))
        brace = _beam((px, y, base + .01), (cx + side * outer, y, equipment + .2 - ct), .04)
        parts.append(NamedPart(f"tank-brace-{i}", "tank_support", brace, "metal"))
    parts.append(NamedPart("capital-water-tank", "water_tank", _cylinder((cx + (-.9 if t.key == "A" else .9), cy + .2, equipment + .2), .6, 1.5, "z", 20), "metal"))
    parts.extend(_capital_signage(t))
    return Assembly(tuple(parts))


def _podium_edge_profile(a, b, profile):
    """🏬 Extrude a facade cross-section along one authored podium edge."""
    length = math.dist(a, b)
    normal = ((b[1] - a[1]) / length, (a[0] - b[0]) / length)
    if sum(p[0] * q[1] - q[0] * p[1] for p, q in zip(profile, profile[1:] + profile[:1])) < 0:
        profile = tuple(reversed(profile))
    rings = tuple(tuple((point[0] + normal[0] * depth, point[1] + normal[1] * depth, z) for depth, z in profile) for point in (a, b))
    return _rings(rings, False)


def _podium_room_parts(p):
    """☕ Assumed room boundaries and fittings implement the original cafe/office programs without claiming a recovered room plan."""
    architecture = p["architecture"]
    levels = tuple(q["value"] for q in p["levels"])
    parts = []
    def box(key, role, bounds, material="wood"):
        parts.append(NamedPart(key, role, _box(*bounds, meta={"room": room["key"], "basis": room["layout_basis"]}), material))
    def fixture(key, role, solid, material):
        parts.append(NamedPart(key, role, _closed(solid.vertices, solid.faces, {"room": room["key"], "basis": room["layout_basis"]}), material))
    for room in architecture["rooms"]:
        key, program = room["key"], room["program"]
        x0, x1, y0, y1 = (q["value"] for q in room["bounds"])
        z = levels[room["level"]]
        floor = z + .035
        top = z + min(architecture["room_height"]["value"], levels[room["level"] + 1] - z - .3)
        box(key + "-finish", "podium_finish", (x0, x1, y0, y1, z, floor), "ochre" if program == "office" else "stone")
        box(key + "-ceiling", "podium_ceiling", (x0, x1, y0, y1, top, top + .05), "plaster")
        for i, x in enumerate((x0 + (x1 - x0) / 3, x0 + 2 * (x1 - x0) / 3)):
            box(f"{key}-luminaire-{i}", "podium_lighting", (x - .3, x + .3, (y0 + y1) / 2 - .15, (y0 + y1) / 2 + .15, top - .04, top), "ceramic")
        doors = {"north": [(tower.center[0], 1.2) for tower in towers()], "south": [(-5.55, 1.8)]} if program == "hall" else {"west": [((y0 + y1) / 2, 1.1)]} if program == "cafe" or key == "office-east" else {"north": [((x0 + x1) / 2, 1.1)]}
        for side, fixed, lo, hi in (("north", y1, x0, x1), ("south", y0, x0, x1), ("east", x1, y0, y1), ("west", x0, y0, y1)):
            holes = [(max(lo, center - width / 2), min(hi, center + width / 2)) for center, width in doors.get(side, []) if lo < center < hi]
            cuts = sorted({lo, hi, *(v for hole in holes for v in hole)})
            for cell, (a, b) in enumerate(zip(cuts, cuts[1:])):
                doorway = any(c < (a + b) / 2 < d for c, d in holes)
                bottom = floor + architecture["door_height"]["value"] if doorway else floor
                if top <= bottom:
                    continue
                bounds = (a, b, fixed - .035, fixed + .035, bottom, top) if side in ("north", "south") else (fixed - .035, fixed + .035, a, b, bottom, top)
                box(f"{key}-{side}-{cell}", "podium_partition", bounds, "glass" if program in ("hall", "cafe") else "plaster")
        if program == "hall":
            box(key + "-reception", "reception_desk", (-8.4, -6.7, -5.2, -4.55, floor, floor + .95))
            for i, x in enumerate((-10.8, -1.5)):
                box(f"{key}-bench-{i}", "hall_bench", (x - .8, x + .8, -5.15, -4.65, floor, floor + .43))
        elif program == "cafe":
            for i, (x, y) in enumerate(((4.2, .1), (5.8, .1), (4.2, 2), (5.8, 2))):
                fixture(f"{key}-table-leg-{i}", "cafe_table", _cylinder((x, y, floor), .06, .72, "z", 12), "metal")
                fixture(f"{key}-table-top-{i}", "cafe_table", _cylinder((x, y, floor + .72), .32, .045, "z", 20), "wood")
                for j, sign in enumerate((-1, 1)):
                    cx = x + sign * .55
                    box(f"{key}-chair-seat-{i}-{j}", "cafe_chair", (cx - .19, cx + .19, y - .19, y + .19, floor + .4, floor + .45))
                    box(f"{key}-chair-base-{i}-{j}", "cafe_chair", (cx - .14, cx + .14, y - .14, y + .14, floor, floor + .4), "metal")
                    back = cx + sign * .17
                    box(f"{key}-chair-back-{i}-{j}", "cafe_chair", (back - .0175, back + .0175, y - .19, y + .19, floor + .45, floor + .82))
            box(key + "-service-counter", "cafe_counter", (3.6, 6.4, 2.65, 3.05, floor, floor + .9))
            box(key + "-serving-shelf", "cafe_counter", (3.55, 6.45, 2.6, 3.1, floor + .9, floor + .95), "metal")
        else:
            for i, x in enumerate((x0 + 1, x1 - 1)):
                y = y0 + (1.1 if key == "office-east" else 1.4)
                box(f"{key}-desk-top-{i}", "office_desk", (x - .6, x + .6, y - .32, y + .32, floor + .71, floor + .76))
                for j, px in enumerate((x - .5, x + .5)):
                    box(f"{key}-desk-support-{i}-{j}", "office_desk", (px - .035, px + .035, y - .25, y + .25, floor, floor + .71), "metal")
                box(f"{key}-chair-{i}", "office_chair", (x - .22, x + .22, y - 1, y - .56, floor, floor + .45))
                box(f"{key}-chair-back-{i}", "office_chair", (x - .22, x + .22, y - 1, y - .96, floor + .45, floor + .85))
            cabinet_y = y1 - .38 if key == "office-east" else y0 + .08
            box(key + "-filing", "office_storage", (x0 + .25, x1 - .25, cabinet_y, cabinet_y + .3, floor, floor + 1))
    return tuple(parts)


def podium_assembly() -> Assembly:
    """🏬 Original opaque podium bands, high strip windows and rounded fascia; cafe/office fitout placement remains assumed."""
    p = _data()["podium"]
    architecture = p["architecture"]
    footprint = tuple(tuple(v["value"] for v in q) for q in p["footprint"])
    levels = tuple(v["value"] for v in p["levels"])
    cx, cy = (sum(point[k] for point in footprint) / len(footprint) for k in range(2))
    ground = tuple((cx + (x - cx) * .88, cy + (y - cy) * .88) for x, y in footprint)
    parts = []
    slab = _detail_value("floor_thickness")
    for level, z in enumerate(levels):
        for cell, solid in enumerate(_pierced_slab(ground if level == 0 else footprint, z - slab, z)):
            parts.append(NamedPart(f"podium-floor-{level}-{cell}", "roof" if level == len(levels) - 1 else "floor", solid, "concrete"))
    radius = architecture["fascia_radius"]["value"]
    thickness = architecture["cladding_thickness"]["value"]
    z0, z1 = levels[1:]
    edge_top = z1 + architecture["roof_edge_height"]["value"]
    w0, w1 = (z0 + (z1 - z0) * q["value"] for q in architecture["window_band"])
    for edge, (a, b) in enumerate(zip(footprint, footprint[1:] + footprint[:1])):
        length = math.dist(a, b)
        spans = max(1, math.ceil(length / architecture["panel_pitch"]["value"]))
        for i in range(spans):
            p0, p1 = (tuple(a[k] + (b[k] - a[k]) * value / spans for k in range(2)) for value in (i, i + 1))
            for band, lo, hi in (("lower", z0 + radius, w0), ("upper", w1, edge_top - radius)):
                parts.append(NamedPart(f"podium-cladding-{edge}-{i}-{band}", "podium_cladding", _wall_strip(p0, p1, lo, hi, thickness), "shell"))
            recess = architecture["window_recess"]["value"]
            normal = ((b[1] - a[1]) / length, (a[0] - b[0]) / length)
            g0, g1 = (tuple(point[k] - normal[k] * recess for k in range(2)) for point in (p0, p1))
            parts.append(NamedPart(f"podium-glass-1-{edge}-{i}", "podium_glazing", _wall_strip(g0, g1, w0, w1, .024), "glass"))
            parts.append(NamedPart(f"podium-window-mullion-{edge}-{i}", "podium_window_frame", _wall_strip(g0, tuple(g0[k] + (b[k] - a[k]) / length * .03 for k in range(2)), w0, w1, .045), "metal"))
            parts.append(NamedPart(f"podium-panel-joint-{edge}-{i}", "podium_panel_joint", _wall_strip(p0, tuple(p0[k] + (b[k] - a[k]) / length * architecture["joint_width"]["value"] for k in range(2)), z0 + radius, w0, thickness + .01), "metal"))
        rail_a, rail_b = (tuple(point[k] - normal[k] * recess for k in range(2)) for point in (a, b))
        for j, z in enumerate((w0, w1)):
            parts.append(NamedPart(f"podium-window-rail-{edge}-{j}", "podium_window_frame", _wall_strip(rail_a, rail_b, z - .025, z + .025, .045), "metal"))
        for end, center, sign in (("bottom", z0 + radius, -1), ("top", edge_top - radius, 1)):
            outer = tuple((-radius + radius * math.cos(i * math.pi / 10), center + sign * radius * math.sin(i * math.pi / 10)) for i in range(6))
            inner = tuple((-radius + max(0, radius - thickness) * math.cos(i * math.pi / 10), center + sign * max(0, radius - thickness) * math.sin(i * math.pi / 10)) for i in range(5, -1, -1))
            solid = _podium_edge_profile(a, b, outer + inner)
            parts.append(NamedPart(f"podium-fascia-1-{edge}-{end}", "podium_fascia", solid, "shell"))
            lip = architecture["fascia_anchor_width"]["value"]
            lo, hi = (z0, z0 + .05) if end == "bottom" else (z1 - .05, z1)
            anchor = _podium_edge_profile(a, b, ((-radius - lip, lo), (-radius, lo), (-radius, hi), (-radius - lip, hi)))
            parts.append(NamedPart(f"podium-fascia-anchor-{edge}-{end}", "podium_fascia_support", anchor, "metal"))
            if end == "top" and edge_top > z1:
                coping = _podium_edge_profile(a, b, ((-radius - lip, edge_top - .04), (-radius, edge_top - .04), (-radius, edge_top), (-radius - lip, edge_top)))
                parts.append(NamedPart(f"podium-coping-{edge}", "podium_coping", coping, "metal"))
                inner = _podium_edge_profile(a, b, ((-radius - thickness, z1), (-radius, z1), (-radius, edge_top - .04), (-radius - thickness, edge_top - .04)))
                parts.append(NamedPart(f"podium-parapet-inner-{edge}", "podium_parapet", inner, "shell"))
    for edge, (a, b) in enumerate(zip(ground, ground[1:] + ground[:1])):
        count = max(1, math.ceil(math.dist(a, b) / 5))
        for i in range(count):
            x, y = (a[k] + (b[k] - a[k]) * i / count for k in range(2))
            parts.append(NamedPart(f"podium-pilotis-{edge}-{i}", "podium_column", _box(x - .15, x + .15, y - .15, y + .15, levels[0], levels[1]), "concrete"))
    parts.extend(_podium_room_parts(p))
    return Assembly(tuple(parts))


def _capital_signage(t):
    """🈚 Historic vertical 中銀 lettering, with manually authored vector strokes and explicitly inferred mounting dimensions."""
    data = _data()["capital"]["signage"]
    if t.key != data["tower"]:
        return ()
    height_scale = t.capital_height / _capital_profile_height()
    scale = min(height_scale, (t.core_side + 2 * t.capital_overhang) / data["reference_base_side"]["value"])
    size, stroke, depth, pitch, offset, width, height = (data[k]["value"] * scale for k in ("glyph_size", "stroke_width", "stroke_depth", "vertical_pitch", "corner_offset", "mount_width", "mount_height"))
    s = t.core_side / 2 + t.capital_overhang
    top = capital_base(t) + t.capital_height - data["top_offset"]["value"] * height_scale
    bottom = top + .08 * scale - height
    parts = []
    for face in data["faces"]:
        frame = Plane((t.center[0] - s + offset, t.center[1] + s + .025, bottom), (1, 0, 0), (0, 1, 0), (0, 0, 1)) if face == "north" else Plane((t.center[0] - s - .025, t.center[1] + s - offset, bottom), (0, 1, 0), (-1, 0, 0), (0, 0, 1))
        parts.append(NamedPart(f"sign-{face}-mount", "sign_mount", place(_box(-width / 2, width / 2, 0, depth, 0, height), frame), "dark"))
        for index, character in enumerate(data["text"]):
            for stroke_index, path in enumerate(data["glyphs"][character]):
                points = tuple(((u - .5) * size, depth + stroke, top - bottom - index * pitch - v * size) for u, v in path)
                for segment, (a, b) in enumerate(zip(points, points[1:])):
                    solid = place(_beam(a, b, stroke), frame)
                    solid = _closed(solid.vertices, solid.faces, {"character": character, "face": face, "source": data["source"]})
                    parts.append(NamedPart(f"sign-{face}-{index}-{stroke_index}-{segment}", "historic_sign", solid, "shell"))
    return tuple(parts)


def bridge_assembly(level) -> Assembly:
    """🌉 Reconstruct native flat plated bases and tubular V side frames at the shared bridge datum."""
    a, b = sorted(towers(), key=lambda t: t.center[0])
    x0, x1 = a.center[0]+a.core_side/2, b.center[0]-b.core_side/2
    z, width = bridge_elevation(level), _detail_value("bridge_width")
    cy = _v("detail.bridge_geometry.lateral_offset")
    def parameter(key):
        return _v("detail.bridge_geometry."+key)
    plate, slab, gap = (parameter(k) for k in ("wearing_plate_thickness","base_thickness","expansion_gap"))
    middle, inset = (x0+x1)/2, parameter("end_inset")
    low, high = cy-width/2, cy+width/2
    parts = []
    def add(key,role,solid,material="metal"):
        parts.append(NamedPart(f"bridge-{level}"+key,role,solid,material))
    deck = _box(x0,x1,low,high,z-plate,z)
    add("","bridge",_closed(deck.vertices,deck.faces,{"walking_axis":"x","native_flat_deck":True,"datum":z}))
    for half,(start,end) in enumerate(((x0,middle-gap/2),(middle+gap/2,x1))):
        add(f"-base-{half}","bridge_base",_box(start,end,low,high,z-plate-slab,z-plate),"concrete")
        for side,y in enumerate((low+inset,high-inset)):
            guard_start = start + parameter("native_return_width") if half == 0 and side == 0 and _data()["detail"]["bridge_geometry"]["return_enabled"] else start
            positions = (guard_start+inset,(guard_start+end)/2,end-inset)
            for post,x in enumerate(positions):
                mast = (half==0 and post==0) or (half==1 and post==2)
                height = parameter("brace_height") if mast else parameter("rail_height")
                add(f"-post-{half}-{side}-{post}","bridge_mast" if mast else "bridge_post",_fixture_tube(((x,y,z),(x,y,z+height)),parameter("brace_radius") if mast else parameter("post_radius"),16))
            for key,height,radius in (("rail",parameter("rail_height"),parameter("rail_radius")),("midrail",parameter("midrail_height"),parameter("midrail_radius"))):
                add(f"-{key}-{half}-{side}","bridge_rail",_fixture_tube(((guard_start+inset,y,z+height),(end-inset,y,z+height)),radius,16))
            upper, lower = ((guard_start+inset,z+parameter("brace_height")),(end-inset,z)) if half==0 else ((end-inset,z+parameter("brace_height")),(start+inset,z))
            add(f"-diagonal-{half}-{side}","bridge_diagonal",_fixture_tube(((upper[0],y,upper[1]),(lower[0],y,lower[1])),parameter("brace_radius"),16))
        x = start+inset if half==0 else end-inset
        south_x = x + parameter("native_return_width") if half == 0 and _data()["detail"]["bridge_geometry"]["return_enabled"] else x
        add(f"-crossbar-{half}","bridge_crossbar",_fixture_tube(((south_x,low+inset,z+parameter("brace_height")),(x,high-inset,z+parameter("brace_height"))),parameter("brace_radius"),16))
    if _data()["detail"]["bridge_geometry"]["return_enabled"]:
        rx1, ry0 = x0 + parameter("native_return_width"), low - parameter("native_return_length")
        margin = parameter("passage_width") / 2 + inset + parameter("rail_radius")
        deck = _box(x0, rx1, ry0, low, z - plate, z)
        add("-return", "bridge_return", _closed(deck.vertices, deck.faces, {"walking_axis": "y", "walking_route": (((x0 + rx1) / 2, low), ((x0 + rx1) / 2, ry0 + margin)), "native_return": True, "datum": z}))
        add("-return-base", "bridge_return_base", _box(x0, rx1, ry0, low, z - plate - slab, z - plate), "concrete")
        sides = (((x0 + inset, ry0 + inset), (x0 + inset, low - inset)), ((rx1 - inset, ry0 + inset), (rx1 - inset, low - inset)), ((x0 + inset, ry0 + inset), (rx1 - inset, ry0 + inset)))
        for edge, (start, end) in enumerate(sides):
            for name, height, radius in (("rail", parameter("rail_height"), parameter("rail_radius")), ("midrail", parameter("midrail_height"), parameter("midrail_radius"))):
                add(f"-return-{name}-{edge}", "bridge_return_rail", _fixture_tube(((start[0], start[1], z + height), (end[0], end[1], z + height)), radius, 16))
        for index, (x, y) in enumerate(((x0 + inset, ry0 + inset), (x0 + inset, low - inset), (rx1 - inset, ry0 + inset), (rx1 - inset, low - inset))):
            add(f"-return-post-{index}", "bridge_return_post", _fixture_tube(((x, y, z), (x, y, z + parameter("rail_height"))), parameter("post_radius"), 16))
    return Assembly(tuple(parts))


def structure_assembly(detail="full", cancelled=None) -> Assembly:
    """🌉 Both cores and capitals, podium and three explicitly assumed connecting bridge levels."""
    parts = []
    for t in towers():
        _check_cancel(cancelled)
        for group in (core_assembly(t, detail, cancelled=cancelled), capital_assembly(t)):
            parts.extend(NamedPart(t.key + "/" + p.key, p.role, p.solid, p.material) for p in group.parts)
    parts.extend(podium_assembly().parts)
    parts.extend(foundation_assembly().parts)
    for level in _data()["detail"]["bridge_storeys"]:
        parts.extend(bridge_assembly(level).parts)
    return Assembly(tuple(parts))


class AssemblyCancelled(Exception):
    """🛑 Assembly generation was cancelled at a safe component boundary."""


@dataclass(frozen=True)
class AssemblyProgress:
    """📈 Language-neutral generation progress: current phase and completed/total work units."""
    phase: str
    completed: int
    total: int


def _check_cancel(cancelled):
    if cancelled is not None and cancelled():
        raise AssemblyCancelled("Building assembly cancelled")


def building_assembly(detail="exterior", interior="reference", progress=None, cancelled=None) -> Assembly:
    """🏢 Build all authored capsules and structure, with optional progress events and cooperative cancellation at component boundaries."""
    _check_cancel(cancelled)
    def report(phase, completed, total):
        if progress is not None:
            progress(AssemblyProgress(phase, completed, total))
        _check_cancel(cancelled)
    report("structure", 0, 1)
    structure = structure_assembly(detail, cancelled=cancelled)
    parts = []
    for part in structure.parts:
        _check_cancel(cancelled)
        parts.append(part)
    report("structure", 1, 1)
    prototypes = {}
    slots_to_build = slots()
    report("capsules", 0, len(slots_to_build))
    for completed, slot in enumerate(slots_to_build, 1):
        _check_cancel(cancelled)
        key = (slot.type if interior == "study" else "A", slot.layout)
        if key not in prototypes:
            prototypes[key] = capsule_assembly(capsule_type(key[0]), detail, slot.layout, interior)
        tag = "/".join(str(i) for i in (*slot.path, slot.item))
        for part in place_assembly(prototypes[key], slot.plane).parts:
            _check_cancel(cancelled)
            parts.append(NamedPart(tag + "/" + part.key, part.role, part.solid, part.material))
        report("capsules", completed, len(slots_to_build))
    report("complete", len(parts), len(parts))
    return Assembly(tuple(parts))


def capital_base(tower) -> float:
    """📏 Explicit selected concrete roof datum shared by core and machinery head."""
    return _tower(tower).height


def _beam(start, end, side):
    delta = tuple(b - a for a, b in zip(start, end))
    length = math.sqrt(sum(v * v for v in delta))
    z = tuple(v / length for v in delta)
    seed = (0, 0, 1) if abs(z[2]) < .9 else (0, 1, 0)
    x = (seed[1] * z[2] - seed[2] * z[1], seed[2] * z[0] - seed[0] * z[2], seed[0] * z[1] - seed[1] * z[0])
    norm = math.sqrt(sum(v * v for v in x))
    x = tuple(v / norm for v in x)
    y = (z[1] * x[2] - z[2] * x[1], z[2] * x[0] - z[0] * x[2], z[0] * x[1] - z[1] * x[0])
    return place(_box(-side / 2, side / 2, -side / 2, side / 2, 0, length), Plane(start, x, y, z))


def _swapped_xy(solid, offset):
    return _closed(tuple((b + offset, a, c) for a, b, c in solid.vertices), tuple(tuple(reversed(f)) for f in solid.faces), solid.meta)


def _perforated_profile(profile, center, radius, depth0, depth1, count, other=None):
    """🧱 Conformal planar caps preserve both exact profile boundaries and the unchanged aperture polygon."""
    profiles=[tuple(profile),tuple(profile if other is None else other)]
    profiles=[p if sum(a[0]*b[1]-a[1]*b[0] for a,b in zip(p,p[1:]+p[:1]))>0 else tuple(reversed(p)) for p in profiles]
    if len(profiles[0])!=len(profiles[1]):
        raise ValueError("Lofted profile boundaries require corresponding vertices")
    def ray(profile,angle):
        direction=(math.cos(angle),math.sin(angle))
        hits=[]
        for edge,(a,b) in enumerate(zip(profile,profile[1:]+profile[:1])):
            delta=(b[0]-a[0],b[1]-a[1])
            offset=(a[0]-center[0],a[1]-center[1])
            denominator=direction[0]*delta[1]-direction[1]*delta[0]
            if abs(denominator)<1e-14:
                continue
            distance=(offset[0]*delta[1]-offset[1]*delta[0])/denominator
            fraction=(offset[0]*direction[1]-offset[1]*direction[0])/denominator
            if distance>0 and -1e-12<=fraction<=1+1e-12:
                if distance<radius-1e-10:
                    raise ValueError("Aperture must lie inside its material profile")
                if fraction>=1-1e-12:
                    edge,fraction=(edge+1)%len(profile),0
                hits.append((distance,(edge,round(max(0,fraction),12))))
        if not hits:
            raise ValueError("Aperture center must lie inside its material profile")
        return min(hits,key=lambda hit:hit[0])[1]
    rays=[[ray(p,2*math.pi*i/count) for i in range(count)] for p in profiles]
    cuts={i:{0.0} for i in range(len(profile))}
    for boundary in rays:
        for edge,fraction in boundary:
            cuts[edge].add(fraction)
    parameters=[(edge,fraction) for edge in range(len(profile)) for fraction in sorted(cuts[edge])]
    indices={parameter:i for i,parameter in enumerate(parameters)}
    outlines=[tuple((p[edge][0]+fraction*(p[(edge+1)%len(p)][0]-p[edge][0]),p[edge][1]+fraction*(p[(edge+1)%len(p)][1]-p[edge][1])) for edge,fraction in parameters) for p in profiles]
    inner=_circle_loop(center[0],center[1],radius,count)
    vertices,faces=[],[]
    def face(points):
        offset=len(vertices)
        vertices.extend(points)
        faces.append(tuple(range(offset,offset+len(points))))
    for side,(outline,depth) in enumerate(zip(outlines,(depth0,depth1))):
        for i in range(count):
            start,stop=indices[rays[side][i]],indices[rays[side][(i+1)%count]]
            chain=[outline[start]]
            while start!=stop:
                start=(start+1)%len(outline)
                chain.append(outline[start])
                if len(chain)>len(outline):
                    raise ValueError("Profile is not star shaped around the aperture")
            polygon=[inner[i],*chain,inner[(i+1)%count]]
            middle=tuple(sum(point[k] for point in polygon)/len(polygon) for k in range(2))
            for a,b in zip(polygon,polygon[1:]+polygon[:1]):
                triangle=tuple((x,depth,z) for x,z in (middle,a,b))
                face(tuple(reversed(triangle)) if side else triangle)
    for i in range(len(parameters)):
        j=(i+1)%len(parameters)
        a,b=outlines[0][i],outlines[0][j]
        c,d=outlines[1][i],outlines[1][j]
        points=((a[0],depth0,a[1]),(c[0],depth1,c[1]),(d[0],depth1,d[1]),(b[0],depth0,b[1]))
        face((points[0],points[1],points[2]))
        face((points[0],points[2],points[3]))
    for a,b in zip(inner,inner[1:]+inner[:1]):
        face(((a[0],depth0,a[1]),(b[0],depth0,b[1]),(b[0],depth1,b[1]),(a[0],depth1,a[1])))
    return _positive_material(vertices,faces,{"aperture_radius":radius,"conformal_profile":True})


def _panel(bounds, depth0, depth1, hole=None, radius=0, count=48):
    x0, x1, z0, z1 = bounds
    if hole and hole[0] == "circle":
        _, cx, cz, hr = hole
        profile = tuple((x+(x0+x1)/2,z+z0) for x,z in _profile(x1-x0,z1-z0,radius,max(1,count//4)))
        return _perforated_profile(profile,(cx,cz),hr,depth0,depth1,count)
    else:
        outer = ((x0, z0), (x1, z0), (x1, z1), (x0, z1))
        inner = None if hole is None else ((hole[1], hole[3]), (hole[2], hole[3]), (hole[2], hole[4]), (hole[1], hole[4]))
    if inner is None:
        return _rings((_at_depth(outer, depth0), _at_depth(outer, depth1)), False)
    return _rings((_at_depth(outer, depth0), _at_depth(outer, depth1), _at_depth(inner, depth1), _at_depth(inner, depth0)))


def _profile_panel(profile, depth0, depth1, hole=None):
    """🧩 Extrude the planar material union around a true rectangular aperture without internal partition faces."""
    if hole is None:
        return _rings((_at_depth(profile,depth0),_at_depth(profile,depth1)),False)
    _,x0,x1,z0,z1 = hole
    middle = _clip_polygon(_clip_polygon(profile,0,x0,True),0,x1,False)
    clipped = (_clip_polygon(profile,0,x0,False),_clip_polygon(profile,0,x1,True),_clip_polygon(middle,1,z0,False),_clip_polygon(middle,1,z1,True))
    regions=[]
    canonical={tuple(round(v,11) for v in point):point for point in profile}
    for region in clipped:
        clean=[]
        for point in region:
            point=canonical.setdefault(tuple(round(v,11) for v in point),point)
            if not clean or math.dist(point,clean[-1])>1e-10:
                clean.append(point)
        if len(clean)>1 and math.dist(clean[0],clean[-1])<1e-10:
            clean.pop()
        area=sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(clean,clean[1:]+clean[:1]))
        if abs(area)>1e-12:
            regions.append(clean if area>0 else list(reversed(clean)))
    points=tuple(dict.fromkeys(point for region in regions for point in region))
    vertices,faces,boundary=[],[],{}
    for region in regions:
        split=[]
        for a,b in zip(region,region[1:]+region[:1]):
            direction=(b[0]-a[0],b[1]-a[1])
            length=direction[0]**2+direction[1]**2
            along=[]
            for point in points:
                t=((point[0]-a[0])*direction[0]+(point[1]-a[1])*direction[1])/length
                cross=(point[0]-a[0])*direction[1]-(point[1]-a[1])*direction[0]
                if -1e-10<=t<1-1e-10 and abs(cross)<1e-10:
                    along.append((t,point))
            split.extend(point for _,point in sorted(along))
        centre=tuple(sum(point[k] for point in region)/len(region) for k in range(2))
        offset=len(vertices)
        vertices.extend((x,depth,z) for depth in (depth0,depth1) for x,z in [centre]+split)
        n=len(split)+1
        for i,(a,b) in enumerate(zip(split,split[1:]+split[:1])):
            j=(i+1)%len(split)
            faces.extend(((offset,offset+i+1,offset+j+1),(offset+n,offset+n+j+1,offset+n+i+1)))
            key=tuple(sorted((a,b)))
            if key in boundary:
                del boundary[key]
            else:
                boundary[key]=(a,b)
    for a,b in boundary.values():
        offset=len(vertices)
        vertices.extend(((a[0],depth0,a[1]),(b[0],depth0,b[1]),(b[0],depth1,b[1]),(a[0],depth1,a[1])))
        faces.append(tuple(offset+i for i in reversed(range(4))))
    return _positive_material(vertices,faces,{"rect_aperture":(x0,x1,z0,z1),"profile_clipped":True})


def _material_faces(vertices, faces):
    """🔺 Ear-clip planar material caps so concave radial voids remain genuine geometry."""
    result=[]
    for face in faces:
        if len(face)<=4:
            result.append(face)
            continue
        normal=tuple(sum(vertices[a][(k+1)%3]*vertices[b][(k+2)%3]-vertices[a][(k+2)%3]*vertices[b][(k+1)%3] for a,b in zip(face,face[1:]+face[:1])) for k in range(3))
        drop=max(range(3),key=lambda k:abs(normal[k]))
        axes=[k for k in range(3) if k!=drop]
        points={i:tuple(vertices[i][k] for k in axes) for i in face}
        def cross(a,b,c):
            return (points[b][0]-points[a][0])*(points[c][1]-points[a][1])-(points[b][1]-points[a][1])*(points[c][0]-points[a][0])
        area=sum(points[a][0]*points[b][1]-points[a][1]*points[b][0] for a,b in zip(face,face[1:]+face[:1]))
        sign=1 if area>0 else -1
        pending=list(face)
        while len(pending)>3:
            for j,b in enumerate(pending):
                a,c=pending[j-1],pending[(j+1)%len(pending)]
                if sign*cross(a,b,c)<=1e-13:
                    continue
                occupied=any(all(sign*cross(u,v,q)>=-1e-13 for u,v in ((a,b),(b,c),(c,a))) for q in pending if q not in (a,b,c))
                if occupied:
                    continue
                result.append((a,b,c))
                pending.pop(j)
                break
            else:
                raise ValueError("Material cap is not a simple nondegenerate planar polygon")
        result.append(tuple(pending))
    return tuple(result)


def _positive_material(vertices, faces, meta=None):
    """🧱 Weld analytic clipping vertices, then close and orient the actual material patch."""
    welded,index,mapped=[],{},[]
    for point in vertices:
        key=tuple(round(v,11) for v in point)
        if key not in index:
            index[key]=len(welded)
            welded.append(point)
        mapped.append(index[key])
    clean=[]
    for face in faces:
        face=tuple(dict.fromkeys(mapped[i] for i in face))
        if len(face)>=3:
            normal=tuple(sum(welded[a][(k+1)%3]*welded[b][(k+2)%3]-welded[a][(k+2)%3]*welded[b][(k+1)%3] for a,b in zip(face,face[1:]+face[:1])) for k in range(3))
            if math.sqrt(sum(v*v for v in normal))>1e-13:
                clean.append(face)
    vertices=tuple(welded)
    faces = _material_faces(vertices,tuple(clean))
    volume = sum(sum(vertices[f[0]][0]*(vertices[f[i]][1]*vertices[f[i+1]][2]-vertices[f[i]][2]*vertices[f[i+1]][1])-vertices[f[0]][1]*(vertices[f[i]][0]*vertices[f[i+1]][2]-vertices[f[i]][2]*vertices[f[i+1]][0])+vertices[f[0]][2]*(vertices[f[i]][0]*vertices[f[i+1]][1]-vertices[f[i]][1]*vertices[f[i+1]][0]) for i in range(1,len(f)-1)) for f in faces)
    return _closed(vertices,tuple(tuple(reversed(f)) for f in faces) if volume<0 else faces,meta)


def _skin_patch(outer, inner, rows, cols, meta=None):
    """🌐 Weld a finite spherical material grid, triangulate curved faces and close its boundary."""
    count = len(outer)
    points,faces = list(outer)+list(inner),[]
    for i in range(rows):
        for j in range(cols):
            a=i*(cols+1)+j
            for face in ((a,a+cols+1,a+cols+2),(a,a+cols+2,a+1)):
                faces.append(face)
                faces.append(tuple(count+k for k in reversed(face)))
    boundary = [i*(cols+1) for i in range(rows+1)]+[rows*(cols+1)+j for j in range(1,cols+1)]+[i*(cols+1)+cols for i in range(rows-1,-1,-1)]+list(range(cols-1,0,-1))
    faces.extend((b,a,count+a,count+b) for a,b in zip(boundary,boundary[1:]+boundary[:1]))
    return _positive_material(tuple(points),tuple(faces),meta)


def _round_band(cx, cz, start, radius, thickness, segments, ceiling=None):
    """📐 Clip a circular material band analytically at the actual door sill."""
    def arc(r):
        angles=[_rad(start+90*i/segments) for i in range(segments+1)]
        if ceiling is not None and abs((ceiling-cz)/r)<1:
            a=math.asin((ceiling-cz)/r)
            for angle in (a,math.pi-a):
                for turn in range(-2,3):
                    candidate=angle+turn*2*math.pi
                    if angles[0]+1e-10<candidate<angles[-1]-1e-10:
                        angles.append(candidate)
        return [(cx+r*math.cos(a),cz+r*math.sin(a)) for a in sorted(angles) if ceiling is None or cz+r*math.sin(a)<=ceiling+1e-10]
    return tuple(arc(radius)+list(reversed(arc(radius-thickness))))


def _loft_material_panel(outer,inner,depth0,depth1,hole,count):
    """📦 Loft finite outer/inner wall boundaries around a single manifold aperture ring."""
    if hole and hole[0]=="circle":
        rectangle=lambda b:((b[0],b[2]),(b[1],b[2]),(b[1],b[3]),(b[0],b[3]))
        return _perforated_profile(rectangle(outer),(hole[1],hole[2]),hole[3],depth0,depth1,count,rectangle(inner))
    def outline(bounds):
        x0,x1,z0,z1=bounds
        return ((x0,z0),(x1,z0),(x1,z1),(x0,z1))
    loops=(_at_depth(outline(outer),depth0),_at_depth(outline(inner),depth1))
    if hole:
        aperture=((hole[1],hole[3]),(hole[2],hole[3]),(hole[2],hole[4]),(hole[1],hole[4]))
        loops+=(_at_depth(aperture,depth1),_at_depth(aperture,depth0))
    solid=_rings(loops,bool(hole))
    faces=tuple(triangle for face in solid.faces for triangle in ((face[0],face[1],face[2]),(face[0],face[2],face[3]))) if all(len(face)==4 for face in solid.faces) else solid.faces
    return _positive_material(solid.vertices,faces)


def _thin_radius_casing_parts(t,layout,thickness,count,material):
    """🧊 Exact Box and sub-skin fillets offset to a sharp inner box, without negative radii."""
    c,family=capsule_casing(t),_family(layout)
    radius=_param(t,"corner_radius")
    outer=((c.left+radius,c.right-radius),(c.back+radius,c.front-radius),(radius,c.height-radius))
    inner=((c.left+thickness,c.right-thickness),(c.back+thickness,c.front-thickness),(thickness,c.height-thickness))
    dw,dh,finish=_detail_value("door_width"),_detail_value("door_height"),_detail_value("floor_finish_height")
    coordinate=family["entry_coordinate"]["value"]
    parts=[]
    def add(key,solid,role="shell"):
        parts.append(NamedPart(key,role,_positive_material(solid.vertices,solid.faces,{"wall_thickness":thickness,"rounding_radius":radius}),material))
    for axis,name in ((0,"side"),(1,"end"),(2,"level")):
        other=[k for k in range(3) if k!=axis]
        ob=(outer[other[0]][0],outer[other[0]][1],outer[other[1]][0],outer[other[1]][1])
        ib=(inner[other[0]][0],inner[other[0]][1],inner[other[1]][0],inner[other[1]][1])
        for sign in (-1,1):
            label=("left" if sign<0 else "right") if axis==0 else ("back" if sign<0 else "front") if axis==1 else ("floor" if sign<0 else "roof")
            depth=(c.left if sign<0 else c.right) if axis==0 else (c.back if sign<0 else c.front) if axis==1 else (0 if sign<0 else c.height)
            hole=None
            if family["entry_wall"]==label:
                hole=("rect",coordinate-dw/2,coordinate+dw/2,finish,finish+dh)
            elif family["window_wall"]==label:
                centre=_param(t,"window_u") if axis==1 else _param(t,"length")/2+family["window_longitudinal_offset"]["value"]+_param(t,"window_u")
                hole=("circle",centre,_param(t,"window_v"),_param(t,"window_diameter")/2+thickness)
            solid=_loft_material_panel(ob,ib,depth,depth-sign*thickness,hole,count)
            if axis==0:
                solid=_closed(tuple((y,x,z) for x,y,z in solid.vertices),solid.faces)
            elif axis==2:
                solid=_closed(tuple((x,z,y) for x,y,z in solid.vertices),solid.faces)
            add("shell-"+label,solid,"shell_back" if label=="back" else "shell")
    if radius<=1e-12:
        return parts
    q=_capsule_quarters(t,count)
    for axis in range(3):
        other=[k for k in range(3) if k!=axis]
        for a in (0,1):
            for b in (0,1):
                grids=[]
                for skin in (False,True):
                    grid=[]
                    for end in (0,1):
                        for j in range(q+1):
                            point=[0.,0.,0.]
                            point[axis]=(inner if skin else outer)[axis][end]
                            point[other[0]]=(inner if skin else outer)[other[0]][a]+(0 if skin else (-1 if a==0 else 1)*radius*math.cos(j*math.pi/2/q))
                            point[other[1]]=(inner if skin else outer)[other[1]][b]+(0 if skin else (-1 if b==0 else 1)*radius*math.sin(j*math.pi/2/q))
                            grid.append(tuple(point))
                    grids.append(tuple(grid))
                add(f"shell-small-edge-{axis}-{a}-{b}",_skin_patch(grids[0],grids[1],1,q))
    for a in (0,1):
        for b in (0,1):
            for d in (0,1):
                grids=[]
                for skin in (False,True):
                    centre=((inner if skin else outer)[0][a],(inner if skin else outer)[1][b],(inner if skin else outer)[2][d])
                    r=0 if skin else radius
                    grids.append(tuple((centre[0]+(-1 if a==0 else 1)*r*math.cos(i*math.pi/2/q)*math.cos(j*math.pi/2/q),centre[1]+(-1 if b==0 else 1)*r*math.cos(i*math.pi/2/q)*math.sin(j*math.pi/2/q),centre[2]+(-1 if d==0 else 1)*r*math.sin(i*math.pi/2/q)) for i in range(q+1) for j in range(q+1)))
                add(f"shell-small-sphere-{a}-{b}-{d}",_skin_patch(grids[0],grids[1],q,q))
    return parts


def _rounded_casing_parts(t, layout, thickness, count, material):
    """📦 Six flat panels, twelve rolled edges and eight spherical corners with real curved doorway cuts."""
    c,family = capsule_casing(t),_family(layout)
    if _param(t,"corner_radius")<=thickness:
        return _thin_radius_casing_parts(t,layout,thickness,count,material)
    r,u,v = _param(t,"corner_radius"),_param(t,"window_u"),_param(t,"window_v")
    segments = _capsule_quarters(t,count)
    xl,xr,yl,yr,zl,zr = c.left+r,c.right-r,c.back+r,c.front-r,c.bottom+r,c.top-r
    dw,dh,finish = _detail_value("door_width"),_detail_value("door_height"),_detail_value("floor_finish_height")
    coordinate = family["entry_coordinate"]["value"]
    radius = _param(t,"window_diameter")/2+thickness
    parts=[]
    def add(key,solid,role="shell"):
        parts.append(NamedPart(key,role,_positive_material(solid.vertices,solid.faces,{**solid.meta,"wall_thickness":thickness,"rounding":"full3d"}),material))
    front_hole = ("circle",u,v,radius) if family["window_wall"]=="front" else None
    add("shell-front",_panel((xl,xr,zl,zr),c.front-thickness,c.front,front_hole,count=count))
    back_hole = ("rect",coordinate-dw/2,coordinate+dw/2,finish,finish+dh) if family["entry_wall"]=="back" else None
    add("shell-back",_profile_panel(((xl,zl),(xr,zl),(xr,zr),(xl,zr)),c.back,c.back+thickness,back_hole),"shell_back")
    for side,x in (("left",c.left),("right",c.right-thickness)):
        hole=None
        if family["entry_wall"]==side:
            hole=("rect",coordinate-dw/2,coordinate+dw/2,finish,finish+dh)
            panel=_profile_panel(((yl,zl),(yr,zl),(yr,zr),(yl,zr)),0,thickness,hole)
        else:
            if side=="right" and family["window_wall"]=="right":
                hole=("circle",_param(t,"length")/2+family["window_longitudinal_offset"]["value"]+u,v,radius)
            panel=_panel((yl,yr,zl,zr),0,thickness,hole,count=count)
        add("shell-"+side,_swapped_xy(panel,x))
    add("shell-floor",_box(xl,xr,yl,yr,c.bottom,c.bottom+thickness))
    add("shell-roof",_box(xl,xr,yl,yr,c.top-thickness,c.top))
    def edge(key,cx,cz,start,lo,hi,axis,cut=False):
        intervals=[(lo,hi,None)]
        if cut:
            intervals=[(lo,coordinate-dw/2,None),(coordinate-dw/2,coordinate+dw/2,finish),(coordinate+dw/2,hi,None)]
        for index,(a,b,ceiling) in enumerate(intervals):
            profile=_round_band(cx,cz,start,r,thickness,segments,ceiling)
            if len(profile)<3 or b-a<1e-10:
                continue
            solid=_rings((_at_depth(profile,a),_at_depth(profile,b)),False)
            if axis=="x":
                solid=_closed(tuple((y,x,z) for x,y,z in solid.vertices),solid.faces)
            elif axis=="z":
                solid=_closed(tuple((x,z,y) for x,y,z in solid.vertices),solid.faces)
            add(f"shell-edge-{key}-{index}",solid)
    for key,cx,cz,start,side in (("right-bottom",xr,zl,-90,"right"),("right-top",xr,zr,0,None),("left-top",xl,zr,90,None),("left-bottom",xl,zl,180,"left")):
        edge("y-"+key,cx,cz,start,yl,yr,"y",family["entry_wall"]==side)
    for key,cy,cz,start,cut in (("back-bottom",yl,zl,180,family["entry_wall"]=="back"),("front-bottom",yr,zl,-90,False),("front-top",yr,zr,0,False),("back-top",yl,zr,90,False)):
        edge("x-"+key,cy,cz,start,xl,xr,"x",cut)
    for key,cx,cy,start in (("right-front",xr,yr,0),("left-front",xl,yr,90),("left-back",xl,yl,180),("right-back",xr,yl,270)):
        edge("z-"+key,cx,cy,start,zl,zr,"z")
    for sx,cx in ((-1,xl),(1,xr)):
        for sy,cy in ((-1,yl),(1,yr)):
            for sz,cz in ((-1,zl),(1,zr)):
                grids=[]
                for radial in (r,r-thickness):
                    grids.append(tuple((cx+sx*radial*math.cos(i*math.pi/2/segments)*math.cos(j*math.pi/2/segments),cy+sy*radial*math.cos(i*math.pi/2/segments)*math.sin(j*math.pi/2/segments),cz+sz*radial*math.sin(i*math.pi/2/segments)) for i in range(segments+1) for j in range(segments+1)))
                add(f"shell-spherical-{sx}-{sy}-{sz}",_skin_patch(grids[0],grids[1],segments,segments))
    return parts


def _shell_parts(t, layout, offset, thickness, count, role, material):
    if role == "shell":
        return _rounded_casing_parts(t,layout,thickness,count,material)
    casing = capsule_casing(t)
    w,l,h,r = casing.width,_param(t,"length"),casing.height,_param(t,"corner_radius")
    u, v, hr = _param(t, "window_u"), _param(t, "window_v"), _param(t, "window_diameter") / 2 + (_detail_value("frame_width") if role == "lining" else _detail_value("steel_skin_thickness"))
    family = _family(layout)
    side_window = family["window_wall"] == "right"
    side_door = family["entry_wall"] != "back"
    dx = family["entry_coordinate"]["value"] if not side_door else 0
    dw, dh = _detail_value("door_width"), _detail_value("door_height")
    finish = _detail_value("floor_finish_height")
    xmin,xmax,ymin,ymax,zmin,zmax = casing.left+offset,casing.right-offset,casing.back+offset,casing.front-offset,offset,h-offset
    rr = max(.01,r-offset)
    if role == "lining":
        usable = capsule_interior(t)
        xmin,xmax,ymin,ymax,zmin,zmax = usable.left-thickness,usable.right+thickness,usable.back-thickness,usable.front+thickness,usable.bottom-thickness,usable.top+thickness
        rr = _interior_radius(t)+thickness
    profile = tuple((x+(xmin+xmax)/2,z+zmin) for x,z in _profile(xmax-xmin,zmax-zmin,rr,_capsule_quarters(t,count)))
    front = _perforated_profile(profile,(u,v),hr,ymax-thickness,ymax,count) if not side_window else _rings((_at_depth(profile,ymax-thickness),_at_depth(profile,ymax)),False)
    front = _closed(front.vertices, front.faces, {"aperture_radius": hr if not side_window else None, "wall_thickness": thickness})
    parts = [NamedPart(role + "-front", role, front, material)]
    back_hole = None if side_door else ("rect", dx - dw / 2, dx + dw / 2, finish, finish + dh)
    back = _profile_panel(profile, ymin, ymin + thickness, back_hole)
    parts.append(NamedPart(role + "-back", "shell_back" if role == "shell" else role, back, material))
    for side, x in (("left", xmin), ("right", xmax - thickness)):
        entry = side_door and family["entry_wall"] == side
        if entry:
            centre = family["entry_coordinate"]["value"]
            lo, hi = centre - dw / 2, centre + dw / 2
            strips = [(ymin, lo, zmin+rr, zmax-rr), (hi, ymax, zmin+rr, zmax-rr), (lo, hi, finish+dh, zmax-rr)]
            if finish > zmin+rr:
                strips.append((lo, hi, zmin+rr, finish))
            for i, bounds in enumerate(strips):
                if bounds[1] - bounds[0] <= 1e-9 or bounds[3] - bounds[2] <= 1e-9:
                    continue
                panel = _swapped_xy(_panel(bounds, 0, thickness), x)
                parts.append(NamedPart(f"{role}-{side}-{i}", role, panel, material))
            continue
        hole = ("circle", l / 2 + family["window_longitudinal_offset"]["value"] + u, v, hr) if side_window and side == "right" else None
        panel = _swapped_xy(_panel((ymin, ymax, zmin+rr, zmax-rr), 0, thickness, hole, count=count), x)
        parts.append(NamedPart(role + "-" + side, role, panel, material))
    for side, z in (("floor", zmin), ("roof", zmax-thickness)):
        if role == "lining" and side == "floor":
            continue
        parts.append(NamedPart(role + "-" + side, role, _box(xmin + rr, xmax - rr, ymin, ymax, z, z + thickness), material))
    segments = _capsule_quarters(t,count)
    for corner, (cx, cz, start) in enumerate(((xmax - rr, zmin+rr, -90), (xmax - rr, zmax-rr, 0), (xmin + rr, zmax-rr, 90), (xmin + rr, zmin+rr, 180))):
        outer = [(cx + rr * math.cos(_rad(start + 90 * i / segments)), cz + rr * math.sin(_rad(start + 90 * i / segments))) for i in range(segments + 1)]
        inner = [(cx + (rr - thickness) * math.cos(_rad(start + 90 * i / segments)), cz + (rr - thickness) * math.sin(_rad(start + 90 * i / segments))) for i in range(segments, -1, -1)]
        profile = tuple(outer + inner)
        lower_entry = side_door and ((family["entry_wall"] == "left" and corner == 3) or (family["entry_wall"] == "right" and corner == 0))
        if lower_entry and finish < zmin+rr:
            centre = family["entry_coordinate"]["value"]
            lo, hi = centre - dw / 2, centre + dw / 2
            sections = ((profile, ymin, lo), (_clip_polygon(profile, 1, finish, False), lo, hi), (profile, hi, ymax))
        else:
            sections = ((profile, ymin, ymax),)
        for section, (polygon, y0, y1) in enumerate(sections):
            if len(polygon) < 3 or y1 - y0 <= 1e-9:
                continue
            shape = _rings((_at_depth(polygon, y0), _at_depth(polygon, y1)), False)
            parts.append(NamedPart(f"{role}-corner-{corner}-{section}", role, shape, material))
    return [NamedPart(p.key,p.role,_positive_material(p.solid.vertices,p.solid.faces,p.solid.meta),p.material) for p in parts]


def _layout_door(solid, layout, t):
    family = _family(layout)
    entry = capsule_entry(t,layout)
    if family["entry_wall"] == "back":
        entry = Plane((0,entry.origin[1],0),entry.x,entry.y,entry.z)
    return place(solid,entry)


def _layout_window(solid, layout, t):
    family = _family(layout)
    if family["window_wall"] != "right":
        return solid
    casing = capsule_casing(t)
    centre = _param(t,"length")/2 + family["window_longitudinal_offset"]["value"]
    return _closed(tuple((casing.right+b-casing.front,centre+a,c) for a,b,c in solid.vertices),tuple(tuple(reversed(f)) for f in solid.faces),solid.meta)


def slot_assembly(slot: Slot, detail="exterior") -> Assembly:
    """🏢 Actual family geometry placed in its authored compact slot frame."""
    return place_assembly(capsule_assembly(capsule_type(slot.type), detail, slot.layout), slot.plane)



def bridge_elevation(floor) -> float:
    """🌉 Bridge deck at the source attachment threshold for actual floors 6, 9 and 12."""
    return podium_height() + (floor - 3) * _v("core.storey_height") + _detail_value("floor_finish_height")


def _polygon_prism(points, z0, z1):
    if sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(points, points[1:] + points[:1])) < 0:
        points = tuple(reversed(points))
    n = len(points)
    vertices = tuple((x, y, z) for z in (z0, z1) for x, y in points)
    faces = [tuple(range(n - 1, -1, -1)), tuple(range(n, 2 * n))]
    faces.extend((i, (i + 1) % n, (i + 1) % n + n, i + n) for i in range(n))
    return _closed(vertices, faces)


def _wall_strip(a, b, z0, z1, thickness):
    length = math.dist(a, b)
    nx, ny = (b[1] - a[1]) / length * thickness / 2, (a[0] - b[0]) / length * thickness / 2
    return _polygon_prism(((a[0] + nx, a[1] + ny), (b[0] + nx, b[1] + ny), (b[0] - nx, b[1] - ny), (a[0] - nx, a[1] - ny)), z0, z1)


def _clip_polygon(points, axis, limit, above):
    out = []
    for a, b in zip(points, points[1:] + points[:1]):
        ina = a[axis] >= limit - 1e-9 if above else a[axis] <= limit + 1e-9
        inb = b[axis] >= limit - 1e-9 if above else b[axis] <= limit + 1e-9
        if ina:
            out.append(a)
        if ina != inb:
            fraction = (limit - a[axis]) / (b[axis] - a[axis])
            out.append(tuple(a[k] + fraction * (b[k] - a[k]) for k in range(2)))
    return tuple(out)


def _pierced_slab(polygon, z0, z1):
    holes = tuple((t.center[0] - t.core_side / 2, t.center[0] + t.core_side / 2, t.center[1] - t.core_side / 2, t.center[1] + t.core_side / 2) for t in towers())
    xs = sorted({min(x for x, _ in polygon), max(x for x, _ in polygon), *(v for h in holes for v in h[:2])})
    ys = sorted({min(y for _, y in polygon), max(y for _, y in polygon), *(v for h in holes for v in h[2:])})
    out = []
    for x0, x1 in zip(xs, xs[1:]):
        for y0, y1 in zip(ys, ys[1:]):
            if any(a < (x0 + x1) / 2 < b and c < (y0 + y1) / 2 < d for a, b, c, d in holes):
                continue
            cell = polygon
            for axis, bound, above in ((0, x0, True), (0, x1, False), (1, y0, True), (1, y1, False)):
                if not cell:
                    break
                cell = _clip_polygon(cell, axis, bound, above)
            area = abs(sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(cell, cell[1:] + cell[:1]))) / 2 if len(cell) >= 3 else 0
            if area > 1e-7:
                out.append(_polygon_prism(cell, z0, z1))
    return tuple(out)


def foundation_assembly() -> Assembly:
    """🧱 Assumed basement, core footings and lift pits reconstructed separately for structural cutaways."""
    p = _data()["podium"]
    polygon = tuple(tuple(v["value"] for v in q) for q in p["footprint"])
    depth = _v("foundation.basement_depth")
    parts = []
    for i, solid in enumerate(_pierced_slab(polygon, -depth - .25, -depth)):
        parts.append(NamedPart(f"basement-floor-{i}", "basement_floor", solid, "concrete"))
    for i, (a, b) in enumerate(zip(polygon, polygon[1:] + polygon[:1])):
        parts.append(NamedPart(f"basement-wall-{i}", "basement_wall", _wall_strip(a, b, -depth, 0, .25), "concrete"))
    for t in towers():
        cx, cy = t.center
        side, thickness = _v("foundation.footing_side") / 2, _v("foundation.footing_depth")
        footing_bounds = ((cx - side, cx - .9, cy - side, cy + side), (cx + .9, cx + side, cy - side, cy + side), (cx - .9, cx + .9, cy - side, cy - .9), (cx - .9, cx + .9, cy + .9, cy + side))
        for i, bounds in enumerate(footing_bounds):
            parts.append(NamedPart(f"{t.key}-footing-{i}", "foundation", _box(*bounds, -depth - thickness, -depth), "concrete"))
        s, wall = t.core_side / 2, _detail_value("wall_thickness")
        for side_name, bounds in (("north", (cx - s, cx + s, cy + s - wall, cy + s)), ("east", (cx + s - wall, cx + s, cy - s + wall, cy + s - wall)), ("west", (cx - s, cx - s + wall, cy - s + wall, cy + s - wall))):
            parts.append(NamedPart(f"{t.key}-basement-core-{side_name}", "basement_core_wall", _box(*bounds, -depth, 0), "concrete"))
        entrance = _panel((cx - s, cx + s, -depth, 0), cy - s, cy - s + wall, ("rect", cx - .55, cx + .55, -depth + .02, -depth + 2.12))
        parts.append(NamedPart(t.key + "-basement-core-south", "basement_core_wall", entrance, "concrete"))
        floor_bounds = ((cx - s + wall, cx - .9, cy - s + wall, cy + s - wall), (cx + .9, cx + s - wall, cy - s + wall, cy + s - wall), (cx - .9, cx + .9, cy - s + wall, cy - .9), (cx - .9, cx + .9, cy + .9, cy + s - wall))
        for i, bounds in enumerate(floor_bounds):
            parts.append(NamedPart(f"{t.key}-basement-core-floor-{i}", "basement_floor", _box(*bounds, -depth - .25, -depth), "concrete"))
        pit = _v("foundation.lift_pit_depth")
        parts.append(NamedPart(t.key + "-lift-pit", "lift_pit", _box(cx - .9, cx + .9, cy - .9, cy + .9, -depth - pit, -depth - pit + .2), "concrete"))
        for part in _stair_ring(t, (-depth, 0), "full"):
            parts.append(NamedPart(t.key + "-basement-" + part.key, part.role, part.solid, part.material))
        leg, inside = _v("core.profile.pier_leg"), s - wall
        for index, (sx, sy) in enumerate(((1, 1), (-1, 1), (-1, -1), (1, -1))):
            points = ((sx * inside, sy * inside), (sx * (inside - leg), sy * inside), (sx * inside, sy * (inside - leg)))
            parts.append(NamedPart(f"{t.key}-basement-corner-pier-{index}", "basement_core_pier", place(_polygon_prism(points, -depth, 0), _core_frame(t)), "concrete"))
        for part in _lift_parts(t, -depth, 0, (("B1", -depth),)):
            parts.append(NamedPart(t.key + "-basement-" + part.key, part.role, part.solid, part.material))
        for i, bounds in enumerate(((cx - .9, cx - .7, cy - .9, cy + .9), (cx + .7, cx + .9, cy - .9, cy + .9), (cx - .7, cx + .7, cy - .9, cy - .7), (cx - .7, cx + .7, cy + .7, cy + .9))):
            parts.append(NamedPart(f"{t.key}-pit-wall-{i}", "lift_pit", _box(*bounds, -depth - pit, -depth), "concrete"))
    for division in _data()["foundation"]["section_divisions"]:
        bounds = tuple(q["value"] for q in division["bounds"])
        solid = _box(*bounds, -depth, -.3, {"layout_basis": "inferred_section_visible_division", "equipment_identity": "unverified"})
        parts.append(NamedPart(division["key"], "basement_partition", solid, "concrete"))
    return Assembly(tuple(parts))

def _fixture_loop(cx, cy, rx, ry, z, count, power=1, angle=0):
    """🥚 Sample a rotated elliptical or softly rectangular sanitary profile."""
    result = []
    c, s = math.cos(angle), math.sin(angle)
    for i in range(count):
        a = 2 * math.pi * i / count
        x = rx * math.copysign(abs(math.cos(a)) ** power, math.cos(a))
        y = ry * math.copysign(abs(math.sin(a)) ** power, math.sin(a))
        result.append((cx + c*x - s*y, cy + s*x + c*y, z))
    return tuple(result)


def _fixture_bowl(cx, cy, rx, ry, bottom, rim, wall, power=1, angle=0):
    """🛁 Closed material with an open tapered cavity and a continuous solid floor."""
    n = int(_v("capsule.detail.segments"))
    loops = (_fixture_loop(cx, cy, rx*.95, ry*.95, bottom, n, power, angle),
             _fixture_loop(cx, cy, rx, ry, rim, n, power, angle),
             _fixture_loop(cx, cy, rx-wall, ry-wall, rim, n, power, angle),
             _fixture_loop(cx, cy, rx*.55, ry*.55, bottom+.07, n, power, angle))
    solid = _rings(loops, False)
    return _closed(solid.vertices, tuple(tuple(reversed(f)) for f in solid.faces), {"cavity": True, "wall": wall, "rim": rim, "floor": bottom+.07})


def _fixture_tube(points, radius, segments=8):
    """🚿 Sweep a closed circular tube along a curved three-dimensional plumbing centreline."""
    loops = []
    for i, p in enumerate(points):
        a, b = points[max(0,i-1)], points[min(len(points)-1,i+1)]
        delta = tuple(b[k]-a[k] for k in range(3))
        magnitude = math.sqrt(sum(q*q for q in delta))
        tangent = tuple(q/magnitude for q in delta)
        reference = (0,0,1) if abs(tangent[2]) < .9 else (0,1,0)
        cross = (tangent[1]*reference[2]-tangent[2]*reference[1], tangent[2]*reference[0]-tangent[0]*reference[2], tangent[0]*reference[1]-tangent[1]*reference[0])
        magnitude = math.sqrt(sum(q*q for q in cross))
        u = tuple(q/magnitude for q in cross)
        v = (tangent[1]*u[2]-tangent[2]*u[1], tangent[2]*u[0]-tangent[0]*u[2], tangent[0]*u[1]-tangent[1]*u[0])
        loops.append(tuple(tuple(p[k]+radius*(u[k]*math.cos(2*math.pi*j/segments)+v[k]*math.sin(2*math.pi*j/segments)) for k in range(3)) for j in range(segments)))
    solid = _rings(loops, False)
    return _closed(solid.vertices, tuple(tuple(reversed(f)) for f in solid.faces), {"tube_radius": radius})


def _reference_fixtures():
    """🚽 Domus 520 reference wetroom: contoured hollow sanitary ware and inferred working hardware."""
    parts = []
    def add(key, role, solid, material="ceramic"):
        parts.append(NamedPart(key, role, solid, material))
    def tube(key, role, points, radius=None):
        add(key, role, _fixture_tube(points, radius or _v("capsule.detail.pipe_radius")), "metal")
    add("tub-bowl", "bathtub", _fixture_bowl(-.61,.575,.47,.305,.29,.8,_v("capsule.detail.tub_wall"),.45))
    add("washbasin", "washbasin", _fixture_bowl(-.24,.99,.18,.18,.79,.98,_v("capsule.detail.basin_wall")))
    add("toilet-pan", "toilet", _fixture_bowl(-.89,1.51,.19,.24,.36,.66,_v("capsule.detail.toilet_wall"),1,-.6))
    seat_loops = tuple(_fixture_loop(-.89,1.51,rx,ry,z,32,1,-.6) for rx,ry,z in ((.203,.253,.663),(.203,.253,.687),(.169,.219,.687),(.169,.219,.663)))
    seat = _rings(seat_loops)
    add("toilet-seat", "toilet", _closed(seat.vertices,tuple(tuple(reversed(f)) for f in seat.faces)))
    add("toilet-pedestal", "toilet", _cylinder((-.89,1.51,.245),.1,.115,"z",24))
    add("toilet-cistern", "toilet", _box(-1.06,-.84,1.68,1.79,.38,.94))
    add("toilet-flush", "tap", _cylinder((-.83,1.735,.87),.018,.018,"x",16), "metal")
    for key, cx, cy, z, radius in (("tub",-.61,.575,.365,.025),("basin",-.24,.99,.865,.017),("toilet",-.89,1.51,.435,.035)):
        add(key+"-drain", "drain", _cylinder((cx,cy,z),radius,.004,"z",24), "dark")
        for j in range(2):
            add(key+f"-drain-cross-{j}", "drain", _box(cx-radius,cx+radius,cy-.002,cy+.002,z+.004+j*.002,z+.006+j*.002) if j==0 else _box(cx-.002,cx+.002,cy-radius,cy+radius,z+.006,z+.008), "metal")
    for key,cx,cy,z in (("basin",-.24,.82,1.0),("tub",-.61,.265,.88)):
        path = tuple((cx,cy+.09*(1-math.cos(math.pi*i/24)),z+.16*math.sin(math.pi*i/24)) for i in range(25))
        tube("tap-"+key+"-spout", "tap", path)
        for side in (-1,1):
            x = cx+side*.085
            add(f"tap-{key}-valve-{side}","tap",_cylinder((x,cy,z-.03),.02,.05,"z",16),"metal")
            tube(f"tap-{key}-handle-{side}","tap",((x-.033,cy,z+.02),(x+.033,cy,z+.02)),.006)
    tube("basin-trap", "plumbing", ((-.24,.99,.79),(-.24,.99,.67)) + tuple((-.24,1.045-.055*math.cos(math.pi*i/24),.67-.075*math.sin(math.pi*i/24)) for i in range(1,25)) + ((-.24,1.10,.75),(-.24,.23,.75)))
    tube("tub-waste", "plumbing", ((-.61,.575,.29),(-.61,.575,.255),(-.61,.23,.255)))
    tube("toilet-waste", "plumbing", tuple((-.89,1.51+.12*i/24,.36-.09*math.sin(math.pi*i/48)) for i in range(25)),.035)
    tube("shower-riser", "shower", ((-.95,.245,.87),(-.95,.245,1.87),(-.95,.32,1.95)))
    add("shower-rose", "shower", _cylinder((-.95,.32,1.94),_v("capsule.detail.shower_radius"),.018,"z",24), "metal")
    for i in range(12):
        a = i*math.pi/6
        add(f"shower-nozzle-{i}","shower",_cylinder((-.95+.035*math.cos(a),.32+.035*math.sin(a),1.93),.003,.01,"z",8),"dark")
    tube("shower-hose", "shower", tuple((-.78+.065*math.sin(2*math.pi*i/40),.25+.03*math.sin(math.pi*i/40),.9+.65*i/40) for i in range(41)),.007)
    tube("shower-handset-grip", "shower", ((-.78,.25,1.55),(-.78,.26,1.67)),.012)
    add("shower-handset-head", "shower", _cylinder((-.78,.26,1.68),.027,.018,segments=20), "metal")
    for i in range(6):
        a = i*math.pi/3
        add(f"shower-handset-jet-{i}","shower",_cylinder((-.78+.017*math.cos(a),.259,1.68+.017*math.sin(a)),.002,.002,segments=8),"dark")
    return tuple(parts)


def _reference_media():
    """📺 Model the documented integrated CRT, audio panels, open grilles and wired telephone."""
    parts = []
    def add(key, role, solid, material="metal"):
        parts.append(NamedPart(key,role,solid,material))
    add("tv-bezel", "tv", _box(.736,.78,1.78,2.44,1.30,1.86), "shell")
    loops = []
    for radius,x in ((1,.733),(.75,.722),(.35,.717),(.1,.715)):
        loop = _fixture_loop(2.06,1.58,.235*radius,.205*radius,0,32,.4)
        loops.append(tuple((x,y,z) for y,z,_ in loop))
    lens = _rings(loops,False)
    add("tv-screen","tv",lens,"dark")
    for i,z in enumerate((1.44,1.69)):
        add(f"tv-knob-{i}","television_control",_cylinder((.707,2.39,z),.032,.035,"x",20))
        add(f"tv-knob-index-{i}","television_control",_box(.703,.707,2.387,2.393,z+.016,z+.029),"shell")
    add("audio-console","audio_console",_box(.733,.778,1.2,1.68,1.08,1.32),"dark")
    for i in range(8):
        add(f"audio-button-{i}","audio_control",_box(.721,.733,1.22+i*.053,1.247+i*.053,1.10,1.13),"shell")
    for i in range(3):
        add(f"audio-knob-{i}","audio_control",_cylinder((.714,1.26+i*.17,1.245),.022,.02,"x",16))
    for speaker,y in enumerate((1.02,2.55)):
        add(f"speaker-cone-{speaker}","speaker",_cylinder((.739,y,1.63),.105,.025,"x",24),"dark")
        for i in range(-5,6):
            offset = i*_v("capsule.detail.grille_pitch")
            half = math.sqrt(max(0,.105**2-offset**2))
            add(f"speaker-grille-{speaker}-{i}","speaker_grille",_box(.728,.734,y+offset-.002,y+offset+.002,1.63-half,1.63+half))
    add("telephone-base","telephone",_box(.36,.60,1.38,1.62,.91,.97),"dark")
    add("telephone-dial","telephone",_cylinder((.48,1.50,.97),.065,.012,"z",24),"metal")
    for i in range(10):
        a = i*2*math.pi/10
        add(f"telephone-dial-hole-{i}","telephone",_cylinder((.48+.046*math.cos(a),1.50+.046*math.sin(a),.982),.009,.002,"z",12),"dark")
    add("telephone-handset","telephone",_fixture_tube(tuple((.38+.20*i/24,1.42,.995+.035*math.sin(math.pi*i/24)) for i in range(25)),.021),"shell")
    for i,x in enumerate((.38,.58)):
        add(f"telephone-earpiece-{i}","telephone",_cylinder((x,1.42,.984),.035,.027,"z",20),"shell")
    add("telephone-cord","telephone",_fixture_tube(tuple((.6+.019*math.cos(i*math.pi/3),1.43+.019*math.sin(i*math.pi/3),.96-.25*i/96) for i in range(97)),.003),"dark")
    for i in range(18):
        add(f"vent-grille-{i}","vent",_box(.77,.79,1.20+i*.055,1.225+i*.055,2.16,2.22),"metal")
    for i in range(2):
        add(f"shelf-bottle-rail-{i}","shelf",_fixture_tube(((.56,.3,1.99+i*.16),(.56,2.7,1.99+i*.16)),.006))
    return tuple(parts)


def _reference_mounts(parts):
    """🔧 Inferred discrete fasteners and supports connect documented fixtures without filling their functional voids."""
    result = []
    by_key = {p.key:p.solid for p in parts}
    def add(key, solid, material="metal"):
        meta = {**solid.meta,"basis":"assumed","source":"domus","detail":"fixture connection"}
        result.append(NamedPart("fixture-mount-"+key,"fixture_mount",_closed(solid.vertices,solid.faces,meta),material))
    def box(key, bounds, material="metal"):
        add(key,_box(*bounds),material)
    def tube(key, points, radius=.006, material="metal"):
        add(key,_fixture_tube(points,radius),material)
    def annulus(key,x0,x1,y,z,inner,outer):
        loops = tuple(tuple((x,a,b) for a,b,_ in _fixture_loop(y,z,r,r,0,32)) for x,r in ((x1,outer),(x0,outer),(x0,inner),(x1,inner)))
        add(key,_rings(loops))
    loops = tuple(tuple((x,y,z) for y,z,_ in _fixture_loop(2.06,1.58,.235,.205,0,32,.4)) for x in (.736,.733))
    add("tv",_rings(loops,False),"dark")
    box("audio",(.778,.78,1.2,1.68,1.08,1.32),"dark")
    for i in range(2):
        cone = by_key[f"speaker-cone-{i}"]
        y = (min(v[1] for v in cone.vertices)+max(v[1] for v in cone.vertices))/2
        annulus(f"speaker-{i}-driver",.764,.78,y,1.63,.08,.112)
        annulus(f"speaker-{i}-grille",.728,.739,y,1.63,.100,.112)
        rail = by_key[f"shelf-bottle-rail-{i}"]
        y0,y1 = min(v[1] for v in rail.vertices),max(v[1] for v in rail.vertices)
        for j,y in enumerate((y0+.03,y1-.03)):
            tube(f"shelf-{i}-{j}",((.56,y,1.965+i*.16),(.56,y,1.99+i*.16)))
    for i,(x,y) in enumerate(((-.86,.435),(-.36,.435),(-.86,.715),(-.36,.715))):
        add(f"tub-{i}",_cylinder((x,y,.245),.025,.045,"z",16))
    for i,y in enumerate((.9,1.08)):
        box(f"basin-arm-{i}",(-.16,.055,y-.015,y+.015,.765,.79))
        box(f"basin-wall-{i}",(.03,.055,y-.015,y+.015,.73,.86))
        tube(f"cistern-{i}",((-1.095,1.71+i*.05,.53+i*.29),(-1.06,1.71+i*.05,.53+i*.29)),.012)
        tube(f"riser-{i}",((-.95,.225,1.05+i*.67),(-.95,.245,1.05+i*.67)),.009)
        a = math.pi/3
        tube(f"wetroom-handle-{i}",((-.47+.55*math.cos(a),1.3+.55*math.sin(a),.97+i*.08),(-.47+.585*math.cos(a),1.3+.585*math.sin(a),.97+i*.08)),.006)
    for i in range(4):
        a=i*math.pi/2
        x,y=.1795*math.cos(a),.2295*math.sin(a)
        x,y=-.89+math.cos(-.6)*x-math.sin(-.6)*y,1.51+math.sin(-.6)*x+math.cos(-.6)*y
        add(f"seat-{i}",_cylinder((x,y,.66),.003,.003,"z",12),"dark")
    for key,cx,cy,z in (("basin",-.24,.82,1.0),("tub",-.61,.265,.88)):
        box(f"tap-{key}-mixer",(cx-.105,cx+.105,cy-.015,cy+.015,z-.025,z+.005))
    for i,x in enumerate((-.695,-.525)):
        add(f"tap-tub-leg-{i}",_cylinder((x,.29,.79),.015,.07,"z",16))
    tube("flush",((-.84,1.735,.87),(-.83,1.735,.87)))
    tube("basin-outlet",((-.24,.225,.75),(-.24,.23,.75)),.009)
    tube("tub-outlet",((-.61,.225,.255),(-.61,.23,.255)),.009)
    tube("hose-outlet",((-.78,.225,.9),(-.78,.25,.9)),.01)
    tube("handset",((-.78,.225,1.61),(-.78,.255,1.61)),.012)
    tube("phone-head",((.619,1.43,.96),(.628,1.43,.985),(.607,1.42,.998)),.003,"dark")
    tube("phone-base",((.619,1.43,.71),(.635,1.43,.69),(.65,1.43,.72),(.65,1.43,.94),(.6,1.43,.94)),.003,"dark")
    return tuple(result)


def _fit_reference_cavity(solid, profile):
    """✂️ Trim fixture material at the actual faceted lining corners while retaining its floor contact."""
    polygons = [tuple(solid.vertices[i] for i in f) for f in solid.faces]
    clipped = False
    for (ax,az),(bx,bz) in zip(profile,profile[1:]+profile[:1]):
        nx,nz=bz-az,ax-bx
        length=math.hypot(nx,nz)
        nx,nz=nx/length,nz/length
        limit=nx*ax+nz*az
        if all(nx*x+nz*z<=limit+1e-10 for polygon in polygons for x,y,z in polygon):
            continue
        clipped=True
        output=[]
        boundary={}
        for polygon in polygons:
            cut=[]
            for a,b in zip(polygon,polygon[1:]+polygon[:1]):
                da,db=nx*a[0]+nz*a[2]-limit,nx*b[0]+nz*b[2]-limit
                if da<=1e-10:
                    cut.append(a)
                if (da < -1e-10 and db > 1e-10) or (da > 1e-10 and db < -1e-10):
                    f=da/(da-db)
                    cut.append(tuple(a[k]+f*(b[k]-a[k]) for k in range(3)))
            clean=[]
            for p in cut:
                if not clean or sum((p[k]-clean[-1][k])**2 for k in range(3))>1e-20:
                    clean.append(p)
            if len(clean)>1 and sum((clean[0][k]-clean[-1][k])**2 for k in range(3))<1e-20:
                clean.pop()
            if len(clean)>=3:
                output.append(tuple(clean))
                for p in clean:
                    if abs(nx*p[0]+nz*p[2]-limit)<1e-9:
                        boundary[tuple(round(v,10) for v in p)]=p
        if not output:
            raise ValueError("Reference fixture is outside the usable capsule cavity")
        points=tuple(boundary.values())
        if len(points)>=3:
            cy=sum(p[1] for p in points)/len(points)
            cv=sum(-nz*p[0]+nx*p[2] for p in points)/len(points)
            output.append(tuple(sorted(points,key=lambda p:math.atan2(-nz*p[0]+nx*p[2]-cv,p[1]-cy))))
        polygons=output
    if not clipped:
        return solid
    vertices=[]
    indices={}
    faces=[]
    for polygon in polygons:
        face=[]
        for p in polygon:
            key=tuple(round(v,10) for v in p)
            if key not in indices:
                indices[key]=len(vertices)
                vertices.append(p)
            face.append(indices[key])
        faces.append(tuple(face))
    return _closed(vertices,faces,{**solid.meta,"cavity_fit":"actual lining profile"})


def _reference_interior(t, layout):
    usable = capsule_interior(t)
    w, l, h = 2.5, 4.0, 2.5
    floor = .2
    parts = []
    def box(key, role, bounds, material="shell"):
        parts.append(NamedPart(key, role, _box(*bounds), material))
    def add(key, role, solid, material="ceramic"):
        parts.append(NamedPart(key, role, solid, material))
    box("wetroom-floor", "bathroom", (-1.12, .08, .2, 1.85, floor, floor + .045), "ceramic")
    for key, bounds in (("back", (-1.12, .08, .2, .225, floor, 2.12)), ("left", (-1.12, -1.095, .2, 1.85, floor, 2.12)), ("right", (.055, .08, .2, 1.3, floor, 2.12))):
        box("wetroom-" + key, "wetroom_enclosure", bounds, "ceramic")
    add("wetroom-front", "wetroom_enclosure", _panel((-1.12, .08, floor, 2.12), 1.825, 1.85, ("rect", -.58, .03, floor + .02, 1.98)))
    arc = tuple((-.47 + radius * math.cos(_rad(angle)), 1.3 + radius * math.sin(_rad(angle))) for radius, angles in ((.55, range(0, 91, 10)), (.525, range(90, -1, -10))) for angle in angles)
    add("wetroom-curved-door", "wetroom_door", _polygon_prism(arc, floor + .02, 1.98))
    add("wetroom-handle", "door_handle", _fixture_tube(tuple((-.47+.585*math.cos(math.pi/3),1.3+.585*math.sin(math.pi/3),z) for z in (.95,1.07)),.007), "metal")
    parts.extend(_reference_fixtures())
    box("bed-platform", "floor_bed", (-1.12, 1.12, l - 1.2, l - .12, .38, .4), "wood")
    box("bed-front-rail", "bed", (-1.12, 1.12, l - 1.2, l - 1.16, floor, .38), "wood")
    for i in range(4):
        x = -1 + .5 * i
        box(f"bed-drawer-{i}", "bed_drawer", (x, x + .48, l - 1.205, l - .605, .22, .36), "wood")
        box(f"drawer-handle-{i}", "door_handle", (x + .15, x + .33, l - 1.22, l - 1.205, .28, .3), "metal")
    box("bed-mattress", "mattress", (-1.08, 1.08, l - 1.16, l - .12, .4, .55), "fabric")
    for i, x in enumerate((-.98, .12)):
        box(f"bed-pillow-{i}", "pillow", (x, x + .86, l - .55, l - .18, .55, .64), "fabric")
    box("media-wall", "media_cabinet", (.78, 1.11, .25, l - 1.25, floor, h - .27), "shell")
    box("media-storage", "cabinet", (.74, .78, .25, .9, .35, 1.0), "shell")
    box("folding-desk", "desk", (.27, .78, 1.12, 2.18, .87, .91), "shell")
    parts.extend(_reference_media())
    for i, z in enumerate((1.94, 2.1)):
        box(f"media-shelf-{i}", "shelf", (.53, .78, .25, l - 1.25, z, z + .025), "shell")
    add("ceiling-light", "light", _cylinder((0, 2.14, 2.275), .16, .025, "z", 24), "shell")
    add("smoke-detector", "detector", _cylinder((-.5, 1.12, 2.28), .045, .02, "z", 16), "shell")
    if layout in ("J", "L", "S"):
        adjusted = []
        for p in parts:
            if p.key == "media-storage":
                continue
            solid = p.solid
            if p.role in ("media_cabinet", "shelf", "audio_console"):
                lower = min(v[1] for v in solid.vertices)
                if lower < 1.2:
                    solid = _closed(tuple((x, 1.2 if abs(y - lower) < 1e-8 else y, z) for x, y, z in solid.vertices), solid.faces, solid.meta)
            if p.key.startswith(("speaker-cone-0", "speaker-grille-0")):
                solid = _closed(tuple((x, y + .4, z) for x, y, z in solid.vertices), solid.faces, solid.meta)
            adjusted.append(NamedPart(p.key, p.role, solid, p.material))
        parts = adjusted
    parts.extend(_reference_mounts(parts))
    if _family(layout)["hand"] == "left":
        parts = [NamedPart(p.key, p.role, _closed(tuple((-x, y, z) for x, y, z in p.solid.vertices), tuple(tuple(reversed(f)) for f in p.solid.faces), p.solid.meta), p.material) for p in parts]
    factors = (usable.width/2.3,usable.length/3.8,usable.height/2.1)
    parts = [NamedPart(p.key,p.role,_closed(tuple((x*factors[0],usable.back+(y-.1)*factors[1],usable.bottom+(z-.2)*factors[2]) for x,y,z in p.solid.vertices),p.solid.faces,p.solid.meta),p.material) for p in parts]
    profile=capsule_interior_profile(t)
    return tuple(NamedPart(p.key,p.role,_fit_reference_cavity(p.solid,profile),p.material) for p in parts)


def _face_slab(points, thickness):
    a, b, c = points[:3]
    u, v = tuple(y - x for x, y in zip(a, b)), tuple(y - x for x, y in zip(a, c))
    normal = (u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0])
    length = math.sqrt(sum(q * q for q in normal))
    shift = tuple(-q / length * thickness for q in normal)
    n = len(points)
    vertices = tuple(points) + tuple(tuple(p[k] + shift[k] for k in range(3)) for p in points)
    faces = [tuple(range(n)), tuple(range(2 * n - 1, n - 1, -1))]
    faces.extend((i, i + n, (i + 1) % n + n, (i + 1) % n) for i in range(n))
    return _closed(vertices, faces)



def capsule_families() -> tuple:
    """💊 Physical door/window families from the shared schema catalog, independent from proposed room-program studies."""
    return tuple(f["key"] for f in _data()["families"])


def _family(layout):
    return next(f for f in _data()["families"] if f["key"] == layout)


def _study_interior(t, layout):
    usable = capsule_interior(t)
    ix,iy,h = usable.right,usable.front,2.5
    floor = usable.bottom
    fittings = _param(t, "fittings")
    side = _family(layout)["entry_wall"] != "back"
    sofa = any(f["key"] == "sofa" and f["count"] > 0 for f in fittings)
    parts = []
    for fitting in fittings:
        key, width, length = fitting["key"], fitting["width"], fitting["length"]
        for index in range(fitting["count"]):
            rotated = key == "bed" and side or key == "sofa" and side
            aw, al = (length, width) if rotated else (width, length)
            if key == "bath":
                x, y = -ix + .02, .15 + index * (length + .05)
            elif key == "storage":
                x, y = (-.1, .15) if side and sofa else (-ix + .03, 1.4) if side else (ix - .03 - width, .65)
                y += index * (length + .05)
            elif key == "bed":
                x, y = (-aw / 2, iy - .08 - al) if side else (-ix + .04 + index * (width + .05), iy - .08 - length)
            elif key == "desk":
                x = ix - .03 - width if index % 2 == 0 else -ix + .03
                y = 1.4 if side or index % 2 else 1.9
            elif key == "sofa":
                x, y = (-aw / 2, iy - .02 - al) if side else (-width / 2, iy - .05 - length)
            else:
                x, y = -ix + .03 + index * (width + .05), 1.4
            if aw > usable.width + 1e-9 or al > usable.length + 1e-9:
                raise ValueError(f"Study fitting {key} exceeds the usable interior")
            x = min(max(x,usable.left),usable.right-aw)
            y = min(max(y,usable.back),usable.front-al)
            angle = math.pi / 2 if rotated else 0
            c, sn = math.cos(angle), math.sin(angle)
            plane = Plane((x + aw / 2, y + al / 2, 0), (c, sn, 0), (-sn, c, 0), (0, 0, 1))
            def add(name, role, solid, material, primary=False):
                meta = {**solid.meta, "program_fitting": key, "program_index": index, "primary_footprint": primary}
                solid = _closed(solid.vertices, solid.faces, meta)
                parts.append(NamedPart(f"{key}-{index}-{name}", role, place(solid, plane), material))
            def box(name, role, bounds, material="wood", primary=False):
                add(name, role, _box(*bounds), material, primary)
            x0, x1, y0, y1 = -width / 2, width / 2, -length / 2, length / 2
            if key == "bed":
                box("base", "bed", (x0, x1, y0, y1, floor, floor + .28), primary=True)
                box("mattress", "mattress", (x0 + .015, x1 - .015, y0 + .015, y1 - .015, floor + .28, floor + .43), "fabric")
                box("pillow", "pillow", (x0 + .06, x1 - .06, y1 - .45, y1 - .08, floor + .43, floor + .5), "fabric")
            elif key == "bath":
                box("base", "bathroom", (x0, x1, y0, y1, floor, floor + .08), "ceramic", True)
                box("partition", "bathroom", (x1 - .05, x1, y0, y1, floor, min(h - .25, 2.05)), "shell")
                box("end", "bathroom", (x0, x1 - .05, y1 - .05, y1, floor, min(h - .25, 2.05)), "shell")
                box("washbasin", "washbasin", (x0 + .1, x0 + min(.5, width * .5), y0 + .1, y0 + min(.4, length * .35), floor + .65, floor + .78), "ceramic")
                add("toilet", "toilet", _cylinder((width * .2, length * .15, floor), min(width, length) * .16, .4, "z", 16), "ceramic")
                box("cistern", "toilet", (.02, min(.4, x1 - .03), y0 + .12, y0 + .28, floor + .25, floor + .75), "ceramic")
            elif key == "storage":
                box("cabinet", "cabinet", (x0, x1, y0, y1, floor, min(h - .3, 2.0)), primary=True)
            elif key == "desk":
                box("top", "desk", (x0, x1, y0, y1, floor + .65, floor + .7), primary=True)
                for corner, (cx, cy) in enumerate(((x0 + .04, y0 + .04), (x1 - .08, y0 + .04), (x0 + .04, y1 - .08), (x1 - .08, y1 - .08))):
                    box(f"leg-{corner}", "desk", (cx, cx + .04, cy, cy + .04, floor, floor + .65), "metal")
                for shelf, z in enumerate((1.25, 1.57)):
                    box(f"shelf-{shelf}", "shelf", (x0, x1, y0, y1, z, z + .035))
            elif key == "sofa":
                box("base", "sofa", (x0, x1, y0, y1, floor, floor + .35), "fabric", True)
                box("back", "sofa", (x1 - min(.15, width / 4), x1, y0, y1, floor + .35, floor + .85), "fabric")
            else:
                box("base", key, (x0, x1, y0, y1, floor, floor + .75), primary=True)
    if _family(layout)["hand"] == "left":
        parts = [NamedPart(p.key, p.role, _closed(tuple((-x, y, z) for x, y, z in p.solid.vertices), tuple(tuple(reversed(f)) for f in p.solid.faces), p.solid.meta), p.material) for p in parts]
    factor = usable.height / _v("capsule.interior.height")
    return tuple(NamedPart(p.key,p.role,_closed(tuple((x,y,floor+(z-floor)*factor) for x,y,z in p.solid.vertices),p.solid.faces,p.solid.meta),p.material) for p in parts)


def _logical_floor_sill(floor, pitch):
    """📏 Recover the normal datum even when a sparse floor contains only staggered capsule entries."""
    return round(min(item["center"][2]["value"] - item["sublevel"] * pitch / 3 + .2 for group in floor["groups"] for item in group["items"]), 6)


def _normal_lift_stops(t):
    stops = [(index + 1, q["value"]) for index, q in enumerate(_data()["podium"]["levels"][:2])]
    for floor in _data()["towers"][t.index]["floors"]:
        z = _logical_floor_sill(floor, t.storey_height)
        stops.append((floor["floor"], round(z, 6)))
    head = _detail_value("lift_door_height")
    return tuple((floor, z) for floor, z in stops if z + head <= t.height)


def _core_frame(t, angle=0):
    """🪞 Reflect the B compound and circulation across X after each canonical A stair rotation."""
    sign = -1 if _data()["core"]["circulation"]["handing"][t.key] == "mirror_x" else 1
    c, s = math.cos(angle), math.sin(angle)
    return Plane((*t.center, 0), (sign * c, s, 0), (-sign * s, c, 0), (0, 0, 1))


def _circulation_parts(t, detail, cancelled):
    """🚶 Follow explicit capsule sill datums and podium levels without fixed-height assumptions."""
    raw = _data()["towers"][t.index]["floors"]
    sparse = _logical_floor_sill(raw[0], t.storey_height)
    regular = sorted({_logical_floor_sill(floor, t.storey_height) for floor in raw[1:]})
    podium = tuple(q["value"] for q in _data()["podium"]["levels"][:2])
    anchors = sorted({round(z, 6) for z in (*podium, sparse, *regular, (regular[-1] if regular else sparse) + t.storey_height, t.height) if 0 <= z <= t.height})
    return _stair_ring(t, anchors, detail, cancelled) + _lift_parts(t, 0, t.height, _normal_lift_stops(t))


def _lift_parts(t, bottom, top, stops):
    """🛗 Clipped compound encloses two unidentified service cells and a separate east lift chamber."""
    x, y = _v("core.profile.compound_width") / 2, _v("core.profile.compound_depth") / 2
    chamfer, wall, partition = (_v("core.profile." + key) for key in ("chamfer", "wall", "partition_x"))
    width, head, slab = _detail_value("lift_door_width"), _detail_value("lift_door_height"), _detail_value("floor_thickness")
    frame, parts = _core_frame(t), []
    outer = ((-x + chamfer, -y), (x - chamfer, -y), (x, -y + chamfer), (x, y - chamfer), (x - chamfer, y), (-x + chamfer, y), (-x, y - chamfer), (-x, -y + chamfer))
    inner = tuple((a - math.copysign(wall, a), b - math.copysign(wall, b)) for a, b in outer)
    def add(key, role, solid, material="concrete"):
        parts.append(NamedPart(key, role, place(solid, frame), material))
    def box(key, role, bounds, material="concrete", meta=None):
        add(key, role, _box(*bounds, meta), material)
    for index in range(8):
        if index == 2:
            continue
        next_index = (index + 1) % 8
        polygon = (outer[index], outer[next_index], inner[next_index], inner[index])
        key = "compound-chamfer" if index % 2 else "compound-wall"
        add(f"{key}-{index}", "lift_shaft", _polygon_prism(polygon, bottom, top))
    box("compound-service-partition", "lift_shaft", (partition - wall / 2, partition + wall / 2, -y + wall, y - wall, bottom, top))
    box("compound-service-divider", "lift_shaft", (-x + wall, partition - wall / 2, -wall / 2, wall / 2, bottom, top))
    for index, bounds in enumerate(((-x + chamfer, partition - wall / 2, -y + wall, -wall / 2), (-x + chamfer, partition - wall / 2, wall / 2, y - wall))):
        box(f"service-compartment-{index}", "service_compartment", (*bounds, bottom - .08, bottom), meta={"unidentified_service": True, "basis": "inferred_primary_plan"})
    for side, (a, b) in enumerate(((-y + chamfer, -width / 2), (width / 2, y - chamfer))):
        box(f"lift-front-jamb-{side}", "lift_shaft", (x - wall, x, a, b, bottom, top))
    cursor = bottom
    for stop, z in stops:
        if z > cursor:
            box(f"lift-front-header-{stop}", "lift_shaft", (x - wall, x, -width / 2, width / 2, cursor, z))
        cursor = max(cursor, z + head)
        box(f"lift-door-{stop}", "lift_door", (x - .04, x - .01, -width / 2, width / 2, z, z + head), "metal")
        box(f"lift-approach-{stop}", "lift_approach", (x, _v("core.circulation.flight_inner") + _v("core.circulation.flight_width"), -.55, .55, z - slab, z), meta={"walking_axis": "x", "lift_stop": stop})
        box(f"lift-call-button-{stop}", "lift_call_button", (x - .02, x, width / 2 + .03, width / 2 + .09, z + 1.05, z + 1.17), "metal")
    if cursor < top:
        box("lift-front-header-top", "lift_shaft", (x - wall, x, -width / 2, width / 2, cursor, top))
    if bottom == 0:
        cabin = _data()["core"]["lift_cabin"]
        w, d, h, cx, cy, skin = (cabin[key]["value"] for key in ("width", "depth", "height", "center_x", "center_y", "wall"))
        z = _data()["podium"]["levels"][cabin["parked_level"]["value"]]["value"]
        x0, x1, y0, y1 = cx - w / 2, cx + w / 2, cy - d / 2, cy + d / 2
        box("reference-cabin-floor", "lift_cabin_floor", (x0, x1, y0, y1, z - .08, z), "metal", {"reference_position": "parked_ground_floor", "mechanism": "unverified_reference_display_support"})
        for index, bounds in enumerate(((x0, x0 + skin, y0, y1), (x0 + skin, x1, y0, y0 + skin), (x0 + skin, x1, y1 - skin, y1))):
            box(f"reference-cabin-wall-{index}", "lift_cabin_wall", (*bounds, z, z + h), "metal")
        box("reference-cabin-ceiling", "lift_cabin_ceiling", (x0, x1, y0, y1, z + h, z + h + skin), "metal")
        for index, (a, b) in enumerate(((y0, cy - .012), (cy + .012, y1))):
            box(f"reference-cabin-door-{index}", "lift_cabin_door", (x1 - skin, x1, a, b, z, z + h), "metal")
        add("reference-cabin-handrail", "lift_cabin_handrail", _beam((x0 + .09, y0 + .07, z + .92), (x0 + .09, y1 - .07, z + .92), .025), "metal")
        box("reference-cabin-control", "lift_call_button", (x1 - .12, x1 - .05, y0 + skin, y0 + skin + .015, z + .9, z + 1.25), "metal")
        for index, xx in enumerate((x0 + .08, x1 - .16)):
            box(f"reference-cabin-support-{index}", "lift_reference_support", (xx, xx + .08, y0 + .08, y1 - .08, -_v("foundation.basement_depth"), z - .08), "metal", {"basis": "reference_display_support_not_historical_mechanism"})
    return tuple(parts)


def _stair_ring(t, anchors, detail, cancelled=None):
    """🌀 Three short perimeter flights skirt the primary compound and triangular piers."""
    slab = _detail_value("floor_thickness")
    inner, start, width = (_v("core.circulation." + key) for key in ("flight_inner", "flight_start", "flight_width"))
    edge, outer = inner + width, t.core_side / 2 - _detail_value("wall_thickness")
    parts, frame = [], _core_frame(t)
    def platform(key, bounds, z, role="landing", axis=None, route=None):
        meta = {} if axis is None else {"walking_axis": axis}
        x0, x1, y0, y1 = bounds
        points = ((x0, y0), (x1, y0), (x1, y1), (x0, y1))
        limit = 2 * outer - _v("core.profile.pier_leg")
        for nx, ny in ((1, 1), (-1, 1), (-1, -1), (1, -1)):
            clipped = []
            for a, b in zip(points, points[1:] + points[:1]):
                da, db = limit - nx * a[0] - ny * a[1], limit - nx * b[0] - ny * b[1]
                if da >= -1e-9:
                    clipped.append(a)
                if (da >= 0) != (db >= 0):
                    clipped.append(tuple(a[k] + (b[k] - a[k]) * da / (da - db) for k in range(2)))
            points = tuple(clipped)
        if route is not None:
            meta["walking_route"] = tuple(tuple(frame.origin[k] + frame.x[k] * a + frame.y[k] * b for k in range(2)) for a, b in route)
        solid = _polygon_prism(points, z - slab, z)
        solid = _closed(solid.vertices, solid.faces, meta)
        parts.append(NamedPart(key, role, place(solid, frame), "concrete"))
    def phase_platforms(tag, phase, z, arms):
        sectors = (((start, edge, inner, edge), (inner, edge, -edge, -inner), (inner, edge, -inner, inner)), ((-edge, -inner, start, edge),), ((-edge, -start, -edge, -inner),))
        for index, bounds in enumerate(sectors[phase]):
            platform(f"turning-landing-{tag}-{index}", bounds, z)
        if arms:
            bands = (((edge, outer, -inner, inner), (start, inner, edge, outer), (start, edge, -outer, -edge)), ((-edge, -start, edge, outer), (-outer, -edge, start, inner)), ((-inner, -start, -outer, -edge), (-outer, -edge, -edge, -start)))
            for index, bounds in enumerate(bands[phase]):
                diagonal = _stair_corner_route(phase, inner, edge, outer) if (phase, index) in ((0, 2), (1, 0), (2, 1)) else None
                platform(f"phase-walkway-{tag}-{index}", bounds, z, route=diagonal)
    lobby_levels = (anchors[0], _data()["podium"]["levels"][1]["value"])
    raw = _data()["towers"][t.index]["floors"]
    sparse = _logical_floor_sill(raw[0], t.storey_height)
    highest_entry = max(item["center"][2]["value"] + .2 for floor in raw for group in floor["groups"] for item in group["items"])
    roof_return = len(anchors) > 1 and abs(anchors[-1] - t.height) < 1e-6 and anchors[-2] > highest_entry
    for index, z in enumerate(anchors):
        roof = abs(z - t.height) < 1e-6
        if roof_return and roof:
            platform(f"roof-return-landing-{index}", (start, edge, inner, edge), z)
        elif roof_return and index == len(anchors) - 2:
            platform(f"roof-return-turn-{index}", (inner, edge, -edge, -inner), z)
        else:
            phase_platforms(f"{index}-0", 0, z, not roof)
        if any(abs(z - level) < 1e-6 for level in (*lobby_levels, sparse)):
            platform(f"lobby-cross-{index}-2", (-start, start, -outer, -edge), z, "floor")
            platform(f"lobby-cross-{index}-1", (-start, start, edge, outer), z, "floor")
    flight = 0
    for interval, (bottom, top) in enumerate(zip(anchors, anchors[1:])):
        levels = [bottom, bottom + (top - bottom) / 3, bottom + 2 * (top - bottom) / 3, top]
        final_return = roof_return and interval == len(anchors) - 2
        if not final_return:
            for phase in (1, 2):
                z = levels[phase]
                access = any(s.tower == t.index and abs(s.facade_plane.origin[2] - z) < 1e-3 for s in slots())
                phase_platforms(f"{interval}-{phase}", phase, z, access)
        if detail != "full":
            continue
        flights = ((0, bottom, top, -inner, inner),) if final_return else tuple((phase + 1, z0, z1, -start, inner) for phase, (z0, z1) in enumerate(zip(levels, levels[1:])))
        for angle, z0, z1, run_start, run_end in flights:
            _check_cancel(cancelled)
            plane = _core_frame(t, angle * math.pi / 2)
            steps = max(1, math.ceil((z1 - z0) / .17))
            for step in range(steps):
                y0, y1 = run_start + step * (run_end - run_start) / steps, run_start + (step + 1) * (run_end - run_start) / steps
                z = z0 + (step + 1) * (z1 - z0) / steps
                parts.append(NamedPart(f"stair-{flight}-{step}", "stair_tread", place(_box(inner, edge, y0, y1, z - .09, z), plane), "concrete"))
            rail = _beam((inner + .01, run_start + .03, z0 + .95), (inner + .01, run_end - .03, z1 + .95), .02)
            parts.append(NamedPart(f"stair-rail-{flight}", "stair_rail", place(rail, plane), "metal"))
            for end, y, z in ((0, run_start + .04, z0), (1, run_end - .04, z1)):
                post = _cylinder((inner + .01, y, z), .01, .95, "z", 12)
                parts.append(NamedPart(f"stair-rail-post-{flight}-{end}", "stair_rail", place(post, plane), "metal"))
            flight += 1
    return tuple(parts)

def _stair_corner_route(phase, inner, edge, outer):
    """🧭 Follow a supported rounded inner turn with a continuous six-decimetre body footprint."""
    radius = .302
    points = tuple((inner + radius * math.cos(i * math.pi / 16), -edge - radius * math.sin(i * math.pi / 16)) for i in range(9)) + ((.9, -outer + .3),)
    angle = (0, math.pi, -math.pi / 2)[phase]
    c, s = math.cos(angle), math.sin(angle)
    return tuple((c * x - s * y, s * x + c * y) for x, y in points)
