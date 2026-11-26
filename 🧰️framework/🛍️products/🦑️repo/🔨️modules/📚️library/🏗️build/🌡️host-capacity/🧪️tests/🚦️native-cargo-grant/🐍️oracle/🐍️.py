"""🚦️ Independent Python oracle of one native cargo admission (standard library only): the request a cargo run is sized to, and the verdict, jobs and reason the machine-wide gate gives it on a host with no other grant and no other waiter; the memory kept for the interactive apps comes off the budget beside the floor. Reads {"policy": hostCapacity policy, "cases": [...]} as JSON on stdin and prints one {"verdict","jobs","reason"} per case."""
import json
import sys


def floor_megabytes(policy, total):
    memory = policy["memory"]
    share = total * memory["floorBasisPoints"] // 10000
    return min(total, max(memory["floorMinimumMegabytes"], min(memory["floorMaximumMegabytes"], share)))


def interactive_reserve_megabytes(policy, total):
    memory = policy["memory"]
    share = total * memory["interactiveReserveBasisPoints"] // 10000
    return min(total, max(memory["interactiveReserveMinimumMegabytes"], min(memory["interactiveReserveMaximumMegabytes"], share)))


def request(policy, total, preferred):
    operation = next(row for row in policy["operations"] if row["name"] == "cargo")
    capacity = max(0, total - floor_megabytes(policy, total))
    reach = max(operation.get("minimumMegabytes", 0), capacity * policy["estimates"]["maximumShareBasisPoints"] // 10000)
    fixed = min(operation["fixedMegabytes"], reach)
    kind = policy["kinds"][operation["kind"]]
    per_job = min(kind["megabytes"], max(policy["estimates"]["minimumMegabytes"], reach - fixed))
    return {"fixed": fixed, "perJob": per_job, "cpu": kind["cpu"], "preferred": max(1, min(preferred, 1024))}


def hard_stop(policy, host):
    if host["pressure"] == "critical":
        return "pressure-critical"
    rate = host["swapOutMegabytesPerSecond"]
    if rate is None:
        return None
    rules, total = policy["swap"], host["totalMegabytes"]
    swapping = rate * 10000 >= total * rules["activeBasisPointsPerSecond"]
    short = host["availableMegabytes"] * 10000 < total * rules["shortBasisPoints"]
    used = host["swapUsedMegabytes"]
    crowded = used is not None and used * 10000 > total * rules["ceilingBasisPoints"]
    if crowded and swapping and short:
        return "swap"
    return "swap-activity" if rate * 10000 > total * rules["outBasisPointsPerSecond"] else None


def soft_stop(policy, host):
    if host["pressure"] == "warn":
        return "pressure-warn"
    if host["openFiles"] is not None and host["openFilesMaximum"] is not None and host["openFiles"] * 10000 > host["openFilesMaximum"] * policy["files"]["ceilingBasisPoints"]:
        return "files"
    if host["attachedImages"] is not None and host["attachedImages"] >= policy["attaches"]["maximum"]:
        return "attaches"
    return None


def decide(policy, case):
    host, total = case["host"], case["host"]["totalMegabytes"]
    asked = request(policy, total, case["preferredJobs"])
    floor = floor_megabytes(policy, total)
    capacity = max(0, total - floor)
    share = max(0, capacity - interactive_reserve_megabytes(policy, total))
    room = min(host["availableMegabytes"] - floor, share * policy["memory"]["budgetBasisPoints"] // 10000)
    ceiling = policy["load"]["perCoreCeiling"] * host["cores"]
    load_room = float("inf") if host["load"] is None else ceiling - host["load"]

    def need(jobs):
        return min(asked["fixed"] + jobs * asked["perJob"], capacity)

    stop = hard_stop(policy, host)
    if stop:
        return {"verdict": "wait", "jobs": 0, "reason": stop}
    fits_one = need(1) <= room
    pause = soft_stop(policy, host)
    if pause:
        return {"verdict": "wait", "jobs": 0, "reason": "memory" if not fits_one else pause}
    for jobs in range(asked["preferred"], 0, -1):
        if need(jobs) <= room and jobs * asked["cpu"] <= load_room:
            return {"verdict": "admit", "jobs": jobs, "reason": "fits" if jobs == asked["preferred"] else "reduced"}
    if fits_one and asked["cpu"] <= ceiling:
        return {"verdict": "admit", "jobs": 1, "reason": "fits" if asked["preferred"] == 1 else "reduced"}
    return {"verdict": "wait", "jobs": 0, "reason": "memory" if not fits_one else "load"}


payload = json.load(sys.stdin)
print(json.dumps([decide(payload["policy"], case) for case in payload["cases"]]))
