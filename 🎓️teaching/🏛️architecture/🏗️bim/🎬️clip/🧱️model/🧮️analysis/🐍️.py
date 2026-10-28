"""🔬 Analysis derivations of the Villa Savoye teaching model: structural analysis translation, tributary-area estimate, clearance check."""

from __future__ import annotations

import importlib
import math

EPS = 1e-9
LEVELS = {"first": "FLOOR", "roof": "ROOF"}
LINK_KINDS = ("slab-load", "roof-load", "piloti-section")
LOAD_NAMES = {"self_weight": ("Self-weight", "Eigengewicht"), "finishes": ("Finishes", "Ausbau"), "imposed": ("Imposed load", "Nutzlast"), "total": ("Total", "Summe")}
ZONE_NAMES = {"slab": {"en": "First-floor slab", "de": "Geschossdecke"}, "planted": {"en": "Planted roof", "de": "Bepflanzte Dachfläche"}}
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
    signed = sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(polygon, polygon[1:] + polygon[:1]))
    return list(polygon) if signed >= 0 else list(reversed(polygon))


def _side(p, a, b) -> float:
    return (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])


def _half(polygon: list, a, b, keep_left: bool = True) -> list:
    sign = 1 if keep_left else -1
    out = []
    for p, q in zip(polygon, polygon[1:] + polygon[:1]):
        sp, sq = sign * _side(p, a, b), sign * _side(q, a, b)
        if sp >= 0:
            out.append(p)
        if (sp > 0 > sq) or (sp < 0 < sq):
            t = sp / (sp - sq)
            out.append([p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1])])
    return [[_round(x), _round(y)] for x, y in out]


def _convex(polygon: list) -> bool:
    signs = {_side(c, a, b) > EPS for a, b, c in zip(polygon, polygon[1:] + polygon[:1], polygon[2:] + polygon[:2]) if abs(_side(c, a, b)) > EPS}
    return len(signs) <= 1


def _clip(subject: list, clip: list) -> list:
    clip = _ccw(clip)
    if not _convex(clip):
        raise ValueError("Tributary cells are clipped against convex slab outlines only")
    for a, b in zip(clip, clip[1:] + clip[:1]):
        if not subject:
            break
        subject = _half(subject, a, b)
    return subject


def _subtract(piece: list, void: list) -> list:
    void = _ccw(void)
    if not _convex(void):
        raise ValueError("Tributary cells are cut by convex voids only")
    out, rest = [], piece
    for a, b in zip(void, void[1:] + void[:1]):
        out.append(_half(rest, a, b, False))
        rest = _half(rest, a, b)
    return [p for p in out if len(p) >= 3 and _area(p) > EPS]


def _on(q, a, b) -> bool:
    cross = abs(_side(q, a, b))
    along = (q[0] - a[0]) * (b[0] - a[0]) + (q[1] - a[1]) * (b[1] - a[1])
    return cross < EPS and EPS < along < math.dist(a, b) ** 2 - EPS


def _signed(polygon: list) -> float:
    return sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(polygon, polygon[1:] + polygon[:1])) / 2


def _loops(pieces: list) -> list:
    points = {tuple(p) for piece in pieces for p in piece}
    edges = {}
    for piece in pieces:
        for a, b in zip(piece, piece[1:] + piece[:1]):
            a, b = tuple(a), tuple(b)
            chain = [a] + sorted((q for q in points if _on(q, a, b)), key=lambda q: math.dist(a, q)) + [b]
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
        loops.append([p for i, p in enumerate(loop) if abs(_side(p, loop[i - 1], loop[(i + 1) % len(loop)])) > EPS])
    return sorted(loops, key=lambda loop: -abs(_signed(loop)))


def _outline(pieces: list) -> list:
    return next((loop for loop in _loops(pieces) if _signed(loop) > 0), [])


def _slab(geometry: dict, level: str) -> dict:
    if level not in LEVELS:
        raise ValueError(f"Unknown level: {level}")
    return next(s for s in geometry["slabs"] if s["kind"] == LEVELS[level])


def _entry(value: float, basis: str, en: str, de: str, **extra) -> dict:
    return {"value": _round(value), "unit": "kN/m²", "basis": basis, "en": en, "de": de, **extra}


def _breakdown(core, m: dict, geometry: dict, level: str, zone: str | None = None) -> dict:
    slab = _slab(geometry, level)
    if level == "first":
        density = core.element(slab["tag"], m)["material"]["density"]
        parts = {"self_weight": _entry(density["value"] * slab["thickness"], density["basis"], *LOAD_NAMES["self_weight"], density=density["value"], thickness=slab["thickness"]),
                 "finishes": _entry(m["loads"]["finishes"]["first"]["value"], m["loads"]["finishes"]["first"]["basis"], *LOAD_NAMES["finishes"]),
                 "imposed": _entry(m["loads"]["imposed"]["first"]["value"], m["loads"]["imposed"]["first"]["basis"], *LOAD_NAMES["imposed"])}
    else:
        parts = {l["id"]: _entry(l["density"] * l["thickness"], "assumed", l["name"]["en"], l["name"]["de"], density=l["density"], thickness=l["thickness"])
                 for l in slab["layers"] if not l["zones"] or zone in l["zones"]}
        imposed = m["loads"]["imposed"].get(zone) or m["loads"]["imposed"][f"roof_{zone}"]
        parts["imposed"] = _entry(imposed["value"], imposed["basis"], *LOAD_NAMES["imposed"])
    parts["total"] = _entry(sum(p["value"] for p in parts.values()), "assumed", *LOAD_NAMES["total"])
    return parts


def _zones(core, m: dict, geometry: dict, level: str) -> list:
    slab = _slab(geometry, level)
    if level == "first":
        return [{"id": "slab", "name": ZONE_NAMES["slab"], "polygon": None, "breakdown": _breakdown(core, m, geometry, level)}]
    explicit = slab.get("zones", {})
    ids = list(dict.fromkeys(z for l in slab["layers"] for z in l["zones"]))
    rest = [z for z in ids if z not in explicit]
    if len(rest) != 1 or not set(explicit) <= set(ids):
        raise ValueError("The roof build-up must name its explicit zones and exactly one remaining zone")
    spaces = core.element(slab["tag"], m)["geometry"]["zones"]
    return [{"id": z, "name": core.element(spaces[z], m)["name"] if z in explicit else ZONE_NAMES[z], "polygon": explicit.get(z), "breakdown": _breakdown(core, m, geometry, level, z)} for z in ids]


def _names(zones: list, key: str) -> tuple:
    entry = next(z["breakdown"][key] for z in zones if key in z["breakdown"])
    return entry["en"], entry["de"]


def _split(pieces: list, zones: list) -> dict:
    out, rest = {}, pieces
    for zone in (z for z in zones if z["polygon"]):
        out[zone["id"]] = [c for p in pieces for c in [_clip(p, zone["polygon"])] if len(c) >= 3 and _area(c) > EPS]
        rest = [part for p in rest for part in _subtract(p, zone["polygon"])]
    out[next(z["id"] for z in zones if not z["polygon"])] = rest
    return out


def _labels(lines: list, grid_lines: list, names: list) -> list:
    return [names[grid_lines.index(v)] if v in grid_lines else ("0" if v < grid_lines[0] else str(len(grid_lines) + 1)) for v in lines]


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
    for pil in geometry["pilotis"]:
        i, j = xs.index(pil["x"]), ys.index(pil["y"])
        cell = [[x_cuts[i], y_cuts[j]], [x_cuts[i + 1], y_cuts[j]], [x_cuts[i + 1], y_cuts[j + 1]], [x_cuts[i], y_cuts[j + 1]]]
        pieces = [_clip(cell, outline)]
        for void in slab["voids"]:
            pieces = [part for piece in pieces for part in _subtract(piece, void)]
        pieces = [p for p in pieces if len(p) >= 3 and _area(p) > EPS]
        area = _round(sum(_area(p) for p in pieces))
        split = _split(pieces, zones)
        shares = {z["id"]: {"area": _round(sum(_area(p) for p in split[z["id"]])), "pieces": split[z["id"]]} for z in zones}
        for z in zones:
            shares[z["id"]].update(share=shares[z["id"]]["area"] / area if area else 0.0, reaction=_round(z["breakdown"]["total"]["value"] * shares[z["id"]]["area"]))
        edges = (j in (0, len(ys) - 1)) + (i in (0, len(xs) - 1))
        out.append({"tag": pil["tag"], "guid": pil["guid"], "row": grid["labels_y"][j], "column": grid["labels_x"][i], "x": pil["x"], "y": pil["y"],
                    "kind": ("interior", "edge", "corner")[edges], "area": area, "reaction": _round(sum(v["reaction"] for v in shares.values())), "share": area / slab_area,
                    "zones": shares, "cell": _outline(pieces), "pieces": pieces})
    total_area = _round(sum(p["area"] for p in out))
    total = _round(sum(p["reaction"] for p in out))
    listed = []
    for z in zones:
        pieces = [piece for p in out for piece in p["zones"][z["id"]]["pieces"]]
        loops = _loops(pieces)
        area = _round(sum(p["zones"][z["id"]]["area"] for p in out))
        listed.append({"id": z["id"], "name": z["name"], "area": area, "area_load": z["breakdown"]["total"]["value"], "load_breakdown": z["breakdown"], "share": area / slab_area,
                       "reaction": _round(sum(p["zones"][z["id"]]["reaction"] for p in out)), "outline": next((l for l in loops if _signed(l) > 0), []),
                       "outlines": [l for l in loops if _signed(l) > 0], "holes": [l for l in loops if _signed(l) < 0], "pieces": pieces})
    names = list(dict.fromkeys(k for z in zones for k in z["breakdown"] if k != "total"))
    mean = {k: _entry(sum(z["load_breakdown"][k]["value"] * z["area"] for z in listed if k in z["load_breakdown"]) / slab_area, "assumed", *_names(zones, k)) for k in names}
    mean["total"] = _entry(sum(v["value"] for v in mean.values()), "assumed", *LOAD_NAMES["total"])
    return {"level": level, "method": METHOD["en"], "method_de": METHOD["de"], "slab_tag": slab["tag"], "slab_guid": slab["guid"], "area_load": mean["total"]["value"], "load_breakdown": mean,
            "slab_area": slab_area, "total_area": total_area, "total_reaction": total, "balanced": math.isclose(total_area, slab_area, rel_tol=1e-9), "zones": listed,
            "cantilever": {"west": _round(xs[0] - x_min), "east": _round(x_max - xs[-1]), "south": _round(ys[0] - y_min), "north": _round(y_max - ys[-1])},
            "cuts": {"x": x_cuts, "y": y_cuts}, "pilotis": out}


def link_values(model: dict, kind: str) -> dict:
    """🧷 Give the plain numbers a stored analysis link relies on, so a later model state can be compared with them."""
    core, m = _context(model)
    if kind == "slab-load":
        first = tributary(m, "first")
        return {"thickness": _slab(core.building_geometry(m), "first")["thickness"], "area_load": first["area_load"], "total_reaction": first["total_reaction"]}
    if kind == "roof-load":
        roof = tributary(m, "roof")
        return {"area_load": roof["area_load"], **{f"{z['id']}_area_load": z["area_load"] for z in roof["zones"]}, "total_reaction": roof["total_reaction"],
                **{f"reaction_{p['tag']}": p["reaction"] for p in roof["pilotis"]}}
    if kind == "piloti-section":
        section = core.quantities("TY-PILOTI", m)
        return {key: section[key] for key in ("diameter", "height", "cross_section_area")}
    raise ValueError(f"Unknown link kind: {kind}")


def _assumptions(m: dict, loads: dict, counts: dict, slab: dict) -> list:
    given = {a["id"]: {**a, "basis": a.get("basis", "assumed")} for a in m["translation"]["assumptions"]}
    for key, en, de in EXTRA_ASSUMPTIONS:
        given.setdefault(key, {"id": key, "en": en, "de": de, "basis": "assumed"})
    density = loads["self_weight"]["density"]

    def text(language):
        values = [_fmt(density, language, 1), _fmt(slab["thickness"], language), _fmt(loads["self_weight"]["value"], language), _fmt(loads["finishes"]["value"], language), _fmt(loads["imposed"]["value"], language)]
        return ("Loads on the first-floor slab: self-weight {2} kN/m² ({0} kN/m³ × {1} m), finishes {3} kN/m², imposed {4} kN/m²; all assumed." if language == "en"
                else "Lasten auf der Geschossdecke: Eigengewicht {2} kN/m² ({0} kN/m³ × {1} m), Ausbau {3} kN/m², Nutzlast {4} kN/m²; alle angenommen.").format(*values)

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
    """🏗️ Translate the structure of the teaching model into a structural analysis model with explicit assumptions and losses.

    See https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralCurveMember.htm and
    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralPointConnection.htm for the target classes.
    """
    core, m = _context(model)
    geometry = core.building_geometry(m)
    spec = m["translation"]
    slab = _slab(geometry, "first")
    grid = geometry["grid"]
    xs = list(spec["mesh"]["x_lines"])
    ys = list(grid["y"] if spec["mesh"]["y_lines"] == "grid" else spec["mesh"]["y_lines"])
    outline = slab["outline"]
    if not (math.isclose(xs[0], min(p[0] for p in outline)) and math.isclose(xs[-1], max(p[0] for p in outline))):
        raise ValueError("The analysis mesh must span the slab outline")
    z = _round(slab["top"] - slab["thickness"] / 2)
    columns, rows = _labels(xs, grid["x"], grid["labels_x"]), list(grid["labels_y"])
    nodes = [{"id": f"N-{rows[j]}{columns[i]}", "role": "slab", "position": [xs[i], ys[j], z]} for j in range(len(ys)) for i in range(len(xs))]
    shells = []
    for j in range(len(ys) - 1):
        for i in range(len(xs) - 1):
            corners = [(i, j), (i + 1, j), (i + 1, j + 1), (i, j + 1)]
            shells.append({"id": f"SH-{rows[j]}{columns[i]}", "nodes": [f"N-{rows[b]}{columns[a]}" for a, b in corners], "thickness": slab["thickness"], "source_guid": slab["guid"],
                           "source_tag": slab["tag"], "area": _round((xs[i + 1] - xs[i]) * (ys[j + 1] - ys[j])), "bounds": [xs[i], ys[j], xs[i + 1], ys[j + 1]]})
    members, supports = [], []
    for pil in geometry["pilotis"]:
        row, column = rows[ys.index(pil["y"])], columns[xs.index(pil["x"])]
        foot = {"id": f"F-{pil['tag']}", "role": "base", "position": [pil["x"], pil["y"], pil["base"]]}
        nodes.append(foot)
        members.append({"id": f"M-{pil['tag']}", "kind": "curve", "ifc_class": "IfcStructuralCurveMember", "source_guid": pil["guid"], "source_tag": pil["tag"],
                        "nodes": [foot["id"], f"N-{row}{column}"], "length": _round(z - pil["base"]), "physical_height": _round(pil["top"] - pil["base"]), "diameter": _round(2 * pil["radius"])})
        supports.append({"node": foot["id"], "position": foot["position"], "ifc_class": "IfcStructuralPointConnection", "condition": spec["boundary"]["class"], "fixity": spec["boundary"]["id"],
                         "basis": spec["boundary"]["basis"], "label": {"en": spec["boundary"]["en"], "de": spec["boundary"]["de"]}, "source_tag": pil["tag"], "source_guid": pil["guid"]})
    members += [{"id": f"M-{s['id']}", "kind": "surface", "ifc_class": "IfcStructuralSurfaceMember", "source_guid": slab["guid"], "source_tag": slab["tag"], "nodes": s["nodes"],
                 "thickness": s["thickness"], "shell": s["id"]} for s in shells]
    first = _breakdown(core, m, geometry, "first")
    loads = {**first, "area_load": first["total"]["value"]}
    roof = tributary(m, "roof")
    point_loads = [{"node": next(n["id"] for n in nodes if n["role"] == "slab" and n["position"][:2] == [p["x"], p["y"]]), "value": p["reaction"], "unit": "kN", "basis": "assumed",
                    "level": "roof", "source_tag": p["tag"]} for p in roof["pilotis"] if p["reaction"] > 0]
    slab_nodes, base_nodes = len(nodes) - len(supports), len(supports)
    counts = {"curve_members": len(supports), "surface_members": len(shells), "members": len(members), "shells": len(shells), "slab_nodes": slab_nodes, "base_nodes": base_nodes, "nodes": len(nodes),
              "supports": len(supports), "point_loads": len(point_loads)}
    quantities = core.quantities(slab["tag"], m)
    losses, loss_groups = _losses(m)
    mapping = [{**rule, "count": counts[{"curve": "curve_members", "surface": "surface_members", "support": "supports"}[rule["kind"]]]} for rule in spec["mapping"]]
    structure_revision = core.partial_model("structure", m)["revision"]
    return {"source_schema": spec["source_schema"], "target_schema": spec["target_schema"], "target_domain": spec["target_domain"], "model_class": "IfcStructuralAnalysisModel",
            "source_revision": structure_revision, "mid_surface": {"z": z, "level": "first", "elevation": slab["top"], "thickness": slab["thickness"]},
            "mesh": {"x_lines": xs, "y_lines": ys, "columns": columns, "rows": rows}, "slab": {"tag": slab["tag"], "guid": slab["guid"], "gross_area": quantities["gross_area"],
            "void_area": quantities["void_area"], "net_area": quantities["area"], "thickness": slab["thickness"]},
            "members": members, "nodes": nodes, "shells": shells, "supports": supports, "loads": loads, "point_loads": point_loads, "mapping": mapping,
            "assumptions": _assumptions(m, loads, counts, slab), "losses": losses, "loss_groups": loss_groups, "counts": counts,
            "used": {kind: link_values(m, kind) for kind in LINK_KINDS}}


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


def clash_check(model: dict | None = None, include_passed: bool = False) -> list:
    """🚧 Check authored clearance rules between services and structure and report the pairs that come closer than required.

    See https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcPipeSegment.htm for the checked service class.
    """
    core, m = _context(model)
    geometry = core.building_geometry(m)
    levels = sorted(geometry["storeys"].items(), key=lambda item: item[1])
    out = []
    for rule in m["rules"]["clearance"]:
        if rule["b_class"] != "IfcColumn":
            raise ValueError(f"Unsupported clearance partner: {rule['b_class']}")
        required = rule["required"]["value"]
        for pipe in (s for s in geometry["services"] if s["class"] == rule["a_class"] and s["system"] == rule["a_system"]):
            found = []
            for pil in geometry["pilotis"]:
                axis = ([pil["x"], pil["y"], pil["base"]], [pil["x"], pil["y"], pil["top"]])
                gaps = [_closest(a, b, *axis) for a, b in zip(pipe["points"], pipe["points"][1:])]
                on_pipe, on_axis = min(gaps, key=lambda g: math.dist(*g))
                centre = math.dist(on_pipe, on_axis)
                distance = _round(centre - pipe["diameter"] / 2 - pil["radius"])
                reach = pipe["diameter"] / 2 + distance / 2
                point = on_pipe if centre < EPS else [a + (b - a) / centre * reach for a, b in zip(on_pipe, on_axis)]
                found.append({"a_tag": pipe["tag"], "a_guid": pipe["guid"], "b_tag": pil["tag"], "b_guid": pil["guid"], "kind": "clearance", "rule": rule["id"], "name": rule["name"],
                              "required": required, "required_basis": rule["required"]["basis"], "distance": distance, "clearance": distance, "deficit": _round(max(0.0, required - distance)),
                              "point": [_round(c, 6) for c in point], "level": [name for name, elevation in levels if elevation <= point[2] + EPS][-1],
                              "status": "violation" if distance < required else "ok"})
            nearest = min(found, key=lambda f: f["distance"])
            out += [f for f in found if f["status"] == "violation" or (include_passed and f is nearest)]
    out.sort(key=lambda f: (f["status"] != "violation", f["distance"]))
    return [{"id": f"clash-{i:02d}", **f} for i, f in enumerate(out, 1)]
