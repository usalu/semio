"""🧩️ Independent oracle of the image shard rules: key and id of a logical path, capacity at birth, sizing from the measured need and growth targets, shard selection, aggregation, upfront admission, path budget and the due decision, written from the policy text with the standard library only."""
import hashlib,json,re,sys
GIB=1<<30
CAPS={"darwin":32*GIB,"linux":128*GIB,"win32":64*GIB}
SUPPORTED={"darwin"}
MINIMUM=GIB;ALIGN=GIB;BIRTH=128*(1<<20);MAXIMUM_SHARDS=256;BUDGET=256
def shard_id(key):
    return key if re.fullmatch(r"[a-z0-9]{1,12}",key) else "x"+hashlib.sha256(key.encode()).hexdigest()[:11]
def key_of(rule,path):
    parts=[] if path=="" else path.split("/")
    if any(p in("",".","..") or "\\" in p or "\0" in p for p in parts):return None
    for index,root in enumerate(rule["roots"]):
        prefix=[] if root["prefix"]=="" else root["prefix"].split("/")
        if len(parts)>len(prefix) and parts[:len(prefix)]==prefix and re.fullmatch(root["pattern"],parts[len(prefix)]):
            k=parts[len(prefix)];return {"root":index,"key":k,"id":shard_id(k)}
    return None
def capacity(platform,rule,need,host_free,budget_room=None):
    cap=CAPS[platform]
    if "maximumImageBytes" in rule:cap=min(cap,rule["maximumImageBytes"])
    cap=cap//ALIGN*ALIGN
    want=max(MINIMUM,-(-(need*(100+rule["headroomPercent"])//100)//ALIGN)*ALIGN)
    if want>cap:return None
    room=host_free*rule["hostFreeReservePercent"]//100//ALIGN*ALIGN
    if room<want:return None
    if budget_room is None:return min(cap,room)
    left=max(0,budget_room)//ALIGN*ALIGN
    return "budget" if left<want else min(cap,room,left)
def want(rule,need):
    return max(MINIMUM,-(-(need*(100+rule["headroomPercent"])//100)//ALIGN)*ALIGN)
def cap_of(platform,rule):
    return min(CAPS[platform],rule.get("maximumImageBytes",1<<62))//ALIGN*ALIGN
def sized(platform,rule,need,measured):
    return min(cap_of(platform,rule),want(rule,max(need,measured or 0)))
GROW_AT=85
def grow_target(platform,rule,capacity,used,measured,host_free,budget_room):
    peak=max(used,measured or 0)
    if peak*100<capacity*GROW_AT:return None
    room=host_free*rule["hostFreeReservePercent"]//100//ALIGN*ALIGN
    budget=1<<62 if budget_room is None else capacity+max(0,budget_room)//ALIGN*ALIGN
    target=min(cap_of(platform,rule),want(rule,peak),room,budget)
    return target if target>capacity else None
def select(rule,platform,live,path,host_free,budget=None):
    if not rule["enabled"] or platform not in SUPPORTED:return {"kind":"home"}
    found=key_of(rule,path)
    if found is None:return {"kind":"home"}
    base={"id":found["id"],"key":found["key"],"root":found["root"]}
    record=next((r for r in live if r["id"]==found["id"]),None)
    if record is not None:
        if record["key"]==found["key"]:return {"kind":"existing",**base,"capacityBytes":record["capacityBytes"]}
        return {"kind":"refused",**base,"reason":"id-collision"}
    if len(live)>=MAXIMUM_SHARDS:return {"kind":"refused",**base,"reason":"shard-limit"}
    need=rule["roots"][found["root"]]["needBytes"]
    want=max(MINIMUM,-(-(need*(100+rule["headroomPercent"])//100)//ALIGN)*ALIGN)
    cap=min(CAPS[platform],rule.get("maximumImageBytes",1<<62))//ALIGN*ALIGN
    if want>cap:return {"kind":"refused",**base,"reason":"cap"}
    room=None if budget is None else budget-sum(r["capacityBytes"] for r in live if r["root"]==found["root"])
    c=capacity(platform,rule,need,host_free,room)
    if c is None:return {"kind":"refused",**base,"reason":"host-free"}
    if c=="budget":return {"kind":"refused",**base,"reason":"budget"}
    return {"kind":"birth",**base,"capacityBytes":c}
def aggregate(rows):
    vol={}
    for r in rows:
        v=vol.setdefault(r["volume"],{"free":0,"host":None})
        v["free"]+=max(0,r["capacityBytes"]-r["usedBytes"])
        v["host"]=r["hostFreeBytes"] if v["host"] is None else min(v["host"],r["hostFreeBytes"])
    return {"shards":len(rows),"capacityBytes":sum(r["capacityBytes"] for r in rows),"usedBytes":sum(r["usedBytes"] for r in rows),
            "allocatedBytes":sum(r["allocatedBytes"] for r in rows),"slackBytes":sum(max(0,r["allocatedBytes"]-r["usedBytes"]) for r in rows),
            "freeBytes":sum(min(v["free"],v["host"]) for v in vol.values()),"hostFreeBytes":sum(v["host"] for v in vol.values()),
            "rows":[{"id":r["id"],"freeBytes":min(max(0,r["capacityBytes"]-r["usedBytes"]),vol[r["volume"]]["host"])} for r in rows]}
def admission(rows,reserve,credit):
    vol={}
    for r in rows:
        growth=max(0,r["needBytes"]-r["slackBytes"]*credit//10000)+(BIRTH if r["newShard"] else 0)
        v=vol.setdefault(r["volume"],{"need":0,"host":None});v["need"]+=growth
        v["host"]=r["hostFreeBytes"] if v["host"] is None else min(v["host"],r["hostFreeBytes"])
    out={};total=0
    for name in sorted(vol):
        need=vol[name]["need"]+reserve;short=max(0,need-vol[name]["host"])
        out[name]={"needBytes":need,"hostFreeBytes":vol[name]["host"],"shortfallBytes":short};total+=short
    return {"volumes":out,"shortfallBytes":total}
def join(flavor,*parts):
    sep="\\" if flavor=="win32" else "/"
    out=""
    for p in parts:
        if out=="":out=p
        else:out=out.rstrip(sep)+sep+p.lstrip(sep)
    return out
def paths(flavor,directory,sid):
    sep="\\" if flavor=="win32" else "/"
    owned=[join(flavor,directory,"s",sid,"fixed.dmg"),join(flavor,directory,"s",sid,"shard.json"),join(flavor,directory,"shards.jsonl")]
    longest=max(len(o) for o in owned)
    suffix=max(len(sep.join(["s",sid,"fixed.dmg"])),len(sep.join(["s",sid,"shard.json"])),len("shards.jsonl"))+1
    return {"longest":longest,"maximumDirectoryCharacters":BUDGET-max(suffix,len("shards.jsonl")+1),"fits":longest<=BUDGET}
def due(rule,f):
    tight=f["hostFreeBytes"]<rule["reclaimTightHostFreeBytes"]
    floor=rule["reclaimFloorBytes"]//8 if tight else rule["reclaimFloorBytes"]
    return f["sinceLastKickMilliseconds"]>=60000 and f["allocatedBytes"]-f["usedBytes"]>=floor and (tight or f["allocatedBytes"]*100>=f["usedBytes"]*rule["reclaimRatioPercent"])
def answers(c):
    rule=c["rule"]
    return {
     "keys":[{"path":k["path"],**({"root":None,"key":None,"id":None} if key_of(rule,k["path"]) is None else key_of(rule,k["path"]))} for k in c["keys"]],
     "capacity":[capacity(x["platform"],rule,x["need"],x["hostFreeBytes"]) for x in c["capacity"]],
     "selection":[select({**rule,"enabled":x["enabled"]},x["platform"],x["live"],x["path"],x["hostFreeBytes"]) for x in c["selection"]],
     "commitment":[select(rule,"darwin",x["live"],x["path"],x["hostFreeBytes"],x["budgetBytes"]) for x in c["commitment"]],
     "aggregate":[aggregate(x["rows"]) for x in c["aggregate"]],
     "admission":[admission(x["rows"],x["reserveBytes"],x["slackCreditBasisPoints"]) for x in c["admission"]],
     "paths":[paths(x["flavor"],x["directory"],x["id"]) for x in c["paths"]],
     "reclaim":[[due(x["rule"],f) for f in x["facts"]] for x in c["reclaim"]],
     "sizing":[sized(x["platform"],rule,x["need"],x["measuredBytes"]) for x in c["sizing"]],
     "growth":[grow_target(x["platform"],rule,x["capacityBytes"],x["usedBytes"],x["measuredBytes"],x["hostFreeBytes"],x["budgetRoomBytes"]) for x in c["growth"]]}
if __name__=="__main__":
    if sys.argv[1]=="id":print(shard_id(sys.argv[2]))
    elif sys.argv[1]=="ids":print(json.dumps([shard_id(k) for k in sys.argv[2:]]))
    elif sys.argv[1]=="allocated":
        import os;print(os.stat(sys.argv[2]).st_blocks*512)
    elif sys.argv[1]=="answers":print(json.dumps(answers(json.load(open(sys.argv[2]))),sort_keys=True))
