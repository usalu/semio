"""🛁 Source-evidenced built-in fixtures checked independently against actual solids."""
import importlib
import json
import math
from pathlib import Path

model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def _contract():
    return json.loads(Path(__file__).with_name("🛁️details.json").read_text(encoding="utf-8"))


def _slice(parts, z):
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    return unary_union([Polygon(p["outline"]) for p in parts if p["z0"] <= z <= p["z1"]])


def test_details_have_actual_hollow_and_occupied_volumes():
    """💧 Check free bowl volume above a solid bottom rather than accepting fixture names."""
    from shapely.geometry import Point
    registry = {e["tag"]: e for e in model.load_model()["elements"]}
    for case in _contract()["fixtures"]:
        parts = registry[case["tag"]]["geometry"]["parts"]
        for x, y, z in case.get("cavities", []):
            assert not _slice(parts, z).intersects(Point(x, y).buffer(.04)), case["tag"]
            assert _slice(parts, z).area > .02, case["tag"]
        for x, y, z in case.get("occupied", []):
            assert _slice(parts, z).covers(Point(x, y).buffer(.02)), case["tag"]


def test_detail_placement_and_independent_quantities():
    """📐 Verify room containment, real support clearance and NumPy volume parity."""
    import numpy as np
    from shapely.geometry import Point, Polygon
    from shapely.ops import unary_union
    geometry = model.building_geometry()
    registry = {e["tag"]: e for group in ("furnishings", "railings", "details") for e in geometry[group]}
    rooms = {e["tag"]: Polygon(e["outline"]) for e in geometry["spaces"]}
    for case in _contract()["fixtures"]:
        item = registry[case["tag"]]
        volume = 0
        for part in item["parts"]:
            polygon = Polygon(part["outline"])
            assert polygon.is_valid and rooms[case["room"]].buffer(1e-8).covers(polygon), case["tag"]
            xy = np.array(part["outline"])
            area = abs(np.dot(xy[:, 0], np.roll(xy[:, 1], 1)) - np.dot(xy[:, 1], np.roll(xy[:, 0], 1))) / 2
            assert math.isclose(area, polygon.area, abs_tol=1e-8)
            volume += polygon.area * (part["z1"] - part["z0"])
            for column in geometry["pilotis"]:
                if column["tag"] not in case.get("integrates", []):
                    for cp in column["parts"]:
                        if min(part["z1"],cp["z1"])-max(part["z0"],cp["z0"])>1e-9:
                            assert polygon.intersection(Polygon(cp["outline"])).area<1e-8,(case["tag"],column["tag"])
        levels=sorted({z for p in item["parts"] for z in [p["z0"],p["z1"]]})
        volume=sum(unary_union([Polygon(p["outline"]) for p in item["parts"] if p["z0"]<(lo+hi)/2<p["z1"]]).area*(hi-lo) for lo,hi in zip(levels,levels[1:]))
        assert math.isclose(volume, model.quantities(case["tag"])["volume"], abs_tol=1e-7),case["tag"]


def test_details_leave_person_routes_and_door_approaches_clear():
    """🚶 Check the source solids against body-width routes and real door approaches."""
    from shapely.geometry import LineString, Polygon
    geometry = model.building_geometry()
    furniture = geometry["furnishings"]
    for route in _contract()["routes"]:
        passage = LineString(route["points"]).buffer(route["width"] / 2, cap_style=2)
        for item in furniture:
            if item["storey"] == route["storey"]:
                assert all(passage.intersection(Polygon(p["outline"])).area < 1e-8 for p in item["parts"]), item["tag"]
    for wall in geometry["walls"]:
        for opening in wall["openings"]:
            if opening["fill_class"] != "IfcDoor":
                continue
            a, b = opening["p0"], opening["p1"]
            dx, dy = b[0] - a[0], b[1] - a[1]
            length = math.hypot(dx, dy)
            centre = ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)
            normal = (-dy / length * .45, dx / length * .45)
            passage = LineString([(centre[0] - normal[0], centre[1] - normal[1]), (centre[0] + normal[0], centre[1] + normal[1])]).buffer(min(.35, length / 2 - .01), cap_style=2)
            for item in furniture:
                if item["storey"] == wall["storey"]:
                    assert all(passage.intersection(Polygon(p["outline"])).area < 1e-8 for p in item["parts"]), (item["tag"], opening["fill_tag"])


def test_stair_guards_follow_flights_with_connected_tube_and_open_arrival():
    """🌀 Slice actual continuous U-guard meshes at each tread station using independent GEOS."""
    from shapely.geometry import Point,Polygon,MultiPoint
    from shapely.ops import unary_union
    geometry=model.building_geometry();registry={e["tag"]:e for e in geometry["railings"]}
    for case in _contract()["guards"]:
        stair=next(e for e in geometry["stairs"] if e["tag"]==case["stair"]);cx,cy=stair["centre"]
        assert stair["base"]==case["base"] and stair["top"]==case["top"]
        rise=(stair["top"]-stair["base"])/len(stair["steps"])
        for i,step in enumerate(stair["steps"]):
            if i<6:x,y=sum(p[0] for p in step["outline"])/len(step["outline"]),cy-stair["radius"]-.04
            elif i<12:
                angle=-math.pi/2+math.pi*(i-6+.5)/6;x,y=cx+(stair["radius"]+.04)*math.cos(angle),cy+(stair["radius"]+.04)*math.sin(angle)
            else:x,y=sum(p[0] for p in step["outline"])/len(step["outline"]),cy+stair["radius"]+.04
            z=stair["base"]+(i+.5)*rise+.5;slices=[]
            for part in registry[case["guard"]]["parts"]:
                if not part["z0"]<=z<=part["z1"]:continue
                vertices=part["mesh"]["vertices"];points=[]
                for face in part["mesh"]["faces"]:
                    for ia,ib in zip(face,face[1:]+face[:1]):
                        a,b=vertices[ia],vertices[ib]
                        if abs(a[2]-z)<1e-10:points.append(a[:2])
                        if (a[2]-z)*(b[2]-z)<0:
                            t=(z-a[2])/(b[2]-a[2]);points.append([a[k]+t*(b[k]-a[k]) for k in (0,1)])
                if len(points)>=3:slices.append(MultiPoint(points).convex_hull)
            assert unary_union(slices).buffer(1e-8).covers(Point(x,y)),(case["stair"],i)
        assert registry[case["tube"]]["parts"]


def test_guest_washing_bowl_is_actually_under_zenith_light():
    """🌤️ Compare the hollow basin with the registered clear opening, not rooflight material."""
    from shapely.geometry import Polygon,box
    case=json.loads(Path(__file__).with_name('🧺washfixtures.json').read_text(encoding='utf8'))['guest']
    source=model.load_model();clear=box(*case['clear_bounds'])
    slab=next(e for e in source['elements'] if e['tag']==case['slab'])
    aperture=next(v for v in slab['geometry']['voids'] if v['tag']==case['void'])
    assert Polygon(aperture['outline']).equals(clear)
    basin=model.solid_geometry(case['basin'],source)
    assert all(clear.covers(Polygon(p['outline'])) for p in basin['parts'])


def test_registered_wash_fixtures_keep_material_and_body_clearances():
    """🧺 Prove inferred fixture coordination with actual material and open-door body routes."""
    from shapely.geometry import Polygon,Point,LineString,box
    from shapely.ops import unary_union
    case=json.loads(Path(__file__).with_name('🧺washfixtures.json').read_text(encoding='utf8'));source=model.load_model()
    def find(tag):return next(e for e in source['elements'] if e['tag']==tag)
    def shape(p):return Polygon(p['outline'],p.get('holes',[]))
    def overlap(a,b):return shape(a).intersection(shape(b)).area*max(0,min(a['z1'],b['z1'])-max(a['z0'],b['z0']))
    columns=[p for e in source['elements'] if e['class']=='IfcColumn' for p in model.column_geometry(source,e)['parts']]
    rooms={s['tag']:Polygon(s['outline']) for s in source['spaces']}
    for group,storey,base in (('laundry','ground',0),('guest','first',3.17)):
        record=case[group];tags=record.get('tags',[record.get('basin'),record.get('tap')]);items={tag:model.solid_geometry(tag,source)['parts'] for tag in tags}
        walls=[p for e in source['elements'] if e['class']=='IfcWall' and e['storey']==storey for p in model._wall_geometry(source,e)['parts']]
        furniture={e['tag']:model.solid_geometry(e['tag'],source)['parts'] for e in source['elements'] if e['class']=='IfcFurnishingElement' and e['storey']==storey}
        basin_tag='FIX-LAUNDRY' if group=='laundry' else record['basin'];bounds=unary_union([shape(p) for p in items[basin_tag]]).bounds
        assert all(math.isclose(a,b,abs_tol=1e-10) for a,b in zip(bounds,record['bounds'])),basin_tag
        for tag,parts in items.items():
            assert all(rooms[record['room']].covers(shape(p)) for p in parts),tag
            assert sum(overlap(p,q) for p in parts for q in columns+walls)<case['tolerances']['volume'],tag
            assert sum(overlap(p,q) for p in parts for other,neighbours in furniture.items() if other!=tag for q in neighbours)<case['tolerances']['volume'],tag
            uncertainty=find(tag)['historical_evidence']['uncertainty'];assert set(uncertainty)=={'en','de'}
            assert 'inferred' in uncertainty['en'].lower() and 'abgeleitet' in uncertainty['de'].lower()
        door='D-LAUNDRY' if group=='laundry' else record['door'];fixed=[p for p in model.filling_geometry(door,source)['parts'] if p['role']=='frame']
        obstacles=columns+walls+fixed+[p for parts in furniture.values() for p in parts]
        passage=LineString(record['route']).buffer(record['body_width']/2,cap_style=2);standing=box(*record['standing_bounds'])
        assert rooms[record['room']].covers(standing)
        for domain in (passage,standing):
            assert sum(domain.intersection(shape(p)).area for p in obstacles if p['z0']<base+record['body_height'] and p['z1']>base)<case['tolerances']['area'],group
    opening=find(case['guest']['opening']);assert opening['geometry']['wall']==case['guest']['host']
    assert [opening['attributes'][k]['value'] for k in ('start','end')]==case['guest']['door_stations']
    assert math.isclose(model.filling_geometry(case['guest']['door'],source)['clear_width'],case['guest']['clear_width'])
    for tap,discharge in case['discharge_samples'].items():
        x,y,z=discharge['point'];basin=find(discharge['basin'])['geometry']['parts'];assert not _slice(basin,z).intersects(Point(x,y).buffer(.02)),tap
        projection=unary_union([Polygon(p['outline']) for p in find(tap)['geometry']['parts']]);assert projection.buffer(1e-10).covers(Point(x,y)),tap
