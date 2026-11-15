import json, sys

scenario = json.load(sys.stdin)
entries = scenario["entries"]
distinct = set()
references = 0
for layers in entries:
    for layer in layers:
        for row in layer:
            distinct.add(row)
            references += 1
unique_layers = {tuple(layer) for layers in entries for layer in layers}
print(json.dumps({"distinctRows": len(distinct), "rowReferences": references, "distinctLayers": len(unique_layers), "entries": len(entries)}))
