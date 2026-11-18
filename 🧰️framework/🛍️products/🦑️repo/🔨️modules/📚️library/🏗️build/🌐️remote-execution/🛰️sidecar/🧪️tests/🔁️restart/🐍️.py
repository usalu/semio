#!/usr/bin/env python3
"""Independent oracle of the agent drain: after the drain starts the agent is looked at at the grace and then every poll; it is retired at the first look at which it serves nothing for other agents, or at which the drain maximum is reached, never earlier. Usage: oracle.py <fixture.json>; prints the abort times as JSON."""
import json,sys
fx=json.load(open(sys.argv[1],encoding="utf8"))
def abort(c):
    t=c["graceMs"]
    while True:
        if not t<c["busyUntilMs"] or t>=c["drainMaxMs"]:return t
        t+=c["pollMs"]
print(json.dumps([abort(c) for c in fx["drain"]]))
