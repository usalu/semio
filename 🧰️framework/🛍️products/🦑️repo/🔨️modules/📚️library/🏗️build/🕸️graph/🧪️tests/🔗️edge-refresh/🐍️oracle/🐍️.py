"""🔗️ Independent model of the owners that only get their edges again when the environment moved: owners of real project roots (no tag) that are neither inferred again nor stale by root, split by whether they are projects or withheld. Reads one world on stdin, prints the two sorted lists."""
import json
import sys

world = json.load(sys.stdin)
moved = not world["cold"] and world["environment"] != world["previousEnvironment"]
projects, withheld = [], []
if moved:
    for name, stub in world["stubs"].items():
        if stub.get("tag") or name in world["inferred"] or stub["root"] in world["roots"]:
            continue
        (projects if stub["kind"] == "project" else withheld).append(name)
print(json.dumps({"projects": sorted(projects), "withheld": sorted(withheld)}))
