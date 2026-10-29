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
    return [("''", role)] if value == "" else ["'", (_escape(value), role), "'"]


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


def _native_profile_kind(part: dict) -> str:
    """🕳️ Select a precise native hollow circle or a polygonal profile with explicit inner loops."""
    profile = part["profile"]
    voids = profile.get("voids",())
    if voids:
        if profile["shape"]=="circle" and len(voids)==1 and voids[0]["offset"]==[0,0]:
            return "circle_hollow"
        return "profile_void"
    return profile["shape"]


def _column(obj: dict, geometry: dict) -> dict:
    p = next(x for x in geometry["pilotis"] if x["tag"] == obj["tag"])
    return {"point": [0., 0., 0.], "refdir": None, "parts": p["parts"], "profile": {}, "depth": p["top"] - p["base"]}


def _slab(obj: dict, geometry: dict) -> dict:
    slab = next(x for x in geometry["slabs"] if x["tag"] == obj["tag"])
    parts = [{**part, "profile": {"shape": "polygon"}} for part in slab["parts"]]
    return {"point": [0., 0., 0.], "refdir": None, "depth": slab["thickness"], "profile": {}, "parts": parts}

def _wall(obj: dict, geometry: dict) -> dict:
    w = next(x for x in geometry["walls"]+geometry.get("screens",[]) if x["tag"] == obj["tag"])
    parts = [{**p,"profile":{"shape":"polygon"}} for p in w["parts"]]
    return {"point":[0.,0.,0.],"refdir":None,"depth":w["top"]-w["base"],"profile":{},"parts":parts}



FILLING_CLASSES = ("IfcWindow","IfcDoor","IfcCurtainWall")


def _filling(obj: dict, geometry: dict) -> dict:
    """🪟 Place shared filling prisms on their aperture axis, preserving occupied geometry."""
    filling = geometry["curtain_wall"] if obj["class"]=="IfcCurtainWall" else next(f for f in geometry["fillings"] if f["tag"]==obj["tag"])
    point,refdir = [0.,0.,0.],None
    if "axis" in filling:
        a,b = filling["axis"]
        length = math.dist(a,b)
        dx,dy = (b[0]-a[0])/length,(b[1]-a[1])/length
        point,refdir = [*a,min(p["z0"] for p in filling["parts"])],[dx,dy,0.]
    else:
        dx,dy = 1.,0.
    parts = []
    for part in filling["parts"]:
        ring = [[(p[0]-point[0])*dx+(p[1]-point[1])*dy,-(p[0]-point[0])*dy+(p[1]-point[1])*dx] for p in part["outline"]]
        parts.append({**part,"outline":ring,"z0":part["z0"]-point[2],"z1":part["z1"]-point[2],"profile":{"shape":"polygon"}})
    return {"point":point,"refdir":refdir,"parts":parts,"profile":{},"depth":max(p["z1"] for p in parts),
            **{k:filling[k] for k in ("rough_width","rough_height") if k in filling}}


def _constituents(parts: list, cls: str) -> list:
    """🧵 Group material solids into uniquely addressable IFC component aspects."""
    groups = {}
    for i,part in enumerate(parts):
        category = {"glass":"Glazing","wing_insulation":"Insulation","wing_skin":"Lining"}.get(part["role"],"Lining" if part["role"]=="frame" and cls!="IfcCurtainWall" else "Framing") if cls in FILLING_CLASSES else part["role"]
        groups.setdefault((category,part["material"]),[]).append(i)
    counts = {category:sum(c==category for c,_ in groups) for category,_ in groups}
    return [{"name":category if counts[category]==1 else category+":"+material,"category":category,"material":material,"parts":indices}
            for (category,material),indices in groups.items()]


def _type_layout(kind: dict) -> list:
    """🚪 Preserve known native layout semantics while leaving unknown handedness undefined."""
    if kind["class"]=="IfcWindowType":
        return [_enum("NOTDEFINED"),_enum("F"),"$"]
    if kind["class"]=="IfcDoorType":
        spec = kind["filling"]
        operation = "DOUBLE_DOOR_SLIDING" if spec["operation"]=="sliding" and len(spec["panel_stations"])==3 else "NOTDEFINED"
        return [_enum(operation),_enum("F"),"$"]
    return []


SERVICE_CLASSES = ("IfcPipeSegment","IfcCovering","IfcChimney","IfcWasteTerminal","IfcBoiler","IfcSpaceHeater")


def _service(obj: dict, geometry: dict) -> dict:
    """🚰 Export occupied service material with its true native profile plane and sweep axis."""
    service = next(item for item in geometry.get("services",geometry.get("details",geometry.get("beams",[]))) if item["tag"]==obj["tag"])
    parts = []
    for part in service["parts"]:
        resolved = {**part,"profile":{"shape":"polygon"}}
        if "mesh" in part:
            resolved["profile"] = {"shape":"brep"}
        elif "section" in part:
            body = _core().part_body(part)
            resolved["outline"] = body["profile"]["points"]
            resolved["native_sweep"] = {"position":body["position"],"axis":body["axis"],"refdir":body["ref_direction"],"direction":[0,0,-1] if body["profile_plane"]=="XZ" else [0,0,1],"depth":body["depth"]}
        parts.append(resolved)
    return {"point":[0.,0.,0.],"refdir":None,"parts":parts,"profile":{},"depth":max(p["z1"] for p in parts)-min(p["z0"] for p in parts)}


BODIES = {"IfcStair":_service,"IfcRamp":_service,"IfcRampFlight":_service,"IfcBeam":_service,"IfcBuildingElementProxy":_service,"IfcFurnishingElement":_service,"IfcColumn":_column,"IfcSlab":_slab,"IfcWall":_wall,**{cls:_filling for cls in FILLING_CLASSES},**{cls:_service for cls in SERVICE_CLASSES}}


def _value(entry: dict) -> list:
    value = entry["value"]
    if isinstance(value, bool):
        return ["IFCBOOLEAN(", (".T." if value else ".F.", "value"), ")"]
    if isinstance(value, str):
        return [("IFCTEXT" if entry.get("ifc_type")=="IfcText" else "IFCLABEL")+"(", _str(value,"value"), ")"]
    return ["IFCREAL(", (_real(value), "value"), ")"]


def _ramp_assembly_excerpt(result: dict, model: dict, obj: dict, language: str) -> dict:
    """🛝️ Aggregate native flight products beneath a parent owning landing material only."""
    core = _core()
    children = core.ramp_geometry(obj["tag"],model)["children"]
    relation = next(r for r in model["relations"] if r["class"]=="IfcRelAggregates" and r["relating"]==obj["tag"])
    if set(relation["related"])!={c["tag"] for c in children}:
        raise ValueError("A ramp aggregation must match its authored native flight products")
    lines = result["lines"][:-2]
    next_id = max(int(ref[1:]) for ref in result["ids"].values())+1
    refs,products = [],[]
    for child in children:
        fragment = step_excerpt(child["tag"],model,first_id=next_id,language=language,spatial=False)
        start = fragment["lines"].index("DATA;")+1
        offset = len(lines)-start
        lines += fragment["lines"][start:-2]
        result["spans"] += [{**span,"line":span["line"]+offset} for span in fragment["spans"] if start<=span["line"]<len(fragment["lines"])-2]
        prefix = "child_"+child["tag"]+"."
        result["ids"].update({prefix+key:value for key,value in fragment["ids"].items()})
        result["entities"].update({prefix+key:value+offset for key,value in fragment["entities"].items()})
        refs.append(fragment["ids"]["element"])
        products.append({**child,"ids":fragment["ids"]})
        next_id = max(int(ref[1:]) for ref in fragment["ids"].values())+1
    result["ids"]["aggregation"] = f"#{next_id}"
    result["entities"]["aggregation"] = len(lines)
    lines.append(f"#{next_id}=IFCRELAGGREGATES('{relation['guid']}',$,'{_escape(relation['name'][language])}',$,{result['ids']['element']},({','.join(refs)}));")
    result.update({"lines":lines+["ENDSEC;","END-ISO-10303-21;"],"children":products,"scope":"selected_parent_owned_landings_and_native_child_products"})
    return result


def step_excerpt(key: str, model: dict | None = None, *, first_id: int = 101, language: str = "en", spatial: bool = True, properties: tuple = PROPERTIES) -> dict:
    """📄 Write a self-consistent ISO 10303-21 fragment (IFC4X3_ADD2) for one native product with its occupied material parts; a property set that belongs to the type hangs on its ``HasPropertySets``, the occurrence reaches it through ``IFCRELDEFINESBYTYPE``.

    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcColumn.htm
    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcGloballyUniqueId.htm
    """
    core = _core()
    m = core.load_model() if model is None else model
    obj = core.element(key, m)
    cls = obj["class"]
    if cls not in BODIES or language not in LANGUAGES or isinstance(first_id, bool) or not isinstance(first_id, int) or first_id < 1:
        raise ValueError(f"No STEP excerpt for {key} ({cls}, {language}, {first_id})")
    if cls=="IfcStair":
        geometry = {"details":[core.stair_geometry(key,m)]}
    elif cls in ("IfcRamp","IfcRampFlight"):
        geometry = {"details":[core.ramp_geometry(key,m)]}
    elif cls in ("IfcBuildingElementProxy","IfcBeam","IfcFurnishingElement"):
        geometry = {"details":[core.solid_geometry(key,m)]}
    elif cls in SERVICE_CLASSES:
        geometry = {"services":[core.service_geometry(key,m)]}
    elif cls in FILLING_CLASSES:
        physical = core.filling_geometry(key,m)
        geometry = {"curtain_wall":physical} if cls=="IfcCurtainWall" else {"fillings":[physical]}
    elif cls == "IfcColumn":
        geometry = {"pilotis":[core.column_geometry(m,obj)]}
    elif cls == "IfcSlab":
        geometry = {"slabs":[core._slab_geometry(m,obj)]}
    else:
        geometry = core.building_geometry(m)
    body = BODIES[cls](obj,geometry)
    profile, tag = body["profile"], obj["tag"]
    multipart = "parts" in body
    centre = "centre" in profile
    material = obj.get("material")
    if cls=="IfcFurnishingElement":
        for part in body["parts"]:
            part["material"]=part.get("material",material["tag"] if material else None)
            if part["material"] is None or core.element(part["material"],m)["class"]!="IfcMaterial":
                raise ValueError("Furnishing material parts require an explicit or product material")
            part.setdefault("role","owned_material")
    furnishing_quantity = core.quantities(key,m) if cls=="IfcFurnishingElement" else None
    constituents = _constituents(body["parts"],cls) if multipart and (cls in FILLING_CLASSES or all("material" in p and "role" in p for p in body["parts"])) else []
    component_materials = list(dict.fromkeys(c["material"] for c in constituents))
    layers = None if constituents else material.get("layers") if material else None
    ownership = {}
    if cls in ("IfcBeam","IfcSlab"):
        quantities = core.quantities(key,m)
        physical = geometry["details"][0] if cls=="IfcBeam" else geometry["slabs"][0]
        owners = list(dict.fromkeys(obj["geometry"].get("embedded_slabs",[])+obj["geometry"].get("joint_owners",[])+obj["geometry"].get("column_reserves",[])+[p["owner"] for p in physical.get("shared_parts",[]) if "owner" in p]))
        values = {"BodyScope":"exclusive_owned_structural_material" if cls=="IfcSlab" else "exclusive_owned_material","FullMemberScope":"authored_slab_net_architectural_openings" if cls=="IfcSlab" else "authored_member_net_reserved_passages","FullMemberVolumeM3":quantities["volume"],"OwnedVolumeM3":quantities["exclusive_volume"],"SharedVolumeM3":quantities["shared_volume"],"SharedOwnerTags":"; ".join(owners)}
        if cls=="IfcSlab":
            values.update({"FinishScope":"separate_owned_weathering_material","FullFinishVolumeM3":quantities["finish_volume"],"OwnedFinishVolumeM3":quantities["exclusive_finish_volume"],"SharedFinishVolumeM3":quantities["shared_finish_volume"]})
        ownership = {name:{"value":value,"ifc_type":"IfcText"} for name,value in values.items()}
    if cls in ("IfcRamp","IfcRampFlight"):
        physical = geometry["details"][0]
        quantities = core.quantities(key,m)
        values = {"BodyScope":"exclusive_owned_material","FullMemberScope":"selected_assembly_with_explicit_child_and_shared_ownership" if cls=="IfcRamp" else "native_flight_product","FullMemberVolumeM3":quantities["volume"],"OwnedVolumeM3":quantities["exclusive_volume"],"SharedVolumeM3":quantities["shared_volume"],"ChildProductVolumeM3":quantities["child_volume"],"SharedComponentReferences":"; ".join(p["owner_guid"]+"/"+p["tag"] for p in physical["shared_parts"]),"ThicknessDirection":"vertical","ThicknessDirectionBasis":"inferred","NominalThicknessM":obj["geometry"]["thickness"]["value"]}
        ownership = {name:{"value":value,"ifc_type":"IfcText"} for name,value in values.items()}
    psets = obj.get("psets", {})
    historical = psets.get("Pset_HistoricalEvidence",{})
    if not historical and obj.get("historical_evidence"):
        evidence = obj["historical_evidence"]
        values = {"TargetState":evidence["state"],"Status":evidence["status"],"SourceReferences":"; ".join(evidence["sources"]),"ReconstructionUncertainty":evidence["uncertainty"][language]}
        historical = {name:{"value":value,"ifc_type":"IfcText"} for name,value in values.items()}
    pset = f"Pset_{cls[3:]}Common" if f"Pset_{cls[3:]}Common" in psets else next(iter(psets), None)
    props = [name for name in properties if pset and name in psets[pset]]
    kind = core.element(obj["type"], m) if obj.get("type") else None
    storey = core.element(obj["storey"], m)
    type_level = bool(props and spatial and kind and pset in (kind.get("psets") or {}))
    keys = ["element", "placement", "axis", "point"] + (["refdir"] if body["refdir"] else []) + ["shape", "representation", "solid", "profile"]
    if multipart:
        keys = [k for k in keys if k not in ("solid", "profile")]
        for i, part in enumerate(body["parts"]):
            if "mesh" in part:
                keys += [f"solid{i}",f"shell{i}"]
                for j in range(len(part["mesh"]["faces"])):
                    keys += [f"face{i}_{j}",f"bound{i}_{j}",f"loop{i}_{j}"]
                keys += [f"brep_vertex{i}_{j}" for j in range(len(part["mesh"]["vertices"]))]
                continue
            keys += [f"solid{i}", f"profile{i}", f"axis{i}", f"point{i}"]
            if "native_sweep" in part:
                keys += [f"sweep_axis{i}",f"sweep_refdir{i}",f"sweep_direction{i}"]
            shape = _native_profile_kind(part)
            if shape in ("polygon","profile_void"):
                if shape == "profile_void" and part["profile"]["shape"] in ("circle","ellipse"):
                    keys += [f"outer_curve{i}",f"outer_axis{i}",f"outer_point{i}",f"outer_dir{i}"]
                else:
                    keys += [f"outer_curve{i}" if shape == "profile_void" else f"polyline{i}"]+[f"vertex{i}_{j}" for j in range(len(part["outline"]))]
                for h,hole in enumerate(part.get("holes",())):
                    keys += [f"hole_curve{i}_{h}",f"hole_axis{i}_{h}",f"hole_point{i}_{h}"]
                if "top_outline" in part:
                    keys += [f"end_profile{i}",f"end_polyline{i}"]+[f"end_vertex{i}_{j}" for j in range(len(part["top_outline"]))]
            else:
                keys += [f"profile_axis{i}", f"profile_point{i}", f"profile_dir{i}"]
    keys += (["profile_axis", "profile_point"] if centre else []) + ["direction", "context", "world_axis", "world_point"]
    if constituents:
        for i in range(len(constituents)):
            keys += [f"aspect_representation{i}",f"aspect{i}"]
        keys += ["constituentset"]+[f"constituent{i}" for i in range(len(constituents))]+[f"component_material{i}" for i in range(len(component_materials))]
    elif layers:
        keys += ["layerset"] + [f"layer{i}" for i in range(len(layers))] + [f"layer_material{i}" for i in range(len(layers))]
    elif material:
        keys += ["material"]
    keys += (["rel_material"] if material or constituents else []) + [f"prop_{name}" for name in props] + (["pset"] + ([] if type_level else ["rel_pset"]) if props else [])
    if historical:
        keys += [f"historical_{name}" for name in historical]+["historical_pset","historical_relation"]
    if ownership:
        keys += [f"ownership_{name}" for name in ownership]+["ownership_pset","ownership_relation"]
    if furnishing_quantity is not None:
        keys += ["quantity_unit","quantity_volume","quantity_set","quantity_relation"]
    if spatial:
        keys += (["type", "rel_type"] if kind else []) + ["storey", "rel_storey"]
    w = _Writer(first_id, keys)

    def guid(suffix, owner=None):
        return _str(core.global_id(f"{owner or tag}.{suffix}"))

    w.lines += ["ISO-10303-21;", "HEADER;", "FILE_DESCRIPTION(('ViewDefinition [DesignTransferView]'),'2;1');",
                f"FILE_NAME('{FILENAME}','{STAMP}',('Teaching'),('semio'),'','','');", f"FILE_SCHEMA(('{SCHEMA}'));", "ENDSEC;", "DATA;"]
    attributes = [_str(obj["guid"],"guid"),"$",_str(obj["name"][language],"name"),"$","$",w.ref("placement","placement"),w.ref("shape","representation"),_str(tag,"tag")]
    has_predefined = any("PredefinedType" in level["adds"] for level in core.class_chain(cls,m))
    predefined = "$" if not has_predefined or cls in FILLING_CLASSES and spatial and kind else _enum(obj["predefined_type"],"predefined")
    if cls in ("IfcWindow","IfcDoor"):
        attributes += [(_real(body["rough_height"]),"height"),(_real(body["rough_width"]),"width"),predefined,"$","$"]
    elif has_predefined:
        attributes += [predefined]
    if cls == "IfcChimney":
        attributes += [(_real(body["depth"]),"height")]
    w.add("element",cls.upper(),*attributes,role="class")
    w.add("placement", "IFCLOCALPLACEMENT", "$", w.ref("axis"))
    w.add("axis", "IFCAXIS2PLACEMENT3D", w.ref("point"), "$", w.ref("refdir") if body["refdir"] else "$")
    w.add("point", "IFCCARTESIANPOINT", _coords(body["point"], "placement"))
    if body["refdir"]:
        w.add("refdir", "IFCDIRECTION", _coords(body["refdir"]))
    w.add("shape", "IFCPRODUCTDEFINITIONSHAPE", "$", "$", _tuple(w.ref("representation")))
    solids = [w.ref(f"solid{i}") for i in range(len(body["parts"]))] if multipart else [w.ref("solid")]
    representation_type = "SolidModel" if multipart and any("mesh" in p for p in body["parts"]) else "SweptSolid"
    w.add("representation", "IFCSHAPEREPRESENTATION", w.ref("context"), _str("Body", "representation"), _str(representation_type, "representation"), _tuple(*solids))
    if multipart:
        for i, part in enumerate(body["parts"]):
            if "mesh" in part:
                vertices,faces = core.part_vertices(part),core.part_faces(part)
                w.add(f"solid{i}","IFCFACETEDBREP",w.ref(f"shell{i}"))
                w.add(f"shell{i}","IFCCLOSEDSHELL",_tuple(*[w.ref(f"face{i}_{j}") for j in range(len(faces))]))
                for j,face in enumerate(faces):
                    w.add(f"face{i}_{j}","IFCFACE",_tuple(w.ref(f"bound{i}_{j}")))
                    w.add(f"bound{i}_{j}","IFCFACEOUTERBOUND",w.ref(f"loop{i}_{j}"),_enum("T"))
                    w.add(f"loop{i}_{j}","IFCPOLYLOOP",_tuple(*[w.ref(f"brep_vertex{i}_{k}") for k in face]))
                for j,vertex in enumerate(vertices):
                    w.add(f"brep_vertex{i}_{j}","IFCCARTESIANPOINT",_coords(vertex))
                continue
            p = part["profile"]
            shape = _native_profile_kind(part)
            solid_class = "IFCEXTRUDEDAREASOLIDTAPERED" if "top_outline" in part else "IFCEXTRUDEDAREASOLID"
            end = [w.ref(f"end_profile{i}")] if "top_outline" in part else []
            native = part.get("native_sweep")
            direction = w.ref(f"sweep_direction{i}") if native else w.ref("direction")
            depth = native["depth"] if native else part["z1"]-part["z0"]
            w.add(f"solid{i}",solid_class,w.ref(f"profile{i}"),w.ref(f"axis{i}"),direction,(_real(depth),"depth"),*end)
            if shape in ("polygon","profile_void"):
                if shape == "profile_void":
                    w.add(f"profile{i}","IFCARBITRARYPROFILEDEFWITHVOIDS",_enum("AREA"),"$",w.ref(f"outer_curve{i}"),_tuple(*[w.ref(f"hole_curve{i}_{h}") for h in range(len(part["holes"]))]))
                else:
                    w.add(f"profile{i}", "IFCARBITRARYCLOSEDPROFILEDEF", _enum("AREA"), "$", w.ref(f"polyline{i}"))
            else:
                clsname = {"circle":"IFCCIRCLEPROFILEDEF","circle_hollow":"IFCCIRCLEHOLLOWPROFILEDEF","rectangle":"IFCRECTANGLEPROFILEDEF","ellipse":"IFCELLIPSEPROFILEDEF"}[shape]
                dimensions = [p["diameter"]["value"]/2] if shape in ("circle","circle_hollow") else [p["width"]["value"],p["depth"]["value"]]
                if shape == "circle_hollow":
                    dimensions.append((p["diameter"]["value"]-p["voids"][0]["diameter"]["value"])/2)
                if shape == "ellipse":
                    dimensions = [v / 2 for v in dimensions]
                w.add(f"profile{i}", clsname, _enum("AREA"), "$", w.ref(f"profile_axis{i}"), *[(_real(v), "profile") for v in dimensions])
            w.add(f"axis{i}","IFCAXIS2PLACEMENT3D",w.ref(f"point{i}"),w.ref(f"sweep_axis{i}") if native else "$",w.ref(f"sweep_refdir{i}") if native else "$")
            w.add(f"point{i}","IFCCARTESIANPOINT",_coords(native["position"] if native else [0.,0.,part["z0"]]))
            if native:
                w.add(f"sweep_axis{i}","IFCDIRECTION",_coords(native["axis"]))
                w.add(f"sweep_refdir{i}","IFCDIRECTION",_coords(native["refdir"]))
                w.add(f"sweep_direction{i}","IFCDIRECTION",_coords(native["direction"]))
            if shape in ("polygon","profile_void"):
                if shape == "profile_void" and p["shape"] in ("circle","ellipse"):
                    dimensions = [p["diameter"]["value"]/2] if p["shape"] == "circle" else [p["width"]["value"]/2,p["depth"]["value"]/2]
                    w.add(f"outer_curve{i}","IFCCIRCLE" if p["shape"] == "circle" else "IFCELLIPSE",w.ref(f"outer_axis{i}"),*[_real(d) for d in dimensions])
                    centre = [sum(v[k] for v in part["outline"])/len(part["outline"]) for k in (0,1)]
                    angle = math.radians(p.get("rotation",0))
                    w.add(f"outer_axis{i}","IFCAXIS2PLACEMENT2D",w.ref(f"outer_point{i}"),w.ref(f"outer_dir{i}"))
                    w.add(f"outer_point{i}","IFCCARTESIANPOINT",_coords(centre))
                    w.add(f"outer_dir{i}","IFCDIRECTION",_coords([math.cos(angle),math.sin(angle)]))
                else:
                    refs = [w.ref(f"vertex{i}_{j}") for j in range(len(part["outline"]))]
                    w.add(f"outer_curve{i}" if shape == "profile_void" else f"polyline{i}","IFCPOLYLINE",_tuple(*(refs+refs[:1])))
                    for j,point in enumerate(part["outline"]):
                        w.add(f"vertex{i}_{j}","IFCCARTESIANPOINT",_coords(point))
                for h,hole in enumerate(part.get("holes",())):
                    centre = [sum(v[k] for v in hole)/len(hole) for k in (0,1)]
                    radius = p["voids"][h]["diameter"]["value"]/2
                    w.add(f"hole_curve{i}_{h}","IFCCIRCLE",w.ref(f"hole_axis{i}_{h}"),(_real(radius),"profile"))
                    w.add(f"hole_axis{i}_{h}","IFCAXIS2PLACEMENT2D",w.ref(f"hole_point{i}_{h}"),"$")
                    w.add(f"hole_point{i}_{h}","IFCCARTESIANPOINT",_coords(centre))
                if "top_outline" in part:
                    w.add(f"end_profile{i}","IFCARBITRARYCLOSEDPROFILEDEF",_enum("AREA"),"$",w.ref(f"end_polyline{i}"))
                    refs = [w.ref(f"end_vertex{i}_{j}") for j in range(len(part["top_outline"]))]
                    w.add(f"end_polyline{i}","IFCPOLYLINE",_tuple(*(refs+refs[:1])))
                    for j,point in enumerate(part["top_outline"]):
                        w.add(f"end_vertex{i}_{j}","IFCCARTESIANPOINT",_coords(point))
            else:
                centre_point = [sum(v[k] for v in part["outline"]) / len(part["outline"]) for k in (0,1)]
                angle = math.radians(p.get("rotation", 0))
                w.add(f"profile_axis{i}", "IFCAXIS2PLACEMENT2D", w.ref(f"profile_point{i}"), w.ref(f"profile_dir{i}"))
                w.add(f"profile_point{i}", "IFCCARTESIANPOINT", _coords(centre_point))
                w.add(f"profile_dir{i}", "IFCDIRECTION", _coords([math.cos(angle), math.sin(angle)]))
    else:
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
    if constituents:
        for i,component in enumerate(constituents):
            refs = [w.ref(f"solid{j}") for j in component["parts"]]
            w.add(f"aspect_representation{i}","IFCSHAPEREPRESENTATION",w.ref("context"),_str("Body"),_str("SweptSolid"),_tuple(*refs))
            w.add(f"aspect{i}","IFCSHAPEASPECT",_tuple(w.ref(f"aspect_representation{i}")),_str(component["name"]),"$",_enum("T"),w.ref("shape"))
        w.add("constituentset","IFCMATERIALCONSTITUENTSET",_str(obj["name"][language],"material"),"$",_tuple(*[w.ref(f"constituent{i}") for i in range(len(constituents))]))
        for i,component in enumerate(constituents):
            material_index = component_materials.index(component["material"])
            w.add(f"constituent{i}","IFCMATERIALCONSTITUENT",_str(component["name"]),"$",w.ref(f"component_material{material_index}"),"$",_str(component["category"]))
        for i,material_tag in enumerate(component_materials):
            w.add(f"component_material{i}","IFCMATERIAL",_str(core.element(material_tag,m)["name"][language],"material"),"$",_str(core.element(material_tag,m)["category"]) if "category" in core.element(material_tag,m) else "$")
    elif layers:
        w.add("layerset", "IFCMATERIALLAYERSET", _tuple(*[w.ref(f"layer{i}") for i in range(len(layers))]), _str(material["name"][language], "material"), "$")
        for i, layer in enumerate(layers):
            w.add(f"layer{i}", "IFCMATERIALLAYER", w.ref(f"layer_material{i}"), _real(layer["thickness"]["value"]), "$", "$", "$", "$", "$")
        for i, layer in enumerate(layers):
            w.add(f"layer_material{i}", "IFCMATERIAL", _str(core.element(layer["material"], m)["name"][language], "material"), "$", "$")
    elif material:
        w.add("material", "IFCMATERIAL", _str(material["name"][language], "material"), "$", "$")
    if material or constituents:
        related = [w.ref("element")]+([w.ref("type")] if constituents and spatial and kind else [])
        w.add("rel_material","IFCRELASSOCIATESMATERIAL",guid("material"),"$","$","$",_tuple(*related),w.ref("constituentset" if constituents else "layerset" if layers else "material"))
    for name in props:
        w.add(f"prop_{name}", "IFCPROPERTYSINGLEVALUE", _str(name, "property"), "$", _value(psets[pset][name]), "$")
    if props:
        w.add("pset", "IFCPROPERTYSET", guid(pset, kind["tag"] if type_level else None), "$", _str(pset, "pset"), "$", _tuple(*[w.ref(f"prop_{name}") for name in props]))
        if not type_level:
            w.add("rel_pset", "IFCRELDEFINESBYPROPERTIES", guid(f"{pset}.rel"), "$", "$", "$", _tuple(w.ref("element")), w.ref("pset"))
    if historical:
        for name,entry in historical.items():
            w.add(f"historical_{name}","IFCPROPERTYSINGLEVALUE",_str(name),"$",_value(entry),"$")
        w.add("historical_pset","IFCPROPERTYSET",guid("Pset_HistoricalEvidence"),"$",_str("Pset_HistoricalEvidence"),"$",_tuple(*[w.ref(f"historical_{name}") for name in historical]))
        w.add("historical_relation","IFCRELDEFINESBYPROPERTIES",guid("Pset_HistoricalEvidence.rel"),"$","$","$",_tuple(w.ref("element")),w.ref("historical_pset"))
    if ownership:
        for name,entry in ownership.items():
            w.add(f"ownership_{name}","IFCPROPERTYSINGLEVALUE",_str(name),"$",_value(entry),"$")
        w.add("ownership_pset","IFCPROPERTYSET",guid("Pset_MaterialOwnership"),"$",_str("Pset_MaterialOwnership"),"$",_tuple(*[w.ref(f"ownership_{name}") for name in ownership]))
        w.add("ownership_relation","IFCRELDEFINESBYPROPERTIES",guid("Pset_MaterialOwnership.rel"),"$","$","$",_tuple(w.ref("element")),w.ref("ownership_pset"))
    if furnishing_quantity is not None:
        w.add("quantity_unit","IFCSIUNIT","*",_enum("VOLUMEUNIT"),"$",_enum("CUBIC_METRE"))
        w.add("quantity_volume","IFCQUANTITYVOLUME",_str("NetVolume"),_str("Occupied material union" if language=="en" else "Vereinigung des belegten Materials"),w.ref("quantity_unit"),(_real(furnishing_quantity["volume"]),"quantity"),"$")
        w.add("quantity_set","IFCELEMENTQUANTITY",guid("FurnishingOwnedQuantities"),"$",_str("FurnishingOwnedQuantities"),"$",_str("Union of occupied material" if language=="en" else "Vereinigung des belegten Materials"),_tuple(w.ref("quantity_volume")))
        w.add("quantity_relation","IFCRELDEFINESBYPROPERTIES",guid("FurnishingOwnedQuantities.rel"),"$","$","$",_tuple(w.ref("element")),w.ref("quantity_set"))
    if spatial:
        if kind:
            w.add("type",kind["class"].upper(),_str(kind["guid"]),"$",_str(kind["name"][language]),"$","$",_tuple(w.ref("pset")) if type_level else "$","$","$","$",_enum(kind["predefined_type"]),*_type_layout(kind))
            w.add("rel_type", "IFCRELDEFINESBYTYPE", guid("type"), "$", "$", "$", _tuple(w.ref("element")), w.ref("type"))
        w.add("storey", "IFCBUILDINGSTOREY", _str(storey["guid"]), "$", _str(storey["name"][language]), "$", "$", "$", "$", "$", _enum("ELEMENT"), _real(storey["elevation"]["value"]))
        w.add("rel_storey", "IFCRELCONTAINEDINSPATIALSTRUCTURE", guid("storey"), "$", "$", "$", _tuple(w.ref("element")), w.ref("storey"))
    w.lines += ["ENDSEC;", "END-ISO-10303-21;"]
    result = {"key": tag, "guid": obj["guid"], "class": cls, "schema": SCHEMA, "language": language, "first_id": first_id, "lines": w.lines, "spans": w.spans,
            "ids": {name: f"#{number}" for name, number in w.ids.items()}, "entities": w.entities}

    return _ramp_assembly_excerpt(result,m,obj,language) if cls=="IfcRamp" else result


def _triangulate(ring: list) -> list:
    indices, triangles = list(range(len(ring))), []
    def cross(a, b, c):
        return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
    if all(cross(ring[i-1], ring[i], ring[(i+1) % len(ring)]) > 1e-14 for i in range(len(ring))):
        return [[0, i, i+1] for i in range(1, len(ring)-1)]
    while len(indices) > 3:
        for j, b in enumerate(indices):
            a, c = indices[j-1], indices[(j+1)%len(indices)]
            if cross(ring[a],ring[b],ring[c]) <= 1e-14:
                continue
            if any(all(cross(ring[u],ring[v],ring[k]) >= -1e-14 for u,v in ((a,b),(b,c),(c,a))) for k in indices if k not in (a,b,c)):
                continue
            triangles.append([a,b,c])
            indices.pop(j)
            break
        else:
            raise ValueError("Column profile must be a simple polygon")
    return triangles + [indices]


def mesh(key: str, segments: int = 16, model: dict | None = None) -> dict:
    """🔺 Tessellate physical column profile sections into closed triangle meshes with ear-clipped caps and list what the mesh keeps and loses.

    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcTriangulatedFaceSet.htm
    """
    core = _core()
    m = core.load_model() if model is None else model
    obj = core.element(key, m)
    if obj["class"] != "IfcColumn":
        raise ValueError(f"No mesh export for {obj['class']}")
    if isinstance(segments, bool) or not isinstance(segments, int) or segments < 3:
        raise ValueError("A mesh needs at least 3 segments")
    p = core.column_geometry(m, core.element(key,m), segments)
    vertices, faces = [], []
    layouts = []
    for part in p["parts"]:
        if part.get("holes"):
            cap = core.profile_mesh([part["outline"],*part["holes"]])
            points,native = core.part_vertices(part),core.part_faces(part)
            offset,face_offset,n = len(vertices),len(faces),len(points)//2
            vertices += points
            boundary_count = sum(len(r) for r in cap["boundaries"])
            for face in native:
                faces += [[offset+face[0],offset+face[i],offset+face[i+1]] for i in range(1,len(face)-1)]
            side_end = face_offset+2*boundary_count
            top_end = side_end+len(cap["triangles"])
            layouts.append({"bottom":list(range(offset,offset+n)),"top":list(range(offset+n,offset+2*n)),"side_faces":[face_offset,side_end],"top_faces":[side_end,top_end],"bottom_faces":[top_end,len(faces)],"holes":len(part["holes"])})
            continue
        ring = part["outline"]
        n, offset, face_offset = len(ring), len(vertices), len(faces)
        vertices += [[px, py, part["z0"]] for px, py in ring] + [[px, py, part["z1"]] for px, py in ring]
        cap = _triangulate(ring)
        sides = [face for i in range(n) for j in [(i + 1) % n] for face in ([i,j,n+j],[i,n+j,n+i])]
        faces += [[offset + k for k in f] for f in sides + [[n+a,n+b,n+c] for a,b,c in cap] + [[c,b,a] for a,b,c in cap]]
        layouts.append({"bottom": list(range(offset,offset+n)), "top": list(range(offset+n,offset+2*n)),
                        "side_faces": [face_offset, face_offset+2*n], "top_faces": [face_offset+2*n,face_offset+2*n+len(cap)],
                        "bottom_faces": [face_offset+2*n+len(cap),len(faces)]})
    x, y = p["x"], p["y"]
    psets = obj.get("psets", {})
    pset = next((name for name, props in psets.items() if "LoadBearing" in props), None)
    kind = core.element(obj["type"], m) if obj.get("type") else None
    material = obj.get("material")
    kept = [{"id": "vertices", "en": "Vertices", "de": "Punkte"}, {"id": "triangles", "en": "Triangles", "de": "Dreiecke"}]
    lost = [{"id": "class", "en": f"Class {obj['class']}", "de": f"Klasse {obj['class']}"}, {"id": "guid", "en": "GlobalId", "de": "GlobalId"}]
    lost += [{"id": "property", "en": f"{pset}.LoadBearing", "de": f"{pset}.LoadBearing"}] if pset else []
    lost += [{"id": "material", "en": f"Material {material['name']['en']}", "de": f"Material {material['name']['de']}"}] if material else []
    lost += [{"id": "type", "en": f"Type {kind['name']['en']}", "de": f"Typ {kind['name']['de']}"}] if kind else []
    return {"tag": obj["tag"], "guid": obj["guid"], "segments": segments, "height": p["top"] - p["base"], "centre": [x, y],
            "vertices": vertices, "faces": faces, "vertex_count": len(vertices), "triangle_count": len(faces), "kept": kept, "lost": lost,
            "layout": layouts[0], "part_layouts": layouts, "profile": p["profile"]}



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
                                                                                                                                                                                                                                         