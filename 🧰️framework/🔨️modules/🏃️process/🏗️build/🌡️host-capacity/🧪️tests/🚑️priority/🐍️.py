"""🚑️ Independent Python oracle (standard library only) of interactive priority in host admission. A worker that is also a dev workstation holds the retained reservations of two idle agent jobs; the readiness stage command asks for 2048 MB (minimum 1024) as interactive or batch. Contract: an interactive request is not held behind a batch request at the head of the line, it is lent the idle part of retained batch reservations for as long as it waits (never more than its minimum), a batch request never spends the loan, and the memory handed out never exceeds the room plus the loan."""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = json.load(open(os.path.join(HERE, "🧫️fixtures", "🔣️.json"), encoding="utf-8"))
BASE = json.load(open(os.environ.get("BASE_FIXTURE") or os.path.join(HERE, "..", "🧫️fixtures", "🔣️.json"), encoding="utf-8"))["policy"]
KEEP_MB, KEEP_BP, AFTER = 512, 15000, 600000


def lendable(reserved, observed, age):
    mem = FIX["memory"]
    if age < AFTER or observed * 10000 >= reserved * mem["rampIdleObservedBasisPoints"]:
        return 0
    keep = min(reserved, max(KEEP_MB, -(-observed * KEEP_BP // 10000)))
    return max(0, reserved - keep)


def room(case, loan):
    total = FIX["totalMegabytes"]
    m = BASE["memory"]
    floor = min(total, max(m["floorMinimumMegabytes"], min(m["floorMaximumMegabytes"], total * m["floorBasisPoints"] // 10000)))
    mem = FIX["memory"]
    reserve = min(total, max(mem["interactiveReserveMinimumMegabytes"], min(mem["interactiveReserveMaximumMegabytes"], total * mem["interactiveReserveBasisPoints"] // 10000)))
    share = (total - floor) - reserve
    reserved = sum(row[0] for row in case["lenders"]) - loan
    resident = sum(row[1] for row in case["lenders"])
    available = total - FIX["foreignMegabytes"] - resident
    observed_room = available - floor - max(0, reserved - resident)
    budget_room = share * m["budgetBasisPoints"] // 10000 - reserved
    return min(observed_room, budget_room)


def decide(case):
    stage = FIX["stage"]
    need, minimum = stage["fixedMegabytes"], stage["minimumMegabytes"]
    if case["stage"] == "batch":
        spare = room(case, 0)
        return {"verdict": "wait", "reserved": 0, "spare": spare, "head": FIX["headMegabytes"] > spare - need}
    free = room(case, 0)
    if free >= need:
        return {"verdict": "admit", "reserved": need}
    if free >= minimum:
        return {"verdict": "admit", "reserved": free}
    loan = min(minimum, sum(lendable(row[0], row[1], row[2]) for row in case["lenders"]))
    borrowed = room(case, loan)
    if loan > 0 and borrowed >= minimum:
        return {"verdict": "admit", "reserved": min(need, borrowed), "loan": loan, "small": "wait" if case["small"] else None}
    return {"verdict": "wait", "reserved": 0}


def main():
    out = {"cases": {case["name"]: decide(case) for case in FIX["cases"]}, "environments": [row["class"] for row in FIX["environments"]]}
    print(json.dumps(out))


if __name__ == "__main__":
    main()
