"""🪫️ Independent oracle of the image relief rules: the clamp of a mounted image, the right-size decision, the in-place shrink, the host pressure and the backoff of a writer that met a full image, written from the policy text with the standard library only."""
import json,sys
GIB=1<<30
SLACK=512*(1<<20);FLOOR=2*GIB;SOFT=40*GIB;HARD=20*GIB;CRITICAL=8*GIB;BACKOFF=[0,2000,8000]
def clamp(row):
    inner=max(0,row["capacityBytes"]-row["usedBytes"]);phantom=max(0,inner-row["availableBytes"])
    return {"capacityBytes":row["capacityBytes"],"usedBytes":row["usedBytes"],"availableBytes":row["availableBytes"],"innerFreeBytes":inner,"phantomBytes":phantom,"clamped":phantom>SLACK,"starved":row["availableBytes"]<FLOOR and inner>=FLOOR+SLACK}
def right(row):
    return {"capacityBytes":row["capacityBytes"],"sizedBytes":row["sizedBytes"],"clamped":row["clamped"],"retire":bool(row["clamped"]) and row["capacityBytes"]-row["sizedBytes"]>=GIB}
def shrink(row):
    keep=-(-row["usedBytes"]*2//GIB)*GIB;target=min(row["capacityBytes"],max(row["sizedBytes"],keep))
    return {"capacityBytes":row["capacityBytes"],"usedBytes":row["usedBytes"],"sizedBytes":row["sizedBytes"],"clamped":row["clamped"],"shrink":bool(row["clamped"]) and row["capacityBytes"]-target>=GIB,"targetBytes":target}
def pressure(row):
    starved=[i["id"] for i in row["images"] if clamp(i)["starved"]]
    level="critical" if row["hostFreeBytes"]<CRITICAL else "hard" if row["hostFreeBytes"]<HARD or starved else "soft" if row["hostFreeBytes"]<SOFT else "none"
    return {"hostFreeBytes":row["hostFreeBytes"],"images":row["images"],"level":level,"demandBytes":max(0,SOFT-row["hostFreeBytes"]) if level!="none" else 0,"starved":starved}
def backoff(row):
    return {"attempt":row["attempt"],"milliseconds":BACKOFF[min(row["attempt"],len(BACKOFF))-1]}
def answers(path):
    d=json.load(open(path))
    return {"clamp":[clamp(r) for r in d["clamp"]],"rightSize":[right(r) for r in d["rightSize"]],"shrink":[shrink(r) for r in d["shrink"]],"pressure":[pressure(r) for r in d["pressure"]],"backoff":[backoff(r) for r in d["backoff"]]}
if __name__=="__main__":
    if sys.argv[1]=="answers":print(json.dumps(answers(sys.argv[2])))
    else:raise SystemExit("unknown command")
