#!/usr/bin/env python3
"""🐍️ Independent model of the subcommand dispatch selection (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



HANDLERS = {"a", "run"}
for c in corpus["dispatch"]:
    key = c["segments"][0] if c["segments"] else c["default"]
    handler = key if key in HANDLERS else None
    if (handler is None) != c["usage"]:
        failures.append("dispatch %r" % c["segments"])
    elif handler is not None and (key != c["key"] or c["segments"][1:] != c["calls"]):
        failures.append("dispatch args %r" % c["segments"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
