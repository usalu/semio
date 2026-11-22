"""Independent oracle of the retained product lane rules of the lane pool. Usage: python -I oracle.py <vectors.json>"""
import json, sys


def retained(policy, lane_id, last_used_ms, now_ms):
    return lane_id.startswith(policy["productPrefix"]) and now_ms - last_used_ms < policy["retainMilliseconds"]


def trim(policy, case):
    now = case["nowMs"]
    lanes = [lane for lane in case["lanes"] if not retained(policy, lane["id"], lane["lastUsedMs"], now)]
    bounds = case["bounds"]
    count, total, victims = len(lanes), sum(lane["bytes"] or 0 for lane in lanes), []
    for lane in sorted((lane for lane in lanes if not lane["claimed"]), key=lambda lane: (lane["lastUsedMs"], lane["id"])):
        if count <= bounds["maximumLanes"] and total <= bounds["quotaBytes"]:
            break
        victims.append(lane["id"])
        count -= 1
        total -= lane["bytes"] or 0
    return victims


if __name__ == "__main__":
    corpus = json.load(open(sys.argv[1], encoding="utf-8"))
    policy = corpus["policy"]
    print(json.dumps({
        "retained": [retained(policy, row["id"], row["lastUsedMs"], row["nowMs"]) for row in corpus["retained"]],
        "trim": [trim(policy, case) for case in corpus["trim"]],
    }))
