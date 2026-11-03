"""🧊️ Oracle of the three views: shapely recomputes the extents of the hand-computed vector shapes (rect, capsule, rest-pose coordinates moved by the hand-computed bone travel) and of exported evaluated rings (collection bounds).

Input: the path of a JSON document written by the test with `fixture` (cases of the vectors, hand travel included) and `rings` (evaluated rings of catalog shapes).
Output: JSON `{ "fixture": {case: {"view/shape": extent, "view": extent}}, "rings": {key: extent | null} }`.
"""
import json
import math
import sys

from shapely import affinity
from shapely.geometry import GeometryCollection, LineString, Point, Polygon, box
from shapely.ops import unary_union


def extent(geometry):
    if geometry.is_empty:
        return None
    minx, miny, maxx, maxy = geometry.bounds
    return {"minX": minx, "maxX": maxx, "minY": miny, "maxY": maxy}


def travel(value, lift):
    return sum(lift if part == "lift" else float(part) for part in str(value).split("+"))


def local_geometry(shape):
    if shape["kind"] == "rect":
        return box(shape["x"], shape["y"], shape["x"] + shape["w"], shape["y"] + shape["h"])
    first = Point(shape["x0"], shape["y0"]).buffer(shape["r0"], 64)
    second = Point(shape["x1"], shape["y1"]).buffer(shape["r1"], 64)
    return unary_union([first, second]).convex_hull


def stretch_y(geometry, y_of):
    return Polygon([(x, y_of(y)) for x, y in geometry.exterior.coords])


def place(view, shape, case):
    bone = case["bones"][shape["bone"]]
    lift = case["liftM"]
    origin = case["originTravel"][shape["bone"]]
    tip = case["tipTravel"][shape["bone"]]
    dx0, dy0 = travel(origin["dx"], lift), travel(origin["dy"], lift)
    dx1, dy1 = travel(tip["dx"], lift), travel(tip["dy"], lift)
    follow = shape.get("followTip") is True
    if view == "elevation":
        rotated = affinity.rotate(local_geometry(shape), bone["angleDeg"], origin=(0, 0))
        return affinity.translate(rotated, bone["restOrigin"]["x"] + dx0, bone["restOrigin"]["y"] + dy0)
    if view == "front":
        geometry = local_geometry(shape)
        y0 = bone["restOrigin"]["y"]
        span = bone["restTip"]["y"] - y0
        if span != 0:
            return stretch_y(geometry, lambda y: y + dy0 + (dy1 - dy0) * min(1.0, max(0.0, (y - y0) / span)))
        return affinity.translate(geometry, 0, dy0)
    if shape["kind"] == "capsule" and follow:
        return local_geometry({**shape, "x0": shape["x0"] + dx0, "x1": shape["x1"] + dx1})
    return affinity.translate(local_geometry(shape), dx0, 0)


def fixture_case(case):
    shapes = {}
    result = {}
    for view, items in case["shapes"].items():
        drawn = []
        for shape in items:
            geometry = place(view, shape, case)
            result[f"{view}/{shape['id']}"] = extent(geometry)
            if shape["role"] != "ground":
                drawn.append(geometry)
        result[view] = extent(GeometryCollection(drawn)) if drawn else None
    return result


def ring_geometry(ring):
    if len(ring) >= 3:
        return Polygon([(p[0], p[1]) for p in ring])
    return LineString([(p[0], p[1]) for p in ring])


def rings_extent(rings):
    geometries = [ring_geometry(ring) for ring in rings if len(ring) >= 2]
    return extent(GeometryCollection(geometries)) if geometries else None


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        document = json.load(handle)
    output = {
        "fixture": {key: fixture_case(case) for key, case in document["fixture"].items()},
        "rings": {key: rings_extent(rings) for key, rings in document["rings"].items()},
    }
    json.dump(output, sys.stdout, allow_nan=False)


if __name__ == "__main__":
    main()
