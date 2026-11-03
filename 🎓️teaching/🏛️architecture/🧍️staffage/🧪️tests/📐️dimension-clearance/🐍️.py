"""📐️ Oracle of the dimension clearance: shapely measures the distance of every dimension line and tick to the bounding box of the drawn geometry.

Input: the path of a JSON document `[{"name": ..., "box": [minx, miny, maxx, maxy], "items": [[[x, y], ...], ...]}]`. Output: JSON `{name: {"intersects": bool, "distance": mm}}`.
"""
import json
import sys

from shapely.geometry import LineString, box

out = {}
for case in json.load(open(sys.argv[1], encoding="utf8")):
    region = box(*case["box"])
    lines = [LineString(points) for points in case["items"] if len(points) >= 2]
    out[case["name"]] = {"intersects": any(line.intersects(region) for line in lines), "distance": min((line.distance(region) for line in lines), default=-1)}
print(json.dumps(out))
