"""Independent oracle of the shrinking admission: one request on an idle host, no ledger."""
import json,sys
def floor_of(total,p):
    share=total*p["floorBasisPoints"]//10000
    return min(total,max(p["floorMinimumMegabytes"],min(p["floorMaximumMegabytes"],share)))
def decide(case,p):
    total,avail=case["total"],case["available"]
    floor=floor_of(total,p);cap=max(0,total-floor)
    limit=cap*p["budgetBasisPoints"]//10000
    need=min(case["fixed"],cap);minimum=case.get("minimum")
    room=avail-floor
    if minimum is not None and minimum>limit: return ["wait",0,"never"]
    if room>=need: return ["admit",need,"fits"]
    if minimum is not None and minimum<need and room>=minimum: return ["admit",room,"shrunk"]
    if minimum is None and need<case["fixed"] and case["waited"]>=p["starvedMilliseconds"]: return ["wait",0,"starved"]
    return ["wait",0,"memory"]
if __name__=="__main__":
    data=json.load(sys.stdin)
    print(json.dumps([decide(c,data["policy"]) for c in data["cases"]]))
