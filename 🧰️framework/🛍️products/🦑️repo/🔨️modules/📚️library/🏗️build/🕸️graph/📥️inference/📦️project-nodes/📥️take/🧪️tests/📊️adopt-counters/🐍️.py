import json,sys
events=json.load(sys.stdin)["events"]
listed=set();skipped=set();stale={};s=dict(looks=0,adoptLooks=0,inferLooks=0,passLooks=0,fetched=0,present=0,stale=0,incomplete=0,miss=0,fault=0,gateWaits=0,gateMilliseconds=0,inferredListed=0)
for e in events:
    k=e["k"]
    if k=="list":listed.update(e["names"])
    elif k=="look":
        s["looks"]+=1
        lane=e["lane"]
        s["adoptLooks" if lane=="adopt" else "inferLooks" if lane=="infer" else "passLooks"]+=1
    elif k=="outcome" and e["o"] in ("fetched","present","stale","incomplete","miss","fault"):s[e["o"]]+=1
    elif k=="staleRow":
        kind=e["kind"] if isinstance(e["kind"],str) and e["kind"]!="" else "unnamed"
        stale[kind]=stale.get(kind,0)+1
    elif k=="skip":skipped.add(e["name"])
    elif k=="gate":s["gateWaits"]+=1;s["gateMilliseconds"]+=e["ms"]
    elif k=="inferredListed":s["inferredListed"]+=1
out=dict(s);out.update(listed=len(listed),adopted=s["fetched"]+s["present"],skippedInferring=len(skipped),gateSeconds=int(s["gateMilliseconds"]/1000+0.5),staleKinds=dict(sorted(stale.items())))
print(json.dumps(out,sort_keys=True))
