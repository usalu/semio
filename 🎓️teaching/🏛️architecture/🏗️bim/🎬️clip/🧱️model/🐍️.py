"""🏛️ Schema-first Villa Savoye teaching model: one IFC 4.3 JSON source, stdlib-only derivations."""

from __future__ import annotations

from copy import deepcopy
from functools import lru_cache
import importlib
import json
import math
from pathlib import Path
import uuid

ALPHABET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$"
SEED = "semio:villa-savoye:"
LANGUAGES = ("en", "de")
ARC_STEP = 7.5


def compress_guid(value: int) -> str:
    """🔐 Encode a 128-bit integer as a 22-character IFC GlobalId."""
    if isinstance(value, bool) or not isinstance(value, int) or not 0 <= value < 1 << 128:
        raise ValueError("A GlobalId encodes an integer of 128 bits")
    digits = []
    for _ in range(22):
        value, digit = divmod(value, 64)
        digits.append(ALPHABET[digit])
    return "".join(reversed(digits))


def decompress_guid(text: str) -> int:
    """🔓 Decode a 22-character IFC GlobalId into its 128-bit integer."""
    if not isinstance(text, str) or len(text) != 22 or any(char not in ALPHABET for char in text):
        raise ValueError("A GlobalId has 22 characters of 0-9A-Za-z_$")
    value = 0
    for char in text:
        value = value * 64 + ALPHABET.index(char)
    if value >= 1 << 128:
        raise ValueError("The first GlobalId character is 0 to 3")
    return value


def global_id(tag: str) -> str:
    """🪪 Derive the stable GlobalId of a teaching object from its tag."""
    return compress_guid(uuid.uuid5(uuid.NAMESPACE_URL, SEED + tag).int)


@lru_cache(maxsize=1)
def _source() -> str:
    return Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8")


def load_model() -> dict:
    """📦 Load a fresh, language-independent Villa Savoye teaching model."""
    return json.loads(_source())


def _m(model):
    return load_model() if model is None else model


def _registry(model: dict) -> dict:
    items = {}
    for group in ("elements", "types", "spaces", "storeys", "materials"):
        for obj in model[group]:
            items[obj["tag"]] = obj
            items[obj["guid"]] = obj
    for name in ("project", "site", "building"):
        items[model[name]["tag"]] = items[model[name]["guid"]] = model[name]
    return items


def _find(model: dict, key: str) -> dict:
    try:
        return _registry(model)[key]
    except KeyError:
        raise ValueError(f"Unknown object: {key}") from None


def _elevations(model: dict) -> dict:
    return {s["id"]: s["elevation"]["value"] for s in model["storeys"]}


def _z(model: dict, value):
    return _elevations(model)[value] if isinstance(value, str) else value


def _box(model: dict) -> tuple:
    box = model["building"]["box"]
    return box["length"]["value"], box["width"]["value"], box["height"]["value"]


def _material(model: dict, obj: dict):
    tag = obj.get("material")
    if tag is None and obj.get("type"):
        tag = _find(model, obj["type"]).get("material")
    return None if tag is None else _find(model, tag)


def _layers(model: dict, layerset: dict) -> list:
    out = []
    for layer in layerset["layers"]:
        density = _find(model, layer["material"])["density"]
        out.append({**deepcopy(layer), "density": deepcopy(density)})
    return out


def _slab_thickness(model: dict, slab: dict) -> float:
    if "thickness" in slab["attributes"]:
        return slab["attributes"]["thickness"]["value"]
    layerset = _material(model, slab)
    return next(l["thickness"]["value"] for l in layerset["layers"] if l["id"] == "concrete")


def _wall_thickness(model: dict, wall: dict) -> float:
    if "thickness" in wall["attributes"]:
        return wall["attributes"]["thickness"]["value"]
    return sum(l["thickness"]["value"] for l in _material(model, wall)["layers"])


def _top(model: dict, spec) -> float:
    if isinstance(spec, (int, float)):
        return spec
    if spec == "overall":
        return _box(model)[2]
    if spec.startswith("soffit:"):
        slab = _find(model, spec[7:])
        return _elevations(model)[slab["geometry"]["level"]] - _slab_thickness(model, slab)
    return _elevations(model)[spec]


def _round(value: float, digits: int = 9) -> float:
    return round(value, digits)


def _area(polygon: list) -> float:
    return abs(sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(polygon, polygon[1:] + polygon[:1]))) / 2


def _length(path: list) -> float:
    return sum(math.dist(a, b) for a, b in zip(path, path[1:]))


def _arc(centre, radius, start, end, step=ARC_STEP) -> list:
    count = max(1, round(abs(end - start) / step))
    return [[_round(centre[0] + radius * math.cos(math.radians(start + (end - start) * i / count)), 6),
             _round(centre[1] + radius * math.sin(math.radians(start + (end - start) * i / count)), 6)] for i in range(count + 1)]


def _car(model: dict) -> dict:
    car = model["car"]
    return {key: car[key]["value"] for key in ("length", "width", "turning_radius", "wall_clearance", "min_side_margin")}


def _curve_radius(model: dict) -> float:
    car = _car(model)
    return _round(car["turning_radius"] - car["width"] - car["wall_clearance"], 9)


def _curtain(model: dict, tag: str = "CW-1") -> dict:
    wall = _find(model, tag)
    centre = wall["geometry"]["centre"]
    return {"centre": centre, "radius": _curve_radius(model), "south_y": wall["geometry"]["south_y"]}


def _clean(points: list) -> list:
    out = []
    for point in points:
        if not out or math.dist(point, out[-1]) > 1e-9:
            out.append(point)
    if len(out) > 1 and math.dist(out[0], out[-1]) < 1e-9:
        out.pop()
    return out


def _expand(model: dict, items: list) -> list:
    out = []
    for item in items:
        if isinstance(item, dict) and "arc" in item:
            arc = item["arc"]
            out += _arc(arc["centre"], arc["radius"], arc["start"], arc["end"])
        elif isinstance(item, dict):
            cw = _curtain(model, item["arc_of"])
            points = _arc(cw["centre"], cw["radius"], 0, 180)
            out += points[::-1] if not item.get("reverse") else points
        else:
            out.append(list(item))
    return _clean(out)


def _storey_of(model: dict, obj: dict) -> str:
    return obj["storey"]


def _by_class(model: dict, cls: str, predefined: str | None = None) -> list:
    return [e for e in model["elements"] if e["class"] == cls and (predefined is None or e["predefined_type"] == predefined)]


def _revision(model: dict, obj: dict) -> int:
    return obj["metadata"]["revision"]


def element(key: str, model: dict | None = None) -> dict:
    """🧱 Return one object by GlobalId or tag with merged type psets and a resolved material."""
    m = _m(model)
    obj = deepcopy(_find(m, key))
    kind = _find(m, obj["type"]) if obj.get("type") else None
    if kind is not None or "psets" in obj:
        psets = deepcopy(kind["psets"]) if kind is not None else {}
        for name, properties in obj.get("psets", {}).items():
            psets.setdefault(name, {}).update(deepcopy(properties))
        obj["psets"] = psets
    material = _material(m, obj)
    if material is not None:
        obj["material_tag"] = material["tag"]
        obj["material"] = deepcopy(material)
        if "layers" in material:
            obj["material"]["layers"] = _layers(m, material)
            obj["material"]["total_thickness"] = _round(sum(l["thickness"]["value"] for l in material["layers"]))
    return obj


def _opening_values(model: dict, opening: dict) -> dict:
    a = opening["attributes"]
    return {"start": a["start"]["value"], "end": a["end"]["value"], "sill": a["sill"]["value"], "head": a["head"]["value"]}


def _wall_geometry(model: dict, wall: dict) -> dict:
    path = wall["geometry"]["path"]
    base = _z(model, wall["geometry"]["base_level"])
    top = _top(model, wall["geometry"]["top_level"])
    length = math.dist(path[0], path[1])
    direction = [(path[1][0] - path[0][0]) / length, (path[1][1] - path[0][1]) / length]
    openings = []
    for opening in [e for e in _by_class(model, "IfcOpeningElement") if e["geometry"]["wall"] == wall["tag"]]:
        v = _opening_values(model, opening)
        window = next(e for e in _by_class(model, "IfcWindow") if e["geometry"]["opening"] == opening["tag"])
        p0 = [path[0][0] + direction[0] * v["start"], path[0][1] + direction[1] * v["start"]]
        p1 = [path[0][0] + direction[0] * v["end"], path[0][1] + direction[1] * v["end"]]
        openings.append({"tag": opening["tag"], "guid": opening["guid"], "window_tag": window["tag"], "window_guid": window["guid"],
                         "start": v["start"], "end": v["end"], "sill": v["sill"], "head": v["head"],
                         "sill_z": _round(base + v["sill"]), "head_z": _round(base + v["head"]), "p0": [_round(c, 6) for c in p0], "p1": [_round(c, 6) for c in p1]})
    return {"tag": wall["tag"], "guid": wall["guid"], "start": list(path[0]), "end": list(path[1]), "length": _round(length),
            "thickness": _round(_wall_thickness(model, wall)), "base": base, "top": _round(top), "openings": openings}


def _flights(model: dict, ramp: dict) -> list:
    out = []
    for flight in [e for e in _by_class(model, "IfcRampFlight") if e["geometry"]["ramp"] == ramp["tag"]]:
        g = flight["geometry"]
        lo, hi = _z(model, g["from_level"]), _z(model, g["to_level"])
        z0, z1 = _round(lo + (hi - lo) * g["portion"][0]), _round(lo + (hi - lo) * g["portion"][1])
        run = math.dist(g["path"][0], g["path"][1])
        out.append({"tag": flight["tag"], "guid": flight["guid"], "ramp": ramp["tag"], "start": list(g["path"][0]), "end": list(g["path"][1]),
                    "width": flight["attributes"]["width"]["value"], "z0": z0, "z1": z1, "run": _round(run), "slope": _round((z1 - z0) / run)})
    return out


def _slab_geometry(model: dict, slab: dict) -> dict:
    g = slab["geometry"]
    level = _z(model, g["level"])
    out = {"tag": slab["tag"], "guid": slab["guid"], "kind": slab["predefined_type"], "outline": _expand(model, g["outline"]),
           "voids": [_expand(model, v["outline"]) for v in g["voids"]], "void_tags": [v["tag"] for v in g["voids"]],
           "top": level, "thickness": _round(_slab_thickness(model, slab))}
    layerset = _material(model, slab)
    if layerset is not None and "layers" in layerset:
        out["layers"] = [{"id": l["id"], "name": l["name"], "thickness": l["thickness"]["value"], "zones": l.get("zones", []),
                          "density": _find(model, l["material"])["density"]["value"]} for l in layerset["layers"]]
        out["total_thickness"] = _round(sum(l["thickness"]["value"] for l in layerset["layers"]))
    if "zones" in g:
        out["zones"] = {name: _expand(model, _find(model, tag)["outline"]) for name, tag in g["zones"].items()}
    return out


def _service(model: dict, e: dict) -> dict:
    kinds = {"IfcBoiler": "boiler", "IfcSpaceHeater": "radiator", "IfcWasteTerminal": "drain"}
    kind = kinds.get(e["class"]) or ("downpipe" if e.get("system") == "rainwater" else "pipe")
    points = [[p[0], p[1], _z(model, p[2])] for p in e["geometry"]["points"]]
    out = {"tag": e["tag"], "guid": e["guid"], "class": e["class"], "kind": kind, "system": e.get("system"), "points": points}
    if "size" in e["geometry"]:
        out["size"] = e["geometry"]["size"]
    if "diameter" in e["attributes"]:
        out["diameter"] = e["attributes"]["diameter"]["value"]
    return out


def building_geometry(model: dict | None = None) -> dict:
    """🗺️ Give every drawing the same metric geometry of the villa (metres, plan x east, y north, z up)."""
    m = _m(model)
    length, width, height = _box(m)
    grid = m["grid"]
    pilotis = []
    for e in _by_class(m, "IfcColumn"):
        kind = _find(m, e["type"])
        pilotis.append({"tag": e["tag"], "guid": e["guid"], "x": e["geometry"]["axis"][0], "y": e["geometry"]["axis"][1],
                        "radius": kind["profile"]["diameter"]["value"] / 2, "base": _z(m, e["geometry"]["base_level"]),
                        "top": _round(_top(m, e["geometry"]["top_level"]))})
    walls = [_wall_geometry(m, w) for w in _by_class(m, "IfcWall", "STANDARD")]
    cw = _find(m, "CW-1")
    curtain = _curtain(m)
    cx, cy = curtain["centre"]
    r = curtain["radius"]
    path = [[cx - r, curtain["south_y"]]] + _arc(curtain["centre"], r, 180, 0) + [[cx + r, curtain["south_y"]]]
    ramps = []
    for ramp in _by_class(m, "IfcRamp"):
        g = ramp["geometry"]
        lo, hi = _z(m, g["from_level"]), _z(m, g["to_level"])
        ramps.append({"tag": ramp["tag"], "guid": ramp["guid"], "predefined_type": ramp["predefined_type"], "width": ramp["attributes"]["width"]["value"],
                      "flights": _flights(m, ramp),
                      "landings": [{"outline": l["outline"], "z": _round(lo + (hi - lo) * l["portion"])} for l in g["landings"]]})
    first_ramp = ramps[0]
    spaces = [{"id": s["tag"], "tag": s["tag"], "guid": s["guid"], "name": s["name"], "outline": _expand(m, s["outline"]), "storey": s["storey"],
               "area": _round(_area(_expand(m, s["outline"]))), "predefined_type": s["predefined_type"]} for s in m["spaces"]]
    return {
        "box": {"length": length, "width": width, "height": height},
        "storeys": _elevations(m),
        "grid": {"x": grid["x"], "y": grid["y"], "labels_x": grid["labels_x"], "labels_y": grid["labels_y"], "spacing": grid["spacing"]["value"]},
        "pilotis": pilotis,
        "slabs": [_slab_geometry(m, s) for s in _by_class(m, "IfcSlab")],
        "walls": walls,
        "curtain_wall": {"tag": cw["tag"], "guid": cw["guid"], "path": path, "base": _z(m, cw["geometry"]["base_level"]),
                         "top": _round(_top(m, cw["geometry"]["top_level"])), "radius": r, "centre": list(curtain["centre"])},
        "ramp": {"tag": first_ramp["tag"], "guid": first_ramp["guid"], "ramps": ramps, "flights": [f for ramp in ramps for f in ramp["flights"]],
                 "landings": [l for ramp in ramps for l in ramp["landings"]], "width": first_ramp["width"],
                 "voids": {v["tag"]: _expand(m, v["outline"]) for s in _by_class(m, "IfcSlab") for v in s["geometry"]["voids"] if v["tag"].startswith("V-RAMP")}},
        "screens": [{"tag": s["tag"], "guid": s["guid"], "path": _expand(m, s["geometry"]["path"]), "base": _z(m, s["geometry"]["base_level"]),
                     "top": _round(_top(m, s["geometry"]["top_level"])), "thickness": _round(_wall_thickness(m, s))} for s in _by_class(m, "IfcWall", "SOLIDWALL")],
        "spaces": spaces,
        "services": [_service(m, e) for e in m["elements"] if "services" in e["metadata"]["domains"]],
    }


def _pose_distance(pose: list, car: dict, centre: tuple, radius: float) -> float:
    heading = math.radians(pose[2])
    dx, dy = centre[0] - pose[0], centre[1] - pose[1]
    u = dx * math.cos(heading) + dy * math.sin(heading)
    v = -dx * math.sin(heading) + dy * math.cos(heading)
    ex, ey = max(abs(u) - car["length"] / 2, 0.0), max(abs(v) - car["width"] / 2, 0.0)
    return math.hypot(ex, ey) - radius


def car_path(model: dict | None = None, samples: int = 120) -> dict:
    """🚘 Drive the assumed car under the house: north entry between two pilotis, then along the curved glass wall."""
    m = _m(model)
    car = _car(m)
    cw = _curtain(m)
    cx, cy = cw["centre"]
    rc = car["turning_radius"] - car["width"] / 2
    length, width, _ = _box(m)
    y1 = cy + 2 * rc
    y_start, y_end = width + 4.0, cw["south_y"] - 1.5
    xe = cx + rc

    def straight(a, b, heading):
        return (math.dist(a, b), lambda s, a=a, b=b, h=heading: (a[0] + (b[0] - a[0]) * s, a[1] + (b[1] - a[1]) * s, h))

    def arc(centre, a0, a1, turn):
        return (math.radians(abs(a1 - a0)) * rc,
                lambda s, c=centre, a0=a0, a1=a1, t=turn: (c[0] + rc * math.cos(math.radians(a0 + (a1 - a0) * s)),
                                                           c[1] + rc * math.sin(math.radians(a0 + (a1 - a0) * s)),
                                                           (a0 + (a1 - a0) * s + t) % 360))

    parts = [straight((xe, y_start), (xe, y1), 270.0), arc((cx, y1), 0, -90, -90), arc((cx, cy), 90, 180, 90), straight((cx - rc, cy), (cx - rc, y_end), 270.0)]
    total = sum(p[0] for p in parts)

    def pose(distance):
        for span, fn in parts:
            if distance <= span + 1e-12:
                x, y, h = fn(min(1.0, distance / span))
                return [_round(x, 6), _round(y, 6), _round(h, 4)]
            distance -= span
        x, y, h = parts[-1][1](1.0)
        return [_round(x, 6), _round(y, 6), _round(h, 4)]

    poses = [pose(total * i / (samples - 1)) for i in range(samples)]
    dense = [pose(total * i / 1200) for i in range(1201)]
    grid = m["grid"]
    gx, gy = grid["x"], grid["y"]
    radius = _find(m, "TY-PILOTI")["profile"]["diameter"]["value"] / 2
    crossings = []
    for j, row in enumerate(grid["labels_y"]):
        for index, (a, b) in enumerate(zip(dense, dense[1:])):
            if (a[1] - gy[j]) * (b[1] - gy[j]) < 0:
                t = (gy[j] - a[1]) / (b[1] - a[1])
                x = a[0] + (b[0] - a[0]) * t
                heading = math.radians(a[2])
                half = car["length"] / 2 * abs(math.cos(heading)) + car["width"] / 2 * abs(math.sin(heading))
                crossings.append({"row": row, "x": _round(x, 4), "y": gy[j], "half_extent": _round(half, 4), "progress": _round((index + t) / 1200, 6)})
    for c in crossings:
        i = max(k for k in range(len(gx)) if gx[k] <= c["x"]) if c["x"] >= gx[0] else -1
        c["between"] = [None if i < 0 else c["row"] + grid["labels_x"][i], c["row"] + grid["labels_x"][i + 1] if i + 1 < len(gx) else None]
        if i >= 0 and i + 1 < len(gx):
            near = [p for p in dense if abs(p[1] - c["y"]) < 3.0]
            c["clear_left"] = _round(min(_pose_distance(p, car, (gx[i], c["y"]), radius) for p in near), 4)
            c["clear_right"] = _round(min(_pose_distance(p, car, (gx[i + 1], c["y"]), radius) for p in near), 4)
    entry = next(c for c in crossings if c["row"] == grid["labels_y"][-1])
    pair = entry["between"]
    gap = {"start": [_round(gx[grid["labels_x"].index(pair[0][1:])] + radius, 6), entry["y"]], "end": [_round(gx[grid["labels_x"].index(pair[1][1:])] - radius, 6), entry["y"]]}
    clearance_min = min(_pose_distance(p, car, (px, py), radius) for p in dense[::6] for px in gx for py in gy if abs(px - p[0]) < 7 and abs(py - p[1]) < 7)
    return {"poses": poses, "length": _round(total), "entry_pair": pair, "pairs": [c["between"] for c in crossings], "crossings": crossings,
            "gap_progress": entry["progress"], "gap": gap, "centre_radius": _round(rc), "curve_centre": list(cw["centre"]),
            "min_clearance": _round(clearance_min, 4), "car": {"length": car["length"], "width": car["width"]}}


def clearance(model: dict | None = None, spacing: float | None = None) -> dict:
    """🚗 Decide if the assumed 1929 car fits between two pilotis and along the curved glass wall."""
    if isinstance(model, (int, float)) and not isinstance(model, bool):
        model, spacing = spacing, model
    m = _m(model)
    car = _car(m)
    diameter = _find(m, "TY-PILOTI")["profile"]["diameter"]["value"]
    actual = m["grid"]["spacing"]["value"]
    initial = m["grid"]["initial_spacing"]["value"]
    used = actual if spacing is None else spacing
    clear = _round(used - diameter)
    margin = _round((clear - car["width"]) / 2)
    path = car_path(m)
    fits_path = path["min_clearance"] > 0
    return {"spacing": used, "grid_spacing": actual, "initial_spacing": initial, "diameter": diameter, "clear_width": clear,
            "clear_width_initial": _round(initial - diameter), "car_width": car["width"], "car_length": car["length"], "margin": margin,
            "margin_initial": _round((initial - diameter - car["width"]) / 2), "min_margin": car["min_side_margin"],
            "turning_radius": car["turning_radius"], "curve_radius": _curve_radius(m), "wall_clearance": car["wall_clearance"],
            "centre_radius": path["centre_radius"], "fits": margin >= car["min_side_margin"] and (spacing is not None or fits_path),
            "fits_initial": (initial - diameter - car["width"]) / 2 >= car["min_side_margin"], "gaps": path["crossings"],
            "entry_pair": path["entry_pair"], "min_path_clearance": path["min_clearance"]}


def clearance_limits(model: dict | None = None, *, spacing: float | None = None, diameter: float | None = None, car_width: float | None = None) -> dict:
    """🚧 Straight-through test of the assumed car between two pilotis for optional spacing, diameter and car width overrides, with the smallest spacing that still fits."""
    m = _m(model)
    car = _car(m)
    d = _find(m, "TY-PILOTI")["profile"]["diameter"]["value"] if diameter is None else diameter
    w = car["width"] if car_width is None else car_width
    s = m["grid"]["spacing"]["value"] if spacing is None else spacing
    need = _round(w + 2 * car["min_side_margin"])
    margin = _round((s - d - w) / 2)
    return {"spacing": s, "diameter": d, "car_width": w, "min_margin": car["min_side_margin"], "clear_width": _round(s - d), "margin": margin,
            "required_clear_width": need, "min_spacing": _round(need + d), "slack": _round(s - d - need), "fits": margin >= car["min_side_margin"] - 1e-9}


def path_pinches(model: dict | None = None, count: int = 2) -> list:
    """📍 The closest approaches of the assumed car to a piloti along its path, tightest first: tag, clearance, progress and pose."""
    m = _m(model)
    car = _car(m)
    poses = car_path(m, samples=1201)["poses"]
    found = []
    for piloti in building_geometry(m)["pilotis"]:
        distance, index = min((_pose_distance(pose, car, (piloti["x"], piloti["y"]), piloti["radius"]), i) for i, pose in enumerate(poses))
        found.append({"tag": piloti["tag"], "clearance": _round(distance, 4), "progress": _round(index / 1200, 6), "pose": list(poses[index]), "centre": [piloti["x"], piloti["y"]],
                      "radius": piloti["radius"]})
    return sorted(found, key=lambda item: item["clearance"])[:count]


def class_chain(ifc_class: str, model: dict | None = None) -> list:
    """🧬 List the IFC supertype chain from IfcRoot down to a class with the attributes each level adds."""
    classes = _m(model)["classes"]
    if ifc_class not in classes:
        raise ValueError(f"Unknown IFC class: {ifc_class}")
    chain, name = [], ifc_class
    while name is not None:
        chain.append({"class": name, "adds": list(classes[name]["adds"])})
        name = classes[name]["supertype"]
    return chain[::-1]


def type_occurrences(type_id: str, model: dict | None = None) -> list:
    """🔢 List the tags of every occurrence defined by one type."""
    m = _m(model)
    kind = _find(m, type_id)
    if kind["tag"] not in {t["tag"] for t in m["types"]}:
        raise ValueError(f"Not a type: {type_id}")
    return [e["tag"] for e in m["elements"] if e.get("type") == kind["tag"]]


def _opening_area(model: dict, wall: dict) -> float:
    return sum((v["end"] - v["start"]) * (v["head"] - v["sill"]) for v in (_opening_values(model, o) for o in _by_class(model, "IfcOpeningElement") if o["geometry"]["wall"] == wall["tag"]))


def _slab_areas(model: dict, slab: dict) -> dict:
    g = _slab_geometry(model, slab)
    gross = _area(g["outline"])
    voids = sum(_area(v) for v in g["voids"])
    out = {"gross_area": _round(gross), "void_area": _round(voids), "area": _round(gross - voids), "perimeter": _round(_length(g["outline"] + [g["outline"][0]])),
           "thickness": g["thickness"]}
    out["volume"] = _round(out["area"] * g["thickness"])
    out["gross_volume"] = _round(gross * g["thickness"])
    return out


def quantities(key: str, model: dict | None = None) -> dict:
    """🧮 Derive the quantities of an element, a type total or a space from the authored values."""
    m = _m(model)
    obj = _find(m, key)
    cls = obj["class"]
    if cls == "IfcColumn":
        d = _find(m, obj["type"])["profile"]["diameter"]["value"]
        h = _top(m, obj["geometry"]["top_level"]) - _z(m, obj["geometry"]["base_level"])
        area = math.pi * d * d / 4
        return {"diameter": d, "radius": d / 2, "cross_section_area": area, "height": _round(h), "volume": area * _round(h), "outer_surface_area": math.pi * d * h}
    if cls == "IfcColumnType":
        occurrences = type_occurrences(obj["tag"], m)
        each = quantities(occurrences[0], m)
        return {"count": len(occurrences), "diameter": each["diameter"], "cross_section_area": each["cross_section_area"], "height": each["height"],
                "volume": sum(quantities(t, m)["volume"] for t in occurrences), "occurrences": occurrences}
    if cls == "IfcWall" and obj["predefined_type"] == "STANDARD":
        g = _wall_geometry(m, obj)
        gross = g["length"] * (g["top"] - g["base"])
        opening = _opening_area(m, obj)
        return {"length": g["length"], "height": _round(g["top"] - g["base"]), "thickness": g["thickness"], "gross_side_area": _round(gross), "opening_area": _round(opening),
                "net_side_area": _round(gross - opening), "glazing_share": opening / gross, "volume": _round((gross - opening) * g["thickness"]),
                "gross_volume": _round(gross * g["thickness"])}
    if cls == "IfcWall":
        path = _expand(m, obj["geometry"]["path"])
        h = _top(m, obj["geometry"]["top_level"]) - _z(m, obj["geometry"]["base_level"])
        t = _wall_thickness(m, obj)
        return {"length": _round(_length(path)), "height": _round(h), "thickness": t, "gross_side_area": _round(_length(path) * h), "volume": _round(_length(path) * h * t)}
    if cls == "IfcWallType":
        return {"thickness": _round(sum(l["thickness"]["value"] for l in _find(m, obj["material"])["layers"])),
                "layers": [{"id": l["id"], "name": l["name"], "thickness": l["thickness"]["value"]} for l in _find(m, obj["material"])["layers"]]}
    if cls == "IfcSlab":
        out = _slab_areas(m, obj)
        geometry = _slab_geometry(m, obj)
        if "layers" in geometry:
            out["total_thickness"] = geometry["total_thickness"]
        return out
    if cls in ("IfcOpeningElement", "IfcWindow"):
        opening = obj if cls == "IfcOpeningElement" else _find(m, obj["geometry"]["opening"])
        v = _opening_values(m, opening)
        return {"width": _round(v["end"] - v["start"]), "height": _round(v["head"] - v["sill"]), "area": _round((v["end"] - v["start"]) * (v["head"] - v["sill"])),
                "sill": v["sill"], "head": v["head"]}
    if cls == "IfcCurtainWall":
        cw = _curtain(m, obj["tag"])
        total = _length([[0, 0], [0, cw["centre"][1] - cw["south_y"]]]) * 2 + math.pi * cw["radius"]
        h = _top(m, obj["geometry"]["top_level"]) - _z(m, obj["geometry"]["base_level"])
        return {"radius": cw["radius"], "length": _round(total), "height": _round(h), "area": _round(total * h)}
    if cls == "IfcRamp":
        flights = _flights(m, obj)
        return {"width": obj["attributes"]["width"]["value"], "flight_count": len(flights), "length": _round(sum(f["run"] for f in flights)),
                "rise": _round(max(f["z1"] for f in flights) - min(f["z0"] for f in flights)), "slope": _round(sum(f["slope"] for f in flights) / len(flights))}
    if cls == "IfcRampFlight":
        f = next(f for f in _flights(m, _find(m, obj["geometry"]["ramp"])) if f["tag"] == obj["tag"])
        return {"length": f["run"], "run": f["run"], "rise": _round(f["z1"] - f["z0"]), "slope": f["slope"], "slope_percent": _round(f["slope"] * 100, 6),
                "width": f["width"], "area": _round(f["run"] * f["width"])}
    if cls == "IfcSpace":
        outline = _expand(m, obj["outline"])
        out = {"area": _round(_area(outline)), "perimeter": _round(_length(outline + [outline[0]]))}
        if "reference_area" in obj:
            out["reference_area"] = obj["reference_area"]["value"]
            out["fit"] = _round(out["area"] / out["reference_area"] - 1, 6)
        return out
    if cls == "IfcPipeSegment":
        points = [[p[0], p[1], _z(m, p[2])] for p in obj["geometry"]["points"]]
        return {"length": _round(sum(math.dist(a, b) for a, b in zip(points, points[1:]))), "diameter": obj["attributes"]["diameter"]["value"]}
    raise ValueError(f"No derived quantities for {key}")


def quantity(element_: dict | str, name: str, model: dict | None = None) -> float:
    """📏 Read one authored or derived quantity (a plain number) by name, layer id included."""
    m = _m(model)
    obj = _find(m, element_ if isinstance(element_, str) else element_["tag"])
    if name in obj.get("attributes", {}):
        return obj["attributes"][name]["value"]
    layerset = _material(m, obj)
    if layerset is not None and "layers" in layerset:
        for layer in layerset["layers"]:
            if layer["id"] == name:
                return layer["thickness"]["value"]
    try:
        derived = quantities(obj["tag"], m)
        if name in derived and isinstance(derived[name], (int, float)):
            return derived[name]
    except (ValueError, KeyError, StopIteration):
        pass
    if obj.get("type"):
        kind = _find(m, obj["type"])
        if name == "diameter" and "profile" in kind:
            return kind["profile"]["diameter"]["value"]
    raise ValueError(f"Unknown quantity {name} for {obj['tag']}")


def _special(model: dict, pid: str):
    car = model["car"]
    table = {"grid.spacing": ("spacing", model["grid"]["spacing"]), "grid.initial_spacing": ("initial_spacing", model["grid"]["initial_spacing"]),
             "car.width": ("car_width", car["width"]), "car.length": ("car_length", car["length"]), "car.turning_radius": ("turning_radius", car["turning_radius"]),
             "piloti.diameter": ("diameter", _find(model, "TY-PILOTI")["profile"]["diameter"])}
    return table.get(pid)


def information(tag: str, property_id: str, language: str, model: dict | None = None) -> dict:
    """📊 Give a raw value its unit, basis, object identity, localized label and meaning."""
    m = _m(model)
    if language not in LANGUAGES:
        raise ValueError("Unknown language")
    obj = element(tag, m)
    names = m["property_names"]
    special = _special(m, "piloti.diameter" if property_id == "diameter" and obj["class"] == "IfcColumn" else property_id)
    if special is not None:
        key, q = special
        value, unit, basis = q["value"], q["unit"], q["basis"]
    elif property_id == "material":
        material = obj.get("material")
        if material is None:
            raise ValueError(f"No material for {tag}")
        key, value, unit, basis = "material", material["name"][language], "", "assumed"
    elif property_id in ("GlobalId", "PredefinedType"):
        key, value, unit, basis = property_id, obj["guid"] if property_id == "GlobalId" else obj["predefined_type"], "", "derived"
    else:
        pset, _, prop = property_id.rpartition(".")
        found = None
        for name, properties in obj.get("psets", {}).items():
            if (not pset or name == pset) and prop in properties:
                found = properties[prop]
        if found is not None:
            key, value, unit, basis = prop, found["value"], "", found["basis"]
        else:
            key = property_id
            value = quantity(obj["tag"], property_id, m)
            unit = names.get(property_id, {}).get("unit", "m")
            attributes = obj.get("attributes", {})
            basis = attributes[property_id]["basis"] if property_id in attributes else "derived"
            unit = attributes[property_id]["unit"] if property_id in attributes else unit
    entry = names.get(key)
    label = entry["name"][language] if entry else key
    if entry and "meaning" in entry:
        meaning = entry["meaning"][language]
    else:
        meaning = f"{label} of {obj['name'][language]}" if language == "en" else f"{label} von {obj['name'][language]}"
    return {"value": value, "unit": unit, "basis": basis, "tag": obj["tag"], "guid": obj["guid"], "class": obj["class"], "storey": obj.get("storey"),
            "property_id": property_id, "label": label, "meaning": meaning}


def relations_of(key: str, model: dict | None = None) -> list:
    """🔗 List every IFC relationship touching an object together with the other side."""
    m = _m(model)
    obj = _find(m, key)
    registry = _registry(m)

    def side(tag):
        item = registry[tag]
        return {"tag": item["tag"], "guid": item["guid"], "class": item["class"]}

    out = []
    for rel in m["relations"]:
        if obj["tag"] == rel["relating"] or obj["tag"] in rel["related"]:
            role = "relating" if obj["tag"] == rel["relating"] else "related"
            others = rel["related"] if role == "relating" else [rel["relating"]]
            out.append({"guid": rel["guid"], "tag": rel["tag"], "class": rel["class"], "role": role, "name": rel["name"],
                        "relating": side(rel["relating"]), "related": [side(t) for t in rel["related"]], "others": [side(t) for t in others]})
    return out


def partial_model(model_id: str, model: dict | None = None) -> dict:
    """🧩 Select one discipline model: same schema, different authors, shared GlobalIds."""
    m = _m(model)
    spec = next((x for x in m["models"] if x["id"] == model_id), None)
    if spec is None:
        raise ValueError(f"Unknown model: {model_id}")
    registry = _registry(m)
    counts = {}
    for tag in spec["element_tags"]:
        counts[registry[tag]["class"]] = counts.get(registry[tag]["class"], 0) + 1
    return {"id": spec["id"], "schema_id": spec["schema_id"], "author": spec["author"], "author_label": deepcopy(spec.get("author_label") or {"en": spec["author"], "de": spec["author"]}),
            "name": deepcopy(spec["name"]), "revision": spec["revision"],
            "element_tags": list(spec["element_tags"]), "element_guids": [registry[t]["guid"] for t in spec["element_tags"]], "counts_by_class": counts}


def _footprint(model: dict, wall: dict) -> list:
    g = _wall_geometry(model, wall)
    (x0, y0), (x1, y1) = g["start"], g["end"]
    ux, uy = (x1 - x0) / g["length"], (y1 - y0) / g["length"]
    nx, ny = -uy * g["thickness"] / 2, ux * g["thickness"] / 2
    return [[_round(x0 - nx, 6), _round(y0 - ny, 6)], [_round(x1 - nx, 6), _round(y1 - ny, 6)], [_round(x1 + nx, 6), _round(y1 + ny, 6)], [_round(x0 + nx, 6), _round(y0 + ny, 6)]]


def represent(key: str, kind: str, model: dict | None = None) -> dict:
    """👁️ Derive one representation (plan, section, elevation, 3d, axis, footprint, body) from the current geometry."""
    m = _m(model)
    obj = _find(m, key)
    cls = obj["class"]
    if kind not in ("plan", "section", "elevation", "3d", "axis", "footprint", "body"):
        raise ValueError(f"Unknown representation: {kind}")
    if cls == "IfcColumn":
        d = _find(m, obj["type"])["profile"]["diameter"]["value"]
        x, y = obj["geometry"]["axis"]
        z0, z1 = _z(m, obj["geometry"]["base_level"]), _top(m, obj["geometry"]["top_level"])
        derived = {"radius": d / 2, "diameter": d, "height": _round(z1 - z0), "profile": "IfcCircleProfileDef", "solid": "IfcExtrudedAreaSolid", "cut_z": _round((z0 + z1) / 2)}
        circle = {"kind": "circle", "centre": [x, y], "radius": d / 2}
        rect = {"kind": "polygon", "role": "outline", "points": [[x - d / 2, z0], [x + d / 2, z0], [x + d / 2, z1], [x - d / 2, z1]]}
        table = {"axis": ("Axis", "Curve3D", [{"kind": "polyline", "points": [[x, y, z0], [x, y, z1]]}]), "footprint": ("FootPrint", "Curve2D", [circle]),
                 "plan": ("Body", "SweptSolid", [circle]), "section": ("Body", "SweptSolid", [rect]), "elevation": ("Body", "SweptSolid", [rect]),
                 "3d": ("Body", "SweptSolid", [{"kind": "cylinder", "centre": [x, y], "radius": d / 2, "z0": z0, "z1": z1}]),
                 "body": ("Body", "SweptSolid", [{"kind": "extrusion", "profile": circle, "position": [x, y, z0], "direction": [0, 0, 1], "depth": _round(z1 - z0)}])}
    elif cls == "IfcWall" and obj["predefined_type"] == "STANDARD":
        g = _wall_geometry(m, obj)
        outline = _footprint(m, obj)
        h = g["top"] - g["base"]
        openings = g["openings"]
        cut = _round(g["base"] + (openings[0]["sill"] + openings[0]["head"]) / 2) if openings else _round(g["base"] + h / 2)
        glazing = [{"kind": "polyline", "role": "glazing", "points": [o["p0"], o["p1"]]} for o in openings]
        elevation = [{"kind": "polygon", "role": "outline", "points": [[0, g["base"]], [g["length"], g["base"]], [g["length"], g["top"]], [0, g["top"]]]}]
        for o in openings:
            elevation.append({"kind": "polygon", "role": "hole", "tag": o["tag"], "points": [[o["start"], o["sill_z"]], [o["end"], o["sill_z"]], [o["end"], o["head_z"]], [o["start"], o["head_z"]]]})
            elevation.append({"kind": "polygon", "role": "glazing", "tag": o["window_tag"], "points": [[o["start"], o["sill_z"]], [o["end"], o["sill_z"]], [o["end"], o["head_z"]], [o["start"], o["head_z"]]]})
        t = g["thickness"]
        o = openings[0] if openings else None
        section = [{"kind": "polygon", "role": "outline", "points": [[0, g["base"]], [t, g["base"]], [t, o["sill_z"] if o else g["top"]], [0, o["sill_z"] if o else g["top"]]]}]
        if o:
            section.append({"kind": "polygon", "role": "outline", "points": [[0, o["head_z"]], [t, o["head_z"]], [t, g["top"]], [0, g["top"]]]})
            section.append({"kind": "polyline", "role": "glazing", "points": [[t / 2, o["sill_z"]], [t / 2, o["head_z"]]]})
        derived = {**quantities(obj["tag"], m), "cut_z": cut, "profile": "IfcRectangleProfileDef", "solid": "IfcExtrudedAreaSolid", "depth": _round(h)}
        table = {"axis": ("Axis", "Curve2D", [{"kind": "polyline", "points": [g["start"], g["end"]]}]),
                 "footprint": ("FootPrint", "Curve2D", [{"kind": "polygon", "role": "outline", "points": outline}]),
                 "plan": ("Body", "SweptSolid", [{"kind": "polygon", "role": "outline", "points": outline}] + glazing),
                 "section": ("Body", "SweptSolid", section), "elevation": ("Body", "SweptSolid", elevation),
                 "3d": ("Body", "SweptSolid", [{"kind": "prism", "polygon": outline, "z0": g["base"], "z1": g["top"], "holes": [
                     {"tag": o["tag"], "p0": o["p0"], "p1": o["p1"], "z0": o["sill_z"], "z1": o["head_z"]} for o in openings]}]),
                 "body": ("Body", "SweptSolid", [{"kind": "extrusion", "profile": {"kind": "rectangle", "length": g["length"], "thickness": t}, "position": [g["start"][0], g["start"][1], g["base"]],
                                                   "direction": [0, 0, 1], "depth": _round(h)}])}
    elif cls == "IfcSlab":
        g = _slab_geometry(m, obj)
        rings = [{"kind": "polygon", "role": "outline", "points": g["outline"]}] + [{"kind": "polygon", "role": "hole", "points": v} for v in g["voids"]]
        extent = max(p[0] for p in g["outline"])
        strip = {"kind": "polygon", "role": "outline", "points": [[0, g["top"] - g["thickness"]], [extent, g["top"] - g["thickness"]], [extent, g["top"]], [0, g["top"]]]}
        derived = {**quantities(obj["tag"], m), "cut_z": _round(g["top"] - g["thickness"] / 2), "profile": "IfcArbitraryProfileDefWithVoids", "solid": "IfcExtrudedAreaSolid", "depth": g["thickness"]}
        table = {"axis": ("Axis", "Curve2D", [{"kind": "polyline", "points": [g["outline"][0], g["outline"][1]]}]), "footprint": ("FootPrint", "Curve2D", rings),
                 "plan": ("Body", "SweptSolid", rings), "section": ("Body", "SweptSolid", [strip]), "elevation": ("Body", "SweptSolid", [strip]),
                 "3d": ("Body", "SweptSolid", [{"kind": "prism", "polygon": g["outline"], "z0": g["top"] - g["thickness"], "z1": g["top"], "holes": [{"polygon": v} for v in g["voids"]]}]),
                 "body": ("Body", "SweptSolid", [{"kind": "extrusion", "profile": {"kind": "polygon", "outer": g["outline"], "inner": g["voids"]}, "position": [0, 0, g["top"] - g["thickness"]],
                                                   "direction": [0, 0, 1], "depth": g["thickness"]}])}
    elif cls in ("IfcWindow", "IfcOpeningElement"):
        opening = obj if cls == "IfcOpeningElement" else _find(m, obj["geometry"]["opening"])
        wall = next(w for w in _wall_geometry_list(m) if w["tag"] == opening["geometry"]["wall"])
        o = next(o for o in wall["openings"] if o["tag"] == opening["tag"])
        rect = {"kind": "polygon", "role": "glazing" if cls == "IfcWindow" else "hole", "points": [[o["start"], o["sill_z"]], [o["end"], o["sill_z"]], [o["end"], o["head_z"]], [o["start"], o["head_z"]]]}
        line = {"kind": "polyline", "points": [o["p0"], o["p1"]]}
        derived = {**quantities(obj["tag"], m), "cut_z": _round((o["sill_z"] + o["head_z"]) / 2), "wall": wall["tag"]}
        table = {"axis": ("Axis", "Curve2D", [line]), "footprint": ("FootPrint", "Curve2D", [line]), "plan": ("Body", "SweptSolid", [line]), "section": ("Body", "SweptSolid", [rect]),
                 "elevation": ("Body", "SweptSolid", [rect]), "3d": ("Body", "SweptSolid", [{"kind": "prism", "polygon": [o["p0"], o["p1"]], "z0": o["sill_z"], "z1": o["head_z"], "holes": []}]),
                 "body": ("Body", "SweptSolid", [{"kind": "extrusion", "profile": {"kind": "rectangle", "length": derived["width"], "height": derived["height"]}, "position": [o["p0"][0], o["p0"][1], o["sill_z"]],
                                                   "direction": [0, 0, 1], "depth": derived["height"]}])}
    else:
        raise ValueError(f"No representation for {cls}")
    identifier, kind_type, primitives = table[kind]
    return {"key": obj["tag"], "guid": obj["guid"], "kind": kind, "identifier": identifier, "type": kind_type, "primitives": primitives, "derived": derived}


def _wall_geometry_list(model: dict) -> list:
    return [_wall_geometry(model, w) for w in _by_class(model, "IfcWall", "STANDARD")]


def _bump(model: dict, tags: list) -> None:
    model["revision"] += 1
    registry = _registry(model)
    for tag in tags:
        registry[tag]["metadata"]["revision"] = model["revision"]
    for spec in model["models"]:
        if any(tag in spec["element_tags"] for tag in tags):
            spec["revision"] = model["revision"]


def demo_event(name: str, model: dict | None = None) -> dict:
    """🎬 Return an authored scene event payload by name (sill, roof_soil, roof_soil_alt)."""
    events = _m(model)["demo_events"]
    if name not in events:
        raise ValueError(f"Unknown demo event: {name}")
    return deepcopy(events[name])


def _number(value, name: str) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        raise ValueError(f"{name} must be a finite number")
    return value


def apply_event(event: dict, model: dict | None = None) -> dict:
    """⚡ Apply one design event to a copy of the model, raise revisions and keep every GlobalId."""
    source = deepcopy(_m(model))
    kind = event.get("type")
    allowed = {"OpeningResized": {"type", "object_guid", "sill", "head"}, "LayerThicknessChanged": {"type", "object_guid", "layer", "value", "unit"}}
    if kind not in allowed or not set(event) - {"basis", "note"} == allowed[kind]:
        raise ValueError("Invalid event")
    registry = _registry(source)
    target = registry.get(event["object_guid"])
    if target is None:
        raise ValueError("Unknown object")
    if kind == "OpeningResized":
        if target["class"] != "IfcWindow":
            raise ValueError("OpeningResized needs a window")
        sill, head = _number(event["sill"], "sill"), _number(event["head"], "head")
        opening = registry[target["geometry"]["opening"]]
        wall = registry[opening["geometry"]["wall"]]
        wall_height = _top(source, wall["geometry"]["top_level"]) - _z(source, wall["geometry"]["base_level"])
        if not 0 <= sill < head <= wall_height:
            raise ValueError("Opening must lie between floor and wall top")
        opening["attributes"]["sill"]["value"], opening["attributes"]["head"]["value"] = sill, head
        _bump(source, [target["tag"], opening["tag"], wall["tag"]])
    else:
        if target["class"] != "IfcSlab" or event["unit"] != "m":
            raise ValueError("LayerThicknessChanged needs a layered slab and metres")
        value = _number(event["value"], "value")
        layerset = _material(source, target)
        if value <= 0 or layerset is None or "layers" not in layerset:
            raise ValueError("Layer thickness must be positive on a layered slab")
        layer = next((l for l in layerset["layers"] if l["id"] == event["layer"]), None)
        if layer is None:
            raise ValueError("Unknown layer")
        layer["thickness"]["value"] = value
        _bump(source, [target["tag"]])
    return source


def _link_values(model: dict, kind: str) -> dict:
    function = globals().get("link_values")
    if function is None:
        raise RuntimeError("The analysis submodule is missing")
    return function(model, kind)


def consistency_issues(model: dict | None = None) -> list:
    """🚦 Compare analysis links with the current model: revision lag and differing used values."""
    m = _m(model)
    registry = _registry(m)
    issues = []
    for link in m["links"]:
        target = registry.get(link["object_guid"])
        if target is None:
            issues.append({"code": "unknown-object", "id": link["id"], "object_guid": link["object_guid"], "expected": None, "actual": link["object_guid"]})
            continue
        if _revision(m, target) > link["source_revision"]:
            issues.append({"code": "stale-link", "id": link["id"], "object_guid": link["object_guid"], "expected": _revision(m, target), "actual": link["source_revision"]})
        current = _link_values(m, link["kind"])
        for name, used in link["used"].items():
            if not math.isclose(current[name], used, rel_tol=1e-9, abs_tol=1e-9):
                issues.append({"code": "stale-value", "id": link["id"], "object_guid": link["object_guid"], "key": name, "expected": current[name], "actual": used})
    for spec in m["models"]:
        for tag in spec["element_tags"]:
            if tag not in registry:
                issues.append({"code": "unknown-object", "id": spec["id"], "object_guid": tag, "expected": None, "actual": tag})
    return issues


def resolve_link(model: dict | None, link_id: str) -> dict:
    """🔧 Regenerate one analysis link from the current model on a copy and keep the revision."""
    source = deepcopy(_m(model))
    link = next((x for x in source["links"] if x["id"] == link_id), None)
    if link is None:
        raise ValueError(f"Unknown link: {link_id}")
    link["used"] = _link_values(source, link["kind"])
    link["source_revision"] = source["revision"]
    return source


_exchange = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
_analysis = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")
step_excerpt, mesh, ids_check = _exchange.step_excerpt, _exchange.mesh, _exchange.ids_check
translate_structure, tributary = _analysis.translate_structure, _analysis.tributary
clash_check, link_values = _analysis.clash_check, _analysis.link_values
