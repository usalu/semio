"""🐍️ Independent oracle of the stale build unit plan: reads the policy path as argv[1] and a list of states from stdin, prints the removals of every state. It shares no code with the TypeScript planner."""
import json
import sys

policy = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Policy"]["const"]


def plan(state):
    now, need = state["nowMs"], state["needBytes"]
    units = sorted(state["units"], key=lambda unit: (unit["lastUsedMs"], unit["path"]))
    groups = {}
    for unit in units:
        groups.setdefault(unit["path"].rsplit("/", 1)[0], []).append(unit)
    newest = set()
    for group in groups.values():
        for unit in sorted(group, key=lambda unit: (-unit["lastUsedMs"], unit["path"]))[: policy["keepNewestPerCrate"]]:
            newest.add(unit["path"])

    def eligible(unit):
        if unit["held"]:
            return False
        age = now - unit["lastUsedMs"]
        if unit["busy"]:
            return unit["path"] not in newest and age >= policy["busyMinimumAgeMilliseconds"]
        return age >= policy["minimumAgeMilliseconds"]

    removed = {}
    for unit in units:
        if eligible(unit) and unit["path"] not in newest and now - unit["lastUsedMs"] > policy["idleMilliseconds"]:
            removed[unit["path"]] = "superseded"
    freed = sum(unit["bytes"] for unit in units if unit["path"] in removed)
    for pass_newest in (False, True):
        for unit in units:
            if freed >= need:
                break
            if (unit["path"] in newest) == pass_newest and eligible(unit) and unit["path"] not in removed:
                removed[unit["path"]] = "pressure"
                freed += unit["bytes"]
    return [{"path": unit["path"], "bytes": unit["bytes"], "reason": removed[unit["path"]]} for unit in units if unit["path"] in removed]


print(json.dumps([plan(state) for state in json.load(sys.stdin)]))
