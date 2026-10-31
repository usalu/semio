"""🧱 Verify editable shared layers against original exclusive flooring and fixed structural joints."""
import importlib,json,math
from copy import deepcopy
from pathlib import Path
from unittest.mock import patch
from fractions import Fraction
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
from shapely.ops import unary_union
from jsonschema import Draft202012Validator

A=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
CASE=json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))


def model():
    """🌐 Declare two actual shared-layer hosts, three finishes and a fixed raised structural joint."""
    def q(value):return {"value":value,"unit":"m","basis":"assumed","source":"Neutral layer event control"}
    def obj(tag,kind,material,geometry):
        return {"tag":tag,"guid":A.global_id(tag),"class":kind,"predefined_type":"FLOORING" if kind=="IfcCovering" else "ROOF" if kind=="IfcSlab" else "BEAM","name":{"en":tag,"de":tag},"storey":"roof","material":material,"attributes":{},"psets":{},"metadata":{"author":"Neutral","revision":1,"domains":["architecture","structure"]},"geometry":geometry}
    def part(ring,lo,hi,role,material):return {"outline":deepcopy(ring),"z0":lo,"z1":hi,"role":role,"material":material}
    d=CASE["domains"];left=[[0,0],[.8,0],[.8,1],[0,1]]
    paving=[part(left,-.09,-.04,"weathering_reserve","M-SUPPORT"),part(left,-.04,0,"wearing","M-PAVING"),part(d["beam"],-.09,-.03,"weathering_reserve","M-SUPPORT"),part(d["constrained"],-.03,-.015,"weathering_reserve","M-SUPPORT"),part(d["constrained"],-.015,0,"wearing","M-CONSTRAINED")]
    elements=[]
    for tag,domain,owners in (("SL-A",[[0,0],[3,0],[3,1],[0,1]],[("F-PAVING",d["paving"],["B-JOINT"]),("F-STAIR",d["stair"],[])]),("SL-B",d["gravel"],[("F-GRAVEL",d["gravel"],[])])):
        elements.append(obj(tag,"IfcSlab","LS-TEST",{"outline":domain,"level":0,"top_offset":q(-.09),"regions":[{"tag":"R-"+tag,"outline":domain,"thickness":q(.17),"top_offset":q(-.09),"material":"M-RC"}],"voids":[],"finish_voids":[{"tag":"RES-"+owner,"owner":owner,"outline":ring,**({"joint_owners":joints} if joints else {})} for owner,ring,joints in owners]}))
    elements[0]["geometry"]["regions"]=[{"tag":"R-LOW","outline":[[0,0],[1.2,0],[1.2,1],[0,1]],"thickness":q(.17),"top_offset":q(-.09),"material":"M-RC"},{"tag":"R-HIGH","outline":[[1.2,0],[3,0],[3,1],[1.2,1]],"thickness":q(.17),"top_offset":q(-.03),"material":"M-RC"}]
    elements[0]["geometry"]["top_offset"]=q(-.03)
    elements.append(obj("B-JOINT","IfcBeam","M-RC",{"kind":"beam","base_level":0,"parts":[part(d["beam"],-.03,0,"structural","M-RC")]}))
    for tag,material,depth,support,role,cells in (("F-PAVING","M-PAVING",.04,"M-SUPPORT","weathering_reserve",paving),("F-GRAVEL","M-GRAVEL",.02,"M-SUPPORT","weathering_reserve",[part(d["gravel"],-.09,-.02,"weathering_reserve","M-SUPPORT"),part(d["gravel"],-.02,0,"wearing","M-GRAVEL")]),("F-STAIR","M-TILE",.015,"M-MORTAR","bedding",[part(d["stair"],-.03,-.015,"bedding","M-MORTAR"),part(d["stair"],-.015,0,"wearing","M-TILE")])):
        item=obj(tag,"IfcCovering",material,{"kind":"flooring","base_level":0,"parts":cells,"layer_binding":{"material":"LS-TEST","layer":"paving","reference_thickness":q(.09),"support_material":support,"support_role":role}})
        if tag=="F-PAVING":item["geometry"]["layer_binding"]["wearing_bands"]=[{"maximum_reference_capacity":q(.03),"depth":q(.015),"material":"M-CONSTRAINED"}]
        item["attributes"]["finish_thickness"]=q(depth);elements.append(item)
    materials=[{"tag":tag,"guid":A.global_id(tag),"class":"IfcMaterial","name":{"en":tag,"de":tag},"density":{"value":24,"unit":"kN/m³","basis":"assumed"}} for tag in ("M-RC","M-PAVING","M-CONSTRAINED","M-GRAVEL","M-TILE","M-SUPPORT","M-MORTAR")]
    materials.append({"tag":"LS-TEST","guid":A.global_id("LS-TEST"),"class":"IfcMaterialLayerSet","name":{"en":"Layer set","de":"Schichten"},"layers":[{"id":"structural","material":"M-RC","name":{"en":"Structure","de":"Tragwerk"},"thickness":q(.17)},{"id":"paving","material":"M-SUPPORT","name":{"en":"Finish","de":"Ausbau"},"thickness":q(.09)}]})
    return {"version":1,"schema_id":"IFC4X3_ADD2","languages":["en","de"],"revision":1,"elements":elements,"types":[],"spaces":[],"storeys":[],"materials":materials,"relations":[],"models":[{"id":"architecture","element_tags":[e["tag"] for e in elements],"revision":1}],"car":{},"grid":{},**{key:{"tag":key.upper(),"guid":A.global_id(key.upper()),"class":kind} for key,kind in (("project","IfcProject"),("site","IfcSite"),("building","IfcBuilding"))}}


def event(value):
    """⚡ Address one host while its explicitly shared layer propagates through the full dependency graph."""
    return {"type":"LayerThicknessChanged","object_guid":A.global_id("SL-A"),"layer":"paving","value":value,"unit":"m"}


def oracle(parts):
    """🔬 Independently integrate actual GEOS occupied slices and SciPy convex physical cells."""
    levels=sorted({p[k] for p in parts for k in ("z0","z1")})
    volume=math.fsum((hi-lo)*unary_union([Polygon(p["outline"]) for p in parts if p["z0"]<(lo+hi)/2<p["z1"]]).area for lo,hi in zip(levels,levels[1:]))
    hull=math.fsum(ConvexHull(np.asarray(A.part_vertices(p),float)).volume for p in parts)
    return volume,hull


def test_layer_binding_schema_and_original_native_baseline_are_exact_and_owned():
    """🧬 Bind original parts explicitly while retaining source, outputs and canonical producer invalidation."""
    Draft202012Validator(json.loads(Path(__file__).with_name("🧬️.schema.json").read_text(encoding="utf-8"))).validate(CASE)
    m=model();before=deepcopy(m);schema=json.loads((Path(A.__file__).parent/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
    for obj in m["elements"]:
        if obj["class"]!="IfcCovering":continue
        Draft202012Validator({"$defs":schema["$defs"],"$ref":"#/$defs/geometry"}).validate(obj["geometry"])
        geometry=A.solid_geometry(obj["tag"],m);actual=geometry["parts"]
        assert [{k:p[k] for k in ("outline","z0","z1","role","material")} for p in geometry["full_parts"]]==obj["geometry"]["parts"]
        assert math.isclose(sum(A._polyhedron_volume(p)for p in actual),sum(A._polyhedron_volume(p)for p in obj["geometry"]["parts"]),abs_tol=1e-13)
        actual[0]["outline"][0][0]=99
        assert m==before
    backend=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🧱️layers.🐍️")
    first=A.geometry_revision(m);original=backend.parts
    with patch.object(backend,"parts",side_effect=original):assert A.geometry_revision(m)!=first
    assert m==before


def test_baseline_owned_recipe_accepts_native_mesh_joints_and_retains_original_reference():
    """🧩 Preserve frozen source references while actual baseline material follows exact declared native joints."""
    m=model();case=CASE["baseline_authority"];owner=A._find(m,case["owner"]);raw=owner["geometry"]["parts"][0]
    owner["geometry"]["parts"]=[{**raw,**A._polyhedron_part(A.part_vertices(raw),A.part_faces(raw))}]
    saved=deepcopy(m);finish=A._find(m,case["finish"]);backend=A._flooring_layer_backend()
    private=backend.parts(A,m,finish,exact=True)
    assert private and any(isinstance(v,Fraction)for part in private for point in A.part_vertices(part)for v in point)
    actual=A._solid_geometry(m,finish,exact=True)
    reference=[{k:p[k]for k in ("outline","z0","z1","role","material")}for p in actual["full_parts"]]
    assert reference==finish["geometry"]["parts"] and m==saved
    volume=sum(A._polyhedron_volume(part)for part in actual["parts"])
    independent=sum(ConvexHull(np.asarray(A.part_vertices(part),float)).volume for part in actual["parts"])
    assert math.isclose(volume,case["expected_owned_volume"],abs_tol=1e-13)
    assert math.isclose(volume,independent,abs_tol=1e-13)
    for material,expected in case["materials"].items():
        assert math.isclose(sum(A._polyhedron_volume(part)for part in actual["parts"]if part["material"]==material),expected,abs_tol=1e-13)
    assert all(not A._native_parts_have_volume(A._reference_material_parts(part,[],[owner["geometry"]["parts"][0]]))for part in actual["parts"])


def test_bound_material_inherits_actual_host_holes_and_preserves_original_reference():
    """🕳️ Preserve a declared host aperture in physical material without erasing its original Reference."""
    m=model();case=CASE["host_hole"];host=A._find(m,case["slab"]);finish=A._find(m,case["finish"])
    host["geometry"]["voids"]=[{"tag":"NEUTRAL-BOUND-HOLE","outline":case["ring"]}];saved=deepcopy(m)
    result=A.solid_geometry(finish["tag"],m);parts=result["parts"]
    assert math.isclose(sum(A._polyhedron_volume(part)for part in parts),case["expected_owned_volume"],abs_tol=1e-13)
    independent,hull=oracle(parts)
    assert math.isclose(independent,case["expected_owned_volume"],abs_tol=1e-13)and math.isclose(hull,independent,abs_tol=1e-13)
    assert all(Polygon(part["outline"]).intersection(Polygon(case["ring"])).area==0 for part in parts)
    assert [{k:p[k]for k in ("outline","z0","z1","role","material")}for p in result["full_parts"]]==finish["geometry"]["parts"]
    A._slab_geometry_entry(m,host,exact=True)
    assert m==saved


def test_repeated_bound_windows_form_one_disjoint_owned_material_union():
    """🔗 Treat overlapping same-product queries as one physical recipe without counting material twice."""
    m=model();case=CASE["window_union"];host=A._find(m,case["slab"]);finish=A._find(m,case["finish"])
    host["geometry"]["finish_voids"].append({"tag":"NEUTRAL-REPEATED-QUERY","owner":finish["tag"],"outline":case["ring"]});saved=deepcopy(m)
    parts=A.solid_geometry(finish["tag"],m)["parts"]
    assert math.isclose(sum(A._polyhedron_volume(part)for part in parts),case["expected_owned_volume"],abs_tol=1e-13)
    assert A._parts_disjoint(parts)
    independent,hull=oracle(parts)
    assert math.isclose(independent,case["expected_owned_volume"],abs_tol=1e-13)and math.isclose(hull,independent,abs_tol=1e-13)
    A._slab_geometry_entry(m,host,exact=True)
    assert m==saved


def test_shared_layer_events_rebuild_exclusive_material_with_fixed_joint_and_preserved_wearing():
    """⚖️ Prove actual reserve ownership, material mass and positive two-nanometre newly exposed material."""
    baseline=model();saved=deepcopy(baseline);beam=A.solid_geometry("B-JOINT",baseline)["parts"]
    for value in CASE["layer_values"]:
        changed=A.apply_event(event(value),baseline)
        assert baseline==saved and changed["revision"]==2
        assert sorted(o["tag"] for o in changed["elements"] if o["metadata"]["revision"]==2)==CASE["affected_tags"]
        assert changed["models"][0]["revision"]==2
        assert [o["guid"] for o in changed["elements"]]==[o["guid"] for o in baseline["elements"]]
        all_parts=[];top=float(Fraction(str(value))-Fraction(str(CASE["baseline"])))
        for tag in ("F-PAVING","F-GRAVEL","F-STAIR"):
            parts=A.solid_geometry(tag,changed)["parts"];all_parts+=parts
            assert math.isclose(max(p["z1"] for p in parts),top,abs_tol=CASE["comparison"]["coordinate"])
            q=A.quantities(tag,changed);geos,hull=oracle(parts)
            assert math.isclose(q["volume"],geos,abs_tol=CASE["comparison"]["volume"])
            assert math.isclose(geos,hull,abs_tol=CASE["comparison"]["volume"])
            assert A._parts_disjoint(parts)
            for kind in CASE["representations"]:
                representation=A.represent(tag,kind,changed)
                assert representation["primitives"] and math.isclose(representation["derived"]["volume"],geos,abs_tol=CASE["comparison"]["volume"])
                if kind=="body":
                    volume=math.fsum(ConvexHull(np.asarray(p["vertices"],float)).volume if p["kind"]=="faceted_brep"else Polygon(p["profile"]["points"]).area*p["depth"]for p in representation["primitives"])
                    assert math.isclose(volume,geos,abs_tol=CASE["comparison"]["volume"])
            assert all(not (p["z0"]<0<p["z1"] and Polygon(p["outline"]).intersection(Polygon(CASE["domains"]["beam"])).area>0) for p in parts)
        actual,independent=oracle(all_parts+beam);expected=4*value-.108
        assert math.isclose(actual,expected,abs_tol=CASE["comparison"]["volume"])
        assert math.isclose(independent,expected,abs_tol=CASE["comparison"]["volume"])
        assert A.solid_geometry("B-JOINT",changed)["parts"]==beam
        for tag in ("SL-A","SL-B"):A._slab_geometry_entry(changed,A._find(changed,tag))
        tiny=[p for p in all_parts if p["z0"]==0 and 0<p["z1"]<1e-8 and Polygon(p["outline"]).intersection(Polygon(CASE["domains"]["beam"])).area>0]
        if value<.091:assert tiny and math.fsum(A._polyhedron_volume(p) for p in tiny)>0
        constrained=[p for p in all_parts if p["material"]=="M-CONSTRAINED"]
        assert constrained and all(math.isclose(p["z1"]-p["z0"],.015,abs_tol=1e-12) for p in constrained)


def test_invalid_layer_events_and_missing_actual_owners_remain_rejected():
    """🛡️ Preserve strict event values and real two-nanometre reserve gaps without masking missing material."""
    m=model();before=deepcopy(m)
    for value in CASE["invalid_values"]:
        try:A.apply_event(event(value),m)
        except ValueError:pass
        else:raise AssertionError("Invalid layer thickness accepted")
    owner=A._find(m,"F-PAVING")
    for part in owner["geometry"]["parts"]:
        if part["role"]=="wearing":part["z1"]-=2e-9
    try:A._slab_geometry_entry(m,A._find(m,"SL-A"))
    except ValueError as error:assert "exactly occupied" in str(error)
    else:raise AssertionError("A positive missing material band was accepted")
    assert before==model()


def test_bound_floor_progress_cancellation_and_retry_do_not_publish_partial_results():
    """🛑 Keep exact stock construction observable, cancellable and isolated from the canonical memo."""
    changed=A.apply_event(event(.14),model());events=[]
    try:A.solid_geometry("F-PAVING",changed,progress=events.append,cancel=lambda:True)
    except InterruptedError:pass
    else:raise AssertionError("Bound floor cancellation ignored")
    assert not events
    value=A.solid_geometry("F-PAVING",changed,progress=events.append)
    assert events and events[-1]=={"completed":1,"total":1} and value["parts"]
    value["parts"][0]["outline"][0][0]=100
    assert A.solid_geometry("F-PAVING",changed)["parts"][0]["outline"][0][0]!=100


def test_bound_wearing_band_units_are_explicit_metres():
    """📏 Reject globally valid area/volume units while accepting both declared metre operands."""
    schema=json.loads((Path(A.__file__).parent/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
    validator=Draft202012Validator({"$defs":schema["$defs"],"$ref":"#/$defs/flooring_layer_binding"})
    actual=[]
    for case in CASE["binding_units"]:
        binding=deepcopy(A._find(model(),"F-PAVING")["geometry"]["layer_binding"])
        binding["wearing_bands"][0][case["field"]]["unit"]=case["unit"]
        actual.append(not list(validator.iter_errors(binding)))
    assert actual==[case["valid"] for case in CASE["binding_units"]],(actual,CASE["binding_units"])
