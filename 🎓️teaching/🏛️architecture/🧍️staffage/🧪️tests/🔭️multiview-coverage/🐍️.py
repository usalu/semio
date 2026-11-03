"""🐍️ Oracle of the multiview coverage: independent third-party recomputation of three outputs.

- svgelements parses the `d` attribute of every path of the layer groups `staffage-fill|field|body|detail|texture|glazing` of an exported SVG and returns the union bounding box (paper millimetres).
- scipy computes the symmetric Hausdorff distance (`directed_hausdorff`) of point sets the test passes.
- shapely measures the distance between two polygons.

Input: path of a JSON document `{ "svgs": [{ "key", "svg" }], "hausdorff": [{ "key", "a", "b" }], "distances": [{ "key", "a", "b" }] }`.
Output on stdout: JSON `{ "svgs": {key: [minX, minY, maxX, maxY] | null}, "hausdorff": {key: number}, "distances": {key: number} }`.
"""

import json
import sys
import xml.etree.ElementTree as ET

import numpy as np
from scipy.spatial.distance import directed_hausdorff
from shapely.geometry import Polygon
from svgelements import Path

LAYERS = {"staffage-fill", "staffage-field", "staffage-body", "staffage-detail", "staffage-texture", "staffage-glazing"}


def svg_box(text):
    root = ET.fromstring(text.encode("utf-8"))
    boxes = []
    for group in root.iter("{http://www.w3.org/2000/svg}g"):
        if group.get("id") not in LAYERS:
            continue
        for node in group.iter("{http://www.w3.org/2000/svg}path"):
            d = node.get("d")
            if not d:
                continue
            box = Path(d).bbox()
            if box is not None:
                boxes.append(box)
    if not boxes:
        return None
    return [min(b[0] for b in boxes), min(b[1] for b in boxes), max(b[2] for b in boxes), max(b[3] for b in boxes)]


def hausdorff(a, b):
    a = np.array(a, dtype=float)
    b = np.array(b, dtype=float)
    return float(max(directed_hausdorff(a, b)[0], directed_hausdorff(b, a)[0]))


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        job = json.load(handle)
    out = {
        "svgs": {item["key"]: svg_box(item["svg"]) for item in job.get("svgs", [])},
        "hausdorff": {item["key"]: hausdorff(item["a"], item["b"]) for item in job.get("hausdorff", [])},
        "distances": {item["key"]: Polygon(item["a"]).distance(Polygon(item["b"])) for item in job.get("distances", [])},
    }
    print(json.dumps(out))


main()
