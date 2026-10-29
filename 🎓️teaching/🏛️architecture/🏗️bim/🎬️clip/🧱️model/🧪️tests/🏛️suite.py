"""🛌 Source-backed parents alcove and independent private/public circulation checks."""
import importlib
import json
import math
from pathlib import Path

model=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
remodel=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️remodel")


def _case():
    return json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))


def test_alcove_uses_registered_circular_posts_and_existing_frame():
    """🪵 Use actual27/32 upper columns and the registered frame rather than fabricated stilts."""
    source,case=model.load_model(),_case()
    elements={e["tag"]:e for e in source["elements"]}
    types={e["tag"]:e for e in source["types"]}
    assert all(types[elements[t]["type"]]["profile"]["shape"]=="circle" for t in case["alcove"]["posts"])
    assert elements[case["alcove"]["beam"]]["class"]=="IfcBeam"
    assert not any(e["tag"].startswith("ALCOVE-P") for e in source["elements"])


def test_parent_room_contains_full_registered_alcove_bay():
    """🛏️ Keep the bed-head bay between existing posts inside the bedroom, clear of false crosswalls."""
    from shapely.geometry import Polygon,box
    source,case=model.load_model(),_case()
    geometry=model.building_geometry(source)
    posts={e["tag"]:e for e in geometry["pilotis"]}
    low,high=[posts[t] for t in case["alcove"]["posts"]]
    y0=low["y"]+.12;y1=high["y"]-.12
    bay=box(4.38,y0,4.85,y1)
    room=Polygon(next(s for s in geometry["spaces"] if s["tag"]==case["alcove"]["room"])["outline"])
    assert room.buffer(1e-8).covers(bay)
    assert bay.intersection(remodel._physical_wall_polygons(geometry,"first")).area<1e-8


def test_parents_bath_opens_physically_into_bedroom():
    """🛁 Preserve the source-attested open bathroom connection instead of a sealed masonry cell."""
    from shapely.geometry import LineString
    source,case=model.load_model(),_case()
    geometry=model.building_geometry(source)
    passage=LineString(case["bath_bed_route"]).buffer(case["body_width"]/2,cap_style=2)
    assert passage.intersection(remodel._physical_wall_polygons(geometry,"first")).area<1e-8
    assert all(passage.intersection(shape).area<1e-8 for _,shape in remodel._column_shapes(geometry,3.17,5.27))


def test_son_public_route_does_not_cross_parents_suite():
    """🚶 Require a full0.8m physical son-bedroom approach independent of the private suite."""
    from shapely.geometry import LineString,Polygon
    from shapely.ops import unary_union
    source,case=model.load_model(),_case()
    geometry=model.building_geometry(source)
    passage=LineString(case["public_route"]).buffer(case["body_width"]/2,cap_style=2)
    spaces={s["tag"]:Polygon(s["outline"]) for s in geometry["spaces"] if s["storey"]=="first"}
    permitted=unary_union([spaces[t] for t in ["SP-FIRST-HALL","SP-PANTRY","SP-SON"]])
    assert permitted.buffer(1e-8).covers(passage)
    private=unary_union([spaces["SP-PARENTS"],spaces["SP-PARENT-BATH"]])
    assert passage.intersection(private).area<1e-8
    for filling in geometry["fillings"]:
        if filling["class"]=="IfcDoor" and filling["storey"]=="first":
            for part in filling["parts"]:
                if part["role"]=="frame" and part["z0"]<geometry["storeys"]["first"]+1.8:
                    assert passage.intersection(Polygon(part["outline"])).area<1e-8,filling["tag"]
    assert passage.intersection(remodel._physical_wall_polygons(geometry,"first")).area<1e-8
    assert all(passage.intersection(shape).area<1e-8 for _,shape in remodel._column_shapes(geometry,3.17,5.27))


def test_upper_stair_arrival_is_supported_and_clear_of_its_guards():
    """🌀 Check real landing support and0.8m body clearance through the actual stair exit."""
    from shapely.geometry import LineString,Polygon,MultiPoint
    from shapely.ops import unary_union
    geometry=model.building_geometry()
    for tag in ["ST-GF","ST-FR"]:
        stair=next(e for e in geometry["stairs"] if e["tag"]==tag)
        x,y=stair["centre"]
        going=.25
        west=x-6*going
        middle=y+(stair["radius"]+stair["inner_radius"])/2
        route=LineString([(west+going/2,middle),(west-.6,middle)])
        feet=route.buffer(.04,cap_style=2)
        support=unary_union([Polygon(p["outline"]) for p in stair["steps"]+stair["landings"] if p["z1"]>=stair["top"]-.01])
        slab=next(p for p in geometry["slabs"] if p["tag"]==("SL-1" if tag=="ST-GF" else "SL-R"))
        local={}
        for part in slab["parts"]:
            ps=part["outline"];xs=[p[0] for p in ps];ys=[p[1] for p in ps]
            if max(xs)<feet.bounds[0] or min(xs)>feet.bounds[2] or max(ys)<feet.bounds[1] or min(ys)>feet.bounds[3]:continue
            local.setdefault(tuple(sorted(tuple(p) for p in ps)),ps)
        floor=unary_union([Polygon(ps) for ps in local.values()])
        assert support.union(floor).buffer(1e-8).covers(feet)
        body=route.buffer(.4,cap_style=2)
        for rail in geometry["railings"]:
            if rail["tag"] not in ["GUARD-"+tag,"TUBE-"+tag]:continue
            for p in rail["parts"]:
                if p["z1"]>stair["top"]+.05 and p["z0"]<stair["top"]+1.8:
                    assert body.intersection(MultiPoint(p["outline"]+p.get("top_outline",[])).convex_hull).area<1e-8,(tag,rail["tag"])


def test_u_handrails_have_connected_physical_sections_and_nominal_volume():
    """🔗 Check the complete U handrail against GEOS caps and independent NumPy volume."""
    import numpy as np
    from shapely.geometry import Polygon
    geometry=model.building_geometry()
    rails={e["tag"]:e for e in geometry["railings"]}
    for tag in ["TUBE-ST-GF","TUBE-ST-FR"]:
        parts=rails[tag]["parts"]
        oracle=0.;length=0.
        for p in parts:
            bottom,top=np.array(p["outline"]),np.array(p["top_outline"])
            assert np.allclose(top-bottom,(top-bottom)[0],atol=1e-10)
            area=abs(np.dot(bottom[:,0],np.roll(bottom[:,1],1))-np.dot(bottom[:,1],np.roll(bottom[:,0],1)))/2
            dz=p["z1"]-p["z0"]
            oracle+=area*dz
            delta=top.mean(axis=0)-bottom.mean(axis=0)
            length+=np.linalg.norm(np.r_[delta,dz])
        assert abs(oracle/(math.pi*.0175**2*length)-1)<.012
        assert math.isclose(model._occupied_volume(parts),oracle,rel_tol=1e-8,abs_tol=1e-9)
        for a,b in zip(parts,parts[1:]):
            assert math.isclose(a["z1"],b["z0"],abs_tol=1e-9)
            assert Polygon(a["top_outline"]).intersection(Polygon(b["outline"])).area>1e-6
  