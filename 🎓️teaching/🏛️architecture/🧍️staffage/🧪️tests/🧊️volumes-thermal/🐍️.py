"""🧿️ Oracle of the thermal area measures of the volumes: trimesh reproduces, from the very triangles the kernel tessellated, the outer area of the union of the solids of a region (C29), the full mesh area of a hidden region, and the area behind `sectionFaces`.

Input: the path of a JSON document `{"regions": [{"name", "hidden", "solids": [{"positions", "indices", "sections": [[x, y, z], ...]}]}], "eps": metres}`.
Output: JSON `{name: {"area": m2}}`:
- a visible region: the faces within 45 degrees of a section axis are dropped; every other face is sampled (area weighted, trimesh random surface points) and a sample whose probe, moved `eps` out along the outward normal, lies inside another solid of the region (trimesh `contains`) is internal;
- a hidden region: the sum of the mesh areas (trimesh `area`), nothing buried, nothing dropped."""
import json
import sys

import numpy as np
import trimesh

COS45 = np.cos(np.pi / 4)


def solid(entry):
    m = trimesh.Trimesh(np.array(entry["positions"]).reshape(-1, 3), np.array(entry["indices"]).reshape(-1, 3), process=False)
    m.merge_vertices(merge_tex=False, merge_norm=False, digits_vertex=6)
    if m.is_watertight:
        trimesh.repair.fix_normals(m)
        if m.volume < 0:
            m.invert()
    return m


def outer_area(entries, eps):
    meshes = [solid(e) for e in entries]
    rng = np.random.default_rng(11)
    total = 0.0
    for k, (m, e) in enumerate(zip(meshes, entries)):
        keep = np.ones(len(m.faces), dtype=bool)
        for axis in e["sections"]:
            keep &= ~(m.face_normals @ np.array(axis) >= COS45)
        if not keep.any():
            continue
        n = 20000
        weights = m.area_faces * keep
        faces = rng.choice(len(m.faces), size=n, p=weights / weights.sum())
        r = rng.random((n, 2))
        flip = r.sum(axis=1) > 1
        r[flip] = 1 - r[flip]
        tri = m.triangles[faces]
        points = tri[:, 0] + r[:, :1] * (tri[:, 1] - tri[:, 0]) + r[:, 1:] * (tri[:, 2] - tri[:, 0])
        probes = points + m.face_normals[faces] * eps
        buried = np.zeros(n, dtype=bool)
        for j, other in enumerate(meshes):
            if j == k or not other.is_watertight:
                continue
            near = np.all((probes >= other.bounds[0]) & (probes <= other.bounds[1]), axis=1)
            if near.any():
                buried[near] |= other.contains(probes[near])
        total += float(m.area_faces[keep].sum() * (1 - buried.mean()))
    return total


data = json.load(open(sys.argv[1]))
out = {}
for region in data["regions"]:
    if region["hidden"]:
        out[region["name"]] = {"area": float(sum(solid(e).area for e in region["solids"]))}
    else:
        out[region["name"]] = {"area": outer_area(region["solids"], data["eps"])}
print(json.dumps(out))
