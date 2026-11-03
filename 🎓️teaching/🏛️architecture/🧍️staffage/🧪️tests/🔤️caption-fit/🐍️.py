"""🔤️ Oracle of the text widths: PyMuPDF measures every character in its built-in Helvetica (`helv`) and Helvetica-Bold (`hebo`) and sums the advances (whole-string measuring drops accented letters).

Input: the path of a JSON list `[{"text": ..., "size": mm, "bold": bool}]`. Output: JSON list of widths in the same unit as the size.
"""
import json
import sys
import warnings

warnings.simplefilter("ignore")
import fitz  # noqa: E402



def width(entry):
    font = "hebo" if entry["bold"] else "helv"
    return sum(fitz.get_text_length(char, fontname=font, fontsize=entry["size"]) for char in entry["text"])


print(json.dumps([width(entry) for entry in json.load(open(sys.argv[1], encoding="utf8"))]))
