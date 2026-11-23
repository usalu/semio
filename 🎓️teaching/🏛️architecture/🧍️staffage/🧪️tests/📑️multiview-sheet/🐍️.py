"""🐍️ Oracle for the multiview sheet: lxml reads the SVG, PyMuPDF the PDF and ezdxf the DXF, and each reports the physical size of the sheet and the extent of the figure in every view, measured from the file alone."""
import json
import re
import sys

import ezdxf
import pymupdf
from lxml import etree

VIEWS = ("elevation", "front", "plan")
SKIPPED_LAYERS = ("staffage-ground", "staffage-annotations", "staffage-legend", "staffage-background", "staffage-forces", "staffage-diagrams")
NUMBER = re.compile(r"-?\d+(?:\.\d+)?(?:e-?\d+)?")
MM_PER_PT = 25.4 / 72
PDF_OVERLAYS = ("Ground", "Forces and supports", "Diagrams", "Annotations", "Legend", "Background")


def grow(box, x, y):
    if box is None:
        return [x, y, x, y]
    return [min(box[0], x), min(box[1], y), max(box[2], x), max(box[3], y)]


def svg_report(path):
    root = etree.parse(path).getroot()
    ns = {"s": "http://www.w3.org/2000/svg", "f": "https://semio.tech/staffage/v2"}
    boxes = {view: None for view in VIEWS}
    for element in root.iter("{http://www.w3.org/2000/svg}path"):
        chain = [element, *element.iterancestors()]
        ids = [node.get("id", "") for node in chain]
        layer = next((i for i in ids if i.startswith("staffage-")), "")
        if layer in SKIPPED_LAYERS:
            continue
        view = next((v for i in ids for v in VIEWS if i.startswith(v + "-")), None)
        if view is None:
            continue
        numbers = [float(n) for n in NUMBER.findall(re.sub(r"[A-Za-z]", " ", element.get("d")))]
        for x, y in zip(numbers[0::2], numbers[1::2]):
            boxes[view] = grow(boxes[view], x, y)
    scene = root.find(".//f:scene", ns)
    return {
        "widthMm": float(root.get("width").removesuffix("mm")),
        "heightMm": float(root.get("height").removesuffix("mm")),
        "viewBox": [float(v) for v in root.get("viewBox").split()],
        "boxes": boxes,
        "scale": scene.get("scale"),
        "mmPerMetre": float(scene.get("mm-per-metre")),
        "projection": scene.get("projection"),
        "title": root.find("s:title", ns).text,
        "desc": root.find("s:desc", ns).text,
        "attributes": dict(scene.attrib),
        "origins": {v.get("viewpoint"): [float(v.get("origin-x-mm")), float(v.get("origin-y-mm"))] for v in scene.findall("f:view", ns)},
    }


def visible(points, clips):
    """👁️ The corner points of what a clipped item shows: its points as they are without a clip, else the corners of its box cut by every enclosing clip rectangle (none when nothing of it shows)."""
    if not clips or not points:
        return points
    box = [min(p[0] for p in points), min(p[1] for p in points), max(p[0] for p in points), max(p[1] for p in points)]
    for clip in clips:
        box = [max(box[0], clip.x0), max(box[1], clip.y0), min(box[2], clip.x1), min(box[3], clip.y1)]
    return [] if box[0] > box[2] or box[1] > box[3] else [(box[0], box[1]), (box[2], box[3])]


def pdf_report(path, regions):
    document = pymupdf.open(path)
    page = document[0]
    found = {view: None for view in VIEWS}
    clips = {}
    for drawing in page.get_drawings(extended=True):
        level = drawing["level"]
        clips = {depth: rect for depth, rect in clips.items() if depth < level}
        if drawing["type"] == "clip":
            clips[level] = drawing["scissor"]
            continue
        if drawing["type"] == "group" or drawing.get("layer") in PDF_OVERLAYS:
            continue
        for item in drawing["items"]:
            points = [(p.x, p.y) for p in item[1:] if hasattr(p, "x")] if item[0] != "re" else [(item[1].x0, item[1].y0), (item[1].x1, item[1].y1)]
            for px, py in visible(points, list(clips.values())):
                for view, region in regions.items():
                    x, y = px * MM_PER_PT, py * MM_PER_PT
                    if region[0] - 0.5 <= x <= region[2] + 0.5 and region[1] - 0.5 <= y <= region[3] + 0.5:
                        found[view] = grow(found[view], x, y)
    return {"pages": len(document), "widthMm": page.rect.width * MM_PER_PT, "heightMm": page.rect.height * MM_PER_PT, "boxes": found, "metadata": document.metadata}


def dxf_report(path):
    document = ezdxf.readfile(path)
    errors = [str(e.message) for e in document.audit().errors]
    boxes = {view: None for view in VIEWS}
    layers = sorted(layer.dxf.name for layer in document.layers)
    for line in document.modelspace().query("LWPOLYLINE"):
        name = line.dxf.layer
        view = name.split("-")[0].lower()
        if view not in VIEWS or name.endswith("GROUND") or "-ANALYSIS-" in name:
            continue
        for x, y in line.get_points("xy"):
            boxes[view] = grow(boxes[view], x, y)
    texts = [{"layer": e.dxf.layer, "text": e.dxf.text} for e in document.modelspace().query("TEXT")]
    return {"errors": errors, "insunits": document.header["$INSUNITS"], "projectname": document.header.get("$PROJECTNAME"), "boxes": boxes, "layers": layers, "texts": texts}


jobs = json.load(open(sys.argv[1], encoding="utf-8"))
results = {}
for name, files in jobs.items():
    svg = svg_report(files["svg"])
    results[name] = {"svg": svg, "pdf": pdf_report(files["pdf"], svg["boxes"]), "dxf": None if files["dxf"] is None else dxf_report(files["dxf"])}
print(json.dumps(results))
