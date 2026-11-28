"""🧯️ Independent Python oracle for host admission. Warden guard, warden danger and swap-out above the policy rate block a start. Under permitted host conditions, unknown memory allows a start. A unit that fits the parent grant while nothing runs starts whatever the host spare and the warden level are (only swap holds it). A unit that cannot fit its forecast waits until it fits or the idle wait bound expires."""
import json, sys


def decide(policy, case):
    available = case["availableBytes"]
    granted = case.get("granted", False) and case["runningUnits"] == 0
    if not granted and case["wardenLevel"] in ("guard", "danger"):
        return {"admit": False, "reason": "warden"}
    if case.get("swapOutBytesPerSecond", 0) > policy["swapOutBytesPerSecond"]:
        return {"admit": False, "reason": "swap"}
    if granted:
        return {"admit": True, "reason": "granted"}
    if available is None:
        return {"admit": True, "reason": "unknown"}
    margin = case["totalBytes"] * policy["available"]["marginBasisPoints"] // 10000
    room = available - margin - case["runningUnrealizedBytes"]
    if case["predictedBytes"] <= room:
        return {"admit": True, "reason": "fits"}
    bound = policy["available"]["waitMilliseconds"] * (2 if case["cancelled"] else 1)
    if case["wardenLevel"] == "calm" and case["runningUnits"] == 0 and case["waitedMilliseconds"] >= bound:
        return {"admit": True, "reason": "waited-out"}
    return {"admit": False, "reason": "available"}


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        policy = json.load(handle)["definitions"]["Policy"]["const"]
    with open(sys.argv[2], encoding="utf-8") as handle:
        corpus = json.load(handle)
    print(json.dumps([decide(policy, case["input"]) for case in corpus["cases"]]))


main()
