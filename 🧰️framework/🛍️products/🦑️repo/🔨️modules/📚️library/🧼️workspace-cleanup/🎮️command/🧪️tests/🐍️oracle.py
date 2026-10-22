#!/usr/bin/env python3
"""🐍️ Independent model of the workspace cleanup entry grammar: closed mode, optional dry marker, pass-through for taxonomy and test (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



MODES = ("workspace", "coverage", "empty-folders", "marker-only-folders")


def parse(seg):
    if not isinstance(seg, list) or any(not isinstance(s, str) for s in seg):
        raise ValueError("closed")
    if seg and seg[0] in ("taxonomy", "test"):
        return {"mode": seg[0], "dry": False, "arguments": seg[1:]}
    marker = lambda s: s in ("--dry", "dry")
    mode = "workspace" if not seg or marker(seg[0]) else seg[0]
    tail = [] if not seg else seg if marker(seg[0]) else seg[1:]
    if mode not in MODES or len(tail) > 1 or any(not marker(t) for t in tail):
        raise ValueError("closed")
    return {"mode": mode, "dry": len(tail) == 1, "arguments": []}


for c in corpus["accepted"]:
    if parse(c["segments"]) != {"mode": c["mode"], "dry": c["dry"], "arguments": c["arguments"]}:
        failures.append("accept %r" % c["segments"])
for c in corpus["rejected"]:
    try:
        parse(c["segments"])
        failures.append("reject %r" % c["segments"])
    except ValueError:
        pass
for v in corpus["nonArrays"]:
    try:
        parse(v)
        failures.append("non-array %r" % v)
    except ValueError:
        pass

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
