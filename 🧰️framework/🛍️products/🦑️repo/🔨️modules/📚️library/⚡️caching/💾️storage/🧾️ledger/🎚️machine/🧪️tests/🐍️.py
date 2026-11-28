"""🐍️ Independent oracle of the machine disk budget plan: exact rationals, a sorted-by-ratio water-fill instead of the iterative rounds of the TypeScript, no shared code. Reads {"stores","hostFreeFloorBytes","alignBytes","cases"} from the file in argv[1] and prints one plan per case as JSON."""
import json
import sys
from fractions import Fraction


def split(total, rows, spare):
    quota = {r["id"]: r["floor"] for r in rows}
    remaining = total - sum(quota.values())
    if remaining < 0:
        raise ValueError("floors")
    caps = {r["id"]: max(0, r["demand"] - r["floor"]) for r in rows}
    weight = {r["id"]: r["weight"] for r in rows}
    pending = sorted((r["id"] for r in rows if caps[r["id"]] > 0), key=lambda i: (Fraction(caps[i], weight[i]), i))
    total_weight = sum(weight[i] for i in pending)
    while pending and remaining > 0:
        first = pending[0]
        if caps[first] * total_weight <= remaining * weight[first]:
            quota[first] += caps[first]
            remaining -= caps[first]
            total_weight -= weight[first]
            pending.pop(0)
        else:
            break
    if pending and remaining > 0:
        parts = []
        for i in pending:
            exact = Fraction(remaining * weight[i], total_weight)
            parts.append([i, exact.numerator // exact.denominator, exact - exact.numerator // exact.denominator])
        left = remaining - sum(p[1] for p in parts)
        for p in sorted(parts, key=lambda p: (-p[2], p[0])):
            if left <= 0:
                break
            p[1] += 1
            left -= 1
        for i, whole, _ in parts:
            quota[i] += whole
        remaining = 0
    if spare and remaining > 0:
        everyone = [r["id"] for r in rows]
        weights = sum(weight[i] for i in everyone)
        parts = []
        for i in everyone:
            exact = Fraction(remaining * weight[i], weights)
            parts.append([i, exact.numerator // exact.denominator, exact - exact.numerator // exact.denominator])
        left = remaining - sum(p[1] for p in parts)
        for p in sorted(parts, key=lambda p: (-p[2], p[0])):
            if left <= 0:
                break
            p[1] += 1
            left -= 1
        for i, whole, _ in parts:
            quota[i] += whole
    return quota


def ceil_div(a, b):
    return -((-a) // b)


def plan(spec, case):
    stores = spec["stores"]
    uses = {u["id"]: u for u in case["uses"]}
    align = spec["alignBytes"]
    rows = []
    for s in stores:
        u = uses.get(s["id"], {"usedBytes": 0, "envelopeBytes": None, "evictableBytes": None})
        h = s["headroomBasisPoints"]
        used = u["usedBytes"]
        demand = used if h == 0 else ceil_div(used * 10000, 10000 - h)
        rows.append((s, u, demand))
    floors = sum(s["floorBytes"] for s in stores)
    demand_total = sum(r[2] for r in rows)
    need_total = sum(max(r[2], r[0]["floorBytes"]) for r in rows)
    free = case["hostFreeBytes"]
    floor_bytes = spec["hostFreeFloorBytes"]
    total_host = case.get("hostTotalBytes")
    if spec.get("hostFreeFloorPercent") is not None and total_host:
        floor_bytes = min(floor_bytes, max(spec["alignBytes"], total_host * spec["hostFreeFloorPercent"] // 100 // spec["alignBytes"] * spec["alignBytes"]))
    host_reserve = max(floor_bytes, spec["hostReserveBytes"]) if spec.get("hostReserveBytes") is not None else floor_bytes
    if spec.get("hostReserveBytes") is not None and spec.get("hostReservePercent") is not None and total_host:
        host_reserve = max(floor_bytes, min(spec["hostReserveBytes"], max(align, total_host * spec["hostReservePercent"] // 100 // align * align)))
    pressure = host_reserve - free if free is not None and free < host_reserve else 0
    used_total = sum(r[1]["usedBytes"] for r in rows)
    slack = None if free is None else free - host_reserve
    if slack is not None and used_total != 0:
        slack = (slack * demand_total) // used_total if slack < 0 else slack * demand_total // used_total
    host_budget = None if free is None else max(0, need_total + slack)
    ceiling = case["totalBytes"]
    if spec.get("hostSharePercent") is not None and total_host:
        ceiling = min(ceiling, max(spec["alignBytes"], total_host * spec["hostSharePercent"] // 100 // spec["alignBytes"] * spec["alignBytes"]))
    effective = max(floors, min(ceiling, host_budget if host_budget is not None else ceiling))
    quotas = split(effective, [{"id": s["id"], "weight": s["weightBasisPoints"], "floor": s["floorBytes"], "demand": d} for s, _, d in rows], True)
    out = []
    for s, u, demand in rows:
        h = s["headroomBasisPoints"]
        quota = quotas[s["id"]]
        env = u["envelopeBytes"]
        if env is not None:
            reserve = spec["admission"]
            room = env
            if s.get("admitsCommands"):
                room -= max(reserve["floorMinimumBytes"], env * reserve["floorBasisPoints"] // 10000) + env * reserve["maximumShareBasisPoints"] // 10000
            quota = max(s["floorBytes"], min(quota, room))
        limit = quota * (10000 - h) // 10000
        used = u["usedBytes"]
        excess = max(0, used - limit)
        excess = excess if excess > spec["toleranceBytes"] else 0
        evictable = u["evictableBytes"] if u["evictableBytes"] is not None else used
        evict = min(excess, evictable)
        blocked = excess - evict
        env = u["envelopeBytes"]
        resize = None
        if env is not None:
            fit = max(ceil_div(demand, align) * align, quota // align * align) if quota >= demand else quota // align * align
            if env - fit >= align:
                resize = fit
        out.append({"id": s["id"], "usedBytes": used, "demandBytes": demand, "quotaBytes": quota, "contentLimitBytes": limit, "excessBytes": excess, "evictBytes": evict, "blockedBytes": blocked, "envelopeBytes": env, "resizeToBytes": resize, "state": "blocked" if blocked > 0 else "evict" if evict > 0 else "within"})
    footprint = sum((u["envelopeBytes"] if u["envelopeBytes"] is not None else u["usedBytes"]) for _, u, _ in rows)
    return {"totalBytes": case["totalBytes"], "effectiveTotalBytes": effective, "reserveBytes": host_reserve, "hostBudgetBytes": host_budget, "pressureBytes": pressure, "usedBytes": sum(u["usedBytes"] for _, u, _ in rows), "footprintBytes": footprint, "overBytes": max(0, footprint - ceiling), "evictBytes": sum(o["evictBytes"] for o in out), "blockedBytes": sum(o["blockedBytes"] for o in out), "converged": all(o["blockedBytes"] == 0 for o in out), "stores": out}


spec = json.load(open(sys.argv[1], encoding="utf8"))
print(json.dumps([plan(spec, case) for case in spec["cases"]]))
