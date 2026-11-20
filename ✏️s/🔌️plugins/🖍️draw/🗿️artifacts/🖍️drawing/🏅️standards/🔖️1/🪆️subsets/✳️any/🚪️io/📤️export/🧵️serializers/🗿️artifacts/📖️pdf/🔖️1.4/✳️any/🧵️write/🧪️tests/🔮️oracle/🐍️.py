"""🔮️ Samples RGBA from each one-page PDF framed by its four-byte big-endian length."""
import sys

import pypdfium2 as pdfium
from pypdfium2 import raw


def sample(data: bytes) -> bytes:
    pixels = bytearray()
    with pdfium.PdfDocument(data) as document:
        if len(document) != 1:
            raise ValueError("expected one Draw PDF page")
        page = document[0]
        try:
            bitmap = page.render(scale=256, force_bitmap_format=raw.FPDFBitmap_BGRA)
            try:
                if (bitmap.width, bitmap.height) != (512, 256):
                    raise ValueError("unexpected Draw PDF raster extent")
                view = memoryview(bitmap.buffer).cast("B")
                for x in (128, 384):
                    offset = 128 * bitmap.stride + x * 4
                    blue, green, red, alpha = view[offset : offset + 4]
                    pixels.extend((red, green, blue, alpha))
            finally:
                bitmap.close()
        finally:
            page.close()
    return bytes(pixels)


source = sys.stdin.buffer
while header := source.read(4):
    size = int.from_bytes(header, "big")
    if len(header) != 4 or not 0 < size <= 65_536:
        raise ValueError("invalid Draw PDF oracle frame")
    data = source.read(size)
    if len(data) != size:
        raise ValueError("truncated Draw PDF oracle frame")
    sys.stdout.buffer.write(sample(data))
