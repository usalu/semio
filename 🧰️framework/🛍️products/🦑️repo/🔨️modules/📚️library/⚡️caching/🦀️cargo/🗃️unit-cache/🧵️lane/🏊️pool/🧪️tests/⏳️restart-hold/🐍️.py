"""Independent oracle of the restart hold of the lane pool. Usage: python -I oracle.py <vectors.json>"""
import json, sys


def retained(policy, lane_id, last_used_ms, now_ms, hold_until_ms=None):
    product = lane_id.startswith(policy["productPrefix"]) and now_ms - last_used_ms < policy["retainMilliseconds"]
    return product or (hold_until_ms is not None and now_ms < hold_until_ms)


def trim(case):
    lanes = [lane for lane in case["lanes"] if not lane.get("held")]
    bounds = case["bounds"]
    count, total, victims = len(lanes), sum(lane["bytes"] or 0 for lane in lanes), []
    for lane in sorted((lane for lane in lanes if not lane["claimed"]), key=lambda lane: (lane["lastUsedMs"], lane["id"])):
        if count <= bounds["maximumLanes"] and total <= bounds["quotaBytes"]:
            break
        victims.append(lane["id"])
        count -= 1
        total -= lane["bytes"] or 0
    return victims


def acquire(case):
    lanes, bounds = case["lanes"], case["bounds"]
    free = [lane for lane in lanes if not lane["claimed"] and not lane.get("product")]
    warm = sorted((lane for lane in free if lane["crateKey"] == case["request"]["crateKey"]), key=lambda lane: (-lane["lastUsedMs"], lane["id"]))
    if warm:
        return {"kind": "warm", "id": warm[0]["id"]}
    counted = [lane for lane in lanes if not lane.get("held")]
    if len(counted) < bounds["maximumLanes"] and sum(lane["bytes"] or 0 for lane in counted) <= bounds["quotaBytes"]:
        return {"kind": "new"}
    cold = sorted((lane for lane in free if not lane.get("held")), key=lambda lane: (lane["lastUsedMs"], lane["id"]))
    if cold:
        return {"kind": "lru", "id": cold[0]["id"]}
    return {"kind": "new"} if len(counted) < bounds["maximumLiveLanes"] else {"kind": "busy"}


if __name__ == "__main__":
    corpus = json.load(open(sys.argv[1], encoding="utf-8"))
    policy = corpus["policy"]
    print(json.dumps({
        "retained": [retained(policy, row["id"], row["lastUsedMs"], row["nowMs"], row.get("holdUntilMs")) for row in corpus["retained"]],
        "trim": [trim(case) for case in corpus["trim"]],
        "acquire": [acquire(case) for case in corpus["acquire"]],
    }))
