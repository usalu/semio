"""📕️ PDF oracle (pikepdf with the bundled qpdf, pypdfium2 with PDFium): structure check, metadata, page boxes, text and raster. Usage: pdf.py inspect <pdf> | pdf.py text <pdf> | pdf.py render <pdf> <dpi> <page-1-based> <out.ppm>."""
import json
import sys

import pikepdf
import pypdfium2


def problems(pdf):
    found = [str(problem) for problem in pdf.check_pdf_syntax()]
    for stream in (item for item in pdf.objects if isinstance(item, pikepdf.Stream)):
        try:
            stream.read_bytes()
        except Exception as error:
            found.append("stream %s: %s" % (stream.objgen, error))
    return found


def inspect(path):
    try:
        pdf = pikepdf.open(path)
    except pikepdf.PdfError as error:
        return {"problems": [str(error)]}
    with pdf:
        info = pdf.docinfo
        return {
            "problems": problems(pdf),
            "version": pdf.pdf_version,
            "pages": len(pdf.pages),
            "title": str(info.get("/Title", "")),
            "subject": str(info.get("/Subject", "")),
            "mediaBoxes": [[float(value) for value in page.MediaBox] for page in pdf.pages],
        }


def text(path):
    document = pypdfium2.PdfDocument(path)
    return {"pages": [document[index].get_textpage().get_text_range() for index in range(len(document))]}


def render(path, dpi, page, target):
    image = pypdfium2.PdfDocument(path)[page - 1].render(scale=dpi / 72).to_pil().convert("RGB")
    with open(target, "wb") as file:
        file.write(b"P6\n%d %d\n255\n" % image.size)
        file.write(image.tobytes())
    return {"widthPx": image.width, "heightPx": image.height}


command, path = sys.argv[1], sys.argv[2]
print(json.dumps(inspect(path) if command == "inspect" else text(path) if command == "text" else render(path, float(sys.argv[3]), int(sys.argv[4]), sys.argv[5])))
