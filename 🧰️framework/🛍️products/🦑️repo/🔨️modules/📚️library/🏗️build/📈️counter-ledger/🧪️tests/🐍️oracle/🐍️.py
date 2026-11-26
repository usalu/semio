import json, math, re, sys

fixture = json.load(sys.stdin)
pattern = re.compile(fixture["pattern"])
name_ok = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$")


def whole(value):
    return math.floor(value) if isinstance(value, (int, float)) and value > 0 else 0


result = {"cases": [], "temporaries": []}
for case in fixture["cases"]:
    total = {}
    for run in case["runs"]:
        for group, counts in run["groups"].items():
            if not name_ok.match(group):
                continue
            row = total.setdefault(group, {})
            for name, count in counts.items():
                if name_ok.match(name):
                    row[name] = row.get(name, 0) + whole(count)
    result["cases"].append({"total": {g: dict(sorted(r.items())) for g, r in sorted(total.items())}, "restarts": len(case["runs"])})
for name in fixture["temporaries"]:
    found = pattern.match(name)
    result["temporaries"].append([found.group(1), int(found.group(2))] if found else None)
json.dump(result, sys.stdout)
