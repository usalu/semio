#!/usr/bin/env python3
"""🐍️ Independent oracle of the sealed-reuse text laws: reads the same fixture and the same graph source and checks every law with plain string search (no shared code with the Bun test).

usage: 🐍️.py FIXTURE.json GRAPH_SOURCE.ts      (exit 0 when every law holds)"""
import json, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
source = open(sys.argv[2], encoding="utf8").read()
failures = []


def body(name):
    row = fixture["functions"][name]
    start = source.find(row["start"])
    if start < 0 or source.find(row["start"], start + 1) >= 0:
        failures.append("%s: start marker missing or not unique" % name)
        return ""
    end = source.find(row["end"], start + len(row["start"]))
    if end < 0:
        failures.append("%s: end marker missing" % name)
        return ""
    return source[start:end]


for row in fixture["order"]:
    text = body(row["function"])
    before, after = text.find(row["before"]), text.find(row["after"])
    if before < 0 or after < 0 or before >= after:
        failures.append("order: " + row["name"])
for row in fixture["present"]:
    if row["text"] not in body(row["function"]):
        failures.append("present: " + row["name"])
for row in fixture["absent"]:
    if row["text"] in body(row["function"]):
        failures.append("absent: " + row["name"])
print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
