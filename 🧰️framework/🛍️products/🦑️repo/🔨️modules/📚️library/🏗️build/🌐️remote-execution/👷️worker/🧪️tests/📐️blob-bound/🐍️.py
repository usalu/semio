"""📐️ Independent Python oracle of the worker's blob cache bound: the cache keeps at most the maximum, the work volume keeps the room every action slot needs (reserve plus one action per slot, the agent's disk rule), the host volume keeps the content store's hard reserve of the worker role (CAS7: the largest of reservePercent of the partition, reserveFloorBytes bounded by reserveFloorMaxPercent of it and the machine budget's host free floor, at most half of it); a volume that cannot be measured does not bound the cache. `python3 🐍️.py` rewrites the vectors, `--check` fails when they are stale."""
import json,os,sys

HERE=os.path.dirname(os.path.abspath(__file__))
BUILD=os.path.join(HERE,"..","..","..","..")
DISK=json.load(open(os.path.join(BUILD,"🔣️policy.json")))["remoteExecution"]["worker"]["disk"]
SIZING=json.load(open(os.path.join(BUILD,"🗄️cas","🧬️schema","🔣️.json")))["definitions"]["Policy"]["const"]["sizing"]
ROLE=[row for row in SIZING["roles"] if row["id"]=="worker"][0]
MACHINE=json.load(open(os.path.join(BUILD,"..","⚡️caching","💾️storage","🧾️ledger","🎚️machine","🔣️policy.json")))
TARGET=os.path.join(HERE,"🔣️.json")
GIB=2**30

def keep_work(concurrency):
    return -(-int((DISK["reserveGigabytes"]+max(1,concurrency)*DISK["perActionGigabytes"])*GIB)//1)

def host_floor(total):
    return min(MACHINE["hostFreeFloorBytes"],max(MACHINE["alignBytes"],(total*MACHINE["hostFreeFloorPercent"]//100//MACHINE["alignBytes"])*MACHINE["alignBytes"]))

def host_reserve(total):
    return min(total//2,max(total*ROLE["reservePercent"]//100,min(ROLE["reserveFloorBytes"],total*SIZING["reserveFloorMaxPercent"]//100),host_floor(total)))

def bound(maximum,cache,work_free,keep,host_free,reserve):
    limits=[maximum]
    if work_free is not None:limits.append(cache+work_free-keep)
    if host_free is not None:limits.append(cache+host_free-reserve)
    return max(0,min(limits))

def build():
    cases=[]
    for cache in (0,4*GIB,10*GIB):
        for work_free in (None,1*GIB,6*GIB,20*GIB):
            for host_free,host_total in ((None,None),(9*GIB,233*GIB),(40*GIB,233*GIB),(200*GIB,926*GIB)):
                keep=keep_work(10);reserve=0 if host_total is None else host_reserve(host_total)
                row={"maximumBytes":12*GIB,"cacheBytes":cache,"workFreeBytes":work_free,"keepWorkBytes":keep,"hostFreeBytes":host_free,"hostReserveBytes":reserve}
                cases.append({"input":{key:value for key,value in row.items() if value is not None},"bound":bound(12*GIB,cache,work_free,keep,host_free,reserve)})
    return {"cases":cases}

if __name__=="__main__":
    text=json.dumps(build(),indent=1)+"\n"
    if "--check" in sys.argv:
        if open(TARGET).read()!=text:sys.exit("stale vectors")
    else:open(TARGET,"w").write(text)
