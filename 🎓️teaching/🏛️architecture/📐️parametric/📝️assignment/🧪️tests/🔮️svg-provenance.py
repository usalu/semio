"""🔮 Independently parse and verify the exact canonical Metabolism SVG source icons."""

import hashlib
import json
from pathlib import Path

from lxml import etree

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework/🛍️products/📓️print").exists())
FIXTURE = json.loads((HERE / "🧫️cases.json").read_text(encoding="utf-8"))
NS = {"svg": "http://www.w3.org/2000/svg"}
projection = []
for table in FIXTURE["exercise"]["tables"]:
    rows = []
    for index, row in enumerate(table["types"]):
        source = ROOT / row["icon"]
        raw = source.read_bytes()
        document = etree.fromstring(raw)
        assert document.tag == "{http://www.w3.org/2000/svg}svg"
        assert not document.xpath("//svg:image", namespaces=NS)
        assert hashlib.sha256(raw).hexdigest() == row["sha256"]
        viewbox = [float(value) for value in document.get("viewBox").split()]
        paths = len(document.xpath("//svg:path", namespaces=NS))
        assert viewbox == row["viewBox"]
        assert paths == row["paths"]
        assert row["family"] == ("capsule" if index % 2 == 0 else "ellipsoid-capsule")
        assert all(node.get("d", "").strip() for node in document.xpath("//svg:path", namespaces=NS))
        rows.append({"family": row["family"], "variant": row["variant"], "viewBox": viewbox, "paths": paths})
    projection.append(rows)
assert len({row["icon"] for table in FIXTURE["exercise"]["tables"] for row in table["types"]}) == 16
print("lxml: 16 canonical Metabolism SVG icons verified")
