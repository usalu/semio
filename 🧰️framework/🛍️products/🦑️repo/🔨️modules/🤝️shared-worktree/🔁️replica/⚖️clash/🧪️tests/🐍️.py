#!/usr/bin/env python3
"""⚖️ Independent oracle of the structural conflict rule and of the stale-republish rule. It shares no code with `../🟦️.ts`: it replays the vectors of a fixture with Python's own case mapping.

usage: python3 -I oracle.py vectors FIXTURE      replays the clash and stale vectors; exit 1 on the first disagreement
"""
import json
import sys


def key(path):
    return path.upper().lower()


def retired(entries):
    order = sorted(entries, key=lambda e: (e["hlc"], e["path"]), reverse=False)
    newest = sorted(order, key=lambda e: e["hlc"], reverse=True)
    out = {}
    kept = []
    for entry in newest:
        k = key(entry["path"])
        winner = None
        for other in kept:
            o = key(other)
            if o == k or k.startswith(o + "/") or o.startswith(k + "/"):
                winner = other
                break
        if winner is None:
            kept.append(entry["path"])
        else:
            out[entry["path"]] = winner
    return out


def stale(events, hlc):
    order = sorted(events, key=lambda e: e["hlc"])
    hlcs = [e["hlc"] for e in order]
    if hlc not in hlcs or hlcs.index(hlc) == 0:
        return None
    index = hlcs.index(hlc)
    event, before = order[index], order[index - 1]
    if event["sha256"] is None or event["sha256"] == before["sha256"] or event["base"] == before["sha256"]:
        return None
    if any(older["sha256"] == event["sha256"] for older in order[: index - 1]):
        return {"winner": before["sha256"]}
    return None


def main():
    if len(sys.argv) != 3 or sys.argv[1] != "vectors":
        sys.stderr.write(__doc__)
        return 2
    with open(sys.argv[2], encoding="utf-8") as handle:
        fixture = json.load(handle)
    vectors = fixture["vectors"]
    for vector in vectors:
        got = retired(vector["entries"])
        if got != vector["retired"]:
            sys.stderr.write("MISMATCH %s: %s != %s\n" % (vector["name"], got, vector["retired"]))
            return 1
    for vector in fixture["stale"]:
        got = stale(vector["events"], vector["hlc"])
        if got != vector["stale"]:
            sys.stderr.write("MISMATCH %s: %s != %s\n" % (vector["name"], got, vector["stale"]))
            return 1
    print("ok %d %d" % (len(vectors), len(fixture["stale"])))
    return 0


sys.exit(main())
