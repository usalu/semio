import json, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
grace = fixture["graceMilliseconds"]
failures = []
for scenario in fixture["scenarios"]:
    live = {digest for blocks in scenario["headers"].values() for digest in blocks}
    removed = sorted(block["digest"] for block in scenario["blocks"] if block["digest"] not in live and block["ageMilliseconds"] >= grace)
    if removed != sorted(scenario["expectedRemoved"]):
        failures.append(scenario["name"])
print(json.dumps({"failures": failures}))
