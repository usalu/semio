#!/usr/bin/env python3
# 🐍 Independent implementation of pin resolution and version matching: reads {table, files, packages, matches} on stdin and prints the pin of every row and the verdict of every match as JSON. Standard library only; shares no code with the TypeScript engine.
import json, re, sys

def resolve(row, files, packages):
    pin = row["pin"]
    kind = pin["format"]
    if kind == "value":
        return pin["value"]
    name = pin["file"] if "package" not in pin else packages[pin["package"]] + "/" + pin["file"]
    text = files[name]
    if kind == "regex":
        found = re.search(pin["pattern"], text, re.M if pin.get("flags") == "m" else 0)
        return found.group(1) if found else None
    node = json.loads(text)
    for key in pin["pointer"].split("/")[1:]:
        node = node[key]
    if kind == "json":
        return node
    for item in node:
        if all(str(item.get(key)) == wanted for key, wanted in pin["where"].items()):
            return item[pin["field"]]
    return None

def matches(found, pin):
    return found == pin or found.startswith(pin + ".")

request = json.load(sys.stdin)
table, files, packages = request["table"], request["files"], request.get("packages", {})
print(json.dumps({
    "pins": {row["id"]: resolve(row, files, packages) for row in table["tools"]},
    "matches": [matches(case["found"], case["pin"]) for case in request.get("matches", [])],
}, sort_keys=True))
