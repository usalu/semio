"""🧿 Retain original analytic profile placements independently of tessellation and material centroid."""
import importlib,json,math
from copy import deepcopy
from pathlib import Path
from unittest.mock import patch
from jsonschema import Draft202012Validator
from shapely.geometry import Polygon,Point,box
from shapely.affinity import rotate,translate
A=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
B=Path(__file__).parent


def test_explicit_analytic_profile_axis_preserves_source_placement_and_off_center_material():
    """📍 Validate the authored analytic axis and independent GEOS material centroid after placement and rotation."""
    case=json.loads((B/"🔣️.json").read_text(encoding="utf-8"));Draft202012Validator(json.loads((B/"🧬️.schema.json").read_text(encoding="utf-8"))).validate(case)
    for row in case["cases"]:
        profile=deepcopy(row["profile"]);axis=deepcopy(row["axis"]);actual=A.profile_geometry(profile,axis);polygon=Polygon(actual["outline"],actual["holes"])
        assert actual.get(case["axis_field"])==axis,row["id"]
        assert actual[case["axis_field"]]is not axis,row["id"]
        assert math.dist(actual["centroid"],[polygon.centroid.x,polygon.centroid.y])<1e-9,row["id"]
        assert (math.dist(actual["centroid"],axis)>1e-3)==row["offset_centroid"],row["id"]
        actual[case["axis_field"]][0]+=1
        assert profile==row["profile"]and axis==row["axis"],row["id"]


def test_analytic_profile_producer_changes_invalidate_canonical_geometry_revision():
    """🧬 Bind analytic profile placement producers to the same canonical geometry revision as their consumers."""
    neutral=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📦️disjoint")
    model=neutral.model({"id":"analytic-profile-producer"},[{"outline":[[0,0],[1,0],[1,1],[0,1]],"z0":0,"z1":1,"material":"MAT-TEST"}]);before=A.geometry_revision(model)
    with patch.object(A,"profile_geometry",side_effect=A.profile_geometry):assert A.geometry_revision(model)!=before


def test_vehicle_circle_clearance_consumes_analytic_axis_and_rejects_missing_placement():
    """🚗 Compare actual car pose clearance with independent GEOS and retain analytic placement despite uneven facets."""
    row=json.loads((B/"🔣️.json").read_text(encoding="utf-8"))["car_case"];car=row["car"];pose=row["pose"]
    car_shape=translate(rotate(box(-car["length"]/2,-car["width"]/2,car["length"]/2,car["width"]/2),pose[2],origin=(0,0)),pose[0],pose[1]);expected=Point(row["axis"]).distance(car_shape)-row["radius"]
    assert math.isclose(A._part_pose_distance(pose,car,row),expected,rel_tol=1e-12,abs_tol=1e-12)
    missing={k:v for k,v in row.items()if k!="axis"}
    try:A._part_pose_distance(pose,car,missing)
    except ValueError:pass
    else:raise AssertionError("Analytic circle collision must not invent a placement from facets")
