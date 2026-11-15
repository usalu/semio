"""🤲️ Independent Python oracle of the lending of a ramping grant to the elastic requests of its own run: the view of the world the decision reads (the holder is charged what its children are charged less, a lent request costs the host nothing and is capped at the whole jobs the holder can lend) and the decisions on that view, taken from the Python port of the decision in the upgrade oracle. Run `python3 🐍️.py` to regenerate `../🧫️fixtures/🔣️.json`, `python3 🐍️.py --check` to prove the stored vectors are what the oracle produces."""
import importlib.util,json,math,os,random,sys

HERE=os.path.dirname(os.path.abspath(__file__))
TARGET=os.path.join(HERE,"..","🧫️fixtures","🔣️.json")
spec=importlib.util.spec_from_file_location("upgrade_oracle",os.path.join(HERE,"..","..","🔼️upgrade","🐍️oracle","🐍️.py"))
upgrade=importlib.util.module_from_spec(spec);spec.loader.exec_module(upgrade)
KEEP=upgrade.PROTOCOL["lendKeepBasisPoints"]

def lend(policy,ledger,queue,now):
    def ramping(g):return g.get("quiet") is not True and now-g.get("resumedAtMs",g["admittedAtMs"])<g.get("rampMilliseconds",policy["memory"]["rampMilliseconds"])
    active=[g for g in ledger if "lentSinceMs" not in g]
    holders={g["id"]:g for g in active if "holder" not in g and g.get("quiet") is not True and ramping(g)}
    kids={}
    for g in active:
        if g.get("holder") in holders:kids.setdefault(g["holder"],[]).append(g)
    named=[w for w in queue if w["request"].get("holder") in holders and w["request"].get("elastic") is True and w.get("upgrade") is not True and w.get("grow") is not True]
    if not kids and not named:return ledger,queue,[],{}
    room={};charged={}
    for hid in set(kids)|{w["request"]["holder"] for w in named}:
        h=holders[hid];ks=kids.get(hid,[]);held=sum(k["reservedMegabytes"] for k in ks);seen=sum(k["observedMegabytes"] or 0 for k in ks)
        own=0 if h["observedMegabytes"] is None else max(0,h["observedMegabytes"]-seen)
        kept=max(own,h["reservedMegabytes"]*KEEP//10000);unrealized=max(0,h["reservedMegabytes"]-(h["observedMegabytes"] or 0))
        room[hid]={"kept":kept,"remaining":max(0,min(h["reservedMegabytes"]-kept-held,unrealized-held))};charged[hid]=held
    lent={};priced={}
    for w in sorted(named,key=lambda w:(w["arrivedAtMs"],w["request"]["id"])):
        r=w["request"];state=room[r["holder"]]
        fit=[j for j in range(r["preferredJobs"],r["minimumJobs"]-1,-1) if r["fixedMegabytes"]+j*r["perJobMegabytes"]<=state["remaining"]]
        if not fit:continue
        jobs=fit[0];state["remaining"]-=r["fixedMegabytes"]+jobs*r["perJobMegabytes"]
        lent[r["id"]]={"holder":r["holder"],"fixedMegabytes":r["fixedMegabytes"],"perJobMegabytes":r["perJobMegabytes"],"keptMegabytes":state["kept"]}
        priced[r["id"]]={**w,"request":{**r,"preferredJobs":jobs,"fixedMegabytes":0,"perJobMegabytes":0}}
    view_ledger=[{**g,"reservedMegabytes":max(0,g["reservedMegabytes"]-charged[g["id"]])} if g["id"] in charged else g for g in ledger]
    return view_ledger,[w for w in queue if w["request"]["id"] not in priced],[priced[w["request"]["id"]] for w in queue if w["request"]["id"] in priced],lent

def evaluate(policy,host,ledger,queue,now):
    view_ledger,rest,lent_waiters,lent=lend(policy,ledger,queue,now)
    if not lent_waiters:
        return {"view":{"ledger":view_ledger,"waiters":rest,"lentWaiters":[],"lent":{}},"expect":upgrade.evaluate(policy,host,view_ledger,rest,now)}
    free={**host,"availableMegabytes":host["totalMegabytes"]}
    unpriced=[{**g,"reservedMegabytes":0,"observedMegabytes":0} for g in view_ledger]
    first=upgrade.evaluate(policy,free,unpriced,lent_waiters,now)["decisions"]
    decisions=[];taken={};started=[]
    for d in first:
        worth=lent[d["id"]]
        if d["verdict"]=="admit":
            reserved=worth["fixedMegabytes"]+d["jobs"]*worth["perJobMegabytes"];d={**d,"reservedMegabytes":reserved,"reason":"lent"}
            w=next(x for x in queue if x["request"]["id"]==d["id"]);taken[worth["holder"]]=taken.get(worth["holder"],0)+reserved
            row={"id":d["id"],"operation":w["request"]["operation"],"jobs":d["jobs"],"cpu":d["jobs"]*w["request"]["cpuPerJob"],"reservedMegabytes":reserved,"admittedAtMs":now,"observedMegabytes":None,"pid":w["pid"],"bootAtMs":0,"holder":worth["holder"]}
            if w.get("session") is not None:row["session"]=w["session"]
            started.append(row)
        decisions.append(d)
    second_ledger=[{**g,"reservedMegabytes":max(0,g["reservedMegabytes"]-taken[g["id"]])} if g["id"] in taken else g for g in view_ledger]+started
    result=upgrade.evaluate(policy,host,second_ledger,rest,now)
    return {"view":{"ledger":view_ledger,"waiters":rest,"lentWaiters":lent_waiters,"lent":lent},"expect":{**result,"decisions":decisions+result["decisions"]}}

NOW=1_800_000_000_000
def host(total,available,**over):
    return {"version":1,"atMs":NOW,"platform":"darwin","cores":10,"totalMegabytes":total,"availableMegabytes":available,"pressure":"normal","swapUsedMegabytes":0,"swapOutMegabytesPerSecond":0,"load":2,"openFiles":8000,"openFilesMaximum":245760,"attachedImages":2,**over}
def holder(reserved,observed,**over):
    return {"id":"showholder1","operation":"show","jobs":1,"cpu":1,"reservedMegabytes":reserved,"admittedAtMs":NOW-10000,"observedMegabytes":observed,"pid":4242,"bootAtMs":1_700_000_000_000,"label":"projects","rampMilliseconds":300000,**over}
def child(identifier,reserved,observed,holder_id="showholder1",**over):
    return {"id":identifier,"operation":"exec","jobs":reserved//1536,"cpu":reserved//1536,"reservedMegabytes":reserved,"admittedAtMs":NOW-5000,"observedMegabytes":observed,"pid":4243,"bootAtMs":1_700_000_000_000,"holder":holder_id,**over}
def workers(identifier="farmreq0001",preferred=8,holder_id="showholder1",arrived=NOW-1000,**over):
    return {"request":{"id":identifier,"operation":"exec","priority":"batch","minimumJobs":1,"preferredJobs":preferred,"fixedMegabytes":0,"perJobMegabytes":1536,"cpuPerJob":1,"elastic":True,"label":"graph-farm","holder":holder_id,**over},"arrivedAtMs":arrived,"pid":4243}
def other(identifier,fixed,arrived=NOW-2000):
    return {"request":{"id":identifier,"operation":"cargo","priority":"batch","minimumJobs":1,"preferredJobs":1,"fixedMegabytes":fixed,"perJobMegabytes":0,"cpuPerJob":1},"arrivedAtMs":arrived,"pid":77}

def hand():
    sixteen=lambda available:host(16384,available)
    return [
     ("16 GB machine, show admitted with 6553 MB and the parent at 1100 MB: two workers inside its reservation",sixteen(5200),[holder(6553,1100)],[workers()]),
     ("16 GB machine, show admitted with 4096 MB, the parent keeps 1843 MB: one worker fits",sixteen(5200),[holder(4096,900)],[workers()]),
     ("16 GB machine, the two workers run: the third request finds 533 MB left and takes the ordinary path, which waits for memory",sixteen(3200),[holder(6553,2600),child("farmgrant01",3072,1500)],[workers("farmreq0002")]),
     ("16 GB machine, the parent holds 5000 MB: one worker still fits into the 1553 MB left",sixteen(5200),[holder(6553,5000)],[workers()]),
     ("16 GB machine, the parent holds 5500 MB: nothing is left to lend, the worker waits for memory like any request",sixteen(5200),[holder(6553,5500)],[workers()]),
     ("32 GB machine of the incident, 22 380 MB reserved for the rebuild: all eight workers inside, host room is negative",host(32768,8000),[holder(22380,1000)],[workers()]),
     ("the holder is past its ramp: it is charged by what it holds and the workers take the ordinary path",host(32768,8000),[holder(22380,1000,admittedAtMs=NOW-400000)],[workers()]),
     ("the holder only waits in a queue (lent out): nothing is lent",host(32768,8000),[holder(22380,1000,lentSinceMs=NOW-3000)],[workers()]),
     ("the named holder is not in the ledger: ordinary path",host(32768,8000),[holder(22380,1000,id="otherholder")],[workers()]),
     ("memory pressure is critical: the hard stop refuses lent workers too",host(32768,8000,pressure="critical"),[holder(22380,1000)],[workers()]),
     ("a quiet grant holds nothing to lend",host(32768,8000),[holder(2048,100,quiet=True,arrivedAtMs=NOW-10000)],[workers()]),
     ("a child cannot be a holder",host(32768,8000),[holder(22380,1000),child("farmgrant01",3072,0)],[workers("farmreq0002",holder_id="farmgrant01")]),
     ("two requests of one holder: the older takes what fits, the younger finds less",host(32768,8000),[holder(6553,1100)],[workers("farmreq0001",arrived=NOW-3000),workers("farmreq0002",arrived=NOW-1000)]),
     ("a request that is not elastic is never lent to",host(32768,8000),[holder(22380,1000)],[workers(elastic=False)]),
     ("a request below its minimum jobs takes the ordinary path",host(32768,8000),[holder(2048,900)],[workers(minimumJobs=2)]),
     ("a larger waiter ahead that cannot fit does not hold back workers that run inside the reservation",host(32768,8000),[holder(22380,1000)],[other("cargowaiter",30000),workers()]),
    ]

def random_world(rng,policy,index):
    total=rng.choice([16384,32768]);h=host(total,rng.randrange(1500,total-1500),pressure=rng.choice(["normal"]*9+["warn"]),load=rng.choice([None,1,3]))
    reserved=rng.choice([2048,4096,6553,12000,22380]);observed=rng.choice([None,0,reserved//10,reserved//3,reserved//2,reserved])
    hg=holder(reserved,observed,admittedAtMs=NOW-rng.choice([1000,10000,200000,400000]),rampMilliseconds=rng.choice([30000,300000]))
    if rng.random()<0.1:hg["lentSinceMs"]=NOW-2000
    ledger=[hg]
    for i in range(rng.choice([0,0,1,2])):
        r=rng.choice([1536,3072,4608]);ledger.append(child(f"farmgrant{i:02d}",r,rng.choice([None,0,r//2,r]),admittedAtMs=NOW-rng.choice([1000,50000])))
    for i in range(rng.choice([0,1,2])):ledger.append(upgrade.random_grant(rng,NOW))
    for i,g in enumerate(ledger[1:]):
        if "holder" not in g:g["id"]=f"plainrun{i:03d}"
    queue=[]
    for i in range(rng.choice([1,1,2,3])):
        queue.append(workers(f"farmreq{i:04d}",preferred=rng.choice([1,2,4,8]),arrived=NOW-rng.choice([0,500,5000,31000]),minimumJobs=rng.choice([1,1,2]),elastic=rng.random()<0.9,**({"perJobMegabytes":rng.choice([768,1536,2048])})))
    if rng.random()<0.3:queue.append(other("cargowaiter",rng.choice([500,2000,9000]),arrived=NOW-rng.choice([0,3000,40000])))
    rng.shuffle(queue)
    return f"random-{index}",h,ledger,queue

def generate():
    fixture=json.load(open(upgrade.DECISION,encoding="utf8"));policy={**fixture["policy"],"memory":{**fixture["policy"]["memory"],"maximumQuiet":4}}
    worlds=[]
    for name,h,ledger,queue in hand():worlds.append({"name":name,"host":h,"ledger":ledger,"queue":queue,"nowMs":NOW,**evaluate(policy,h,ledger,queue,NOW)})
    rng=random.Random(20261114)
    for index in range(240):
        name,h,ledger,queue=random_world(rng,policy,index)
        worlds.append({"name":name,"host":h,"ledger":ledger,"queue":queue,"nowMs":NOW,**evaluate(policy,h,ledger,queue,NOW)})
    lentCount=sum(1 for w in worlds if any(d["reason"]=="lent" for d in w["expect"]["decisions"]))
    return {"policy":{"maximumQuiet":4},"protocol":{"lendKeepBasisPoints":KEEP},"worlds":worlds,"coverage":{"lent":lentCount,"worlds":len(worlds),"withChildren":sum(1 for w in worlds if any("holder" in g for g in w["ledger"]))}}

def dump(value):return json.dumps(value,separators=(",",":"),ensure_ascii=False)+"\n"

if __name__=="__main__":
    value=generate();text=dump(value)
    if sys.argv[1:]==["--check"]:
        stored=open(TARGET,encoding="utf8").read()
        print("vectors equal the oracle" if stored==text else "STALE vectors");sys.exit(0 if stored==text else 1)
    with open(TARGET,"w",encoding="utf8") as handle:handle.write(text)
    print("wrote",len(value["worlds"]),"worlds",value["coverage"])
