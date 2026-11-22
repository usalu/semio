import json,sys
rows=[]
for case in json.load(open(sys.argv[1]))["cases"]:
    outcomes=case["outcomes"]
    requests=2 if outcomes[0]=="stalled" else 1
    last=outcomes[requests-1]
    rows.append({"name":case["name"],"requests":requests,"result":{"ok":"bytes","miss":"miss"}.get(last,"down"),"marked":last in("timeout","stalled","network")})
print(json.dumps(rows))
