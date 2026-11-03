"""🔌 Exchange forms of the Villa Savoye teaching model: STEP excerpt, triangle mesh and IDS-style check (stdlib only)."""

from __future__ import annotations

import importlib.util
import importlib
import math
import json
import re
import sys
from pathlib import Path
from types import SimpleNamespace

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
    """🔢 Preserve every finite binary64 value in a valid round-trip STEP real token."""
    value=float(value)
    if not math.isfinite(value):raise ValueError("Native STEP reals must be finite")
    if value==0:return "0."
    mantissa,separator,exponent=repr(value).upper().partition("E")
    if "." not in mantissa:mantissa+="."
    return mantissa+(separator+exponent if separator else "")


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


def _analytic_geometry_plan(operand: dict, key: str):
    """🌳 Plan exact native analytic stock operands without importing a display tessellation."""
    return importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.➖️difference.🐍️').Plan(SimpleNamespace(**globals()),operand,key)


def _profile_axis(part: dict) -> list:
    """🧿 Preserve explicit analytic source placement without facet or material-centroid inference."""
    axis=part.get('axis')
    if not isinstance(axis,(list,tuple)) or len(axis)!=2 or any(type(value) not in (int,float) or not math.isfinite(value) for value in axis):raise ValueError('Native analytic profiles require an explicit finite source axis')
    return list(axis)


def _void_axis(part: dict, hole: dict) -> list:
    """🎯 Place each source bore by its declared profile-local offset and rotation."""
    axis=_profile_axis(part);angle=math.radians(part['profile'].get('rotation',0));c,s=math.cos(angle),math.sin(angle);x,y=hole['offset']
    return [axis[0]+c*x-s*y,axis[1]+s*x+c*y]


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
    parts = [{**part} if 'csg' in part else {**part, "profile": {"shape": "polygon"}} for part in slab["parts"]]
    return {"point": [0., 0., 0.], "refdir": None, "depth": slab["thickness"], "profile": {}, "parts": parts}

def _surface_properties(definitions: list) -> dict:
    """🧾 Preserve source coating facts and unknown film/RGB measurements on native material definitions."""
    result = {}
    for key,material in definitions:
        finish = material.get("surface_finish")
        if not finish:
            continue
        fields = {"Binder":finish["binder"],"ColourFamily":finish["colour_family"],"Substrate":finish["substrate"],"QuantityScope":finish["quantity_scope"],"FilmThickness":finish["film_thickness"],"CoatingMass":"unmeasured","OriginalRGB":"unmeasured","EvidenceBasis":finish["basis"],"EvidenceSource":finish["source"]}
        if finish.get("display_rgb"):
            fields.update(DisplayRGBBasis=finish["display_rgb"]["basis"],DisplayRGBSource=finish["display_rgb"]["source"])
        result[key] = {name:{"value":value,"ifc_type":"IfcText"} for name,value in fields.items()}
    return result


def _native_faces(core, part: dict) -> list:
    """🔻 Preserve the oriented existing substrate faces when styling a real exposed plane."""
    return [list(face) for face in part["mesh"]["faces"]] if "surface_finish" in part and "mesh" in part else core.part_faces(part)


def _wall(obj: dict, geometry: dict) -> dict:
    w = next(x for x in geometry["walls"]+geometry.get("screens",[]) if x["tag"] == obj["tag"])
    parts = []
    coated = any("surface_finish" in p for p in w["parts"])
    for part in w["parts"]:
        if 'csg' in part:
            parts.append({**part})
            continue
        resolved = {**part,"profile":{"shape":"polygon"}}
        if coated:
            resolved.setdefault("role",part.get("layer","body"))
        if "surface_finish" in part:
            resolved["mesh"] = {"vertices":_core().part_vertices(part),"faces":_core().part_faces(part)}
            resolved["profile"] = {"shape":"brep"}
        parts.append(resolved)
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
            **{k:filling[k] for k in ("rough_width","rough_height","overall_height") if k in filling}}


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


SERVICE_CLASSES = ("IfcPipeSegment","IfcCovering","IfcChimney","IfcWasteTerminal","IfcBoiler","IfcSpaceHeater","IfcSanitaryTerminal","IfcLightFixture")


def _service(obj: dict, geometry: dict) -> dict:
    """🚰 Export occupied service material with its true native profile plane and sweep axis."""
    service = next(item for item in geometry.get("services",geometry.get("details",geometry.get("beams",[]))) if item["tag"]==obj["tag"])
    parts = []
    for part in service["parts"]:
        if 'csg' in part:
            parts.append({**part})
            continue
        resolved = {**part,"profile":{"shape":"polygon"}}
        if "mesh" in part:
            resolved["profile"] = {"shape":"brep"}
        elif "section" in part:
            body = _core().part_body(part)
            resolved["outline"] = body["profile"]["points"]
            resolved["native_sweep"] = {"position":body["position"],"axis":body["axis"],"refdir":body["ref_direction"],"direction":[0,0,-1] if body["profile_plane"]=="XZ" else [0,0,1],"depth":body["depth"]}
        parts.append(resolved)
    return {"point":[0.,0.,0.],"refdir":None,"parts":parts,"profile":{},"depth":max(_part_interval(p)[1] for p in parts)-min(_part_interval(p)[0] for p in parts)}


def _part_interval(part: dict) -> tuple:
    """📏 Read native extrusion bounds from authoritative geometry without a display fallback for analytic parts."""
    if 'csg' in part:
        backend=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.➖️difference.🐍️');return backend.interval(backend.stock(part['csg']))
    if 'z0' in part and 'z1' in part:return part['z0'],part['z1']
    vertices=_core().part_vertices(part);return min(point[2] for point in vertices),max(point[2] for point in vertices)


BODIES = {"IfcStair":_service,"IfcRamp":_service,"IfcRampFlight":_service,"IfcBeam":_service,"IfcFooting":_service,"IfcRailing":_service,"IfcOpeningElement":_service,"IfcBuildingElementProxy":_service,"IfcFurnishingElement":_service,"IfcColumn":_column,"IfcSlab":_slab,"IfcWall":_wall,**{cls:_filling for cls in FILLING_CLASSES},**{cls:_service for cls in SERVICE_CLASSES}}


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


def _opening_host_excerpt(result: dict, model: dict, obj: dict, language: str, spatial: bool) -> dict:
    """🕳️ Relate a reference void to its already-cut native host without spatially containing the opening.

    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcOpeningElement.htm
    """
    host = next(obj["geometry"][key] for key in ("wall","slab","curtain_wall") if key in obj["geometry"])
    relations = [r for r in model["relations"] if r["class"]=="IfcRelVoidsElement" and r["relating"]==host and r["related"]==[obj["tag"]]]
    if len(relations)!=1:
        raise ValueError("A native opening requires exactly one authored host void relationship")
    lines = result["lines"][:-2]
    first_id = max(int(ref[1:]) for ref in result["ids"].values())+1
    fragment = step_excerpt(host,model,first_id=first_id,language=language,spatial=spatial)
    start = fragment["lines"].index("DATA;")+1
    offset = len(lines)-start
    lines += fragment["lines"][start:-2]
    result["spans"] += [{**span,"line":span["line"]+offset} for span in fragment["spans"] if start<=span["line"]<len(fragment["lines"])-2]
    result["ids"].update({"host."+key:value for key,value in fragment["ids"].items()})
    result["entities"].update({"host."+key:value+offset for key,value in fragment["entities"].items()})
    result["ids"]["void_relation"] = f"#{max(int(ref[1:]) for ref in result['ids'].values())+1}"
    result["entities"]["void_relation"] = len(lines)
    relation = relations[0]
    lines.append(f"{result['ids']['void_relation']}=IFCRELVOIDSELEMENT('{relation['guid']}',$,'{_escape(relation['name'][language])}',$,{fragment['ids']['element']},{result['ids']['element']});")
    result.update({"lines":lines+["ENDSEC;","END-ISO-10303-21;"],"scope":"selected_subtraction_and_native_host","host":host})
    return result


def _product_reference_excerpt(result: dict, model: dict, obj: dict, language: str) -> dict:
    """🧭 Attach nominal product Reference geometry without duplicate products or material relationships."""
    first = max(int(ref[1:]) for ref in result['ids'].values())+1
    reference = step_excerpt(obj['tag'],model,first_id=first,language=language,spatial=False,representation='Reference')
    lines = reference['lines'];by_id = {line.split('=',1)[0]:line for line in lines if re.match(r'^#[0-9]+=',line)}
    selected = {reference['ids']['representation']};pending = list(selected)
    while pending:
        line = by_id[pending.pop()]
        for item in re.findall(r'#[0-9]+',line.split('=',1)[1]):
            if item not in selected:selected.add(item);pending.append(item)
    kept = [i for i,line in enumerate(lines) if line.split('=',1)[0] in selected]
    offset = len(result['lines'])-2
    result['lines'] = result['lines'][:-2]+[lines[i] for i in kept]+result['lines'][-2:]
    positions = {old:offset+index for index,old in enumerate(kept)}
    result['spans'] += [{**span,'line':positions[span['line']]} for span in reference['spans'] if span['line'] in positions]
    for key,ref in reference['ids'].items():
        if ref in selected:result['ids']['reference.'+key]=ref;result['entities']['reference.'+key]=positions[reference['entities'][key]]
    index=result['entities']['shape'];line=result['lines'][index]
    result['lines'][index]=line[:-3]+','+reference['ids']['representation']+'));'
    result['spans']=[span for span in result['spans'] if span['line']!=index]
    return result


def wall_junction_excerpt(key: str, model: dict | None = None, *, first_id: int = 101, language: str | None = None) -> dict:
    """🔗 Export two actual joined walls and their explicit native IFC path or general material relationship."""
    core=_core();m=core.load_model() if model is None else model
    core._wall_junction_backend().context(core,m)
    relation=next((row for row in m.get('relations',()) if key in (row['tag'],row['guid'])),None)
    if relation is None or not(relation['class']=='IfcRelConnectsPathElements' or relation['class']=='IfcRelConnectsElements' and relation.get('connection',{}).get('kind')=='material_junction'):raise ValueError('Select an explicitly declared wall junction relationship')
    if language not in LANGUAGES:raise ValueError('Select an explicit known language for the native wall junction')
    if relation['class']=='IfcRelConnectsPathElements' and any(not core._wall_junction_backend().path_material_layers(core,m,core._find(m,tag)) for tag in (relation['relating'],*relation['related'])):raise ValueError('Native path priorities require actual ordered native material layers; use an explicit bounded general material interface for complex constituents')
    tags=[relation['relating'],*relation['related']];result=None;refs=[];next_id=first_id
    for index,tag in enumerate(tags):
        fragment=step_excerpt(tag,m,first_id=next_id,language=language,spatial=False)
        refs.append(fragment['ids']['element'])
        if result is None:result=fragment
        else:
            start=fragment['lines'].index('DATA;')+1;offset=len(result['lines'])-2-start
            result['lines']=result['lines'][:-2]+fragment['lines'][start:-2]+result['lines'][-2:]
            result['spans'] += [{**span,'line':span['line']+offset} for span in fragment['spans'] if start<=span['line']<len(fragment['lines'])-2]
            result['ids'].update({'related.'+name:ref for name,ref in fragment['ids'].items()})
            result['entities'].update({'related.'+name:line+offset for name,line in fragment['entities'].items()})
        next_id=max(int(ref[1:]) for ref in result['ids'].values())+1
    data=relation.get('path_connection',relation.get('connection'))
    description=json.dumps(data,sort_keys=True,separators=(',',':'),ensure_ascii=False)
    common=f"'{relation['guid']}',$,'{_escape(relation['name'][language])}','{_escape(description)}',$,{refs[0]},{refs[1]}"
    if relation['class']=='IfcRelConnectsPathElements':
        priorities=','.join('('+','.join(str(value) for value in data[field])+')' for field in ('relating_priorities','related_priorities'))
        suffix=','+priorities+',.'+data['related_connection_type']+'.,.'+data['relating_connection_type']+'.'
    else:suffix=''
    ref='#'+str(next_id);result['ids']['junction']=ref;result['entities']['junction']=len(result['lines'])-2
    result['lines'].insert(-2,ref+'='+relation['class'].upper()+'('+common+suffix+');')
    result.update({'key':relation['tag'],'guid':relation['guid'],'class':relation['class'],'participants':tags})
    return result


def step_excerpt(key: str, model: dict | None = None, *, first_id: int = 101, language: str = "en", spatial: bool = True, properties: tuple = PROPERTIES, property_set: str | None = None, representation: str = "Body") -> dict:
    """📄 Write a self-consistent ISO 10303-21 fragment (IFC4X3_ADD2) for one native product with its occupied material parts; a property set that belongs to the type hangs on its ``HasPropertySets``, the occurrence reaches it through ``IFCRELDEFINESBYTYPE``.

    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcColumn.htm
    https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcGloballyUniqueId.htm
    """
    core = _core()
    m = core.load_model() if model is None else model
    obj = core.element(key, m)
    cls = obj["class"]
    reference_classes=('IfcWall','IfcColumn','IfcSlab','IfcBeam','IfcFooting','IfcBuildingElementProxy','IfcRailing','IfcStair','IfcRamp','IfcRampFlight')
    if representation not in ("Body","Reference") or representation=="Reference" and cls not in reference_classes:raise ValueError("Reference excerpts require an explicit nominal product stock interface")
    if cls not in BODIES or language not in LANGUAGES or isinstance(first_id, bool) or not isinstance(first_id, int) or first_id < 1:
        raise ValueError(f"No STEP excerpt for {key} ({cls}, {language}, {first_id})")
    host_spatial = spatial
    if cls=="IfcOpeningElement":
        spatial = False
        geometry = {"details":[core.opening_geometry(key,m)]}
    elif cls=="IfcStair":
        geometry = {"details":[core.stair_geometry(key,m)]}
    elif cls in ("IfcRamp","IfcRampFlight"):
        geometry = {"details":[core.ramp_geometry(key,m)]}
    elif cls in ("IfcBuildingElementProxy","IfcBeam","IfcFooting","IfcRailing","IfcFurnishingElement") or core.is_architectural_covering(obj):
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
    elif cls == "IfcWall":
        physical = core._wall_geometry(m,obj)
        if representation=="Reference":
            if "interface_reference_parts" not in physical:raise ValueError("This wall has no declared full material reference")
            physical = {**physical,"parts":physical["interface_reference_parts"]}
        geometry = {"walls":[physical]}
    else:
        geometry = core.building_geometry(m)
    if representation=='Reference' and cls!='IfcWall':
        reference=core.reference_parts(key,m)
        if not reference:raise ValueError('The selected product has no nominal material stock')
        for family in ('details','services','walls','screens','slabs','pilotis'):
            if family in geometry:geometry[family]=[{**item,'parts':reference} if item['tag']==obj['tag'] else item for item in geometry[family]]
    body = BODIES[cls](obj,geometry)
    uniform_styles = {i:part["surface_appearance"] for i,part in enumerate(body.get("parts",())) if part.get("surface_appearance",{}).get("display_rgb")}
    surface_faces = {}
    for index,part in enumerate(body.get("parts",())):
        finish = part.get("surface_finish")
        if finish and finish.get("display_rgb") and 'csg' not in part:
            vertices = core.part_vertices(part)
            selected = {j for j,face in enumerate(_native_faces(core,part)) if core.surface_finishes().face_matches([vertices[k] for k in face],finish)}
            if selected:
                surface_faces[index] = selected
    profile, tag = body["profile"], obj["tag"]
    multipart = "parts" in body
    for part in body.get('parts',()):
        if 'csg' in part and any(name in part for name in ('outline','mesh','section','top_outline','holes','z0','z1','radius','profile','display_mesh')):raise ValueError('Analytic native parts must retain one authoritative CSG geometry')
    analytic = {i:_analytic_geometry_plan(part['csg'],f'solid{i}') for i,part in enumerate(body.get('parts',())) if 'csg' in part}
    frames = {i:importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.📐️frames.🐍️").coordinate_frame(core.part_vertices(part),_native_faces(core,part)) for i,part in enumerate(body.get("parts",())) if "mesh" in part}
    frames.update({i:plan.frame for i,plan in analytic.items()})
    mapped = any(frame["conditioned"] for frame in frames.values())
    centre = "centre" in profile
    material = obj.get("material")
    if cls in ("IfcFurnishingElement","IfcFooting","IfcRailing"):
        for part in body["parts"]:
            part["material"]=part.get("material",material["tag"] if material else None)
            if part["material"] is None or core.element(part["material"],m)["class"]!="IfcMaterial":
                raise ValueError("Owned construction parts require an explicit or product material")
            part.setdefault("role","owned_material")
    furnishing_quantity = core.quantities(key,m) if cls=="IfcFurnishingElement" else None
    layered_wall = cls=='IfcWall' and core._wall_junction_backend().path_material_layers(core,m,obj)
    constituents = _constituents(body["parts"],cls) if multipart and not layered_wall and (cls in FILLING_CLASSES or all("material" in p and "role" in p for p in body["parts"])) else []
    component_materials = list(dict.fromkeys(c["material"] for c in constituents))
    layers = None if constituents else material.get("layers") if material else None
    definitions = [(f"component_material{i}",core.element(tag,m)) for i,tag in enumerate(component_materials)] if constituents else [(f"layer_material{i}",core.element(layer["material"],m)) for i,layer in enumerate(layers)] if layers else [("material",material)] if material else []
    surface_properties = _surface_properties(definitions)
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
    if analytic:
        ownership.update(AnalyticGeometryAuthority={'value':'exact_native_stock_difference_operands','ifc_type':'IfcText'},AnalyticOperandAuthority={'value':json.dumps([{'part':i,'csg':body['parts'][i]['csg'],'operations':plan.operations} for i,plan in analytic.items()],sort_keys=True,separators=(',',':')),'ifc_type':'IfcText'})
        identities=[{'part':i,'surface_finish':part['surface_finish']} for i,part in enumerate(body['parts']) if 'csg' in part and 'surface_finish' in part]
        if identities:ownership['AnalyticSurfaceIdentity']={'value':json.dumps(identities,sort_keys=True,separators=(',',':')),'ifc_type':'IfcText'}
    psets = obj.get("psets", {})
    historical = psets.get("Pset_HistoricalEvidence",{})
    if not historical and obj.get("historical_evidence"):
        evidence = obj["historical_evidence"]
        values = {"TargetState":evidence["state"],"Status":evidence["status"],"SourceReferences":"; ".join(evidence["sources"]),"ReconstructionUncertainty":evidence["uncertainty"][language]}
        historical = {name:{"value":value,"ifc_type":"IfcText"} for name,value in values.items()}
    if property_set is not None and property_set not in psets:
        raise ValueError(f"No authored property set {property_set} for {key}")
    pset = property_set if property_set is not None else f"Pset_{cls[3:]}Common" if f"Pset_{cls[3:]}Common" in psets else next(iter(psets), None)
    props = [name for name in properties if pset and name in psets[pset]]
    kind = core.element(obj["type"], m) if obj.get("type") else None
    storey = core.element(obj["storey"], m)
    type_level = bool(props and spatial and kind and pset in (kind.get("psets") or {}))
    keys = ["element", "placement", "axis", "point"] + (["refdir"] if body["refdir"] else []) + ["shape", "representation", "solid", "profile"]
    if multipart:
        keys = [k for k in keys if k not in ("solid", "profile")]
        for i, part in enumerate(body["parts"]):
            if i in analytic:
                keys += analytic[i].keys
                continue
            if "mesh" in part:
                keys += [f"solid{i}",f"shell{i}"]
                if i in surface_faces:
                    keys += [f"surface_rgb{i}",f"surface_shading{i}",f"surface_style{i}"]
                for j in range(len(part["mesh"]["faces"])):
                    keys += [f"face{i}_{j}"]
                    if j in surface_faces.get(i,()):
                        keys += [f"surface_item{i}_{j}"]
                    keys += [f"bound{i}_{j}",f"loop{i}_{j}"]
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
    if mapped:
        keys += ["mapping_zero","mapping_origin"]
        for i in range(len(body["parts"])):keys += [f"mapped_representation{i}",f"mapping{i}",f"mapping_point{i}",f"mapping_target{i}",f"mapped{i}"]
    keys += (["profile_axis", "profile_point"] if centre else [])
    keys += [f"uniform_{name}{i}" for i in uniform_styles for name in ("rgb","shading","style","item")]
    keys += ["direction", "context", "world_axis", "world_point"]
    if constituents:
        for i in range(len(constituents)):
            keys += [f"aspect_representation{i}",f"aspect{i}"]
        keys += ["constituentset"]+[f"constituent{i}" for i in range(len(constituents))]+[f"component_material{i}" for i in range(len(component_materials))]
    elif layers:
        keys += ["layerset"] + [f"layer{i}" for i in range(len(layers))] + [f"layer_material{i}" for i in range(len(layers))]
    elif material:
        keys += ["material"]
    for index,surface_values in enumerate(surface_properties.values()):
        keys += [f"material_surface{index}_{name}" for name in surface_values]+[f"material_surface{index}"]
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
    attributes = [_str(obj["guid"],"guid"),"$",_str(obj["name"][language],"name"),"$",_str(obj["name"][language]) if obj.get("predefined_type")=="USERDEFINED" else "$",w.ref("placement","placement"),w.ref("shape","representation"),_str(tag,"tag")]
    has_predefined = any("PredefinedType" in level["adds"] for level in core.class_chain(cls,m))
    predefined = "$" if not has_predefined or cls in FILLING_CLASSES and spatial and kind else _enum(obj["predefined_type"],"predefined")
    if cls in ("IfcWindow","IfcDoor"):
        attributes += [(_real(body["overall_height"]),"height"),(_real(body["rough_width"]),"width"),predefined,"$","$"]
    elif has_predefined:
        attributes += [predefined]
    w.add("element",cls.upper(),*attributes,role="class")
    w.add("placement", "IFCLOCALPLACEMENT", "$", w.ref("axis"))
    w.add("axis", "IFCAXIS2PLACEMENT3D", w.ref("point"), "$", w.ref("refdir") if body["refdir"] else "$")
    w.add("point", "IFCCARTESIANPOINT", _coords(body["point"], "placement"))
    if body["refdir"]:
        w.add("refdir", "IFCDIRECTION", _coords(body["refdir"]))
    w.add("shape", "IFCPRODUCTDEFINITIONSHAPE", "$", "$", _tuple(w.ref("representation")))
    solids = [w.ref(f"mapped{i}" if mapped else f"solid{i}") for i in range(len(body["parts"]))] if multipart else [w.ref("solid")]
    representation_type = "MappedRepresentation" if mapped else "SolidModel" if analytic or multipart and any("mesh" in p for p in body["parts"]) else "SweptSolid"
    w.add("representation", "IFCSHAPEREPRESENTATION", w.ref("context"), _str("Reference" if cls=="IfcOpeningElement" else representation, "representation"), _str(representation_type, "representation"), _tuple(*solids))
    if multipart:
        for i, part in enumerate(body["parts"]):
            if i in analytic:
                analytic[i].write(w)
                continue
            if "mesh" in part:
                vertices,faces = frames[i]["vertices"],_native_faces(core,part)
                w.add(f"solid{i}","IFCFACETEDBREP",w.ref(f"shell{i}"))
                w.add(f"shell{i}","IFCCLOSEDSHELL",_tuple(*[w.ref(f"face{i}_{j}") for j in range(len(faces))]))
                finish = part.get("surface_finish")
                display = finish.get("display_rgb") if finish else None
                if i in surface_faces:
                    material_name = core.element(part["material"],m)["name"][language]
                    label = material_name+(" (illustrative RGB; original unmeasured)" if language=="en" else " (RGB nur Darstellung; Original ungemessen)")
                    w.add(f"surface_rgb{i}","IFCCOLOURRGB","$",*[_real(value) for value in display["value"]])
                    w.add(f"surface_shading{i}","IFCSURFACESTYLESHADING",w.ref(f"surface_rgb{i}"),_real(0))
                    w.add(f"surface_style{i}","IFCSURFACESTYLE",_str(label),_enum("POSITIVE"),_tuple(w.ref(f"surface_shading{i}")))
                for j,face in enumerate(faces):
                    w.add(f"face{i}_{j}","IFCFACE",_tuple(w.ref(f"bound{i}_{j}")))
                    if j in surface_faces.get(i,()):
                        w.add(f"surface_item{i}_{j}","IFCSTYLEDITEM",w.ref(f"face{i}_{j}"),_tuple(w.ref(f"surface_style{i}")),_str(label))
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
                    centre = _profile_axis(part)
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
                    centre = _void_axis(part,p['voids'][h])
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
                centre_point = _profile_axis(part)
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
    if mapped:
        w.add('mapping_zero','IFCCARTESIANPOINT',_coords([0.,0.,0.]))
        w.add('mapping_origin','IFCAXIS2PLACEMENT3D',w.ref('mapping_zero'),'$','$')
        for i,part in enumerate(body['parts']):
            frame=frames.get(i,{'origin':[0.,0.,0.],'scale':1.})
            w.add(f'mapped_representation{i}','IFCSHAPEREPRESENTATION',w.ref('context'),'$',_str('CSG' if i in analytic else 'Brep' if 'mesh' in part else 'SweptSolid'),_tuple(w.ref(f'solid{i}')))
            w.add(f'mapping{i}','IFCREPRESENTATIONMAP',w.ref('mapping_origin'),w.ref(f'mapped_representation{i}'))
            w.add(f'mapping_point{i}','IFCCARTESIANPOINT',_coords(frame['origin']))
            w.add(f'mapping_target{i}','IFCCARTESIANTRANSFORMATIONOPERATOR3D','$','$',w.ref(f'mapping_point{i}'),_real(1/frame['scale']),'$')
            w.add(f'mapped{i}','IFCMAPPEDITEM',w.ref(f'mapping{i}'),w.ref(f'mapping_target{i}'))
    for i,finish in uniform_styles.items():
        label=finish["colour_family"]+(" (illustrative RGB; original unmeasured)" if language=="en" else " (RGB nur Darstellung; Original ungemessen)")
        w.add(f"uniform_rgb{i}","IFCCOLOURRGB","$",*[_real(value) for value in finish["display_rgb"]["value"]])
        w.add(f"uniform_shading{i}","IFCSURFACESTYLESHADING",w.ref(f"uniform_rgb{i}"),_real(0))
        w.add(f"uniform_style{i}","IFCSURFACESTYLE",_str(label),_enum("POSITIVE"),_tuple(w.ref(f"uniform_shading{i}")))
        w.add(f"uniform_item{i}","IFCSTYLEDITEM",w.ref(f"solid{i}"),_tuple(w.ref(f"uniform_style{i}")),_str(label))
    w.add("direction", "IFCDIRECTION", _coords([0.0, 0.0, 1.0]))
    w.add("context", "IFCGEOMETRICREPRESENTATIONCONTEXT", "$", _str("Model"), "3", "1.E-05", w.ref("world_axis"), "$")
    w.add("world_axis", "IFCAXIS2PLACEMENT3D", w.ref("world_point"), "$", "$")
    w.add("world_point", "IFCCARTESIANPOINT", _coords([0.0, 0.0, 0.0]))
    if constituents:
        for i,component in enumerate(constituents):
            refs = [w.ref(f"mapped{j}" if mapped else f"solid{j}") for j in component["parts"]]
            w.add(f"aspect_representation{i}","IFCSHAPEREPRESENTATION",w.ref("context"),_str("Body"),_str("MappedRepresentation" if mapped else "SolidModel" if any(j in analytic or "mesh" in body["parts"][j] for j in component["parts"]) else "SweptSolid"),_tuple(*refs))
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
    for index,(material_key,surface_values) in enumerate(surface_properties.items()):
        for name,value in surface_values.items():
            w.add(f"material_surface{index}_{name}","IFCPROPERTYSINGLEVALUE",_str(name),"$",_value(value),"$")
        w.add(f"material_surface{index}","IFCMATERIALPROPERTIES",_str("Surface treatment" if language=="en" else "Oberflächenbehandlung"),_str("Existing substrate quantity; coating film and mass unmeasured" if language=="en" else "Bestehende Trägermenge; Beschichtungsfilm und Masse ungemessen"),_tuple(*[w.ref(f"material_surface{index}_{name}") for name in surface_values]),w.ref(material_key))
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

    if representation=='Body' and (cls=='IfcWall' and 'interface_reference_parts' in physical or cls in reference_classes and cls!='IfcWall'):
        result = _product_reference_excerpt(result,m,obj,language)
    if cls=="IfcOpeningElement":
        return _opening_host_excerpt(result,m,obj,language,host_spatial)
    return _ramp_assembly_excerpt(result,m,obj,language) if cls=="IfcRamp" and representation=='Body' else result


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
        if 'mesh' in part:
            points,native=core.part_vertices(part),core.part_faces(part)
            offset,face_offset,n=len(vertices),len(faces),len(points)//2
            vertices+=points
            for face in native[2:]+[native[1],native[0]]:
                faces += [[offset+face[0],offset+face[i],offset+face[i+1]] for i in range(1,len(face)-1)]
            side_end=face_offset+2*n
            top_end=side_end+n-2
            layouts.append({'bottom':list(range(offset,offset+n)),'top':list(range(offset+n,offset+2*n)),'side_faces':[face_offset,side_end],'top_faces':[side_end,top_end],'bottom_faces':[top_end,len(faces)]})
            continue
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
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
