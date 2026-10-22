#!/usr/bin/env python3
"""🐍️ Independent crate-budget parser (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def parse(text):
    parts = text.split("/")
    if len(parts) != 3 or any(not re.match(r"^[1-9][0-9]*$", p) or int(p) > 9007199254740991 for p in parts):
        raise ValueError("budget")
    return {"files": int(parts[0]), "bytes": int(parts[1]), "lines": int(parts[2])}


for c in corpus["ok"]:
    if parse(c["text"]) != c["expected"]:
        failures.append("ok " + c["text"])
for t in corpus["errors"]:
    try:
        parse(t)
        failures.append("accepted %r" % t)
    except ValueError:
        pass

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
