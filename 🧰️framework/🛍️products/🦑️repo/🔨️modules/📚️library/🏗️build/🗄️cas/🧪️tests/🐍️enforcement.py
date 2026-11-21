import json,math,sys

def decide(policy,case):
    part=case["partition"];collected=case["collected"];store=case["storeBytes"];incoming=case["incomingBytes"]
    p=policy["partition"]
    if part is not None:
        used=(part["totalBytes"]-part["freeBytes"]+incoming)/part["totalBytes"]
        if used>=p["criticalUsedFraction"]:return ["refuse" if collected else "collect-now","partition-critical"]
        if used>=p["highUsedFraction"] and not collected:return ["collect-background","partition-high"]
    if store is None:return ["measure","unknown-usage"]
    held=store+incoming
    if held>policy["capBytes"] and not collected:return ["collect-now","cap-exceeded"]
    if held>policy["capBytes"]*policy["highWatermark"] and not collected:return ["collect-background","cap-high"]
    return ["allow","fits"]

def need(policy,row):
    used=row["totalBytes"]-row["freeBytes"];p=policy["partition"]
    return math.ceil(used-row["totalBytes"]*p["lowUsedFraction"]) if used>row["totalBytes"]*p["highUsedFraction"] else 0

data=json.load(sys.stdin)
if "localCompletion" in data:
    print(json.dumps([int(not (row["result"] == "completed" and row["owner"] == "same" and row["origin"] == row["request"])) for row in data["localCompletion"]]))
elif "automaticCompletion" in data:
    corpus=data["automaticCompletion"]
    def scans(row):
        compatible=(row["scope"],row["cap"],row["timestamp"])==("same",corpus["capBytes"],"valid")
        if compatible and not row["critical"] and row["advance"] in range(corpus["minAgeMs"]):
            return int(row["priorBytes"]<=row["cap"])
        return 1
    print(json.dumps([scans(row) for row in corpus["rows"]]))
else:
    print(json.dumps({"decisions":[decide(data["policy"],c) for c in data["cases"]],"needs":[need(data["policy"],n) for n in data["needs"]]}))
