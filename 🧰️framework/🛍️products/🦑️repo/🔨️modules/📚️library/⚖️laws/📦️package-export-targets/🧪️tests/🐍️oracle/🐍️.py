#!/usr/bin/env python3
"""Independent model of the package export target law (standard library only): usage `oracle.py ROOT LIST.json` prints path -> missing export targets as JSON. It shares only data with the TypeScript law."""
import json, os, sys

GENERATED = {"dist", "build", "out", "pkg", "🕸️bindings"}

def targets(value):
    if isinstance(value, str):
        return [value] if value.startswith("./") and "*" not in value and not (set(value.split("/")) & GENERATED) else []
    if isinstance(value, list):
        return [t for item in value for t in targets(item)]
    if isinstance(value, dict):
        return [t for item in value.values() for t in targets(item)]
    return []

root, listing = sys.argv[1], json.load(open(sys.argv[2], encoding="utf8"))
out = {}
for path in listing:
    try:
        exports = json.load(open(os.path.join(root, path), encoding="utf8")).get("exports")
    except Exception:
        continue
    missing = [t for t in targets(exports) if not os.path.exists(os.path.join(root, os.path.dirname(path), t))]
    if missing:
        out[path] = missing
print(json.dumps(out, ensure_ascii=False))
