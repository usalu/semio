"""🐍️ Independent model of the consumers of a patched fact index (stdlib only): the owners after a sequence of patches (drop, upsert, append) and, for every row path, the sorted names of the owners that hold it.

usage: 🐍️.py consumers SCRIPT   SCRIPT = {"owners": {name: [path, ...]}, "steps": [{"drop": [name], "upsert": {name: [path]}, "append": {name: [path]}}]}; prints {"owners": [...], "consumers": {path: [names]}} after all steps as JSON"""
import json, sys


def apply(owners, step):
    for name in step.get("drop", []):
        if name not in step.get("upsert", {}):
            owners.pop(name, None)
    for name, paths in step.get("upsert", {}).items():
        owners[name] = list(paths)
    for name, paths in step.get("append", {}).items():
        owners[name] = owners.get(name, []) + list(paths)


def consumers(script):
    owners = {name: list(paths) for name, paths in script["owners"].items()}
    for step in script["steps"]:
        apply(owners, step)
    held = {}
    for name, paths in owners.items():
        for path in paths:
            held.setdefault(path, set()).add(name)
    return {"owners": sorted(owners), "consumers": {path: sorted(names) for path, names in sorted(held.items())}}


if __name__ == "__main__":
    if sys.argv[1] != "consumers":
        raise SystemExit("unknown command")
    print(json.dumps(consumers(json.load(open(sys.argv[2], encoding="utf8"))), ensure_ascii=False))
