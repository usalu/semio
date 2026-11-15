#!/usr/bin/env python3
"""🐍️ Independent oracle of the shared worktree trust rules. It shares no code with `../🟦️.ts`: it reads the same language-agnostic fixture and decides every case again (scope of a path, class of a failure key, tier of a health file, statement of the tracker after a sequence of steps).
usage: 🐍️.py            exit 0 when every vector agrees, 1 on any mismatch
Python 3.9 compatible."""
import json,os,re,sys
HERE=os.path.dirname(os.path.abspath(__file__))
FX=json.load(open(os.path.join(HERE,"🧫️fixtures","🔣️.json"),encoding="utf8"))
TICKETS=".🧬semio/🦑️repo/🎫️tickets/"
HEALTH_MAX=30000
VERIFIED_MAX=600000
bad=[]

def in_scope(path):
    name=path.split(":")[0]
    base=name.rsplit("/",1)[-1]
    if name==".git" or name.startswith(".git/") or name.startswith(".cache/") or name.startswith(TICKETS) or base.startswith("._"): return False
    return True

def classify(key):
    head,sep,rest=key.partition(":")
    if head.startswith("git-broken-") or head in ("attest","skipped","watch","seal","entry-boot","skew","scan","compaction","unsupported"): return "neutral"
    scoped=lambda: "manifest" if in_scope(rest) else "neutral"
    if head=="blocked":
        if rest.startswith("journal:"): return "manifest"
        if rest.startswith("objects:"): return "neutral"
        return scoped()
    if head in ("blocked-long","blob-wanted","entry-guard","refused","unstable"): return scoped()
    if head=="error":
        if rest.startswith("git:"): return "neutral"
        if rest.startswith("clash:"): return "manifest"
        first=rest.split(":")[0]
        if "/" not in first and re.match(r"^[a-z0-9][a-z0-9-]{0,39}$",first): return "neutral"
        return scoped()
    return "manifest"

def tier(health,now,alive):
    if not (abs(now-health["wallMs"])<=HEALTH_MAX and health["store"]=="ok" and alive): return "unusable"
    t=health.get("trust")
    if t is None or t["verifiedMs"] is None: return "verify"
    if t["lostMs"] is not None and t["verifiedMs"]<=t["lostMs"]: return "verify"
    if now-t["verifiedMs"]>VERIFIED_MAX: return "verify"
    return "trusted"

def kind_of(key):
    head=key.partition(":")[0].lower()
    head=re.sub(r"[^a-z0-9-]","-",head)
    head=re.sub(r"^[^a-z]+","",head)
    return (head or "failure")[:40]

def run(tracker):
    clean={"file":None,"directory":None,"walk":None}
    lost=[None,None]
    for i,s in enumerate(tracker["steps"]):
        at=s["at"]
        if s["do"]=="failure":
            if classify(s["key"])=="manifest": lost=[at,kind_of(s["key"])]
        elif s["do"]=="lost": lost=[at,s["kind"]]
        elif s["do"]=="verified":
            old=clean[s["part"]]
            if old is None or s["startedMs"]>old: clean[s["part"]]=s["startedMs"]
        else:
            f,d,w=clean["file"],clean["directory"],clean["walk"]
            both=None if f is None or d is None else min(f,d)
            v=w if both is None else both if w is None else max(both,w)
            rec={"scope":"shared-source","verifiedMs":v,"lostMs":lost[0],"lostKind":lost[1]}
            if rec!=s["trust"]: bad.append("%s #%d record %r != %r"%(tracker["name"],i,rec,s["trust"]))
            got=tier({"wallMs":at,"pid":1,"store":"ok","trust":rec},at,True)
            if got!=s["tier"]: bad.append("%s #%d tier %s != %s"%(tracker["name"],i,got,s["tier"]))

for item in FX["scopes"]:
    if in_scope(item["path"])!=item["inScope"]: bad.append("scope "+item["path"])
for item in FX["failures"]:
    if classify(item["key"])!=item["class"]: bad.append("failure "+item["name"])
for item in FX["tiers"]:
    got=tier(item["health"],item["now"],item["alive"])
    if got!=item["expected"]: bad.append("tier %s: %s != %s"%(item["name"],got,item["expected"]))
for item in FX["trackers"]: run(item)
for line in bad: print(line)
sys.exit(1 if bad else 0)
