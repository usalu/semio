"""🔗️ Oracle of the same-region merge: shapely measures the visible outline length of every shape. A shape's outline is hidden by every filled shape of a higher z, and, for a filled shape, by every filled shape of the same region and bone (they read as one silhouette); unfilled strokes are never merged.

Input: the path of a JSON document with the fixture `shapes`, `paperMmPerM`. Output: JSON `{shape id: visible outline length in paper mm}`.
"""
import json
import sys

from shapely.geometry import box

data = json.load(open(sys.argv[1], encoding="utf8"))
k = data["paperMmPerM"]
shapes = data["shapes"]


def polygon(shape):
    return box(shape["x"], shape["y"], shape["x"] + shape["w"], shape["y"] + shape["h"])


out = {}
for p in shapes:
    outline = polygon(p).exterior
    for q in shapes:
        if q is p or not q["filled"]:
            continue
        merged = p["filled"] and q["region"] == p["region"] and q["bone"] == p["bone"]
        if q["z"] > p["z"] or merged:
            outline = outline.difference(polygon(q))
    out[p["id"]] = outline.length * k
print(json.dumps(out))
