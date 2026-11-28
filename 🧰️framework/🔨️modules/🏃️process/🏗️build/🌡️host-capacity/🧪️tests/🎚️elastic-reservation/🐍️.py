"""🐍️ Independent Python oracle (standard library only) of elastic reservations in host admission, on a snapshot of the live gate: a grant that retains its reservation is charged its whole reservation while it warms up (60 s and three heartbeats), then converges within 120 s to its observed peak plus a margin (the largest of a floor, three standard deviations of its own measurements and 15 % of the slack its class estimate still holds above the peak), never under what it holds, never over its reservation; the charge of a settled retained grant stays the smaller of this and the older settle rule. The headroom is what the host has above its floor beside those charges and within the share it leaves for grants; a single request is admitted when its size fits it. `python3 🐍️.py` prints, per case, the charge of every grant, the unrealized memory, the headroom and the verdict of the probe request. It shares no code with the TypeScript implementation."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = json.load(open(os.path.join(HERE, "🔣️.json"), encoding="utf-8"))
POLICY = json.load(open(os.path.join(HERE, *[".."] * 6, "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🏗️build", "🔣️policy.json"), encoding="utf-8"))["hostCapacity"]
MEMORY = POLICY["memory"]
ELASTIC = MEMORY["elastic"]


def elastic(row, age_ms):
    if "peakMegabytes" not in row or row.get("observations", 0) < ELASTIC["minimumObservations"] or age_ms < ELASTIC["warmupMilliseconds"]:
        return None
    reserved = row["reservedMegabytes"]
    peak = max(row["peakMegabytes"], row["observedMegabytes"])
    slack = max(0, reserved - peak)
    margin = max(ELASTIC["marginMinimumMegabytes"], row["deviationMegabytes"] * ELASTIC["deviationFactor"], math.ceil(slack * ELASTIC["classKeepBasisPoints"] / 10000))
    target = min(reserved, peak + margin)
    elapsed = min(age_ms - ELASTIC["warmupMilliseconds"], ELASTIC["convergeMilliseconds"])
    return reserved - (reserved - target) * elapsed // ELASTIC["convergeMilliseconds"]


def charge_of(row, age_ms, with_elastic):
    reserved, held = row["reservedMegabytes"], row["observedMegabytes"]
    settled = reserved
    if row["operation"] != "graph-plan" and age_ms >= MEMORY["settledAfterMilliseconds"] and held > 0:
        settled = min(reserved, max(math.ceil(held * MEMORY["retainedKeepBasisPoints"] / 10000), held + MEMORY["retainedKeepMegabytes"]))
    value = elastic(row, age_ms) if with_elastic else None
    return min(settled, reserved if value is None else value)


def run():
    total = FIX["totalMegabytes"]
    floor = min(MEMORY["floorMaximumMegabytes"], max(MEMORY["floorMinimumMegabytes"], total * MEMORY["floorBasisPoints"] // 10000))
    reserve = min(MEMORY["interactiveReserveMaximumMegabytes"], max(MEMORY["interactiveReserveMinimumMegabytes"], total * MEMORY["interactiveReserveBasisPoints"] // 10000))
    out = []
    for case in FIX["cases"]:
        grants = []
        for row in FIX["grants"]:
            row = dict(row, **case.get("overrides", {}).get(row["id"], {}))
            if "observations" in case and not row.get("quiet"):
                row["observations"] = case["observations"]
            grants.append((row, case.get("ages", {}).get(row["id"], row["ageSeconds"]) * 1000))
        with_elastic = case.get("elastic", True)
        charges, unrealized, committed, held_sum = {}, 0, 0, 0
        for row, age in grants:
            held = row["observedMegabytes"]
            held_sum += held
            if row.get("retain"):
                charges[row["id"]] = charge_of(row, age, with_elastic)
                unrealized += max(0, charges[row["id"]] - held)
                committed += max(charges[row["id"]], held)
            else:
                charges[row["id"]] = 0
                committed += held
        unmanaged = max(reserve, total - case["availableMegabytes"] - held_sum)
        share = total - floor - unmanaged
        headroom = min(case["availableMegabytes"] - floor - unrealized, share * MEMORY["budgetBasisPoints"] // 10000 - committed)
        out.append({"name": case["name"], "charges": charges, "unrealizedMegabytes": unrealized, "headroomMegabytes": headroom, "probe": "admit" if case["probe"] <= headroom else "wait"})
    return out


print(json.dumps(run()))
