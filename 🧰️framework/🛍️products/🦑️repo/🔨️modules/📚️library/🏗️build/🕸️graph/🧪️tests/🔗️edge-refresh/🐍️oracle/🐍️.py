"""🔗️ Independent model of the owners that only get their edges again, without inference: owners of real project roots (no tag) that are neither inferred again nor stale by root are refreshed when the environment moved, or when they are named stale (their stored edges are of another environment: always refreshed, never carried, also while the environment stays; a scanned root excuses nothing); split by whether they are projects or withheld; when the consumers of the edit are known, the projects that are no consumer keep their edges (carried). A world with a `heal` key is a stored owner table after a compute: the stale untagged owners get their edges again first, tagged ones and the untagged ones that stayed stale although they were given their edges (`tried`) are inferred once (`inferred`), owners `pending` for a later wave are never stale, nothing is left on a cold build or after `limit` rounds. Reads one world on stdin, prints the sorted lists."""
import json
import sys

world = json.load(sys.stdin)


def heal(world):
    heal = world["heal"]
    if heal["cold"] or heal["healed"] >= heal.get("limit", 64):
        return {"edges": [], "infer": []}
    pending = set(heal.get("pending", []))
    stale = sorted(name for name, row in heal["owners"].items() if row["edgesEnv"] != heal["environment"] and name not in pending)
    edges = [name for name in stale if not heal["owners"][name].get("tag")]
    tagged = [name for name in stale if heal["owners"][name].get("tag")]
    tried, inferred = set(heal.get("tried", [])), set(heal.get("inferred", []))
    return {"edges": [name for name in edges if name not in tried], "infer": sorted(name for name in tagged + [name for name in edges if name in tried] if name not in inferred)}


def refresh(world):
    forced = set(world.get("stale", []))
    moved = not world["cold"] and world["environment"] != world["previousEnvironment"]
    projects, withheld, carried = [], [], []
    consumers = world.get("consumers")
    if world["cold"] or not (moved or forced):
        return {"projects": [], "withheld": [], "carried": []}
    for name, stub in world["stubs"].items():
        if stub.get("tag") or name in world["inferred"]:
            continue
        if not (moved or name in forced):
            continue
        if stub["kind"] != "project":
            withheld.append(name)
        elif moved and consumers is not None and name not in consumers and name not in forced:
            carried.append(name)
        else:
            projects.append(name)
    return {"projects": sorted(projects), "withheld": sorted(withheld), "carried": sorted(carried)}


print(json.dumps(heal(world) if "heal" in world else refresh(world)))
