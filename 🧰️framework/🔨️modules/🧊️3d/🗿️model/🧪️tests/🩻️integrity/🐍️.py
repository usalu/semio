"""🩻️ Oracle of the kernel mesh integrity: trimesh and shapely re-derive, from the very triangles the kernel tessellated, what `meshIntegrity` and `outwardNormals` claim.

Input: the path of a JSON document `{"cases": [...]}`; a case has a `name`, `positions` and `indices` (a triangle mesh) and optionally `polygon` (a ring of [axial, radial] points of a revolve profile closed through the axis, and `loop`, the ring of the chord closure).
Output: JSON `{name: {...}}` with
- `watertight`, `windingConsistent`, `isVolume`, `volume` (signed, trimesh mass properties), `area`;
- with `normals` (the kernel's claimed outward unit normal per triangle, flattened) and `components` (triangle ranges `[first, count]` of the closed bodies, default all): `outwardFaces` and `testedFaces`, the number of faces whose probe moved 1e-4 of the body size out along the normal lies outside its body and whose probe moved the same distance in lies inside, by trimesh `contains` (ray parity, independent of the winding);
- `polygonArea`, `polygonCentroidRadial`, `pappusVolume` (2 pi x area x radial centroid, the volume of the revolved polygon) and `loopArea`, from shapely.
"""
import json
import sys

import numpy as np
import trimesh
from shapely.geometry import Polygon


def mesh_of(case):
    """The triangles as a mesh with coincident vertices merged (the tessellator duplicates vertices along creases; trimesh judges topology on shared vertices)."""
    m = trimesh.Trimesh(vertices=np.array(case["positions"], dtype=float).reshape(-1, 3), faces=np.array(case["indices"], dtype=np.int64).reshape(-1, 3), process=False)
    m.merge_vertices()
    return m


def probe(case):
    faces = np.array(case["indices"], dtype=np.int64).reshape(-1, 3)
    vertices = np.array(case["positions"], dtype=float).reshape(-1, 3)
    normals = np.array(case["normals"], dtype=float).reshape(-1, 3)
    agree = tested = 0
    for first, count in case.get("components") or [[0, len(faces)]]:
        body = trimesh.Trimesh(vertices=vertices, faces=faces[first:first + count], process=False)
        body.merge_vertices()
        n = normals[first:first + count]
        live = np.linalg.norm(n, axis=1) > 0.5
        eps = 1e-4 * float(np.linalg.norm(body.bounds[1] - body.bounds[0]))
        centres = body.triangles_center
        outside = body.contains(centres + n * eps)
        inside = body.contains(centres - n * eps)
        agree += int(np.sum(live & ~outside & inside))
        tested += int(np.sum(live))
    return {"outwardFaces": agree, "testedFaces": tested}


def answer(case):
    out = {}
    if case.get("indices"):
        m = mesh_of(case)
        out.update(watertight=bool(m.is_watertight), windingConsistent=bool(m.is_winding_consistent), isVolume=bool(m.is_volume), volume=float(m.volume), area=float(m.area))
        if case.get("normals"):
            out.update(probe(case))
    if case.get("polygon"):
        polygon = Polygon(case["polygon"])
        out.update(polygonArea=float(polygon.area), polygonCentroidRadial=float(polygon.centroid.y), pappusVolume=float(2 * np.pi * polygon.area * polygon.centroid.y))
    if case.get("loop"):
        out.update(loopArea=float(Polygon(case["loop"]).area))
    return out


if __name__ == "__main__":
    with open(sys.argv[1], encoding="utf8") as handle:
        cases = json.load(handle)["cases"]
    print(json.dumps({case["name"]: answer(case) for case in cases}))
