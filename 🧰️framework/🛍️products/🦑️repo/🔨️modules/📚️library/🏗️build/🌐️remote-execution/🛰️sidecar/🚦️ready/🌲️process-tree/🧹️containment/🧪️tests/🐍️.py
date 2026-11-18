#!/usr/bin/env python3
"""Independent oracle of the containment rules: the sample delay, the windows of the stopped tries, the Windows `taskkill` arguments and the orphan plan (a fixpoint over the table for the stage tree, explicit parent walks for the shelter rules, instead of the breadth-first walk of the code). Reads the schema for the numbers, the process-tree schema for the tolerance, the fixture for the cases; the owner tokens (a SHA-256 of the lower-cased path, `/` separators), proof of ownership by token instead of by start time, the spared resident services; prints the verdicts as JSON. Usage: oracle.py <containment schema.json> <process-tree schema.json> <fixture.json>"""
import hashlib,json,re,sys
C=json.load(open(sys.argv[1],encoding="utf8"))["definitions"]["Policy"]["const"]
T=json.load(open(sys.argv[2],encoding="utf8"))["definitions"]["Policy"]["const"];TOL=T["toleranceMilliseconds"];RESERVED=T["reservedPids"]
fx=json.load(open(sys.argv[3],encoding="utf8"))
key=lambda s:s.replace("\\","/").lower()
def delay(c):return C["fastSampleMilliseconds"] if c["elapsedMs"]<C["fastWindowMilliseconds"] else C["slowSampleMilliseconds"]
def windows(c):return [{"stage":r["stage"],"startedMs":r["startedMs"],"endedMs":r["endedMs"]} for r in c["runs"] if r["outcome"] in C["stoppedOutcomes"]]
def owner(c):return "ready:"+hashlib.sha256(key(c["root"]).encode("utf8")).hexdigest()[:12]+":"+c["stage"]+":"+str(c["startedMs"])
def taskkill(c):return ["/PID",str(c["pid"]),"/T","/F"]
def heirs(c):
    table={r["pid"]:r for r in c["table"]};prev={m["pid"]:m for m in c["previous"]};out=[]
    for r in c["table"]:
        m=prev.get(r["ppid"])
        if m is None or r["pid"] in prev or m["pid"] in table:continue
        if r["startMs"]>=m["startMs"]-TOL and r["startMs"]>=c["sinceMs"]-TOL:out.append(r["pid"])
    return out
def orphans(c):
    table={r["pid"]:r for r in c["table"]};root=key(c["root"])
    prot=set();at=c["self"]
    while at is not None and at not in prot and len(prot)<256:
        prot.add(at);r=table.get(at);at=r["ppid"] if r else None
    stage=set()
    if c["stage"]:
        seeds=[c["stage"]["root"]]+c["stage"]["known"]
        live=[s["pid"] for s in seeds if s["pid"] in table and (s["startMs"]==0 or abs(table[s["pid"]]["startMs"]-s["startMs"])<=TOL)]
        stage=set(live)
        child={}
        for r in c["table"]:
            p=table.get(r["ppid"])
            if p is not None and r["startMs"]>=p["startMs"]-TOL and r["pid"]!=r["ppid"]:child[r["pid"]]=r["ppid"]
        grew=True
        while grew:
            grew=False
            for pid,par in child.items():
                if par in stage and pid not in stage:stage.add(pid);grew=True
        stage={p for p in stage if p>RESERVED}
    owners={owner({"root":c["root"],"stage":w["stage"],"startedMs":w["startedMs"]}) for w in c["windows"]}
    out=set();h=c["human"]
    for e in c["entries"]:
        pid=e["pid"]
        if pid<=RESERVED or pid in prot or pid in stage or not e["startedAtMs"]>0:continue
        if e.get("owner") not in owners:continue
        if re.match(C["sharedCommandPattern"],e["command"]):continue
        r=table.get(pid)
        if r is None or abs(r["startMs"]-e["startedAtMs"])>TOL:continue
        s=e["startedAtMs"]
        if h and h["startedMs"]-C["edgeToleranceMilliseconds"]<=s<h["startedMs"]+C["humanWindowMilliseconds"]:continue
        sheltered=False;p=table.get(r["ppid"]);n=0
        while p is not None and n<256 and not sheltered:
            if p["startMs"]<=r["startMs"]+TOL and (p["pid"] in stage or (h and h.get("pid")==p["pid"])):sheltered=True
            p=table.get(p["ppid"]);n+=1
        if sheltered:continue
        out.add(pid)
    return sorted(out)
print(json.dumps({"delay":[delay(c) for c in fx["delay"]],"window":[windows(c) for c in fx["window"]],"taskkill":[taskkill(c) for c in fx["taskkill"]],"orphan":[orphans(c) for c in fx["orphan"]],"heir":[heirs(c) for c in fx["heir"]],"owner":[owner(c) for c in fx["owner"]]},sort_keys=True))
