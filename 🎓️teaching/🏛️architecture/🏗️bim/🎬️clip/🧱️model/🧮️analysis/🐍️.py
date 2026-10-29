"""🔬 Analysis derivations of the Villa Savoye teaching model: structural analysis translation, tributary-area estimate, clearance check."""

from __future__ import annotations

import importlib
import math

EPS = 1e-9
LEVELS = {"first": "FLOOR", "roof": "ROOF"}
LINK_KINDS = ("slab-load", "roof-load", "piloti-section")
LOAD_NAMES = {"self_weight": ("Self-weight", "Eigengewicht"), "finishes": ("Finishes", "Ausbau"), "imposed": ("Imposed load", "Nutzlast"), "total": ("Total", "Summe")}
ZONE_NAMES = {"slab": {"en": "First-floor slab", "de": "Geschossdecke"}}
METHOD = {"en": "tributary-area estimate", "de": "Lasteinzugsflächen-Näherung"}
EXTRA_ASSUMPTIONS = (
    ("linear-elastic", "Concrete is treated as homogeneous, isotropic and linear elastic; cracking and creep are ignored.",
     "Beton gilt als homogen, isotrop und linear elastisch; Rissbildung und Kriechen bleiben unberücksichtigt."),
)
ASSUMPTION_ORDER = ("mid-surface", "mesh-grid", "ramp-void", "material", "linear-elastic", "support-fixed", "loads", "load-path")


def _core():
    return importlib.import_module(__package__.rpartition(".")[0] + ".🐍️")


def _context(model):
    core = _core()
    return core, (core.load_model() if model is None else model)


def _round(value: float, digits: int = 9) -> float:
    return round(value, digits)


def _fmt(value: float, language: str, digits: int = 2) -> str:
    text = f"{value:,.{digits}f}"
    return text if language == "en" else text.replace(",", "\0").replace(".", ",").replace("\0", ".")


def _area(polygon: list) -> float:
    return abs(sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(polygon, polygon[1:] + polygon[:1]))) / 2


def _ccw(polygon: list) -> list:
    """🧹 Remove exact zero-area ring excursions while preserving every real facet."""
    ring = []
    for point in polygon:
        point = list(point)
        if ring and point == ring[-1]:
            continue
        while len(ring)>1 and _side(point,ring[-2],ring[-1]) == 0:
            ring.pop()
        if not ring or point != ring[-1]:
            ring.append(point)
    if len(ring)>1 and ring[0] == ring[-1]:
        ring.pop()
    while len(ring)>2:
        if _side(ring[1],ring[-1],ring[0]) == 0:
            ring.pop(0)
        elif _side(ring[0],ring[-2],ring[-1]) == 0:
            ring.pop()
        else:
            break
    signed = sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(ring,ring[1:]+ring[:1]))
    return ring if signed>=0 else list(reversed(ring))


def _side(p, a, b) -> float:
    return (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])


def _half(polygon: list, a, b, keep_left: bool = True, digits: int | None = 9) -> list:
    sign = 1 if keep_left else -1
    out = []
    for p, q in zip(polygon, polygon[1:] + polygon[:1]):
        sp, sq = sign * _side(p, a, b), sign * _side(q, a, b)
        if sp >= 0:
            out.append(p)
        if (sp > 0 > sq) or (sp < 0 < sq):
            t = sp / (sp - sq)
            out.append([p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1])])
    return _ccw(out if digits is None else [[_round(x,digits), _round(y,digits)] for x, y in out])


def _convex(polygon: list) -> bool:
    signs = {_side(c, a, b) > EPS for a, b, c in zip(polygon, polygon[1:] + polygon[:1], polygon[2:] + polygon[:2]) if abs(_side(c, a, b)) > EPS}
    return len(signs) <= 1


def _convex_parts(polygon: list) -> list:
    """🔺 Partition a simple polygon into convex ears without changing its occupied area."""
    ring = _ccw(polygon)
    if _convex(ring):
        return [ring]
    parts = []
    while len(ring) > 3:
        for i, b in enumerate(ring):
            a, c = ring[i - 1], ring[(i + 1) % len(ring)]
            if _side(c, a, b) <= EPS:
                continue
            others = [p for j, p in enumerate(ring) if j not in (i, (i - 1) % len(ring), (i + 1) % len(ring))]
            if any(all(_side(p, u, v) >= -EPS for u, v in ((a, b), (b, c), (c, a))) for p in others):
                continue
            parts.append([a, b, c])
            ring.pop(i)
            break
        else:
            raise ValueError("A surface ring must be simple and have nonzero area")
    return parts + [ring]


def _voronoi(supports: list, outline: list) -> list:
    """📍 Clip perpendicular bisectors to the slab, partitioning sparse physical supports once."""
    cells = []
    for index, support in enumerate(supports):
        cell = _ccw(outline)
        x, y = support["x"], support["y"]
        for other_index, other in enumerate(supports):
            if other_index == index:
                continue
            dx, dy = other["x"] - x, other["y"] - y
            if math.hypot(dx, dy) <= EPS:
                raise ValueError("Supports on one slab cannot share a plan position")
            mid = [(x + other["x"]) / 2, (y + other["y"]) / 2]
            cell = _half(cell, mid, [mid[0] - dy, mid[1] + dx])
            if not cell:
                break
        cells.append(cell)
    return cells


def _inside(point, ring: list) -> bool:
    """🔎 Test a plan point against one polygon ring by even-odd crossings."""
    x, y = point
    result = False
    for a, b in zip(ring, ring[1:] + ring[:1]):
        if (a[1] > y) != (b[1] > y) and x < a[0] + (y - a[1]) * (b[0] - a[0]) / (b[1] - a[1]):
            result = not result
    return result


def _bearing(column: dict, slab: dict) -> bool:
    """🏢 Intersect real profile sections with the matching occupied regional soffits."""
    for surface in slab["parts"]:
        for part in column["parts"]:
            if not part["z0"] < surface["z0"] - EPS or part["z1"] < surface["z0"] - EPS:
                continue
            outer = _convex_parts(part["outline"])
            for face in _convex_parts(surface["outline"]):
                contact = sum(_area(_clip(profile,face)) for profile in outer)
                contact -= sum(_area(_clip(hole,face)) for hole in part.get("holes",()))
                if contact > EPS:
                    return True
    return False

def _supports(geometry: dict, slab: dict) -> list:
    """🏛️ Select columns physically reaching an occupied regional slab underside."""
    return [column for column in geometry["pilotis"] if _bearing(column, slab)]

def _clip(subject: list, clip: list, digits: int | None = 9) -> list:
    """✂️ Preserve operand precision through every half-plane before publishing rounded coordinates."""
    clip = _ccw(clip)
    if not _convex(clip):
        raise ValueError("Tributary cells are clipped against convex slab outlines only")
    for a, b in zip(clip, clip[1:] + clip[:1]):
        if not subject:
            break
        subject = _half(subject, a, b,digits=None)
    return subject if digits is None else _ccw([[_round(x,digits),_round(y,digits)] for x,y in subject])


def _subtract(piece: list, void: list, digits: int = 9, tolerance: float = EPS) -> list:
    if not piece:
        return []
    if not void:
        return [piece]
    for axis in (0,1):
        a,b = [p[axis] for p in piece],[p[axis] for p in void]
        if max(a) <= min(b) or max(b) <= min(a):
            return [piece]
    void = _ccw(void)
    if not _convex(void):
        pieces = [piece]
        for triangle in _convex_parts(void):
            pieces = [part for p in pieces for part in _subtract(p, triangle,digits,tolerance)]
        return pieces
    out, rest = [], piece
    for a, b in zip(void, void[1:] + void[:1]):
        out.append(_half(rest, a, b, False,digits))
        rest = _half(rest, a, b, True,digits)
    return [p for p in out if len(p) >= 3 and _area(p) > tolerance]


def _on(q, a, b, tolerance: float = EPS) -> bool:
    cross = abs(_side(q, a, b))
    along = (q[0] - a[0]) * (b[0] - a[0]) + (q[1] - a[1]) * (b[1] - a[1])
    return cross < tolerance and tolerance < along < math.dist(a, b) ** 2 - tolerance


def _signed(polygon: list) -> float:
    return sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(polygon, polygon[1:] + polygon[:1])) / 2


def _loops(pieces: list, tolerance: float = EPS) -> list:
    points = {tuple(p) for piece in pieces for p in piece}
    edges = {}
    for piece in pieces:
        for a, b in zip(piece, piece[1:] + piece[:1]):
            a, b = tuple(a), tuple(b)
            chain = [a] + sorted((q for q in points if _on(q, a, b, tolerance)), key=lambda q: math.dist(a, q)) + [b]
            for u, v in zip(chain, chain[1:]):
                if (v, u) in edges:
                    del edges[(v, u)]
                else:
                    edges[(u, v)] = None
    following = {}
    for u, v in edges:
        following.setdefault(u, []).append(v)
    loops = []
    while following:
        start = next(iter(following))
        loop, point = [], start
        while point in following:
            loop.append(list(point))
            target = following[point].pop(0)
            if not following[point]:
                del following[point]
            point = target
            if point == start:
                break
        loops.append([p for i, p in enumerate(loop) if abs(_side(p, loop[i - 1], loop[(i + 1) % len(loop)])) > tolerance])
    return sorted(loops, key=lambda loop: -abs(_signed(loop)))


def _outline(pieces: list) -> list:
    return next((loop for loop in _loops(pieces) if _signed(loop) > 0), [])


def _slab(geometry: dict, level: str) -> dict:
    if level not in LEVELS:
        raise ValueError(f"Unknown level: {level}")
    tag = geometry["roles"][{"first": "floor", "roof": "roof"}[level]]
    slab = next(slab for slab in geometry["slabs"] if slab["tag"] == tag)
    return {**slab,"parts":slab.get("full_parts",slab["parts"]),"finish_parts":slab.get("full_finish_parts",slab.get("finish_parts",[]))}

def _entry(value: float, basis: str, en: str, de: str, **extra) -> dict:
    return {"value": _round(value), "unit": "kN/m²", "basis": basis, "en": en, "de": de, **extra}


def _composite_weight(core, model: dict, composite: dict, depth: float, material: str | None = None) -> dict:
    """🧱 Estimate repeated-field RC and hollow-unit-envelope weight from sourced geometry."""
    compression = composite["compression_layer"]["value"]
    width, pitch = composite["rib_width"]["value"], composite["rib_pitch"]["value"]
    if not 0 < compression <= depth or not 0 < width <= pitch:
        raise ValueError("Composite compression layer and rib dimensions must fit the structural depth")
    concrete = depth if material else compression + width / pitch * (depth - compression)
    components = {}
    for key, thickness in (("concrete", concrete), ("ceramic", depth - concrete)):
        tag = material if material and key == "concrete" else composite[key]
        density = core._find(model, tag)["density"]
        components[key] = {"material": tag, "thickness": thickness, "density": density["value"], "basis": density["basis"], "value": density["value"] * thickness}
    weight = sum(c["value"] for c in components.values())
    return _entry(weight, "assumed", *LOAD_NAMES["self_weight"], thickness=depth, density=weight / depth, components=components)


def _local_breakdown(core, m: dict, slab: dict, level: str, depth: float, zone: str, material: str | None = None) -> dict:
    authored = core._find(m, slab["tag"])
    assembly = core._material(m, authored)
    field = next(layer["material"] for layer in assembly["layers"] if layer["id"] == "structural") if "layers" in assembly else assembly["tag"]
    solid = material if material != field else None
    out = {"self_weight": _composite_weight(core, m, authored["geometry"]["composite"], depth, solid)}
    if level == "first":
        finish = m["loads"]["finishes"]["first"]
        imposed = m["loads"]["imposed"]["first"]
        out["finishes"] = _entry(finish["value"], finish["basis"], *LOAD_NAMES["finishes"])
    else:
        layers = core._material(m, authored)["layers"]
        nominal = next(layer["thickness"]["value"] for layer in layers if layer["id"] == "structural")
        layer = next(layer for layer in layers if layer["id"] == "paving")
        reserve = layer["thickness"]["value"] - (depth - nominal)
        if reserve < -EPS:
            raise ValueError("Roof paving reserve must stay above every structural region")
        density = core._find(m, layer["material"])["density"]
        out["paving"] = _entry(density["value"] * reserve, "assumed", layer["name"]["en"], layer["name"]["de"], thickness=reserve, density=density["value"])
        imposed = m["loads"]["imposed"][zone]
    out["imposed"] = _entry(imposed["value"], imposed["basis"], *LOAD_NAMES["imposed"])
    out["total"] = _entry(sum(value["value"] for value in out.values()), "assumed", *LOAD_NAMES["total"])
    return out


def _mean_breakdown(regions: list) -> dict:
    area = sum(region["area"] for region in regions)
    if area <= EPS:
        raise ValueError("A load zone must have occupied area")
    names = list(dict.fromkeys(key for region in regions for key in region["breakdown"]))
    out = {}
    for key in names:
        entries = [(region["area"], region["breakdown"][key]) for region in regions if key in region["breakdown"]]
        first = entries[0][1]
        out[key] = _entry(sum(weight * entry["value"] for weight, entry in entries) / area, "assumed", first["en"], first["de"])
        if all("thickness" in entry for _, entry in entries):
            out[key]["thickness"] = sum(weight * entry["thickness"] for weight, entry in entries) / area
            out[key]["density"] = out[key]["value"] / out[key]["thickness"] if out[key]["thickness"] else 0.
        if all("components" in entry for _, entry in entries):
            out[key]["components"] = {name: {**first["components"][name], "thickness": sum(weight * entry["components"][name]["thickness"] for weight, entry in entries) / area,
                                     "value": sum(weight * entry["components"][name]["value"] for weight, entry in entries) / area} for name in first["components"]}
    return out


def _breakdown(core, m: dict, geometry: dict, level: str, zone: str | None = None) -> dict:
    zones = _zones(core, m, geometry, level)
    regions = [region for item in zones if zone is None or item["id"] == zone for region in item["regions"]]
    return _mean_breakdown(regions)

def _zones(core, m: dict, geometry: dict, level: str) -> list:
    slab = _slab(geometry, level)
    if level == "first":
        surfaces = [{"id": "slab", "name": ZONE_NAMES["slab"], "polygon": None}]
    else:
        explicit = slab.get("zones", {})
        ids = list(dict.fromkeys(zone for layer in slab["layers"] for zone in layer["zones"]))
        rest = [zone for zone in ids if zone not in explicit]
        if len(rest) != 1 or not set(explicit) <= set(ids):
            raise ValueError("The roof build-up must name explicit zones and one remaining zone")
        spaces = core._find(m, slab["tag"])["geometry"]["zones"]
        surfaces = [{"id": zone, "name": core._find(m, spaces[zone])["name"] if zone in explicit else {"en": "Roof", "de": "Dach"},
                     "polygon": explicit.get(zone)} for zone in ids]
    for surface in surfaces:
        regions = []
        for region in slab["regions"]:
            pieces = [part["outline"] for part in slab["parts"] if part["region"] == region["tag"]]
            if surface["polygon"]:
                pieces = [clipped for piece in pieces for bound in _convex_parts(surface["polygon"]) for clipped in [_clip(piece, bound)] if len(clipped) >= 3 and _area(clipped) > EPS]
            else:
                for other in surfaces:
                    if other["polygon"]:
                        pieces = [remaining for piece in pieces for remaining in _subtract(piece, other["polygon"])]
            area = sum(_area(piece) for piece in pieces)
            if area > EPS:
                regions.append({"tag": region["tag"], "area": area, "thickness": region["thickness"], "pieces": pieces,
                                "breakdown": _local_breakdown(core, m, slab, level, region["thickness"], surface["id"], region.get("material"))})
        surface["regions"] = regions
        surface["pieces"] = [piece for region in regions for piece in region["pieces"]]
        surface["breakdown"] = _mean_breakdown(regions)
    return surfaces

def _names(zones: list, key: str) -> tuple:
    entry = next(z["breakdown"][key] for z in zones if key in z["breakdown"])
    return entry["en"], entry["de"]


def _split(pieces: list, zones: list) -> dict:
    return {zone["id"]: [cut for piece in pieces for bound in zone["pieces"] for cut in [_clip(piece, bound)] if len(cut) >= 3 and _area(cut) > EPS] for zone in zones}

def _labels(lines: list, grid_lines: list, names: list) -> list:
    return [names[grid_lines.index(v)] if v in grid_lines else f"X{i}" for i, v in enumerate(lines)]


def tributary(model: dict | None = None, level: str = "first") -> dict:
    """📐 Estimate the load each piloti receives from its tributary cell of the first-floor or roof slab (zoned area load times area; not a finite element result).

    See https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralSurfaceMember.htm for the analysis member this estimate replaces.
    """
    core, m = _context(model)
    geometry = core.building_geometry(m)
    slab = _slab(geometry, level)
    zones = _zones(core, m, geometry, level)
    grid = geometry["grid"]
    xs, ys = grid["x"], grid["y"]
    outline = slab["outline"]
    x_min, x_max = min(p[0] for p in outline), max(p[0] for p in outline)
    y_min, y_max = min(p[1] for p in outline), max(p[1] for p in outline)
    x_cuts = [x_min] + [(a + b) / 2 for a, b in zip(xs, xs[1:])] + [x_max]
    y_cuts = [y_min] + [(a + b) / 2 for a, b in zip(ys, ys[1:])] + [y_max]
    slab_area = core.quantities(slab["tag"], m)["area"]
    out = []
    supports = _supports(geometry, slab)
    for pil, cell in zip(supports, _voronoi(supports, [[x_min,y_min],[x_max,y_min],[x_max,y_max],[x_min,y_max]])):
        i = xs.index(pil["x"]) if pil["x"] in xs else None
        j = ys.index(pil["y"]) if pil["y"] in ys else None
        pieces = [cut for part in slab["parts"] for cut in [_clip(cell, part["outline"])] if len(cut) >= 3 and _area(cut) > EPS]
        area = _round(sum(_area(piece) for piece in pieces))
        shares = {}
        for zone in zones:
            regional = []
            for region in zone["regions"]:
                clipped = [cut for piece in region["pieces"] for cut in [_clip(cell, piece)] if len(cut) >= 3 and _area(cut) > EPS]
                occupied = sum(_area(piece) for piece in clipped)
                regional.append({"tag": region["tag"], "area": occupied, "reaction": occupied * region["breakdown"]["total"]["value"], "pieces": clipped})
            occupied = sum(region["area"] for region in regional)
            shares[zone["id"]] = {"area": _round(occupied), "pieces": [piece for region in regional for piece in region["pieces"]], "regions": regional,
                                  "share": occupied / area if area else 0., "reaction": _round(sum(region["reaction"] for region in regional))}
        edges = (j in (0, len(ys) - 1)) + (i in (0, len(xs) - 1))
        out.append({"tag": pil["tag"], "guid": pil["guid"], "row": grid["labels_y"][j] if j is not None else None, "column": grid["labels_x"][i] if i is not None else None, "x": pil["x"], "y": pil["y"],
                    "kind": ("interior", "edge", "corner")[edges], "area": area, "reaction": _round(sum(v["reaction"] for v in shares.values())), "share": area / slab_area,
                    "zones": shares, "cell": _outline(pieces), "pieces": pieces})
    total_area = _round(sum(p["area"] for p in out))
    total = _round(sum(p["reaction"] for p in out))
    listed = []
    for z in zones:
        pieces = z["pieces"]
        loops = _loops(pieces)
        area = _round(sum(p["zones"][z["id"]]["area"] for p in out))
        listed.append({"id": z["id"], "name": z["name"], "area": area, "area_load": z["breakdown"]["total"]["value"], "load_breakdown": z["breakdown"], "regions": z["regions"], "share": area / slab_area,
                       "reaction": _round(sum(p["zones"][z["id"]]["reaction"] for p in out)), "outline": next((l for l in loops if _signed(l) > 0), []),
                       "outlines": [l for l in loops if _signed(l) > 0], "holes": [l for l in loops if _signed(l) < 0], "pieces": pieces})
    mean = _mean_breakdown([region for zone in zones for region in zone["regions"]])
    return {"level": level, "method": METHOD["en"], "method_de": METHOD["de"], "slab_tag": slab["tag"], "slab_guid": slab["guid"], "area_load": mean["total"]["value"], "load_breakdown": mean,
            "slab_area": slab_area, "total_area": total_area, "total_reaction": total, "balanced": math.isclose(total_area, slab_area, rel_tol=1e-9), "zones": listed,
            "cantilever": {"west": _round(xs[0] - x_min), "east": _round(x_max - xs[-1]), "south": _round(ys[0] - y_min), "north": _round(y_max - ys[-1])},
            "cuts": {"x": x_cuts, "y": y_cuts}, "pilotis": out}


def link_values(model: dict, kind: str) -> dict:
    """🧷 Give the plain numbers a stored analysis link relies on, so a later model state can be compared with them."""
    core, m = _context(model)
    if kind == "slab-load":
        first = tributary(m, "first")
        return {"thickness": first["load_breakdown"]["self_weight"]["thickness"], "area_load": first["area_load"], "total_reaction": first["total_reaction"]}
    if kind == "roof-load":
        roof = tributary(m, "roof")
        return {"area_load": roof["area_load"], **{f"{z['id']}_area_load": z["area_load"] for z in roof["zones"]}, "total_reaction": roof["total_reaction"],
                **{f"reaction_{p['tag']}": p["reaction"] for p in roof["pilotis"]}}
    if kind == "piloti-section":
        link = next(link for link in m["links"] if link["kind"] == kind)
        section = core.quantities(link["object_guid"], m)
        return {key: section[key] for key in ("diameter", "width", "depth", "height", "cross_section_area", "volume") if key in section}
    raise ValueError(f"Unknown link kind: {kind}")


def _assumptions(m: dict, loads: dict, counts: dict, slab: dict) -> list:
    given = {a["id"]: {**a, "basis": a.get("basis", "assumed")} for a in m["translation"]["assumptions"]}
    for key, en, de in EXTRA_ASSUMPTIONS:
        given.setdefault(key, {"id": key, "en": en, "de": de, "basis": "assumed"})
    density = loads["self_weight"]["density"]

    def text(language):
        values = [_fmt(density, language, 1), _fmt(loads["self_weight"]["thickness"], language), _fmt(loads["self_weight"]["value"], language), _fmt(loads["finishes"]["value"], language), _fmt(loads["imposed"]["value"], language)]
        return ("Loads on the first-floor slab: self-weight {2} kN/m² (regular-field composite estimate, mean equivalent density {0} kN/m³ and mean depth {1} m), finishes {3} kN/m², imposed {4} kN/m²; all assumed." if language == "en"
                else "Lasten auf der Geschossdecke: Eigengewicht {2} kN/m² (Verbundfeld-Näherung, mittlere Ersatzdichte {0} kN/m³ und mittlere Dicke {1} m), Ausbau {3} kN/m², Nutzlast {4} kN/m²; alle angenommen.").format(*values)

    given.setdefault("loads", {"id": "loads", "en": text("en"), "de": text("de"), "basis": "assumed"})
    for item in given.values():
        for language in ("en", "de"):
            if "{" in item[language]:
                item[language] = item[language].format_map(counts)
    order = {key: i for i, key in enumerate(ASSUMPTION_ORDER)}
    return sorted(given.values(), key=lambda a: order.get(a["id"], len(order)))


def _losses(m: dict) -> tuple:
    flat, groups = [], []
    for index, rule in enumerate(m["translation"]["losses"]):
        target = rule["applies_to"]
        hits = [e for e in [*m["elements"], *m.get("spaces", [])] if e["class"] == target["class"] and target.get("predefined_type", e["predefined_type"]) == e["predefined_type"]]
        group = f"{target['class']}{'.' + target['predefined_type'] if 'predefined_type' in target else ''}:{rule['property']}"
        for e in hits:
            flat.append({"source_tag": e["tag"], "source_guid": e["guid"], "source_class": e["class"], "property": rule["property"], "en": rule["en"], "de": rule["de"], "group": group})
        if hits:
            groups.append({"id": group, "index": index, "class": target["class"], "predefined_type": target.get("predefined_type"), "property": rule["property"], "en": rule["en"], "de": rule["de"],
                           "count": len(hits), "source_tags": [e["tag"] for e in hits]})
    return flat, groups


def translate_structure(model: dict | None = None) -> dict:
    """🏗️ Translate actual regional mid-surfaces and physical column sections with explicit idealisations."""
    core, m = _context(model)
    geometry = core.building_geometry(m)
    spec = m["translation"]
    slab = _slab(geometry, "first")
    grid = geometry["grid"]
    physical = _supports(geometry, slab) + _supports(geometry, _slab(geometry, "roof"))
    xs = sorted({*[p[0] for region in slab["regions"] for p in region["outline"]], *grid["x"], *[p["x"] for p in physical]})
    ys = sorted({*[p[1] for region in slab["regions"] for p in region["outline"]], *grid["y"], *[p["y"] for p in physical]})
    columns, rows = _labels(xs, grid["x"], grid["labels_x"]), _labels(ys, grid["y"], grid["labels_y"])
    nodes, shells, lookup = [], [], {}
    def node(point, z):
        key = tuple(_round(value) for value in [*point, z])
        if key not in lookup:
            identifier = f"N-{len(nodes):05d}"
            lookup[key] = identifier
            nodes.append({"id": identifier, "role": "slab", "position": list(key)})
        return lookup[key]
    def mid(part):
        return (part["z0"] + part["z1"]) / 2
    def station(point):
        candidates = [part for part in slab["parts"] if _inside(point, part["outline"]) or any(_on(point, u, v) or math.dist(point, u) < EPS for u, v in zip(part["outline"], part["outline"][1:] + part["outline"][:1]))]
        if not candidates:
            candidates = sorted(slab["parts"], key=lambda part: min(math.dist(point, p) for p in part["outline"]))[:1]
        return mid(max(candidates, key=mid))
    for part_index, part in enumerate(slab["parts"]):
        xmin, xmax = min(p[0] for p in part["outline"]), max(p[0] for p in part["outline"])
        ymin, ymax = min(p[1] for p in part["outline"]), max(p[1] for p in part["outline"])
        for j, (lo_y, hi_y) in enumerate(zip(ys, ys[1:])):
            if hi_y <= ymin + EPS or lo_y >= ymax - EPS:
                continue
            for i, (lo_x, hi_x) in enumerate(zip(xs, xs[1:])):
                if hi_x <= xmin + EPS or lo_x >= xmax - EPS:
                    continue
                piece = _clip(part["outline"], [[lo_x,lo_y],[hi_x,lo_y],[hi_x,hi_y],[lo_x,hi_y]],digits=None)
                if len(piece) < 3 or _area(piece) <= EPS:
                    continue
                shells.append({"id": f"SH-{part_index}-{j}-{i}", "nodes": [node(p, mid(part)) for p in piece], "thickness": part["thickness"],
                               "mid_surface": _round(mid(part)), "region": part["region"], "source_guid": slab["guid"], "source_tag": slab["tag"],
                               "area": _round(_area(piece)), "bounds": [min(p[0] for p in piece), min(p[1] for p in piece), max(p[0] for p in piece), max(p[1] for p in piece)]})
    members, supports = [], []
    for pil in _supports(geometry, slab):
        foot = {"id": f"F-{pil['tag']}", "role": "base", "position": [pil["x"], pil["y"], pil["base"]]}
        nodes.append(foot)
        z = station([pil["x"], pil["y"]])
        sections = [{"profile": part["profile"], "cross_section_area":part["area"],"second_moments":part["second_moments"],"section_scope":part["section_scope"],"void_area":part["void_area"],"base":part["z0"],"top":part["z1"]} for part in pil["parts"]]
        member = {"id": f"M-{pil['tag']}", "kind": "curve", "ifc_class": "IfcStructuralCurveMember", "source_guid": pil["guid"], "source_tag": pil["tag"],
                  "nodes": [foot["id"], node([pil["x"], pil["y"]], z)], "length": _round(z - pil["base"]), "physical_height": _round(pil["top"] - pil["base"]), "sections": sections,"stiffness_basis":"not_assessed"}
        if len(sections) == 1:
            member["profile"],member["cross_section_area"] = sections[0]["profile"],sections[0]["cross_section_area"]
            member["second_moments"] = sections[0]["second_moments"]
            if member["profile"]["shape"] == "circle":
                member["diameter"] = member["profile"]["diameter"]["value"]
        members.append(member)
        supports.append({"node": foot["id"], "position": foot["position"], "ifc_class": "IfcStructuralPointConnection", "condition": spec["boundary"]["class"], "fixity": spec["boundary"]["id"],
                         "basis": spec["boundary"]["basis"], "label": {"en": spec["boundary"]["en"], "de": spec["boundary"]["de"]}, "source_tag": pil["tag"], "source_guid": pil["guid"]})
    members += [{"id": f"M-{shell['id']}", "kind": "surface", "ifc_class": "IfcStructuralSurfaceMember", "source_guid": slab["guid"], "source_tag": slab["tag"],
                 "nodes": shell["nodes"], "thickness": shell["thickness"], "mid_surface": shell["mid_surface"], "region": shell["region"], "shell": shell["id"]} for shell in shells]
    first = _breakdown(core, m, geometry, "first")
    loads = {**first, "area_load": first["total"]["value"]}
    roof = tributary(m, "roof")
    point_loads = [{"node": node([p["x"], p["y"]], station([p["x"], p["y"]])), "value": p["reaction"], "unit": "kN", "basis": "assumed", "level": "roof", "source_tag": p["tag"]}
                   for p in roof["pilotis"] if p["reaction"] > 0]
    by_plan = {}
    for item in nodes:
        if item["role"] == "slab":
            by_plan.setdefault(tuple(item["position"][:2]), []).append(item)
    offsets = [{"nodes": [a["id"], b["id"]], "offset": _round(b["position"][2] - a["position"][2]), "basis": "assumed"}
               for group in by_plan.values() for ordered in [sorted(group, key=lambda item: item["position"][2])] for a, b in zip(ordered, ordered[1:])]
    slab_nodes = sum(item["role"] == "slab" for item in nodes)
    counts = {"curve_members": len(supports), "surface_members": len(shells), "members": len(members), "shells": len(shells), "slab_nodes": slab_nodes, "base_nodes": len(supports),
              "nodes": len(nodes), "supports": len(supports), "point_loads": len(point_loads)}
    quantities = core.quantities(slab["tag"], m)
    mean_z = sum(_area(part["outline"]) * mid(part) for part in slab["parts"]) / quantities["area"]
    losses, loss_groups = _losses(m)
    mapping = [{**rule, "count": counts[{"curve": "curve_members", "surface": "surface_members", "support": "supports"}[rule["kind"]]]} for rule in spec["mapping"]]
    return {"source_schema": spec["source_schema"], "target_schema": spec["target_schema"], "target_domain": spec["target_domain"], "model_class": "IfcStructuralAnalysisModel",
            "source_revision": core.partial_model("structure", m)["revision"], "mid_surface": {"z": _round(mean_z), "level": "first", "elevation": slab["top"], "thickness": slab["thickness"],
            "regions": [{"tag": region["tag"], "z": _round(region["top"] - region["thickness"] / 2), "thickness": region["thickness"]} for region in slab["regions"]]},
            "mesh": {"x_lines": xs, "y_lines": ys, "columns": columns, "rows": rows}, "slab": {"tag": slab["tag"], "guid": slab["guid"], "gross_area": quantities["gross_area"],
            "void_area": quantities["void_area"], "net_area": quantities["area"], "thickness": slab["thickness"]}, "members": members, "nodes": nodes, "shells": shells, "supports": supports,
            "offset_links": offsets, "loads": loads, "point_loads": point_loads, "mapping": mapping, "assumptions": _assumptions(m, loads, counts, slab), "losses": losses, "loss_groups": loss_groups,
            "counts": counts, "used": {kind: link_values(m, kind) for kind in LINK_KINDS}}

def _closest(p1, q1, p2, q2) -> tuple:
    d1, d2, r = [b - a for a, b in zip(p1, q1)], [b - a for a, b in zip(p2, q2)], [a - b for a, b in zip(p1, p2)]

    def dot(u, v):
        return sum(a * b for a, b in zip(u, v))

    def clamp(v):
        return max(0.0, min(1.0, v))

    a, e, f = dot(d1, d1), dot(d2, d2), dot(d2, r)
    if a <= EPS and e <= EPS:
        s = t = 0.0
    elif a <= EPS:
        s, t = 0.0, clamp(f / e)
    else:
        c = dot(d1, r)
        if e <= EPS:
            s, t = clamp(-c / a), 0.0
        else:
            b = dot(d1, d2)
            denom = a * e - b * b
            if denom > EPS:
                s = clamp((b * f - c * e) / denom)
            else:
                u = [-c / a, (b - c) / a]
                low, high = max(0.0, min(u)), min(1.0, max(u))
                s = (low + high) / 2 if low <= high else clamp(sum(u) / 2)
            t = (b * s + f) / e
            if t < 0:
                s, t = clamp(-c / a), 0.0
            elif t > 1:
                s, t = clamp((b - c) / a), 1.0
    return [p + d * s for p, d in zip(p1, d1)], [p + d * t for p, d in zip(p2, d2)]


def _occupied_components(core, parts: list, distance) -> list:
    """🧱 Resolve exact convex occupied components without filling inner contours."""
    result = []
    for part in parts:
        if "mesh" in part or "section" in part or "top_outline" in part:
            vertices = core.part_vertices(part)
            if "mesh" in part:
                for face in core.part_faces(part):
                    a = vertices[face[0]]
                    normal = [0.,0.,0.]
                    for i in range(1,len(face)-1):
                        u,v = [vertices[face[i]][k]-a[k] for k in range(3)],[vertices[face[i+1]][k]-a[k] for k in range(3)]
                        cross = [u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
                        normal = [normal[k]+cross[k] for k in range(3)]
                    length = math.sqrt(sum(n*n for n in normal))
                    if length <= 1e-20:
                        raise ValueError("Clash mesh face requires positive area")
                    signed = [sum(normal[k]*(p[k]-a[k]) for k in range(3))/length for p in vertices]
                    if min(signed)<-EPS and max(signed)>EPS:
                        raise ValueError("Clash mesh components must be convex")
            result.append({"vertices":vertices,"bounds":distance.bounds(vertices)})
        else:
            for ring in core.part_regions(part,(part["z0"]+part["z1"])/2):
                for face in _convex_parts(ring):
                    vertices = [[*p,part[z]] for z in ("z0","z1") for p in face]
                    result.append({"vertices":vertices,"bounds":distance.bounds(vertices)})
    if not result:
        raise ValueError("Clash geometry requires occupied material")
    return result


def _hosting_relations(core, model: dict, columns: dict, pipes: dict, rules: dict, distance) -> dict:
    """🔗 Validate explicit native conduit hosts against their gross physical envelopes."""
    hosts = {}
    for rel in model["relations"]:
        connection = rel.get("connection")
        if connection is None:
            continue
        if connection["kind"] != "hosted_conduit" or rel["class"] != "IfcRelConnectsElements" or len(rel["related"]) != 1:
            raise ValueError("Invalid native conduit host relation")
        host,pipe = columns.get(rel["relating"]),pipes.get(rel["related"][0])
        rule = rules.get(connection["clearance_rule"])
        if host is None or pipe is None or rule is None or "hosted" not in rule or pipe["class"] != rule["a_class"] or pipe["system"] != rule["a_system"]:
            raise ValueError("Conduit host relation does not match native products and rule")
        key = (rule["id"],pipe["tag"],host["tag"])
        if key in hosts:
            raise ValueError("Duplicate conduit host relation")
        occupied = any(distance.convex_distance([a,b],core.part_vertices(part))["intersects"] for a,b in zip(pipe["points"],pipe["points"][1:]) for part in host["parts"])
        if not occupied:
            raise ValueError("Conduit host relation has no gross host occupancy")
        hosts[key] = rel
    return hosts


def clash_check(model: dict | None = None, include_passed: bool = False, *, progress=None, cancelled=None) -> list:
    """🚧 Compare actual occupied service and column solids under explicit clearance and hosting rules."""
    if cancelled is not None and cancelled():
        raise InterruptedError("Clash check cancelled")
    core,m = _context(model)
    distance = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.📏️distance.🐍️")
    rules = {rule["id"]:rule for rule in m["rules"]["clearance"]}
    if any(rule["b_class"] != "IfcColumn" or rule["a_class"] != "IfcPipeSegment" for rule in rules.values()):
        raise ValueError("Unsupported native clearance product classes")
    if any(rule["required"]["unit"] != "m" or rule.get("hosted",{}).get("required",{"unit":"m"})["unit"] != "m" for rule in rules.values()):
        raise ValueError("Clearance rules require metres")
    columns = {e["tag"]:core.column_geometry(m,e) for e in m["elements"] if e["class"] == "IfcColumn"}
    pipes = {e["tag"]:core.service_geometry(e["tag"],m) for e in m["elements"] if e["class"] == "IfcPipeSegment"}
    hosts = _hosting_relations(core,m,columns,pipes,rules,distance)
    column_bounds = {tag:distance.bounds([v for p in column["parts"] for v in core.part_vertices(p)]) for tag,column in columns.items()}
    components,pipe_components = {},{}
    out,completed = [],0
    total = sum(len(columns)*sum(p["class"] == r["a_class"] and p["system"] == r["a_system"] for p in pipes.values()) for r in rules.values())
    if progress is not None:
        progress({"phase":"pairs","completed":0,"total":total})
    for rule in rules.values():
        for pipe in (p for p in pipes.values() if p["class"] == rule["a_class"] and p["system"] == rule["a_system"]):
            if pipe["tag"] not in pipe_components:
                pipe_components[pipe["tag"]] = _occupied_components(core,pipe["parts"],distance)
            parts = pipe_components[pipe["tag"]]
            pipe_bounds = distance.bounds([v for p in parts for v in p["vertices"]])
            ordered = sorted(columns.values(),key=lambda c:distance.bounds_distance(pipe_bounds,column_bounds[c["tag"]]))
            nearest,violations = None,[]
            for column in ordered:
                if cancelled is not None and cancelled():
                    raise InterruptedError("Clash check cancelled")
                relation = hosts.get((rule["id"],pipe["tag"],column["tag"]))
                specification = rule["hosted"] if relation is not None else rule
                required = specification["required"]["value"]
                lower = distance.bounds_distance(pipe_bounds,column_bounds[column["tag"]])
                limit = max(required,nearest["distance"] if include_passed and nearest is not None else float("inf") if include_passed else required)
                if lower <= limit+EPS:
                    if column["tag"] not in components:
                        components[column["tag"]] = _occupied_components(core,column["parts"],distance)
                    result = distance.material_distance(parts,components[column["tag"]],cancelled=cancelled)
                    contact = result["intersects"] or result["distance"] <= EPS
                    violation = contact or result["distance"] < required-EPS
                    point = [(a+b)/2 for a,b in zip(result["a_point"],result["b_point"])]
                    row = {"a_tag":pipe["tag"],"a_guid":pipe["guid"],"b_tag":column["tag"],"b_guid":column["guid"],"kind":"clearance","rule":rule["id"],"name":specification["name"],
                           "required":required,"required_basis":specification["required"]["basis"],"distance":_round(result["distance"]),"intersects":contact,"deficit":_round(max(0.,required-result["distance"])),
                           "a_point":[_round(v) for v in result["a_point"]],"b_point":[_round(v) for v in result["b_point"]],"point":[_round(v,6) for v in point],"level":column["storey"],
                           "status":"violation" if violation else "ok","policy":"reserved_void" if relation is not None else "separation"}
                    if relation is not None:
                        row["host_relation"] = relation["tag"]
                    if nearest is None or row["distance"] < nearest["distance"]:
                        nearest = row
                    if violation:
                        violations.append(row)
                completed += 1
                if progress is not None:
                    progress({"phase":"pairs","completed":completed,"total":total,"a_tag":pipe["tag"],"b_tag":column["tag"]})
            out += violations
            if include_passed and nearest is not None and nearest not in violations:
                out.append(nearest)
    out.sort(key=lambda row:(row["status"] != "violation",row["distance"],row["a_tag"],row["b_tag"]))
    return [{"id":f"clash-{i:02d}",**row} for i,row in enumerate(out,1)]
