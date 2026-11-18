#!/usr/bin/env python3
"""Independent oracle of the process-tree rules: the kill plan (a fixpoint over the table instead of a breadth-first walk), the lease verdict, the deadline verdict and the progress key (the last matching line), the stall verdict and the wait on a named holder. Reads the schema for the numbers and the patterns, the fixture for the cases, prints the verdicts as JSON. Usage: oracle.py <schema.json> <fixture.json>"""
import json,re,sys
schema=json.load(open(sys.argv[1],encoding="utf8"));fixture=json.load(open(sys.argv[2],encoding="utf8"))
P=schema["definitions"]["Policy"]["const"];TOL=P["toleranceMilliseconds"]
def identity(known,table):
    row=next((r for r in table if r["pid"]==known["pid"]),None)
    if row is None:return "gone"
    return "same" if known["startMs"]==0 or abs(row["startMs"]-known["startMs"])<=TOL else "reused"
def plan(case):
    table={r["pid"]:r for r in case["table"]}
    parent={}
    for r in case["table"]:
        p=table.get(r["ppid"])
        if p is not None and r["startMs"]>=p["startMs"]-TOL and r["pid"]!=r["ppid"]:parent[r["pid"]]=r["ppid"]
    protected=set();at=case["self"]
    for _ in range(256):
        if at is None or at in protected:break
        protected.add(at);row=table.get(at);at=row["ppid"] if row else None
    seeds=[k for k in [case["root"]]+case["known"] if identity(k,case["table"])=="same"]
    depth={k["pid"]:0 for k in seeds}
    changed=True
    while changed:
        changed=False
        for pid,par in parent.items():
            if par in depth:
                d=depth[par]+1
                if pid not in depth or d<depth[pid]:depth[pid]=d;changed=True
    spared=set(p for p in case.get("spare",[]) if p in depth)
    grew=True
    while grew:
        grew=False
        for pid,par in parent.items():
            if par in spared and pid not in spared:spared.add(pid);grew=True
    for pid in case.get("spare",[]):
        if pid in depth:
            at=parent.get(pid);guard=0
            while at is not None and at!=case["root"]["pid"] and guard<256:protected.add(at);at=parent.get(at);guard+=1
    members=[(pid,d) for pid,d in depth.items() if pid>P["reservedPids"] and pid not in protected and pid not in spared]
    return [pid for pid,d in sorted(members,key=lambda x:(-x[1],x[0]))]
def cpu(case):
    rows={};par={}
    for line in case["rows"].split("\n"):
        m=re.match(r"^\s*(\d+)\s+(\d+)\s+\d+\s+(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+(?:[.,]\d+)?)\s*$",line)
        if m:
            s=((int(m.group(3) or 0)*24+int(m.group(4) or 0))*60+int(m.group(5)))*60+float(m.group(6).replace(",","."))
            rows[int(m.group(1))]=round(s*1000);par[int(m.group(1))]=int(m.group(2))
    tree={}
    for pid,ms in rows.items():
        at=pid;hit=pid in case["extra"]
        for _ in range(64):
            if hit or at is None:break
            if at==case["root"]:hit=True;break
            at=par.get(at)
        if hit:tree[str(pid)]=ms
    before={int(k):v for k,v in case["before"].items()};spent=0;joined=False
    for k,v in tree.items():
        b=before.get(int(k))
        if b is None:joined=joined or bool(before)
        else:spent+=max(0,v-b)
    return {"tree":tree,"alive":joined or spent>=P["livenessCpuMilliseconds"]}
def lease(case):
    l=case.get("lease")
    if not l:return {"state":"free","reason":"none"}
    if case["nowMs"]-l["wallMs"]>case["staleMs"]:return {"state":"free","reason":"expired"}
    if case["process"]=="gone":return {"state":"free","reason":"owner-gone"}
    s=l.get("startMs");ps=case.get("processStartMs")
    if case["process"]=="alive" and s and ps is not None and abs(s-ps)>TOL:return {"state":"free","reason":"pid-reused"}
    return {"state":"held","reason":"held"}
def deadline(case):
    e=case["elapsedMs"];soft=case["softMs"]
    if e<=soft:return "run"
    if e>soft*P["hardCapMultiplier"]:return "timeout"
    return "extended" if case["sinceProgressMs"]<=P["stallMilliseconds"] else "stalled"
def progress(case):
    pats=[re.compile(p) for p in P["progressPatterns"]]
    for line in reversed(case["lines"]):
        for p in pats:
            m=p.search(line)
            if m:return m.group(0)
    return None
def normalise(line):
    text=line.strip()
    for pat in P["normalisePatterns"]:text=re.sub(pat,"#",text)
    return text
def stall(case):
    seen=set();out=[]
    for lines in case["rounds"]:
        fresh=False
        for line in lines:
            key=normalise(line)
            if key=="" or key in seen:continue
            seen.add(key);fresh=True
        out.append(fresh)
    return out
def holder(case):
    pats=[re.compile(x) for x in P["holderWaitPatterns"]];since=None;out=[]
    for r in case["rounds"]:
        waiting=any(p.search(l) for l in r["lines"] for p in pats)
        if not waiting:since=None;out.append({"waitingMs":0,"restart":False});continue
        if since is None:since=r["atMs"]
        w=r["atMs"]-since
        out.append({"waitingMs":w,"restart":bool(r["codeChanged"]) and w>=P["holderWaitRestartMilliseconds"]})
    return out
print(json.dumps({"cpu":[cpu(c) for c in fixture["cpu"]],"holder":[holder(c) for c in fixture["holder"]],"stall":[stall(c) for c in fixture["stall"]],"tree":[plan(c) for c in fixture["tree"]],"lease":[lease(c) for c in fixture["lease"]],"deadline":[deadline(c) for c in fixture["deadline"]],"progress":[progress(c) for c in fixture["progress"]]},sort_keys=True))
