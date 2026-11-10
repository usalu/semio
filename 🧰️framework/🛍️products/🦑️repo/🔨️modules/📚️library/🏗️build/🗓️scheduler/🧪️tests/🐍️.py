#!/usr/bin/env python3
"""🐍️ Independent oracle of the fair global scheduler. It shares no code with `../🟦️.ts`: it reads only the closed policy of the schema and replays a scenario (arrivals, cancellations, estimates, priorities) with a reference weighted-round-robin, aging and backfill implementation, printing the start order.
usage: 🐍️.py simulate <scenario.json>        print the starts [[jobId, atMs], ...] of one scenario
       🐍️.py generate <seed> <count>          print `count` JSON lines of seeded random scenarios with their starts
Python 3.9 compatible."""
import json,os,random,sys
HERE=os.path.dirname(os.path.abspath(__file__))
SCHEMA=json.load(open(os.path.join(HERE,"..","🧬️schema","🔣️.json"),encoding="utf8"))
POLICY=SCHEMA["definitions"]["Policy"]["const"]

def simulate(scenario):
    p=dict(POLICY);p.update(scenario["policy"]);bands=p["priorityBands"]
    budget=scenario["budgetMegabytes"];avail=scenario.get("availableMegabytes")
    specs={j["ticket"]:j for j in scenario["jobs"]}
    jobs={};ticket_job={};key_job={};agents={};vt=[0];seq=[0];starts=[];running_end={}
    def eff(j,t): return min(len(bands)-1,bands.index(j["priority"])+(t-j["at"])//p["agingMs"])
    def tag(j): return max(agents.get(j["owner"],{"vt":0})["vt"],vt[0])
    def pick(cands,t):
        if not cands: return None
        top=max(eff(j,t) for j in cands);band=[j for j in cands if eff(j,t)==top]
        least=min(tag(j) for j in band)
        near=[j for j in band if tag(j)<=least+p["fairnessSlack"]*1000]
        near.sort(key=lambda j:(-j["depth"],tag(j),j["seq"]));return near[0]
    def finish(j,t):
        for k in j["tickets"]: ticket_job.pop(k,None)
        key_job.pop(j["key"],None);jobs.pop(j["id"],None);running_end.pop(j["id"],None)
        if j["owner"] in agents: agents[j["owner"]]["last"]=t
    def live(j): return [k for k,v in j["tickets"].items() if v=="live"]
    def submit(s,t):
        if s["ticket"] in ticket_job: return
        jid=key_job.get(s["key"])
        if jid is not None:
            j=jobs[jid]
            if len(j["tickets"])>=p["maxAttached"]: return
            j["tickets"][s["ticket"]]="live";ticket_job[s["ticket"]]=jid
            if bands.index(s["priority"])>bands.index(j["priority"]): j["priority"]=s["priority"]
            j["depth"]=max(j["depth"],s["depth"]);return
        if len([1 for j in jobs.values() if j["status"]=="queued"])>=p["maxQueued"]: return
        seq[0]+=1
        jobs[s["ticket"]]={"id":s["ticket"],"key":s["key"],"owner":s["agent"],"priority":s["priority"],"depth":s["depth"],"reserve":s["peakMegabytes"],"at":t,"seq":seq[0],"status":"queued","tickets":{s["ticket"]:"live"},"lane":s.get("lane"),"runner":None}
        ticket_job[s["ticket"]]=s["ticket"];key_job[s["key"]]=s["ticket"]
        a=agents.setdefault(s["agent"],{"vt":0,"last":t});a["last"]=t
    def cancel(k,t):
        jid=ticket_job.get(k)
        if jid is None: return
        j=jobs[jid]
        if j["tickets"][k]!="live": return
        j["tickets"][k]="cancelled"
        if j["status"]=="running":
            if j["runner"]==k: finish(j,t)
        elif not live(j): finish(j,t)
    def tick(t):
        while True:
            run=[j for j in jobs.values() if j["status"]=="running"];reserved=sum(j["reserve"] for j in run)
            if len(run)>=p["maxConcurrent"]: break
            lanes=p["sharedCompilerLanes"]
            elig=[j for j in jobs.values() if j["status"]=="queued" and len([1 for r in run if r["owner"]==j["owner"]])<p["maxPerAgent"] and (j["lane"] is None or j["lane"] not in lanes or len([1 for r in run if r["lane"]==j["lane"]])<lanes[j["lane"]])]
            def fits(j): return len(run)==0 or (reserved+j["reserve"]<=budget and (avail is None or j["reserve"]+p["availableHeadroomMegabytes"]<=avail))
            head=pick(elig,t)
            if head is None: break
            chosen=None
            if fits(head): chosen=head
            elif t-head["at"]<p["agingMs"]: chosen=pick([j for j in elig if fits(j)],t)
            if chosen is None: break
            runner=live(chosen)[0];chosen["status"]="running";chosen["runner"]=runner
            a=agents.setdefault(chosen["owner"],{"vt":0,"last":t});start=max(a["vt"],vt[0])
            cost=max(p["minimumCostMilli"],chosen["reserve"]*1000//p["referenceMegabytes"])
            a["vt"]=start+cost*1000//p["defaultWeight"];vt[0]=start;a["last"]=t
            running_end[chosen["id"]]=t+specs[runner]["durationMs"];starts.append([chosen["id"],t])
        busy=set(j["owner"] for j in jobs.values())
        if len(agents)>p["maxAgents"] or any(n not in busy and t-a["last"]>=p["agingMs"]*10 for n,a in agents.items()):
            idle=sorted([(a["last"],n) for n,a in agents.items() if n not in busy])
            excess=max(0,len(agents)-p["maxAgents"])
            for last,n in idle:
                if excess>0 or t-last>=p["agingMs"]*10:
                    del agents[n];excess-=1
    cancels=[(specs[k]["cancelAt"],k) for k in specs if "cancelAt" in specs[k]]
    arrivals=[(s["at"],s["ticket"]) for s in scenario["jobs"]]
    times=set(a for a,_ in arrivals)|set(c for c,_ in cancels)
    clock=-1
    while True:
        nxt=[x for x in times if x>clock]+[e for e in running_end.values() if e>clock]
        if not nxt: break
        clock=min(nxt)
        for jid in sorted([i for i,e in running_end.items() if e<=clock],key=lambda i:(running_end[i],jobs[i]["seq"])):
            finish(jobs[jid],clock)
        for c,k in cancels:
            if c==clock: cancel(k,clock)
        for a,k in arrivals:
            if a==clock: submit(specs[k],clock)
        tick(clock)
    return starts

AGENTS=["alpha","bravo","charlie","delta","echo","foxtrot"]
def generate(seed):
    rng=random.Random(seed);agents=rng.randint(1,6);count=rng.randint(4,40)
    policy={"agingMs":rng.choice([500,2000,120000]),"maxConcurrent":rng.randint(1,5),"maxPerAgent":rng.randint(1,3),"heartbeatTimeoutMs":10**9}
    if rng.random()<0.3: policy["sharedCompilerLanes"]={"rustc":1,"tsc":2}
    jobs=[]
    for i in range(count):
        job={"ticket":"t%d"%i,"at":rng.randint(0,rng.choice([0,3000,20000])),"agent":rng.choice(AGENTS[:agents]),"durationMs":rng.randint(100,3000),"priority":rng.choice(["background","agent","agent","interactive"]),"depth":rng.randint(0,6),"key":"k%d"%rng.randint(0,count),"peakMegabytes":rng.choice([0,200,512,1024,2048,4096])}
        if rng.random()<0.3: job["lane"]=rng.choice(["rustc","tsc"])
        if rng.random()<0.12: job["cancelAt"]=job["at"]+rng.randint(0,2500)
        jobs.append(job)
    scenario={"id":"seed-%d"%seed,"policy":policy,"budgetMegabytes":rng.choice([2048,4096,8192]),"jobs":jobs}
    if rng.random()<0.3: scenario["availableMegabytes"]=rng.choice([1024,3000,6000])
    scenario["starts"]=simulate(scenario);return scenario

if __name__=="__main__":
    command=sys.argv[1]
    if command=="simulate": print(json.dumps(simulate(json.load(open(sys.argv[2],encoding="utf8")))))
    elif command=="generate":
        for n in range(int(sys.argv[3])): print(json.dumps(generate(int(sys.argv[2])+n),ensure_ascii=False))
    else: sys.exit(2)
