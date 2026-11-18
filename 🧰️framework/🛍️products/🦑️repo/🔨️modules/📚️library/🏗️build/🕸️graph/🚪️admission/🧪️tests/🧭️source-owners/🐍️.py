#!/usr/bin/env python3
"""🐍️ Independent oracle of the source ownership of the admission closure: among the owner roots (and declared native roots) that contain the path (the workspace root `.` contains every path), the longest root wins, equal lengths keep the smallest name.

usage: 🐍️.py < {"worlds":[{"owners":[{"name","root","nativeRoots"?}],"paths":[...]}]}   prints {"answers":[[name|null,...],...]}"""
import json, sys

def answer(world):
    rows = []
    for owner in world["owners"]:
        for root in [owner["root"], *owner.get("nativeRoots", [])]:
            rows.append((root, owner["name"]))
    result = []
    for path in world["paths"]:
        best = None
        for root, name in rows:
            if not (root == "." or path == root or path.startswith(root + "/")):
                continue
            if best is None or len(root) > len(best[0]) or (len(root) == len(best[0]) and name < best[1]):
                best = (root, name)
        result.append(best[1] if best else None)
    return result

print(json.dumps({"answers": [answer(world) for world in json.load(sys.stdin)["worlds"]]}))
