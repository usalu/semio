"""🏠 Registered ground enclosure, spaces, access and fixture contracts."""
import importlib
import json
import math
from pathlib import Path

model=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def _polygon(source,outline):
    """📐 Independently expand declared cubic controls before checking the native outline interface."""
    import numpy as np
    from shapely.geometry import Polygon
    points=[]
    for item in outline:
        if isinstance(item,dict):
            assert set(item)=={'bezier'}
            controls=np.asarray(item['bezier'],dtype=float)
            for t in np.linspace(0,1,33):
                work=controls.copy()
                for count in (3,2,1):work=(1-t)*work[:count]+t*work[1:count+1]
                points.append(work[0].tolist())
        else:points.append(item)
    expected=Polygon(points);actual=Polygon(model._expand(source,outline))
    assert expected.is_valid and actual.is_valid
    assert expected.symmetric_difference(actual).area<1e-10
    assert expected.hausdorff_distance(actual)<1e-12
    return actual


def test_ground_paths_match_independent_registered_controls():
    """📍 Preserve registered controls rather than an assumed motorcar radius."""
    source=model.load_model()
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    import jsonschema
    jsonschema.Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(case)
    for curve in case["curve_cases"]:
        assert _polygon(source,curve["outline"]).covers(__import__("shapely.geometry",fromlist=["Point"]).Point(curve["midpoint"]))
    assert not set(case["removed_walls"])&{e["tag"] for e in source["elements"]}
    for tag,expected in case["walls"].items():
        assert model._find(source,tag)["geometry"]["path"]==expected,tag
    for tag,expected in case["openings"].items():
        opening=model._find(source,tag)
        assert [opening["attributes"][k]["value"] for k in ("start","end")]==expected,tag


def test_ground_spaces_are_valid_distinct_and_contain_registered_basin_and_hoist():
    """🧭 Independent GEOS verifies physical room ownership and contained hall fittings."""
    from shapely.geometry import Polygon,LineString
    from shapely.ops import unary_union
    source=model.load_model()
    spaces={s["tag"]:_polygon(source,s["outline"]) for s in source["spaces"] if s["storey"]=="ground"}
    assert set(spaces)==set(json.loads(Path(__file__).with_suffix(".json").read_text())["expected_spaces"])
    for tag,shape in spaces.items():
        assert shape.is_valid and shape.area>0,tag
    pairs=list(spaces.items())
    for i,(tag,shape) in enumerate(pairs):
        for other,other_shape in pairs[i+1:]:
            assert shape.intersection(other_shape).area<1e-8,(tag,other)
    basin=model._find(source,"FIX-HALL-BASIN")
    for part in basin["geometry"]["parts"]:
        assert spaces["SP-HALL"].covers(Polygon(part["outline"])),"basin outside hall"
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    enclosure=Polygon(case["envelope"])
    assert unary_union(list(spaces.values())).symmetric_difference(enclosure).area<1e-8
    for tag,shape in spaces.items():
        assert abs(shape.area-model._area(model._expand(source,next(s for s in source["spaces"] if s["tag"]==tag)["outline"])))<1e-8
    hatch=Polygon(model._find(source,"OP-HOIST-HATCH")["geometry"]["outline"])
    assert spaces["SP-GARAGE"].covers(hatch),"registered hatch outside garage"
    access=case["hoist_approach"]
    route=LineString(access["path"]).buffer(access["body_width"]/2,cap_style=2)
    assert spaces["SP-GARAGE"].covers(route)
    for element in source["elements"]:
        if element["storey"]!="ground":
            continue
        parts=[]
        if element["class"]=="IfcWall" and all(isinstance(p,list) for p in element["geometry"].get("path",[])):
            wall=LineString(element["geometry"]["path"]).buffer(model._wall_thickness(source,element)/2,cap_style=2)
            assert route.intersection(wall).area<1e-8,element["tag"]
        if element["class"]=="IfcColumn":
            parts=model.column_geometry(source,element)["parts"]
        elif element["class"] in ("IfcFurnishingElement","IfcRailing","IfcBuildingElementProxy"):
            parts=model.solid_geometry(element["tag"],source)["parts"] if "parts" in element["geometry"] else []
        elif element["class"] in ("IfcChimney","IfcPipeSegment"):
            parts=model.service_geometry(element["tag"],source)["parts"]
        for part in parts:
            if part["z0"]<access["body_height"] and part["z1"]>0:
                assert route.intersection(Polygon(model.part_footprint(part))).area<1e-8,element["tag"]



def test_ground_basin_registration_and_garage_portal_are_physically_consistent():
    """🚰 Compare authored geometry with registered position and independently bounded portal."""
    from shapely.geometry import Polygon,box
    from shapely.ops import unary_union
    source=model.load_model()
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    basin=model._find(source,case["basin"]["tag"])
    assert math.dist(basin["geometry"]["axis"],case["basin"]["centre"])<1e-8
    points=[v for part in model.service_geometry(basin["tag"],source)["parts"] if part["role"] in ("basin_floor","basin_shell") for v in model.part_vertices(part)]
    centre=[(min(p[i] for p in points)+max(p[i] for p in points))/2 for i in (0,1)]
    assert math.dist(centre,case["basin"]["centre"])<1e-8
    host=model._wall_geometry(source,model._find(source,case["portal"]["host"]))
    occupied=unary_union([Polygon(p["outline"]) for p in host["parts"] if p["z0"]<1<p["z1"]])
    portal=box(15.926,*[case["portal"]["world_y"][0]],16.074,case["portal"]["world_y"][1])
    assert occupied.intersection(portal).area<1e-8
    garage=_polygon(source,next(s for s in source["spaces"] if s["tag"]=="SP-GARAGE")["outline"])
    assert garage.covers(box(15.8,10.85,15.9,14.8))


def test_ground_registered_room_fittings_and_private_portals_remain_accessible():
    """🚪 Verify each moved fitting is contained and door crossing reaches the intended private room."""
    from shapely.geometry import Polygon,LineString,Point
    from shapely.ops import unary_union
    source=model.load_model()
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    spaces={s["tag"]:_polygon(source,s["outline"]) for s in source["spaces"] if s["storey"]=="ground"}
    for tag,room in case["fixture_spaces"].items():
        fixture=unary_union([Polygon(model.part_footprint(part)) for part in model.solid_geometry(tag,source)["parts"]])
        assert spaces[room].covers(fixture),tag
    for item in case["room_access"]:
        door=model._find(source,item["door"])
        opening=model._find(source,door["geometry"]["opening"])
        wall=model._find(source,opening["geometry"]["wall"])
        a,b=wall["geometry"]["path"]
        length=math.dist(a,b)
        tangent=[(b[i]-a[i])/length for i in (0,1)]
        distance=sum(opening["attributes"][k]["value"] for k in ("start","end"))/2
        centre=[a[i]+distance*tangent[i] for i in (0,1)]
        normal=[-tangent[1],tangent[0]]
        ends=[[centre[i]+sign*.6*normal[i] for i in (0,1)] for sign in (-1,1)]
        crossing=LineString(ends).buffer(.30,cap_style=2)
        occupied=unary_union([Polygon(p["outline"]) for p in model._wall_geometry(source,wall)["parts"] if p["z0"]<1<p["z1"]])
        assert occupied.intersection(crossing).area<1e-8,item["door"]
        assert any(spaces[item["room"]].covers(Point(p)) for p in ends),item["door"]
        for other in source["elements"]:
            if other["storey"]!="ground" or other["tag"]==wall["tag"]:
                continue
            if other["class"]=="IfcWall" and all(isinstance(p,list) for p in other["geometry"].get("path",[])):
                shape=LineString(other["geometry"]["path"]).buffer(model._wall_thickness(source,other)/2,cap_style=2)
                assert crossing.intersection(shape).area<1e-8,(item["door"],other["tag"])
            if other["class"]=="IfcFurnishingElement":
                for part in model.solid_geometry(other["tag"],source)["parts"]:
                    if part["z0"]<1.8 and part["z1"]>0:
                        assert crossing.intersection(Polygon(model.part_footprint(part))).area<1e-8,(item["door"],other["tag"])
        filling=model._filling_geometry(source,door)
        assert filling["clear_width"]>=.8,item["door"]
        for part in filling["parts"]:
            if part["role"]=="frame" and part["z0"]<1<part["z1"]:
                assert Polygon(part["outline"]).intersection(crossing).area<1e-8,item["door"]


def test_ground_plate_preserves_cellar_ownership_and_registered_apertures():
    """🧱 Ground plate fills registered enclosure without filling cellar access or double-owning concrete."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source=model.load_model()
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    spec=case["plate"]
    slab=model._find(source,spec["tag"])
    cellar=model._find(source,spec["cellar_owner"])
    assert slab["material"]==spec["material"]
    material=model._find(source,spec["material"])
    assert material["density"]["value"]==spec["density"]
    assert material["density"]["basis"]=="assumed"
    assert slab["psets"]["Pset_GroundSubstrate"]["Reinforced"]["value"] is False
    assert slab["geometry"]["outline"]==spec["outline"]
    assert slab["geometry"]["regions"][0]["thickness"]["value"]==spec["thickness"]
    geometry=model._slab_geometry(source,slab)
    from shapely import intersection
    from shapely.strtree import STRtree
    polygons=[Polygon(p["outline"]) for p in geometry["parts"]]
    voids=[Polygon(v["outline"]) for v in slab["geometry"]["voids"]]
    expected=Polygon(spec["outline"]).difference(unary_union(voids))
    assert all(p.is_valid and p.area>0 for p in polygons)
    assert abs(sum(p.area for p in polygons)-expected.area)<1e-8
    assert abs(sum(model._area(p["outline"]) for p in geometry["parts"])-expected.area)<1e-8
    tree=STRtree(polygons)
    overlap_area=0.0
    for i,polygon in enumerate(polygons):
        assert polygon.difference(expected).area<1e-9
        for j in tree.query(polygon):
            if j<=i:
                continue
            other=polygons[j]
            precision=min(1e-10,1e-10/(polygon.length+other.length+1))
            bound=(polygon.length+other.length)*precision
            assert bound<1e-9
            overlap=intersection(polygon,other,grid_size=precision).area
            assert overlap<=bound+1e-8
            overlap_area+=overlap
    source_rounding_bound=sum(p.length for p in polygons)*math.sqrt(.5)*1e-12
    assert overlap_area<=source_rounding_bound
    assert overlap_area<1e-8
    cellar_gross=Polygon(cellar["geometry"]["outline"])
    assert all(p.intersection(cellar_gross).area<1e-9 for p in polygons)
    for tag in spec["preserve_openings"]:
        opening=model._find(source,tag)
        assert opening["geometry"]["slab"]==spec["cellar_owner"]
        assert all(p.intersection(Polygon(opening["geometry"]["outline"])).area<1e-9 for p in polygons)



def test_rejected_garage_hall_door_is_absent_and_did_intersect_the_source_ramp():
    """🚧 Preserve the source ramp and remove unsupported internal access rather than fabricate another door."""
    from shapely.geometry import Polygon,LineString
    source=model.load_model()
    case=json.loads(Path(__file__).with_suffix(".json").read_text())["removed_unsupported_access"]
    for tag in case["tags"]:
        assert tag not in {e["tag"] for e in source["elements"]}
        assert not any(tag in r.get("related",[]) or r.get("relating")==tag for r in source["relations"])
        assert not any(tag in m["element_tags"] for m in source["models"])
    route=LineString(case["rejected_route"]).buffer(case["body_width"]/2,cap_style=2)
    flight=model.ramp_geometry(case["ramp"],source)
    from shapely.ops import unary_union
    occupied=unary_union([Polygon(ring) for part in flight["parts"] if part["z0"]<1.8 and part["z1"]>0 for ring in model.part_band_footprints(part,0.,1.8)])
    actual=route.intersection(occupied).area
    assert abs(actual-case["expected_overlap_area"])<1e-8


def test_hall_gap_remains_source_bound_without_certifying_800mm_passage():
    """📏 Check qualified thin-wall construction, actual proud boards and retained fixed basin controls."""
    from shapely.geometry import Polygon,LineString
    source=model.load_model();case=json.loads(Path(__file__).with_suffix('.json').read_text());circulation=case['northern_circulation'];control=circulation['critical_clearance'];partition=circulation['partition_reconstruction'];host=model._find(source,control['wall_tag'])
    assert host['material']==partition['material'] and host['type']==partition['type']
    assert host['attributes']['thickness']['basis']=='inferred' and host['attributes']['thickness']['value']==partition['thickness']
    assert all(point[0]==partition['axis_x'] for point in host['geometry']['path'])
    material=model._find(source,host['material']);assert math.isclose(math.fsum(layer['thickness']['value'] for layer in material['layers']),partition['thickness'],abs_tol=1e-12)
    section=LineString([[6.8,control['axis_y']],[8.4,control['axis_y']]])
    walls=[Polygon(p['outline']) for p in model._wall_geometry(source,host)['parts'] if p['z0']<.7<p['z1']];sections=[shape.intersection(section) for shape in walls if shape.intersects(section)]
    east=max(shape.bounds[2] for shape in sections);cabinet_side=min(shape.bounds[0] for shape in sections)
    basin=model.service_geometry(control['basin_tag'],source)['parts'];west=min(v[0] for p in basin for v in model.part_vertices(p))
    assert model._find(source,control['basin_tag'])['geometry']['axis']==case['basin']['centre']
    stock=model.solid_geometry(control['skirting_tag'],source)['parts'];board_sections=[Polygon(ring).intersection(section) for part in stock for ring in model.part_band_footprints(part,.05,.06) if Polygon(ring).intersects(section)];board_east=max(shape.bounds[2] for shape in board_sections)
    for actual,expected in ((east,control['actual_east_finish_face_x']),(cabinet_side,partition['retained_cabinet_face_x']),(west,control['actual_west_edge_x']),(west-east,control['nominal_gap']),(board_east,control['actual_east_skirting_face_x']),(west-board_east,control['board_nominal_gap'])):assert math.isclose(actual,expected,abs_tol=1e-9),(actual,expected)
    accepted=json.loads(Path(__file__).with_name(circulation['fixture']).read_text());assert west-board_east>max(row['diameter'] for row in accepted['cases'])+2*accepted['radial_margin']
