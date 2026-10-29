"""🏗️ Typed constructive facade sections and deterministic physical junction ownership."""
from copy import deepcopy
import hashlib

_CACHE={}


def section_rectangles(profile: dict) -> list:
    """📐 Decompose an orthogonal native L or U into disjoint occupied rectangles."""
    width,height=profile["width"]["value"],profile["height"]["value"]
    shape=profile["shape"]
    if width<=0 or height<=0:
        raise ValueError("A section requires positive outer dimensions")
    if shape=="rectangle":
        return [[0.,0.,width,height]]
    web,toe=profile["web"]["value"],profile["toe"]["value"]
    if not 0<web<width/(2 if shape=="U" else 1) or not 0<toe<height:
        raise ValueError("A constructive section requires a nonempty recess")
    out=[[0.,0.,width,toe],[0.,toe,web,height]]
    if shape=="U":
        out.append([width-web,toe,width,height])
    elif shape!="L":
        raise ValueError("Unknown constructive section")
    return out


def raw_parts(api,model: dict,obj: dict) -> list:
    """🧭 Place physical profiles from the exterior host face and actual aperture datums."""
    sweep=obj["geometry"]["section_sweep"]
    wall=api._find(model,sweep["wall"])
    opening=api._find(model,sweep["opening"])
    if opening["geometry"]["wall"]!=wall["tag"]:
        raise ValueError("A section sweep must use its own host aperture")
    a,b=wall["geometry"]["path"]
    length=api.math.dist(a,b)
    dx,dy=(b[0]-a[0])/length,(b[1]-a[1])/length
    side=wall["geometry"]["exterior_side"]
    inward=[side*dy,-side*dx]
    exterior=[a[0]-inward[0]*api._wall_thickness(model,wall)/2,a[1]-inward[1]*api._wall_thickness(model,wall)/2]
    opening_values=api._opening_values(model,opening)
    base=api._z(model,wall["geometry"]["base_level"])
    datums={"base":base,"top":api._top(model,wall["geometry"]["top_level"]),"sill":base+opening_values["sill"],"head":base+opening_values["head"]}
    def station(spec):
        return datums[spec["datum"]]+spec["offset"]["value"]
    z=station(sweep["from"])
    profile=deepcopy(api._find(model,obj["type"])["sweep_profile"])
    if "to" in sweep:
        profile["height"]["value"]=station(sweep["to"])-z
    start,end=sweep["start"]["value"],sweep["end"]["value"]
    if end<=start:
        raise ValueError("A section sweep needs a positive station interval")
    def point(t,d):
        return [exterior[0]+dx*t+inward[0]*d,exterior[1]+dy*t+inward[1]*d]
    depth=sweep["depth"]["value"]
    return [{"outline":[point(start,depth+x0),point(end,depth+x0),point(end,depth+x1),point(start,depth+x1)],"z0":z+y0,"z1":z+y1,"material":api._material(model,obj)["tag"],"role":sweep["role"]} for x0,y0,x1,y1 in section_rectangles(profile)]


def owned_parts(api,model: dict,obj: dict) -> list:
    """🧩 Give primary structure and ordered auxiliaries one invariant owner per junction."""
    classes={"IfcColumn","IfcSlab","IfcBeam","IfcOpeningElement","IfcCurtainWall"}
    elements=sorted((e for e in model["elements"] if e["class"] in classes),key=lambda e:e["tag"])
    dependencies=[[(e["tag"],e["class"],e.get("type"),e["geometry"],e["attributes"],e.get("material")) for e in elements],
                  [(t["tag"],t.get("profile"),t.get("sweep_profile"),t.get("material")) for t in model["types"]],
                  model["storeys"],model["materials"],model["building"],model["grid"],model["car"],model["spaces"]]
    key=hashlib.sha256(repr(dependencies).encode()).hexdigest()
    if key not in _CACHE:
        cutters=[]
        auxiliary=[]
        for item in model["elements"]:
            if "section_sweep" in item["geometry"]:
                auxiliary.append(item)
            elif item["class"]=="IfcColumn":
                cutters+=api.column_geometry(model,item)["parts"]
            elif item["class"]=="IfcSlab":
                cutters+=api._slab_geometry(model,item)["parts"]
            elif item["class"]=="IfcBeam":
                solid=api._solid_geometry(model,item)
                cutters+=solid["parts"]+solid["shared_parts"]
        result={}
        for item in sorted(auxiliary,key=lambda e:(e["geometry"]["section_sweep"]["priority"],e["tag"])):
            raw=raw_parts(api,model,item)
            parts=api._subtract_prisms(raw,cutters)
            result[item["tag"]]=parts
            cutters+=parts
        _CACHE[key]=result
        if len(_CACHE)>4:
            _CACHE.pop(next(iter(_CACHE)))
    return deepcopy(_CACHE[key][obj["tag"]])
