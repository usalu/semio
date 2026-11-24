"""🐍️ Independent Python oracle of the single memory charge of a running remote action: its declared peak minus what is measured of it (nothing measured: all of it until a load report arrived the ramp after its start), the process-group resident megabytes of a `ps` table, and the decimal text of the peak entry. Prints JSON."""
import json,math,random

def charge(declared,measured,started,report_at,ramp):
    if declared<=0:return 0
    if measured is not None:return max(0,declared-measured)
    return 0 if report_at is not None and report_at>=started+ramp else declared

def groups(table,wanted):
    kb={}
    for line in table.split("\n"):
        parts=line.split()
        if len(parts)==2 and all(p.isdigit() for p in parts) and int(parts[0]) in wanted:kb[int(parts[0])]=kb.get(int(parts[0]),0)+int(parts[1])
    return {str(g):math.ceil(v/1024) for g,v in kb.items()}

rand=random.Random(1122)
lifetime=[]
for declared in (0,1024,4096,7938):
    for measured in (None,0,1,512,1024,4000,4096,9000):
        lifetime.append({"declared":declared,"measured":measured,"charge":charge(declared,measured,1000,None,120000)})
ramps=[]
for _ in range(400):
    declared=rand.choice([0,512,2048,7938]);started=rand.randrange(0,500000);ramp=rand.choice([0,60000,120000,600000])
    report=None if rand.random()<0.2 else started+rand.randrange(-200000,700000)
    measured=None if rand.random()<0.5 else rand.randrange(0,9000)
    ramps.append({"declared":declared,"measured":measured,"startedAtMs":started,"reportAtMs":report,"rampMs":ramp,"charge":charge(declared,measured,started,report,ramp)})
tables=[]
for _ in range(60):
    rows=[(rand.choice([1,100,200,300,4242]),rand.randrange(0,9_000_000)) for _ in range(rand.randrange(0,40))]
    table="\n".join(f"{' '*rand.randrange(0,6)}{g} {' '*rand.randrange(1,6)}{k}" for g,k in rows)+rand.choice(["","\n","\nnot a row\n  x y\n"])
    wanted=sorted(rand.sample([1,100,200,300,4242],rand.randrange(1,4)))
    tables.append({"table":table,"groups":wanted,"megabytes":groups(table,set(wanted))})
peaks=[{"megabytes":m,"text":str(max(1,math.ceil(m)))} for m in (0.2,1,17,6550.4,7938)]
print(json.dumps({"lifetime":lifetime,"ramps":ramps,"tables":tables,"peaks":peaks}))
