import json,math,sys
data=json.load(sys.stdin)
p=data["policy"]
for row in data["cases"]:
    m=row["measured"]
    got=p["queueSaturatedMilliseconds"] if m is None or m["samples"]<p["queueSaturatedMinimumSamples"] else min(p["queueSaturatedCapMilliseconds"],max(p["queueSaturatedFloorMilliseconds"],math.ceil(m["p90Milliseconds"]*p["queueSaturatedFactor"])))
    if got!=row["expected"]:
        print("mismatch",row,got);sys.exit(1)
print("ok")
