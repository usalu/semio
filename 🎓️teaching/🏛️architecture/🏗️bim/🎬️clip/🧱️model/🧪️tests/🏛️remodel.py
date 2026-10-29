"""🧪 Portable architectural remodel regression cases with independent polygon checks."""
import importlib
import json
import math
from pathlib import Path

model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_remodel_contract():
    """🏛️ Validate all portable remodel cases and the shared solid protocol."""
    source = model.load_model()
    geometry = model.building_geometry(source)
    cases = json.loads(Path(__file__).with_name("🏛️remodel.json").read_text(encoding="utf-8"))["cases"]
    registry = {e["tag"]: e for group in ("elements", "spaces") for e in source[group]}
    for case in cases:
        for tag in case.get("spaces", []) + case.get("stairs", []) + case.get("tags", []):
            assert tag in registry, tag
        if "storey" in case:
            assert all(registry[tag]["storey"] == case["storey"] for tag in case.get("spaces", []))
    assert model.quantities("SP-GARAGE", source)["area"] >= cases[0]["minimum_garage_area"]
    assert {o["fill_class"] for w in geometry["walls"] for o in w["openings"]} >= {None, "IfcDoor", "IfcWindow"}
    for wall in geometry["walls"]:
        assert wall["base"] < wall["top"]
        for opening in wall["openings"]:
            assert 0 <= opening["start"] < opening["end"] <= wall["length"]
            assert wall["base"] <= opening["sill_z"] < opening["head_z"] <= wall["top"]
    assert len(next(w for w in geometry["walls"] if w["tag"] == "W-G-W")["openings"]) == 3
    for stair in geometry["stairs"]:
        assert math.isclose(stair["steps"][-1]["z1"], stair["top"])
        assert all(step["z1"] > step["z0"] for step in stair["steps"])
        assert model.represent(stair["tag"], "3d", source)["primitives"]
    for ramp in geometry["ramp"]["ramps"]:
        assert len(ramp["landings"]) == 3
    for group in ("details", "furnishings", "railings"):
        assert geometry[group]
        for solid in geometry[group]:
            assert model.quantities(solid["tag"], source)["volume"] > 0
            assert all(p["z0"] < p["z1"] for p in solid["parts"])
    identities = [e["guid"] for e in registry.values()]
    assert len(identities) == len(set(identities))
    assert all(e["guid"] == model.global_id(e["tag"]) for e in registry.values())

def test_remodel_third_party_polygons():
    """📐 Compare every simple part and room with independent Shapely area calculations."""
    from shapely.geometry import Polygon
    source = model.load_model()
    geometry = model.building_geometry(source)
    for room in geometry["spaces"]:
        polygon = Polygon(room["outline"])
        assert polygon.is_valid, room["tag"]
        assert math.isclose(polygon.area, room["area"], abs_tol=1e-8)
    for group in ("stairs", "details", "furnishings", "railings"):
        for solid in geometry[group]:
            for part in solid.get("parts", solid.get("steps", [])):
                polygon = Polygon(part["outline"])
                assert polygon.is_valid and polygon.area > 0, solid["tag"]
                assert math.isclose(polygon.area, model._area(part["outline"]), abs_tol=1e-8)


def test_remodel_spaces_do_not_overlap():
    """🧩 Check first-party room boundaries with independent Shapely intersections."""
    from shapely.geometry import Polygon
    rooms = model.building_geometry()["spaces"]
    for index, a in enumerate(rooms):
        for b in rooms[index + 1:]:
            if a["storey"] == b["storey"]:
                assert Polygon(a["outline"]).intersection(Polygon(b["outline"])).area < 1e-7, (a["tag"], b["tag"])


def test_vehicle_sweep_against_independent_geometry():
    """🚙 Independently test the vehicle against every actual physical profile within its height envelope."""
    from shapely.geometry import Polygon
    source=model.load_model();route=model.car_path(source,samples=121);car=route['car']
    ground=next(s['elevation']['value'] for s in source['storeys'] if s['tag']=='ground')
    assert route['basis']=='assumed' and route['collision_free'] and route['certified_min_clearance']>0
    obstacles=model.vehicle_obstacles(source,ground,ground+car['height'])
    assert obstacles and all(o['z0']<ground+car['height'] and o['z1']>ground for o in obstacles)
    shapes=[(o,Polygon(o['outline'])) for o in obstacles]
    for x,y,heading in route['poses']:
        angle=math.radians(heading);c,s=math.cos(angle),math.sin(angle)
        vehicle=Polygon([(x+c*u-s*v,y+s*u+c*v) for u,v in [(-car['length']/2,-car['width']/2),(car['length']/2,-car['width']/2),(car['length']/2,car['width']/2),(-car['length']/2,car['width']/2)]])
        for obstacle,shape in shapes:
            assert vehicle.distance(shape)-obstacle.get('curve_error',0.)>=route['certified_min_clearance']-1e-6,obstacle['tag']


def _physical_wall_polygons(geometry, storey):
    from shapely.geometry import LineString
    from shapely.ops import unary_union
    polygons = []
    for wall in geometry["walls"]:
        if wall["storey"] != storey:
            continue
        polygon = LineString([wall["start"], wall["end"]]).buffer(wall["thickness"] / 2, cap_style=2)
        for opening in wall["openings"]:
            if opening["fill_class"] == "IfcDoor":
                polygon = polygon.difference(LineString([opening["p0"], opening["p1"]]).buffer(wall["thickness"] / 2 + .01, cap_style=2))
        polygons.append(polygon)
    return unary_union(polygons)


def test_private_corridor_supports_continuous_person_width():
    """🚶 Verify the actual office-side0.8m route around the measured stair position."""
    from shapely.geometry import LineString, Polygon
    from shapely.ops import unary_union
    geometry = model.building_geometry()
    rooms = [Polygon(s["outline"]) for s in geometry["spaces"] if s["tag"] in ("SP-FIRST-HALL", "SP-PANTRY")]
    slab = next(s for s in geometry["slabs"] if s["tag"] == "SL-1")
    floor = unary_union(rooms).difference(unary_union([Polygon(v) for v in slab["voids"]]))
    free = floor.difference(_physical_wall_polygons(geometry, "first")).buffer(-.4)
    route = LineString([(7.6,7),(7.6,7.25),(4.15,7.25),(4.15,11.4),(4.05,11.4),(4.05,15.2)])
    assert free.buffer(1e-8).covers(route)
    body = route.buffer(.4, cap_style=2)
    level = geometry["storeys"]["first"]
    assert all(body.intersection(shape).area < 1e-8 for _, shape in _column_shapes(geometry, level+.01, level+1.8))
    assert all(body.intersection(Polygon(part["outline"])).area < 1e-8 for item in geometry["furnishings"] if item["storey"] == "first" for part in item["parts"] if part["z1"] > level+.01)

def test_son_door_opens_to_public_hall():
    """🚪 Verify the son's two sides are his bedroom and public hall without a crossing partition."""
    from shapely.geometry import LineString, Point, Polygon
    geometry = model.building_geometry()
    rooms = {s["tag"]: Polygon(s["outline"]) for s in geometry["spaces"]}
    door = next(o for w in geometry["walls"] for o in w["openings"] if o["fill_tag"] == "D-SON")
    centre = tuple((door["p0"][i]+door["p1"][i])/2 for i in (0,1))
    dx,dy = door["p1"][0]-door["p0"][0],door["p1"][1]-door["p0"][1]
    length = math.hypot(dx,dy)
    sides = [(centre[0]-dy/length*.5,centre[1]+dx/length*.5),(centre[0]+dy/length*.5,centre[1]-dx/length*.5)]
    assert any(rooms["SP-SON"].contains(Point(p)) for p in sides)
    assert any(rooms["SP-FIRST-HALL"].contains(Point(p)) for p in sides)
    crossing = LineString(sides).buffer(.4, cap_style=2)
    assert crossing.intersection(_physical_wall_polygons(geometry, "first")).area < 1e-8


def test_parents_bath_has_private_lower_boundary():
    """🛁 Check the lower enclosure while preserving the source-attested open bathroom-to-bedroom path."""
    from shapely.geometry import LineString, Polygon
    geometry = model.building_geometry()
    wall = next(w for w in geometry["walls"] if w["tag"] == "W-PARENT-BATH-S")
    edge = LineString([(4.5, 11.8), (6.8, 11.8)])
    physical = LineString([wall["start"], wall["end"]]).buffer(wall["thickness"] / 2, cap_style=2)
    assert physical.covers(edge)
    crossing = LineString([(5.8, 16.8), (5.8, 18.1)]).buffer(.4, cap_style=2)
    assert crossing.intersection(_physical_wall_polygons(geometry, "first")).area < 1e-8
    fittings = [e for e in geometry["furnishings"] if e["tag"] in ("FIX-BATH", "FIX-CHAISE")]
    assert all(crossing.intersection(Polygon(p["outline"])).area < 1e-8 for fitting in fittings for p in fitting["parts"])


def test_flat_entry_bay_has_real_glass_interruption():
    """🌤️ Check the registered entrance with moving timber leaves assumed open, retaining fixed material."""
    from shapely.geometry import LineString,Polygon
    fixture=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['entrance_registration']
    ground=json.loads(Path(__file__).with_name(fixture['ground_fixture']).read_text(encoding='utf8'))
    assert all(point in ground['envelope'] for point in fixture['registered_portal'])
    source=model.load_model();geometry=model.building_geometry(source)
    door=next(e for e in source['elements'] if e['tag']==fixture['door'])
    opening=next(e for e in source['elements'] if e['tag']==door['geometry']['opening'])
    assert opening['tag']==fixture['opening']
    assert opening['geometry']['curtain_wall']==fixture['host']
    assert opening['geometry']['curtain_path']==fixture['path']
    curtain=geometry['curtain_wall'];assert curtain['tag']==fixture['host']
    assert any(path['tag']==fixture['path'] for path in curtain['paths'])
    centre=[sum(p[i] for p in fixture['registered_portal'])/2 for i in (0,1)]
    passage=LineString([[centre[0],centre[1]+offset] for offset in fixture['route_offsets']]).buffer(fixture['body_half_width'],cap_style=2)
    filling=next(e for e in geometry['fillings'] if e['tag']==fixture['door'])
    fixed=[p for p in filling['parts'] if p['role'] in fixture['fixed_roles']]
    parts=curtain['parts']+fixed+[p for w in geometry['walls'] if w['storey']=='ground' for p in w['parts']]
    assert sum(passage.intersection(Polygon(p['outline'],p.get('holes',[]))).area for p in parts if p['z0']<fixture['body_height'] and p['z1']>0)<fixture['area_tolerance']


def test_landscape_aperture_is_not_covered_by_curved_screen():
    """🖼️ Verify the actual hosted screen aperture instead of the removed assumed extra frame."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source=model.load_model();geometry=model.building_geometry(source)
    host=next(w for w in geometry["screens"] if w["tag"]=="SC-1")
    opening=next(o for o in host["openings"] if o["tag"]=="OP-SC-VIEW")
    aperture=unary_union([Polygon(p["outline"]) for p in opening["parts"]])
    assert aperture.area>0
    assert not any(e["geometry"].get("opening")=="OP-SC-VIEW" for e in source["elements"] if e["class"] in ["IfcDoor","IfcWindow"])
    z=(opening["sill_z"]+opening["head_z"])/2
    for screen in geometry["screens"]:
        for part in screen["parts"]:
            if part["z0"]<z<part["z1"]:
                assert aperture.intersection(Polygon(part["outline"])).area<1e-8


def test_salon_radiator_containment_and_retired_pipe_absence():
    """♨️ Keep the teaching radiator in its room without fabricating retired service branches."""
    from shapely.geometry import Polygon
    source=model.load_model();geometry=model.building_geometry(source)
    case=next(c for c in json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['cases'] if c['id']=='radiator-room-assignment')
    salon=Polygon(next(s for s in geometry['spaces'] if s['tag']==case['room'])['outline'])
    radiator=next(e for e in geometry['services'] if e['tag']==case['element'])
    assert all(salon.covers(Polygon(p['outline'],p.get('holes',[]))) for p in radiator['parts'])
    assert set(case['absent_retired_services']).isdisjoint(e['tag'] for e in source['elements'])
    assert set(case['absent_retired_services']).isdisjoint(e['tag'] for e in geometry['services'])


def test_bath_chaise_clears_actual_upper_column():
    """🪑 Check the actual tiled chaise against every measured upper column contour."""
    from shapely.geometry import Polygon
    geometry = model.building_geometry()
    chaise = next(e for e in geometry["furnishings"] if e["kind"] == "chaise")
    level = geometry["storeys"]["first"]
    assert all(Polygon(part["outline"]).intersection(profile).area < 1e-8 for part in chaise["parts"] for _, profile in _column_shapes(geometry, level, level+2))

def test_guest_door_full_body_width_and_public_approach():
    """🧳 Preserve the full guest doorway and source-correct left-wing public approach."""
    from shapely.geometry import LineString, Point, Polygon
    geometry = model.building_geometry()
    door = next(o for w in geometry["walls"] for o in w["openings"] if o["fill_tag"] == "D-GUEST")
    filling=next(f for f in geometry["fillings"] if f["tag"]=="D-GUEST")
    assert math.isclose(filling["clear_width"], .85)
    x,y = door["p0"][0], (door["p0"][1]+door["p1"][1])/2
    crossing = LineString([(x-.5,y),(4.05,y)]).buffer(.4,cap_style=2)
    assert crossing.intersection(_physical_wall_polygons(geometry,"first")).area < 1e-8
    slab = next(s for s in geometry["slabs"] if s["tag"] == "SL-1")
    assert all(crossing.intersection(Polygon(v)).area < 1e-8 for v in slab["voids"])
    hall = Polygon(next(s for s in geometry["spaces"] if s["tag"] == "SP-FIRST-HALL")["outline"])
    assert hall.covers(Point(4.05,y).buffer(.4))
    assert all(crossing.intersection(Polygon(p["outline"])).area<1e-8 for p in filling["parts"] if p["role"]=="frame" and p["z0"]<geometry["storeys"]["first"]+1.8)
    level = geometry["storeys"]["first"]
    assert all(crossing.intersection(shape).area < 1e-8 for _,shape in _column_shapes(geometry,level+.01,level+1.8))

def test_solarium_right_cap_bulges_outward():
    """🌀 Check the opposite outward solarium ends against the published CMN plan topology."""
    screen = next(s for s in model.building_geometry()["screens"] if s["tag"] == "SC-1")
    assert max(p[0] for p in screen["path"]) > 17
    assert min(p[0] for p in screen["path"]) < 2.2


def test_covered_garden_canopy_preserves_open_terrace():
    """⛱️ Distinguish the archive main roof opening from the separately owned covered rear kiosk."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    geometry = model.building_geometry()
    roof = next(s for s in geometry["slabs"] if s["tag"] == "SL-R")
    canopy = next(s for s in geometry["slabs"] if s["tag"] == "SL-KIOSK")
    cover = unary_union([Polygon(p["outline"]) for p in canopy["parts"]])
    assert sum(Polygon(p["outline"]).intersection(cover).area for p in roof["parts"]) < 1e-8
    expected = Polygon([[14.25,15.5],[19.15,15.5],[19.15,21.55],[14.25,21.55]])
    assert cover.symmetric_difference(expected).area < 1e-8
    opening = Polygon(roof["voids"][roof["void_tags"].index("V-TERRACE")])
    assert opening.equals(Polygon([[9.5,6.006],[19,6.006],[19,15.5],[9.5,15.5]]))
    assert cover.intersection(opening).area < 1e-8
    assert all(math.isclose(p["z0"],6.34) for p in canopy["parts"] if p["material"] != "MAT-RC")

def test_actual_column_profiles_bear_their_assigned_slabs():
    """🏗️ Check actual bearing intersections and preserve distinct wall/ramp support roles."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source = model.load_model()
    geometry = model.building_geometry(source)
    slabs = {s["tag"]:s for s in geometry["slabs"]}
    for column in geometry["pilotis"]:
        connections = [r for r in source["relations"] if r["class"] == "IfcRelConnectsElements" and (r["relating"] == column["tag"] or column["tag"] in r["related"])]
        assert connections, column["tag"]
        targets = [tag for r in connections if r["relating"] == column["tag"] for tag in r["related"] if tag in slabs]
        for tag in targets:
            slab=slabs[tag]
            authored=next(e for e in source['elements'] if e['tag']==tag)
            if column['tag'] in authored['geometry'].get('column_reserves',[]):
                overlaps=[Polygon(p['outline'],p.get('holes',[])).intersection(Polygon(s['outline'],s.get('holes',[]))).area*max(0,min(p['z1'],s['z1'])-max(p['z0'],s['z0'])) for p in column['parts'] for s in slab['full_parts']]
                assert sum(overlaps)>1e-7,(column['tag'],tag)
                assert any(p['owner']==column['tag'] for p in slab['shared_parts']),(column['tag'],tag)
            else:
                supports = [Polygon(p["outline"]).intersection(Polygon(s["outline"])).area for p in column["parts"] for s in slab["parts"] if abs(p["z1"]-s["z0"]) < 1e-6]
                assert supports and max(supports) > 1e-7,(column["tag"],tag)
    assert set(str(i) for i in range(1,34)).issubset(p["tag"] for p in geometry["pilotis"])

def test_planters_leave_two_real_garage_rooflights_clear():
    """🪴 Match source planters and glazing to actual first-floor slab holes above the garage."""
    from shapely.geometry import Polygon
    geometry = model.building_geometry()
    garage = Polygon(next(s for s in geometry["spaces"] if s["tag"] == "SP-GARAGE")["outline"])
    slab = next(s for s in geometry["slabs"] if s["tag"] == "SL-1")
    for index in (1, 2):
        void = Polygon(slab["voids"][slab["void_tags"].index(f"V-GARAGE-LIGHT-{index}")])
        light = next(d for d in geometry["details"] if d["tag"] == f"GL-GARAGE-{index}")
        assert garage.covers(void)
        assert Polygon(light["parts"][0]["outline"]).equals(void)
        planter = next(e for e in geometry["furnishings"] if e["tag"] == f"FIX-TERRACE-PLANTER-{index}")
        assert all(Polygon(p["outline"]).intersection(void).area < 1e-8 for p in planter["parts"])


def test_chauffeur_programme_partitions_the_real_diagonal_studio():
    """🛏️ Verify3 disjoint chauffeur functions and actual internal door approaches."""
    from shapely.geometry import LineString, Polygon
    from shapely.ops import unary_union
    geometry = model.building_geometry()
    tags = ("SP-CHAUFFEUR", "SP-CHAUFFEUR-BEDROOM", "SP-CHAUFFEUR-BATH")
    rooms = [Polygon(s["outline"]) for s in geometry["spaces"] if s["tag"] in tags]
    fixture=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['chauffeur_registration']
    ground=json.loads(Path(__file__).with_name(fixture['ground_fixture']).read_text(encoding='utf8'))
    envelope=unary_union([Polygon(ground['spaces'][tag]) for tag in tags])
    assert len(rooms) == 3
    assert sum(room.area for room in rooms)-unary_union(rooms).area < fixture['area_tolerance']
    assert unary_union(rooms).symmetric_difference(envelope).area < fixture['area_tolerance']
    for path in fixture['door_approaches']:
        assert LineString(path).buffer(fixture['body_half_width'],cap_style=2).intersection(_physical_wall_polygons(geometry,'ground')).area < fixture['area_tolerance']


def test_pantry_hatch_is_a_real_empty_hosted_opening():
    """🍽️ Verify an unfilled serving aperture on the shared kitchen/pantry wall and binary IFC relation."""
    from shapely.geometry import Point, Polygon
    source = model.load_model()
    geometry = model.building_geometry(source)
    host = next(w for w in geometry["walls"] if w["tag"] == "W-PANTRY-WEST")
    aperture = next(o for o in host["openings"] if o["tag"] == "OP-SERVING-HATCH")
    assert aperture["fill_class"] is None and aperture["sill"] == .9
    y = (aperture["p0"][1] + aperture["p1"][1]) / 2
    rooms = {s["tag"]: Polygon(s["outline"]) for s in geometry["spaces"]}
    assert rooms["SP-KITCHEN"].contains(Point(2.9, y))
    assert rooms["SP-PANTRY"].contains(Point(3.5, y))
    relations = [r for r in source["relations"] if r["class"] == "IfcRelVoidsElement" and "OP-SERVING-HATCH" in r["related"]]
    assert len(relations) == 1 and relations[0]["related"] == ["OP-SERVING-HATCH"]


def _column_shapes(geometry,low,high):
    """📐 Select only physical profile parts intersecting a given body-height range."""
    from shapely.geometry import Polygon
    for column in geometry["pilotis"]:
        for part in column["parts"]:
            if min(high,part["z1"])-max(low,part["z0"])>1e-8:
                yield column["tag"],Polygon(part["outline"])
