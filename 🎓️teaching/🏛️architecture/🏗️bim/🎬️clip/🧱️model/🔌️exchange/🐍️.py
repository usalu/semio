"""🔌 Exchange forms of the Villa Savoye teaching model: STEP excerpt, triangle mesh and IDS-style check (stdlib only)."""

from __future__ import annotations

import importlib.util
import math
import sys
from pathlib import Path

SCHEMA = "IFC4X3_ADD2"
STAMP = "2026-10-27T10:00:00"
FILENAME = "villa-savoye.ifc"
LANGUAGES = ("en", "de")
PROPERTIES = ("LoadBearing", "IsExternal")
SPEC_COLUMN = {"entity": "IfcColumn", "pset": "Pset_ColumnCommon", "property": "LoadBearing", "datatype": "boolean", "cardinality": "required"}
SPEC_WALL = {"entity": "IfcWall", "property": "IsExternal", "cardinality": "required"}
CARDINALITIES = {"exists": "required", "required": "required", "optional": "optional", "absent": "prohibited", "prohibited": "prohibited"}
DATATYPES = {"boolean": (bool,), "text": (str,), "number": (int, float)}
WORDS = {
    "required": ("requires", "verlangt", "required", "Pflicht"),
    "optional": ("allows", "erlaubt", "optional", "optional"),
    "prohibited": ("forbids", "verbietet", "prohibited", "verboten"),
}
DATATYPE_WORDS = {"boolean": ("boolean", "Boolean"), "text": ("text", "Text"), "number": ("number", "Zahl")}
_CORE = None


def _core():
    global _CORE
    if _CORE is None:
        path = next(p for p in sorted(Path(__file__).resolve().parent.parent.glob("*.py")) if "def load_model(" in p.read_text(encoding="utf-8"))
        for module in tuple(sys.modules.values()):
            file = getattr(module, "__dict__", {}).get("__file__")
            if file and "load_model" in module.__dict__ and Path(file).resolve() == path:
                _CORE = module
                break
        else:
            spec = importlib.util.spec_from_file_location("bim_model_core_exchange", path)
            _CORE = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(_CORE)
    return _CORE


def _real(value) -> str:
    text = f"{round(float(value), 9):.9f}".rstrip("0")
    return "0." if text in ("-0.", "0.") else text


def _escape(text: str) -> str:
    out, run = "", ""
    for char in text + "\0":
        if char != "\0" and ord(char) > 126:
            run += char
            continue
        if run:
            out += "\\X2\\" + run.encode("utf-16-be").hex().upper() + "\\X0\\"
            run = ""
        out += "" if char == "\0" else "''" if char == "'" else "\\\\" if char == "\\" else char
    return out


def _parts(item) -> list:
    if isinstance(item, str):
        return [(item, None)]
    if isinstance(item, tuple):
        return [item]
    return [part for sub in item for part in _parts(sub)]


def _join(items, separator=",") -> list:
    return [part for index, item in enumerate(items) for part in ([separator] if index else []) + [item]]


def _str(value: str, role=None) -> list:
    return ["'", (_escape(value), role), "'"]


def _enum(name: str, role=None) -> tuple:
    return (f".{name}.", role)


def _tuple(*items) -> list:
    return ["(", _join(items), ")"]


def _coords(values, role=None) -> tuple:
    return ("(" + ",".join(_real(v) for v in values) + ")", role)


class _Writer:
    def __init__(self, first_id: int, keys: list):
        self.first = first_id
        self.ids = {key: first_id + index for index, key in enumerate(keys)}
        self.lines, self.spans, self.entities = [], [], {}

    def ref(self, key: str, role="reference") -> tuple:
        return (f"#{self.ids[key]}", role)

    def raw(self, parts) -> None:
        text, line = "", len(self.lines)
        for chunk, role in _parts(parts):
            if role:
                self.spans.append({"line": line, "start": len(text), "end": len(text) + len(chunk), "role": role})
            text += chunk
        self.lines.append(text)

    def add(self, key: str, name: str, *attrs, role=None) -> None:
        if self.ids[key] != self.first + len(self.entities):
            raise AssertionError(f"Entity {key} is out of order")
        self.entities[key] = len(self.lines)
        self.raw([(f"#{self.ids[key]}", "number"), "=", (name, role), "(", _join(attrs), ");"])


def _column(obj: dict, geometry: dict) -> dict:
    p = next(x for x in geometry["pilotis"] if x["tag"] == obj["tag"])
    return {"point": [p["x"], p["y"], p["base"]], "refdir": None, "profile": {"radius": p["radius"]}, "depth": p["top"] - p["base"]}


def _slab(obj: dict, geometry: dict) -> dict:
    s = next(x for x in geometry["slabs"] if x["tag"] == obj["tag"])
    xs, ys = sorted({round(p[0], 9) for p in s["outline"]}), sorted({round(p[1], 9) for p in s["outline"]})
    if len(s["outline"]) != 4 or len(xs) != 2 or len(ys) != 2:
        raise ValueError(f"Only rectangular slab outlines have a STEP excerpt: {obj['tag']}")
    return {"point": [0.0, 0.0, s["top"] - s["thickness"]], "refdir": None, "depth": s["thickness"],
            "profile": {"x": xs[1] - xs[0], "y": ys[1] - ys[0], "centre": [(xs[0] + xs[1]) / 2, (ys[0] + ys[1]) / 2]}}


def _wall(obj: dict, geometry: dict) -> dict:
    w = next(x for x in geometry["walls"] if x["tag"] == obj["tag"])
    (x0, y0), (x1, y1), length = w["start"], w["end"], w["length"]
    return {"point": [x0, y0, w["base"]], "refdir": [(x1 - x0) / length, (y1 - y0) / length, 0.0], "depth": w["top"] - w["base"],
            "profile": {"x": length, "y": w["thickness"], "centre": [length / 2, 0.0]}}


BODIES = {"IfcColumn": _column, "IfcSlab": _slab, "IfcWall": _wall}


def _value(entry: dict) -> list:
    value = entry["value"]
    if isinstance(value, bool):
        return ["IFCBOOLEAN(", (".T." if value else ".F.", "value"), ")"]
    if isinstance(value, str):
        return ["IFCLABEL('", (_escape(value), "value"), "')"]
    return ["IFCREAL(", (_real(value), "value"), ")"]


def step_excerpt(key: str, model: dict | None = None, *, first_id: int = 101, language: str = "en", spatial: bool = True, properties: tuple = PROPERTIES) -> dict:
    """📄 Write a self-consistent ISO 10303-21 fragment (IFC4X3_ADD2) for one piloti, slab or wall of the model; a property set that belongs to the type hangs on its ``HasPropertySets``, the occurrence reaches it through ``IFCRELDEFINESBYTYPE``.

    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcColumn.htm
    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcGloballyUniqueId.htm
    """
    core = _core()
    m = core.load_model() if model is None else model
    obj = core.element(key, m)
    cls = obj["class"]
    if cls not in BODIES or language not in LANGUAGES or isinstance(first_id, bool) or not isinstance(first_id, int) or first_id < 1:
        raise ValueError(f"No STEP excerpt for {key} ({cls}, {language}, {first_id})")
    body = BODIES[cls](obj, core.building_geometry(m))
    profile, tag = body["profile"], obj["tag"]
    centre = "centre" in profile
    material = obj.get("material")
    layers = material.get("layers") if material else None
    psets = obj.get("psets", {})
    pset = f"Pset_{cls[3:]}Common" if f"Pset_{cls[3:]}Common" in psets else next(iter(psets), None)
    props = [name for name in properties if pset and name in psets[pset]]
    kind = core.element(obj["type"], m) if obj.get("type") else None
    storey = core.element(obj["storey"], m)
    type_level = bool(props and spatial and kind and pset in (kind.get("psets") or {}))
    keys = ["element", "placement", "axis", "point"] + (["refdir"] if body["refdir"] else []) + ["shape", "representation", "solid", "profile"]
    keys += (["profile_axis", "profile_point"] if centre else []) + ["direction", "context", "world_axis", "world_point"]
    if layers:
        keys += ["layerset"] + [f"layer{i}" for i in range(len(layers))] + [f"layer_material{i}" for i in range(len(layers))]
    elif material:
        keys += ["material"]
    keys += (["rel_material"] if material else []) + [f"prop_{name}" for name in props] + (["pset"] + ([] if type_level else ["rel_pset"]) if props else [])
    if spatial:
        keys += (["type", "rel_type"] if kind else []) + ["storey", "rel_storey"]
    w = _Writer(first_id, keys)

    def guid(suffix, owner=None):
        return _str(core.global_id(f"{owner or tag}.{suffix}"))

    w.lines += ["ISO-10303-21;", "HEADER;", "FILE_DESCRIPTION(('ViewDefinition [DesignTransferView]'),'2;1');",
                f"FILE_NAME('{FILENAME}','{STAMP}',('Teaching'),('semio'),'','','');", f"FILE_SCHEMA(('{SCHEMA}'));", "ENDSEC;", "DATA;"]
    w.add("element", cls.upper(), _str(obj["guid"], "guid"), "$", _str(obj["name"][language], "name"), "$", "$", w.ref("placement", "placement"), w.ref("shape", "representation"), _str(tag, "tag"), _enum(obj["predefined_type"], "predefined"), role="class")
    w.add("placement", "IFCLOCALPLACEMENT", "$", w.ref("axis"))
    w.add("axis", "IFCAXIS2PLACEMENT3D", w.ref("point"), "$", w.ref("refdir") if body["refdir"] else "$")
    w.add("point", "IFCCARTESIANPOINT", _coords(body["point"], "placement"))
    if body["refdir"]:
        w.add("refdir", "IFCDIRECTION", _coords(body["refdir"]))
    w.add("shape", "IFCPRODUCTDEFINITIONSHAPE", "$", "$", _tuple(w.ref("representation")))
    w.add("representation", "IFCSHAPEREPRESENTATION", w.ref("context"), _str("Body", "representation"), _str("SweptSolid", "representation"), _tuple(w.ref("solid")))
    w.add("solid", "IFCEXTRUDEDAREASOLID", w.ref("profile"), w.ref("world_axis"), w.ref("direction"), (_real(body["depth"]), "depth"))
    if centre:
        w.add("profile", "IFCRECTANGLEPROFILEDEF", _enum("AREA"), "$", w.ref("profile_axis"), (_real(profile["x"]), "profile"), (_real(profile["y"]), "profile"))
        w.add("profile_axis", "IFCAXIS2PLACEMENT2D", w.ref("profile_point"), "$")
        w.add("profile_point", "IFCCARTESIANPOINT", _coords(profile["centre"]))
    else:
        w.add("profile", "IFCCIRCLEPROFILEDEF", _enum("AREA"), "$", "$", (_real(profile["radius"]), "profile"))
    w.add("direction", "IFCDIRECTION", _coords([0.0, 0.0, 1.0]))
    w.add("context", "IFCGEOMETRICREPRESENTATIONCONTEXT", "$", _str("Model"), "3", "1.E-05", w.ref("world_axis"), "$")
    w.add("world_axis", "IFCAXIS2PLACEMENT3D", w.ref("world_point"), "$", "$")
    w.add("world_point", "IFCCARTESIANPOINT", _coords([0.0, 0.0, 0.0]))
    if layers:
        w.add("layerset", "IFCMATERIALLAYERSET", _tuple(*[w.ref(f"layer{i}") for i in range(len(layers))]), _str(material["name"][language], "material"), "$")
        for i, layer in enumerate(layers):
            w.add(f"layer{i}", "IFCMATERIALLAYER", w.ref(f"layer_material{i}"), _real(layer["thickness"]["value"]), "$", "$", "$", "$", "$")
        for i, layer in enumerate(layers):
            w.add(f"layer_material{i}", "IFCMATERIAL", _str(core.element(layer["material"], m)["name"][language], "material"), "$", "$")
    elif material:
        w.add("material", "IFCMATERIAL", _str(material["name"][language], "material"), "$", "$")
    if material:
        w.add("rel_material", "IFCRELASSOCIATESMATERIAL", guid("material"), "$", "$", "$", _tuple(w.ref("element")), w.ref("layerset" if layers else "material"))
    for name in props:
        w.add(f"prop_{name}", "IFCPROPERTYSINGLEVALUE", _str(name, "property"), "$", _value(psets[pset][name]), "$")
    if props:
        w.add("pset", "IFCPROPERTYSET", guid(pset, kind["tag"] if type_level else None), "$", _str(pset, "pset"), "$", _tuple(*[w.ref(f"prop_{name}") for name in props]))
        if not type_level:
            w.add("rel_pset", "IFCRELDEFINESBYPROPERTIES", guid(f"{pset}.rel"), "$", "$", "$", _tuple(w.ref("element")), w.ref("pset"))
    if spatial:
        if kind:
            w.add("type", kind["class"].upper(), _str(kind["guid"]), "$", _str(kind["name"][language]), "$", "$", _tuple(w.ref("pset")) if type_level else "$", "$", "$", "$", _enum(kind["predefined_type"]))
            w.add("rel_type", "IFCRELDEFINESBYTYPE", guid("type"), "$", "$", "$", _tuple(w.ref("element")), w.ref("type"))
        w.add("storey", "IFCBUILDINGSTOREY", _str(storey["guid"]), "$", _str(storey["name"][language]), "$", "$", "$", "$", "$", _enum("ELEMENT"), _real(storey["elevation"]["value"]))
        w.add("rel_storey", "IFCRELCONTAINEDINSPATIALSTRUCTURE", guid("storey"), "$", "$", "$", _tuple(w.ref("element")), w.ref("storey"))
    w.lines += ["ENDSEC;", "END-ISO-10303-21;"]
    return {"key": tag, "guid": obj["guid"], "class": cls, "schema": SCHEMA, "language": language, "first_id": first_id, "lines": w.lines, "spans": w.spans,
            "ids": {name: f"#{number}" for name, number in w.ids.items()}, "entities": w.entities}


def mesh(key: str, segments: int = 16, model: dict | None = None) -> dict:
    """🔺 Tessellate a piloti into a closed triangle mesh (cylinder, fan-triangulated caps) and list what the mesh keeps and loses.

    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcTriangulatedFaceSet.htm
    """
    core = _core()
    m = core.load_model() if model is None else model
    obj = core.element(key, m)
    if obj["class"] != "IfcColumn":
        raise ValueError(f"No mesh export for {obj['class']}")
    if isinstance(segments, bool) or not isinstance(segments, int) or segments < 3:
        raise ValueError("A mesh needs at least 3 segments")
    p = next(x for x in core.building_geometry(m)["pilotis"] if x["tag"] == obj["tag"])
    n, r, x, y = segments, p["radius"], p["x"], p["y"]
    angles = [2 * math.pi * i / n for i in range(n)]
    ring = [(round(x + r * math.cos(a), 9), round(y + r * math.sin(a), 9)) for a in angles]
    vertices = [[px, py, p["base"]] for px, py in ring] + [[px, py, p["top"]] for px, py in ring] + [[x, y, p["base"]], [x, y, p["top"]]]
    sides = [face for i in range(n) for j in [(i + 1) % n] for face in ([i, j, n + j], [i, n + j, n + i])]
    top = [[2 * n + 1, n + i, n + (i + 1) % n] for i in range(n)]
    bottom = [[2 * n, (i + 1) % n, i] for i in range(n)]
    faces = sides + top + bottom
    psets = obj.get("psets", {})
    pset = next((name for name, props in psets.items() if "LoadBearing" in props), None)
    kind = core.element(obj["type"], m) if obj.get("type") else None
    material = obj.get("material")
    kept = [{"id": "vertices", "en": "Vertices", "de": "Punkte"}, {"id": "triangles", "en": "Triangles", "de": "Dreiecke"}]
    lost = [{"id": "class", "en": f"Class {obj['class']}", "de": f"Klasse {obj['class']}"}, {"id": "guid", "en": "GlobalId", "de": "GlobalId"}]
    lost += [{"id": "property", "en": f"{pset}.LoadBearing", "de": f"{pset}.LoadBearing"}] if pset else []
    lost += [{"id": "material", "en": f"Material {material['name']['en']}", "de": f"Material {material['name']['de']}"}] if material else []
    lost += [{"id": "type", "en": f"Type {kind['name']['en']}", "de": f"Typ {kind['name']['de']}"}] if kind else []
    return {"tag": obj["tag"], "guid": obj["guid"], "segments": n, "radius": r, "height": p["top"] - p["base"], "centre": [x, y],
            "vertices": vertices, "faces": faces, "vertex_count": len(vertices), "triangle_count": len(faces), "kept": kept, "lost": lost,
            "layout": {"bottom": list(range(n)), "top": list(range(n, 2 * n)), "bottom_centre": 2 * n, "top_centre": 2 * n + 1,
                       "side_faces": [0, 2 * n], "top_faces": [2 * n, 3 * n], "bottom_faces": [3 * n, 4 * n]}}


def _normalise(spec) -> dict:
    if isinstance(spec, str):
        words = spec.split()
        if len(words) != 3 or words[2] not in CARDINALITIES:
            raise ValueError("A spec string reads '<Entity> [<Pset>.]<Property> exists|optional|prohibited'")
        pset, _, prop = words[1].rpartition(".")
        spec = {"entity": words[0], "property": prop, "cardinality": CARDINALITIES[words[2]], **({"pset": pset} if pset else {})}
    if not isinstance(spec, dict) or not {"entity", "property"} <= set(spec) or set(spec) - {"id", "name", "entity", "predefined", "pset", "property", "datatype", "value", "cardinality"}:
        raise ValueError("A spec needs entity and property")
    out = {"cardinality": "required", **spec}
    if out["cardinality"] not in WORDS or out.get("datatype") not in (None, *DATATYPES):
        raise ValueError("Unknown cardinality or datatype")
    return out


def _texts(spec: dict) -> tuple:
    verb_en, verb_de, card_en, card_de = WORDS[spec["cardinality"]]
    target = (spec["pset"] + "." if spec.get("pset") else "") + spec["property"]
    entity = spec["entity"] + (f" {spec['predefined']}" if spec.get("predefined") else "")
    kind_en, kind_de = DATATYPE_WORDS[spec["datatype"]] if spec.get("datatype") else ("", "")
    value_en, value_de = (f"value = {spec['value']}", f"Wert = {spec['value']}") if "value" in spec else ("any value, not forced", "beliebiger Wert, nicht erzwungen")
    rule_en = ", ".join(x for x in (card_en, kind_en, value_en) if x)
    rule_de = ", ".join(x for x in (card_de, kind_de, value_de) if x)
    applicability = {"entity": spec["entity"], "predefined": spec.get("predefined"), "en": f"applies to: every {entity}", "de": f"gilt für: jede {entity}"}
    requirement = {"property": target, "pset": spec.get("pset"), "cardinality": spec["cardinality"], "datatype": spec.get("datatype"),
                   "en": f"{verb_en}: {target}", "de": f"{verb_de}: {target}"}
    return applicability, requirement, {"id": spec["cardinality"], "en": rule_en, "de": rule_de}


def _judge(spec: dict, obj: dict) -> tuple:
    psets = obj.get("psets", {})
    entry = next((props[spec["property"]] for name, props in psets.items() if spec.get("pset") in (None, name) and spec["property"] in props), None)
    value = None if entry is None else entry["value"]
    typed = spec.get("datatype") is None or (value is not None and isinstance(value, DATATYPES[spec["datatype"]]) and not (spec["datatype"] == "number" and isinstance(value, bool)))
    valued = "value" not in spec or value == spec["value"]
    ok = {"required": entry is not None and typed and valued, "optional": entry is None or (typed and valued), "prohibited": entry is None}[spec["cardinality"]]
    return ok, value


def ids_check(model: dict | None = None, spec=None, *, source: str = "semantic") -> dict:
    """✅ Evaluate a minimal IDS-style requirement (applicability and one property facet) on the model or on its mesh export.

    https://www.buildingsmart.org/standards/bsi-standards/information-delivery-specifications-ids/
    """
    core = _core()
    m = core.load_model() if model is None else model
    if source not in ("semantic", "mesh"):
        raise ValueError("Unknown source")
    if spec is None:
        rules = m.get("rules", {}).get("ids", [])
        spec = {"datatype": "boolean", **rules[0]} if rules else SPEC_COLUMN
    spec = _normalise(spec)
    applicability, requirement, cardinality = _texts(spec)
    expected = [e["tag"] for e in m["elements"] if e["class"] == spec["entity"] and spec.get("predefined") in (None, e["predefined_type"])]
    applicable = expected if source == "semantic" else []
    items = []
    for tag in applicable:
        ok, value = _judge(spec, core.element(tag, m))
        items.append({"tag": tag, "ok": ok, "value": value})
    passed = [item["tag"] for item in items if item["ok"]]
    return {"spec": spec, "source": source, "applicability": applicability, "requirement": requirement, "cardinality": cardinality,
            "expected": len(expected), "found": len(applicable), "checked": len(items), "passed": len(passed), "failed": [item["tag"] for item in items if not item["ok"]],
            "applicable": applicable, "passed_tags": passed, "items": items}
