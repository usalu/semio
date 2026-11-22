"""MuPDF oracle of the PDF 1.4 page canvas fixture.

Usage: uv run --with pymupdf==1.26.5 python -I pdf14-canvas-oracle.py write|check [fixture.json]

Every case page is written the way the PDF 1.4 writer saves it (one Helvetica text object at 72, height - 72)
and rendered by MuPDF at `scale`. `write` stores the page-local probe colours found in that render
(white paper, and the centres of fully black 3 x 3 pixel blocks of the glyphs); `check` renders again,
verifies every stored probe and verifies every search expectation with MuPDF's own text search.
"""
import json
import pathlib
import sys
import zlib

import pymupdf

FIXTURE = pathlib.Path(__file__).with_name("🧪️pdf14-canvas.json")
INK_PROBES = 6


def literal(text):
    out = bytearray(b"(")
    for byte in text.encode("utf-8"):
        out += {0x5C: b"\\\\", 0x28: b"\\(", 0x29: b"\\)", 0x0D: b"\\r", 0x0A: b"\\n"}.get(byte, bytes([byte]))
    return bytes(out + b")")


def content(page):
    if not page["text"]:
        return b"BT\nET\n"
    return b"BT\n/F1 12 Tf\n72 %s Td\n" % number(page["height"] - 72).encode() + literal(page["text"]) + b" Tj\nET\n"


def number(value):
    return ("%f" % value).rstrip("0").rstrip(".") if value != int(value) else str(int(value))


def document(pages):
    objects = {1: b"<< /Type /Catalog /Pages 2 0 R >>", 3: b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"}
    kids = []
    for index, page in enumerate(pages):
        page_number, content_number = 4 + 2 * index, 5 + 2 * index
        packed = zlib.compress(content(page))
        objects[content_number] = b"<< /Length %d /Filter /FlateDecode >>\nstream\n" % len(packed) + packed + b"\nendstream"
        objects[page_number] = ("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 %s %s] /Contents %d 0 R /Resources << /Font << /F1 3 0 R >> >> >>" % (number(page["width"]), number(page["height"]), content_number)).encode()
        kids.append("%d 0 R" % page_number)
    objects[2] = ("<< /Type /Pages /Kids [%s] /Count %d >>" % (" ".join(kids), len(pages))).encode()
    out = bytearray(b"%PDF-1.4\n")
    offsets = {}
    for key in sorted(objects):
        offsets[key] = len(out)
        out += b"%d 0 obj\n" % key + objects[key] + b"\nendobj\n"
    start = len(out)
    out += b"xref\n0 %d\n0000000000 65535 f \n" % (len(objects) + 1)
    for key in sorted(objects):
        out += b"%010d 00000 n \n" % offsets[key]
    out += b"trailer\n<< /Size %d /Root 1 0 R >>\nstartxref\n%d\n%%%%EOF\n" % (len(objects) + 1, start)
    return bytes(out)


def render(page, scale):
    return page.get_pixmap(matrix=pymupdf.Matrix(scale, scale), alpha=False, annots=False)


def rgb(pixmap, x, y):
    return list(pixmap.pixel(x, y))[:3]


def ink(pixmap):
    black = lambda x, y: rgb(pixmap, x, y) == [0, 0, 0]
    found = []
    for y in range(1, pixmap.height - 1):
        for x in range(1, pixmap.width - 1):
            if all(black(x + dx, y + dy) for dx in (-1, 0, 1) for dy in (-1, 0, 1)):
                found.append((x, y))
    step = max(1, len(found) // INK_PROBES)
    return found[::step][:INK_PROBES]


def probes_of(case, scale):
    doc = pymupdf.open("pdf", document(case["pages"]))
    found = []
    for index, (spec, page) in enumerate(zip(case["pages"], doc)):
        pixmap = render(page, scale)
        for at in ([10, 10], [spec["width"] - 10, spec["height"] - 10]):
            found.append({"page": index, "at": at, "rgb": rgb(pixmap, int(at[0] * scale), int(at[1] * scale))})
        for x, y in ink(pixmap):
            found.append({"page": index, "at": [(x + 0.5) / scale, (y + 0.5) / scale], "rgb": rgb(pixmap, x, y)})
    return found


def search_hits(case, search):
    doc = pymupdf.open("pdf", document(case["pages"]))
    if search["matchCase"]:
        return sum(page.get_text().count(search["query"]) for page in doc)
    return sum(len(page.search_for(search["query"])) for page in doc)


def main():
    mode = sys.argv[1]
    path = pathlib.Path(sys.argv[2]) if len(sys.argv) > 2 else FIXTURE
    fixture = json.loads(path.read_text())
    scale = fixture["scale"]
    failures = []
    for case in fixture["cases"]:
        if mode == "write":
            case["probes"] = probes_of(case, scale)
            continue
        doc = pymupdf.open("pdf", document(case["pages"]))
        pixmaps = [render(page, scale) for page in doc]
        for probe in case["probes"]:
            pixmap = pixmaps[probe["page"]]
            actual = rgb(pixmap, int(probe["at"][0] * scale), int(probe["at"][1] * scale))
            if actual != probe["rgb"]:
                failures.append((case["name"], probe, actual))
        for search in case.get("search", []):
            hits = search_hits(case, search)
            if hits != search["hits"]:
                failures.append((case["name"], search, hits))
    if mode == "write":
        path.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n")
        print("wrote", sum(len(case["probes"]) for case in fixture["cases"]), "probes")
        return
    for failure in failures:
        print("MISMATCH", failure)
    print("pymupdf", pymupdf.VersionBind, "checked", sum(len(case["probes"]) for case in fixture["cases"]), "probes", "failures", len(failures))
    sys.exit(1 if failures else 0)


main()
