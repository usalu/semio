"""🧯️ Independent Python oracle of the progress guarantees of the host capacity gate on hosts of 8, 16 and 32 GB, read from the authored policy `hostCapacity` of the build library: the most one learned peak may count (host share and class ceiling), which finished runs may teach, the size a learned 200 % request is scaled to, and what admission does with it for every room the host has: admit it whole, shrink it to the room when the room covers the declared minimum, wait, and refuse past the starved bound. `python3 🐍️.py` rewrites the vectors, `python3 🐍️.py --check` fails when they are stale."""
import json,os,re,sys

HERE=os.path.dirname(os.path.abspath(__file__))
POLICY=os.path.join(HERE,*([".."]*7),"🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","🔣️policy.json")
TARGET=os.path.join(HERE,"..","🧫️fixtures","🔣️.json")
TOTALS=[8192,16384,32768]

def floor_mb(policy,total):
    m=policy["memory"];share=total*m["floorBasisPoints"]//10000
    return min(total,max(m["floorMinimumMegabytes"],min(m["floorMaximumMegabytes"],share)))

def learn_ceiling(policy,capacity,declared):
    share=capacity*policy["estimates"]["maximumShareBasisPoints"]//10000
    return share if declared is None else min(share,declared)

def learnable(status,limited,pressured):
    return (not pressured) and (status=="ok" or limited)

def sample_ratio(peak,ceiling,default):
    return max(1,int(min(peak,ceiling)*10000/default+0.5))

def learned_command(policy,capacity):
    operation=next(row for row in policy["operations"] if row["name"]=="command")
    wanted=operation["fixedMegabytes"]*policy["estimates"]["maximumRatioBasisPoints"]//10000
    return min(wanted,capacity*policy["estimates"]["maximumShareBasisPoints"]//10000),operation["minimumMegabytes"]

def admission(policy,total,holder,available,waited,hard):
    floor=floor_mb(policy,total);capacity=total-floor
    fixed,minimum=learned_command(policy,capacity)
    room=min(available-floor,capacity-holder)
    starved=policy["wait"]["starvedMilliseconds"]
    if hard:
        if waited>=policy["wait"]["deadlineBatchMilliseconds"] and room>=minimum:return ["admit",min(fixed,room),"shrunk"]
        return ["wait",0,"starved" if waited>=starved else "pressure-critical"]
    if fixed<=room:return ["admit",fixed,"fits"]
    if room>=minimum:return ["admit",min(fixed,room),"shrunk"]
    return ["wait",0,"starved" if waited>=starved else "memory"]

def phase_size(policy,peak):
    e=policy["estimates"];ratio=max(1,int(peak*10000/e["phaseReferenceMegabytes"]+0.5));learned=int(ratio*(10000+e["marginBasisPoints"])*e["phaseReferenceMegabytes"]/10000/10000+0.5)
    return max(e["minimumMegabytes"],min(e["maximumPhaseMegabytes"],learned))

def class_ceiling(policy,key):
    rows=[row["megabytes"] for row in policy["classCeilings"] if re.search(row["pattern"],key)]
    return min(rows) if rows else None

def generate():
    policy=json.load(open(POLICY,encoding="utf8"))["hostCapacity"]
    ceilings=[];admissions=[];learns=[]
    for total in TOTALS:
        floor=floor_mb(policy,total);capacity=total-floor
        for declared in (None,4096,12288):
            ceilings.append({"totalMegabytes":total,"capacityMegabytes":capacity,"declared":declared,"expect":learn_ceiling(policy,capacity,declared)})
        for peak in (6163,20000):
            ceilings.append({"totalMegabytes":total,"capacityMegabytes":capacity,"declared":None,"peak":peak,"default":2048,"expectRatio":sample_ratio(peak,learn_ceiling(policy,capacity,None),2048)})
        holder=3000
        for spare in (6000,2500,1600,1100,900,300):
            for waited in (0,policy["wait"]["starvedMilliseconds"]):
                for hard in (False,True):
                    available=floor+spare
                    admissions.append({"totalMegabytes":total,"holder":holder,"available":available,"waited":waited,"hard":hard,"expect":admission(policy,total,holder,available,waited,hard)})
    for status in ("ok","failed","cancelled"):
        for limited in (False,True):
            for pressured in (False,True):
                learns.append({"status":status,"limited":limited,"pressured":pressured,"expect":learnable(status,limited,pressured)})
    phases=[{"peak":peak,"expect":phase_size(policy,peak)} for peak in (100,400,1094,14243)]
    classes=[{"key":key,"expect":class_ceiling(policy,key)} for key in ("run:dry","run:repo:test-cache-reset:dry","run:build","show","graph:dry","run:build:@teaching/architecture-course:build")]
    return {"ceilings":ceilings,"admissions":admissions,"learns":learns,"phases":phases,"classes":classes}

def dump(value):return json.dumps(value,separators=(",",":"),ensure_ascii=False)+"\n"

if __name__=="__main__":
    text=dump(generate())
    if sys.argv[1:]==["--check"]:
        stored=open(TARGET,encoding="utf8").read()
        print("vectors equal the oracle" if stored==text else "STALE vectors");sys.exit(0 if stored==text else 1)
    open(TARGET,"w",encoding="utf8").write(text);print("wrote",len(text),"bytes")
