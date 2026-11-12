"""🧊️ Oracle of the volume profile rules: shapely judges every polygon (`Polygon.is_valid` with a non-zero area, holes included) and every open polyline (`LineString.is_simple`) of the vectors.

Input: the path of a JSON document with `polygons` and `polylines`. Output: JSON `{"polygons": {name: {"valid": bool}}, "polylines": {name: {"simple": bool}}}`.
"""
import json
import sys

from shapely.geometry import LineString, Polygon

data = json.load(open(sys.argv[1], encoding="utf8"))
polygons = {}
for case in data["polygons"]:
    polygon = Polygon(case["points"], case.get("holes", []))
    polygons[case["name"]] = {"valid": bool(polygon.is_valid and polygon.area > 0.0)}
polylines = {case["name"]: {"simple": bool(LineString(case["points"]).is_simple)} for case in data["polylines"]}
print(json.dumps({"polygons": polygons, "polylines": polylines}))
