"""🚦️ Independent Python oracle of the lane admission rules (standard library only, no code shared with the TypeScript reference).
usage: 🐍️.py <corpus.json>   prints one verdict list per section as JSON

Rules. Acquire: a free lane of the same crate set is taken warm (newest first); else, when the request names an owner and a crate set, a lane held by a live process with the same owner, crate set and flavour is waited for (busy, naming it); else the pool grows. Copies: lanes of one owner, crate set and flavour (owner and crate set known, not held, not a product lane) keep one lane: a held one, else a requested one, else the newest; every other idle, unrequested lane is a copy, listed oldest first. Trim: copies first, then least recently used until the bounds hold. Admission: least recently used first until the measured bytes cover the host shortage and the capacities cover the commitment shortage; a shortage that all candidates cannot cover is dropped."""
import json, sys


def newest(l):
    return (-l["lastUsedMs"], l["id"])


def oldest(l):
    return (l["lastUsedMs"], l["id"])


def acquire(case):
    lanes, req = case["lanes"], case["request"]
    free = [l for l in lanes if not l["claimed"] and not l.get("product")]
    warm = sorted([l for l in free if l["crateKey"] == req["crateKey"]], key=newest)
    if warm:
        return {"kind": "warm", "id": warm[0]["id"]}
    if req.get("owner") is not None and req["crateKey"] != "":
        for l in lanes:
            if l["claimed"] and not l.get("product") and l.get("owner") == req["owner"] and l["crateKey"] == req["crateKey"] and l.get("flavour") == req.get("flavour"):
                return {"kind": "busy", "id": l["id"]}
    return {"kind": "new"}


def duplicates(lanes, requested):
    groups = {}
    for l in lanes:
        if l.get("owner") is not None and l["crateKey"] != "" and not l.get("product") and not l.get("held"):
            groups.setdefault((l["owner"], l["crateKey"], l.get("flavour", "")), []).append(l)
    victims = []
    for group in groups.values():
        if len(group) < 2:
            continue
        pool = [l for l in group if l["claimed"]] or [l for l in group if l["id"] in requested] or group
        kept = sorted(pool, key=newest)[0]
        victims += [l for l in group if l is not kept and not l["claimed"] and l["id"] not in requested]
    return [l["id"] for l in sorted(victims, key=oldest)]


def trim(case):
    lanes, b, requested = case["lanes"], case["bounds"], case["requested"]
    gone = set(duplicates(lanes, requested))
    out = sorted(gone, key=lambda i: oldest(next(l for l in lanes if l["id"] == i)))
    count = len(lanes) - len(gone)
    total = sum(l["bytes"] or 0 for l in lanes if l["id"] not in gone)
    for l in sorted([l for l in lanes if l["id"] not in gone and not l["claimed"] and l["id"] not in requested and (case["pressure"] or not l.get("recent"))], key=oldest):
        if count <= b["maximumLanes"] and total <= b["quotaBytes"]:
            break
        out.append(l["id"])
        count -= 1
        total -= l["bytes"] or 0
    return out


def admission(case):
    order = sorted(case["candidates"], key=oldest)
    host, commit = case["shortage"]["hostBytes"], case["shortage"]["commitmentBytes"]
    if host > sum(l["bytes"] or 0 for l in order):
        host = 0
    if commit > sum(l["capacityBytes"] for l in order):
        commit = 0
    out = []
    for l in order:
        if host <= 0 and commit <= 0:
            break
        out.append(l["id"])
        host -= l["bytes"] or 0
        commit -= l["capacityBytes"]
    return out


if __name__ == "__main__":
    c = json.load(open(sys.argv[1]))
    print(json.dumps({
        "acquire": [acquire(x) for x in c["acquire"]],
        "duplicates": [duplicates(x["lanes"], x["requested"]) for x in c["duplicates"]],
        "trim": [trim(x) for x in c["trim"]],
        "admission": [admission(x) for x in c["admission"]],
    }))
