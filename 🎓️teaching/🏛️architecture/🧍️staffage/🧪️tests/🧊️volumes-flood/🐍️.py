"""🌊️ Oracle of the flood from outside (C31) and of the contents of closed and glazed cavities (E3): trimesh and scipy decide, from the very triangles the kernel tessellated, which faces of a region the air from outside reaches and which belong to the contents of a closed cavity.

Input: the path of a JSON document `{"pitch": metres, "eps": metres, "cover": metres, "seam": metres, "cases": [{"name", "target": region, "transparent": [region], "solids": [{"region", "positions", "indices"}]}]}`.
Output: JSON `{name: {"area": m2, "seen": m2}}`.

Method, sharing no code with the product: every mesh is voxelised at `pitch` (its trimesh triangles sampled on a lattice of half a pitch along the shortest edge and the next one, the cells of the samples nudged by a quarter pitch in every direction are surface cells), the cells no surface touches are labelled into connected components (scipy `ndimage.label`, six neighbours) and the component of the grid corner is the outside. The faces of the target region are sampled by area (systematically along the cumulated face areas, uniformly within a face); a sample is dropped when its probe `eps` out of the face lies inside another mesh of the target region (parity of the crossings of one skewed ray with the triangles trimesh welded and oriented, in numpy); it reaches when the first free cell met by stepping out along the normal, or along one of the four directions 45 degrees to it (a ledge narrower than a cell beside another surface), at most three pitches, belongs to the outside, or when its probe lies inside a mesh of another region, or such a mesh is met within `cover` in front of the face, that mesh has itself a sample the outside reaches and, stepping on along the normal by half of `cover`, the first point that lies outside every mesh (and stays so for `cover`) is reached by the outside (a part that passes through a closed shell covers nothing inside it). A sample that does neither is a cavity sample when, stepping from it against the normal in quarter pitches, the first point that lies outside every mesh of the target region (and stays so for `seam`) is not reached by the outside; the region counts its cavity samples only when they outweigh the others (E3). `seen` floods again without the meshes of the `transparent` regions and counts the samples whose probe that flood reaches."""
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


RAY = np.array([0.000123, 0.000317, 1.0]) / np.linalg.norm([0.000123, 0.000317, 1.0])


def within(m, points):
    near = np.flatnonzero(np.all((points >= m.bounds[0]) & (points <= m.bounds[1]), axis=1))
    hit = np.zeros(len(points), dtype=bool)
    if len(near) == 0:
        return hit
    tri = m.triangles
    origin, e1, e2 = tri[:, 0], tri[:, 1] - tri[:, 0], tri[:, 2] - tri[:, 0]
    h = np.cross(RAY, e2)
    det = (e1 * h).sum(axis=1)
    sound = np.abs(det) > 1e-18
    inv = np.where(sound, 1 / np.where(sound, det, 1), 0)
    chunk = max(1, 300_000 // len(tri))
    for start in range(0, len(near), chunk):
        rows = near[start:start + chunk]
        s = points[rows, None, :] - origin[None]
        u = (s * h).sum(axis=2) * inv
        q = np.cross(s, e1[None])
        v = (q * RAY).sum(axis=2) * inv
        t = (q * e2[None]).sum(axis=2) * inv
        crossings = (sound & (u >= 0) & (v >= 0) & (u + v <= 1) & (t > 0)).sum(axis=1)
        hit[rows] = crossings % 2 == 1
    return hit


def samples(m, n, rng):
    faces = np.minimum(len(m.faces) - 1, np.searchsorted(np.cumsum(m.area_faces) / m.area_faces.sum(), (rng.random() + np.arange(n)) / n))
    r = rng.random((n, 2))
    flip = r.sum(axis=1) > 1
    r[flip] = 1 - r[flip]
    tri = m.triangles[faces]
    return tri[:, 0] + r[:, :1] * (tri[:, 1] - tri[:, 0]) + r[:, 1:] * (tri[:, 2] - tri[:, 0]), m.face_normals[faces]


def surface(m, spacing):
    tri = m.triangles
    lengths = np.linalg.norm(tri[:, [1, 2, 0]] - tri, axis=2)
    first = np.argmin(lengths, axis=1)
    rows = np.arange(len(tri))
    a, b, c = tri[rows, first], tri[rows, (first + 1) % 3], tri[rows, (first + 2) % 3]
    ab, ac = b - a, c - a
    n1 = np.maximum(1, np.ceil(np.linalg.norm(ab, axis=1) / spacing).astype(int))
    n2 = np.maximum(1, np.ceil(np.linalg.norm(ac, axis=1) / spacing).astype(int))
    for k1, k2 in np.unique(np.stack([n1, n2], axis=1), axis=0):
        pick = np.flatnonzero((n1 == k1) & (n2 == k2))
        i, j = np.meshgrid(np.arange(k1 + 1) / k1, np.arange(k2 + 1) / k2, indexing="ij")
        keep = i + j <= 1 + 1e-12
        u, v = i[keep][None, :, None], j[keep][None, :, None]
        chunk = max(1, 500_000 // int(keep.sum()))
        for start in range(0, len(pick), chunk):
            rows = pick[start:start + chunk]
            yield (a[rows, None] + u * ab[rows, None] + v * ac[rows, None]).reshape(-1, 3)


def flood(meshes, lo, shape, pitch):
    wall = np.zeros(shape, dtype=bool)
    for m in meshes:
        for points in surface(m, pitch / 2):
            for dx in (-0.26, 0.26):
                for dy in (-0.26, 0.26):
                    for dz in (-0.26, 0.26):
                        idx = np.floor((points + np.array([dx, dy, dz]) * pitch - lo) / pitch).astype(int)
                        wall[idx[:, 0], idx[:, 1], idx[:, 2]] = True
    labels, _ = ndimage.label(~wall)
    outside = labels[0, 0, 0]

    def along(points, directions):
        out = np.zeros(len(points), dtype=bool)
        done = np.zeros(len(points), dtype=bool)
        for step in np.arange(0.25, 3.01, 0.25):
            idx = np.clip(np.floor((points + directions * step * pitch - lo) / pitch).astype(int), 0, np.array(shape) - 1)
            free = ~wall[idx[:, 0], idx[:, 1], idx[:, 2]] & ~done
            out[free] = labels[idx[free, 0], idx[free, 1], idx[free, 2]] == outside
            done |= free
        return out

    def reached(points, normals):
        out = along(points, normals)
        if len(points) == 0:
            return out
        side = np.cross(normals, np.where(np.abs(normals[:, :1]) < 0.9, [[1.0, 0.0, 0.0]], [[0.0, 1.0, 0.0]]))
        side /= np.linalg.norm(side, axis=1, keepdims=True)
        other = np.cross(normals, side)
        for tangent in (side, -side, other, -other):
            todo = np.flatnonzero(~out)
            out[todo] = along(points[todo], (normals[todo] + tangent[todo]) / np.sqrt(2))
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
            hit |= within(m, points)
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

    def inside_any(points):
        hit = np.zeros(len(points), dtype=bool)
        for m in meshes:
            hit |= within(m, points)
        return hit

    def beyond(p, n):
        out = np.ones(len(p), dtype=bool)
        todo = np.arange(len(p))
        step = cover / 2
        ahead = [~inside_any(p + n * (eps + k * step)) for k in range(2)]
        k = 0
        while len(todo) > 0 and eps + k * step < span:
            ahead.append(np.zeros(len(p), dtype=bool))
            ahead[k + 2][todo] = ~inside_any(p[todo] + n[todo] * (eps + (k + 2) * step))
            free = ahead[k][todo] & ahead[k + 1][todo] & ahead[k + 2][todo]
            exits = todo[free]
            out[exits] = reached(p[exits] + n[exits] * (eps + k * step), n[exits])
            todo = todo[~free]
            k += 1
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
                buried |= within(other, probes)
            elif open_mesh[j]:
                for distance in (eps, cover / 4, cover / 2, 3 * cover / 4, cover):
                    covered |= within(other, p + n * distance)
        air = reached(probes, n)
        behind = np.flatnonzero(~buried & ~air & covered)
        covered[behind] = beyond(p[behind], n[behind])
        counts = ~buried & (air | covered)
        candidates = np.flatnonzero(~buried & ~counts)
        inner = np.zeros(count, dtype=bool)
        inner[candidates] = ~closure(p[candidates], n[candidates])
        outer += float(m.area * counts.mean())
        cavity += float(m.area * inner.mean())
        seen += float(m.area * (~buried & seen_from(probes, n)).mean())
    return {"area": outer + (cavity if cavity > outer else 0.0), "seen": seen}


data = json.load(open(sys.argv[1]))
print(json.dumps({case["name"]: measure(case, data["pitch"], data["eps"], data["cover"], data["seam"]) for case in data["cases"]}))
