"""⭕️ Oracle of the roundness of the tessellated solids: numpy and trimesh recompute the segments of the rule and measure, on the very triangles and normals the kernel produced, how round each mesh is.

Input: the path of a JSON document `{"meshes": [{"name", "kind": "cylinder" | "sphere" | "capsule" | "loft" | "revolved", "radiusM", "lengthM", "chordM", "featureM", "minSegments", "maxSegments", "roundSegments", "positions", "normals", "indices"}]}`; every mesh is about the Y axis from y = 0.
Output: JSON `{name: {"expectedSegments", "meridians", "gapM", "normalErrorDeg", "hardRim", "triangles"}}`:
- `expectedSegments`: min(maxSegments, max(minSegments, ceil(pi / acos(1 - chordM / radiusM)), roundSegments when radiusM > featureM));
- `meridians`: the distinct azimuths about the axis of the vertices off the axis (trimesh vertices);
- `gapM`: the radius minus the smallest distance from the axis (sphere: from the centre) of the face centres and edge midpoints of the round surface (trimesh `triangles_center`, `edges_unique`, `face_normals`); `kind` "revolved" measures none;
- `normalErrorDeg`: the largest angle between a vertex normal of the round surface and the analytic normal;
- `hardRim`: the vertices of the rims (y = 0 or y = length at the full radius) whose normal is axial."""
import json
import math
import sys

import numpy as np
import trimesh


def measure(entry):
    r, length = entry["radiusM"], entry["lengthM"]
    ratio = min(1.0, entry["chordM"] / r)
    chord = entry["minSegments"] if ratio >= 1 else math.ceil(math.pi / math.acos(1 - ratio))
    floor = entry["roundSegments"] if r > entry["featureM"] else 0
    expected = min(entry["maxSegments"], max(entry["minSegments"], chord, floor))
    vertices = np.array(entry["positions"], dtype=float).reshape(-1, 3)
    normals = np.array(entry["normals"], dtype=float).reshape(-1, 3)
    mesh = trimesh.Trimesh(vertices, np.array(entry["indices"]).reshape(-1, 3), process=False)
    radial = np.hypot(vertices[:, 0], vertices[:, 2])
    off = radial > 1e-9 * max(r, 1e-9) + 1e-12
    turns = np.round(np.arctan2(vertices[off, 2], vertices[off, 0]) / (2 * math.pi) * 1e6).astype(int) % 1000000
    out = {"expectedSegments": expected, "meridians": int(len(np.unique(turns))), "triangles": int(len(mesh.faces)), "gapM": 0.0, "normalErrorDeg": 0.0, "hardRim": 0}
    kind = entry["kind"]
    if kind == "revolved":
        return out
    mid = (mesh.vertices[mesh.edges_unique[:, 0]] + mesh.vertices[mesh.edges_unique[:, 1]]) / 2
    if kind == "sphere":
        points = np.vstack([mesh.triangles_center, mid])
        out["gapM"] = float(r - np.linalg.norm(points, axis=1).min())
        analytic = vertices / np.linalg.norm(vertices, axis=1)[:, None]
        cosine = np.clip((normals * analytic).sum(axis=1), -1, 1)
        out["normalErrorDeg"] = float(np.degrees(np.arccos(cosine)).max())
        return out
    tolerance = 1e-9 * r
    barrel = np.abs(mesh.face_normals[:, 1]) < 0.3
    inside = (mesh.triangles[:, :, 1].min(axis=1) >= -tolerance) & (mesh.triangles[:, :, 1].max(axis=1) <= length + tolerance)
    centres = mesh.triangles_center[barrel & inside]
    on = np.abs(radial - r) < 1e-6 * r
    between = (mid[:, 1] >= -tolerance) & (mid[:, 1] <= length + tolerance) & on[mesh.edges_unique[:, 0]] & on[mesh.edges_unique[:, 1]]
    points = np.vstack([centres, mid[between]])
    out["gapM"] = float(r - np.hypot(points[:, 0], points[:, 2]).min())
    side = (np.abs(normals[:, 1]) < 0.3) & (np.abs(radial - r) < 1e-6 * r) & (vertices[:, 1] >= -tolerance) & (vertices[:, 1] <= length + tolerance)
    analytic = np.zeros_like(vertices[side])
    analytic[:, 0] = vertices[side, 0] / radial[side]
    analytic[:, 2] = vertices[side, 2] / radial[side]
    cosine = np.clip((normals[side] * analytic).sum(axis=1), -1, 1)
    out["normalErrorDeg"] = float(np.degrees(np.arccos(cosine)).max())
    rim = (np.abs(radial - r) < 1e-6 * r) & ((np.abs(vertices[:, 1]) < tolerance) | (np.abs(vertices[:, 1] - length) < tolerance)) & (np.abs(normals[:, 1]) > 0.9)
    out["hardRim"] = int(rim.sum())
    return out


document = json.load(open(sys.argv[1], encoding="utf-8"))
json.dump({entry["name"]: measure(entry) for entry in document["meshes"]}, sys.stdout)
