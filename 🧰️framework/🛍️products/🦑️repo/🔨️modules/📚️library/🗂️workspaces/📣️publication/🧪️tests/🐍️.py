#!/usr/bin/env python3
"""🐍️ Independent check of the written root document: the file must equal `json.dumps` of the corpus document with the expected membership (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



if len(sys.argv) > 2:
    written = open(sys.argv[2], encoding="utf8").read()
    doc = dict(corpus["rootDocument"])
    doc["workspaces"] = corpus["expected"]
    if written != json.dumps(doc, indent=2, ensure_ascii=False) + "\n":
        failures.append("written document")
if corpus["rendered"] != json.dumps(dict(corpus["rootDocument"], workspaces=corpus["expected"]), indent=2, ensure_ascii=False) + "\n":
    failures.append("rendered fixture")
for name, s in corpus["scenarios"].items():
    cur = s["document"]["workspaces"]
    exp = corpus["expected"]
    fresh = cur == exp
    if s["mode"] == "check" and (s["throws"] is None) != fresh:
        failures.append("scenario " + name)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
