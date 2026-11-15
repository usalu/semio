import json, sys
case = json.load(sys.stdin)
def row(layer, index):
    return json.dumps({"kind": "analysis", "path": "/w/l%d/f%d" % (layer, index), "fact": {"digest": str(layer * 100000 + index).ljust(64, "d"), "size": index}}, sort_keys=True)
held = [row(layer, index) for entry in case["entries"] for layer in entry["layers"] for index in range(case["rowsPerLayer"])]
print(json.dumps({"heldRows": len(held), "distinctRows": len(set(held))}))
