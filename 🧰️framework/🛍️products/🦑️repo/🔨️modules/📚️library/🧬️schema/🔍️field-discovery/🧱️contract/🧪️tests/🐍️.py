#!/usr/bin/env python3
"""🐍️ Independent model of the field-discovery contract (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



SC = {"String": "string", "string": "string", "bool": "bool", "boolean": "bool", "Boolean": "bool", "i32": "int32", "u32": "uint32", "i64": "int64", "u64": "uint64", "f32": "float32", "f64": "float64", "Int": "int32", "Float": "float64", "bytes": "bytes", "Vec<u8>": "bytes"}
for c in corpus["state"]:
    if c["raw"].strip().lower().replace("_", "-") != c["expected"]:
        failures.append("state %r" % c["raw"])
for c in corpus["scalar"]:
    t = re.sub(r"\s+", "", c["raw"])
    if SC.get(t, t) != c["expected"]:
        failures.append("scalar %r" % c["raw"])
for c in corpus["snake"]:
    p = c["name"].split("_")
    if p[0] + "".join(x[:1].upper() + x[1:] for x in p[1:]) != c["expected"]:
        failures.append("snake %r" % c["name"])
for c in corpus["find"]:
    found = None
    if c["expected"]:
        for m in re.finditer(c["pattern"].replace("\\w", "[A-Za-z0-9_]"), c["text"]):
            if m.group(1) == c["expected"]:
                found = {"typeName": m.group(1), "bodyStart": m.end()}
                break
    if found != c["found"]:
        failures.append("find: " + c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
