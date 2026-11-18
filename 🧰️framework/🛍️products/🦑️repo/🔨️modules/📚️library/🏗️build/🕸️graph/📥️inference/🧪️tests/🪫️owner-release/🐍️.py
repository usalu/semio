import json, sys
case = json.load(sys.stdin)
held, after = 0, []
for owner in range(1, case["owners"] + 1):
    held += case["entriesPerOwner"]
    if owner % case["interval"] == 0:
        held = 0
    after.append(held)
grant = case["grantMegabytes"]
def share(footprint):
    percent = footprint * 100 / grant
    return "wait" if percent >= case["hardPercent"] else "settle" if percent >= case["softPercent"] else "run"
readings = [{"floored": max(row["footprint"], row["managed"]), "decision": share(max(row["footprint"], row["managed"]))} for row in case["readings"]]
lowest, due, threshold = None, [], case["lookReleaseGrowthBytes"] // 1048576
for managed, shrink in zip(case["lookManagedMegabytes"], case["lookShrink"]):
    lowest = managed if lowest is None else min(lowest, managed)
    release = shrink or managed - lowest >= threshold
    due.append(release)
    if release:
        lowest = None
print(json.dumps({"held": after, "peak": max(after), "readings": readings, "looks": due}))
