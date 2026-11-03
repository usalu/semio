"""🙈️ Oracle of the feature width on the visible part: shapely subtracts the union of the covers, each grown by half its outline pen (`buffer`), from the shape (`difference`), drops the pieces smaller than (minFeature/2)² and measures the rest by opening (the smallest width whose opening removes more than max(1 % of the area, (minFeature/2)²) + 4(1 − π/4)(w/2)², the criterion of the measure). Nothing visible gives null.

Input: the path of a JSON document with the fixture `minFeatureMm` and `cases`. Output: JSON `{case name: width in mm or null}`.
"""
import json
import math
import sys

from shapely.geometry import Polygon
from shapely.ops import unary_union

data = json.load(open(sys.argv[1], encoding="utf8"))
min_feature = data["minFeatureMm"]


def polygon(rings):
    return Polygon([(p["x"], p["y"]) for p in rings[0]])


def erosion(shape):
    low, high = 0.0, max(shape.bounds[2] - shape.bounds[0], shape.bounds[3] - shape.bounds[1])
    for _ in range(40):
        mid = (low + high) / 2
        if shape.buffer(-mid / 2, join_style="mitre").is_empty:
            high = mid
        else:
            low = mid
    return low


def opening(shape):
    tolerance = max(0.01 * shape.area, (min_feature / 2) ** 2)

    def thin(width):
        opened = shape.buffer(-width / 2, join_style="round").buffer(width / 2, join_style="round")
        return shape.difference(opened).area > tolerance + 4 * (1 - math.pi / 4) * (width / 2) ** 2

    low, high = 0.0, erosion(shape)
    if not thin(high):
        return high
    for _ in range(40):
        mid = (low + high) / 2
        if thin(mid):
            high = mid
        else:
            low = mid
    return high


def grown(cover):
    shape = polygon(cover["rings"])
    return shape.buffer(cover["penMm"] / 2, join_style="mitre") if cover["penMm"] > 0 else shape


def visible_width(case):
    shape = polygon(case["rings"])
    if case["covers"]:
        shape = shape.difference(unary_union([grown(cover) for cover in case["covers"]]))
        pieces = list(shape.geoms) if hasattr(shape, "geoms") else [shape]
        kept = [piece for piece in pieces if piece.area >= (min_feature / 2) ** 2]
        if not kept:
            return None
        shape = unary_union(kept)
    return opening(shape)


print(json.dumps({c["name"]: visible_width(c) for c in data["cases"]}))
