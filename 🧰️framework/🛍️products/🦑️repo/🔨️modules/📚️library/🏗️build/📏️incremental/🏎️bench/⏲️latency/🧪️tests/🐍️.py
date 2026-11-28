#!/usr/bin/env python3
"""🐍️ Independent model of the latency budgets (standard library only): `<schema> <budgets> <corpus>` reduces every profile of the corpus to time per phase group and round trip counts, and judges every observation against a command's budget with the declared tolerance, breach ratio and speed factor."""
import json, math, sys

schema = json.load(open(sys.argv[1], encoding="utf-8"))["definitions"]
budgets = json.load(open(sys.argv[2], encoding="utf-8"))
corpus = json.load(open(sys.argv[3], encoding="utf-8"))
policy, groups, trips = schema["Policy"]["const"], schema["Groups"]["const"], schema["Trips"]["const"]
group_ids = [g["id"] for g in groups]
tenth = lambda x: math.floor(x * 10 + 0.5) / 10


def group_of(phase):
    if phase.startswith(policy["tripPrefix"]):
        return None
    for group in groups:
        if any(phase.startswith(prefix) for prefix in group["prefixes"]):
            return group["id"]
    return "other"


def observe(documents, wall):
    phases = {g: 0 for g in group_ids}
    counts = {t: 0 for t in trips}
    for document in documents:
        for row in document["rows"]:
            g = group_of(row["phase"])
            if g is not None:
                phases[g] += row["self"]
            for t, spec in trips.items():
                marker = spec["marker"]
                if (row["phase"].startswith(marker) if marker.endswith(".") else row["phase"] == marker):
                    counts[t] += 1
    phases["uncovered"] = max(0, wall - sum(v for k, v in phases.items() if k != "uncovered"))
    return {
        "wallMs": tenth(wall),
        "processes": len(documents),
        "phases": {g: tenth(v) for g, v in phases.items()},
        "trips": {t: (counts[t] if trips[t]["measured"] else None) for t in trips},
    }


def judge(observation, command, factor):
    scale, tol, ratio = max(1, factor), policy["toleranceMilliseconds"], policy["breachRatio"]
    rows = []

    def time(metric, value, budget):
        limit = math.ceil(budget * scale)
        status = "ok" if value <= limit + tol else "warn" if value <= limit * ratio + tol else "breach"
        rows.append({"metric": metric, "value": value, "budget": limit, "ratio": None if limit == 0 else math.floor(value / limit * 100 + 0.5) / 100, "status": status})

    def count(metric, value, budget):
        if value is None:
            status = "unmeasured"
        elif value <= budget:
            status = "ok"
        elif value <= max(budget + 1, math.ceil(budget * ratio)):
            status = "warn"
        else:
            status = "breach"
        rows.append({"metric": metric, "value": value, "budget": budget, "ratio": None if value is None or budget == 0 else math.floor(value / budget * 100 + 0.5) / 100, "status": status})

    time("wall", observation["wallMs"], command["wallMs"])
    for g in group_ids:
        if g in command["phases"]:
            time("phase:" + g, observation["phases"][g], command["phases"][g])
    for t in trips:
        if t in command["trips"]:
            count("trips:" + t, observation["trips"][t], command["trips"][t])
    statuses = [r["status"] for r in rows]
    overall = "breach" if "breach" in statuses else "warn" if "warn" in statuses else "ok" if "ok" in statuses else "unmeasured"
    return {"status": overall, "rows": rows}


commands = {c["id"]: c for c in budgets["commands"]}
print(json.dumps({
    "observe": [observe(c["documents"], c["wall"]) for c in corpus["observe"]],
    "judge": [judge(c["observation"], commands[c["command"]], c["factor"]) for c in corpus["judge"]],
}))
