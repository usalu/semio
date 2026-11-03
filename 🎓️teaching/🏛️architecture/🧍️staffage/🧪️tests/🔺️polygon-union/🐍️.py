"""🔺️ Oracle of the polygon union (🧪️tests/🔺️polygon-union): shapely `unary_union`.

Input: path of a JSON document `{cases: [{polygons: [[[x, y], ...], ...]}]}`. Output: JSON `[{area, rings}]` with the area of the union and its number of rings (outer rings and holes).
"""
import json
import sys

from shapely.geometry import Polygon
from shapely.ops import unary_union


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        document = json.load(handle)
    output = []
    for case in document["cases"]:
        union = unary_union([Polygon(ring).buffer(0) for ring in case["polygons"]])
        parts = list(union.geoms) if hasattr(union, "geoms") else [union]
        output.append({"area": union.area, "rings": sum(1 + len(part.interiors) for part in parts)})
    json.dump(output, sys.stdout, allow_nan=False)


if __name__ == "__main__":
    main()
