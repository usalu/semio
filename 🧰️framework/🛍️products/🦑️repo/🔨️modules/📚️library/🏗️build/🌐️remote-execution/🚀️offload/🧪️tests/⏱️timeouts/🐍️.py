import json,math,sys
data=json.load(sys.stdin)
p=data["policy"]
for row in data["cases"]:
    cap=p["linkedCapMilliseconds"] if row["linked"] else p["capMilliseconds"]
    unknown=p["linkedUnknownMilliseconds"] if row["linked"] else p["unknownMilliseconds"]
    got=min(cap,unknown) if row["predicted"] is None else min(cap,max(p["floorMilliseconds"],math.ceil(row["predicted"]*p["factor"])))
    if got!=row["expected"]:
        print("mismatch",row,got);sys.exit(1)
print("ok")
