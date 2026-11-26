import json
import os
from pathlib import Path
import sys


def outcomes(root, row):
    keys = set()
    decisions = []
    for checkout in row["checkoutOrder"]:
        cwd = root / row["crate"] / checkout
        artifact = cwd / "target" / ("lib" + row["crate"] + ".rlib")
        key = cwd if os.fsencode(cwd) in artifact.read_bytes() else None
        decisions.append("local-hit" if key in keys else "miss")
        keys.add(key)
    return decisions


fixture, root, case = sys.argv[1:]
rows = json.loads(Path(fixture).read_text(encoding="utf-8"))["cases"]
print(json.dumps(outcomes(Path(root), next(row for row in rows if row["crate"] == case))))
