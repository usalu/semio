#!/usr/bin/env python3
"""🐍️ Independent path model of the data directories (stdlib os.path).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os
if os.sep == "/":
    for c in corpus["hub"]:
        if os.path.join(c["root"], ".🧬semio", "🌐hub") != c["expected"]:
            failures.append("hub")
    for c in corpus["space"]:
        base = os.path.join(c["root"], ".🧬semio", "🔗space")
        if (os.path.join(base, c["instance"]) if c["instance"] else base) != c["expected"]:
            failures.append("space")
    for c in corpus["map"]:
        if os.path.join(c["root"], ".🧬semio", "🗺" + VS + "map", c["provider"]) != c["expected"]:
            failures.append("map")

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
