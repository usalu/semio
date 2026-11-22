"""🧊️ Oracle of the volume rules: shapely and trimesh reproduce, from the very triangles the kernel tessellated, the areas, volumes and silhouettes that `checkVolumes` measures.

Input: the path of a JSON document `{"cases": [...]}`. A case has a `name`, a list `meshes` (each `{"positions": [...], "indices": [...]}`, a closed triangle mesh in world metres) and optionally `other`
(a second list of meshes), `view` (`elevation`, `front` or `plan`) and `cell` (grid pitch in metres for the volume).

Output: JSON `{name: {"unionVolume": ..., "outerArea": ..., "meshVolume": ..., "triangles": ..., "iou": ..., "hausdorff": ...}}`:
- `unionVolume` m3: trimesh `contains` on a regular grid of `cell` metres (the volume of the union of all meshes);
- `meshVolume` m3: the sum of the signed volumes of the meshes (trimesh), equal to the union for disjoint meshes;
- `outerArea` m2: the area of the triangles of every mesh whose centroid lies in none of the other meshes (trimesh `contains`);
- `exposedArea` m2 (case key `exposed`): the area of the faces of one mesh that lie in no contact plane, within no section-face cone and whose probe moved `eps` out of the face lies in no other mesh;
- `area` (case key `unionOuter`: `eps`): the surface of the meshes not inside another mesh, by 20000 random surface samples per mesh moved `eps` out along the face normal (trimesh `sample_surface`, `contains`), the independent outer area of a union;
- `width` (case key `minrect`: points): the shorter side of the minimum rotated rectangle of the shape (shapely), its smallest width for a rectangle or a capsule;
- `pathLength` (case key `pappus`: `mesh`, `profile`): the length of the centre line of a swept tube by Pappus, the volume of the mesh (trimesh) over the area of the cross-section profile (shapely);
- `area` of an annulus (case key `annulus`: `radius`, `width`): shapely area of the ring between two buffered points;
- `area` and `closedArea` (case key `closing`: `rects`, `radius`): shapely area of the union of axis-aligned rectangles and of its morphological closing (a gap narrower than twice the radius is filled, one wider stays);
- `area` of strokes (case key `stroke`: `lines`, `width`): shapely area of the union of the lines buffered by half the width with square caps (a pen stroke);
- `area` of the retained half (case key `half`: `axis`, `plane`, `keepHigh`, `view`): shapely area of the silhouette of the meshes cut by trimesh `slice_plane` (the cap lies in the view plane, so the clipped triangles already fill the outline) and kept on one side of the plane;
- `band` (case key): the volumes of the fine union, the coarse union and the fine union eroded and dilated by a distance, on a regular grid with scipy's Euclidean distance transform;
- `outlineArea` (case key `outline`): shapely area of an explicit outline;
- `iou` (with `tolerance`: `iouTolerant`, the IoU where the part of one region farther than the tolerance from the other counts against it) and `hausdorff` (m): shapely `unary_union` of the projected triangles of `meshes` against `other`; the Hausdorff distance is that of the filled regions (the largest distance from a vertex of the part of one region outside the other to that other region, both ways).
"""
import json
import sys

import numpy as np
import trimesh
from scipy import ndimage
from shapely.geometry import Point, Polygon
from shapely.ops import unary_union

PROJECTION = {"elevation": (0, 1), "front": (2, 1), "plan": (0, 2)}


def mesh_of(entry):
    return trimesh.Trimesh(vertices=np.array(entry["positions"], dtype=float).reshape(-1, 3), faces=np.array(entry["indices"], dtype=np.int64).reshape(-1, 3), process=False)


def union_volume(meshes, cell):
    lo = np.min([m.bounds[0] for m in meshes], axis=0)
    hi = np.max([m.bounds[1] for m in meshes], axis=0)
    axes = [np.arange(lo[k] + cell / 2, hi[k], cell) for k in range(3)]
    return float(occupancy(meshes, axes).sum() * cell**3)


def occupancy(meshes, axes):
    shape = tuple(len(axis) for axis in axes)
    inside = np.empty(shape, dtype=bool)
    flat = inside.reshape(-1)
    for start in range(0, len(flat), 2048):
        stop = min(start + 2048, len(flat))
        indices = np.unravel_index(np.arange(start, stop), shape)
        points = np.column_stack(tuple(axis[index] for axis, index in zip(axes, indices)))
        flat[start:stop] = contains_any(meshes, points)
    return inside


def outer_area(meshes):
    total = 0.0
    for k, m in enumerate(meshes):
        centroids = m.triangles_center
        covered = np.zeros(len(centroids), dtype=bool)
        for j, other in enumerate(meshes):
            if j == k:
                continue
            near = np.all((centroids >= other.bounds[0]) & (centroids <= other.bounds[1]), axis=1)
            if near.any():
                covered[near] |= other.contains(centroids[near])
        total += float(m.area_faces[~covered].sum())
    return total


def contains_any(meshes, points):
    inside = np.zeros(len(points), dtype=bool)
    for m in meshes:
        near = np.all((points >= m.bounds[0]) & (points <= m.bounds[1]), axis=1)
        if near.any():
            inside[near] |= m.contains(points[near])
    return inside


def exposed_area(meshes, target, planes, sections, cosine, eps):
    m = meshes[target]
    normals = m.face_normals
    centroids = m.triangles_center
    keep = np.ones(len(normals), dtype=bool)
    for axis in sections.get(str(target), []):
        keep &= ~(normals @ np.array(axis) >= cosine)
    for plane in planes:
        n = np.array(plane["normal"])
        on = (np.abs(normals @ n) >= 0.9) & (np.abs((centroids - np.array(plane["point"])) @ n) <= plane["toleranceM"])
        keep &= ~on
    others = [o for k, o in enumerate(meshes) if k != target]
    buried = contains_any(others, centroids + normals * eps) if others else np.zeros(len(normals), dtype=bool)
    keep &= ~buried
    return float(m.area_faces[keep].sum())


def band_volumes(fine, coarse, distance, cell):
    lo = np.min([m.bounds[0] for m in fine + coarse], axis=0) - distance
    hi = np.max([m.bounds[1] for m in fine + coarse], axis=0) + distance
    axes = [np.arange(lo[k] + cell / 2, hi[k], cell) for k in range(3)]
    a = occupancy(fine, axes)
    cube = cell**3
    coarse_volume = float(occupancy(coarse, axes).sum() * cube)
    dilated = float((ndimage.distance_transform_edt(~a) * cell <= distance).sum() * cube)
    eroded = float((ndimage.distance_transform_edt(a) * cell > distance).sum() * cube)
    return {"fine": float(a.sum() * cube), "coarse": coarse_volume, "dilated": dilated, "eroded": eroded}


def directed(a, b):
    outside = a.difference(b)
    if outside.is_empty:
        return 0.0
    polygons = [outside] if outside.geom_type == "Polygon" else list(outside.geoms)
    worst = 0.0
    for polygon in polygons:
        if polygon.geom_type != "Polygon":
            continue
        for ring in [polygon.exterior, *polygon.interiors]:
            for x, y in ring.coords:
                worst = max(worst, b.distance(Point(x, y)))
    return worst


def silhouette(meshes, view):
    a, b = PROJECTION[view]
    polygons = []
    for m in meshes:
        for tri in m.triangles:
            polygon = Polygon(tri[:, [a, b]])
            if polygon.area > 1e-12:
                polygons.append(polygon.buffer(0))
    return unary_union(polygons)


data = json.load(open(sys.argv[1], encoding="utf8"))
out = {}
for case in data["cases"]:
    if "outline" in case:
        out[case["name"]] = {"outlineArea": float(Polygon(case["outline"]).area)}
        continue
    if "band" in case:
        out[case["name"]] = band_volumes([mesh_of(e) for e in case["band"]["fine"]], [mesh_of(e) for e in case["band"]["coarse"]], case["band"]["distance"], case["band"]["cell"])
        continue
    if "bounds" in case:
        from shapely.geometry import MultiPoint
        b = case["bounds"]
        first, second = MultiPoint(b["first"]).bounds, MultiPoint(b["second"]).bounds
        lo, hi = b["axis"], b["axis"] + 2
        out[case["name"]] = {"shift": float(max(abs(first[lo] - second[lo]), abs(first[hi] - second[hi]))), "extentDifference": float(abs((first[hi] - first[lo]) - (second[hi] - second[lo])))}
        continue
    if "minrect" in case:
        box = Polygon(case["minrect"]).minimum_rotated_rectangle.exterior.coords
        sides = [float(np.hypot(box[k + 1][0] - box[k][0], box[k + 1][1] - box[k][1])) for k in range(4)]
        out[case["name"]] = {"width": min(sides)}
        continue
    if "pappus" in case:
        c = case["pappus"]
        out[case["name"]] = {"pathLength": float(mesh_of(c["mesh"]).volume / Polygon(c["profile"]).area)}
        continue
    if "annulus" in case:
        c = case["annulus"]
        ring = Point(0, 0).buffer(c["radius"] + c["width"] / 2, 256).difference(Point(0, 0).buffer(c["radius"] - c["width"] / 2, 256))
        out[case["name"]] = {"area": float(ring.area)}
        continue
    if "closing" in case:
        from shapely.geometry import box
        c = case["closing"]
        union = unary_union([box(*r) for r in c["rects"]])
        out[case["name"]] = {"area": float(union.area), "closedArea": float(union.buffer(c["radius"], join_style=2).buffer(-c["radius"], join_style=2).area)}
        continue
    if "stroke" in case:
        from shapely.geometry import LineString
        st = case["stroke"]
        out[case["name"]] = {"area": float(unary_union([LineString(line).buffer(st["width"] / 2, cap_style="square") for line in st["lines"]]).area)}
        continue
    meshes = [mesh_of(entry) for entry in case["meshes"]]
    if "unionOuter" in case:
        rng = np.random.default_rng(7)
        total = 0.0
        for k, m in enumerate(meshes):
            points, faces = trimesh.sample.sample_surface(m, 20000, seed=7)
            probes = points + m.face_normals[faces] * case["unionOuter"]["eps"]
            buried = contains_any([o for j, o in enumerate(meshes) if j != k], probes)
            total += float(m.area * (1 - buried.mean()))
        out[case["name"]] = {"area": total}
        continue
    if "half" in case:
        h = case["half"]
        normal = np.zeros(3)
        normal[h["axis"]] = 1.0 if h["keepHigh"] else -1.0
        origin = np.zeros(3)
        origin[h["axis"]] = h["plane"]
        kept = []
        for m in meshes:
            whole = m.bounds[0][h["axis"]] >= h["plane"] if h["keepHigh"] else m.bounds[1][h["axis"]] <= h["plane"]
            if whole or (m.bounds[0][h["axis"]] < h["plane"] < m.bounds[1][h["axis"]]):
                kept.append(m.slice_plane(origin, normal, cap=False))
        out[case["name"]] = {"area": float(silhouette(kept, case["view"]).area)}
        continue
    if "exposed" in case:
        ex = case["exposed"]
        out[case["name"]] = {"exposedArea": exposed_area(meshes, ex["target"], ex.get("planes", []), ex.get("sections", {}), ex["cosine"], ex["eps"])}
        continue
    result = {"triangles": int(sum(len(m.faces) for m in meshes)), "meshVolume": float(sum(m.volume for m in meshes))}
    if "cell" in case:
        result["unionVolume"] = union_volume(meshes, case["cell"])
    if case.get("outer"):
        result["outerArea"] = outer_area(meshes)
    if "view" in case:
        first = silhouette(meshes, case["view"])
        result["area"] = float(first.area)
        if "other" in case:
            second = silhouette([mesh_of(entry) for entry in case["other"]], case["view"])
            result["iou"] = float(first.intersection(second).area / first.union(second).area)
            if "tolerance" in case:
                tol = case["tolerance"]
                lost = first.difference(second.buffer(tol)).area + second.difference(first.buffer(tol)).area
                result["iouTolerant"] = float(1 - lost / first.union(second).area)
            result["hausdorff"] = float(max(directed(first, second), directed(second, first)))
            if case.get("hull"):
                result["iouHull"] = float(first.convex_hull.intersection(second.convex_hull).area / first.convex_hull.union(second.convex_hull).area)
    out[case["name"]] = result
print(json.dumps(out))
