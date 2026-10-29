"""🏗️ Registered foundation coverage and independent solid-geometry oracles."""
import importlib
import json
import math
from pathlib import Path

model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def _case():
    return json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))


def test_foundations_cover_actual_exterior_supports_once():
    """🧭 Require traced pads for22 exterior supports and no duplicates for11 cellar supports."""
    source, case = model.load_model(), _case()
    pads = {e["tag"]: e for e in source["elements"] if e["tag"].startswith("FT-PAD-")}
    blindings = {e["tag"]: e for e in source["elements"] if e["tag"].startswith("FT-BLINDING-")}
    assert set(pads) == {"FT-PAD-"+p for p in case["expected_pad_ids"]}
    assert set(blindings) == {"FT-BLINDING-"+p for p in case["expected_pad_ids"]}
    coverage = []
    for pad in case["expected_pad_ids"]:
        rel = next(r for r in source["relations"] if r["tag"] == "REL-FOUNDATION-SUPPORT-"+pad)
        assert rel["relating"] == "FT-PAD-"+pad
        coverage.extend(rel["related"])
    assert sorted(coverage) == sorted(case["exterior_columns"])
    assert len(coverage) == len(set(coverage)) == 22
    assert set(coverage).isdisjoint(case["cellar_supported"])
    assert set(coverage) | set(case["cellar_supported"]) == {str(i) for i in range(1,34)}
    combined = pads["FT-PAD-"+case["combined_pad"]]
    assert len(combined["geometry"]["parts"]) == 4


def test_foundation_materials_and_vertical_contacts_are_physical():
    """🪨 Check separate lime andRC solids and exact base-to-pyramid-to-stem contact."""
    from shapely.geometry import Polygon
    source, case = model.load_model(), _case()
    elements = {e["tag"]:e for e in source["elements"]}
    for pad_id in case["expected_pad_ids"]:
        blind = elements["FT-BLINDING-"+pad_id]
        pad = elements["FT-PAD-"+pad_id]
        assert blind["material"] == "MAT-LIME-CONCRETE"
        assert pad["material"] == "MAT-RC"
        bp = model._solid_geometry(source,blind)["parts"][0]
        assert math.isclose(bp["z0"],case["bearing"],abs_tol=1e-9)
        assert .35-1e-9 <= bp["z1"]-bp["z0"] <= .45+1e-9
        for i,column_id in enumerate(pad_id.split("-")):
            base,pyramid = model._solid_geometry(source,pad)["parts"][2*i:2*i+2]
            assert math.isclose(base["z0"],bp["z1"],abs_tol=1e-9)
            assert math.isclose(base["z1"]-base["z0"],.1,abs_tol=1e-9)
            assert math.isclose(base["z1"],pyramid["z0"],abs_tol=1e-9)
            assert Polygon(bp["outline"]).buffer(1e-8).covers(Polygon(base["outline"]))
            column = elements[column_id]
            parts=sorted(model.column_geometry(source,column)['parts'],key=lambda p:p['z0'])
            stems=[p for p in parts if p['z0']>=pyramid['z1']-1e-9 and p['z1']<=case['stem_top']+1e-9]
            assert stems and math.isclose(stems[0]['z0'],pyramid['z1'],abs_tol=1e-9)
            assert math.isclose(stems[-1]['z1'],case['stem_top'],abs_tol=1e-9)
            assert all(math.isclose(a['z1'],b['z0'],abs_tol=1e-9) for a,b in zip(stems,stems[1:]))
            above=[p for p in parts if math.isclose(p['z0'],case['stem_top'],abs_tol=1e-9)]
            assert len(above)==1,column_id
            size = case['drain_stem'] if column_id in case['drain_columns'] else case['standard_stem']
            for stem in stems:
                shape=Polygon(stem['outline'],stem.get('holes',[]));x0,y0,x1,y1=shape.bounds
                assert math.isclose(x1-x0,size,abs_tol=1e-9) and math.isclose(y1-y0,size,abs_tol=1e-9)
                assert shape.area>0
            contact=Polygon(stems[0]['outline'],stems[0].get('holes',[]))
            assert contact.symmetric_difference(Polygon(pyramid['top_outline'])).area<1e-8
            assert math.isclose(Polygon(pyramid["top_outline"]).area,size*size,abs_tol=1e-9)
            assert Polygon(pyramid["outline"]).covers(Polygon(pyramid["top_outline"]))


def test_foundation_frustum_quantities_match_scipy_convex_hulls():
    """📐 Validate each physical pad volume with independent SciPy3D convex hulls."""
    from scipy.spatial import ConvexHull
    source,case = model.load_model(),_case()
    elements={e["tag"]:e for e in source["elements"]}
    for pad_id in case["expected_pad_ids"]:
        tag="FT-PAD-"+pad_id
        obj=elements[tag]
        oracle=0.
        for p in model._solid_geometry(source,obj)["parts"]:
            top=p.get("top_outline",p["outline"])
            vertices=[[x,y,p["z0"]] for x,y in p["outline"]]+[[x,y,p["z1"]] for x,y in top]
            oracle+=ConvexHull(vertices).volume
        assert math.isclose(model.quantity(tag,"volume",source),oracle,rel_tol=1e-8,abs_tol=1e-9)


def test_foundation_material_totals_match_portable_oracle():
    """⚖️ Preserve independently hull-checked material totals without treating blinding asRC."""
    source,case=model.load_model(),_case()
    for material,expected in case["material_volumes"].items():
        actual=sum(model.quantity(e["tag"],"volume",source) for e in source["elements"] if e["tag"].startswith(("FT-PAD-","FT-BLINDING-")) and e["material"]==material)
        assert math.isclose(actual,expected,rel_tol=1e-8,abs_tol=1e-8)


def test_foundations_preserve_cellar_material_ownership():
    """🧱 Prevent exterior foundation volumes from occupying existing cellar masonry or footings."""
    from shapely.geometry import LineString,Polygon
    source=model.load_model()
    geometry=model.building_geometry(source)
    new=[e for e in geometry["foundations"] if e["tag"].startswith(("FT-PAD-","FT-BLINDING-"))]
    old=[e for e in geometry["foundations"] if e not in new]
    walls=[e for e in geometry["walls"] if e["storey"]=="cellar"]
    for item in new:
        for p in item["parts"]:
            shape=Polygon(p["outline"])
            for w in walls:
                if min(p["z1"],w["top"])-max(p["z0"],w["base"])>1e-9:
                    assert shape.intersection(LineString([w["start"],w["end"]]).buffer(w["thickness"]/2,cap_style=2)).area<1e-8
            for other in old:
                for q in other["parts"]:
                    if min(p["z1"],q["z1"])-max(p["z0"],q["z0"])>1e-9:
                        assert shape.intersection(Polygon(q["outline"])).area<1e-8
