#!/usr/bin/env python3
"""🐍️ Independent set-difference model of the field comparison (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



for i, c in enumerate(corpus["cases"]):
    r, a = set(c["reference"]), set(c["candidate"])
    if sorted(r - a) != c["missing"] or sorted(a - r) != c["extra"]:
        failures.append("case %d" % i)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
