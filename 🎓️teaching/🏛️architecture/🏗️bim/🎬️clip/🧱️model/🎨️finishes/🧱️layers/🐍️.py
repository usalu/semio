"""🧱 Resolve editable shared floor layers from original material allocations and exclusive host reserves."""
from copy import deepcopy
from fractions import Fraction as Q
import math


def _q(value):
    """🔢 Retain resolved native binary operands until one final publication."""
    return value if isinstance(value,Q) else Q(value)


def _bounds(ring):
    """📦 Reject only provably empty planar bounds."""
    return tuple(min(p[k] for p in ring) for k in range(2))+tuple(max(p[k] for p in ring) for k in range(2))


def _overlap(a,b):
    """🤝 Retain all positive intersections, including nanometre bands."""
    return all(a[k+2]>b[k] and b[k+2]>a[k] for k in range(2))


def _emit(part,base,exact=False):
    """📤 Retain local exact cells until the public model boundary publishes their coordinates."""
    value=deepcopy(part)
    if "mesh" in value:
        value["mesh"]["vertices"]=[[*p[:2],p[2]-base]for p in value["mesh"]["vertices"]]
        value["z0"]=min(p[2]for p in value["mesh"]["vertices"]);value["z1"]=max(p[2]for p in value["mesh"]["vertices"])
    else:value["z0"],value["z1"]=value["z0"]-base,value["z1"]-base
    if exact:return value
    def publish(item):
        """🔓 Publish only scalar coordinates while preserving the native schema."""
        if isinstance(item,Q):return float(item)
        if isinstance(item,dict):return {key:publish(v)for key,v in item.items()}
        if isinstance(item,list):return [publish(v)for v in item]
        return item
    return publish(value)


def _cut(api,parts,cutters,progress=None,cancel=None):
    """🧩 Subtract actual closed native material without replacing it by a planar envelope."""
    return [cell for part in parts for cell in api._reference_material_parts(part,cutters,progress=progress,cancel=cancel)]if cutters else deepcopy(parts)


def _downward_shadows(api,stock,cutters):
    """🌤️ Bound actual vertical accessibility by the upper and vertical native supporting faces."""
    if not cutters:return []
    result=[];backend=api.surface_finishes();box=api._material_bounds(stock);stocks=api._native_material_cells(stock)
    def shadow(source,cell):
        """🔦 Retain the exact portion of one stock cell below a real native upper boundary."""
        clipped=backend.clip_stock(source,[],exact=True);vertices=[[_q(v)for v in point]for point in api.part_vertices(cell)]
        for face in api.part_faces(cell):
            anchor=vertices[face[0]]
            for i in range(1,len(face)-1):
                u,v=[[vertices[index][k]-anchor[k]for k in range(3)]for index in (face[i],face[i+1])]
                normal=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
                if any(normal):break
            else:raise ValueError("Bound layer owners require nondegenerate native faces")
            if normal[2]<0:continue
            clipped=backend.clip_stock(clipped,[{"normal":normal,"offset":sum(n*p for n,p in zip(normal,anchor)),"points":[anchor,vertices[face[i]],vertices[face[i+1]]]}])
            if clipped is None:break
        return clipped
    for cutter in cutters:
        other=api._material_bounds(cutter)
        if other[2][1]<=box[2][0]or not all(a[0]<b[1]and b[0]<a[1]for a,b in zip(box[:2],other[:2])):continue
        for cell in api._native_material_cells(cutter):
            for source in stocks:
                clipped=shadow(source,cell)
                if clipped is not None:result.append({"mesh":clipped._exact_mesh,"material":stock["material"],"role":stock["role"]})
    return result


def dependencies(model,material,layer):
    """⚡ Identify every shared-layer host and explicitly bound dependent physical product."""
    return [o["tag"] for o in model["elements"] if (o["class"]=="IfcSlab" and o.get("material")==material) or (o.get("geometry",{}).get("layer_binding",{}).get("material")==material and o["geometry"]["layer_binding"]["layer"]==layer)]


def parts(api,model,obj,progress=None,cancel=None,exact=False):
    """🧱 Construct occupied bound layers from exact source hosts while original parts remain Reference."""
    g=obj["geometry"];binding=g["layer_binding"];layers=api._find(model,binding["material"])["layers"]
    layer=next((row for row in layers if row["id"]==binding["layer"]),None)
    if layer is None:raise ValueError("Bound flooring requires an existing shared material layer")
    value=layer["thickness"]["value"];reference=binding["reference_thickness"]["value"]
    depth=obj["attributes"]["finish_thickness"]["value"]
    if not all(math.isfinite(v) and v>0 for v in (value,reference,depth)):raise ValueError("Bound floor layers require positive finite thickness")
    base=_q(api._z(model,g["base_level"]));default=(_q(depth),obj["material"])
    for raw in g["parts"]:
        if any(k in raw for k in ("mesh","section","top_outline")) or raw.get("holes"):raise ValueError("Bound flooring requires explicit native vertical reference prisms")
    hosts=[o for o in model["elements"] if o["class"]=="IfcSlab" and o.get("material")==binding["material"]]
    total=sum(sum(v["owner"]==obj["tag"] for v in h["geometry"].get("finish_voids",())) for h in hosts)
    result=[];completed=0;owners={};datums=set()
    for host in hosts:
        lowest=min(api._round(api._z(model,host["geometry"]["level"])+row["top_offset"]["value"])for row in host["geometry"]["regions"])
        datums.add(_q(api._round(lowest+reference)))
    if any(_q(api._round(float(base)+raw["z1"]))not in datums for raw in g["parts"]if raw.get("role")=="wearing"):
        raise ValueError("Bound floor Reference is not exactly occupied at its declared surface datum")
    for host in hosts:
        outline=api._expand(model,host["geometry"]["outline"]);voids=[api._expand(model,row["outline"])for row in api._slab_voids(model,host)]
        regions=[(ring,_q(api._round(api._z(model,host["geometry"]["level"])+row["top_offset"]["value"])))for row in host["geometry"]["regions"]for ring in api._slab_surface(api._expand(model,row["outline"]),outline,voids,exact=True)]
        top=_q(api._round(min(float(level)for ring,level in regions)+value));reference_top=_q(api._round(min(float(level)for ring,level in regions)+reference))
        host_parts=[];processed=[];rows=[row for row in host["geometry"].get("finish_voids",())if row["owner"]==obj["tag"]]
        joint_tags=list(dict.fromkeys(tag for row in rows for tag in row.get("joint_owners",())))
        for tag in joint_tags:
            if tag not in owners:owners[tag]=api._solid_geometry(model,api._find(model,tag),resolve_embedded=False,exact=True)["parts"]
        cutters=[part for tag in joint_tags for part in owners[tag]]
        for reserve in rows:
            if cancel and cancel():raise InterruptedError("Bound flooring construction cancelled")
            reserve_ring=api._expand(model,reserve["outline"]);stock=[];wearing_stock=[];reserve_bounds=_bounds(reserve_ring)
            previous=[ring for ring,bounds in processed if _overlap(reserve_bounds,bounds)]
            for region,low in regions:
                if not _overlap(_bounds(region),_bounds(reserve_ring)):continue
                rings=api._slab_surface(reserve_ring,region,previous,exact=True)
                if not rings:continue
                if top<=low:raise ValueError("Edited floor layer must remain above its actual structural cap")
                capacity=reference_top-low
                bands=sorted((v for v in binding.get("wearing_bands",()) if capacity<=_q(v["maximum_reference_capacity"]["value"])),key=lambda v:v["maximum_reference_capacity"]["value"])
                wearing,material=(_q(bands[0]["depth"]["value"]),bands[0]["material"]) if bands else default
                split=top-wearing
                if split<low:raise ValueError("Edited layer cannot fit its declared wearing depth above the structural cap")
                for ring in rings:
                    if split>low:stock.append({"outline":ring,"z0":low,"z1":split,"material":binding["support_material"],"role":binding["support_role"]})
                    wearing_stock.append({"outline":ring,"z0":split,"z1":top,"material":material,"role":"wearing"})
            occupied=_cut(api,stock,cutters,progress,cancel)
            for wearing in wearing_stock:
                shadows=_downward_shadows(api,wearing,cutters);retained=_cut(api,[wearing],shadows,progress,cancel)
                occupied.extend(retained)
                if shadows:
                    detached=_cut(api,[wearing],[*cutters,*retained],progress,cancel)
                    occupied.extend({**part,"material":binding["support_material"],"role":binding["support_role"]}for part in detached)
            host_parts.extend(occupied);processed.append((reserve_ring,reserve_bounds));completed+=1
            if progress:progress({"completed":completed,"total":total})
        if host["geometry"].get("column_reserves")and host_parts:
            host_parts,_,_=api._column_partition(model,host,host_parts,host["geometry"]["column_reserves"],require_contact=False,exact=True)
        result.extend(_emit(part,base,exact)for part in host_parts)
    if not result:raise ValueError("Bound flooring has no actual host reserve")
    return result


def producers():
    """🧬 Bind every physical construction and dependency producer to the canonical native memo."""
    return (parts,dependencies,_q,_bounds,_overlap,_emit,_cut,_downward_shadows,producers)
_BIM_LOADED_MODULE_CODE = __import__("sys")._getframe().f_code
