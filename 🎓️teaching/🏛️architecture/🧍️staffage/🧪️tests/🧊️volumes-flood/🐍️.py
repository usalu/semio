"""🌊️ Oracle of the flood from outside (C31): trimesh and scipy decide, from the very triangles the kernel tessellated, which faces of a region the air from outside reaches.

Input: the path of a JSON document `{"pitch": metres, "eps": metres, "cover": metres, "cases": [{"name", "target": region, "solids": [{"region", "positions", "indices"}]}]}`.
Output: JSON `{name: {"area": m2}}`.

Method, sharing no code with the product: every mesh is voxelised at `pitch` (trimesh `voxelize`, surface cells), the cells no surface touches are labelled into connected components (scipy `ndimage.label`, six neighbours) and the component of the grid corner is the outside. The faces of the target region are sampled by area; a sample is dropped when its probe `eps` out of the face lies inside another mesh of the target region (trimesh `contains`); it counts when the first free cell met by stepping out along the normal (at most three pitches) belongs to the outside, or when its probe lies inside a mesh of another region, or such a mesh is met within `cover` in front of the face, and that mesh has itself a sample the outside reaches."""
import json
import sys

import numpy as np
import trimesh
from scipy import ndimage


def solid(entry):
    m = trimesh.Trimesh(np.array(entry["positions"]).reshape(-1, 3), np.array(entry["indices"]).reshape(-1, 3), process=False)
    m.merge_vertices(merge_tex=False, merge_norm=False, digits_vertex=7)
    trimesh.repair.fix_normals(m)
    if m.volume < 0:
        m.invert()
    return m


def samples(m, n, rng):
    faces = rng.choice(len(m.faces), size=n, p=m.area_faces / m.area_faces.sum())
    r = rng.random((n, 2))
    flip = r.sum(axis=1) > 1
    r[flip] = 1 - r[flip]
    tri = m.triangles[faces]
    return tri[:, 0] + r[:, :1] * (tri[:, 1] - tri[:, 0]) + r[:, 1:] * (tri[:, 2] - tri[:, 0]), m.face_normals[faces]


def measure(case, pitch, eps, cover):
    meshes = [solid(e) for e in case["solids"]]
    regions = [e["region"] for e in case["solids"]]
    lo = np.min([m.bounds[0] for m in meshes], axis=0) - 4 * pitch
    hi = np.max([m.bounds[1] for m in meshes], axis=0) + 4 * pitch
    shape = np.ceil((hi - lo) / pitch).astype(int) + 1
    wall = np.zeros(shape, dtype=bool)
    for m in meshes:
        points = trimesh.voxel.creation.voxelize(m, pitch / 2).points
        for dx in (-0.26, 0.26):
            for dy in (-0.26, 0.26):
                for dz in (-0.26, 0.26):
                    idx = np.floor((points + np.array([dx, dy, dz]) * pitch - lo) / pitch).astype(int)
                    wall[idx[:, 0], idx[:, 1], idx[:, 2]] = True
    labels, _ = ndimage.label(~wall)
    outside = labels[0, 0, 0]

    def reached(points, normals):
        out = np.zeros(len(points), dtype=bool)
        done = np.zeros(len(points), dtype=bool)
        for step in np.arange(0.25, 3.01, 0.25):
            idx = np.floor((points + normals * step * pitch - lo) / pitch).astype(int)
            free = ~wall[idx[:, 0], idx[:, 1], idx[:, 2]] & ~done
            out[free] = labels[idx[free, 0], idx[free, 1], idx[free, 2]] == outside
            done |= free
        return out

    rng = np.random.default_rng(31)
    open_mesh = []
    for m in meshes:
        p, n = samples(m, 4000, rng)
        open_mesh.append(bool(reached(p + n * eps, n).any()))
    total = 0.0
    for k, m in enumerate(meshes):
        if regions[k] != case["target"]:
            continue
        count = 20000
        p, n = samples(m, count, rng)
        probes = p + n * eps
        buried = np.zeros(count, dtype=bool)
        covered = np.zeros(count, dtype=bool)
        for j, other in enumerate(meshes):
            if j == k:
                continue
            if regions[j] == case["target"]:
                buried |= other.contains(probes)
            elif open_mesh[j]:
                for distance in (eps, cover / 4, cover / 2, 3 * cover / 4, cover):
                    covered |= other.contains(p + n * distance)
        counts = ~buried & (reached(probes, n) | covered)
        total += float(m.area * counts.mean())
    return total


data = json.load(open(sys.argv[1]))
print(json.dumps({case["name"]: {"area": measure(case, data["pitch"], data["eps"], data["cover"])} for case in data["cases"]}))
