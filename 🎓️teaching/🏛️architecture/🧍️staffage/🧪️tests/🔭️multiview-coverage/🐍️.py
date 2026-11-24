"""🐍️ Oracle of the multiview coverage: independent third-party recomputation of its outputs.

- svgelements parses the `d` attribute of every path of the layer groups `staffage-fill|field|body|detail|texture|glazing` of an exported SVG and returns the union bounding box (paper millimetres); with `ids` (a regular expression) only the paths whose own id or whose group id matches, in any layer.
- svgelements returns the bounding box of every path of an exported SVG that is filled with a hatch pattern (`url(#staffage-pattern-<name>-…)`), with the name of the pattern.
- scipy computes the symmetric Hausdorff distance (`directed_hausdorff`) of point sets the test passes.
- shapely measures the distance between two polygons, the distance from a point to the nearest of several polygons (0 inside) and the bounds of point sets.
- The standard JSON parser reads the record of one asset of a matrix run and counts its cells by status and by failing check.

Input: path of a JSON document `{ "svgs": [{ "key", "svg", "ids"? }], "patterns": [{ "key", "svg" }], "hausdorff": [{ "key", "a", "b" }], "distances": [{ "key", "a", "b" }], "pointDistances": [{ "key", "point", "polygons" }], "bounds": [{ "key", "points" }], "records": [{ "key", "path" }] }`.
Output on stdout: JSON `{ "svgs": {key: [minX, minY, maxX, maxY] | null}, "patterns": {key: [{ "pattern", "box" }]}, "hausdorff": {key: number}, "distances": {key: number}, "pointDistances": {key: number}, "bounds": {key: [minX, minY, maxX, maxY]}, "records": {key: { "asset", "outcome", "statuses", "checks" }} }`.
"""

import json
import re
import sys
import xml.etree.ElementTree as ET

import numpy as np
from scipy.spatial.distance import directed_hausdorff
from shapely.geometry import MultiPoint, Point, Polygon
from svgelements import Path

LAYERS = {"staffage-fill", "staffage-field", "staffage-body", "staffage-detail", "staffage-texture", "staffage-glazing"}


SVG = "{http://www.w3.org/2000/svg}"


def union(boxes):
    if not boxes:
        return None
    return [min(b[0] for b in boxes), min(b[1] for b in boxes), max(b[2] for b in boxes), max(b[3] for b in boxes)]


def path_boxes(nodes):
    boxes = []
    for node in nodes:
        d = node.get("d")
        if not d:
            continue
        box = Path(d).bbox()
        if box is not None:
            boxes.append(box)
    return boxes


def svg_box(text, ids=None):
    root = ET.fromstring(text.encode("utf-8"))
    if ids is None:
        return union([box for group in root.iter(SVG + "g") if group.get("id") in LAYERS for box in path_boxes(group.iter(SVG + "path"))])
    wanted = re.compile(ids)
    nodes = [node for node in root.iter(SVG + "path") if wanted.search(node.get("id") or "")]
    nodes += [node for group in root.iter(SVG + "g") if wanted.search(group.get("id") or "") for node in group.iter(SVG + "path")]
    return union(path_boxes(nodes))


def svg_patterns(text):
    root = ET.fromstring(text.encode("utf-8"))
    found = []
    for node in root.iter(SVG + "path"):
        match = re.match(r"url\(#staffage-pattern-([a-z]+)-", node.get("fill") or "")
        if match and node.get("d"):
            found.append({"pattern": match.group(1), "box": list(Path(node.get("d")).bbox())})
    return found


def point_distance(point, polygons):
    return min(Polygon(polygon).distance(Point(point)) for polygon in polygons)


def record(path):
    with open(path, encoding="utf-8") as handle:
        document = json.load(handle)
    statuses = {}
    checks = {}
    for cell in document.get("cells", []):
        statuses[cell["status"]] = statuses.get(cell["status"], 0) + 1
        for check in cell["failures"]:
            checks[check] = checks.get(check, 0) + 1
    return {"asset": document["asset"], "outcome": document["outcome"], "statuses": statuses, "checks": checks}


def hausdorff(a, b):
    a = np.array(a, dtype=float)
    b = np.array(b, dtype=float)
    return float(max(directed_hausdorff(a, b)[0], directed_hausdorff(b, a)[0]))


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        job = json.load(handle)
    out = {
        "svgs": {item["key"]: svg_box(item["svg"], item.get("ids")) for item in job.get("svgs", [])},
        "patterns": {item["key"]: svg_patterns(item["svg"]) for item in job.get("patterns", [])},
        "pointDistances": {item["key"]: point_distance(item["point"], item["polygons"]) for item in job.get("pointDistances", [])},
        "bounds": {item["key"]: list(MultiPoint(item["points"]).bounds) for item in job.get("bounds", [])},
        "records": {item["key"]: record(item["path"]) for item in job.get("records", [])},
        "hausdorff": {item["key"]: hausdorff(item["a"], item["b"]) for item in job.get("hausdorff", [])},
        "distances": {item["key"]: Polygon(item["a"]).distance(Polygon(item["b"])) for item in job.get("distances", [])},
    }
    print(json.dumps(out))


main()
