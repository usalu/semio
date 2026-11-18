import json,sys
doc=json.load(open(sys.argv[1]));policy=json.load(open(sys.argv[2]))["definitions"]["Policy"]["const"]
out=[]
for case in doc["cases"]:
    removed=[]
    for d in case["directories"]:
        if not d["name"].startswith(policy["prefix"]):continue
        if "/tmp/"+d["name"]==case["own"]:continue
        if "owner" in d:gone=not d["alive"]
        else:gone=d["ageMilliseconds"]>policy["graceMilliseconds"]
        if gone:removed.append(d["name"])
    out.append(removed)
print(json.dumps(out))
