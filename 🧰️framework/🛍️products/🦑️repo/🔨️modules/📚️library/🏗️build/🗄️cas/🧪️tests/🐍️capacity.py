import json,sys

def derived(total,budget,factor):
    reserve=budget["largestNeedBytes"]*factor
    return total>=(reserve+budget["largestNeedBytes"])*budget["minimumTotalFactor"],reserve

def host_floor(m,total):
    return min(m["hostFreeFloorBytes"],max(m["alignBytes"],(total*m["hostFreeFloorPercent"]//100//m["alignBytes"])*m["alignBytes"]))

def cap(v,row):
    sz=v["sizing"];roles={r["id"]:r for r in sz["roles"]};r=roles.get(row["role"],roles[sz["defaultRole"]])
    total,free,store=row["total"],row["free"],row["store"]
    ok,reserve=derived(total,v["budget"],v["factor"])
    headroom=max(host_floor(v["machine"],total),reserve if ok else -(-total*(100-int(v["highUsedPercent"]))//100))
    floor=sz["transientBytes"]+r["releases"]*sz["releaseBytes"]+(sz["serverBytes"] if r["server"] else 0)
    room=float("inf") if store is None else store+max(0,free-headroom)
    claim=0 if store is None else room*r["freePercent"]//100
    wanted=max(total*r["sharePercent"]//100,floor,claim)
    w=max(v["minimumCapBytes"],min(wanted,room));step=v["machine"]["alignBytes"]
    m=v["machine"];share=m.get("hostSharePercent");host_cap=float("inf") if share is None else max(step,(total*share//100//step)*step)
    proportional=min(v["capBytes"],host_cap,max(v["minimumCapBytes"],int(w//step)*step))
    if store is None:return proportional
    reserve_bytes=min(total//2,max(total*r["reservePercent"]//100,min(r["reserveFloorBytes"],total*sz["reserveFloorMaxPercent"]//100),host_floor(v["machine"],total)))
    hard=store+free-reserve_bytes
    return min(proportional,max(v["minimumCapBytes"],(hard//step)*step))

v=json.load(sys.stdin)
print(json.dumps([cap(v,row) for row in v["rows"]]))
