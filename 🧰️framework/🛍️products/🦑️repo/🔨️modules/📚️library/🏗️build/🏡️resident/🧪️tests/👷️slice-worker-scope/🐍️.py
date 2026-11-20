import json
import sys

fixture = json.load(sys.stdin)
counts = {name: {"baseline": 0, "probes": 0, "cheap": 0} for name in ("first", "second")}
proved = {}
observations = []
for step in fixture["steps"]:
    request = step["request"]
    elapsed = step["at"]
    action = step["action"]
    count = counts[request]
    count["baseline"] += action == "verify"
    if action == "cancel":
        observations.append("cancelled")
        continue
    count["cheap"] += 1
    if action == "replace-identity":
        observations.append("identity")
        continue
    if request not in proved or elapsed - proved[request] > fixture["windowMilliseconds"]:
        count["probes"] += 1
        if action == "change-mapping":
            observations.append("mapping")
            continue
        proved[request] = elapsed
    observations.append("verified")

print(json.dumps({**{key: [row[key] for row in counts.values()] for key in ("baseline", "probes", "cheap")}, "observations": observations}))
