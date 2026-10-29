"""🐍️ Independent oracle of the image headroom admission: argv[1] is the policy schema, stdin a list of {kind, input}; prints the result of each (kind is floor, quantile, estimate, outstanding or decision)."""
import json
import sys

policy = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Policy"]["const"]
estimate_rules = policy["estimate"]


def clamp(value, low, high):
    return min(high, max(low, value))


def floor_bytes(volume):
    return max(policy["floorMinimumBytes"], volume * policy["floorBasisPoints"] // 10000)


def quantile(values, basis_points):
    ordered = sorted(values)
    rank = max(1, -(-basis_points * len(ordered) // 10000))
    return ordered[min(len(ordered), rank) - 1]


def estimate(state):
    ceiling = state["volumeBytes"] * policy["maximumShareBasisPoints"] // 10000
    for name in reversed(state["classes"]):
        rows = state["history"]["classes"].get(name, [])[-estimate_rules["maximumSamples"]:]
        if len(rows) >= estimate_rules["minimumSamples"]:
            bytes_ = quantile([r["bytes"] for r in rows], estimate_rules["quantileBasisPoints"]) * (10000 + estimate_rules["marginBasisPoints"]) // 10000
            wall = quantile([r["wallMs"] for r in rows], estimate_rules["quantileBasisPoints"])
            return {"reservedBytes": clamp(bytes_, policy["minimumBytes"], ceiling), "rampMilliseconds": clamp(wall, estimate_rules["minimumRampMilliseconds"], estimate_rules["maximumRampMilliseconds"]), "samples": len(rows), "class": name}
    prior = state["prior"]
    return {"reservedBytes": clamp(prior["bytes"], policy["minimumBytes"], ceiling), "rampMilliseconds": clamp(prior["rampMilliseconds"], estimate_rules["minimumRampMilliseconds"], estimate_rules["maximumRampMilliseconds"]), "samples": 0, "class": None}


def outstanding(state):
    row = state["row"]
    if row["state"] != "running" or row["admittedMs"] is None:
        return 0
    ramp = max(1, row["rampMilliseconds"])
    left = max(0, ramp - max(0, state["nowMs"] - row["admittedMs"]))
    return row["reservedBytes"] * (left * 10000 // ramp) // 10000


def decision(state):
    request = state["request"]
    others = [r for r in state["rows"] if r["id"] != request["id"]]
    running = [r for r in others if r["state"] == "running"]
    floor = floor_bytes(state["volumeBytes"])
    held = sum(outstanding({"row": r, "nowMs": state["nowMs"]}) for r in running)
    ahead = [r for r in others if r["state"] == "waiting" and (r["arrivedMs"], r["id"]) < (request["arrivedMs"], request["id"])]
    ahead_bytes = sum(r["reservedBytes"] for r in ahead)
    room = state["freeBytes"] - floor - held - ahead_bytes
    shortfall = max(0, request["reservedBytes"] - room)
    facts = {"floorBytes": floor, "heldBytes": held, "aheadBytes": ahead_bytes, "shortfallBytes": shortfall, "running": len(running)}
    if shortfall == 0:
        return {"verdict": "admit", "reason": "fits", **facts}
    if not running and not ahead and state["reclaimed"]:
        return {"verdict": "admit", "reason": "alone", **facts}
    queued = ahead_bytes > 0 and state["freeBytes"] - floor - held >= request["reservedBytes"]
    return {"verdict": "wait", "reason": "queue" if queued else "space", **facts}


run = {
    "floor": lambda state: floor_bytes(state),
    "quantile": lambda state: quantile(state["values"], state["basisPoints"]),
    "estimate": estimate,
    "outstanding": outstanding,
    "decision": decision,
}
print(json.dumps([run[case["kind"]](case["input"]) for case in json.load(sys.stdin)]))
