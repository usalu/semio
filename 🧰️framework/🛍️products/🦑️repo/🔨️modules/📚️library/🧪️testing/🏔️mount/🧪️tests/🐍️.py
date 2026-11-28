"""🐍 Independent oracle of the mount fold and plan admission: reads the vectors on stdin, prints the verdicts as JSON."""
import json, re, sys

def fold(plan, observations):
    statuses, failure = [], None
    for index, phase in enumerate(plan["phases"]):
        seen = observations[index] if index < len(observations) else None
        if failure is not None or seen is None:
            statuses.append("skipped")
            continue
        kind = "deadline" if seen["kind"] == "done" and seen["elapsedMs"] > phase["budgetMs"] else seen["kind"]
        statuses.append("passed" if kind == "done" else "failed")
        if kind != "done":
            failure = {"phase": phase["id"], "code": kind}
    return {"passed": failure is None and all(s == "passed" for s in statuses), "statuses": statuses, "failure": failure}

def valid(plan):
    if set(plan) != {"version", "app", "phases"} or plan["version"] != 1 or not re.fullmatch(r"[a-z][a-z0-9.-]{0,63}", plan["app"]):
        return False
    phases = plan["phases"]
    if not 1 <= len(phases) <= 16 or len({p.get("id") for p in phases}) != len(phases):
        return False
    for p in phases:
        if set(p) != {"id", "budgetMs"} or not re.fullmatch(r"[a-z][a-z0-9-]{0,63}", p["id"]) or not 50 <= p["budgetMs"] <= 60000:
            return False
    return sum(p["budgetMs"] for p in phases) <= 60000

vectors = json.load(sys.stdin)
print(json.dumps({"fold": [fold(r["plan"], r["observations"]) for r in vectors["fold"]], "plans": [valid(r["plan"]) for r in vectors["plans"]]}))
