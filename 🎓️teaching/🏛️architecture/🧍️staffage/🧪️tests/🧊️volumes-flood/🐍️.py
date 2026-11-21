"""🌊️ Oracle of the flood from outside (C31) and of the contents of closed and glazed cavities (E3): trimesh and scipy decide, from the very triangles the kernel tessellated, which faces of a region the air from outside reaches and which belong to the contents of a closed cavity.

Input: the path of a JSON document `{"pitch": metres, "eps": metres, "cover": metres, "seam": metres, "cases": [{"name", "target": region, "transparent": [region], "solids": [{"region", "positions", "indices"}]}]}`.
Output: JSON `{name: {"area": m2, "seen": m2}}`.

Method, sharing no code with the product: every mesh is voxelised at `pitch` (trimesh `voxelize`, surface cells), the cells no surface touches are labelled into connected components (scipy `ndimage.label`, six neighbours) and the component of the grid corner is the outside. The faces of the target region are sampled by area; a sample is dropped when its probe `eps` out of the face lies inside another mesh of the target region (trimesh `contains`); it reaches when the first free cell met by stepping out along the normal (at most three pitches) belongs to the outside, or when its probe lies inside a mesh of another region, or such a mesh is met within `cover` in front of the face, and that mesh has itself a sample the outside reaches. A sample that does neither is a cavity sample when, stepping from it against the normal in quarter pitches, the first point that lies outside every mesh of the target region (and stays so for `seam`) is not reached by the outside; the region counts its cavity samples only when they outweigh the others (E3). `seen` floods again without the meshes of the `transparent` regions and counts the samples whose probe that flood reaches."""
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


def flood(meshes, lo, shape, pitch):
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
            idx = np.clip(np.floor((points + normals * step * pitch - lo) / pitch).astype(int), 0, np.array(shape) - 1)
            free = ~wall[idx[:, 0], idx[:, 1], idx[:, 2]] & ~done
            out[free] = labels[idx[free, 0], idx[free, 1], idx[free, 2]] == outside
            done |= free
        return out

    return reached


def measure(case, pitch, eps, cover, seam):
    meshes = [solid(e) for e in case["solids"]]
    regions = [e["region"] for e in case["solids"]]
    lo = np.min([m.bounds[0] for m in meshes], axis=0) - 4 * pitch
    hi = np.max([m.bounds[1] for m in meshes], axis=0) + 4 * pitch
    shape = tuple(np.ceil((hi - lo) / pitch).astype(int) + 1)
    reached = flood(meshes, lo, shape, pitch)
    clear = set(case.get("transparent", []))
    seen_from = flood([m for m, r in zip(meshes, regions) if r not in clear], lo, shape, pitch)
    own = [m for m, r in zip(meshes, regions) if r == case["target"]]
    span = float(np.linalg.norm(hi - lo))

    def inside_own(points):
        hit = np.zeros(len(points), dtype=bool)
        for m in own:
            hit |= m.contains(points)
        return hit

    def closure(p, n):
        out = np.ones(len(p), dtype=bool)
        todo = np.arange(len(p))
        t = eps
        while len(todo) > 0 and t < span:
            free = ~inside_own(p[todo] - n[todo] * t)
            for extra in (seam / 2, seam):
                free &= ~inside_own(p[todo] - n[todo] * (t + extra))
            exits = todo[free]
            out[exits] = reached(p[exits] - n[exits] * t, -n[exits])
            todo = todo[~free]
            t += pitch / 4
        return out

    rng = np.random.default_rng(31)
    open_mesh = []
    for m in meshes:
        p, n = samples(m, 4000, rng)
        open_mesh.append(bool(reached(p + n * eps, n).any()))
    outer = 0.0
    cavity = 0.0
    seen = 0.0
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
        candidates = np.flatnonzero(~buried & ~counts)
        inner = np.zeros(count, dtype=bool)
        inner[candidates] = ~closure(p[candidates], n[candidates])
        outer += float(m.area * counts.mean())
        cavity += float(m.area * inner.mean())
        seen += float(m.area * (~buried & seen_from(probes, n)).mean())
    return {"area": outer + (cavity if cavity > outer else 0.0), "seen": seen}


data = json.load(open(sys.argv[1]))
print(json.dumps({case["name"]: measure(case, data["pitch"], data["eps"], data["cover"], data["seam"]) for case in data["cases"]}))
