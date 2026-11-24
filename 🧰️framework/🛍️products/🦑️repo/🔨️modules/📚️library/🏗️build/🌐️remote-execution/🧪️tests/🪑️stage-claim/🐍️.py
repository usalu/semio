"""🐍️ Independent Python oracle of the readiness stage claim: the entitlement of a stage on hosts of 8, 16, 32 and 64 GB (learned from completed runs only, a share of the machine otherwise, bounded), the hold (what the stage may still grow into), the memory the host view leaves the others, and how many children the owner pool then runs on a 16 GB machine. Reads the policy and the pool policy from their schemas. `python3 🐍️.py` rewrites the vectors, `--check` fails when they are stale."""
import json,math,os,sys

HERE=os.path.dirname(os.path.abspath(__file__))
LOAD=os.path.join(HERE,"..","..","🌡️load","🪑️stage-claim","🧬️schema","🔣️.json")
POOL=os.path.join(HERE,"..","..","..","🕸️graph","📥️inference","📦️project-nodes","🌐️owner-pool","🧬️schema","🔣️.json")
TARGET=os.path.join(HERE,"🔣️.json")
TOTALS=[8192,16384,32768,65536]
OK=lambda peak:{"stage":"workspace","outcome":"ok","peakMegabytes":peak}
RUNS=[
 ("none",[]),
 ("one-ok",[OK(4514)]),
 ("failed-only",[{"stage":"workspace","outcome":"stage-failed","peakMegabytes":9000},{"stage":"workspace","outcome":"stalled","peakMegabytes":7000}]),
 ("other-stage",[{"stage":"tools","outcome":"ok","peakMegabytes":5000}]),
 ("tiny",[OK(300)]),
 ("huge",[OK(40000)]),
 ("twenty",[OK(1000+100*index) for index in range(20)]),
]

def policy():return json.load(open(LOAD))["definitions"]["Policy"]["const"]
def pool():return json.load(open(POOL))["definitions"]["Policy"]["const"]

def entitlement(total,runs,rules):
    ceiling=total*rules["maximumShareBasisPoints"]//10000
    ok=[row["peakMegabytes"] for row in runs if row["stage"]=="workspace" and row["outcome"]=="ok" and row["peakMegabytes"]]
    ok=ok[-rules["samples"]:]
    ok.sort()
    wanted=total*rules["shareBasisPoints"]//10000 if not ok else math.ceil(ok[max(0,math.ceil(len(ok)*rules["percentile"]/100)-1)]*(100+rules["marginPercent"])/100)
    return min(ceiling,max(rules["minimumMegabytes"],wanted))

def reserve(claim,rules):
    if claim["waiting"]:return 0
    return min(claim["entitlementMegabytes"],max(rules["floorMegabytes"],math.ceil(claim["observedMegabytes"]*rules["reservePercent"]/100)))

def hold(claim,now,rules):
    if now-claim["atMs"]>rules["freshMilliseconds"]:return 0
    return max(0,reserve(claim,rules)-claim["observedMegabytes"])

def children(total,available,observed,entitlement_megabytes,rules,limits,need):
    claim={"entitlementMegabytes":entitlement_megabytes,"observedMegabytes":observed,"waiting":False,"atMs":0}
    left=max(0,available-hold(claim,0,rules))
    floor=min(limits["floorMaximumMegabytes"],max(limits["floorMinimumMegabytes"],total*limits["floorShareBasisPoints"]//10000))
    cores=10;minimum=max(1,math.ceil(cores*limits["minimumShareBasisPoints"]/10000))
    by_cores=max(max(0,cores-1),minimum);by_load=max(minimum,math.floor(cores*limits["loadCeilingBasisPointsPerCore"]/10000))
    spare=left-floor
    by_memory=0+math.floor(spare/max(1,need))
    return {"leftMegabytes":left,"children":min(by_cores,by_load,by_memory,limits["maximumChildren"])}

def build():
    rules=policy();limits=pool()
    entitlements=[{"totalMegabytes":total,"case":name,"entitlementMegabytes":entitlement(total,runs,rules)} for total in TOTALS for name,runs in RUNS]
    holds=[{"entitlementMegabytes":4915,"observedMegabytes":observed,"waiting":waiting,"ageMilliseconds":age,"holdMegabytes":hold({"entitlementMegabytes":4915,"observedMegabytes":observed,"waiting":waiting,"atMs":0},age,rules)} for observed in (0,100,1200,3900,4915,9000) for waiting in (False,True) for age in (0,119999,120001)]
    pools=[]
    for available in (6000,8000,10000,12000):
        for observed in (1200,4915):
            for need in (800,1393):
                row=children(16384,available,observed,4915,rules,limits,need)
                pools.append({"totalMegabytes":16384,"availableMegabytes":available,"observedMegabytes":observed,"entitlementMegabytes":4915,"childMegabytes":need,**row,"childrenGrown":children(16384,available,4915,4915,rules,limits,need)["children"]})
    return {"runs":{name:runs for name,runs in RUNS},"entitlements":entitlements,"holds":holds,"pools":pools}

if __name__=="__main__":
    text=json.dumps(build(),ensure_ascii=False,indent=1)+"\n"
    if "--check" in sys.argv:
        if open(TARGET).read()!=text:sys.exit("stale vectors")
    else:open(TARGET,"w").write(text)
