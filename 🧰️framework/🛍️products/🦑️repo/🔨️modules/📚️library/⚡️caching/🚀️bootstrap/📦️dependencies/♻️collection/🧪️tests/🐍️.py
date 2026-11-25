import json
import sys

rows = json.load(sys.stdin)
policy = rows["policy"]
out = []
for case in rows["cases"]:
    current, held, bound = set(case["current"]), set(case["held"]), set(case["bound"])
    retained = {row["key"]: row for row in case["retained"]}
    pressure = case["pressure"]
    result = []
    for row in case["generations"]:
        key = row["key"]
        if key in current or key in held or key in bound:
            continue
        if key in retained:
            kept = retained[key]
            limit = 0 if pressure else (policy["rollbackSeconds"] if kept["newest"] else policy["retainedGraceSeconds"])
            due = kept["retiredSeconds"] >= limit
        else:
            if row["ready"]:
                limit = 0 if pressure else policy["supersededGraceSeconds"]
            else:
                resumed = min((g for g in case["generations"] if not g["ready"]), key=lambda g: (g["ageSeconds"], g["key"]), default=None)
                limit = policy["unfinishedPressureGraceSeconds"] if pressure and key != resumed["key"] else policy["unfinishedGraceSeconds"]
            due = row["ageSeconds"] >= limit
        if due:
            result.append({"key": key, "reason": "superseded" if row["ready"] else "unfinished"})
    out.append(sorted(result, key=lambda item: item["key"]))
print(json.dumps(out))
