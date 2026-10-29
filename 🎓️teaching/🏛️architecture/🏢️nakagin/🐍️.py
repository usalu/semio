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
    capital_taper: float


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
                "items", "core", "towers", "capital", "podium", "energy", "climate", "variants")
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
    pts = []
    for (cx, cz), start in (((w / 2 - r, r), -90), ((w / 2 - r, h - r), 0), ((-w / 2 + r, h - r), 90), ((-w / 2 + r, r), 180)):
        for i in range(segments + 1):
            a = _rad(start + 90 * i / segments)
            pts.append((cx + r * math.cos(a), cz + r * math.sin(a)))
    return pts


def capsule_solid(t) -> Solid:
    """💊 Rounded-box capsule in its local frame: base on World XY, back face at y=0, front face with the window at y=length, four bolts on the back face."""
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
    """🗼 Both towers (A, B) with core, storeys and capital parameters."""
    d = _data()
    core, cap = d["core"], d["capital"]
    pitch, podium = core["storey_height"]["value"], core["podium_storeys"]["value"]
    out = []
    for i, t in enumerate(d["towers"]):
        cs = t["capsule_storeys"]["value"]
        storeys = podium + cs
        out.append(Tower(
            key=t["key"], name=t["name"], index=i, center=(t["center"]["x"]["value"], t["center"]["y"]["value"]),
            storeys=storeys, capsule_storeys=cs, first_capsule_storey=podium,
            core_side=core["side"]["value"], shaft_side=core["shaft_side"]["value"], storey_height=pitch,
            base_elevation=podium * pitch, height=storeys * pitch,
            capital_height=cap["height"]["value"], capital_overhang=cap["overhang"]["value"], capital_taper=cap["taper"]["value"]))
    return tuple(out)


def _tower(tower) -> Tower:
    return tower if isinstance(tower, Tower) else towers()[tower if isinstance(tower, int) else [t.key for t in towers()].index(tower)]


def core_solid(tower) -> Solid:
    """🧱 Square core box of a tower from ground to core top; meta holds the shaft side and storey elevations."""
    t = _tower(tower)
    cx, cy = t.center
    s = t.core_side / 2
    levels = tuple(i * t.storey_height for i in range(t.storeys + 1))
    return _box(cx - s, cx + s, cy - s, cy + s, 0.0, t.height, {"shaft_side": t.shaft_side, "levels": levels})


def capital_solid(tower) -> Solid:
    """👑 Capital (roof head) frustum on top of the core: wider base by the overhang, roof edges inset by the taper."""
    t = _tower(tower)
    cx, cy = t.center
    b = t.core_side / 2 + t.capital_overhang
    r = b - t.capital_taper
    z0, z1 = t.height, t.height + t.capital_height
    vertices = ((cx - b, cy - b, z0), (cx + b, cy - b, z0), (cx + b, cy + b, z0), (cx - b, cy + b, z0),
                (cx - r, cy - r, z1), (cx + r, cy - r, z1), (cx + r, cy + r, z1), (cx - r, cy + r, z1))
    faces = ((0, 3, 2, 1), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7))
    return Solid(vertices, faces, _edges(faces), None, (), {"base_side": 2 * b, "roof_side": 2 * r})


def podium_solid() -> Solid:
    """🏬 Lower building as an extruded box."""
    p = _data()["podium"]
    cx, cy, l, w = p["center"]["x"]["value"], p["center"]["y"]["value"], p["length"]["value"], p["width"]["value"]
    return _box(cx - l / 2, cx + l / 2, cy - w / 2, cy + w / 2, 0.0, podium_height(), {"storeys": p["storeys"]["value"]})


def podium_height() -> float:
    """📏 Podium height: its storeys times the core storey height."""
    return _data()["podium"]["storeys"]["value"] * _data()["core"]["storey_height"]["value"]


def facade_storeys(tower) -> tuple:
    """🏛 Facade indices that carry capsules for each capsule storey of a tower, counted from the first capsule storey."""
    t = _tower(tower)
    raw = _data()["towers"][t.index]
    special = {e["storey"]: tuple(e["facades"]) for e in raw["exceptions"]}
    return tuple(special.get(s, tuple(raw["facades"])) for s in range(t.capsule_storeys))


def _type_key(tower, storey, facade, item) -> str:
    a = _data()["assignment"]
    n = (a["tower_weight"]["value"] * tower + a["storey_weight"]["value"] * storey + a["facade_weight"]["value"] * facade
         + a["item_weight"]["value"] * item + a["offset"]["value"])
    return a["order"][n % len(a["order"])]


def _facade_plane(t: Tower, facade: int, item: int, storey: int) -> Plane:
    d = _data()
    nx, ny = d["facades"][facade]["normal"]
    xaxis = (ny, -nx, 0.0)
    pitch = d["items"]["pitch"]["value"]
    lateral = (item - 0.5) * pitch
    stagger = d["items"]["stagger"]["value"] if item == 1 else 0.0
    half = t.core_side / 2
    z = t.base_elevation + storey * t.storey_height + stagger
    origin = (t.center[0] + nx * half + xaxis[0] * lateral, t.center[1] + ny * half + xaxis[1] * lateral, z)
    return Plane(origin, xaxis, (float(nx), float(ny), 0.0), (0.0, 0.0, 1.0))


def _yaw(plane: Plane, degrees: float) -> Plane:
    """🔄 Yaw a facade plane about its Z axis and push the origin outward by half the width times sin(yaw), so the back face just touches the facade."""
    c, s = math.cos(_rad(degrees)), math.sin(_rad(degrees))
    mix = lambda a, b: tuple(c * p + s * q for p, q in zip(a, b))
    x = mix(plane.x, plane.y)
    y = tuple(-s * p + c * q for p, q in zip(plane.x, plane.y))
    clearance = _data()["capsule"]["width"]["value"] / 2 * abs(s)
    origin = tuple(o + clearance * q for o, q in zip(plane.origin, plane.y))
    return Plane(origin, x, y, plane.z)


@lru_cache(maxsize=1)
def slots() -> tuple:
    """📍 Every capsule slot of the building in tree order: building, tower, storey, facade, item."""
    d = _data()
    rotations = {t["key"]: t["rotation"]["value"] for t in d["types"]}
    out = []
    for t in towers():
        for storey, facades in enumerate(facade_storeys(t)):
            for facade in facades:
                for item in range(d["items"]["per_facade"]["value"]):
                    key = _type_key(t.index, storey, facade, item)
                    base = _facade_plane(t, facade, item, storey)
                    angle = rotations[key] * (1 if item == 0 else -1)
                    out.append(Slot(0, t.index, storey, facade, item, key, _yaw(base, angle), base, angle))
    return tuple(out)


def tree() -> dict:
    """🌳 Aggregated tree {BUILDING;TOWER;STORY;FACADE} to the two capsule slots of each branch."""
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
    return Solid(tuple(at(p) for p in solid.vertices), solid.faces, solid.edges, window, tuple(at(p) for p in solid.bolts), solid.meta)


def slot_solid(slot: Slot) -> Solid:
    """💊 World-placed capsule of a slot."""
    return place(capsule_solid(capsule_type(slot.type)), slot.plane)


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
