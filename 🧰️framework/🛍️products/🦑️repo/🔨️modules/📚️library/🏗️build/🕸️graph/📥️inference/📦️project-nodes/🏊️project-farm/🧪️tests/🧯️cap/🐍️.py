#!/usr/bin/env python3
"""🐍️ Independent model of the farm footprint cap (standard library only): a job whose first run passes the cap runs once more alone; a job whose alone run passes the alone cap fails; everything else is a plain reply. Prints one JSON object with the expected outcome of every job and the number of stopped runs."""
import json, sys

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
cap, alone_cap = fixture["capMegabytes"], fixture["aloneCapMegabytes"]
case = fixture["cases"][int(sys.argv[2])]
outcomes, stopped = {}, 0
for job in case["jobs"]:
    runs = job["megabytes"]
    if runs[0] <= cap:
        outcomes[str(job["n"])] = "reply"
        continue
    stopped += 1
    if runs[min(1, len(runs) - 1)] <= alone_cap:
        outcomes[str(job["n"])] = "alone-reply"
    else:
        stopped += 1
        outcomes[str(job["n"])] = "overrun"
print(json.dumps({"outcomes": outcomes, "stopped": stopped}))
