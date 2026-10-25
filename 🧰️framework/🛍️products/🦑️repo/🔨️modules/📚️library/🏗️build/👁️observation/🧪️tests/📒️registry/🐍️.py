import json, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))

def row(owners, name):
    for owner in owners or []:
        if owner["name"] == name:
            return {"root": owner["root"], "manifest": owner["manifest"]}
    return None

failures = []
for step in fixture["steps"]:
    current = fixture["owners"].get(step["registry"]) if step["registry"] else None
    for entry in fixture["recorded"]:
        recorded = row(fixture["owners"][entry["from"]], entry["name"])
        holds = current is not None and row(current, entry["name"]) == recorded
        if holds != step["holds"][entry["name"]]:
            failures.append(step["label"] + " / " + entry["name"])
print(json.dumps({"failures": failures}))
