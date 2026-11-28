"""🪜️ Independent oracle of the image growth plan and the lane sizing, written from the schema's rules and not from the code: the caps and alignment of the platforms, the keep of the host, the grow threshold and the percentile are repeated here as literals."""
import json,sys
GIB=1<<30
CAPS={"darwin":32*GIB,"linux":128*GIB,"win32":64*GIB}
MODES={"darwin":"detached","linux":"online","win32":"detached"}
GROW_AT=85;KEEP=3*GIB;KEEP_MAX=8*GIB;INITIAL=64*(1<<20)
FLOOR=4*GIB;PERCENTILE=9500;HEADROOM=50
def down(v):return v//GIB*GIB
def want(rule,need):return max(GIB,-(-(need*(100+rule["headroomPercent"])//100)//GIB)*GIB)
def plan(platform,rule,f):
    mode=MODES.get(platform,"unsupported")
    def out(action,reason,target=None):
        t=f["capacityBytes"] if target is None else target
        return {"action":action,"mode":mode,"reason":reason,"targetBytes":t,"deltaBytes":t-f["capacityBytes"]}
    if mode=="unsupported":return out("refused","unsupported")
    cap=down(min(CAPS[platform],rule.get("maximumImageBytes",1<<62)))
    if f["capacityBytes"]>=cap:return out("none","at-cap")
    peak=max(f["usedBytes"],f.get("measuredBytes",0))
    if peak*100<f["capacityBytes"]*GROW_AT:return out("none","below-threshold")
    wanted=min(cap,want(rule,peak))
    if wanted<=f["capacityBytes"]:return out("none","enough")
    keep=min(KEEP_MAX,max(KEEP,f.get("hostKeepBytes",0)))
    share=f["hostFreeBytes"]*rule["hostFreeReservePercent"]//100
    left=f["hostFreeBytes"]-keep-INITIAL
    allowed=down(max(0,min(share,left)))
    budget=down(max(0,f["budgetRoomBytes"])) if "budgetRoomBytes" in f else None
    if allowed<GIB:return out("refused","host-short")
    if budget is not None and budget<GIB:return out("refused","budget")
    target=min(wanted,f["capacityBytes"]+allowed,f["capacityBytes"]+(budget if budget is not None else 1<<62))
    if f["held"] and mode=="detached":return out("wait","held",target)
    return out("grow","ok",target)
def lane(rule,root,samples):
    s=sorted(v for v in samples if v>0)
    if not s or root>=len(rule["roots"]):return {"changed":False}
    rank=-(-len(s)*PERCENTILE//10000)
    need=max(FLOOR,s[max(0,rank-1)])
    declared=rule["roots"][root]["needBytes"]
    pref=-(-max(declared,s[-1])*(100+rule["headroomPercent"])//(100+HEADROOM))
    return {"changed":True,"needBytes":need,"measuredNeedBytes":pref,"headroomPercent":HEADROOM}
def main():
    corpus=json.load(open(sys.argv[2]))
    if sys.argv[1]=="answers":
        print(json.dumps({"growth":[plan(r["platform"],corpus["rule"],r["facts"]) for r in corpus["growth"]],"lane":[lane(corpus["rule"],r["root"],r["samples"]) for r in corpus["lane"]]}))
main()
