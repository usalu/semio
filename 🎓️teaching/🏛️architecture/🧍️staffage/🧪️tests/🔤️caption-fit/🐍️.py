"""🔤️ Oracle of the text widths: PyMuPDF measures every character in its built-in Helvetica (`helv`) and Helvetica-Bold (`hebo`), or in the Anta files of the repo (`"font": "anta"`, the face the SVG writer names first; the first of the four subsets that has the glyph), and sums the advances (whole-string measuring drops accented letters).

Input: the path of a JSON list `[{"text": ..., "size": mm, "bold": bool, "font": "helvetica" | "anta"}]` and the directory of the Anta subsets. Output: JSON list of widths in the same unit as the size.
"""
import json
import sys
import warnings

warnings.simplefilter("ignore")
import fitz  # noqa: E402



SUBSETS = ["🏛️latin", "➕️latin-ext", "🧮️math", "🔣️symbols"]
anta = [fitz.Font(fontfile=f"{sys.argv[2]}/{subset}/📖️regular/🔤️outline.ttf") for subset in SUBSETS] if len(sys.argv) > 2 else []


def anta_advance(char, size):
    for font in anta:
        if font.has_glyph(ord(char)):
            return font.text_length(char, fontsize=size)
    return size


def width(entry):
    if entry.get("font") == "anta":
        return sum(anta_advance(char, entry["size"]) for char in entry["text"])
    font = "hebo" if entry["bold"] else "helv"
    return sum(fitz.get_text_length(char, fontname=font, fontsize=entry["size"]) for char in entry["text"])


print(json.dumps([width(entry) for entry in json.load(open(sys.argv[1], encoding="utf8"))]))
