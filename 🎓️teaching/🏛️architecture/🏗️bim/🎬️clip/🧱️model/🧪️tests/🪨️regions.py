"""🪨 Regional slabs checked against portable expected values and independent GEOS solids."""
import importlib
import json
import math
from copy import deepcopy
from pathlib import Path

model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def _cases():
    return json.loads(Path(__file__).with_name("🪨️regions.json").read_text(encoding="utf-8"))["cases"]


def _fixture(case):
    source = model.load_model()
    slab = model._find(source, "SL-1")
    removed = {e["tag"] for e in source["elements"] if e["geometry"].get("slab") == slab["tag"]}
    source["elements"] = [e for e in source["elements"] if e["tag"] not in removed]
    source["relations"] = [r for r in source["relations"] if r["relating"] not in removed and not any(tag in removed for tag in r["related"])]
    next(s for s in source["storeys"] if s["id"] == "first")["elevation"]["value"] = case["level"]
    quantity = lambda n: {"value": n, "unit": "m", "basis": "assumed"}
    slab["geometry"] = {"level": "first", "top_offset": quantity(case["top_offset"]), "outline": deepcopy(case["outline"]),
                        "voids": [{"tag": f"TEST-VOID-{i}", "outline": v} for i, v in enumerate(case["voids"])],
                        "regions": [{**r, "thickness": quantity(r["thickness"]), "top_offset": quantity(r["top_offset"])} for r in case["regions"]]}
    return source, slab


def test_regional_slabs_match_analytical_and_independent_occupied_volumes():
    """🧊 Compare regional prisms, clipped nonconvex voids, quantities and GEOS slice unions."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    for case in _cases():
        source, slab = _fixture(case)
        geometry = model._slab_geometry(source, slab)
        parts = geometry["parts"]
        assert geometry["storey"] == slab["storey"] and geometry["level"] == "first"
        expected = Polygon(case["outline"]).difference(unary_union([Polygon(v) for v in case["voids"]]))
        actual = unary_union([Polygon(p["outline"], p["voids"]) for p in parts])
        assert actual.symmetric_difference(expected).area < 1e-8
        assert math.isclose(actual.area, case["expected"]["area"], abs_tol=1e-8)
        independent_volume = sum(expected.intersection(Polygon(r["outline"])).area * r["thickness"] for r in case["regions"])
        zs = sorted({p[k] for p in parts for k in ("z0", "z1")})
        integrated = sum((hi - lo) * unary_union([Polygon(p["outline"], p["voids"]) for p in parts if p["z0"] < (lo + hi) / 2 < p["z1"]]).area for lo, hi in zip(zs, zs[1:]))
        quantities = model.quantities(slab["tag"], source)
        for volume in (independent_volume, integrated, quantities["volume"]):
            assert math.isclose(volume, case["expected"]["volume"], abs_tol=1e-8)
        assert math.isclose(quantities["gross_volume"], case["expected"]["gross_volume"], abs_tol=1e-8)
        assert math.isclose(model._top(source, "soffit:" + slab["tag"]), case["expected"]["soffit"], abs_tol=1e-8)
        assert all(math.isclose(p["z1"] - p["z0"], p["thickness"], abs_tol=1e-9) for p in parts)
        assert {p["region"] for p in parts} == {r["tag"] for r in case["regions"]}


def test_regional_representations_preserve_depths_voids_and_actual_section_plane():
    """🪚 Independently intersect physical prism footprints and compare their XZ projections."""
    from shapely.geometry import LineString, Polygon, box
    from shapely.ops import unary_union
    for case in _cases():
        source, slab = _fixture(case)
        parts = model._slab_geometry(source, slab)["parts"]
        cut = case["expected"]["section_y"]
        expected = []
        for p in parts:
            intersection = Polygon(p["outline"], p["voids"]).intersection(LineString([(-100, cut), (100, cut)]))
            for line in getattr(intersection, "geoms", [intersection]):
                if line.geom_type == "LineString" and line.length > 1e-9:
                    x0, _, x1, _ = line.bounds
                    expected.append(box(x0, p["z0"], x1, p["z1"]))
        section = model.represent(slab["tag"], "section", source)
        actual = unary_union([Polygon(p["points"]) for p in section["primitives"]])
        assert math.isclose(section["derived"]["cut_y"], cut, abs_tol=1e-9)
        assert actual.symmetric_difference(unary_union(expected)).area < 1e-8
        elevation = model.represent(slab["tag"], "elevation", source)
        expected_elevation = unary_union([box(min(v[0] for v in p["outline"]), p["z0"], max(v[0] for v in p["outline"]), p["z1"]) for p in parts])
        assert unary_union([Polygon(p["points"]) for p in elevation["primitives"]]).symmetric_difference(expected_elevation).area < 1e-8
        for kind in ("3d", "body"):
            primitives = model.represent(slab["tag"], kind, source)["primitives"]
            assert len(primitives) == len(parts)
            assert len({round(p["z1"] - p["z0"], 9) if kind == "3d" else round(p["depth"], 9) for p in primitives}) == 2
        for kind, identifier in (("axis", "Axis"), ("footprint", "FootPrint")):
            rep = model.represent(slab["tag"], kind, source)
            assert (rep["identifier"], rep["type"]) == (identifier, "Curve2D")


def test_regional_partition_rejects_overlap_gaps_and_invalid_depths():
    """🚫 Refuse ambiguous physical partitions instead of silently double-counting concrete."""
    case = _cases()[0]
    for change in ("overlap", "gap", "depth"):
        source, slab = _fixture(case)
        if change == "overlap":
            slab["geometry"]["regions"][1]["outline"][0][0] = 3
            slab["geometry"]["regions"][1]["outline"][3][0] = 3
        elif change == "gap":
            slab["geometry"]["regions"].pop()
        else:
            slab["geometry"]["regions"][0]["thickness"]["value"] = 0
        try:
            model._slab_geometry(source, slab)
        except ValueError:
            pass
        else:
            raise AssertionError(change)


def test_regional_soffit_uses_conservative_lowest_surface():
    """📉 Resolve an unstationed mixed-soffit reference to the lowest actual underside."""
    source, slab = _fixture(_cases()[0])
    slab["geometry"]["regions"][1]["top_offset"]["value"] -= .12
    assert math.isclose(model._top(source, "soffit:" + slab["tag"]), 2.79, abs_tol=1e-9)


def test_measured_archival_regions_have_actual_structural_tops_and_common_soffits():
    """🏛️ Keep measured17/20/23cm depths and structural tops separate from finished datums."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source = model.load_model()
    contract = json.loads(Path(__file__).with_name("🪨️regions.json").read_text(encoding="utf-8"))
    for case in contract["measured_slabs"]:
        slab = model._find(source, case["tag"])
        geometry = model._slab_geometry(source, slab)
        parts = geometry["full_parts"]
        assert {round(p["thickness"], 2) for p in parts} == {float(k) for k in case["tops_by_depth"]}
        for p in parts:
            assert math.isclose(p["z0"], case["soffit"], abs_tol=1e-8)
            expected_top = next(top for depth, top in case["tops_by_depth"].items() if math.isclose(p["thickness"], float(depth), abs_tol=1e-9))
            assert math.isclose(p["z1"], expected_top, abs_tol=1e-8)
        root = Polygon(geometry["outline"]).difference(unary_union([Polygon(v) for v in geometry["voids"]]))
        from shapely.strtree import STRtree
        polygons = [Polygon(p["outline"], p["voids"]) for p in parts]
        assert all(p.is_valid and p.difference(root).area < 1e-9 for p in polygons)
        assert abs(sum(p.area for p in polygons)-root.area) < 1e-7
        tree = STRtree(polygons)
        for i, polygon in enumerate(polygons):
            for j in tree.query(polygon):
                if int(j)>i:
                    assert polygon.intersection(polygons[int(j)]).area < 1e-9
        volume = sum(polygon.area*(part["z1"]-part["z0"]) for polygon,part in zip(polygons,parts))
        quantities=model.quantities(slab['tag'],source)
        assert quantities['scope']==contract['measured_scope']
        assert math.isclose(quantities['volume'],volume,abs_tol=1e-7)
        owned=sum(Polygon(p['outline'],p.get('voids',[])).area*(p['z1']-p['z0']) for p in geometry['parts'])
        shared=sum(Polygon(p['outline'],p.get('voids',[])).area*(p['z1']-p['z0']) for p in geometry['shared_parts'])
        assert math.isclose(owned+shared,volume,abs_tol=1e-7)
        assert math.isclose(quantities['exclusive_volume'],owned,abs_tol=1e-7)
        assert math.isclose(quantities['shared_volume'],shared,abs_tol=1e-7)
        assert set(p['owner'] for p in geometry['shared_parts'])==set(case['column_reserve_owners'])
        assert set(slab['geometry'].get('column_reserves',[]))==set(case['column_reserve_owners'])
        assert model.represent(slab["tag"], "3d", source)["primitives"]


def test_step_exports_every_regional_solid_with_actual_depth_and_void_subtraction():
    """📤 Reconstruct exported STEP polygon prisms and verify their volume through GEOS."""
    import re
    from unittest.mock import patch
    from shapely.geometry import Polygon
    for case in _cases():
        source, slab = _fixture(case)
        geometry = model._slab_geometry(source, slab)
        with patch.object(model, "building_geometry", return_value={"slabs": [geometry]}):
            exported = model.step_excerpt(slab["tag"], source)
        records = {}
        for line in exported["lines"]:
            match = re.match(r"#(\d+)=(\w+)\((.*)\);$", line)
            if match:
                records[int(match[1])] = (match[2], match[3])
        solids = [arguments for name, arguments in records.values() if name == "IFCEXTRUDEDAREASOLID"]
        assert len(solids) == len(geometry["parts"])
        volume = 0.
        depths = set()
        for solid in solids:
            profile, placement, direction = [int(n) for n in re.findall(r"#(\d+)", solid)]
            depth = float(solid.rsplit(",", 1)[1])
            depths.add(round(depth, 9))
            assert records[profile][0] == "IFCARBITRARYCLOSEDPROFILEDEF"
            polyline = int(re.findall(r"#(\d+)", records[profile][1])[0])
            point_ids = [int(n) for n in re.findall(r"#(\d+)", records[polyline][1])]
            assert point_ids[0] == point_ids[-1]
            outline = [[float(n) for n in records[p][1].strip("()").split(",")] for p in point_ids[:-1]]
            volume += Polygon(outline).area * depth
            point = int(re.findall(r"#(\d+)", records[placement][1])[0])
            bottom = float(records[point][1].strip("()").split(",")[-1])
            assert math.isclose(bottom, case["expected"]["soffit"], abs_tol=1e-9)
        assert depths == {.17, .23}
        assert math.isclose(volume, case["expected"]["volume"], abs_tol=1e-8)
        referenced = {int(n) for line in exported["lines"] for n in re.findall(r"#(\d+)", line.partition("=")[2])}
        assert referenced <= records.keys()
