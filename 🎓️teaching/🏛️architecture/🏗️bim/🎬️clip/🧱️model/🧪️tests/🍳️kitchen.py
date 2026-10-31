"""🍳 Original registered kitchen/pantry fixtures, operated routes and hollow native construction."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_registered_main_and_pantry_bowls_and_open_worktable():
    """📜 Keep two main sinks distinct from the pantry bowl and reject the old filled worktable mass."""
    from jsonschema import Draft202012Validator
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(case)
    for row in case["bowls"]:
        obj=api._find(source,row["tag"])
        assert obj["class"]=="IfcSanitaryTerminal" and obj["predefined_type"]=="SINK"
        assert obj["guid"]==api.global_id(row["tag"])
        assert sum(p["role"]=="tap_cross_handle" for p in api._find(source,row["tap"])["geometry"]["parts"])==case["tap_cross_arms"]
        assert math.dist(obj["geometry"]["axis"],row["registered_center"])<=case["uncertainty_m"]
    table=api._find(source,case["table"]["tag"])
    assert sum(p["role"]=="slender_table_leg" for p in table["geometry"]["parts"])==case["table"]["legs"]

def test_actual_kitchen_lofts_tap_airways_deck_mounts_and_discharge():
    """🚰 Prove real material volume, bowl/drain voids and mounted hollow spouts over actual basin cavities."""
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon,Point
    from shapely.ops import unary_union
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());base=case["floor"]
    def contains(p,q):
        return all(sum(e[k]*q[k] for k in range(3))+e[3]<1e-8 for e in ConvexHull(api.part_vertices(p)).equations)
    def world(q):return [q[0],q[1],q[2]+base]
    checked=set()
    for row in case["bowls"]:
        bowl=api.service_geometry(row["tag"],source);tap=api.solid_geometry(row["tap"],source);deck=api.solid_geometry(row["counter"],source)
        for tag,body in ((row["tag"],bowl),(row["tap"],tap),(row["counter"],deck)):
            if tag in checked:continue
            independent=sum(ConvexHull(api.part_vertices(p)).volume for p in body["parts"])
            assert abs(independent-api.quantities(tag,source)["volume"])<1e-7,tag
            checked.add(tag)
        for q in (row["cavity"],row["drain"]):assert not any(contains(p,world(q)) for p in bowl["parts"]),(row["tag"],q)
        for q in row["airway"]:assert not any(contains(p,world(q)) for p in tap["parts"]),(row["tap"],q)
        q=world(row["mount"]);assert any(contains(p,q) for p in tap["parts"]) and any(contains(p,q) for p in deck["parts"]),(row["tap"],"actual deck mount")
        q=world(row["discharge_probe"]);regions=[Polygon(r) for p in bowl["parts"] if p["z0"]<q[2]<p["z1"] for r in api.part_regions(p,q[2])];wall=unary_union(regions)
        assert wall.convex_hull.contains(Point(q[:2])) and not wall.covers(Point(q[:2])),(row["tap"],"discharges over actual cavity")
    table=api.solid_geometry(case["table"]["tag"],source)
    assert not any(contains(p,world(case["table"]["empty"])) for p in table["parts"]),"Open air under table"


def test_original_kitchen_programme_patio_glazing_and_authored_portal_ownership():
    """🪟 Require the original full-depth pantry, open hall and two panes around the central glazed leaf."""
    from shapely.geometry import Polygon
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    for tag,outline in case["spaces"].items():assert api._find(source,tag)["outline"]==outline
    rooms=[(s["tag"],Polygon(api._expand(source,s["outline"]))) for s in source["spaces"] if s["storey"]=="first"]
    for i,(tag,shape) in enumerate(rooms):
        assert shape.is_valid
        for peer,other in rooms[i+1:]:assert shape.intersection(other).area<1e-8,(tag,peer)
    assert api._find(source,"W-KITCHEN")["geometry"]["path"][-1][1]==6.006
    for row in case["patio_glazing"]:
        filling=api.filling_geometry(row["tag"],source)
        assert any(p["role"]=="glass" for p in filling["parts"])
        opening=api._find(source,row["opening"]);owners=[r for r in source["relations"] if r["class"]=="IfcRelVoidsElement" and row["opening"] in r["related"]]
        assert len(owners)==1 and owners[0]["relating"]==row["host"]
        assert not any(row["opening"] in r["related"] for r in source["relations"] if r["class"]=="IfcRelContainedInSpatialStructure")
    patio=api.filling_geometry(case["patio_door"]["tag"],source)
    assert any(p["role"]=="glass" for p in patio["parts"])
    for row in case["doors"]:
        door=api._find(source,row["tag"]);opening=api._find(source,door["geometry"]["opening"])
        assert opening["geometry"]["wall"]==row["host"]
        assert len([r for r in source["relations"] if r["class"]=="IfcRelVoidsElement" and r["relating"]==row["host"] and opening["tag"] in r["related"]])==1


def test_actual_kitchen_pantry_routes_clear_closed_material_and_operated_leaves():
    """🚶 Sweep standing bodies past actual cabinetry/table into all three basins using explicit open door poses."""
    from shapely.geometry import Polygon,LineString
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());g=api.building_geometry(source);base=case["floor"];top=base+case["body_height"];obstacles=[]
    moving={r["tag"] for r in case["doors"]}
    for group in ("walls","pilotis","screens","beams","details","furnishings","railings","services","stairs","fillings"):
        for body in g[group]:
            if body["tag"] in moving:continue
            for p in body["parts"]:
                if p["z0"]<top and p["z1"]>base:obstacles.extend((body["tag"],Polygon(r)) for r in api.part_band_footprints(p,max(base,p["z0"]),min(top,p["z1"])))
    frames=[];operated=[]
    for row in case["doors"]:
        door=api.filling_geometry(row["tag"],source)
        assert door["clear_width"]>=case["body_diameter"] and door["clear_height"]>=case["body_height"]
        frames.extend((row["tag"],Polygon(p["outline"])) for p in door["parts"] if p["role"]=="frame" and p["z0"]<top and p["z1"]>base)
        leaves=[p for p in door["parts"] if p["role"]!="frame"];vertices=[v for p in leaves for v in api.part_vertices(p)];hx=min(v[0] for v in vertices);hy=min(v[1] for v in vertices);sign=1 if row["angle"]>0 else -1
        for p in leaves:
            closed=Polygon(p["outline"]);opened=Polygon([[hx-sign*(y-hy),hy+sign*(x-hx)] for x,y in p["outline"]])
            for pose,shape in (("closed",closed),("open90",opened)):
                collisions=[(tag,shape.intersection(poly).area) for tag,poly in obstacles+frames if shape.bounds[0]<poly.bounds[2] and poly.bounds[0]<shape.bounds[2] and shape.bounds[1]<poly.bounds[3] and poly.bounds[1]<shape.bounds[3]]
                assert sum(area for tag,area in collisions)<1e-8,(row["tag"],pose,[(tag,a) for tag,a in collisions if a>1e-10])
            operated.append((row["tag"],opened))
    for row in case["routes"]:
        path=LineString(row["points"]).buffer(row.get("diameter",case["body_diameter"])/2,cap_style=1,join_style=1)
        collisions=[(tag,path.intersection(poly).area) for tag,poly in obstacles+frames+operated if path.bounds[0]<poly.bounds[2] and poly.bounds[0]<path.bounds[2] and path.bounds[1]<poly.bounds[3] and poly.bounds[1]<path.bounds[3]]
        assert sum(area for tag,area in collisions)<1e-8,(row["id"],[(tag,a) for tag,a in collisions if a>1e-10])


def test_pantry_divider_has_level_banded_real_facade_contacts_without_shared_material():
    """🧱 Prove actual stepped masonry/wood/glass junction coverage with independent GEOS bands."""
    from shapely.geometry import Polygon,box
    from shapely.ops import unary_union
    m=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());assert all(row["tag"] in api._registry(m) for row in case["divider_junction"]);facade=api._wall_geometry(m,api._find(m,"W-S"))["parts"];facade+=api._filling_geometry(m,api._find(m,"RW-S"))["parts"];facade+=api._wall_cutters(m)
    for row in case["divider_junction"]:
        e=api._find(m,row["tag"]);g=api._wall_geometry(m,e);z=(row["z0"]+row["z1"])/2;target=box(row["axis_x"]-row["width"]/2,row["face_y"],row["axis_x"]+row["width"]/2,row["main_y"]);joint=unary_union([Polygon(r) for p in g["parts"] for r in api.part_regions(p,z)]);host=unary_union([Polygon(r) for p in facade if p["z0"]<z<p["z1"] for r in api.part_regions(p,z)]);assert target.difference(joint.union(host)).area<1e-8,(row["tag"],"jointgap");assert joint.intersection(host).area<1e-8,(row["tag"],"occupiedoverlap");assert joint.intersection(box(2.96,.299999,3.04,.300001)).area>1e-8
