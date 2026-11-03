"""🧍️ Oracle of the plan arm span: shapely measures the lateral (y) extent of the union of the evaluated plan rings of the arm shapes.

Input: the path of a JSON document `{key: [[[x, y], ...], ...]}` (rings per measurement). Output: JSON `{key: {"minY": ..., "maxY": ...}}`.
"""
import json
import sys

from shapely.geometry import Polygon
from shapely.ops import unary_union

data = json.load(open(sys.argv[1], encoding="utf8"))
out = {}
for key, rings in data.items():
    _, miny, _, maxy = unary_union([Polygon(ring) for ring in rings]).bounds
    out[key] = {"minY": miny, "maxY": maxy}
print(json.dumps(out))
