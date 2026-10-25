#!/usr/bin/env python3
"""🐍️ Independent oracle of the graph release laws: reads the same fixture and the same sources and checks every law with plain string search (no shared code with the Bun test).

usage: 🐍️.py FIXTURE.json LIBRARY_ROOT      (exit 0 when every law holds)"""
import json, os, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
library = sys.argv[2]
failures = []


def body(name):
    row = fixture["functions"][name]
    source = open(os.path.join(library, row["file"]), encoding="utf8").read()
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
    before, after = text.find(row["before"]), text.find(row["after"], max(text.find(row["before"]), 0))
    if before < 0 or after < 0 or before >= after:
        failures.append("order: " + row["name"])
for row in fixture["present"]:
    if row["text"] not in body(row["function"]):
        failures.append("present: " + row["name"])
for row in fixture["absent"]:
    if row["text"] in body(row["function"]):
        failures.append("absent: " + row["name"])
for row in fixture["absentAfter"]:
    text = body(row["function"])
    at = text.find(row["marker"])
    if at < 0 or row["text"] in text[at + len(row["marker"]):]:
        failures.append("absentAfter: " + row["name"])
print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
