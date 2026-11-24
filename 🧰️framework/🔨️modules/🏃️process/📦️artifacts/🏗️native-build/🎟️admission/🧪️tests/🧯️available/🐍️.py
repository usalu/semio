"""🧯️ Independent Python oracle of the available-memory admission law (standard library only), written from the prose: a unit asks to start with a predicted peak. (1) Without a known amount of available memory (null) it starts, reason `unknown`. (2) While the host warden is at level `guard` or `danger` it does not start, reason `warden`; no wait bound overrides that. (3) The room is the available bytes minus floor(total bytes * marginBasisPoints / 10000) minus the unrealized remainder of the running units; when the predicted peak is at most the room it starts, reason `fits`. (4) Otherwise it waits, reason `available`, except when the time it waited reached waitMilliseconds (twice that for a unit marked cancelled) and no unit is running: then it starts, reason `waited-out`. usage: 🐍️.py <schema.json> <corpus.json> prints the expected verdict of every case as JSON."""
import json, sys


def decide(policy, case):
    available = case["availableBytes"]
    if available is None:
        return {"admit": True, "reason": "unknown"}
    if case["wardenLevel"] in ("guard", "danger"):
        return {"admit": False, "reason": "warden"}
    margin = case["totalBytes"] * policy["marginBasisPoints"] // 10000
    room = available - margin - case["runningUnrealizedBytes"]
    if case["predictedBytes"] <= room:
        return {"admit": True, "reason": "fits"}
    bound = policy["waitMilliseconds"] * (2 if case["cancelled"] else 1)
    if case["runningUnits"] == 0 and case["waitedMilliseconds"] >= bound:
        return {"admit": True, "reason": "waited-out"}
    return {"admit": False, "reason": "available"}


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        policy = json.load(handle)["definitions"]["Policy"]["const"]["available"]
    with open(sys.argv[2], encoding="utf-8") as handle:
        corpus = json.load(handle)
    print(json.dumps([decide(policy, case["input"]) for case in corpus["cases"]]))


main()
