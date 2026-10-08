"""Third-party (`shapely`) reproduction of the language-agnostic fixtures in `🧫️fixtures/*` and of `semio-framework-2d` regions.

Run from the repo root: `.venv/Scripts/python.exe -m pytest -p no:cacheprovider "<this file>"`.
"""
import json
import math
from pathlib import Path

import pytest
import shapely
from shapely.geometry import LineString, Point, Polygon
from shapely.ops import unary_union

HERE = Path(__file__).resolve().parent
GEOMETRY = HERE.parent.parent
FIXTURES = GEOMETRY / "🧫️fixtures"
REGIONS = GEOMETRY.parent / "◻️2d" / "🧱️regions" / "🧫️fixtures" / "🔣️.json"


def load(domain):
    return json.loads((FIXTURES / domain / "🔣️.json").read_text(encoding="utf-8"))


def arc_points(seg, count=4000):
    start, end, bulge = seg["start"], seg["end"], seg["bulge"]
    if bulge == 0:
        return [tuple(start), tuple(end)]
    sweep = 4 * math.atan(bulge)
    chord = math.dist(start, end)
    radius = chord / (2 * math.sin(abs(sweep) / 2))
    ux, uy = (end[0] - start[0]) / chord, (end[1] - start[1]) / chord
    offset = chord / 2 * (1 - bulge * bulge) / (2 * bulge)
    cx = (start[0] + end[0]) / 2 - uy * offset
    cy = (start[1] + end[1]) / 2 + ux * offset
    a0 = math.atan2(start[1] - cy, start[0] - cx)
    return [(cx + radius * math.cos(a0 + sweep * k / count), cy + radius * math.sin(a0 + sweep * k / count)) for k in range(count + 1)]


def loop_polygon(vertices, count=2000):
    ring = []
    for i, v in enumerate(vertices):
        nxt = vertices[(i + 1) % len(vertices)]
        ring.extend(arc_points({"start": v[:2], "end": nxt[:2], "bulge": v[2]}, count)[:-1])
    return Polygon(ring)


ARCS = load("🌙️bulge")
LOOPS = load("➰️loops")
TRIANGULATION = load("🔺️triangulation")
MESH = load("🕸️mesh")
SECTION = load("🔪️section")
REGION_FIXTURES = json.loads(REGIONS.read_text(encoding="utf-8"))


@pytest.mark.parametrize("case", [c for c in ARCS["arcs"] if c["expected"]["center"] is not None], ids=lambda c: c["name"])
def test_arc_length_and_segment_area_match_shapely(case):
    points = arc_points(case)
    line = LineString(points)
    assert line.length == pytest.approx(case["expected"]["length"], rel=1e-6)
    assert abs(Polygon(points).area) == pytest.approx(abs(case["expected"]["segment_area"]), rel=1e-5, abs=1e-6)
    minx, miny, maxx, maxy = line.bounds
    if "bounds" in case["expected"]:
        assert (minx, miny, maxx, maxy) == pytest.approx(case["expected"]["bounds"], abs=1e-6)
    mid = line.interpolate(0.5, normalized=True)
    assert (mid.x, mid.y) == pytest.approx(case["expected"]["mid"], abs=1e-5)


def segment_line(seg):
    return LineString(arc_points(seg, 6000))


@pytest.mark.parametrize("case", [c for c in ARCS["intersections"] if c["extent"] == "bounded"], ids=lambda c: c["name"])
def test_bounded_intersections_match_shapely(case):
    hit = segment_line(case["a"]).intersection(segment_line(case["b"]))
    points = [] if hit.is_empty else ([g for g in hit.geoms] if hasattr(hit, "geoms") else [hit])
    got = sorted((round(p.x, 3), round(p.y, 3)) for p in points if p.geom_type == "Point")
    want = sorted((round(x, 3), round(y, 3)) for x, y in case["expected"])
    if case["name"] == "tangent at arc end":
        assert len(got) <= 1
        return
    assert got == pytest.approx(want, abs=2e-3) if got and want else got == want


@pytest.mark.parametrize("case", ARCS["offsets"], ids=lambda c: c["name"])
def test_offsets_keep_the_distance_to_the_source_curve_in_shapely(case):
    expected = case["expected"]
    if expected is None:
        return
    source = segment_line(case["seg"])
    offset = segment_line(expected)
    for k in range(0, 11):
        p = offset.interpolate(k / 10, normalized=True)
        assert source.distance(p) == pytest.approx(abs(case["distance"]), abs=1e-4)


@pytest.mark.parametrize("case", LOOPS["loops"], ids=lambda c: c["name"])
def test_loop_measures_match_shapely(case):
    polygon = loop_polygon(case["vertices"])
    e = case["expected"]
    assert polygon.area == pytest.approx(e["area"], rel=1e-5)
    assert polygon.length == pytest.approx(e["perimeter"], rel=1e-5)
    assert (polygon.centroid.x, polygon.centroid.y) == pytest.approx(e["centroid"], abs=1e-4)
    assert polygon.bounds == pytest.approx(e["bounds"], abs=1e-5)
    for probe in case["contains"]:
        p = Point(probe["point"])
        if probe["at"] == "inside":
            assert polygon.contains(p)
        elif probe["at"] == "outside":
            assert not polygon.covers(p)
        else:
            assert polygon.boundary.distance(p) < 1e-3


@pytest.mark.parametrize("case", LOOPS["offsets"], ids=lambda c: c["name"])
def test_loop_offsets_match_shapely_buffer(case):
    if case["expected_area"] is None:
        assert loop_polygon(case["vertices"]).buffer(case["distance"], join_style="mitre", mitre_limit=case["miter_limit"]).is_empty
        return
    polygon = loop_polygon(case["vertices"])
    grown = polygon.buffer(case["distance"], join_style="mitre", mitre_limit=case["miter_limit"], quad_segs=2000)
    assert grown.area == pytest.approx(case["expected_area"], rel=2e-4)
    assert grown.length == pytest.approx(case["expected_perimeter"], rel=2e-4)


@pytest.mark.parametrize("case", TRIANGULATION, ids=lambda c: c["name"])
def test_triangulation_fixtures_match_shapely_constrained_delaunay(case):
    polygon = Polygon(case["outer"], case["holes"])
    assert polygon.area == pytest.approx(case["area"], rel=1e-12)
    triangles = shapely.constrained_delaunay_triangles(polygon)
    parts = list(triangles.geoms)
    assert sum(t.area for t in parts) == pytest.approx(case["area"], rel=1e-9)
    assert len(parts) == case["triangles"]


@pytest.mark.parametrize("case", MESH["extrusions"], ids=lambda c: c["name"])
def test_extrusion_fixtures_match_shapely_prism_formulas(case):
    polygon = Polygon(case["outer"], case["holes"])
    a, b = case["bottom"][0], case["bottom"][1]
    thickness = case["top"][2] - case["bottom"][2]
    e = case["expected"]
    assert polygon.area * thickness == pytest.approx(e["volume"], rel=1e-12)
    flat = a == 0 and b == 0
    if flat:
        assert 2 * polygon.area + polygon.length * thickness == pytest.approx(e["area"], rel=1e-12)
    else:
        slope = math.hypot(a, b)
        assert 2 * polygon.area * math.sqrt(1 + slope * slope) + polygon.length * thickness == pytest.approx(e["area"], rel=1e-12)
    centroid = polygon.centroid
    assert (centroid.x, centroid.y) == pytest.approx(e["centroid"][:2], abs=1e-12)


@pytest.mark.parametrize("case", [c for c in MESH["walls"] if c["axis"]["bulge"] == 0 and c["name"] != "wall with slanted ends"], ids=lambda c: c["name"])
def test_straight_wall_fixtures_match_shapely_elevation_formulas(case):
    face = Polygon(case["left_face"]["outer"], case["left_face"]["holes"])
    thickness = case["left"] + case["right"]
    assert face.area * thickness == pytest.approx(case["volume"][0], rel=1e-12)
    assert 2 * face.area + face.length * thickness == pytest.approx(case["area"][0], rel=1e-12)


def test_slanted_wall_volume_matches_shapely_plan_trapezoid():
    case = next(c for c in MESH["walls"] if c["name"] == "wall with slanted ends")
    left, right = case["left_face"]["outer"], case["right_face"]["outer"]
    plan = Polygon([(left[0][0], 0.1), (left[1][0], 0.1), (right[1][0], -0.1), (right[0][0], -0.1)])
    assert plan.area * 2 == pytest.approx(case["volume"][0], rel=1e-12)


def test_curved_wall_fixture_matches_shapely_annulus_sector():
    case = next(c for c in MESH["walls"] if c["axis"]["bulge"] != 0)
    ring = Point(0, 0).buffer(5.1, quad_segs=4000).difference(Point(0, 0).buffer(4.9, quad_segs=4000))
    quadrant = shapely.box(0, 0, 6, 6)
    sector_area = ring.intersection(quadrant).area
    window = (5.1**2 - 4.9**2) / 2 * (1 / 5) * 1
    assert sector_area * 3 - window == pytest.approx(case["volume"][1], rel=1e-5)


@pytest.mark.parametrize("case", SECTION, ids=lambda c: c["name"])
def test_plan_cut_areas_match_shapely(case):
    polygon = Polygon(case["outer"], case["holes"])
    expected = case["expected"]
    if not expected:
        assert not (case["bottom"] <= case["z"] <= case["top"])
        return
    outer = sum(c["area"] for c in expected if c["area"] > 0)
    holes = -sum(c["area"] for c in expected if c["area"] < 0)
    assert outer - holes == pytest.approx(polygon.area, rel=1e-12)


@pytest.mark.parametrize("case", REGION_FIXTURES["offsets"], ids=lambda c: c["name"])
def test_region_offsets_match_shapely_buffer(case):
    polygon = Polygon(case["outer"], case["holes"])
    join = case["join"]
    kwargs = {"mitre": {"join_style": "mitre", "mitre_limit": join.get("limit", 5)}, "bevel": {"join_style": "bevel"}, "round": {"join_style": "round", "quad_segs": 4096}}[{"miter": "mitre"}.get(join["kind"], join["kind"])]
    result = polygon.buffer(case["distance"], **kwargs)
    expected = case["expected"]
    assert result.area == pytest.approx(expected["area"], abs=max(expected["tolerance"], 1e-9))
    parts = list(result.geoms) if hasattr(result, "geoms") else ([] if result.is_empty else [result])
    assert len(parts) == expected["regions"]
    assert sum(len(p.interiors) for p in parts) == expected["holes"]


@pytest.mark.parametrize("domain", ["🌙️bulge", "➰️loops", "🔺️triangulation", "🕸️mesh", "🔪️section", "🧭️placement"])
def test_fixtures_validate_against_their_schemas(domain):
    jsonschema = pytest.importorskip("jsonschema")
    schema = json.loads((GEOMETRY / domain / "🧬️schema" / "🔣️.json").read_text(encoding="utf-8"))
    jsonschema.Draft7Validator(schema).validate(load(domain))


def test_region_fixtures_validate_against_their_schema():
    jsonschema = pytest.importorskip("jsonschema")
    schema = json.loads((REGIONS.parent.parent / "🧬️schema" / "🔣️.json").read_text(encoding="utf-8"))
    jsonschema.Draft7Validator(schema).validate(REGION_FIXTURES)


@pytest.mark.parametrize("case", REGION_FIXTURES["booleans"], ids=lambda c: c["name"])
def test_region_booleans_match_shapely(case):
    subject = unary_union([Polygon(r) for r in case["subject"]])
    clip = unary_union([Polygon(r) for r in case["clip"]]) if case["clip"] else Polygon()
    result = {"union": subject.union, "intersection": subject.intersection, "difference": subject.difference, "xor": subject.symmetric_difference}[case["operation"]](clip)
    expected = case["expected"]
    assert result.area == pytest.approx(expected["area"], abs=1e-9)
    parts = list(result.geoms) if hasattr(result, "geoms") else ([] if result.is_empty else [result])
    assert len(parts) == expected["regions"]
    assert sum(len(p.interiors) for p in parts) == expected["holes"]
