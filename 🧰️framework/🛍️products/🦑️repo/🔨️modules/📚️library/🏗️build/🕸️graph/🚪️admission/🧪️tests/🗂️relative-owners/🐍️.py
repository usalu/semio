#!/usr/bin/env python3
"""🐍️ Independent oracle of the relative import ownership: the deepest owner root that is the path or an ancestor of it, ties by the smallest name (locale-free code point order is enough for the ASCII names of the fixtures), the workspace root `.` owns nothing.

usage: 🐍️.py < {"worlds":[{"owners":[{"name","root"}],"paths":[...]}]}   prints {"answers":[[name|null,...],...]}"""
import json, sys

def answer(world):
    answers = []
    for path in world["paths"]:
        best = None
        for owner in world["owners"]:
            root = owner["root"]
            if root == "." or not (path == root or path.startswith(root + "/")):
                continue
            if best is None or len(root) > len(best["root"]) or (len(root) == len(best["root"]) and owner["name"] < best["name"]):
                best = owner
        answers.append(best["name"] if best else None)
    return answers


print(json.dumps({"answers": [answer(world) for world in json.load(sys.stdin)["worlds"]]}))
