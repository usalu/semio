"""MuPDF oracle of the DeviceCMYK image conversion.

Usage: uv run --with pymupdf==1.26.7 python -I 📜️cmyk-image-clut.py write

MuPDF converts DeviceCMYK image samples to sRGB through its default CMYK profile, not through the corner interpolation it uses for fill colours.
`write` samples that conversion on a 17 x 17 x 17 x 17 grid (cyan, magenta, yellow, black; black varies fastest; nodes at round(i * 255 / 16)) and stores
three bytes per node in `🌈️cmyk-image-clut.bin`; `check` verifies the stored grid against MuPDF again.
"""
import itertools
import pathlib
import sys

import pymupdf

GRID = 17
TABLE = pathlib.Path(__file__).with_name("🌈️cmyk-image-clut.bin")


def nodes():
    step = [round(index * 255 / (GRID - 1)) for index in range(GRID)]
    return [bytes(point) for point in itertools.product(step, repeat=4)]


def convert(points):
    cmyk = pymupdf.Pixmap(pymupdf.csCMYK, len(points), 1, b"".join(points), False)
    return pymupdf.Pixmap(pymupdf.csRGB, cmyk).samples


def main(mode):
    samples = convert(nodes())
    if mode == "write":
        TABLE.write_bytes(samples)
    elif TABLE.read_bytes() != samples:
        sys.exit("stored CMYK image grid differs from MuPDF")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "check")
