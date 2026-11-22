"""🔗️ Oracle of the contact measure behind `volumes-connected`: trimesh and scipy decide, from the same triangles, which parts touch and how far a group is from the rest.

Input: the path of a JSON document `{"tolerance": metres, "cap": metres, "spacing": metres, "cases": [{"name", "parts": [{"positions", "indices"}]}]}`.
Output: JSON `{name: {"labels": [group label per part], "gaps": [[metres between part i and part j]]}}`; a gap beyond `cap` is `cap`.

Method, sharing no code with the product: every part is remeshed until no edge is longer than `spacing` (trimesh `remesh.subdivide_to_size`), so its vertices sample its whole surface; the distance from part A to part B is the least distance of
those samples to the surface of B (trimesh `proximity.closest_point`, an R-tree over the triangles) and the other way round, and 0 when a sample of one lies inside the other (trimesh `contains`). Parts whose distance is at most
`tolerance` are joined and scipy `sparse.csgraph.connected_components` labels the groups.

With `"spans": {"mesh": {"positions", "indices"}, "samples": count, "segments": [{"name", "from", "to"}]}` in the input the output also carries `"spans": {name: metres}`: how far a bone leaves a member solid that spans it, the largest distance of
`samples` evenly spaced points of the segment to the mesh (trimesh `proximity.closest_point`), a point inside the mesh (trimesh `contains`) counting 0."""
import json
import sys

import numpy as np
import trimesh
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import connected_components


def part(entry, spacing):
    vertices = np.array(entry["positions"], dtype=float).reshape(-1, 3)
    faces = np.array(entry["indices"], dtype=np.int64).reshape(-1, 3)
    mesh = trimesh.Trimesh(vertices, faces, process=False)
    mesh.merge_vertices()
    samples, _ = trimesh.remesh.subdivide_to_size(vertices, faces, spacing)
    return mesh, np.vstack([vertices, samples])


def gap(first, second, cap):
    (mesh_a, points_a), (mesh_b, points_b) = first, second
    if np.any(mesh_a.bounds[0] - cap > mesh_b.bounds[1]) or np.any(mesh_b.bounds[0] - cap > mesh_a.bounds[1]):
        return cap
    if mesh_b.contains(points_a).any() or mesh_a.contains(points_b).any():
        return 0.0
    _, d_ab, _ = trimesh.proximity.closest_point(mesh_b, points_a)
    _, d_ba, _ = trimesh.proximity.closest_point(mesh_a, points_b)
    return float(min(d_ab.min(), d_ba.min(), cap))


data = json.load(open(sys.argv[1], encoding="utf8"))
out = {}
for case in data["cases"]:
    parts = [part(entry, data["spacing"]) for entry in case["parts"]]
    n = len(parts)
    gaps = np.zeros((n, n))
    for i in range(n):
        for j in range(i + 1, n):
            gaps[i, j] = gaps[j, i] = gap(parts[i], parts[j], data["cap"])
    _, labels = connected_components(csr_matrix(gaps <= data["tolerance"]), directed=False)
    out[case["name"]] = {"labels": labels.tolist(), "gaps": gaps.tolist()}
if "spans" in data:
    vertices = np.array(data["spans"]["mesh"]["positions"], dtype=float).reshape(-1, 3)
    solid = trimesh.Trimesh(vertices, np.array(data["spans"]["mesh"]["indices"], dtype=np.int64).reshape(-1, 3), process=False)
    solid.merge_vertices()
    out["spans"] = {}
    for segment in data["spans"]["segments"]:
        points = np.linspace(segment["from"], segment["to"], data["spans"]["samples"])
        _, distances, _ = trimesh.proximity.closest_point(solid, points)
        out["spans"][segment["name"]] = float(np.where(solid.contains(points), 0.0, distances).max())
print(json.dumps(out))
