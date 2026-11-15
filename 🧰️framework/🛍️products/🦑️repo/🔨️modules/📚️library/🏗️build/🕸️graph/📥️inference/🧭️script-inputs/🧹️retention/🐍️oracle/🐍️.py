import json, sys

def slot(r):
    o = (r.get("target") or {}).get("options") or {}
    return json.dumps([r.get("fn"), r.get("projectRoot"), o.get("command"), o.get("cwd"), r.get("role"), r.get("name"), r.get("view"), r.get("port"), r.get("entries")], separators=(",", ":"), ensure_ascii=False)

def plan(c):
    now, grace = c["now"], c["grace"]
    drop = {"entries": [], "layers": [], "values": []}
    kept = [e for e in c["entries"] if e["record"]]
    drop["entries"] += [e["name"] for e in c["entries"] if not e["record"] and now - e["mtimeMs"] >= grace]
    best = {}
    for e in kept:
        k = slot(e["record"]["recipe"])
        b = best.get(k)
        if b is None or (e["mtimeMs"], e["name"]) > (b["mtimeMs"], b["name"]):
            best[k] = e
    live = []
    for e in kept:
        w = best[slot(e["record"]["recipe"])]
        if w is not e and now - w["mtimeMs"] >= grace:
            drop["entries"].append(e["name"])
        else:
            live.append(e)
    named = {d for e in live for d in e["record"]["layers"]}
    valued = {e["record"]["valueDigest"] for e in live}
    drop["layers"] = [l["name"] for l in c["layers"] if l["name"] not in named and now - l["mtimeMs"] >= grace]
    drop["values"] = [v["name"] for v in c["values"] if v["name"] not in valued and now - v["mtimeMs"] >= grace]
    return {k: sorted(v) for k, v in drop.items()}

print(json.dumps([plan(c) for c in json.load(sys.stdin)]))
