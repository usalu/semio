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
    """🪵 Verify the source parquet sleeping zone and tiled washing zone partition the true guest floor."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source = model.load_model()
    guest = Polygon(next(s for s in source["spaces"] if s["tag"] == "SP-GUEST")["outline"])
    parquet = next(e for e in source["elements"] if e["tag"] == "FIN-GUEST-PARQUET")
    tile = next(e for e in source["elements"] if e["tag"] == "FIN-GUEST-TILE")
    a = unary_union([Polygon(p["outline"]) for p in parquet["geometry"]["parts"]])
    b = unary_union([Polygon(p["outline"]) for p in tile["geometry"]["parts"]])
    assert a.intersection(b).area < 1e-8
    assert a.union(b).symmetric_difference(guest).area < 1e-8
    assert parquet["material"] == "MAT-PARQUET" and tile["material"] == "MAT-TILE"
    wardrobe = next(e for e in source["elements"] if e["tag"] == "FIX-GUEST-WARDROBE")
    north_edge = max(p[1] for part in wardrobe["geometry"]["parts"] for p in part["outline"])
    assert math.isclose(a.bounds[1], north_edge)


def test_garden_paving_and_grass_exactly_cover_the_real_floor():
    """🌱 Independently prove concrete and grass parts cover the garden without filling actual lightwell holes."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    geometry = model.building_geometry()
    terrace = Polygon(next(s for s in geometry["spaces"] if s["tag"] == "SP-TERRACE")["outline"])
    slab = next(s for s in geometry["slabs"] if s["tag"] == "SL-1")
    floor = terrace.difference(unary_union([Polygon(v) for v in slab["voids"]]))
    paving = next(e for e in geometry["details"] if e["tag"] == "FIN-TERRACE-PAVING")
    grass = next(e for e in geometry["details"] if e["tag"] == "FIN-TERRACE-GRASS")
    a = unary_union([Polygon(p["outline"]) for p in paving["parts"]])
    b = unary_union([Polygon(p["outline"]) for p in grass["parts"]])
    assert a.intersection(b).area < 1e-8
    assert a.union(b).symmetric_difference(floor).area < 1e-8
    assert a.area > b.area > 0
    assert math.isclose(a.area+b.area,floor.area,abs_tol=1e-8)


def test_finish_solids_are_nominal_slices_below_the_floor_datum():
    """📏 Keep nominal finish representations clear of door thresholds and excluded from structural material."""
    source = model.load_model()
    levels = {s["id"]: s["elevation"]["value"] for s in source["storeys"]}
    structural = next(m for m in source["models"] if m["id"] == "structure")["element_tags"]
    finishes = [e for e in source["elements"] if e["tag"].startswith("FIN-")]
    assert len(finishes) >= 4
    for finish in finishes:
        pset = finish["psets"]["Pset_TeachingRepresentation"]
        assert pset["NominalSurfaceRepresentation"]["value"] is True
        assert pset["IncludedInStructuralMaterial"]["value"] is False
        assert finish["tag"] not in structural
        assert finish["geometry"]["base_level"] == finish["storey"]
        assert all(p["z0"] < p["z1"] == 0 for p in finish["geometry"]["parts"])


def test_kitchen_patio_is_a_real_open_air_room_and_roof_void():
    """☀️ Match the primary-plan small kitchen terrace with a genuine opening in the upper roof."""
    from shapely.geometry import LineString, Polygon
    geometry = model.building_geometry()
    patio = next(s for s in geometry["spaces"] if s["tag"] == "SP-KITCHEN-TERRACE")
    assert patio["predefined_type"] == "EXTERNAL"
    roof = next(s for s in geometry["slabs"] if s["tag"] == "SL-R")
    void = roof["voids"][roof["void_tags"].index("V-KITCHEN-PATIO")]
    assert Polygon(void).equals(Polygon(patio["outline"]))
    crossing = LineString([(2.15,5.6),(2.15,6.8)]).buffer(.4, cap_style=2)
    assert crossing.intersection(remodel._physical_wall_polygons(geometry, "first")).area < 1e-8
    assert any(o["fill_tag"] == "D-KITCHEN-PATIO" for w in geometry["walls"] for o in w["openings"])


def test_office_access_is_clear_between_hall_and_kitchen():
    """🍳 Verify a full-width route through the actual pantry/office doors and past fitted cupboards."""
    from shapely.geometry import LineString, Polygon
    geometry = model.building_geometry()
    route = LineString([(5.3,7.25),(4.15,7.25),(4.15,5.5),(3.925,5.2),(3.925,4.1)]).buffer(.4, cap_style=2)
    assert route.intersection(remodel._physical_wall_polygons(geometry, "first")).area < 1e-8
    cupboard = next(e for e in geometry["furnishings"] if e["tag"] == "FIX-PANTRY-STORAGE")
    assert all(route.intersection(Polygon(p["outline"])).area < 1e-8 for p in cupboard["parts"])
    rooms = {s["tag"]: Polygon(s["outline"]) for s in geometry["spaces"]}
    assert all(rooms["SP-PANTRY"].covers(Polygon(p["outline"])) for p in cupboard["parts"])


def test_kitchen_patio_has_empty_facade_frame_instead_of_false_glass():
    """🪟 Distinguish the open-air patio view frame from neighbouring real ribbon glazing."""
    geometry = model.building_geometry()
    wall = next(w for w in geometry["walls"] if w["tag"] == "W-W")
    frame = next(o for o in wall["openings"] if o["tag"] == "OP-KITCHEN-PATIO-FRAME")
    assert frame["fill_class"] is None
    assert len([o for o in wall["openings"] if o["fill_class"] == "IfcWindow"]) == 2
