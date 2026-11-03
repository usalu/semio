"""🧊️ Oracle of the `member` shape (🧪️tests/🧊️member-solid): a prismatic bar between two bone-local points, projected to the side, front and plan view.

numpy builds the section frame and the corners of both end sections in three dimensions, shapely takes the convex hull of their projection (the exact outline of a convex prism).

Input: path of a JSON document `{frames: {bone: {o, u, v, w}}, bone, member: {from, to, section}, cameras: {view: [[paperX axis, sign], [paperUp axis, sign]]}}`.
Output: JSON `{view: {minX, maxX, minY, maxY, area}}`.
"""
import json
import math
import sys

import numpy as np
from shapely.geometry import MultiPoint

AXES = {"x": 0, "y": 1, "z": 2}


def place(frame, p):
    return np.array(frame["o"]) + p["x"] * np.array(frame["u"]) + p["y"] * np.array(frame["v"]) + p["z"] * np.array(frame["w"])


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        document = json.load(handle)
    frames = document["frames"]
    member = document["member"]
    a = place(frames[member["from"].get("bone", document["bone"])], member["from"])
    b = place(frames[member["to"].get("bone", document["bone"])], member["to"])
    bar = (b - a) / np.linalg.norm(b - a)
    v = np.array(frames[member["from"].get("bone", document["bone"])]["v"])
    width = v - (v @ bar) * bar
    width /= np.linalg.norm(width)
    depth = np.cross(bar, width)
    section = member["section"]
    if section["kind"] == "circle":
        offsets = [(section["d"] / 2 * math.cos(2 * math.pi * i / 32), section["d"] / 2 * math.sin(2 * math.pi * i / 32)) for i in range(32)]
    else:
        offsets = [(s * section["w"] / 2, t * section["h"] / 2) for s, t in ((-1, -1), (1, -1), (1, 1), (-1, 1))]
    points = [centre + s * width + t * depth for centre in (a, b) for s, t in offsets]
    output = {}
    for view, ((ax, sx), (ay, sy)) in document["cameras"].items():
        hull = MultiPoint([(sx * p[AXES[ax]], sy * p[AXES[ay]]) for p in points]).convex_hull
        minx, miny, maxx, maxy = hull.bounds
        output[view] = {"minX": minx, "maxX": maxx, "minY": miny, "maxY": maxy, "area": hull.area}
    json.dump(output, sys.stdout, allow_nan=False)


if __name__ == "__main__":
    main()
