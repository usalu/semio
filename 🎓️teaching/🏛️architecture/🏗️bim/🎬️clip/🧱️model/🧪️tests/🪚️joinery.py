"""🪚 Independent contracts for physical frame, sash and sliding-track joinery."""
import importlib
import json
from pathlib import Path

model=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def test_sashes_and_mullions_occupy_glass_divisions_without_intersection():
    """🧩 Compare source-neutral nonuniform panel boundaries and materials using GEOS."""
    from shapely.geometry import Polygon
    source=model.load_model()
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    spec=next(t["filling"] for t in source["types"] if t["tag"]=="TY-RIBBON")
    spec["panel_stations"]=[{"value":v,"unit":"1","basis":"assumed","source":"Joinery-contract"} for v in case["stations"]]
    for key in ("sash_width","sash_depth","mullion_width"):
        spec[key]={"value":case[key],"unit":"m","basis":"assumed","source":"Joinery-contract"}
    filling=next(e for e in source["elements"] if e.get("type")=="TY-RIBBON")
    geometry=model._filling_geometry(source,filling)
    parts=geometry["parts"]
    assert len([p for p in parts if p["role"]=="mullion"])==len(case["stations"])-2
    assert len([p for p in parts if p["role"]=="sash"])==4*(len(case["stations"])-1)
    glass=[p for p in parts if p["role"]=="glass"]
    assert len(glass)==len(case["stations"])-1
    for p in glass:
        assert len(p["outline"])==4
        for q in parts:
            if q is p:
                continue
            height=min(p["z1"],q["z1"])-max(p["z0"],q["z0"])
            assert height<=0 or Polygon(p["outline"]).intersection(Polygon(q["outline"])).area*height<case["tolerance"]
    assert all(p["material"]=="MAT-WOOD" for p in parts if p["role"] in ("sash","mullion"))


def test_salon_has_hollow_two_lane_tracks_and_real_sashes():
    """🛤️ Confirm physical track webs and lane guides rather than an operation-only label."""
    source=model.load_model()
    filling=next(e for e in source["elements"] if e.get("type")=="TY-TERRACE-GLAZING")
    geometry=model._filling_geometry(source,filling)
    parts=geometry["parts"]
    assert any(p["role"]=="sash" for p in parts)
    assert len([p for p in parts if p["role"]=="track_web"])==2
    assert len([p for p in parts if p["role"]=="track_guide"])==6
    assert all(p["material"]=="MAT-METAL" for p in parts if p["role"].startswith("track_"))


def test_source_glazed_joinery_parts_and_quantities_agree_with_geos():
    """📦 Independently integrate all source planar glazing solids and material intersections."""
    from shapely.geometry import Polygon
    source=model.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());seen=set()
    for filling in source["elements"]:
        if filling["class"] not in ("IfcDoor","IfcWindow"):
            continue
        geometry=model._filling_geometry(source,filling)
        parts=geometry["parts"]
        if not any(p["role"]=="glass" for p in parts):continue
        seen.add(filling["type"])
        volume=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in parts)
        assert abs(model.quantities(filling["tag"],source)["volume"]-volume)<1e-8
        for i,p in enumerate(parts):
            for q in parts[i+1:]:
                height=min(p["z1"],q["z1"])-max(p["z0"],q["z0"])
                assert height<=0 or Polygon(p["outline"]).intersection(Polygon(q["outline"])).area*height<1e-9
    assert set(case['glazed_types'])<=seen
    witness=case['glazed_leaf'];door=next(e for e in source['elements'] if e['tag']==witness['tag'])
    assert door['type']==witness['type']
    spec=next(t['filling'] for t in source['types'] if t['tag']==door['type'])
    assert [p['value'] for p in spec['panel_stations']]==witness['panel_stations']
    parts=model._filling_geometry(source,door)['parts']
    sashes=[p for p in parts if p['role']=='sash']
    assert len(sashes)==witness['sash_parts']
    assert len([p for p in parts if p['role']=='mullion'])==witness['mullion_parts']
    assert all(p['material']==witness['sash_material'] for p in sashes)
