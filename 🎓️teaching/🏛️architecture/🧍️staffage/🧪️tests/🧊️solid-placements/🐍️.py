"""🧊️ Oracle of the solid placements: shapely rotates the rectangle profile (0.45 m along the bone, 0.05 m to its side) by the world angle of the bone and builds the mirrored, circular and rectangular solids.

Input: the path of a JSON document with the fixture `shapes`, `bones` and `cases`. Output: JSON `{pose: {"front"|"plan": {shape: extent}}}`.
"""
import json
import sys

from shapely import affinity
from shapely.geometry import Point, box
from shapely.ops import unary_union


def extent(g):
    minx, miny, maxx, maxy = g.bounds
    return {"minX": minx, "maxX": maxx, "minY": miny, "maxY": maxy}


def solid(view, shape, profile):
    s = shape["solid"]
    minx, miny, maxx, maxy = profile.bounds
    if s["kind"] == "extruded":
        return box(s["z0"], miny, s["z1"], maxy) if view == "front" else box(minx, s["z0"], maxx, s["z1"])
    if view == "plan":
        reach = max(abs(minx - s["axisX"]), abs(maxx - s["axisX"]))
        return Point(s["axisX"], s.get("axisZ", 0)).buffer(reach, 256)
    moved = affinity.translate(profile, -s["axisX"], 0)
    return unary_union([moved, affinity.scale(moved, xfact=-1, origin=(0, 0))]) if s.get("mirror") else moved


data = json.load(open(sys.argv[1], encoding="utf8"))
out = {}
for case in data["cases"]:
    angle = data["bones"]["thigh-l"]["worldAngleDeg"][case["pose"]]
    # the local frame of the bone at rest points down (-90 degrees), the profile is authored along the bone axis
    out[case["pose"]] = {}
    for view in ("front", "plan"):
        out[case["pose"]][view] = {}
        for shape in data["shapes"][view]:
            profile = affinity.rotate(box(0, 0, 0.45, 0.05), angle, origin=(0, 0))
            out[case["pose"]][view][shape["id"]] = extent(solid(view, shape, profile))
print(json.dumps(out))
