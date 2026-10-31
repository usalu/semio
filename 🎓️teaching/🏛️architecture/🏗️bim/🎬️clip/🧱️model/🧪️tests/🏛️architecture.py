"""🏛️ Source-attested architecture details and independent physical finish/portal checks."""
import importlib
import json
import math
from pathlib import Path

model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
remodel = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️remodel")


def test_service_entrance_is_an_independent_free_portal():
    """🚪 Establish a full0.8m route through actual service door, glazing and ground fittings."""
    from shapely.geometry import LineString, Point, Polygon
    source = model.load_model()
    geometry = model.building_geometry(source)
    case = json.loads(Path(__file__).with_name("🏛️architecture.json").read_text(encoding="utf-8"))["cases"][0]
    crossing = LineString(case["path"]).buffer(case["body_width"] / 2, cap_style=2)
    walls = remodel._physical_wall_polygons(geometry, "ground")
    assert crossing.intersection(walls).area < 1e-8
    opening = model._find(source,case["opening"])
    assert opening["geometry"]["curtain_wall"]==case["host"]
    filling = next(f for f in geometry["fillings"] if f["tag"]==case["door"])
    assert math.isclose(filling["clear_width"],case["clear_width"])
    assert filling["rough_width"]>filling["clear_width"]
    assert filling["clear_height"]>=case["body_height"]
    assert all(crossing.intersection(Polygon(p["outline"])).area<1e-8 for p in filling["parts"] if p["role"]=="frame" and p["z0"]<case["body_height"])
    assert all(crossing.intersection(Polygon(part["outline"])).area<1e-8 for part in geometry["curtain_wall"]["parts"] if part["role"]=="glass" and part["z0"]<case["body_height"] and part["z1"]>0)
    hall = Polygon(next(s for s in geometry["spaces"] if s["tag"] == "SP-HALL")["outline"])
    assert hall.covers(Point(case["hall_control"]).buffer(case["body_width"]/2))
    assert all(crossing.intersection(shape).area < 1e-8 for _,shape in remodel._column_shapes(geometry,0,case["body_height"]))
    for group in ("furnishings", "details", "railings", "stairs"):
        for item in geometry[group]:
            if item["storey"] != "ground":
                continue
            for part in item.get("parts", item.get("steps", [])):
                if part["z0"] < case["body_height"] and part["z1"] > 0:
                    assert crossing.intersection(Polygon(part["outline"])).area < 1e-8, item["tag"]


def test_guest_floor_finish_zones_follow_the_integrated_wardrobe():
    """🪵 Verify actual wearing zones at the recovered Y-run cabinet and independent washing niche."""
    from shapely.geometry import Polygon,Point,box
    from shapely.ops import unary_union
    source=model.load_model()
    case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['guest_finish']
    guest=Polygon(model._expand(source,model._find(source,'SP-GUEST')['outline']))
    parquet=model._find(source,'FIN-GUEST-PARQUET');tile=model._find(source,'FIN-GUEST-TILE')
    def wearing(obj):
        """🧱 Inspect material-owned wearing cells independently from their bedding layers."""
        return unary_union([Polygon(p['outline']) for p in obj['geometry']['parts'] if p['role']=='wearing'])
    a,b=wearing(parquet),wearing(tile)
    assert a.intersection(b).area<1e-8
    cuts=[Polygon(model.part_footprint(p)) for obj in source['elements'] if obj['class']=='IfcColumn' for p in model.column_geometry(source,obj)['parts'] if p['z0']<3.17 and p['z1']>3.14]
    slab=model._slab_geometry(source,model._find(source,'SL-1'))
    support=unary_union([Polygon(r) for p in slab['full_parts'] for r in model.part_regions(p,p['z1'])])
    expected=guest.intersection(support).difference(unary_union(cuts))
    zone=box(*case['washing_bounds'])
    assert b.intersection(guest).symmetric_difference(expected.intersection(zone)).area<1e-8
    assert a.intersection(guest).symmetric_difference(expected.difference(zone)).area<1e-8
    assert a.union(b).intersection(guest).symmetric_difference(expected).area<1e-8
    assert a.covers(Point(case['sleeping_probe'])) and b.covers(Point(case['washing_probe']))
    assert parquet['material']=='MAT-PARQUET' and tile['material']=='MAT-GRES-OCHRE'
    wardrobe=model._find(source,case['wardrobe'])
    cabinet=unary_union([Polygon(p['outline']) for p in wardrobe['geometry']['parts']])
    assert b.intersection(cabinet).area<1e-8
    assert b.bounds[0]>cabinet.bounds[2]


def test_garden_paving_and_grass_exactly_cover_the_real_floor():
    """🌱 Independently prove concrete and grass parts cover the garden without filling actual lightwell holes."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    geometry = model.building_geometry()
    terrace = Polygon(next(s for s in geometry["spaces"] if s["tag"] == "SP-TERRACE")["outline"])
    slab = next(s for s in geometry["slabs"] if s["tag"] == "SL-1")
    support=unary_union([Polygon(p["outline"],p.get("holes",[])) for p in slab["full_parts"]])
    columns=unary_union([Polygon(model.part_footprint(p)) for c in geometry["pilotis"] for p in c["parts"] if p["z0"]<3.17 and p["z1"]>3.14])
    floor = terrace.intersection(support).difference(unary_union([Polygon(v) for v in slab["voids"]]+[columns]))
    paving = next(e for e in geometry["details"] if e["tag"] == "FIN-TERRACE-PAVING")
    grass = next(e for e in geometry["details"] if e["tag"] == "FIN-TERRACE-GRASS")
    a = unary_union([Polygon(p["outline"]) for p in paving["parts"]])
    b = unary_union([Polygon(p["outline"]) for p in grass["parts"]])
    assert a.intersection(b).area < 1e-8
    assert a.union(b).symmetric_difference(floor).area < 1e-8,(a.union(b).area,floor.area,a.union(b).difference(floor).area,floor.difference(a.union(b)).area)
    assert a.area > b.area > 0
    assert math.isclose(a.area+b.area,floor.area,abs_tol=1e-8)


def test_finish_solids_are_physical_architectural_layers_below_finished_datums():
    """📏 Keep actual architectural wearing assemblies below finished datums and outside structural ownership."""
    source = model.load_model()
    levels = {s["id"]: s["elevation"]["value"] for s in source["storeys"]}
    structural = next(m for m in source["models"] if m["id"] == "structure")["element_tags"]
    finishes = [e for e in source["elements"] if e["class"]=="IfcCovering" and e.get("predefined_type")=="FLOORING"]
    assert len(finishes) >= 4
    for finish in finishes:
        assert finish["tag"] not in structural
        assert finish["geometry"]["base_level"] == finish["storey"]
        assert finish["class"]=="IfcCovering" and finish["predefined_type"]=="FLOORING"
        assert finish["metadata"]["domains"]==["architecture"] and "system" not in finish
        assert all(p["z0"]<p["z1"]<=0 for p in finish["geometry"]["parts"])
        assert max(p["z1"] for p in finish["geometry"]["parts"] if p["role"]=="wearing")==0


def test_kitchen_patio_is_a_real_open_air_room_and_roof_void():
    """☀️ Match the primary-plan small kitchen terrace with a genuine opening in the upper roof."""
    from shapely.geometry import LineString, Polygon
    geometry = model.building_geometry()
    patio = next(s for s in geometry["spaces"] if s["tag"] == "SP-KITCHEN-TERRACE")
    assert patio["predefined_type"] == "EXTERNAL"
    roof = next(s for s in geometry["slabs"] if s["tag"] == "SL-R")
    void = roof["voids"][roof["void_tags"].index("V-KITCHEN-PATIO")]
    assert Polygon(void).equals(Polygon(patio["outline"]))
    source=model.load_model()
    opening=model._find(source,model._find(source,"D-KITCHEN-PATIO")["geometry"]["opening"])
    host=model._find(source,opening["geometry"]["wall"])
    start,end=host["geometry"]["path"]
    length=math.dist(start,end);station=sum(opening["attributes"][k]["value"] for k in ("start","end"))/2
    center=[start[i]+station*(end[i]-start[i])/length for i in (0,1)]
    normal=[-(end[1]-start[1])/length,(end[0]-start[0])/length]
    crossing = LineString([[center[i]+d*normal[i] for i in (0,1)] for d in (-.6,.6)]).buffer(.4, cap_style=2)
    assert crossing.intersection(remodel._physical_wall_polygons(geometry, "first")).area < 1e-8
    assert any(o["fill_tag"] == "D-KITCHEN-PATIO" for w in geometry["walls"] for o in w["openings"])


def test_office_access_is_clear_between_hall_and_kitchen():
    """🍳 Verify a full-width route through the actual pantry/office doors and past fitted cupboards."""
    from shapely.geometry import LineString, Polygon
    geometry = model.building_geometry()
    source=model.load_model()
    case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf8"))["pantry_access"]
    route = LineString(case["path"]).buffer(case["body_width"]/2, cap_style=2)
    for tag in case["doors"]:
        filling=model.filling_geometry(tag,source)
        assert filling["clear_width"]>=case["body_width"]
        assert all(route.intersection(Polygon(model.part_footprint(p))).area<1e-8 for p in filling["parts"] if p["role"]=="frame" and p["z0"]<5.27 and p["z1"]>3.22)
    assert route.intersection(remodel._physical_wall_polygons(geometry, "first")).area < 1e-8
    cupboard = next(e for e in geometry["furnishings"] if e["tag"] == "FIX-PANTRY-STORAGE")
    assert all(route.intersection(Polygon(p["outline"])).area < 1e-8 for p in cupboard["parts"])
    rooms = {s["tag"]: Polygon(s["outline"]) for s in geometry["spaces"]}
    assert all(Polygon(p["outline"]).difference(rooms["SP-PANTRY"]).area<1e-8 for p in cupboard["parts"])
    from shapely.ops import unary_union
    portals=[Polygon(p["outline"]) for tag in case["doors"] for p in model.opening_geometry(model._find(source,tag)["geometry"]["opening"],source)["parts"]]
    permitted=unary_union([rooms[tag] for tag in case["rooms"]]+portals)
    assert route.difference(permitted).area<1e-8


def test_kitchen_patio_has_empty_facade_frame_instead_of_false_glass():
    """🪟 Preserve the empty patio frame and every source-supported independent ribbon segment."""
    source=model.load_model();geometry=model.building_geometry(source)
    case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['west_facade']
    wall=next(w for w in geometry['walls'] if w['tag']==case['wall'])
    frame=next(o for o in wall['openings'] if o['tag']==case['empty'])
    assert frame['fill_class'] is None and frame['fill_tag'] is None
    assert {o['fill_tag'] for o in wall['openings'] if o['fill_class']=='IfcWindow'}==set(case['windows'])
    assert not any(e['class'] in ('IfcWindow','IfcDoor') and e['geometry']['opening']==case['empty'] for e in source['elements'])
 