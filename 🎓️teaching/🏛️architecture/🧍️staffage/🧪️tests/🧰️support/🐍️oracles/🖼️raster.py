"""🖼️ SVG rasteriser oracle (PyMuPDF/MuPDF, wheel-bundled): measures one element or emulates the rsvg-convert calls of the export CLI. Usage: raster.py measure <svg> <element-id> <pixels-per-mm> | raster.py rsvg <rsvg-convert arguments>."""
import json
import re
import sys

import pymupdf

PT_PER_MM = 72 / 25.4


def render(source, zoom):
    document = pymupdf.open(stream=source, filetype="svg")
    return document[0].get_pixmap(matrix=pymupdf.Matrix(zoom, zoom), alpha=True)


def measure(path, element, pixels_per_mm):
    text = open(path, encoding="utf8").read()
    root = re.search(r"<svg\b[^>]*>", text).group(0)
    body = re.search(r'<g id="%s"[^>]*>.*?</g>' % re.escape(element), text, re.S).group(0)
    pixmap = render((root + body + "</svg>").encode("utf8"), pixels_per_mm / PT_PER_MM)
    alpha = memoryview(pixmap.samples)[pixmap.n - 1 :: pixmap.n]
    rows = [y for y in range(pixmap.height) if any(alpha[y * pixmap.width + x] >= 128 for x in range(pixmap.width))]
    columns = [x for x in range(pixmap.width) if any(alpha[y * pixmap.width + x] >= 128 for y in range(pixmap.height))]
    return {"widthPx": columns[-1] - columns[0] + 1, "heightPx": rows[-1] - rows[0] + 1, "canvasWidthPx": pixmap.width, "canvasHeightPx": pixmap.height}


def rsvg(argv):
    options = {"-b": None}
    while len(argv) > 1:
        flag = argv.pop(0)
        options[flag] = argv.pop(0)
    svg = argv[0]
    dpi = float(options["-d"])
    source = pymupdf.open(svg)[0].get_pixmap(matrix=pymupdf.Matrix(dpi / 72, dpi / 72), alpha=True)
    width, height = int(options["--page-width"]), int(options["--page-height"])
    canvas = pymupdf.Pixmap(pymupdf.csRGB, pymupdf.IRect(0, 0, width, height), True)
    canvas.clear_with(0)
    if options["-b"]:
        colour = options["-b"].lstrip("#")
        canvas.set_rect(canvas.irect, tuple(int(colour[i : i + 2], 16) for i in (0, 2, 4)) + (255,))
    source.set_origin(int(options["--left"]), int(options["--top"]))
    canvas.copy(source, pymupdf.IRect(source.irect) & pymupdf.IRect(canvas.irect))
    canvas.save(options["-o"])
    return {"widthPx": width, "heightPx": height}


command = sys.argv[1]
print(json.dumps(measure(sys.argv[2], sys.argv[3], float(sys.argv[4])) if command == "measure" else rsvg(sys.argv[2:])))
