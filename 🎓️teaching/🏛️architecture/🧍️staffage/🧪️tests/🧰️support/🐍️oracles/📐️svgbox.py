"""📐️ SVG geometry oracle (svgelements): the bounding box of every shape of an SVG file in the user units of the root, with the transforms and the viewports of nested svg elements applied. Usage: svgbox.py <svg>; prints a JSON list of {id, fill, minX, minY, maxX, maxY}."""
import json
import sys

from svgelements import SVG, Shape

document = SVG.parse(sys.argv[1], reify=True)
boxes = []
for element in document.elements():
    if isinstance(element, Shape):
        box = element.bbox()
        if box is not None:
            boxes.append({"id": element.id, "fill": str(element.values.get("fill", "")), "minX": box[0], "minY": box[1], "maxX": box[2], "maxY": box[3]})
print(json.dumps(boxes))
