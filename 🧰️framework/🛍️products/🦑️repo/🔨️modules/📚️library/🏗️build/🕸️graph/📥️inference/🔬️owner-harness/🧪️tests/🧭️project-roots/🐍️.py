import json,sys
workspace,registry=sys.argv[1],sys.argv[2]
rows=set()
for owner in json.load(open(registry,encoding="utf8"))["owners"]:
    rows.add((owner["root"],owner["name"]))
    try:
        text=open(workspace+"/"+owner["root"]+"/📋️project.json",encoding="utf8").read()
        native=json.loads(text).get("metadata",{}).get("nativeRoot")
    except (OSError,ValueError,AttributeError):
        continue
    if isinstance(native,str) and native!=owner["root"]:
        rows.add((native,owner["name"]))
print(json.dumps(sorted(rows)))
