"""🌐️ Oracle of the thermal and transport multiview guard: numpy recomputes (1) the passive wind drift of a zero-momentum parcel in world metres, (2) the orthographic projection of world points to the paper of the elevation, front and plan view, and (3) the symmetric auto range of a thermal legend.

Input: the path of a JSON document written by the test with `drift`, `projection` and `ranges`.
Output: JSON `{ "drift": [[x, z], ...], "projection": {view: [[xMm, yMm], ...]}, "ranges": [[min, max], ...] }`.

World axes (x ahead, y up, z left); paper axes of every view: x right, y down, in millimetres:
  elevation (x, y) -> (x k, -y k)    front (z, y) -> (z k, -y k)    plan (x, z) -> (x k, -z k)
Wind: horizontal bearing phi from +x towards +z, so the drift is U (cos phi, sin phi) per second in (x, z).
"""
import json
import sys

import numpy as np


def drift(case):
    phi = np.radians(case["bearingDeg"])
    ages = np.asarray(case["agesS"], dtype=float)
    offsets = case["windMS"] * np.stack([np.cos(phi) * ages, np.sin(phi) * ages], axis=1)
    return (np.asarray([case["x0"], case["z0"]]) + offsets).tolist()


def project(points, k):
    p = np.asarray(points, dtype=float)
    x, y, z = p[:, 0], p[:, 1], p[:, 2]
    return {
        "elevation": np.stack([x * k, -y * k], axis=1).tolist(),
        "front": np.stack([z * k, -y * k], axis=1).tolist(),
        "plan": np.stack([x * k, -z * k], axis=1).tolist(),
    }


def auto_range(case):
    values = np.asarray(case["values"], dtype=float)
    span = max(case["minSpan"], float(np.max(np.abs(values - case["centre"]))))
    return [case["centre"] - span, case["centre"] + span]


with open(sys.argv[1], encoding="utf-8") as handle:
    document = json.load(handle)

result = {
    "drift": [drift(case) for case in document["drift"]],
    "projection": project(document["projection"]["points"], document["projection"]["k"]),
    "ranges": [auto_range(case) for case in document["ranges"]],
}
print(json.dumps(result))
