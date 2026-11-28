"""📐️ Independent Python oracle of the memory scaling of the host capacity gate: the free-memory floor, the cap, the cold request sizes clamped to a share of the room above the floor, and the swap stop, on hosts of 8, 16, 32 and 64 GB, read from the authored policy `hostCapacity` section of the build library. `python3 🐍️.py` rewrites the vectors, `--check` fails when they are stale."""
import json,math,os,sys

HERE=os.path.dirname(os.path.abspath(__file__))
POLICY=os.path.join(HERE,*([".."]*7),"🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","🔣️policy.json")
TARGET=os.path.join(HERE,"..","🧫️fixtures","🔣️.json")
TOTALS=[8192,16384,32768,65536]
OPERATIONS=[("run",["check"],[]),("run",["test"],["test-e2e-fleet"]),("run",["build"],[]),("show",[],[]),("graph",[],[]),("dependencies",[],[]),("command",[],[]),("tool-query",[],[])]

def floor_mb(policy,total):
    m=policy["memory"];share=total*m["floorBasisPoints"]//10000
    return min(total,max(m["floorMinimumMegabytes"],min(m["floorMaximumMegabytes"],share)))

def reach(policy,operation,total):
    room=max(0,total-floor_mb(policy,total))
    return max(operation.get("minimumMegabytes",0),room*policy["estimates"]["maximumShareBasisPoints"]//10000)

def size(policy,name,kinds,targets,total):
    import re
    operation=next(row for row in policy["operations"] if row["name"]==name)
    limit=reach(policy,operation,total);fixed=min(operation["fixedMegabytes"],limit)
    if operation["shape"]=="fixed":return {"fixedMegabytes":fixed,"perJobMegabytes":0,"envelopePerJobMegabytes":None}
    rows=[policy["kinds"][kind] for kind in ([operation["kind"]] if "kind" in operation else kinds)+["build"]]
    rows+= [row for row in policy["envelopes"] if any(re.search(row["pattern"],target) for target in targets)]
    declared=max(row["megabytes"] for row in rows);per_job=min(declared,max(policy["estimates"]["minimumMegabytes"],limit-fixed))
    return {"fixedMegabytes":fixed,"perJobMegabytes":per_job,"envelopePerJobMegabytes":declared if per_job<declared else None}

def swap_stop(policy,host):
    rules=policy["swap"];total=host["totalMegabytes"];rate=host["swapOutMegabytesPerSecond"]
    if rate is None:return None
    swapping=rate>=total*rules["activeBasisPointsPerSecond"]/10000
    short=host["availableMegabytes"]<total*rules["shortBasisPoints"]/10000
    crowded=host["swapUsedMegabytes"] is not None and host["swapUsedMegabytes"]>total*rules["ceilingBasisPoints"]/10000
    if crowded and swapping and short:return "swap"
    if rate>total*rules["outBasisPointsPerSecond"]/10000:return "swap-activity"
    return None

def swap_hosts():
    rows=[]
    for total in TOTALS:
        for name,available,used,rate in [("idle with old swap",0.55,0.9,0.0),("old swap, light traffic, memory plentiful",0.55,0.9,total*0.00008),("old swap, heavy traffic, memory plentiful",0.55,0.9,total*0.0005),("worked swap, memory short",0.10,0.6,total*0.0012),("worked swap, little occupied",0.10,0.1,total*0.0012),("saturated traffic",0.80,0.0,total*0.0045),("no counters",0.10,None,None)]:
            rows.append({"name":f"{total} {name}","host":{"totalMegabytes":total,"availableMegabytes":int(total*available),"swapUsedMegabytes":None if used is None else int(total*used),"swapOutMegabytesPerSecond":None if rate is None else round(rate,2)}})
    return rows

def generate():
    policy=json.load(open(POLICY,encoding="utf8"))["hostCapacity"]
    floors=[{"totalMegabytes":total,"floorMegabytes":floor_mb(policy,total),"capMegabytes":total-floor_mb(policy,total)} for total in TOTALS]
    sizes=[{"totalMegabytes":total,"operation":name,"kinds":kinds,"targets":targets,"expect":size(policy,name,kinds,targets,total)} for total in TOTALS for name,kinds,targets in OPERATIONS]
    swaps=[{**row,"expect":swap_stop(policy,row["host"])} for row in swap_hosts()]
    return {"floors":floors,"sizes":sizes,"swaps":swaps}

def dump(value):return json.dumps(value,separators=(",",":"),ensure_ascii=False)+"\n"

if __name__=="__main__":
    text=dump(generate())
    if sys.argv[1:]==["--check"]:
        stored=open(TARGET,encoding="utf8").read()
        print("vectors equal the oracle" if stored==text else "STALE vectors");sys.exit(0 if stored==text else 1)
    open(TARGET,"w",encoding="utf8").write(text);print("wrote",len(text),"bytes")
