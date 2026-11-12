import json,sys

def run(case):
    p=case["policy"];cap=p["capBytes"];hi=p["highWatermark"];lo=p["lowWatermark"];now=case["nowMs"]
    order={a["id"]:i for i,a in enumerate(case["areas"])}
    entries=sorted(case["entries"],key=lambda e:(order[e["area"]],e["id"]))
    for i,e in enumerate(entries):e["i"]=i
    victims={}
    for a in case["areas"]:
        if a["maxAgeMs"] is None:continue
        for e in entries:
            if e["area"]==a["id"] and not e["protected"] and e["access"]<now-a["maxAgeMs"]:victims[e["id"]+"@"+e["area"]]="age"
    key=lambda e:e["id"]+"@"+e["area"]
    for a in case["areas"]:
        if a["capBytes"] is None:continue
        members=[e for e in entries if e["area"]==a["id"] and key(e) not in victims]
        total=sum(e["size"] for e in members)
        if total>a["capBytes"]:
            need=total-a["capBytes"]*lo
            for e in sorted([e for e in members if not e["protected"]],key=lambda e:(e["access"],e["i"])):
                if need<=0:break
                victims[key(e)]="area";need-=e["size"]
    left=[e for e in entries if key(e) not in victims]
    after=case["externalBytes"]+sum(e["size"] for e in left)
    planned=sum(e["size"] for e in entries if key(e) in victims)
    cap_need=after-cap*lo if after>cap*hi else 0
    need=max(cap_need,max(0,case.get("partitionNeedBytes",0)-planned))
    if need>0:
        for e in sorted([e for e in left if not e["protected"]],key=lambda e:(e["access"],e["i"])):
            if need<=0:break
            victims[key(e)]="host";need-=e["size"]
    return victims

cases=json.load(sys.stdin)
print(json.dumps([run(c) for c in cases]))
