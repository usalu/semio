"""🕳️ Overlap oracle (shapely): the filled shapes of the catalog whose rings overlap each other without one lying inside the other (a hole). Usage: fillrule-catalog.py <rings.json> where the file lists {asset, scale, view, id, rings} with rings in local metres; prints the offenders with the overlap area in square metres."""
import json
import sys

from shapely.geometry import Polygon

MINIMUM_M2 = 1e-5
NESTED = 0.98
found = []
for shape in json.load(open(sys.argv[1], encoding="utf-8")):
    polygons = [Polygon(ring).buffer(0) for ring in shape["rings"] if len(ring) >= 3]
    for first in range(len(polygons)):
        for second in range(first + 1, len(polygons)):
            smaller = min(polygons[first].area, polygons[second].area)
            overlap = polygons[first].intersection(polygons[second]).area
            if overlap > MINIMUM_M2 and overlap < NESTED * smaller:
                found.append({"asset": shape["asset"], "scale": shape["scale"], "view": shape["view"], "id": shape["id"], "rings": [first, second], "overlapM2": round(overlap, 6)})
print(json.dumps(found))
