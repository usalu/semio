"""Third-party reproduction of the straight-skeleton fixtures in `🧫️fixtures/🦴️skeleton/🔣️.json`.

Expected values are never taken from our implementation. Two independent libraries produce them:

* `py_straight_skeleton` (an independent straight-skeleton implementation, uniform speeds) gives the area swept by every input edge.
  Its answer is accepted only when it satisfies the skeleton invariants (the faces partition the region and every node lies on the
  plane of its face); on inputs where the library fails the case keeps its other oracle.
* `shapely` gives the area still unswept at several times: the mitred inward buffer for uniform speeds (it equals the wavefront only
  while the buffer topology does, so the fixture keeps it only where it agrees with the skeleton library) and the exact intersection
  of the moved half-planes for weighted convex polygons.

Run from the repo root: `.venv/Scripts/python.exe -m pytest -p no:cacheprovider "<this file>"`. The fixture generator of the BIM
ticket (`r7-z-depth-skeleton-fixtures.py`) imports this module, so the oracle logic exists once.
"""
import json
import math
from pathlib import Path

import pytest
from shapely.geometry import Polygon, box

HERE = Path(__file__).resolve().parent
GEOMETRY = HERE.parent.parent
FIXTURE = GEOMETRY / "🧫️fixtures" / "🦴️skeleton" / "🔣️.json"
SCHEMA = GEOMETRY / "🦴️skeleton" / "🧬️schema" / "🔣️.json"
LEVELS = 9


def region(outer, holes):
    return Polygon(outer, holes)


def orient(ring, ccw):
    area = sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring))) / 2
    return list(ring) if (area > 0) == ccw else list(reversed(ring))


def edges_of(rings):
    for r, ring in enumerate(rings):
        for i, p in enumerate(ring):
            yield r, i, p, ring[(i + 1) % len(ring)]


def py_faces(outer, holes):
    """Swept area and (x, y, time) polygon of every input edge, in input edge order, or None when the library fails or is invalid."""
    from py_straight_skeleton import compute_skeleton

    outer_ccw = orient(outer, True)
    holes_cw = [orient(h, False) for h in holes]
    try:
        skeleton = compute_skeleton(exterior=outer_ccw, holes=holes_cw)
        faces = skeleton.get_faces()
    except Exception:
        return None
    rings = [outer_ccw] + holes_cw
    polygons = [[(skeleton.nodes[i].position.x, skeleton.nodes[i].position.y, skeleton.nodes[i].time) for i in face] for face in faces]
    result = {}
    for polygon in polygons:
        for r, i, a, b in edges_of(rings):
            for k in range(len(polygon)):
                p, q = polygon[k], polygon[(k + 1) % len(polygon)]
                if math.dist(p[:2], a) < 1e-7 and math.dist(q[:2], b) < 1e-7 and p[2] == 0 and q[2] == 0:
                    result[(r, i)] = polygon
    if len(result) != sum(len(r) for r in rings):
        return None
    total = region(outer_ccw, holes_cw).area
    areas = {key: abs(Polygon([(x, y) for x, y, _ in polygon]).area) for key, polygon in result.items()}
    if abs(sum(areas.values()) - total) > 1e-6 * max(1.0, total):
        return None
    for (r, i), polygon in result.items():
        a, b = rings[r][i], rings[r][(i + 1) % len(rings[r])]
        d = (b[0] - a[0], b[1] - a[1])
        length = math.hypot(*d)
        for x, y, t in polygon:
            distance = (-d[1] * (x - a[0]) + d[0] * (y - a[1])) / length
            if abs(distance - t) > 1e-6 * max(1.0, abs(distance)):
                return None
    return {key: (areas[key], polygon) for key, polygon in result.items()}


def input_order(outer, holes, faces):
    """Face keys in the order of the INPUT rings (the library works on counter-clockwise exterior and clockwise holes)."""
    order = []
    for ring in [outer] + list(holes):
        for i in range(len(ring)):
            a, b = ring[i], ring[(i + 1) % len(ring)]
            for key, (_, poly) in faces.items():
                pairs = list(zip(poly, poly[1:] + poly[:1]))
                if any((math.dist(p[:2], a) < 1e-7 and math.dist(q[:2], b) < 1e-7) or (math.dist(p[:2], b) < 1e-7 and math.dist(q[:2], a) < 1e-7) for p, q in pairs):
                    order.append(key)
                    break
    return order


def clip_below(polygon, t):
    """Plan area of the part of a planar (x, y, time) face with time <= t."""
    n = len(polygon)
    out = []
    for k in range(n):
        a, b = polygon[k], polygon[(k + 1) % n]
        if a[2] <= t:
            out.append(a[:2])
        if (a[2] - t) * (b[2] - t) < 0:
            s = (t - a[2]) / (b[2] - a[2])
            out.append((a[0] + s * (b[0] - a[0]), a[1] + s * (b[1] - a[1])))
    return abs(Polygon(out).area) if len(out) >= 3 else 0.0


def levels_from_faces(faces, total, horizon):
    return [[horizon * k / (LEVELS + 1), total - sum(clip_below(poly, horizon * k / (LEVELS + 1)) for _, poly in faces.values())] for k in range(1, LEVELS + 1)]


def buffer_levels(outer, holes, horizon):
    shape = region(outer, holes)
    return [[horizon * k / (LEVELS + 1), shape.buffer(-horizon * k / (LEVELS + 1), join_style=2, mitre_limit=1e9).area] for k in range(1, LEVELS + 1)]


def half_plane_levels(outer, speeds, horizon):
    """Exact wavefront of a convex polygon with per-edge speeds: the intersection of the moved half-planes."""
    ring = orient(outer, True)
    big = 1e4
    out = []
    for k in range(1, LEVELS + 1):
        t = horizon * k / (LEVELS + 1)
        shape = box(-big, -big, big, big)
        for i, a in enumerate(ring):
            b = ring[(i + 1) % len(ring)]
            length = math.dist(a, b)
            d = ((b[0] - a[0]) / length, (b[1] - a[1]) / length)
            n = (-d[1], d[0])
            p0 = (a[0] + n[0] * speeds[i] * t, a[1] + n[1] * speeds[i] * t)
            half = Polygon([(p0[0] - d[0] * big, p0[1] - d[1] * big), (p0[0] + d[0] * big, p0[1] + d[1] * big), (p0[0] + d[0] * big + n[0] * big, p0[1] + d[1] * big + n[1] * big), (p0[0] - d[0] * big + n[0] * big, p0[1] - d[1] * big + n[1] * big)])
            shape = shape.intersection(half)
        out.append([t, shape.area])
    return out


def flat(levels):
    return [x for pair in levels for x in pair]


CASES = json.loads(FIXTURE.read_text(encoding="utf-8"))


def test_the_fixture_validates_against_its_schema():
    jsonschema = pytest.importorskip("jsonschema")
    jsonschema.Draft7Validator(json.loads(SCHEMA.read_text(encoding="utf-8"))).validate(CASES)


@pytest.mark.parametrize("case", [c for c in CASES if "py-straight-skeleton" in c["oracles"]], ids=lambda c: c["name"])
def test_face_areas_and_unswept_areas_match_the_independent_skeleton_library(case):
    pytest.importorskip("py_straight_skeleton")
    faces = py_faces(case["outer"], case["holes"])
    assert faces is not None, "the library must answer where the fixture claims it did"
    order = input_order(case["outer"], case["holes"], faces)
    assert [faces[key][0] for key in order] == pytest.approx(case["faces"], rel=1e-7, abs=1e-7)
    peak = max(t for _, poly in faces.values() for _, _, t in poly)
    assert peak == pytest.approx(case["peak"], rel=1e-7)
    total = region(case["outer"], case["holes"]).area
    assert flat(levels_from_faces(faces, total, peak)) == pytest.approx(flat(case["levels"]), rel=1e-7, abs=1e-7)


@pytest.mark.parametrize("case", [c for c in CASES if "shapely-mitre-buffer" in c["oracles"]], ids=lambda c: c["name"])
def test_unswept_areas_match_the_shapely_mitred_buffer(case):
    horizon = case["levels"][-1][0] * (LEVELS + 1) / LEVELS
    got = buffer_levels(case["outer"], case["holes"], horizon)
    total = region(case["outer"], case["holes"]).area
    for (t, area), (want_t, want_area) in zip(got, case["levels"]):
        assert t == pytest.approx(want_t, rel=1e-9)
        assert area == pytest.approx(want_area, abs=1e-6 * max(1.0, total))


@pytest.mark.parametrize("case", [c for c in CASES if "shapely-half-plane-intersection" in c["oracles"]], ids=lambda c: c["name"])
def test_weighted_convex_unswept_areas_match_the_half_plane_intersection(case):
    horizon = case["levels"][-1][0] * (LEVELS + 1) / LEVELS
    got = half_plane_levels(case["outer"], case["speeds"][0], horizon)
    assert flat(got) == pytest.approx(flat(case["levels"]), rel=1e-7, abs=1e-7)
