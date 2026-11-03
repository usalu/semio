"""🧊️ Oracle of the depth ordering: shapely places the rectangle profiles of the plan and the front by the hand-computed world angle of their bone, reads the nearest depth (top in the plan, foremost point in the front) and rebuilds the paint order with its own topological sort.

Input: the path of a JSON document with the fixture `shapes`, `worldAngleDeg`, `toleranceM` and `cases` (poses with a world angle). Output: JSON `{pose: {"front"|"plan": [ids in paint order]}}`.
"""
import json
import sys

from shapely import affinity
from shapely.geometry import box

data = json.load(open(sys.argv[1], encoding="utf8"))
tol = data["toleranceM"]


def depth(view, shape, pose):
    polygon = affinity.rotate(box(shape["x"], shape["y"], shape["x"] + shape["w"], shape["y"] + shape["h"]), data["worldAngleDeg"][shape["bone"]][pose], origin=(0, 0))
    minx, miny, maxx, maxy = polygon.bounds
    return maxy if view == "plan" else maxx


def order(view, pose):
    shapes = sorted(data["shapes"][view], key=lambda s: s["z"])
    n = len(shapes)
    after = {i: set() for i in range(n)}
    for i in range(n):
        for j in range(i + 1, n):
            a, b = shapes[i], shapes[j]
            flip = (
                a["role"] != "ground"
                and b["role"] != "ground"
                and a["bone"] != b["bone"]
                and depth(view, a, pose) - depth(view, b, pose) > tol
                and depth(view, a, "stand") - depth(view, b, "stand") <= tol
            )
            if flip:
                after[j].add(i)
            else:
                after[i].add(j)
    result = []
    left = set(range(n))
    while left:
        free = [i for i in sorted(left) if not any(i in after[k] for k in left if k != i)]
        pick = free[0] if free else min(left)
        result.append(shapes[pick]["id"])
        left.remove(pick)
    return result


poses = [c["pose"] for c in data["cases"] if c["pose"] in data["worldAngleDeg"]["pelvis"]]
print(json.dumps({pose: {view: order(view, pose) for view in ("front", "plan")} for pose in poses}))
