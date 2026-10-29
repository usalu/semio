"""🐍️ Oracle for the PDF writer: reportlab's Helvetica metrics (Adobe AFM) for the strings the writer measures."""
import json
import sys

from reportlab.pdfbase.pdfmetrics import stringWidth

strings = json.loads(sys.argv[1])
print(json.dumps([stringWidth(text, "Helvetica-Bold" if bold else "Helvetica", 1000) for text, bold in strings]))
