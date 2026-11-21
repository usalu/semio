import json
import sys
from collections import defaultdict
from pathlib import Path

case = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
members = [set() for _ in range(case["rowCount"])]
for band in case["bands"]:
    holders = range(band["firstOwner"], band["endOwner"])
    for row in band["rows"]:
        members[row].update(holders)
    if "markerStart" in band:
        for row in range(band["markerStart"], band["markerStart"] + band["runs"]):
            members[row].update(holders)
partition = defaultdict(list)
for row, owners in enumerate(members):
    partition[tuple(sorted(owners))].append(row)
print(json.dumps([{"r": rows, "o": owners} for owners, rows in partition.items()]))
