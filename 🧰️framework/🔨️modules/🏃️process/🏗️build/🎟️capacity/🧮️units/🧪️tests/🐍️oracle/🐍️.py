"""Independent model of the compile units admission decision.

Reads the policy constants from the schema JSON and answers, for every case on stdin
(a JSON list of {"host": {...}, "intent": {...}}), the planned units. It shares no code with the
TypeScript implementation: the share, the limits and their order are written down here once more.
"""
import json
import math
import sys


def load_policy(path):
    with open(path, encoding="utf8") as handle:
        return json.load(handle)["definitions"]["Policy"]["const"]


def plan(policy, host, intent):
    units = max(1, host["cores"] * policy["shareBasisPoints"] // 10000)
    reason = "share"
    limits = []
    if intent["needed"] is not None:
        limits.append((intent["needed"], "needed"))
    if host["grantedJobs"] is not None:
        limits.append((host["grantedJobs"], "granted"))
    if host["availableMegabytes"] is not None:
        spare = max(0, host["availableMegabytes"] - policy["reserveMegabytes"])
        limits.append((spare // policy["megabytesPerUnit"], "memory"))
    for value, why in limits:
        if value < units:
            units, reason = max(1, value), why
    if host["pressure"] == "critical":
        if 1 < units:
            units, reason = 1, "pressure-critical"
    elif host["pressure"] == "warn":
        halved = math.ceil(units / policy["pressureWarnDivisor"])
        if halved < units:
            units, reason = max(1, halved), "pressure-warn"
    minimum = max(1, -(-units * policy["minimumShareBasisPoints"] // 10000))
    return {"preferred": units, "minimum": minimum, "reason": reason}


def fixed(policy, name, cores):
    row = policy["fixedClasses"][name]
    share = max(1, cores * policy["shareBasisPoints"] // 10000)
    maximum = min(row["maximumUnits"], share)
    return {"kind": "available", "maximumParallelism": maximum, "minimumParallelism": min(row["minimumUnits"], maximum)}


def simulate(cores, requests):
    """Counted admission of FIFO requests onto `cores` free cores: each request is (minimum, preferred).

    A request starts once `minimum` cores are free and takes up to `preferred`; a request behind a blocked head waits (first come, first served).
    Returns the grants in start order for requests that start without any release.
    """
    free = cores
    granted = []
    for minimum, preferred in requests:
        if free < minimum:
            break
        take = min(preferred, free)
        free -= take
        granted.append(take)
    return granted


if __name__ == "__main__":
    policy = load_policy(sys.argv[1])
    cases = json.load(sys.stdin)
    out = []
    for case in cases:
        if "fixed" in case:
            out.append(fixed(policy, case["fixed"], case["cores"]))
        elif "requests" in case:
            out.append(simulate(case["cores"], case["requests"]))
        else:
            out.append(plan(policy, case["host"], case["intent"]))
    json.dump(out, sys.stdout)
