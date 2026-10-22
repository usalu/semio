#!/usr/bin/env python3
"""🐍️ Independent model of the CLI output contract: last JSON line wins, GraphQL errors fail, no JSON fails (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def payload(lines):
    last = None
    for l in lines:
        l = l.strip()
        if not l:
            continue
        try:
            last = json.loads(l)
        except ValueError:
            continue
    if not isinstance(last, (dict, list)):
        raise ValueError("no JSON lines")
    if isinstance(last, list):
        return last
    if last.get("errors"):
        raise ValueError("graphql errors: " + "; ".join(e["message"] for e in last["errors"]))
    return last.get("data", last)


for name, spec in corpus["fake"].items():
    if spec["exit"] != 0:
        continue
    try:
        got = payload(spec["lines"])
        if "message" in spec or got != spec["expected"]:
            failures.append("fake " + name)
    except ValueError as e:
        if "message" not in spec or str(e) not in spec["message"]:
            failures.append("fake " + name)
for c in corpus["items"]:
    node = c["node"]
    got = node["items"] if isinstance(node, dict) and isinstance(node.get("items"), list) else []
    if got != c["expected"]:
        failures.append("items")

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
