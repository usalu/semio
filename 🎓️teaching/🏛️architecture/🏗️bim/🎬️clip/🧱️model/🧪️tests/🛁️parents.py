"""🛁 Whole original parents suite, qualified construction and independent physical acceptance."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_original_parents_suite_programme_and_correct_cabinet_identity():
    """📜 Require the corrected cabinet, distinct wet/rest construction and proper sanitary identities."""
    from jsonschema import Draft202012Validator
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(case)
    Draft202012Validator(json.loads((Path(api.__file__).parent/"🧬️schema/🔣️.schema.json").read_text())).validate(source)
    for row in case["fixtures"]:
        obj=api._find(source,row["tag"])
        assert obj["guid"]==api.global_id(row["tag"])
        assert obj["class"]==row["class"] and obj["predefined_type"]==row["predefined_type"]
        if "center" in row:assert math.dist(obj["geometry"]["axis"],row["center"])<1e-8
        assert obj["historical_evidence"]["status"]=="inferred_reconstruction"
    for tag in case["required_bodies"]:
        obj=api._find(source,tag)
        assert obj["geometry"]["parts"]
        assert obj["guid"]==api.global_id(tag)
    chaise=api._find(source,"FIX-CHAISE")
    assert "rc_cantilever" in {part["role"] for part in chaise["geometry"]["parts"]}
    assert chaise["attributes"]["mosaic_module"]["value"]==case["tile_module_m"]
    cabinet=api._find(source,"FIX-DRESSING-STORAGE")
    assert "concrete_cabinet_frame" in {p["role"] for p in cabinet["geometry"]["parts"]}
    for row in case["doors"]:
        door=api._find(source,row["tag"]);opening=api._find(source,door["geometry"]["opening"])
        assert opening["geometry"]["wall"]==row["host"]
        owners=[r for r in source["relations"] if r["class"]=="IfcRelVoidsElement" and opening["tag"] in r["related"]]
        assert len(owners)==1 and owners[0]["relating"]==row["host"]
        assert door["guid"]==api.global_id(door["tag"]) and opening["guid"]==api.global_id(opening["tag"])


def test_parents_hollow_bodies_and_cantilever_match_independent_hulls():
    """📐 Prove service voids, separate concrete cabinet, physical skins and occupied material conservation."""
    from scipy.spatial import ConvexHull
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    def contains(part,point):
        return all(sum(e[k]*point[k] for k in range(3))+e[3]<1e-8 for e in ConvexHull(api.part_vertices(part)).equations)
    for tag,axis in case["fixed_axes"].items():assert api._find(source,tag)["geometry"]["axis"]==axis
    for tag in [row["tag"] for row in case["fixtures"]]+case["required_bodies"]:
        obj=api._find(source,tag)
        geom=api.service_geometry(tag,source) if obj["class"]=="IfcSanitaryTerminal" else api.solid_geometry(tag,source)
        independent=sum(ConvexHull(api.part_vertices(part)).volume for part in geom["parts"])
        assert abs(independent-api.quantities(tag,source)["volume"])<1e-7,(tag,independent,api.quantities(tag,source)["volume"])
        if tag=="FIX-BATH":
            for point in ([5.1,15.35,3.52],[case["drain_xy"][0],case["drain_xy"][1],3.35]):assert not any(contains(p,point) for p in geom["parts"]),"bath/drain cavity"
        if tag=="FIX-CHAISE":assert not any(contains(p,[5.78,15.3,3.42]) for p in geom["parts"]),"cantilever underside"
        if tag=="FIX-DRESSING-STORAGE":assert not any(contains(p,[6.85,15.35,4.07]) for p in geom["parts"]),"cabinet interior"
        if tag=="FIX-PARENTS-WET-PLATFORM":assert not any(contains(p,[4.66,14.5,3.47]) for p in geom["parts"]),"service plinth cavity"

def test_first_floor_rooms_are_disjoint_and_private_suite_is_not_public_hall():
    """🗺️ Independently check every native first-floor room polygon after the complete programme reconciliation."""
    from shapely.geometry import Polygon,Point
    source=api.load_model();rooms=[(s["tag"],Polygon(api._expand(source,s["outline"]))) for s in source["spaces"] if s["storey"]=="first"]
    for i,(tag,shape) in enumerate(rooms):
        assert shape.is_valid and shape.area>0,tag
        for peer,other in rooms[i+1:]:assert shape.intersection(other).area<1e-8,(tag,peer,shape.intersection(other).area)
    lookup=dict(rooms)
    assert lookup["SP-PARENT-BATH"].covers(Point(7.64,15.5))
    assert not lookup["SP-FIRST-HALL"].covers(Point(7.64,15.5))
    assert lookup["SP-FIRST-HALL"].covers(Point(4.05,17.88))
    assert lookup["SP-PARENTS"].covers(Point(5.15,17.88))


def test_parents_public_and_private_routes_clear_actual_closed_fixed_material_and_open_leaves():
    """🚶 Check0.8m public access,0.6m private standing routes and both explicit inward open leaf poses."""
    from shapely.geometry import Polygon,LineString
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());building=api.building_geometry(source);base=3.17;top=base+case["body_height"];obstacles=[]
    for category in ("walls","pilotis","screens","beams","details","furnishings","railings","services","stairs"):
        for item in building[category]:
            for p in item["parts"]:
                if p["z0"]<top and p["z1"]>base:
                    obstacles.extend((item["tag"],Polygon(r)) for r in api.part_band_footprints(p,max(base,p["z0"]),min(top,p["z1"])))
    moving_tags={row["tag"] for row in case["doors"]}
    for item in building["fillings"]:
        if item["tag"] in moving_tags:continue
        for p in item["parts"]:
            if p["z0"]<top and p["z1"]>base:obstacles.extend((item["tag"],Polygon(r)) for r in api.part_band_footprints(p,max(base,p["z0"]),min(top,p["z1"])))
    leaf_sets=[];all_frames=[]
    for row in case["doors"]:
        filling=api.filling_geometry(row["tag"],source)
        assert filling["clear_width"]>=case["public_clear_width"] and filling["clear_height"]>=case["body_height"]
        frames=[Polygon(p["outline"]) for p in filling["parts"] if p["role"]=="frame" and p["z0"]<top and p["z1"]>base]
        all_frames.extend((row["tag"],p) for p in frames)
        leaves=[p for p in filling["parts"] if p["role"]!="frame"]
        points=[v for p in leaves for v in api.part_vertices(p)];sign=-1 if row["tag"]=="D-PARENTS" else 1
        hx=(max if sign<0 else min)(v[0] for v in points);hy=min(v[1] for v in points)
        closed=[Polygon(p["outline"]) for p in leaves];opened=[Polygon([[hx-sign*(y-hy),hy+sign*(x-hx)] for x,y in p["outline"]]) for p in leaves]
        for pose,polygons in (("closed",closed),("open90",opened)):
            for shape in polygons:
                collisions=[(tag,shape.intersection(poly).area) for tag,poly in obstacles+all_frames if shape.bounds[0]<poly.bounds[2] and poly.bounds[0]<shape.bounds[2] and shape.bounds[1]<poly.bounds[3] and poly.bounds[1]<shape.bounds[3]]
                assert sum(area for tag,area in collisions)<1e-8,(row["tag"],pose,[(tag,a) for tag,a in collisions if a>1e-10])
        leaf_sets.extend((row["tag"],p) for p in opened)
    for key,diameter in [("public_route",case["public_clear_width"]),("private_route",case["body_diameter"]),("son_route",case["body_diameter"]),("bath_approach",case["body_diameter"]),("chaise_approach",case["body_diameter"])]:
        route=LineString(case[key]).buffer(diameter/2,cap_style=1,join_style=1)
        collisions=[(tag,route.intersection(poly).area) for tag,poly in obstacles+all_frames+leaf_sets if route.bounds[0]<poly.bounds[2] and poly.bounds[0]<route.bounds[2] and route.bounds[1]<poly.bounds[3] and poly.bounds[1]<route.bounds[3]]
        assert sum(area for tag,area in collisions)<1e-8,(key,[(tag,a) for tag,a in collisions if a>1e-10])
