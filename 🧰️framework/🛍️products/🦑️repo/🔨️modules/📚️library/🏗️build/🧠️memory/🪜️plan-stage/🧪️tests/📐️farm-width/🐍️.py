#!/usr/bin/env python3
"""🐍️ Independent model of the width of the project farm (standard library only): the hard share of the tree minus the parent footprint and what other farms hold must hold one child at its cap, every further child counts at its typical footprint."""
import json, math, sys

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
policy = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]["Policy"]["const"]
footprint = json.load(open(sys.argv[3], encoding="utf-8"))["definitions"]["Policy"]["const"]
child = policy["farmChildFootprintMegabytes"]["project-farm"]
rows = []
for case in fixture["cases"]:
    hard = case["host"] * policy["treeShare"] * footprint["hardPercent"] / 100
    free = hard - policy["parentFootprintMegabytes"] - case["others"]
    rows.append(0 if free < child["capMegabytes"] else 1 + math.floor((free - child["capMegabytes"]) / child["typicalMegabytes"]))
print(json.dumps(rows))
