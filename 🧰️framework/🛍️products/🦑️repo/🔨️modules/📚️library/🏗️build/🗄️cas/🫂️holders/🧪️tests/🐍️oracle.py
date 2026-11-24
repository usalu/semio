import json,sys

def shielded(entries,blobs,floors):
    kept=set()
    for holder,floor in floors.items():
        rows=sorted([e for e in entries if e.get("holder")==holder],key=lambda e:(-e["access"],e["id"]))
        seen=set();size=0
        for e in rows:
            if size>=floor:
                break
            if any(d not in blobs for d in e["digests"]):
                continue
            kept.add(e["id"])
            for d in set(e["digests"]):
                if d not in seen:
                    seen.add(d);size+=blobs[d]["size"]
    return kept

def plan(entries,blobs,need,floors):
    shield=shielded(entries,blobs,floors)
    if need<=0:
        return [],[],0,len(shield)
    order=sorted(entries,key=lambda e:(e["access"],e["id"]))
    alive={e["id"]:set(e["digests"]) for e in entries}
    chosen=[];freed=0;digests=[]
    changed=True
    while changed and freed<need:
        changed=False
        for entry in order:
            if freed>=need:
                break
            name=entry["id"]
            if name not in alive or name in shield:
                continue
            own=alive[name]
            others=set()
            for other,held in alive.items():
                if other!=name:
                    others|=held
            sole=[d for d in own if d not in others]
            dead=any(d not in blobs for d in own)
            gain=[d for d in sole if d in blobs and blobs[d]["freeable"]]
            if not dead and not gain:
                continue
            del alive[name]
            chosen.append(name)
            changed=True
            for d in gain:
                freed+=blobs[d]["size"]
                digests.append(d)
    return chosen,sorted(digests),freed,len(shield)

rows=[]
for case in json.load(sys.stdin):
    chosen,digests,freed,kept=plan(case["entries"],case["blobs"],case["need"],case.get("floors",{}))
    rows.append({"entries":chosen,"digests":digests,"freedBytes":freed,"protectedEntries":kept})
print(json.dumps(rows))
