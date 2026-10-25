#!/usr/bin/env python3
"""🐍️ Independent common-directory model with os.path (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def lca(files):
    parts = [os.path.dirname(f).split("/") if "/" in f else ["."] for f in files]
    if not parts:
        return ""
    n = len(parts[0])
    for p in parts:
        i = 0
        while i < n and i < len(p) and p[i] == parts[0][i]:
            i += 1
        n = i
    return "/".join(parts[0][:n])


import os
for c in corpus["lca"]:
    if lca(c["files"]) != c["expected"]:
        failures.append("lca %r" % c["files"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
