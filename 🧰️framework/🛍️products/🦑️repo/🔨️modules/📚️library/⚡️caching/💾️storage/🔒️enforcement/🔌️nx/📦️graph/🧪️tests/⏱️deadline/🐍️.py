#!/usr/bin/env python3
"""Independent model of Nx namespace release liveness: per scenario it builds the wait-for graph of the nested scope's settlement (I), the retained child image (W) and the owner's release (R) for one design and reports with graphlib whether every wait ends. usage: 🐍️.py FIXTURE DESIGN"""
import graphlib, json, sys

fixture = json.load(open(sys.argv[1]))
design = fixture["designs"][sys.argv[2]]
verdicts = []
for row in fixture["scenarios"]:
    edges = {}
    released_inside = row["refused"] and design["bind"] == "atomic"
    if row["nested"] and design["enrollment"] == "caller-frame" and not released_inside:
        edges.setdefault("I", set()).add("W")
    if not (row["cancelled"] and design["holder"] == "abort-aware") and not released_inside:
        edges.setdefault("W", set()).add("R")
    if row["nested"]:
        edges.setdefault("R", set()).add("I")
    try:
        list(graphlib.TopologicalSorter(edges).static_order())
        verdicts.append(True)
    except graphlib.CycleError:
        verdicts.append(False)
print(json.dumps({"settles": verdicts}))
