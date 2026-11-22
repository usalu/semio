import hashlib, json, math, sys

def clamp(v, lo, hi):
    return max(lo, min(hi, v))

def sizing(case, p):
    h = case["host"]
    total = h["totalMegabytes"]
    floor = clamp(total * p["floorShareBasisPoints"] // 10000, p["floorMinimumMegabytes"], p["floorMaximumMegabytes"])
    cm = case["childMegabytes"]
    fg = h["foreground"]
    ask = fg.get("ask")
    waiting = fg.get("waitingMegabytes")
    hold = fg["unrealizedMegabytes"] if fg["waiters"] > 0 and waiting is None else min(fg["unrealizedMegabytes"], total * p["ownHoldShareBasisPoints"] // 10000)
    room_now = h["availableMegabytes"] - floor - hold
    room_drained = room_now + case["children"] * cm
    def blocked(m):
        return room_now < m <= room_drained
    def counted(m):
        return m if m <= room_drained else 0
    held = hold + counted(waiting or 0) + counted(ask["megabytes"] if ask is not None else 0)
    spare = h["availableMegabytes"] - floor - held
    swap = h["swapOutMegabytesPerSecond"] if h["swapOutMegabytesPerSecond"] is not None else 0
    def limit(kind, children):
        return {"limit": kind, "needMegabytes": cm, "freeMegabytes": max(0, spare - max(0, children - case["children"]) * cm), "heldMegabytes": held, "floorMegabytes": floor}
    lo, hi = h["totalMegabytes"] * p["swapActiveBasisPointsPerSecond"] / 10000, h["totalMegabytes"] * p["swapOutBasisPointsPerSecond"] / 10000
    share = 1 if h["swapOutMegabytesPerSecond"] is None or swap < lo else max(0, (hi - swap) / (hi - lo))
    cap = math.floor(case["children"] * share)
    if h["pressure"] == "critical" or (share < 1 and cap < 1):
        return {"children": 0, "reason": "pressure", "limit": limit("pressure", 0)}
    if ask is not None and ask["alone"]:
        return {"children": 0, "reason": "waiting", "limit": limit("waiting", 0)}
    waited = fg.get("oldestWaitMilliseconds", 0)
    idle = h.get("idleCores")
    cores_held = idle is None or (idle < 1 and idle + case["children"] >= 1)
    memory_held = waiting is None or blocked(waiting)
    if fg["waiters"] > 0 and waited >= p["yieldAfterMilliseconds"] and (memory_held or cores_held):
        return {"children": 0, "reason": "waiting", "limit": limit("waiting", 0)}
    reserved = math.ceil(h["cores"] * p["foregroundCoreShareBasisPoints"] / 10000) if fg["waiters"] > 0 else 0
    minimum = max(1, math.ceil(h["cores"] * p["minimumShareBasisPoints"] / 10000))
    booked = max(0, h["cores"] - fg["cpu"] - reserved)
    by_cores = (case["children"] + math.floor(max(0, idle - reserved)) if fg["waiters"] > 0 else h["cores"]) if idle is not None else (booked if fg["waiters"] > 0 else max(booked, minimum))
    external = 0 if h["load"] is None else max(0, h["load"] - case["children"])
    by_load = max(minimum, math.floor(h["cores"] * p["loadCeilingBasisPointsPerCore"] / 10000 - external) if idle is None else case["children"] + math.floor(idle))
    by_memory = case["children"] + math.floor(spare / max(1, cm))
    bounds = [by_cores, by_load, by_memory, p["maximumChildren"]] + ([cap] if share < 1 else [])
    children = min(bounds)
    if h["pressure"] == "warn":
        children = min(children, max(1, case["children"]))
    wanted = min(h["cores"], p["maximumChildren"])
    if by_memory <= min(by_cores, by_load):
        kind = "memory"
    else:
        kind = "load" if by_load < by_cores else "cores"
    own = fg["cpu"] > 0 or fg["waiters"] > 0 or ask is not None
    if children < 1:
        if own and by_memory < 1:
            return {"children": 0, "reason": "nothing-fits", "limit": limit(kind, 0)}
        return {"children": 1, "reason": "nothing-fits" if by_memory < 1 else "yielding", "limit": limit(kind, 1)}
    if children < wanted:
        return {"children": children, "reason": "nothing-fits" if kind == "memory" else "yielding", "limit": limit(kind, children)}
    return {"children": children, "reason": None, "limit": limit("none", children)}

def target(case, p):
    row = sizing(case, p)
    return {"children": row["children"], "reason": row["reason"]}

def limits(total, p):
    return {"recycleMegabytes": clamp(total * p["childRecycleShareBasisPoints"] // 10000, p["childRecycleMinimumMegabytes"], p["childRecycleMaximumMegabytes"]),
            "hardMegabytes": max(p["childHardMinimumMegabytes"], total * p["childHardShareBasisPoints"] // 10000)}

def child_megabytes(samples, p, live=()):
    recent = sorted([s for s in samples if s > 0][-p["peakSamples"]:] + [s for s in live if s > 0])
    if not recent:
        return p["childMegabytesDefault"]
    return math.ceil(recent[max(0, math.ceil(len(recent) * p["peakPercentile"] / 100) - 1)] * (100 + p["peakMarginPercent"]) / 100)

def recycle(case, p):
    return case["rss"] > case["recycleMegabytes"] and case["rss"] >= case["readyRss"] + p["recycleGrowthMegabytes"]

def seed(text):
    return int(hashlib.sha256(text.encode("utf8")).hexdigest()[:8], 16)

def mulberry(state):
    state = (state + 0x6D2B79F5) & 0xFFFFFFFF
    t = state
    t = ((t ^ (t >> 15)) * (t | 1)) & 0xFFFFFFFF
    t ^= (t + (((t ^ (t >> 7)) * (t | 61)) & 0xFFFFFFFF)) & 0xFFFFFFFF
    return ((t ^ (t >> 14)) & 0xFFFFFFFF) / 4294967296, state

def walk(case):
    state = seed(case["puller"])
    out = []
    for _ in range(case["draws"]):
        v, state = mulberry(state)
        out.append(int(v * case["n"]))
    return out

doc = json.load(open(sys.argv[1], encoding="utf8"))
p = doc["policy"]
print(json.dumps({
    "target": [target(c, p) for c in doc["target"]],
    "limit": [sizing(c, p)["limit"] for c in doc["target"]],
    "limits": [limits(c["total"], p) for c in doc["limits"]],
    "childMegabytes": [child_megabytes(c["samples"], p, c.get("live", [])) for c in doc["childMegabytes"]],
    "recycle": [recycle(c, p) for c in doc["recycle"]],
    "walk": [walk(c) for c in doc["walk"]],
}, ensure_ascii=False))
