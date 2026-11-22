"""Independent Python model of the fleet probe laws: reads {"op","cases"} as JSON on stdin and prints one result per case, using only the standard library."""
import json,sys
request=json.load(sys.stdin)
op,cases=request["op"],request["cases"]

def warm(case):
    p,h=case["published"],case["hits"]
    if not isinstance(p,int) or isinstance(p,bool) or p<1 or not isinstance(h,int) or h<0 or h>p: return None
    return {"hitRatio":h/p,"executedActions":p-h}

def twin(case):
    diff=sum(1 for row in case["outputs"] if row["left"]!=row["right"])
    return {"divergentWrites":diff+sum(1 for a in case["answers"] if a=="diverged")}

def memory(case):
    admitted=[d for d in case["decisions"] if d["verdict"]=="admit"]
    reserved=sum(d["reservedMegabytes"] for d in admitted);jobs=sum(d["jobs"] for d in admitted);out=[]
    if jobs>case["cores"]: out.append("jobs %d exceed %d cores"%(jobs,case["cores"]))
    if reserved>max(0,case["availableMegabytes"]-case["floorMegabytes"]): out.append("reserved %d MB exceed available %d MB beyond the floor %d MB"%(reserved,case["availableMegabytes"],case["floorMegabytes"]))
    if case["pressure"]=="critical" and admitted: out.append("%d requests admitted under critical pressure"%len(admitted))
    if case["pressure"]=="normal" and case["availableMegabytes"]-case["floorMegabytes"]>0 and case["decisions"] and not admitted: out.append("no request admitted on a roomy host")
    return {"value":reserved,"breaches":out}

def disk(case):
    total=sum(case["born"]);out=[]
    for b in case["born"]:
        if b%case["mebibyte"]!=0: out.append("image of %d bytes is not a whole number of mebibytes"%b)
    if total*100>case["freeBytes"]*case["sharePercent"]: out.append("images of %d bytes exceed %d %% of %d free bytes"%(total,case["sharePercent"],case["freeBytes"]))
    return {"value":total,"breaches":out}

def host(case):
    return {"capacityBreaches":sum(len(m["breaches"]) for m in case["memory"])+len(case["disk"]["breaches"]),"reservedBytes":max([0]+[m["value"] for m in case["memory"]])*case["mebibyte"],"storeBytes":case["disk"]["value"]}

def ready(case):
    live=[r for r in case["receipts"] if case["nowMs"]-r["updatedMs"]<=case["maximumAgeMs"]]
    if not live: return None
    rd=[r for r in live if r["ready"]]
    times=sorted(r["readyMs"]-r["startedMs"] for r in rd if r["readyMs"] is not None and r["readyMs"]>=r["startedMs"])
    out={"readyRatio":len(rd)/len(live)}
    if times:
        n=len(times);out["wallMs"]=(times[(n-1)//2]+times[n//2])/2
    return out

def first_task(case):
    for l in case["lines"]:
        if l["text"].startswith("> nx run "): return {"firstTaskMs":l["atMs"],"wallMs":case["wallMs"]}
    return None

ops={"ready":ready,"firsttask":first_task,"warm":warm,"twin":twin,"memory":memory,"disk":disk,"host":host}
if op not in ops: sys.exit("unknown op "+op)
json.dump([ops[op](c) for c in cases],sys.stdout)
