"""🛰 Independent model of the fleet-sized jobs plan.

Reads the policy constants from the compile units schema JSON and answers, for every case on stdin (a JSON list of
{"host": {...}}), the planned jobs: the local grant plus the idle fleet slots, bounded by the authored cap, the memory the
light remote jobs may hold beside the local compiles and the memory pressure. It shares no code with the TypeScript
implementation: the limits and their order are written down here once more.
"""
import json
import sys


def plan(policy, host):
    local = host["local"]
    if host["fleetIdle"] is None or host["fleetIdle"] == 0:
        return {"jobs": local, "local": local, "remote": 0, "reason": "fleet-local-only"}
    remote, reason = host["fleetIdle"], "fleet-all"
    limits = [(max(0, policy["fleetJobsMaximum"] - local), "fleet-cap")]
    if host["availableMegabytes"] is not None:
        room = max(0, host["availableMegabytes"] - policy["reserveMegabytes"] - local * host["unitMegabytes"])
        limits.append((room // policy["lightUnitMegabytes"], "fleet-memory"))
    for value, why in limits:
        if value < remote:
            remote, reason = max(0, value), why
    if host["pressure"] == "critical":
        halved = 0
    elif host["pressure"] == "warn":
        halved = remote // policy["pressureWarnDivisor"]
    else:
        halved = remote
    if halved < remote:
        remote, reason = halved, "fleet-pressure"
    return {"jobs": local + remote, "local": local, "remote": remote, "reason": reason}


if __name__ == "__main__":
    with open(sys.argv[1], encoding="utf8") as handle:
        policy = json.load(handle)["definitions"]["Policy"]["const"]
    json.dump([plan(policy, case["host"]) for case in json.load(sys.stdin)], sys.stdout)
