import hashlib
import json
import sys


def slot_id(domain, parts):
    return hashlib.sha256(json.dumps(["semio-slot-v1", domain] + parts, separators=(",", ":"), ensure_ascii=False).encode("utf-8")).hexdigest()


def classify(rows, now, grace):
    out = {}
    groups = {}
    for row in rows:
        if row["slot"] is None:
            out[row["key"]] = ["unslotted", None, None]
        else:
            groups.setdefault(row["slot"], []).append(row)
    for group in groups.values():
        group.sort(key=lambda r: (-r["storedAt"], r["key"]))
        for index, row in enumerate(group):
            if index == 0:
                out[row["key"]] = ["current", None, None]
                continue
            superseded = group[index - 1]["storedAt"]
            reclaim = max(superseded, row["storedAt"]) + grace
            out[row["key"]] = ["expired" if now >= reclaim else "retained", superseded, reclaim]
    return out


def plan(rows, bounds, dangling, retention):
    """Independent model of the collection plan: dangling, stale or unreached beyond grace (verdicts given), age for entries that are not live, budget by value tier."""
    grace = retention["grace"]
    metas = retention["metas"]
    verdicts = retention.get("verdicts") or {}
    claimed = set(retention["claimed"])
    tiers = {"stale": 0, "unreached": 0, "undeclared": 1, "live": 2, "grace": 2}
    drops = []
    kept = []
    guarded = set()
    total = 0
    for r in rows:
        key = r["key"]
        state = verdicts.get(key, "undeclared")
        stored = (metas.get(key) or {}).get("storedAt")
        is_claimed = key in claimed
        young = stored is not None and bounds["now"] - stored < grace
        if key in dangling:
            drops.append({"key": key, "size": r["size"], "reason": "dangling"})
        elif state in ("stale", "unreached") and not is_claimed and not young:
            d = {"key": key, "size": r["size"], "reason": "stale"}
            if stored is not None:
                d["storedAt"] = stored
            drops.append(d)
        elif tiers[state] < 2 and not is_claimed and bounds["maximumAge"] > 0 and bounds["now"] - r["accessed"] > bounds["maximumAge"]:
            drops.append({"key": key, "size": r["size"], "reason": "age"})
        else:
            kept.append((r, tiers[state]))
            total += r["size"]
            if is_claimed or young:
                guarded.add(key)
    need = bounds.get("need", 0)
    reserved = bounds.get("reserved", 0)
    if total + reserved > bounds["maximumBytes"] - need:
        target = bounds["maximumBytes"] * 9 // 10 - need
        kept.sort(key=lambda item: (item[1], item[0]["accessed"], item[0]["key"]))
        for r, _ in kept:
            if total + reserved <= target:
                break
            if r["key"] in guarded:
                continue
            d = {"key": r["key"], "size": r["size"], "reason": "budget"}
            stored = (metas.get(r["key"]) or {}).get("storedAt")
            if stored is not None:
                d["storedAt"] = stored
            drops.append(d)
            total -= r["size"]
    return drops


def main():
    requests = json.load(sys.stdin)
    result = []
    for request in requests:
        if request["op"] == "plan":
            result.append(plan(request["rows"], request["bounds"], set(request["dangling"]), request["retention"]))
        elif request["op"] == "id":
            result.append(slot_id(request["domain"], request["parts"]))
        else:
            result.append(classify(request["rows"], request["now"], request["grace"]))
    json.dump(result, sys.stdout)


main()
