"""🔗️ Independent model of the owners that only get their edges again when the environment moved: owners of real project roots (no tag) that are neither inferred again nor stale by root, split by whether they are projects or withheld; when the consumers of the edit are known, the projects that are no consumer keep their edges (carried). Reads one world on stdin, prints the two sorted lists."""
import json
import sys

world = json.load(sys.stdin)
moved = not world["cold"] and world["environment"] != world["previousEnvironment"]
projects, withheld, carried = [], [], []
consumers = world.get("consumers")
if moved:
    for name, stub in world["stubs"].items():
        if stub.get("tag") or name in world["inferred"] or stub["root"] in world["roots"]:
            continue
        if stub["kind"] != "project":
            withheld.append(name)
        elif consumers is not None and name not in consumers:
            carried.append(name)
        else:
            projects.append(name)
print(json.dumps({"projects": sorted(projects), "withheld": sorted(withheld), "carried": sorted(carried)}))
