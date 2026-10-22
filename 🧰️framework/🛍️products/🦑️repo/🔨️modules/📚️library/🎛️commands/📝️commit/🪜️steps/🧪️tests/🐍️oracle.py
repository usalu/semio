#!/usr/bin/env python3
"""🐍️ Independent model of the commit step words with Python's own regex engine (same `\b` semantics; stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def steps(seg):
    t = " ".join(seg).lower()
    return {"tag": re.search(r"\b(ct|ctag|tag!|\+tag)\b", t) is not None, "squash": re.search(r"\b(cs|csquash|squash!|\+squash)\b", t) is not None, "push": re.search(r"\b(cp|cpush|push!|\+push)\b", t) is not None}


def prepare_only(seg):
    t = " ".join(seg).lower()
    if re.search(r"\b(c\.|cprepare|prepare!|\+prepare)\b", t):
        return True
    s = steps(seg)
    return not (s["tag"] or s["squash"] or s["push"])


for c in corpus["parse"]:
    if steps(c["segments"]) != c["expected"]:
        failures.append("steps %r" % c["segments"])
for c in corpus["parseDefects"]:
    if steps(c["segments"]) == c["expected"]:
        failures.append("model fixed %r" % c["segments"])
for c in corpus["prepareOnly"]:
    if prepare_only(c["segments"]) != c["expected"]:
        failures.append("prepare %r" % c["segments"])
for c in corpus["prepareOnlyDefects"]:
    if prepare_only(c["segments"]) == c["expected"]:
        failures.append("model fixed prepare %r" % c["segments"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
