"""✅️ Oracle of `lateral-ambiguous` on the side elevation: numpy solves the bone chain with its full rest placement and measures how far a point beside the plane of a bone leaves its flat drawing.

Input: the path of a JSON document written by the test: `cases` {name: {`bones` (id, parent, angleDeg, abductionDeg?, twistDeg?), `bone`, `poses` [{bone: {angleDeg?, abductionDeg?, twistDeg?}}]}}.
Output: JSON {name: {"thetaDeg": largest angle between the joint axis of the bone and the paper normal, "lean": largest |w1 - A·w0| with A = [u1 v1]·[u0 v0]^-1 on paper, 1 when the rest plane is edge-on}}.
"""
import json
import math
import sys

import numpy as np


def rw(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return np.array([[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]])


def rv(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return np.array([[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]])


def ru(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return np.array([[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]])


def turn(placement):
    return rw(placement.get("angleDeg", 0.0)) @ rv(placement.get("abductionDeg", 0.0)) @ ru(placement.get("twistDeg", 0.0))


def frames(bones, offsets):
    solved = {}
    for bone in bones:
        parent = solved[bone["parent"]] if bone["parent"] else np.eye(3)
        solved[bone["id"]] = parent @ turn(bone) @ turn(offsets.get(bone["id"], {}))
    return solved


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        document = json.load(handle)
    output = {}
    for name, case in document["cases"].items():
        rest = frames(case["bones"], {})[case["bone"]]
        paper0 = rest[:2, :2]
        theta, lean = 0.0, 0.0
        for offsets in case["poses"]:
            now = frames(case["bones"], offsets)[case["bone"]]
            theta = max(theta, math.degrees(math.acos(max(-1.0, min(1.0, now[2, 2])))))
            if abs(np.linalg.det(paper0)) < 1e-6:
                lean = max(lean, 1.0)
            else:
                carried = now[:2, :2] @ np.linalg.solve(paper0, rest[:2, 2])
                lean = max(lean, float(np.linalg.norm(now[:2, 2] - carried)))
        output[name] = {"thetaDeg": theta, "lean": lean}
    json.dump(output, sys.stdout, allow_nan=False)


if __name__ == "__main__":
    main()
