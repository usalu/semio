#!/usr/bin/env python3
"""🐍️ Independent rendering of the root package document with Python's json module (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



for c in corpus["render"]:
    doc = json.loads(c["source"])
    doc["workspaces"] = list(c["workspaces"])
    if json.dumps(doc, indent=2, ensure_ascii=False) + "\n" != c["expected"]:
        failures.append("render")
for c in corpus["parse"]:
    doc = json.loads(c["source"])
    if doc.get("workspaces", []) != c["workspaces"]:
        failures.append("parse " + c["name"])
for c in corpus["rejects"]:
    try:
        doc = json.loads(c["source"])
    except ValueError:
        continue
    ok = isinstance(doc, dict) and ("workspaces" not in doc or (isinstance(doc["workspaces"], list) and all(isinstance(w, str) for w in doc["workspaces"])))
    if ok:
        failures.append("accepted " + c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
