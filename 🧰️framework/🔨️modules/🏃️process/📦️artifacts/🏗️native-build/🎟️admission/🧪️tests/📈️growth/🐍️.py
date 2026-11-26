"""📈️ Independent Python oracle of the grant growth law (standard library only): holdable = floor(max(0, total - floor - interactive) * budget / 10000); a unit that needs at most its grant fits; else, when it needs at most what this host holds, it grows when the room above floor and interactive reserve covers the difference and waits otherwise; else it is beyond-host when the fleet is unknown or another machine holds it, and refused when none does. usage: 🐍️.py <vectors.json> prints a JSON list of verdicts, one per case."""
import json, sys

def holdable(h):
    return max(0, h["totalMegabytes"] - h["floorMegabytes"] - h["interactiveReserveMegabytes"]) * h["budgetBasisPoints"] // 10000

def decide(case):
    need, grant, local, fleet = case["need"], case["grant"], case["local"], case["fleet"]
    if need <= grant:
        return {"kind": "fits"}
    mine = holdable(local)
    if need <= mine:
        room = max(0, local["availableMegabytes"] - local["floorMegabytes"] - local["interactiveReserveMegabytes"])
        return {"kind": "grow" if need - grant <= room else "wait", "extraMegabytes": need - grant, "roomMegabytes": room, "holdableMegabytes": mine}
    best = (mine, local["origin"])
    for host in fleet or []:
        if host["origin"] != local["origin"] and holdable(host) > best[0]:
            best = (holdable(host), host["origin"])
    if fleet is None or best[0] >= need:
        return {"kind": "beyond-host", "holdableMegabytes": mine, "largestMegabytes": best[0], "largestOrigin": best[1]}
    return {"kind": "refused", "largestMegabytes": best[0], "largestOrigin": best[1]}

print(json.dumps([decide(case) for case in json.load(open(sys.argv[1]))["cases"]]))
