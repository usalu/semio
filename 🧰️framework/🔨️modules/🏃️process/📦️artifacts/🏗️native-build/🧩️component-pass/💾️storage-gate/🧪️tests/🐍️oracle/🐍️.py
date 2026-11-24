"""💾 Independent model of the component storage gate.

Reads a JSON list of cases on stdin (`kind` selects the law) and answers each with the verdict of the model: a running pass is
charged its prediction less what it already wrote (never negative), a pass is admitted when the free bytes less every charge hold
its prediction and the reserve, the image grows only while no pass runs, only when the host stays at or above its reserve after
holding the whole larger envelope, a batch's prediction takes only the units no earlier batch of its lane accounted, bytes already
on disk are credited against the batches of a lane in planned order, and a ledger of at least two measurements replaces the
computed footprint by its type-7 quantile. It shares no code with the TypeScript implementation.
"""
import json
import math
import sys


def gate(c):
    charged = sum(max(0, r["predicted"] - r["written"]) for r in c["running"])
    free = c["headroom"] - charged
    needed = c["predicted"] + c["reserve"]
    return {"admitted": free >= needed, "neededBytes": needed, "chargedBytes": charged, "freeBytes": free, "shortBytes": max(0, needed - free)}


def grow(c):
    if c["running"] > 0:
        return {"kind": "busy"}
    if c["hostFree"] is None:
        return {"kind": "host-unknown"}
    target = math.ceil((c["image"] + c["need"]) / c["align"]) * c["align"]
    added = target - c["image"]
    after = c["hostFree"] - (added + c["initial"])
    if after < c["hostReserve"]:
        return {"kind": "host-reserve", "addedBytes": added, "hostFreeAfterBytes": after}
    return {"kind": "grow", "targetBytes": target, "addedBytes": added, "hostFreeAfterBytes": after}


def credit(c):
    left = {}
    out = []
    for row in c["rows"]:
        lane = row["lane"]
        if lane not in left:
            left[lane] = max(0, c["present"].get(lane, 0))
        taken = min(row["bytes"], left[lane])
        left[lane] -= taken
        out.append({"bytes": row["bytes"] - taken, "credit": taken})
    return out


def quantile(values, q):
    s = sorted(values)
    h = (len(s) - 1) * q
    lo = math.floor(h)
    hi = min(lo + 1, len(s) - 1)
    return s[lo] + (h - lo) * (s[hi] - s[lo])


def learned(c):
    rows = c["rows"]
    if rows is not None and len(rows) >= 2:
        return {"bytes": math.ceil(quantile(rows, 0.9)), "measured": True}
    return {"bytes": c["bytes"], "measured": c["measured"]}


def additions(c):
    present = set(c["present"])
    added = [i for i in c["identities"] if i not in present]
    return {"added": sorted(added), "present": sorted(present | set(c["identities"]))}


LAWS = {"gate": gate, "grow": grow, "credit": credit, "learned": learned, "additions": additions}
print(json.dumps([LAWS[c["kind"]](c) for c in json.load(sys.stdin)]))
