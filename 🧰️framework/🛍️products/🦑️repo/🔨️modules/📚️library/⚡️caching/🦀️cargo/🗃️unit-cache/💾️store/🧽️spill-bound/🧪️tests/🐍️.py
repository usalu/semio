import json,sys

policy=json.load(open(sys.argv[1],encoding="utf8"))["definitions"]["Policy"]["const"]["spill"]

def bound(facts):
    total=facts["totalBytes"]
    return max(0,min(policy["maximumBytes"],total*policy["sharePercent"]//100,facts["spillBytes"]+facts["availableBytes"]-total*policy["keepFreePercent"]//100))

def plan(state):
    protect=state["nowMs"]-policy["protectMilliseconds"]
    size={row["sha256"]:row["bytes"] for row in state["objects"]}
    used={row["sha256"]:row["usedMs"] for row in state["objects"]}
    users={}
    for entry in state["entries"]:
        for name in set(entry["objects"]):users.setdefault(name,set()).add(entry["key"])
    last={entry["key"]:max([entry["usedMs"]]+[used.get(name,0) for name in entry["objects"]]) for entry in state["entries"]}
    gone_objects=[];gone_entries=[];total=sum(size.values());freed=0
    def drop(name):
        nonlocal total,freed
        if name in size:
            gone_objects.append(name);total-=size[name];freed+=size[name];del size[name]
    for row in state["objects"]:
        if row["sha256"] not in users and row["usedMs"]<protect:drop(row["sha256"])
    live=[]
    for entry in state["entries"]:
        if last[entry["key"]]<protect and not any(name in size for name in entry["objects"]):gone_entries.append(entry["key"])
        else:live.append(entry)
    for entry in sorted((e for e in live if last[e["key"]]<protect),key=lambda e:(last[e["key"]],e["key"])):
        if total<=state["boundBytes"]:break
        gone_entries.append(entry["key"])
        for name in set(entry["objects"]):
            users[name].discard(entry["key"])
            if not users[name]:drop(name)
    return {"entries":gone_entries,"objects":sorted(gone_objects),"bytes":freed}

request=json.load(sys.stdin)
print(json.dumps({"bounds":[bound(row) for row in request["bounds"]],"plans":[plan(row) for row in request["plans"]]}))
