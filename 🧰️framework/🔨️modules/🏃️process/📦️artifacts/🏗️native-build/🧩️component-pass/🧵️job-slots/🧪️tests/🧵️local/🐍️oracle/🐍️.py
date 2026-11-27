"""🏠 Independent model of the local component jobs of one build.

Reads the policy constants from the component jobs schema JSON and answers, for every case on stdin (a JSON list of
{"kind": "plan" | "admit" | "slots", "host": {...}}), the decision: how many local component jobs the memory of this machine
carries (plan), whether one more local job may start beside the running ones (admit) and how the slots of a lane are made
of fleet and local jobs (slots). It shares no code with the TypeScript implementation: the laws and their order are
written down here once more.
"""
import json
import sys


def plan(policy, host):
    peaks = host["peaksMegabytes"]
    state = {"jobs": len(peaks), "reason": "local-all"}

    def limit(value, why):
        if value < state["jobs"]:
            state["jobs"], state["reason"] = max(1, value), why

    limit(policy["localJobsMaximum"], "local-cap")
    limit(host["tokens"], "local-grant")
    if host["availableMegabytes"] is None:
        limit(1, "local-unknown")
    else:
        budget, total, fit = host["availableMegabytes"] - host["floorMegabytes"], 0, 0
        for peak in peaks:
            if total + peak > budget:
                break
            total += peak
            fit += 1
        limit(fit, "local-memory")
    if host["pressure"] not in (None, "normal"):
        limit(1, "local-pressure")
    return dict(state)


def admit(host):
    owned = min(host["ownedMegabytes"], host["reservedMegabytes"])
    if host["availableMegabytes"] is None:
        free = 0
    else:
        free = max(-4194304, host["availableMegabytes"] + owned - host["floorMegabytes"] - host["reservedMegabytes"])
    if host["running"] == 0:
        return {"admit": True, "reason": "local-first", "freeMegabytes": free}
    if host["running"] >= host["maximum"]:
        return {"admit": False, "reason": "local-cap", "freeMegabytes": free}
    if host["availableMegabytes"] is None:
        return {"admit": False, "reason": "local-unknown", "freeMegabytes": free}
    if host["pressure"] not in (None, "normal"):
        return {"admit": False, "reason": "local-pressure", "freeMegabytes": free}
    if host["peakMegabytes"] <= free:
        return {"admit": True, "reason": "local-fits", "freeMegabytes": free}
    return {"admit": False, "reason": "local-memory", "freeMegabytes": free}


def slots(policy, host):
    cap = policy["componentJobsMaximum"]
    light = max(0, min(host["fleetJobs"] - 1, cap - 1))
    wanted = max(1, min(host["wanted"], policy["localJobsMaximum"], host["tokens"]))
    jobs = max(1, min(light + wanted, cap))
    return {"jobs": jobs, "light": light, "local": max(1, min(wanted, jobs - light))}


def serial(policy, host):
    """The indexes of the observations a component final forecasts from: those whose CPU time stays within `serialFinalCpuRatio` times the wall time, or all of them when none does."""
    kept = [at for at, row in enumerate(host["rows"]) if row["cpuMs"] <= row["wallMs"] * policy["serialFinalCpuRatio"]]
    return kept or list(range(len(host["rows"])))


if __name__ == "__main__":
    with open(sys.argv[1], encoding="utf8") as handle:
        policy = json.load(handle)["definitions"]["Policy"]["const"]
    answers = {"plan": lambda host: plan(policy, host), "admit": admit, "slots": lambda host: slots(policy, host), "serial": lambda host: serial(policy, host)}
    json.dump([answers[case["kind"]](case["host"]) for case in json.load(sys.stdin)], sys.stdout)
