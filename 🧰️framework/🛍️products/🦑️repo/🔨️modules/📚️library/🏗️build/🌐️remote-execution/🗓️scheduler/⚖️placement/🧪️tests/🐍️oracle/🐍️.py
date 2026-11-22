import json,sys

def matches(required,offered):
    have={}
    for p in offered:
        have.setdefault(p["name"],set()).add(p["value"])
    return all(p["value"] in have.get(p["name"],set()) for p in required)

def required_of(platform,entry):
    own=[p for p in platform if p["name"] not in ("semio-offload","semio-origin","semio-peak-mb")]
    if entry is None:
        return own
    ident=entry["report"]["platform"]
    named={p["name"] for p in own}
    out=list(own)
    for name,value in (("Arch",ident["arch"]),("OSFamily",ident["os"]),("semio-toolchain",ident["toolchain"]),("semio-target",ident["target"])):
        if value is not None and name not in named:
            out.append({"name":name,"value":value})
    return out

def peak_of(platform):
    for p in platform:
        if p["name"]=="semio-peak-mb" and p["value"].isdigit() and len(p["value"])<=9:
            return int(p["value"])
    return 0

def headroom(w,entry,now,peak=0):
    if w["draining"] or w["running"]>=w["concurrency"] or now<w["refusedUntilMs"]:
        return False
    if entry is None:
        return peak<=0
    r=entry["report"]
    return r["pressure"]!="critical" and r["headroom"] and (peak<=0 or r["freeMegabytes"]-w.get("committedMegabytes",0)>=peak)

def score(w,entry):
    if entry is None:
        return 1000+w["running"]
    r=entry["report"]
    cores=max(1,r["capacity"]["cores"])
    local=max(0,r["running"]-r["worker"]["running"])+r["queued"]
    load=r["load"] if r["load"] is not None else 0
    return (local+w["running"])/cores+(1 if r["pressure"]=="warn" else 0)+0.25*load/cores

def plan(scn):
    table=scn["table"]
    now=scn["now"]
    free=[w for w in scn["workers"] if w["waiting"]]
    out=[]
    for op in scn["queue"]:
        if not free:
            break
        req=required_of(op["platform"],table.get(op["requester"]) if op["requester"] is not None else None)
        best=None
        for w in free:
            key=w["origin"] or w["name"]
            if key in op["tried"] or key==op["requester"]:
                continue
            if not matches(req,w["properties"]):
                continue
            e=table.get(w["origin"])
            if not headroom(w,e,now,peak_of(op["platform"])):
                continue
            s=score(w,e)
            if best is None or s<best[0] or (s==best[0] and (w["running"]<best[1]["running"] or (w["running"]==best[1]["running"] and w["name"]<best[1]["name"]))):
                best=(s,w)
        if best is None:
            continue
        out.append({"operation":op["id"],"worker":best[1]["id"]})
        free=[w for w in free if w["id"]!=best[1]["id"]]
    return out

scenarios=json.load(sys.stdin)
json.dump([plan(s) for s in scenarios],sys.stdout)
