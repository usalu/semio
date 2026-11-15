#!/usr/bin/env python3
"""🐍️ Independent oracle of the shared worktree canary rules. It shares no code with `../🟦️.ts`: it reads the same language-agnostic fixture and decides every case again (who may switch to new code, in which order the candidates go, how long a canary has been healthy).
usage: 🐍️.py            exit 0 when every vector agrees, 1 on any mismatch
Python 3.9 compatible (no match statements)."""
import json,math,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
FIXTURE=json.load(open(os.path.join(HERE,"🧫️fixtures","🔣️.json"),encoding="utf8"))
failures=[]

def key(text): return text.encode("utf-8")

def fresh(now,wall,limit): return abs(now-wall)<=limit

def usable(peer,item,limit):
    return peer["groupId"]==item["groupId"] and peer["originId"] not in item["retired"] and peer["sync"] and not peer["fallback"] and fresh(item["now"],peer["wallMs"],limit)

def order(item):
    limit=item["limits"]["freshMs"]
    found={item["originId"]}
    for peer in item["peers"]:
        if peer["originId"]!=item["originId"] and usable(peer,item,limit): found.add(peer["originId"])
    server=item["groupId"]+"-server"
    rest=sorted([o for o in found if o!=server],key=key)
    return ([server] if server in found else [])+rest

def whole(milliseconds): return max(0,int(math.ceil(milliseconds/1000.0)))

def decide(item):
    if item["digest"]==item["running"]: return {"ok":True,"role":"running"}
    refusals=[r for r in item["refusals"] if r["digest"]==item["digest"]]
    if refusals:
        refusals.sort(key=lambda r:(r["wallMs"],key(r["originId"])))
        return {"ok":False,"why":"refused","origin":refusals[0]["originId"],"seconds":0}
    if item["digest"] in item["snapshots"]: return {"ok":True,"role":"snapshot"}
    limits=item["limits"]
    runners=sorted([p["originId"] for p in item["peers"] if p["originId"]!=item["originId"] and p["code"]==item["digest"] and usable(p,item,limits["freshMs"])],key=key)
    if runners:
        best=runners[0]
        for origin in runners:
            if item["healthy"].get(origin,0)>item["healthy"].get(best,0): best=origin
        age=item["healthy"].get(best,0)
        if age>=limits["canaryMs"]: return {"ok":True,"role":"follower"}
        return {"ok":False,"why":"canary","origin":best,"seconds":whole(limits["canaryMs"]-age)}
    if item["authored"]: return {"ok":True,"role":"author"}
    candidates=order(item); rank=candidates.index(item["originId"]); due=limits["waitMs"]+rank*limits["turnMs"]
    if item["waitedMs"]>=due: return {"ok":True,"role":"leader"}
    if item["waitedMs"]<limits["waitMs"]: return {"ok":False,"why":"mixed","origin":candidates[0],"seconds":whole(limits["waitMs"]-item["waitedMs"])}
    return {"ok":False,"why":"turn","origin":candidates[rank-1],"seconds":whole(due-item["waitedMs"])}

def track(case):
    seen={}; ages=[]
    for step in case["steps"]:
        healthy=set(p["originId"] for p in step["peers"] if p["originId"]!=case["originId"] and p["code"]==case["digest"] and p["sync"] and not p["fallback"] and fresh(step["now"],p["wallMs"],case["freshMs"]))
        for origin in list(seen):
            if origin not in healthy: del seen[origin]
        out={}
        for origin in sorted(healthy,key=key):
            before=seen.get(origin)
            since=before["since"] if before is not None and step["now"]-before["last"]<=case["freshMs"] else step["now"]
            seen[origin]={"since":since,"last":step["now"]}
            out[origin]=step["now"]-since
        ages.append(out)
    return ages

for case in FIXTURE["decisions"]:
    got=decide(case["input"])
    if got!=case["expected"]: failures.append("%s: got %s, expected %s"%(case["name"],got,case["expected"]))
for case in FIXTURE["trackers"]:
    got=track(case)
    want=[s["expected"] for s in case["steps"]]
    if got!=want: failures.append("%s: got %s, expected %s"%(case["name"],got,want))
if failures:
    print("\n".join(failures)); sys.exit(1)
print("%d decisions and %d trackers agree"%(len(FIXTURE["decisions"]),len(FIXTURE["trackers"])))
