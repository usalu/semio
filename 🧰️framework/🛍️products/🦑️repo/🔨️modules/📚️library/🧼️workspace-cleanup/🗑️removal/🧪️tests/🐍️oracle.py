#!/usr/bin/env python3
"""🐍️ Independent model of the cleaner outcome: the expected removals and survivors of the corpus workspace must be disjoint, complete and consistent with the node tree (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



paths = {n["path"] for n in corpus["nodes"]}
gone, alive = corpus["applied"]["gone"], corpus["applied"]["survivors"]


def exists(p):
    return any(q == p or q.startswith(p + "/") for q in paths)


for p in gone + alive:
    if not exists(p):
        failures.append("unknown path " + p)
for g in gone:
    for a in alive:
        if a == g or a.startswith(g + "/") or g.startswith(a + "/"):
            failures.append("overlap %s %s" % (g, a))
if sorted(corpus["dryRun"]["survivors"]) != sorted(alive + gone):
    failures.append("dry survivors")
rows = corpus["dryRun"]["removals"]
if [r["path"] for r in rows] != sorted(r["path"] for r in rows):
    failures.append("rows order")
if sorted(r["path"] for r in rows) != sorted(gone):
    failures.append("rows vs gone")
for r in rows:
    if r["kind"] == "ticket-generated" and "ticket" not in r["path"] and "🎫" not in r["path"]:
        failures.append("ticket-generated outside tickets " + r["path"])
    if r["kind"] == "build-artifact" and not exists(r["path"] + "/CACHEDIR.TAG"):
        failures.append("build artifact without cache tag " + r["path"])
    if r["kind"] == "windows-illegal" and not any(re.search(r'[<>:"|?*]', s) or s.endswith(".") for s in r["path"].split("/")):
        failures.append("legal name listed " + r["path"])
open_ticket = [p for p in paths if "/OPEN/" in p]
if not open_ticket or any(a for a in gone if "/OPEN" in a):
    failures.append("open ticket treated as removable")
for c in corpus["removePath"]:
    if c["expected"] is False and c["exists"] is False and c["path"] in paths:
        failures.append("refused but gone " + c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
