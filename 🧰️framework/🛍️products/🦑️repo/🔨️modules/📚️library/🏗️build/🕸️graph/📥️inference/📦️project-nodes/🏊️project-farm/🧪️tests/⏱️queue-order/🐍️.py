import json,sys
spec=json.load(open(sys.argv[1]))
q=spec["queue"]
def expected(p):
    if p.get("look"): return q["lookMilliseconds"]*max(1,len(p.get("owners",[1])))
    if p.get("warm"): return q["warmMilliseconds"]
    if p.get("observe"): return q["observeMilliseconds"]
    return q["inferMilliseconds"]+q["perMegabyteMilliseconds"]*max(0,p.get("costMegabytes",0))
out={}
for c in spec["cases"]:
    rows=[]
    for i,j in enumerate(c["jobs"]):
        s=float("-inf") if j.get("front") else expected(j["payload"])-(c["now"]-j["queued"])*q["agingPerMille"]/1000
        rows.append((s,i,j["id"]))
    rows.sort(key=lambda r:(r[0],r[1]))
    out[c["name"]]=[r[2] for r in rows]
print(json.dumps(out))
