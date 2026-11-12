"""✂️ Oracle of the section cut-line marker: shapely builds the cut line, cuts its thick ends with `substring` and moves the end points along the viewing direction.

Input: the path of the fixture JSON, whose `markers` give viewpoint, plane and scale and whose `bounds` give the object extent in metres, plus optionally `--bounds <json>` overriding the extent.
Output: JSON `{id: {"line": [[x, y], [x, y]], "ends": [...], "tips": [...]}}` in paper millimetres, y down, relative to the model origin.
"""
import json
import sys

from shapely import affinity
from shapely.geometry import LineString, Point
from shapely.ops import substring

REACH, END, ARROW = 6.0, 4.0, 5.0

data = json.load(open(sys.argv[1], encoding="utf8"))
bounds = json.loads(sys.argv[2]) if len(sys.argv) > 2 else data["bounds"]


def coords(g):
    return [[p[0], p[1]] for p in g.coords]


out = {}
for m in data["markers"]:
    k = m["k"]
    if m["viewpoint"] == "front":
        line = LineString([(m["plane"] * k, -bounds["maxY"] * k - REACH), (m["plane"] * k, -bounds["minY"] * k + REACH)])
        direction = (-ARROW, 0.0)
    else:
        line = LineString([(bounds["minX"] * k - REACH, -m["plane"] * k), (bounds["maxX"] * k + REACH, -m["plane"] * k)])
        direction = (0.0, -ARROW if m["viewpoint"] == "elevation" else ARROW)
    ends = [substring(line, 0, END), substring(line, line.length - END, line.length)]
    tips = [affinity.translate(Point(line.coords[i]), *direction) for i in (0, 1)]
    out[m["id"]] = {"line": coords(line), "ends": [coords(e) for e in ends], "tips": [[t.x, t.y] for t in tips]}
print(json.dumps(out))
