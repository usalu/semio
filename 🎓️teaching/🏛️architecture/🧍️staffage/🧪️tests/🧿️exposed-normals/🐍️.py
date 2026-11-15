"""🧿️ Oracle of the exposed area with outward normals: trimesh decides, for the triangles of a target mesh wound outward, which faces have a probe moved `eps` out along trimesh's own face normal inside any `against` mesh (ray parity `contains`).

Input: the path of a JSON document `{"cases": [{"name", "target": {"positions", "indices"}, "against": [{"positions", "indices"}], "eps"}]}`; the target must be wound outward (the test says how it knows).
Output: JSON `{name: {"exposedArea": ..., "totalArea": ..., "outwardVolume": ...}}`; `outwardVolume` is trimesh's signed volume of the target (positive for a closed outward mesh).
"""
import json
import sys

import numpy as np
import trimesh


def mesh_of(entry):
    m = trimesh.Trimesh(vertices=np.array(entry["positions"], dtype=float).reshape(-1, 3), faces=np.array(entry["indices"], dtype=np.int64).reshape(-1, 3), process=False)
    m.merge_vertices()
    return m


def answer(case):
    target = mesh_of(case["target"])
    probes = target.triangles_center + target.face_normals * case["eps"]
    buried = np.zeros(len(probes), dtype=bool)
    for other in case["against"]:
        buried |= mesh_of(other).contains(probes)
    return {"exposedArea": float(target.area_faces[~buried].sum()), "totalArea": float(target.area), "outwardVolume": float(target.volume)}


if __name__ == "__main__":
    with open(sys.argv[1], encoding="utf8") as handle:
        cases = json.load(handle)["cases"]
    print(json.dumps({case["name"]: answer(case) for case in cases}))
