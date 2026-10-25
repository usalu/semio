import json, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
racy = fixture["racyMilliseconds"]
failures = []
for case in fixture["cases"]:
    old = {row[0]: row for row in case["before"]["directories"]}
    stable = sorted(
        row[0]
        for row in case["after"]["directories"]
        if row[0] in old
        and old[row[0]][1] == row[1]
        and old[row[0]][2] == row[2]
        and case["before"]["capturedAt"] - old[row[0]][1] >= racy
    )
    if stable != sorted(case["stable"]):
        failures.append(case["name"])
print(json.dumps({"failures": failures}))
