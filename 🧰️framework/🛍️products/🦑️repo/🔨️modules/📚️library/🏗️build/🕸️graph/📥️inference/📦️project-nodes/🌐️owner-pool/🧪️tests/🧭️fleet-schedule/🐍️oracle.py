import json, sys


def clamp(v, lo, hi):
    return max(lo, min(hi, v))


def slots(machine, policy):
    total, available, cores, need = machine["totalMegabytes"], machine["availableMegabytes"], machine["cores"], machine["childMegabytes"]
    floor = clamp(total * policy["floorShareBasisPoints"] // 10000, policy["floorMinimumMegabytes"], policy["floorMaximumMegabytes"])
    spare = available - floor
    by_memory = spare // need
    minimum = max(1, -(-cores * policy["minimumShareBasisPoints"] // 10000))
    busy_cpu = 2 if machine["busy"] else 0
    booked = max(0, cores - busy_cpu)
    by_cores = max(booked, minimum)
    by_load = max(minimum, cores * policy["loadCeilingBasisPointsPerCore"] // 10000)
    children = min(by_cores, by_load, by_memory, policy["maximumChildren"])
    if children < 1:
        return 0 if machine["busy"] and by_memory < 1 else 1
    return children


def scenario(case, policy):
    machines = case["machines"]
    each = [slots(machine, policy) for machine in machines]
    tier0, tier1 = case["owners"]["tier0"], case["owners"]["tier1"]
    names = ["t0-%03d" % i for i in range(tier0)] + ["t1-%03d" % i for i in range(tier1)]
    return {"slots": each, "totalSlots": sum(each), "owners": len(names), "tier0": tier0, "names": sorted(names), "served": [m["name"] for m, s in zip(machines, each) if s > 0]}


doc = json.load(open(sys.argv[1], encoding="utf8"))
policy = json.load(open(sys.argv[2], encoding="utf8"))["definitions"]["Policy"]["const"]
print(json.dumps([scenario(case, policy) for case in doc["scenarios"]], ensure_ascii=False))
