"""🕳️ Oracle of the drawing-coverage contract: shapely computes, as one-dimensional geometry on the denominator axis, the union of the promised preset bands of the authored presets minus the union of their valid ranges.

Input: the path of the vectors `🧫️fixtures/🕳️valid-range-coverage/🔣️.json`. Output: JSON `{case name: [[from, to], ...]}`, the holes sorted by `from`.
"""
import json
import sys

from shapely.geometry import LineString
from shapely.ops import unary_union


def segment(interval):
    return LineString([(interval[0], 0.0), (interval[1], 0.0)])


def parts(geometry):
    if geometry.is_empty:
        return []
    pieces = getattr(geometry, "geoms", [geometry])
    return sorted([[min(x for x, _ in piece.coords), max(x for x, _ in piece.coords)] for piece in pieces if piece.length > 0])


def holes(bands, ranges):
    authored = [key for key, value in ranges.items() if value != "unavailable"]
    if not authored:
        return []
    promised = unary_union([segment(bands[key]) for key in authored])
    covered = unary_union([segment(ranges[key]) for key in authored])
    return parts(promised.difference(covered))


vectors = json.load(open(sys.argv[1], encoding="utf-8"))
print(json.dumps({case["name"]: holes(vectors["bands"], case["ranges"]) for case in vectors["cases"]}))
