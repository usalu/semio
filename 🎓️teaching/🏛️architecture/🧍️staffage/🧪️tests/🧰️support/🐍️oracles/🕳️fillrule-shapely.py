"""🕳️ Even-odd oracle (shapely): the painted area and the painted samples of every fill-rule case, from the planar arrangement of the rings (every face is painted when a ray from it crosses the rings an odd number of times) and, for rings that do not cross themselves, from the symmetric difference of the ring polygons. Usage: fillrule-shapely.py <fixture.json>."""
import json
import sys

from shapely.geometry import LineString, Point, Polygon
from shapely.ops import polygonize, unary_union


def crossings(point, rings):
    x, y = point
    count = 0
    for ring in rings:
        for index in range(len(ring)):
            (x0, y0), (x1, y1) = ring[index], ring[(index + 1) % len(ring)]
            if (y0 <= y) != (y1 <= y) and x0 + (y - y0) / (y1 - y0) * (x1 - x0) > x:
                count += 1
    return count


def painted_faces(rings):
    faces = polygonize(unary_union([LineString(ring + [ring[0]]) for ring in rings]))
    return [face for face in faces if crossings(face.representative_point().coords[0], rings) % 2]


def evaluate(case):
    rings = case["rings"]
    faces = painted_faces(rings)
    area = sum(face.area for face in faces)
    shape = unary_union(faces)
    xor = None
    if not case.get("selfCrossing"):
        xor = Polygon(rings[0])
        for ring in rings[1:]:
            xor = xor.symmetric_difference(Polygon(ring))
    return {"areaMm2": area, "xorAreaMm2": None if xor is None else xor.area, "samples": [[x, y, shape.contains(Point(x, y))] for x, y, _ in case["samples"]]}


print(json.dumps([evaluate(case) for case in json.load(open(sys.argv[1], encoding="utf-8"))["cases"]]))
