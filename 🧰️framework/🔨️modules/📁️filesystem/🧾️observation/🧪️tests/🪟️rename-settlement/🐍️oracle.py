import json,sys
fixture=json.load(open(sys.argv[1]))
schema=json.load(open(sys.argv[2]))["definitions"]
policy=schema["RelativeRenameSettlement"]["const"]
names=schema["RelativeWindowsFailureNames"]["const"]["nt"]
SHARING,DENIED,COLLISION,INVALID_CLASS=0xc0000043,0xc0000022,0xc0000035,0xc0000003
BUDGET,CAP,START,GROWTH,EVERY=policy["budgetMilliseconds"],policy["capMilliseconds"],policy["initialMilliseconds"],policy["growth"],policy["reportEveryMilliseconds"]

def status_at(sc,method,t):
    v=sc["volume"];active=[h for h in v["foreign"] if h["releaseAtMs"] is None or t<h["releaseAtMs"]]
    if method=="posix-ex" and not v["posixEx"]:return INVALID_CLASS
    if any(h["where"]=="at" and not h["shareDelete"] for h in active):return SHARING
    if any(h["where"]=="below" and not h["shareDelete"] for h in active):return DENIED
    if v["readonly"] and method!="posix-ex":return DENIED
    if v["collision"]:return COLLISION
    return 0

def texts(reason,status,attempts,waited,holders,count):
    out={}
    for lang in ("en","de"):
        c=policy["texts"][lang];hexs="0x%x"%status
        words=policy["statusWords"].get(str(status),{}).get(lang,hexs)
        values={"source":fixture["source"],"target":fixture["target"],"status":hexs,"statusName":names.get(str(status),hexs),"words":words,"attempts":str(attempts),"waited":str(waited),"holders":", ".join(holders) if holders else c["unknown"],"count":str(count)}
        s=c["own" if reason=="own" else "foreign"]
        for k,v in values.items():s=s.replace("{"+k+"}",v)
        out[lang]=s
    return out

def simulate(sc):
    if sc["own"]>0:
        t=texts("own",0,0,0,[],sc["own"])
        return {"outcome":"refused-own","attempts":0,"waited":0,"pauses":[],"reports":[],"en":t["en"],"de":t["de"]}
    method="posix-ex";t=0;delay=START;attempts=0;pauses=[];reports=[];last=None
    while True:
        if sc["cancelAfterPauses"] is not None and len(pauses)>=sc["cancelAfterPauses"]:
            return {"outcome":"cancelled","attempts":attempts,"waited":t,"pauses":pauses,"reports":reports}
        attempts+=1;st=status_at(sc,method,t)
        if method=="posix-ex" and st in policy["fallbackStatuses"]:method="classic";st=status_at(sc,method,t)
        if st==0:return {"outcome":"moved","method":method,"attempts":attempts,"waited":t,"pauses":pauses,"reports":reports}
        if st not in policy["transientStatuses"]:return {"outcome":"failed","status":st,"attempts":attempts,"waited":t,"pauses":pauses,"reports":reports}
        remaining=BUDGET-t
        if remaining<=0:
            holders=["process %d"%h["pid"] for h in sc["volume"]["foreign"] if h["releaseAtMs"] is None or t<h["releaseAtMs"]]
            tx=texts("foreign",st,attempts,t,holders,0)
            return {"outcome":"refused-foreign","status":st,"attempts":attempts,"waited":t,"pauses":pauses,"reports":reports,"holders":holders,"en":tx["en"],"de":tx["de"]}
        pause=min(delay,remaining)
        if last is None or t-last>=EVERY:reports.append(t);last=t
        pauses.append(pause);t+=pause;delay=min(delay*GROWTH,CAP)

def count_below(table):
    parent={"parent":None};name={"source":"gen"};up={"source":"parent"}
    for h in table["handles"]:
        if h["id"]=="parent":continue
        up[h["id"]]=h["parent"];name[h["id"]]=h["name"]
    total=0
    for ident in up:
        if ident=="source":continue
        chain=ident;hit=False
        while chain in up and chain!="parent":
            if chain=="source" or (up[chain]==up["source"] and name[chain]==name["source"]):hit=True;break
            chain=up[chain]
        total+=1 if hit else 0
    return total

print(json.dumps({"scenarios":[simulate(sc) for sc in fixture["scenarios"]],"tables":[count_below(t) for t in fixture["tables"]]},ensure_ascii=False))
