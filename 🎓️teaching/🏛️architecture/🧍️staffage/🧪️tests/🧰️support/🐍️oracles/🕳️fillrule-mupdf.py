"""🕳️ SVG rasteriser oracle of the fill rule (MuPDF, wheel-bundled): renders an SVG file at `pixels-per-mm` into a binary PPM. Usage: fillrule-mupdf.py <svg> <pixels-per-mm> <out.ppm>."""
import json
import sys

import pymupdf

PT_PER_MM = 72 / 25.4
source, per_mm, target = sys.argv[1], float(sys.argv[2]), sys.argv[3]
page = pymupdf.open(source)[0]
pixmap = page.get_pixmap(matrix=pymupdf.Matrix(per_mm / PT_PER_MM, per_mm / PT_PER_MM), alpha=False)
with open(target, "wb") as file:
    file.write(b"P6\n%d %d\n255\n" % (pixmap.width, pixmap.height))
    file.write(pixmap.samples)
print(json.dumps({"widthPx": pixmap.width, "heightPx": pixmap.height}))
