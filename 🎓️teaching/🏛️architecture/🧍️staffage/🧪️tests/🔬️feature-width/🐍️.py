"""🔬️ Oracle of the minimum feature width: shapely erodes (`erosion`: twice the largest inscribed radius, found by bisection on the buffer) or opens (`opening`: the smallest width whose opening removes more than max(1 % of the area, (minFeature/2)²) + 4(1 − π/4)(w/2)², the criterion of the measure) every shape.

Input: the path of a JSON document with the fixture `minFeatureMm` and `cases`. Output: JSON `{case name: width in mm}`.
"""
import json
import math
import sys

from shapely.geometry import Polygon

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
    area = shape.area
    tolerance = max(0.01 * area, (min_feature / 2) ** 2)

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


print(json.dumps({c["name"]: (erosion if c["oracle"] == "erosion" else opening)(polygon(c["rings"])) for c in data["cases"]}))
