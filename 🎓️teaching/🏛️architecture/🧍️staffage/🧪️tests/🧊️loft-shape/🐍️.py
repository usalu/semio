"""🧵️ Oracle of the loft shape: numpy and scipy place the superelliptic sections (scipy `Rotation` turns the section axes by the section quaternion), shapely takes the convex hull of every two consecutive section rings in each view and unites the hulls.

Input: the path of a JSON document with the fixture `lofts`, `cases` (each with the hand-written `basis` of the bone) and `segments`. Output: JSON `{pose: {loft: {view: {"minX", "maxX", "minY", "maxY", "area"}}}}`, relative to the bone origin.
"""
import json
import sys

import numpy as np
from scipy.spatial.transform import Rotation
from shapely.geometry import MultiPoint
from shapely.ops import unary_union

VIEWS = {"elevation": (0, 1), "front": (2, 1), "plan": (0, 2)}


def axes(sections):
    centres = [np.array(section["at"], float) for section in sections]
    out = []
    for i, section in enumerate(sections):
        if "rotation" in section:
            turn = Rotation.from_quat(section["rotation"])
            out.append((turn.apply([1, 0, 0]), turn.apply([0, 1, 0])))
            continue
        travel = centres[min(len(centres) - 1, i + 1)] - centres[max(0, i - 1)]
        tangent = travel / np.linalg.norm(travel)
        x = np.array([1.0, 0, 0]) - tangent * tangent[0]
        if np.linalg.norm(x) < 1e-6:
            x = np.array([0, 0, 1.0]) - tangent * tangent[2]
        x = x / np.linalg.norm(x)
        out.append((x, np.cross(tangent, x)))
    return out


def rings(sections, segments):
    out = []
    for section, (x, y) in zip(sections, axes(sections)):
        power = 2 / section.get("exponent", 2)
        ring = []
        for j in range(segments):
            angle = 2 * np.pi * j / segments
            c, s = np.cos(angle), np.sin(angle)
            ring.append(np.array(section["at"], float) + x * section["rx"] * np.sign(c) * abs(c) ** power + y * section["ry"] * np.sign(s) * abs(s) ** power)
        out.append(ring)
    return out


data = json.load(open(sys.argv[1], encoding="utf8"))
out = {}
for case in data["cases"]:
    basis = np.array([case["basis"]["u"], case["basis"]["v"], case["basis"]["w"]], float)
    out[case["pose"]] = {}
    for name, loft in data["lofts"].items():
        world = [[point @ basis for point in ring] for ring in rings(loft["sections"], data["segments"])]
        out[case["pose"]][name] = {}
        for view, (a, b) in VIEWS.items():
            hulls = [MultiPoint([(p[a], p[b]) for p in first + second]).convex_hull for first, second in zip(world, world[1:])]
            body = unary_union(hulls)
            minx, miny, maxx, maxy = body.bounds
            out[case["pose"]][name][view] = {"minX": minx, "maxX": maxx, "minY": miny, "maxY": maxy, "area": body.area}
print(json.dumps(out))
