"""🔗 Partition declared same-material monolithic joints while retaining every original full reference."""
from copy import deepcopy
import math

STRUCTURAL_CLASSES={"IfcBeam","IfcFooting","IfcSlab","IfcColumn","IfcBuildingElementProxy","IfcStair","IfcRailing"}


def _graph(api,model,obj):
    """🧭 Validate real product identity and every reachable directed owner before resolving physical stock."""
    if obj["class"] not in {"IfcBeam","IfcFooting","IfcBuildingElementProxy","IfcStair","IfcRailing"}:raise ValueError("A structural joint donor requires native multipart structural stock")
    visiting=set();visited=set()
    def visit(current):
        tag=current["tag"]
        if tag in visiting:raise ValueError("Structural joint ownership cycle")
        if tag in visited:return
        visiting.add(tag);owners=[]
        for joint in current["geometry"].get("structural_joints",()):
            if set(joint)!={"owner","relationship"} or joint["relationship"]!="monolithic_same_material":raise ValueError("A structural joint requires an explicit monolithic same-material relationship")
            owner=api._find(model,joint["owner"])
            if owner["class"] not in STRUCTURAL_CLASSES:raise ValueError("A structural joint requires an independent structural owner")
            if owner["tag"]==tag or owner["tag"] in owners:raise ValueError("Structural joint owners must be distinct independent products")
            owners.append(owner["tag"]);visit(owner)
        visiting.remove(tag);visited.add(tag)
    visit(obj)


def _overlap(api,a,b):
    """📦 Reject only strict empty actual native bounds, preserving every positive material band."""
    return all(x[0]<y[1] and y[0]<x[1] for x,y in zip(api._material_bounds(a),api._material_bounds(b)))


def _intersection(api,part,owner,cancel):
    """✂️ Keep actual convex native common material through exact first-party construction planes."""
    if not _overlap(api,part,owner):return []
    return api._reference_material_parts(part,[],api._native_material_cells(owner),cancel=cancel)


def _material(api,model,obj,part):
    """🧱 Resolve the actual occupied component material behind the native model interface."""
    tag=part.get("material",api._material(model,obj)["tag"])
    if not isinstance(tag,str) or api._find(model,tag)["class"]!="IfcMaterial":raise ValueError("A structural joint needs explicit occupied component materials")
    return tag


def _owned(api,model,owner,cancel):
    """🧱 Retain exact independent ownership through each native structural constructor."""
    if owner["class"]=="IfcSlab":return api._slab_geometry_entry(model,owner,exact=True)["parts"]
    if owner["class"]=="IfcStair":return api._stair_geometry(model,owner,cancel=cancel,exact=True)["parts"]
    if owner["geometry"].get("structural_joints"):return api._solid_geometry(model,owner,resolve_embedded=False,cancel=cancel,exact=True)["parts"]
    return api._interface_owner_parts(model,owner["tag"])


def _publish(api,part):
    """📤 Publish validated Float64 cells once, after all exact ownership and reference operations."""
    value=api._native_public_value(part)
    if "mesh" in value:value.update(api._polyhedron_part(value["mesh"]["vertices"],value["mesh"]["faces"]))
    return value


def partition(api,model,obj,parts,shared,progress=None,cancel=None,exact=False):
    """🏗️ Cut only declared common material; preserve foreign conflicts and owner-bound reference geometry."""
    _graph(api,model,obj)
    parts=[{**deepcopy(p),"material":_material(api,model,obj,p),"role":p.get("role","structural_member")} for p in parts]
    if cancel and cancel():raise InterruptedError("Structural joint construction cancelled")
    owners=[]
    for joint in obj["geometry"]["structural_joints"]:
        owner=api._find(model,joint["owner"])
        if progress:progress({"phase":"structural-owner","owner":owner["tag"],"completed":len(owners),"total":len(obj["geometry"]["structural_joints"])})
        owned=_owned(api,model,owner,cancel)
        owners.append((owner,owned))
    for index,(owner,owned) in enumerate(owners):
        for other,other_parts in owners[:index]:
            for part in parts:
                for a in owned:
                    if cancel and cancel():raise InterruptedError("Structural joint construction cancelled")
                    for first in _intersection(api,part,a,cancel):
                        if any(_intersection(api,first,b,cancel) for b in other_parts):raise ValueError("Declared structural joint owners must have exclusive physical material")
    current=deepcopy(parts);references=deepcopy(shared);issues=[]
    for owner_index,(owner,owned) in enumerate(owners):
        result=[]
        for index,part in enumerate(current):
            if cancel and cancel():raise InterruptedError("Structural joint construction cancelled")
            material=_material(api,model,obj,part);stock={**part,"material":material,"role":part.get("role","structural_member")};cutters=[]
            for owner_index_part,other in enumerate(owned):
                if cancel and cancel():raise InterruptedError("Structural joint construction cancelled")
                common=_intersection(api,stock,other,cancel)
                if not common:continue
                owner_material=_material(api,model,owner,other)
                if material!=owner_material:
                    volume=math.fsum(api._polyhedron_volume(p) for p in common)
                    issues.append({"code":"cross_material_structural_joint","owner":owner["tag"],"owner_guid":owner["guid"],"material":material,"owner_material":owner_material,"volume":volume,"part":index,"owner_part":owner_index_part})
                    continue
                cutters.append(other)
                existing=[p for p in references if p.get("owner")==owner["tag"]]
                for cell in common:
                    residual=api._reference_material_parts(cell,existing,cancel=cancel) if existing else [cell]
                    references.extend({**p,"owner":owner["tag"],"owner_guid":owner["guid"],"joint_host":obj["tag"],"role":"structural_joint_reference"} for p in residual)
            result.extend(api._reference_material_parts(stock,cutters,cancel=cancel) if cutters else [deepcopy(part)])
        current=result
        if progress:progress({"phase":"structural-joints","completed":owner_index+1,"total":len(owners)})
    if cancel and cancel():raise InterruptedError("Structural joint construction cancelled")
    if exact:return deepcopy(current),deepcopy(references),issues
    return [_publish(api,p) for p in current],[_publish(api,p) for p in references],issues


def _interface_graph(api,model,obj):
    """🧭 Reject cyclic independently occupied material reservations before querying their owners."""
    visiting=set();visited=set()
    def visit(current):
        tag=current["tag"]
        if tag in visiting:raise ValueError("Independent structural interface ownership cycle")
        if tag in visited:return
        visiting.add(tag)
        refs=[*current["geometry"].get("interface_owners",()),*(joint["owner"] for joint in current["geometry"].get("structural_joints",()))]
        if len(refs)!=len(set(refs)):raise ValueError("Independent structural interface owners must be distinct")
        for key in refs:visit(api._find(model,key))
        visiting.remove(tag);visited.add(tag)
    visit(obj)


def reserve_interfaces(api,model,obj,parts,progress=None,cancel=None):
    """🤝 Reserve explicitly declared independent occupied material in a receiving member of any material."""
    _interface_graph(api,model,obj)
    stocks=[{**deepcopy(p),"material":_material(api,model,obj,p),"role":p.get("role","structural_member")} for p in parts]
    owners=[]
    for key in obj["geometry"]["interface_owners"]:
        owner=api._find(model,key)
        cells=_owned(api,model,owner,cancel)
        owners.append((owner,cells))
    for index,(owner,cells) in enumerate(owners):
        for other,others in owners[:index]:
            for stock in stocks:
                for cell in cells:
                    for common in _intersection(api,stock,cell,cancel):
                        if any(_intersection(api,common,p,cancel) for p in others):raise ValueError("Independent structural interface owners must have exclusive physical material")
    shared=[]
    for index,(owner,cells) in enumerate(owners):
        if cancel and cancel():raise InterruptedError("Receiving material reservation cancelled")
        if progress:progress({"phase":"receiving-material","completed":index,"total":len(owners)})
        for stock in stocks:
            for cell in cells:
                for common in _intersection(api,stock,cell,cancel):
                    shared.append({**common,"owner":owner["tag"],"owner_guid":owner["guid"],"joint_host":obj["tag"],"material":_material(api,model,owner,cell),"role":"independent_material_reference"})
    cutters=[p for owner,cells in owners for p in cells]
    current=[cell for stock in stocks for cell in api._reference_material_parts(stock,cutters,cancel=cancel)]
    def normalized(items):
        """📐 Preserve exact authoritative meshes while exposing their finite axial extents to slab consumers."""
        result=[]
        for part in items:
            vertices=api.part_vertices(part)
            result.append({**part,"outline":api._convex_hull([[p[0],p[1]]for p in vertices]),"z0":min(p[2]for p in vertices),"z1":max(p[2]for p in vertices)})
        return result
    return normalized(current),normalized(shared)


def producers():
    """🧬 Bind graph, exact physical stock and Float64 publication identities to canonical native revisions."""
    return (partition,reserve_interfaces,_interface_graph,_graph,_overlap,_intersection,_material,_owned,_publish,producers)
