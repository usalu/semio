import json,sys
spec=json.load(open(sys.argv[1]))
out={}
for c in spec["cases"]:
    lanes=[list(l) for l in c["lanes"]];iso=set(c["isolated"]);size=c["size"];batches=[]
    while any(lanes):
        for lane in [l for l in lanes if l]:
            b=[]
            while lane and len(b)<size:
                if lane[0] in iso:
                    if not b: b.append(lane.pop(0))
                    break
                b.append(lane.pop(0))
            batches.append(b)
    out[c["name"]]=batches
print(json.dumps(out))
