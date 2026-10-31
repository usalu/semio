"""🔗 Verify declared monolithic ownership without losing full reference geometry or foreign materials."""
import hashlib,importlib,json,math
from copy import deepcopy
from fractions import Fraction as Q
from pathlib import Path
from unittest.mock import patch
import numpy as np
from scipy.spatial import ConvexHull
from scipy.integrate import quad
from shapely.geometry import Polygon
from shapely.ops import unary_union
from jsonschema import Draft202012Validator

A=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
D=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📦️disjoint")
CASE=json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))


def model(row,declared=True):
    """🌐 Declare physical parts and one explicit owner independently of lexical names or class priority."""
    raw={k:deepcopy(v) for k,v in row["donor"].items() if k!="voids"}
    m=D.model({"id":row["id"]},[{**raw,"material":"MAT-TEST","role":"structural_member"}]);m.update(car={},grid={})
    donor=m["elements"][0];donor.update(tag="DONOR",guid=A.global_id("DONOR"),**{"class":row["donor_class"]})
    donor["geometry"]["kind"]="structural_member";donor["metadata"]["domains"]=["architecture","structure"]
    if row.get("implicit_material"):
        donor["geometry"]["parts"][0].pop("material");donor["geometry"]["parts"][0].pop("role")
    if declared:donor["geometry"]["structural_joints"]=[{"owner":"OWNER","relationship":CASE["relationship"]}]
    if row.get("embedded"):donor["geometry"]["embedded_slabs"]=["OWNER"]
    owner=deepcopy(donor);owner.update(tag="OWNER",guid=A.global_id("OWNER"),**{"class":row["owner_class"]});owner["geometry"].pop("structural_joints",None);owner["geometry"].pop("embedded_slabs",None)
    raw=deepcopy(row["owner"])
    if row["owner_class"]=="IfcSlab":
        q=lambda value:{"value":value,"unit":"m","basis":"assumed"}
        region={"tag":"REGION","outline":raw["outline"],"thickness":q(raw["z1"]-raw["z0"]),"top_offset":q(raw["z1"]),"material":"MAT-TEST"}
        if row.get("embedded"):region["member"]="DONOR"
        owner["geometry"]={"outline":raw["outline"],"level":0,"top_offset":q(raw["z1"]),"regions":[region],"voids":[{"tag":"HOLE"+str(i),"outline":ring} for i,ring in enumerate(raw.get("voids",[]))]}
    else:owner["geometry"]={"kind":"structural_member","base_level":0,"parts":[{**raw,"material":"MAT-TEST","role":"structural_member"}]}
    m["elements"].append(owner)
    if row.get("second_owner"):
        second=deepcopy(owner);second.update(tag="OWNER-SECOND",guid=A.global_id("OWNER-SECOND"),**{"class":"IfcBuildingElementProxy"})
        second["geometry"]={"kind":"structural_member","base_level":0,"parts":[{**deepcopy(row["second_owner"]),"material":"MAT-TEST","role":"structural_member"}]};m["elements"].append(second)
        if row.get("second_owner_first_joint"):second["geometry"]["structural_joints"]=[{"owner":"OWNER","relationship":CASE["relationship"]}]
        if declared:donor["geometry"]["structural_joints"].append({"owner":"OWNER-SECOND","relationship":CASE["relationship"]})
    return m


def geometry(m,tag):
    """🧱 Read only actual physical owner cells through native model geometry."""
    obj=A._find(m,tag)
    return A._slab_geometry_entry(m,obj) if obj["class"]=="IfcSlab" else A.solid_geometry(tag,m)


def sections(part,z):
    """🔬 Intersect actual boundary edges independently, retaining only the verified convex section."""
    if part.get("holes") and not any(k in part for k in ("mesh","section","top_outline")):
        return Polygon(part["outline"],part["holes"]) if part["z0"]<z<part["z1"] else Polygon()
    vertices=[[Q(v) for v in point] for point in A.part_vertices(part)];points=set();height=Q(float(z))
    edges={tuple(sorted((a,b))) for face in A.part_faces(part) for a,b in zip(face,face[1:]+face[:1])}
    for i,j in edges:
        a,b=vertices[i],vertices[j]
        if a[2]==height:points.add(tuple(a[:2]))
        if (a[2]<height<b[2]) or (b[2]<height<a[2]):points.add(tuple(a[k]+(height-a[2])/(b[2]-a[2])*(b[k]-a[k]) for k in range(2)))
    ordered=sorted(points)
    if len(ordered)<3:return Polygon()
    def cross(a,b,c):
        """📐 Determine true section orientation using the published Float64 values exactly."""
        return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
    chains=[]
    for sequence in (ordered,list(reversed(ordered))):
        chain=[]
        for point in sequence:
            while len(chain)>1 and cross(chain[-2],chain[-1],point)<=0:chain.pop()
            chain.append(point)
        chains.append(chain[:-1])
    ring=chains[0]+chains[1]
    if len(ring)<3:return Polygon()
    return Polygon([(float(x),float(y)) for x,y in ring])



def volume(parts):
    """⚖️ Integrate independent GEOS unions, resolving sloping arrangement changes between source vertex datums."""
    levels=sorted({v[2] for p in parts for v in A.part_vertices(p)})
    def vertical(part):
        """📐 Identify only exact vertical prisms for the finite fixed-quadrature path."""
        vertices=A.part_vertices(part);lo=min(p[2]for p in vertices);hi=max(p[2]for p in vertices);groups={}
        for x,y,z in vertices:groups.setdefault((x,y),set()).add(z)
        return all(values=={lo,hi}for values in groups.values())
    def area(height):
        """🔬 Measure the occupied cross-section using GEOS rather than the native material constructor."""
        return unary_union([sections(p,height)for p in parts]).area
    if all(vertical(part)for part in parts):
        nodes,weights=np.polynomial.legendre.leggauss(3)
        return math.fsum((hi-lo)/2*math.fsum(w*area((lo+hi)/2+t*(hi-lo)/2)for t,w in zip(nodes,weights))for lo,hi in zip(levels,levels[1:]))
    return math.fsum(quad(area,float(lo),float(hi),epsabs=1e-12/max(1,len(levels)),epsrel=2e-12,limit=200)[0]for lo,hi in zip(levels,levels[1:]))


def digest(parts):
    """🪪 Preserve full authored geometry exactly while physical ownership changes."""
    return hashlib.sha256(json.dumps(parts,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()


def test_declared_joint_schema_and_actual_exclusive_material_preserve_reference_and_union():
    """🏗️ Confirm source-bound body ownership, real openings, full dimensions and two-nanometre common material."""
    Draft202012Validator(json.loads(Path(__file__).with_name("🧬️.schema.json").read_text(encoding="utf-8"))).validate(CASE)
    schema=json.loads((Path(A.__file__).parent/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"));validator=Draft202012Validator({"$defs":schema["$defs"],"$ref":"#/$defs/geometry"})
    for row in CASE["cases"]:
        m=model(row);saved=deepcopy(m);original=geometry(model(row,False),"DONOR");owner={"parts":[p for joint in A._find(m,"DONOR")["geometry"]["structural_joints"] for p in geometry(m,joint["owner"])["parts"]]};actual=geometry(m,"DONOR")
        errors=list(validator.iter_errors(A._find(m,"DONOR")["geometry"]));assert not errors,[e.message for e in errors]
        before=volume(original["parts"]+owner["parts"]);after=volume(actual["parts"]+owner["parts"])
        assert math.isclose(before,row["union_volume"],abs_tol=CASE["comparison"]["volume"]),(row["id"],before)
        original_common=volume(original["parts"])+volume(owner["parts"])-before
        assert math.isclose(original_common,row["common_volume"],abs_tol=CASE["comparison"]["volume"]),(row["id"],original_common,row["common_volume"])
        assert math.isclose(after,before,abs_tol=CASE["comparison"]["volume"]),row["id"]
        common=volume(actual["parts"])+volume(owner["parts"])-after
        assert abs(common)<=CASE["comparison"]["volume"],(row["id"],common)
        assert digest(original["full_parts"])==digest(actual["full_parts"])
        assert math.isclose(volume(actual["parts"]+actual["shared_parts"]),volume(actual["full_parts"]),abs_tol=CASE["comparison"]["volume"])
        if row["common_volume"]>0:assert any(p.get("owner")=="OWNER" and p.get("owner_guid")==A.global_id("OWNER") for p in actual["shared_parts"])
        for p in actual["parts"]:
            vertices=A.part_vertices(p);faces=A.part_faces(p);native=A._polyhedron_part(vertices,faces)
            hull=ConvexHull(np.asarray(vertices,float)).volume;assert hull>0
            assert math.isclose(A._polyhedron_volume(native),hull,abs_tol=CASE["comparison"]["volume"])
        q=A.quantities("DONOR",m);assert math.isclose(q["exclusive_volume"],volume(actual["parts"]),abs_tol=1e-9)
        body=A.represent("DONOR","body",m);assert body["derived"]["representation_scope"]=="exclusive_owned_material"
        assert math.isclose(body["derived"]["representation_volume"],q["exclusive_volume"],abs_tol=1e-9)
        assert m==saved


def test_cross_material_intersections_are_retained_and_reported_without_ownership_waivers():
    """🚧 Expose unresolved render/core or other-material conflicts rather than treating them as monolithic stock."""
    m=model(CASE["cases"][0]);other=deepcopy(m["materials"][0]);other.update(tag="MAT-OTHER",guid=A.global_id("MAT-OTHER"));m["materials"].append(other)
    owner=A._find(m,"OWNER");owner["material"]="MAT-OTHER"
    for region in owner["geometry"]["regions"]:region["material"]="MAT-OTHER"
    actual=geometry(m,"DONOR");original=geometry(model(CASE["cases"][0],False),"DONOR")
    assert math.isclose(volume(actual["parts"]),volume(original["parts"]),abs_tol=CASE["comparison"]["volume"])
    issues=actual["construction_issues"];assert issues
    assert all(i["code"]=="cross_material_structural_joint" and i["owner"]=="OWNER" and i["material"]=="MAT-TEST" and i["owner_material"]=="MAT-OTHER" and i["volume"]>0 for i in issues)
    assert not actual["shared_parts"]


def test_joint_graph_validation_progress_cancellation_and_producer_isolation():
    """🛑 Keep declared native joints acyclic, independently owned, cancellable and revision-bound."""
    for invalid in CASE["invalid"]:
        m=model(CASE["cases"][6]);donor=A._find(m,"DONOR")
        if invalid=="cycle":A._find(m,"OWNER")["geometry"]["structural_joints"]=[{"owner":"DONOR","relationship":CASE["relationship"]}]
        elif invalid=="self-owner":donor["geometry"]["structural_joints"][0]["owner"]="DONOR"
        elif invalid=="duplicate-owner":donor["geometry"]["structural_joints"]*=2
        else:donor["geometry"]["structural_joints"][0]["owner"]="MISSING"
        try:geometry(m,"DONOR")
        except ValueError:pass
        else:raise AssertionError("Invalid joint graph accepted: "+invalid)
    m=model(CASE["cases"][0]);events=[]
    try:A.solid_geometry("DONOR",m,progress=events.append,cancel=lambda:True)
    except InterruptedError:pass
    else:raise AssertionError("Declared joint cancellation ignored")
    actual=A.solid_geometry("DONOR",m,progress=events.append);assert events and actual["parts"]
    actual["parts"][0]["mesh"]["vertices"][0][0]=99
    assert A.solid_geometry("DONOR",m)["parts"][0]["mesh"]["vertices"][0][0]!=99
    backend=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️structure.🔗️joints.🐍️")
    first=A.geometry_revision(m)
    with patch.object(backend,"partition",side_effect=backend.partition):assert A.geometry_revision(m)!=first


def test_nearly_collinear_actual_float_section_retains_positive_area():
    """🔬 Preserve an exact thin section without Qhull perturbation, vertex filtering or area thresholds."""
    row=CASE["section_control"];expected=Polygon(row["outline"]);actual=sections(row,row["height"])
    assert expected.area>0 and actual.area>0
    assert actual.equals(expected) and actual.area==expected.area


def test_declared_column_reserves_retain_tiny_bearings_and_real_bore_air():
    """🏛️ Compare exact declared column joints with GEOS, including positive sub-threshold height and bore air."""
    for row in CASE["column_controls"]:
        m=model(CASE["cases"][0],False);owner=A._find(m,"OWNER");owner["class"]="IfcColumn";donor=A._find(m,"DONOR")
        raw={**deepcopy(row["donor"]),"material":"MAT-TEST","role":"structural_member"};column=deepcopy(row["column"])
        with patch.object(A,"column_geometry",return_value={"parts":[column]}):owned,shared,voids=A._column_partition(m,donor,[raw],["OWNER"])
        occupied=Polygon(column["outline"],column.get("holes",[]));profile=Polygon(raw["outline"]);common=profile.intersection(occupied).area*(raw["z1"]-raw["z0"])
        assert common>0 and shared,row["id"]
        assert math.isclose(volume(shared),common,rel_tol=1e-12,abs_tol=0),row["id"]
        full_column=Polygon(column["outline"]);expected=profile.difference(full_column).area*(raw["z1"]-raw["z0"])
        assert math.isclose(volume(owned),expected,rel_tol=1e-12,abs_tol=0),row["id"]
        assert all(p.get("owner_guid")==owner["guid"] for p in shared),row["id"]
        assert bool(voids)==bool(column.get("holes")),row["id"]


def test_native_column_joint_cells_preserve_subsequent_explicit_host_voids():
    """🕳️ Subtract declared host apertures from actual native joint cells rather than their projected envelopes."""
    row=CASE["column_controls"][1];m=model(CASE["cases"][0],False);owner=A._find(m,"OWNER");owner["class"]="IfcColumn";donor=A._find(m,"DONOR")
    donor["geometry"].update(parts=[{**deepcopy(row["donor"]),"material":"MAT-TEST","role":"structural_member"}],joint_owners=["OWNER"],voids=[row["additional_void"]])
    with patch.object(A,"column_geometry",return_value={"parts":[row["column"]]}):actual=A._solid_geometry(m,donor)
    outer=Polygon(row["donor"]["outline"]);column=Polygon(row["column"]["outline"]);opening=Polygon(row["additional_void"]["outline"])
    assert math.isclose(volume(actual["parts"]),outer.difference(column).difference(opening).area,rel_tol=1e-12)
    bore=Polygon(row["column"]["holes"][0]);assert math.isclose(volume(actual["full_parts"]),outer.difference(bore).difference(opening).area,rel_tol=1e-12)


def test_explicit_receiving_slab_material_interface_preserves_reference_and_actual_union():
    """🧱 Reserve the actual independent footing in a different-material floor with one physical owner."""
    row=CASE["receiving_interface"];m=model(CASE["cases"][0],False);floor=A._find(m,"OWNER");footing=A._find(m,"DONOR")
    other=deepcopy(m["materials"][0]);other.update(tag=row["owner_material"],guid=A.global_id(row["owner_material"]));m["materials"].append(other)
    footing["material"]=other["tag"];footing["geometry"]["parts"][0].update(outline=row["donor_outline"],material=other["tag"])
    original=A._slab_geometry_entry(m,floor);before=volume(original["parts"]+A._solid_geometry(m,footing)["parts"])
    floor["geometry"]["interface_owners"]=["DONOR"];actual=A._slab_geometry_entry(m,floor);private=A._slab_geometry_entry(m,floor,exact=True)
    assert math.isclose(volume(actual["full_parts"]),row["full_volume"],abs_tol=1e-14)
    assert math.isclose(volume(actual["parts"]),row["owned_volume"],abs_tol=1e-14)
    assert math.isclose(volume(actual["shared_parts"]),row["shared_volume"],abs_tol=1e-14)
    assert digest(original["full_parts"])==digest(actual["full_parts"])
    assert all(p.get("owner")=="DONOR" and p.get("owner_guid")==footing["guid"] and p.get("material")==other["tag"] for p in actual["shared_parts"])
    assert math.isclose(before,volume(actual["parts"]+A._solid_geometry(m,footing)["parts"]),abs_tol=1e-14)
    assert any(isinstance(v,Q) for p in private["parts"] for point in A.part_vertices(p) for v in point)
    quantities=A.quantities("OWNER",m);assert math.isclose(quantities["exclusive_volume"],row["owned_volume"],abs_tol=1e-14)
    assert math.isclose(sum(p["volume"]for p in quantities["material_quantities"]),row["owned_volume"],abs_tol=1e-14)


def test_native_stair_joints_partition_structural_channels_and_preserve_finished_treads():
    """🪜 Keep actual owned stair structure, original full geometry and untouched wearing channels at a slab bearing."""
    row=CASE["native_stair_joint"];m=model(CASE["cases"][0],False);stair=A._find(m,"DONOR");stair["class"]="IfcStair"
    stair["geometry"]={"kind":"compensated_u","from_level":0,"to_level":1,"centre":[0,0],"radius":1,"inner_radius":0.1,"step_count":1,"step_parts":[row["wearing"]],"landing_parts":[],"structural_parts":[row["structural"]],"nosing_parts":[],"structural_joints":[{"owner":"OWNER","relationship":CASE["relationship"]}]}
    floor=A._find(m,"OWNER");floor["geometry"]["top_offset"]["value"]=0.5;floor["geometry"]["regions"][0]["top_offset"]["value"]=0.5;floor["geometry"]["regions"][0]["thickness"]["value"]=0.25
    actual=A._stair_geometry(m,stair)
    assert math.isclose(volume(actual["structural_parts"]),row["structural_owned_volume"],abs_tol=1e-12)
    assert math.isclose(volume(actual["parts"]),row["owned_volume"],abs_tol=1e-12)
    assert math.isclose(volume(actual["full_parts"]),row["full_volume"],abs_tol=1e-12)
    assert actual["steps"][0]["outline"]==row["wearing"]["outline"]
    assert math.isclose(volume(actual["finish_parts"]),row["full_volume"]-4,abs_tol=1e-12)
    assert math.isclose(A.quantities("DONOR",m)["volume"],row["owned_volume"],abs_tol=1e-12)
    assert actual["shared_parts"] and all(p["owner"]=="OWNER"for p in actual["shared_parts"])


def test_ramp_physical_representations_exclude_borrowed_stock_and_preserve_owned_assembly_children():
    """🛝️ Measure exported body profiles independently while source full ramp stock remains a separate reference."""
    row=CASE["ramp_representation"]
    for cls,expected in (("IfcRampFlight",row["flight_volume"]),("IfcRamp",row["assembly_volume"])):
        m=model(CASE["cases"][0],False);obj=A._find(m,"DONOR");obj["class"]=cls
        children=[deepcopy(row["child"])]if cls=="IfcRamp"else[]
        actual={"parts":[row["owned"]],"full_parts":[row["full"],*children],"child_parts":children,"flights":[{"tag":"DONOR","start":[0,0],"end":[2,0],"z0":0,"z1":1}]}
        with patch.object(A,"_ramp_geometry",return_value=actual),patch.object(A,"quantities",return_value={"volume":2+len(children),"exclusive_volume":1}):body=A.represent("DONOR","body",m)
        measured=math.fsum(Polygon(p["profile"]["points"]).area*p["depth"]for p in body["primitives"])
        assert math.isclose(measured,expected,abs_tol=1e-12),(cls,measured)
        assert math.isclose(body["derived"]["representation_volume"],expected,abs_tol=1e-12)
        assert {p["owner"]for p in body["primitives"]}==({"DONOR","CHILD"}if children else{"DONOR"})
        assert actual["full_parts"][0]==row["full"]


def test_independent_union_integration_resolves_crossing_slope_arrangement_events():
    """🔬 Resolve union topology changes between original vertex datums with the known three-quarter unit volume."""
    row=CASE["union_arrangement_control"]
    assert math.isclose(volume(row["parts"]),row["union_volume"],abs_tol=1e-12)


def test_stair_constructor_changes_invalidate_canonical_geometry_revision():
    """🧬 Keep stair structural channel ownership bound to the same canonical geometry memo as its parts."""
    m=model(CASE["cases"][0],False);before=A.geometry_revision(m)
    with patch.object(A,"_stair_geometry",side_effect=A._stair_geometry):assert A.geometry_revision(m)!=before


def test_ramp_interface_resolves_actual_exact_slab_material_and_preserves_openings_and_prior_owners():
    """🛝️ Use only actual owned slab stock for a ramp bearing, retaining its opening and independent footing air."""
    row=CASE["slab_interface_control"];m=model(CASE["cases"][0],False);floor=A._find(m,"OWNER");floor["geometry"].update(voids=[{"tag":"OPENING","outline":row["hole"]}],interface_owners=["DONOR"])
    A._find(m,"DONOR")["geometry"]["parts"][0]["outline"]=row["independent_footing"]
    actual=A._interface_reference_parts(m,"OWNER");expected=Polygon([[0,0],[2,0],[2,2],[0,2]],[row["hole"]]).difference(Polygon(row["independent_footing"])).area*0.08
    assert math.isclose(expected,row["occupied_volume"],abs_tol=1e-12)
    assert math.isclose(volume(actual),expected,abs_tol=1e-12)
    assert any(isinstance(v,Q)for p in actual for point in A.part_vertices(p)for v in point)


def test_explicit_ramp_reference_api_retains_uncut_original_stock_and_isolates_output():
    """🧭 Expose original source ramp section independently of owned interfaces, assembly aliases and mutable callers."""
    row=CASE["ramp_reference_control"];m=model(CASE["cases"][0],False);obj=A._find(m,"DONOR");obj["class"]="IfcRampFlight";q=lambda v:{"value":v,"unit":"m","basis":"assumed"}
    obj["geometry"]={"path":row["path"],"structural_offsets":[q(v)for v in row["offsets"]],"from_level":row["from_level"],"to_level":row["to_level"],"portion":row["portion"],"finish_thickness":q(row["finish"]),"finish_direction":"vertical","thickness":q(row["depth"]),"interface_owners":["OWNER"]}
    actual=A.reference_parts("DONOR",m);independent=math.fsum(ConvexHull(np.asarray(A.part_vertices(p),float)).volume for p in actual)
    assert math.isclose(independent,row["volume"],abs_tol=1e-12)
    saved=digest(actual);obj["geometry"]["interface_owners"]=[];assert digest(A.reference_parts("DONOR",m))==saved
    actual[0]["mesh"]["vertices"][0][0]=999
    assert digest(A.reference_parts("DONOR",m))==saved


def test_procedural_guard_and_stringer_references_use_original_source_stock():
    """🧭 Bind procedural structural references to their source constructors instead of cut bodies or missing parts."""
    for kind,helper in (("ramp_stringer","_ramp_stringer_reference_stock"),("ramp_guard","_guard_reference_stocks")):
        m=model(CASE["cases"][0],False);obj=A._find(m,"DONOR");obj["class"]="IfcRailing";stock=deepcopy(obj["geometry"]["parts"][0]);obj["geometry"]={"interface_construction":{"kind":kind}}
        with patch.object(A,helper,return_value=stock if kind=="ramp_stringer"else[stock]):actual=A.reference_parts("DONOR",m)
        assert digest(actual)==digest([stock])
        assert math.isclose(volume(actual),CASE["cases"][0]["union_volume"],abs_tol=1e-12)
        actual[0]["outline"][0][0]=999;assert stock["outline"][0][0]!=999
