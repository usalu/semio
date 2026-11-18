import json,sys,re
request=json.load(sys.stdin)
stall=request["stall"];grace=request["grace"]
PHASE=re.compile(r"^[a-z]{1,40}$")
def idle(claim,now):
    p=claim.get("progress")
    return None if p is None else max(0,now-p["advancedMs"])
def reason(claim,now,current):
    if claim["state"]!="held" or claim.get("ask") is not None:return None
    holder=claim.get("holder");digest=holder["digest"] if holder else ""
    if current and digest and digest!=current:return "stale-code"
    i=idle(claim,now)
    return "stalled" if i is not None and i>=stall else None
def deposed(claim,now):
    ask=claim.get("ask")
    return ask is not None and now-ask["atMs"]>=grace
def advance(previous,event,now):
    phase=event["phase"] if isinstance(event.get("phase"),str) and PHASE.match(event["phase"]) else (previous["phase"] if previous else "start")
    done=event["done"] if isinstance(event.get("done"),int) and event["done"]>=0 else (previous["done"] if previous else 0)
    total=event["total"] if isinstance(event.get("total"),int) and event["total"]>=0 else (previous["total"] if previous else 0)
    moved=previous is None or phase!=previous["phase"] or done>previous["done"]
    return {"phase":phase,"done":done,"total":total,"advancedMs":now if moved else previous["advancedMs"]}
out={"decisions":[{"reason":reason(c["claim"],c["now"],c.get("current")),"idleMs":idle(c["claim"],c["now"]),"deposed":deposed(c["claim"],c["now"])} for c in request["cases"]],
     "advances":[advance(a.get("previous"),a["event"],a["now"]) for a in request["advances"]]}
print(json.dumps(out))
