"""🚿 Original ground sanitary programme and independent physical acceptance contracts."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_registered_ground_sanitary_programme_and_space_ownership():
    """📜 Check typed original fixtures, raster registration, coherent spaces and unchanged garage."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon,Point
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(case)
    tags={space["tag"] for space in source["spaces"]}
    assert set(case["space_tags"])<=tags
    assert not set(case["removed_spaces"])&tags
    assert api._find(source,"SP-GARAGE")["outline"]==case["garage_outline"]
    spaces={tag:Polygon(api._expand(source,api._find(source,tag)["outline"])) for tag in case["space_tags"]}
    for index,(tag,shape) in enumerate(spaces.items()):
        assert shape.is_valid and shape.area>0,tag
        for peer,other in list(spaces.items())[index+1:]:
            assert shape.intersection(other).area<1e-8,(tag,peer)
    for row in case["fixtures"]:
        obj=api._find(source,row["tag"])
        assert obj["class"]=="IfcSanitaryTerminal" and obj["predefined_type"]==row["predefined_type"]
        assert math.dist(obj["geometry"]["axis"],row["center"])<=case["uncertainty_m"]
        assert spaces[row["space"]].covers(Point(obj["geometry"]["axis"])),row["tag"]
        assert obj["historical_evidence"]["status"]=="inferred_reconstruction"
    assert max(x for x,y in api._find(source,"SP-LAUNDRY")["outline"])==case["partitions"]["laundry_max_x"]

def test_ground_hollow_sanitary_bodies_and_volumes_match_geos():
    """🧱 Integrate every original ground bowl/cistern/bath loft independently with Simpson-exact GEOS areas."""
    from shapely.geometry import Polygon,Point
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    for row in case["fixtures"]:
        obj=api._find(source,row["tag"]);geometry=api._service(source,obj);total=0.
        for part in geometry["parts"]:
            lower=Polygon(part["outline"]);height=part["z1"]-part["z0"]
            if "top_outline" in part:
                upper=Polygon(part["top_outline"]);middle=Polygon([[(a[k]+b[k])/2 for k in (0,1)] for a,b in zip(part["outline"],part["top_outline"])])
                total+=height*(lower.area+4*middle.area+upper.area)/6
            else:
                total+=lower.area*height
        assert abs(total-api.quantities(row["tag"],source)["volume"])<1e-7,row["tag"]
        assert all(part.get("material") for part in geometry["parts"])
        if row["predefined_type"]=="TOILETPAN":
            center=Point(obj["geometry"]["axis"])
            for part in geometry["parts"]:
                if part["role"]=="bowl_shell":
                    fraction=(.3-part["z0"])/(part["z1"]-part["z0"])
                    ring=[[(1-fraction)*a[k]+fraction*b[k] for k in (0,1)] for a,b in zip(part["outline"],part["top_outline"])]
                    assert not Polygon(ring).contains(center)
        if "feet" in row:
            assert sum(part["role"]=="bath_foot" for part in obj["geometry"]["parts"])==row["feet"]


def test_ground_wc_portals_and_body_routes_clear_all_actual_fixed_material():
    """🚶 Sweep a0.6m body through real portals against every native fixed ground obstacle."""
    from shapely.geometry import LineString,Polygon
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());geometry=api.building_geometry(source);obstacles=[]
    for category in ("walls","pilotis","screens","beams","details","furnishings","railings","services","stairs"):
        for item in geometry[category]:
            for part in item["parts"]:
                if part["z0"]<case["body_height"] and part["z1"]>1e-10:
                    obstacles.extend((item["tag"],Polygon(ring)) for ring in api.part_band_footprints(part,max(0,part["z0"]),min(case["body_height"],part["z1"])))
    for row in case["fixtures"]:
        if "route" not in row:continue
        frames=[]
        for door in (["D-WC-GROUND-SERVICE"] if row["tag"]=="WC-GROUND-SERVICE" else ["D-CHAUFFEUR-BATH","D-WC-CHAUFFEUR"]):
            filling=api.filling_geometry(door,source)
            assert filling["clear_width"]>=case["body_diameter"]
            assert filling["clear_height"]>=case["body_height"]
            frames.extend((door,Polygon(part["outline"])) for part in filling["parts"] if part["role"]=="frame" and part["z0"]<case["body_height"] and part["z1"]>0)
        route=LineString(row["route"]).buffer(case["body_diameter"]/2,cap_style=1,join_style=1)
        collisions=[(tag,route.intersection(shape).area) for tag,shape in obstacles+frames if route.bounds[0]<shape.bounds[2] and shape.bounds[0]<route.bounds[2] and route.bounds[1]<shape.bounds[3] and shape.bounds[1]<route.bounds[3]]
        assert sum(area for tag,area in collisions)<1e-8,(row["tag"],[(tag,area) for tag,area in collisions if area>1e-10])


def test_registered_bath_clearance_keeps_the_structural_column_occupied():
    """🛡️ Retain the independently native-proved structural witness while removing bath-shell penetration."""
    from scipy.spatial import ConvexHull
    source=api.load_model();case=json.loads(Path(__file__).with_suffix('.json').read_text())
    def occupied(parts,point):
        """🔬 Evaluate strict interior occupancy independently from native vertices and SciPy halfspaces."""
        return any(all(sum(eq[i]*point[i]for i in range(3))+eq[3]<-1e-9 for eq in ConvexHull(api.part_vertices(p)).equations)for p in parts)
    for row in case['clearance_controls']:
        fixture=api.service_geometry(row['fixture'],source);column=api.column_geometry(source,api.element(row['column'],source))
        assert occupied(fixture['parts'],row['point'])is row['fixture_occupied'],row['fixture']
        assert occupied(column['parts'],row['point'])is row['column_occupied'],row['column']
