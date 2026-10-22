"""🦀️ Independent Python oracle of the selected Rust inventory rules (standard library only): it re-derives, from a request, the literal workspace paths, the fallback or the refusal. Requests arrive as JSON on stdin."""
import json, re, sys

PREFIX = "{workspaceRoot}/"
REQUIRED = ["📋️project.json", "nx.json", "package.json"]
MAXIMUM = 8192


def select(request):
    tasks = request["tasks"]
    if not any(t["kind"] == "unit" for t in tasks):
        return {"paths": None}
    for task in tasks:
        if task["kind"] == "other" or task["inputs"] is None or (task["kind"] == "aggregate" and len(task["inputs"])):
            return {"paths": None}
    paths = set(REQUIRED)
    inputs = [i for t in tasks for i in (t["inputs"] or [])] + request["engineInputs"]
    for item in inputs:
        if not item.startswith(PREFIX):
            return {"paths": None}
        path = item[len(PREFIX):]
        if re.search(r"[?*{}\[\]!]", path):
            return {"paths": None}
        if not path or len(path) > 256 or "\\" in path or "\0" in path or ":" in path or any(part in ("", ".", "..") for part in path.split("/")):
            return {"error": "Original literal Native Source path required"}
        paths.add(path)
    if len(paths) > MAXIMUM:
        return {"paths": None}
    return {"paths": sorted(paths)}


def main():
    request = json.load(sys.stdin)
    print(json.dumps([select(row) for row in request["rows"]]))


main()
