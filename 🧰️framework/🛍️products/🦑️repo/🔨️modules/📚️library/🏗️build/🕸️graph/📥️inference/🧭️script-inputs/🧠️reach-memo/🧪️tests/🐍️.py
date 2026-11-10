#!/usr/bin/env python3
"""🐍️ Independent oracle of the reach sets: reads {"cases":[{"sets":[[numbers]...]}]} and prints, per case, the sorted union, the sorted pairwise intersections count and the distinct set count, computed with Python sets.
usage: 🐍️.py < cases"""
import json, sys

out = []
for case in json.load(sys.stdin)["cases"]:
    sets = [sorted(set(values)) for values in case["sets"]]
    union = sorted(set().union(*map(set, sets))) if sets else []
    out.append({"sets": sets, "union": union, "distinct": len({tuple(values) for values in sets})})
print(json.dumps({"cases": out}))
