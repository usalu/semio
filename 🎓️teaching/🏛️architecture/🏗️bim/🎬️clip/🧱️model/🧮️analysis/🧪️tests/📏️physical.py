"""📏 Physical analysis contracts checked against independent GEOS and SciPy geometry."""
import importlib
import json
import math
from pathlib import Path

analysis = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")
CASES = json.loads(Path(__file__).with_name("📏️physical.json").read_text(encoding="utf-8"))


def test_regional_bearings_intersect_actual_local_soffits():
    """🏗️ Compare profile overlap and elevation against each regional GEOS footprint."""
    from shapely.geometry import Polygon
    case = CASES["bearing"]
    slab = {"top": 3.37, "thickness": .23, "outline": [[0,0],[4,0],[4,2],[0,2]], "voids": [], "parts": case["slab_parts"]}
    for col in case["columns"]:
        column = {"base": col["z0"], "top": col["z1"], "parts": [col]}
        expected = any(col["z0"] < p["z0"] <= col["z1"] + 1e-9 and Polygon(col["outline"]).intersection(Polygon(p["outline"])).area > 1e-9 for p in slab["parts"])
        assert expected == col["supported"]
        assert analysis._bearing(column, slab) == expected


def test_segment_prism_distance_matches_scipy_and_geos():
    """🧵 Independently minimize distance to actual prism caps and polygon sides."""
    import numpy as np
    from scipy.optimize import minimize_scalar
    from shapely.geometry import Point, Polygon
    for case in CASES["clearance"]:
        a, b = map(np.asarray, case["pipe"])
        polygon = Polygon(case["outline"])
        def distance(t):
            p = a + t * (b - a)
            return math.hypot(polygon.distance(Point(p[:2])), max(case["z0"] - p[2], 0, p[2] - case["z1"]))
        expected = min(distance(0), distance(1), minimize_scalar(distance, bounds=(0,1), method="bounded", options={"xatol":1e-14}).fun)
        on_pipe, on_prism = analysis._segment_prism(case["pipe"][0], case["pipe"][1], case)
        actual = math.dist(on_pipe, on_prism) - case["radius"]
        assert math.isclose(actual, expected - case["radius"], abs_tol=1e-8)
        if "gap" in case:
            assert math.isclose(actual, case["gap"], abs_tol=1e-8)


def test_composite_regular_field_depths_match_independent_material_balance():
    """🧱 Check source-controlled rib fractions rather than counting the entire slab as RC."""
    from types import SimpleNamespace
    case = CASES["composite"]
    q = lambda v: {"value":v,"basis":"assumed","unit":"m"}
    materials = {"RC":{"density":q(case["concrete_density"])}, "CLAY":{"density":q(case["ceramic_density"])}}
    core = SimpleNamespace(_find=lambda m, tag: materials[tag])
    composite = {"compression_layer":q(case["compression"]), "rib_width":q(case["rib_width"]), "rib_pitch":q(case["rib_pitch"]), "concrete":"RC", "ceramic":"CLAY"}
    for expected in case["expected"]:
        result = analysis._composite_weight(core, {}, composite, expected["depth"])
        assert math.isclose(result["components"]["concrete"]["thickness"], expected["concrete"], abs_tol=1e-10)
        assert math.isclose(result["components"]["ceramic"]["thickness"], expected["ceramic"], abs_tol=1e-10)
        assert math.isclose(sum(c["thickness"] for c in result["components"].values()), expected["depth"], abs_tol=1e-10)
        assert math.isclose(result["value"], expected["weight"], abs_tol=1e-10)


def _regional_analysis_fixture():
    from copy import deepcopy
    regional = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪨️regions")
    source, slab = regional._fixture(regional._cases()[0])
    core = regional.model
    q = lambda value: {"value":value,"unit":"m","basis":"assumed"}
    source["materials"].append({"tag":"TEST-CLAY", "guid":core.global_id("TEST-CLAY"), "density":{"value":8,"unit":"kN/m³","basis":"assumed"}})
    composite = {"compression_layer":q(.05), "rib_width":q(.06), "rib_pitch":q(.25), "concrete":"MAT-RC", "ceramic":"TEST-CLAY"}
    slab["geometry"]["composite"] = deepcopy(composite)
    roof = core._find(source, "SL-R")
    roof["geometry"] = deepcopy(slab["geometry"])
    roof["geometry"]["level"] = "roof"
    roof["geometry"]["zones"] = {"solarium":"SP-SOLARIUM"}
    core._find(source, "SP-SOLARIUM")["outline"] = [[0,0],[4,0],[4,6],[0,6]]
    next(s for s in source["storeys"] if s["id"] == "roof")["elevation"]["value"] = 6.60
    roof["geometry"]["top_offset"] = q(-.03)
    roof["geometry"]["regions"][0]["top_offset"] = q(-.03)
    roof["geometry"]["regions"][1]["top_offset"] = q(-.09)
    layers = core._material(source, roof)
    layers["layers"] = [{"id":"structural","name":{"en":"Composite structure","de":"Verbundtragwerk"},"material":"MAT-RC","thickness":q(.17),"zones":["roof","solarium"]},
                         {"id":"paving","name":{"en":"Weathering reserve","de":"Witterungsreserve"},"material":"MAT-PAVING","thickness":q(.09),"zones":["roof","solarium"]}]
    core._find(source, "MAT-PAVING")["density"]["value"] = 20
    source["loads"]["imposed"]["roof"] = {"value":1,"unit":"kN/m²","basis":"assumed"}
    columns = []
    for level, base, top in (("ground",0,2.91),("first",3.17,6.34)):
        for i, x in enumerate((1,7)):
            profile = {"shape":"rectangle","width":q(.2),"depth":q(.2)}
            ring = [[x-.1,2.9],[x+.1,2.9],[x+.1,3.1],[x-.1,3.1]]
            columns.append({"tag":f"TEST-{level}-{i}","guid":core.global_id(f"TEST-{level}-{i}"),"storey":level,"x":x,"y":3,"base":base,"top":top,"profile":profile,
                            "parts":[{"outline":ring,"z0":base,"z1":top,"profile":profile,"area":.04}]})
    geometry = {"roles":{"floor":"SL-1","roof":"SL-R"},"slabs":[core._slab_geometry(source, slab),core._slab_geometry(source,roof)],"pilotis":columns,
                "grid":{"x":[1,7],"y":[3],"labels_x":["1","2"],"labels_y":["A"]}}
    return core, source, geometry


def test_regional_reactions_match_independent_piecewise_area_load_integration():
    """⚖️ Integrate GEOS region and Voronoi intersections with source-controlled composite weights."""
    from unittest.mock import patch
    from shapely.geometry import Polygon, box
    core, source, geometry = _regional_analysis_fixture()
    with patch.object(core, "building_geometry", return_value=geometry):
        for level in ("first","roof"):
            result = analysis.tributary(source, level)
            slab = analysis._slab(geometry,level)
            expected = []
            for i, cell in enumerate((box(-100,-100,4,100),box(4,-100,100,100))):
                force = 0.
                for part in slab["parts"]:
                    area = Polygon(part["outline"]).intersection(cell).area
                    h = part["thickness"]
                    concrete = .05 + .06/.25*(h-.05)
                    weight = concrete*25+(h-concrete)*8
                    if level == "first":
                        weight += 3.5
                    else:
                        weight += (.09-(h-.17))*20 + (2 if i == 0 else 1)
                    force += area*weight
                expected.append(force)
            assert result["balanced"]
            assert math.isclose(result["total_reaction"],sum(expected),abs_tol=1e-7)
            assert all(math.isclose(item["reaction"],force,abs_tol=1e-7) for item,force in zip(result["pilotis"],expected))
            assert all(zone["regions"] for zone in result["zones"])


def test_structural_members_keep_actual_profiles_and_regional_mid_surfaces():
    """🔩 Verify shell occupancy with GEOS and preserve rectangular profile areas without diameters."""
    from unittest.mock import patch
    from shapely.geometry import Polygon
    core, source, geometry = _regional_analysis_fixture()
    with patch.object(core,"building_geometry",return_value=geometry), patch.object(analysis,"link_values",return_value={}):
        translated = analysis.translate_structure(source)
    nodes = {node["id"]:node["position"] for node in translated["nodes"]}
    first = analysis._slab(geometry,"first")
    area = 0.
    for shell in translated["shells"]:
        points = [nodes[node] for node in shell["nodes"]]
        polygon = Polygon([p[:2] for p in points])
        assert polygon.is_valid and polygon.area > 0
        area += polygon.area
        assert all(math.isclose(p[2],shell["mid_surface"],abs_tol=1e-9) for p in points)
        assert any(part["region"] == shell["region"] and math.isclose(shell["mid_surface"],(part["z0"]+part["z1"])/2,abs_tol=1e-9) for part in first["parts"])
    assert math.isclose(area,40,abs_tol=1e-7)
    for member in translated["members"]:
        if member["kind"] == "curve":
            assert "diameter" not in member
            assert member["profile"]["shape"] == "rectangle"
            assert math.isclose(member["cross_section_area"],.04,abs_tol=1e-10)
    assert len(nodes) == len(translated["nodes"])


def test_clearance_api_checks_profile_faces_and_caps_without_a_radius_field():
    """🚧 Verify public clearance results for noncircular structure with no circle approximation."""
    from unittest.mock import patch
    core = analysis._core()
    rule = {"id":"TEST-CLEARANCE","name":{"en":"Clearance","de":"Abstand"},"a_class":"IfcPipeSegment","a_system":"heating","b_class":"IfcColumn",
            "required":{"value":.3,"basis":"assumed"}}
    for case in CASES["clearance"][:2]:
        pipe = {"tag":"TEST-PIPE","guid":"pipe","class":"IfcPipeSegment","system":"heating","points":case["pipe"],"diameter":2*case["radius"]}
        column = {"tag":"TEST-COLUMN","guid":"column","parts":[case]}
        geometry = {"storeys":{"ground":0,"roof":3},"services":[pipe],"pilotis":[column]}
        with patch.object(core,"building_geometry",return_value=geometry):
            result = analysis.clash_check({"rules":{"clearance":[rule]}},include_passed=True)
        assert len(result) == 1
        assert math.isclose(result[0]["clearance"],case["gap"],abs_tol=1e-8)
        assert result[0]["status"] == ("violation" if case["gap"] < .3 else "ok")


def test_section_links_follow_the_referenced_object_and_keep_only_applicable_dimensions():
    """🧷 Resolve source link identity and omit invented circular dimensions for rectangles."""
    from unittest.mock import patch
    core = analysis._core()
    source = {"links":[{"kind":"piloti-section","object_guid":"actual-rectangle"}]}
    def quantity(tag, model):
        assert tag == "actual-rectangle"
        return {"width":.16,"depth":.22,"height":2.91,"cross_section_area":.0352,"volume":.102432}
    with patch.object(core,"quantities",side_effect=quantity):
        values = analysis.link_values(source,"piloti-section")
    assert "diameter" not in values
    assert values["cross_section_area"] == .0352


def test_weathering_change_adds_only_the_actual_new_reserve_weight():
    """🌧️ Check event sensitivity across17/23cm structure without adding phantom soil or full RC."""
    from unittest.mock import patch
    core, source, geometry = _regional_analysis_fixture()
    with patch.object(core,"building_geometry",return_value=geometry):
        before = analysis.tributary(source,"roof")
        roof = core._find(source,"SL-R")
        next(layer for layer in core._material(source,roof)["layers"] if layer["id"] == "paving")["thickness"]["value"] = .14
        after = analysis.tributary(source,"roof")
        section = analysis.link_values(source,"slab-load")
    assert math.isclose(after["total_reaction"]-before["total_reaction"],40.,abs_tol=1e-7)
    assert math.isclose(section["thickness"],.1985,abs_tol=1e-9)
    assert "soil" not in after["load_breakdown"]
