"""🐍️ Independent oracle of the stale build unit plan and of the units a running build covers: reads the policy path as argv[1], the mode (`plan`, default, `cover` or `images`) as argv[2] and a list of states from stdin, prints the removals (or the covered paths) of every state. It shares no code with the TypeScript planner."""
import json
import sys

policy = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Policy"]["const"]


def plan(state):
    now, need = state["nowMs"], state["needBytes"]
    if state.get("budgetBytes") is not None:
        need = max(need, sum(unit["bytes"] for unit in state["units"]) - state["budgetBytes"])
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
        return now - unit["lastUsedMs"] >= policy["minimumAgeMilliseconds"]

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


def covered(state):
    now = state["nowMs"]
    groups = {}
    for unit in state["units"]:
        groups.setdefault(unit["path"].rsplit("/", 1)[0], []).append(unit)
    keep = set()
    for group in groups.values():
        for unit in sorted(group, key=lambda unit: (-unit["lastUsedMs"], unit["path"]))[: policy["keepNewestPerCrate"]]:
            keep.add(unit["path"])
    return sorted(unit["path"] for unit in state["units"] if unit["path"] in keep or now - unit["lastUsedMs"] < policy["activeBuildMilliseconds"])


def by_image(state):
    groups = {}
    for unit in state["units"]:
        leading = [mount for mount in state["mounts"] if unit["path"].startswith(mount["prefix"] + "/")]
        image = max(leading, key=lambda mount: len(mount["prefix"]))["id"] if leading else "home"
        groups.setdefault(image, []).append(unit)
    removed = []
    for image in sorted(groups):
        removed += plan({"nowMs": state["nowMs"], "needBytes": state["needs"].get(image, 0), "budgetBytes": state.get("budgets", {}).get(image), "units": groups[image]})
    ages = {unit["path"]: unit["lastUsedMs"] for unit in state["units"]}
    return sorted(removed, key=lambda row: (ages[row["path"]], row["path"]))


mode = sys.argv[2] if len(sys.argv) > 2 else "plan"
print(json.dumps([{"cover": covered, "images": by_image}.get(mode, plan)(state) for state in json.load(sys.stdin)]))
