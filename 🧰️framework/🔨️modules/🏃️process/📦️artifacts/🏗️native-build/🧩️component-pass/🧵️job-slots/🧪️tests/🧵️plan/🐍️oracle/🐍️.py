"""🧵 Independent model of the concurrent component jobs plan.

Reads the policy constants from the compile units schema JSON and answers, for every case on stdin (a JSON list of
{"host": {...}}), the planned concurrent component jobs: the local grant plus the worker slots that can take a wasm
final, bounded by the authored cap, the memory the local residue of an offloaded job may hold beside the local compiles
and the memory pressure. It shares no code with the TypeScript implementation: the limits and their order are written
down here once more.
"""
import json
import sys


def plan(policy, host):
    local = host["local"]
    if host["fleetSlots"] is None or host["fleetSlots"] == 0:
        return {"jobs": local, "local": local, "remote": 0, "reason": "jobs-local-only"}
    remote, reason = host["fleetSlots"], "jobs-all"
    limits = [(max(0, policy["componentJobsMaximum"] - local), "jobs-cap")]
    if host["availableMegabytes"] is not None:
        room = max(0, host["availableMegabytes"] - policy["reserveMegabytes"] - local * host["unitMegabytes"])
        limits.append((room // policy["componentJobResidueMegabytes"], "jobs-memory"))
    for value, why in limits:
        if value < remote:
            remote, reason = max(0, value), why
    if host["pressure"] == "critical" and remote > 0:
        remote, reason = 0, "jobs-pressure"
    return {"jobs": local + remote, "local": local, "remote": remote, "reason": reason}


if __name__ == "__main__":
    with open(sys.argv[1], encoding="utf8") as handle:
        policy = json.load(handle)["definitions"]["Policy"]["const"]
    json.dump([plan(policy, case["host"]) for case in json.load(sys.stdin)], sys.stdout)
