"""🐍️ Oracle of the mesh-plane section: numpy cuts every triangle with the plane, shapely assembles the segments into closed faces, takes the even-odd region of their outer rings (holes included) and returns its net area and bounds.

Input: the path of a JSON file { "triangles": [[x0, y0, z0, x1, y1, z1, x2, y2, z2], ...], "axis": 0 | 1 | 2, "plane": number }.
Output: JSON { "area": net area of the section in square metres, "polygons": count, "bounds": [minA, minB, maxA, maxB] } in the two remaining axes in increasing order.
"""
import json
import sys

import numpy as np
from shapely.geometry import MultiLineString, Polygon
from shapely.ops import polygonize, unary_union

data = json.load(open(sys.argv[1], encoding="utf8"))
triangles = np.asarray(data["triangles"], dtype=float).reshape(-1, 3, 3)
axis, plane = int(data["axis"]), float(data["plane"])
keep = [k for k in range(3) if k != axis]
d = triangles[:, :, axis] - plane
above = d >= 0
crossing = ~(above.all(axis=1) | (~above).all(axis=1))
segments = []
for tri, dist in zip(triangles[crossing], d[crossing]):
    points = []
    for a, b in ((0, 1), (1, 2), (2, 0)):
        if (dist[a] >= 0) != (dist[b] >= 0):
            t = dist[a] / (dist[a] - dist[b])
            p = tri[a] + (tri[b] - tri[a]) * t
            points.append((round(float(p[keep[0]]), 7), round(float(p[keep[1]]), 7)))
    if len(points) == 2 and points[0] != points[1]:
        segments.append(points)
faces = list(polygonize(unary_union(MultiLineString(segments)))) if segments else []
region = None
for face in faces:
    ring = Polygon(face.exterior)
    region = ring if region is None else region.symmetric_difference(ring)
area = region.area if region is not None else 0.0
bounds = region.bounds if region is not None else [0, 0, 0, 0]
print(json.dumps({"area": area, "polygons": len(faces), "bounds": list(bounds)}))
