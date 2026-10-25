import json,sys

policy=json.load(open(sys.argv[1],encoding="utf8"))
fixture=json.load(sys.stdin)
exact={}
prefixes=[]
for row in policy["classes"]:
    for match in row["match"]:
        if match["kind"]=="exact":exact[match["value"]]=row
        else:prefixes.append((match["value"],row))
prefixes.sort(key=lambda entry:-len(entry[0]))
edges={(edge["from"],edge["to"]) for edge in policy["allowed"] if "binding" not in edge}
grace=policy["holdAndWait"]["graceMilliseconds"]

def classify(resource):
    if resource in exact:return exact[resource]
    for value,row in prefixes:
        if resource.startswith(value):return row
    return {"id":"unclassified","rank":0}

def replay(steps):
    held=[]
    found=[]
    for step in steps:
        if step["op"]=="release":
            for position,row in enumerate(held):
                if row[0]==step["resource"]:
                    del held[position]
                    break
            continue
        wanted=classify(step["resource"])
        waited=step.get("waitedMilliseconds",0)
        if wanted["rank"]>0:
            later=[row for row in held if classify(row[0])["rank"]>wanted["rank"] and (classify(row[0])["id"],wanted["id"]) not in edges]
            if later:found.append({"kind":"out-of-order","held":later[0][0],"requested":step["resource"]})
        if waited>=grace:
            blocking=[row for row in held if row[1]=="exclusive" and row[0]!=step["resource"] and (classify(row[0])["id"],wanted["id"]) not in edges]
            if blocking:found.append({"kind":"hold-and-wait","held":blocking[0][0],"requested":step["resource"]})
        held.append((step["resource"],step["mode"]))
    return found

print(json.dumps({row["id"]:replay(row["steps"]) for row in fixture["sequences"]}))
