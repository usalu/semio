"""🛝️ Independent floor-arrival material ownership and ramp finish contracts."""
import importlib,json,math
from pathlib import Path
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_floor_arrivals_have_single_slab_ownership():
    """🧩 Compare all three native floor arrivals against independent GEOS material intersections."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    from jsonschema import Draft202012Validator
    case=json.loads(Path(__file__).with_suffix(".json").read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(case)
    m=a.load_model();errors=[]
    for row in case["arrivals"]:
        raw=a._find(m,row["ramp"]);r=a.ramp_geometry(row["ramp"],m);s=a._slab_geometry(m,a._find(m,row["slab"]));landing=next(x for x in raw["geometry"]["landings"] if x["tag"]==row["landing"])
        zone=Polygon(row["outline"]);own=[p for p in r["parts"] if p.get("tag")==row["landing"]];slab=s["parts"]+s["shared_parts"]
        overlap=sum(Polygon(p["outline"]).intersection(Polygon(q["outline"])).area*max(0,min(p["z1"],q["z1"])-max(p["z0"],q["z0"])) for p in own for q in slab)
        if overlap>1e-8:errors.append((row["landing"],"duplicate occupied material",overlap))
        if landing.get("ownership")!="floor_slab" or landing["owner"]!=row["slab"]:errors.append((row["landing"],"sole slab owner",landing["owner"]))
        if not Polygon(a._expand(m,landing["outline"])).equals(zone):errors.append((row["landing"],"arrival clear support footprint"))
        support=unary_union([Polygon(p["outline"]) for p in slab]);missing=zone.difference(support).area
        if missing>1e-7:errors.append((row["landing"],"unsupported footprint",missing))
    assert not errors,errors


def test_ramp_wearing_assembly_preserves_finished_walklines_and_six_cm_structure():
    """📐 Verify independently sloping RC and physical finish layers with SciPy hull volumes."""
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon
    m=a.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    for row in case["finishes"]:
        ramp=a._find(m,row["ramp"]);assert ramp["geometry"]["finish_thickness"]["value"]==row["depth"]
        assert ramp["geometry"]["finish_direction"]==row["direction"]
        finish=a.solid_geometry(row["tag"],m);assert finish["predefined_type"]=="FLOORING" and finish["domains"]==["architecture"]
        expected_volume=0.
        for flight in a.ramp_geometry(row["ramp"],m)["flights"]:
            factor=math.sqrt(1+flight["slope"]**2) if row["direction"]=="normal" else 1
            expected_volume+=flight["run"]*flight["width"]*row["depth"]*factor
            vertices=[v for p in flight["parts"] for v in a.part_vertices(p)];assert abs(max(p[2] for p in vertices)-(flight["z1"]-row["depth"]*factor))<1e-8
            assert abs(sum(ConvexHull(a.part_vertices(p)).volume for p in flight["parts"])-(flight["run"]*flight["structural_width"]*.06-case["structural_joint_cuts"][flight["tag"]]))<1e-8,(flight["tag"],sum(ConvexHull(a.part_vertices(p)).volume for p in flight["parts"]),flight["run"]*flight["structural_width"]*.06,case["structural_joint_cuts"][flight["tag"]])
        for landing in ramp["geometry"]["landings"]:
            if landing["ownership"]=="ramp_component":
                assert Polygon(a._expand(m,landing["walking_outline"])).difference(Polygon(a._expand(m,landing["outline"]))).area<1e-10
                expected_volume+=a._area(a._expand(m,landing["walking_outline"]))*row["depth"]
        independent=sum(ConvexHull(a.part_vertices(p)).volume for p in finish["parts"]);assert abs(independent-(expected_volume-row["exclusive_cut_m3"]))<1e-7,(row["tag"],independent,expected_volume)
        assert abs(a.quantities(row["tag"],m)["volume"]-independent)<1e-7


def test_asymmetric_structural_and_walking_offsets_are_distinct():
    """📐 Compare independently registered walking and structural faces with GEOS and SciPy."""
    from copy import deepcopy
    from shapely.geometry import Polygon
    from scipy.spatial import ConvexHull
    c=json.loads(Path(__file__).with_suffix(".json").read_text())["asymmetric"];m=a.load_model();parent=deepcopy(a._find(m,"RM-1"));parent.update(tag="NEUTRAL-RAMP",guid=a.global_id("NEUTRAL-RAMP"));parent["attributes"]["width"]["value"]=c["clear_width"];flight=deepcopy(a._find(m,"RF-1A"));flight.update(tag="NEUTRAL-FLIGHT",guid=a.global_id("NEUTRAL-FLIGHT"));q=lambda value:{"value":value,"unit":"m","basis":"inferred","source":"Neutral asymmetric fixture"};flight["geometry"].update(ramp=parent["tag"],path=c["path"],from_level=0.,to_level=1.,portion=[0,1],structural_offsets=[q(v) for v in c["structural_offsets"]],walking_offsets=[q(v) for v in c["walking_offsets"]],finish_thickness=q(c["finish"]),finish_direction="vertical");flight["attributes"]["width"]["value"]=c["clear_width"];m["elements"].extend([parent,flight]);g=a._ramp_geometry(m,flight);p=g["parts"][0];assert Polygon(a.part_footprint(p)).symmetric_difference(Polygon(c["footprint"])).area<1e-10;assert abs(ConvexHull(a.part_vertices(p)).volume-c["volume"])<1e-10;assert g["flights"][0]["width"]==c["clear_width"] and abs(g["flights"][0]["structural_width"]-c["structure_width"])<1e-10
