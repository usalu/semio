"""🐍️ Independent oracle of the suite sweep: reads the raw route tables, project files and fixture lines with the standard library only and answers what the TypeScript module must answer (shards, targets, test counts, skip lines, memory, verdict)."""
import json
import re
import sys

ANSI = re.compile("\x1b\\[[0-9;]*m")
COUNT = re.compile(r"^\s*(\d+)\s+(pass|skip|fail)\s*$")


def shards(table, count):
    loads = [0] * count
    members = [[] for _ in range(count)]
    for suite in sorted(table["suites"], key=lambda row: (-row["budgetMs"], row["id"])):
        lightest = min(range(count), key=lambda index: (loads[index], index))
        loads[lightest] += suite["budgetMs"]
        members[lightest].append(suite["id"])
    return [sorted(rows) for rows in members], loads


def targets(project, ids):
    found = {}
    for name in sorted(project["targets"]):
        command = project["targets"][name].get("options", {}).get("command", "")
        match = re.search(r" test ([a-z][a-z0-9]*(?:-[a-z0-9]+)*)$", command)
        if match and (match.group(1) not in found or name == "test-" + match.group(1)):
            found[match.group(1)] = name
    return {suite: found.get(suite) for suite in ids}


def counts(lines):
    total = {"pass": 0, "skip": 0, "fail": 0}
    for line in lines:
        match = COUNT.match(ANSI.sub("", line))
        if match:
            total[match.group(2)] += int(match.group(1))
    return total


def skipped(lines, suite, english, german):
    patterns = []
    for template in (english, german):
        parts = re.split(r"\{([a-z]+)\}", template)
        body = "".join(re.escape(part) if index % 2 == 0 else (re.escape(suite) if part == "suite" else ".+") for index, part in enumerate(parts))
        patterns.append("^" + body + "$")
    return any(re.match(pattern, ANSI.sub("", line)) for line in lines for pattern in patterns)


def memory(samples):
    if not samples:
        return {"samples": 0, "minimumAvailableMegabytes": None, "peakUsedMegabytes": None, "peakSwapMegabytes": None, "peakTreeMegabytes": None, "worstPressure": None}
    swaps = [row["swapMegabytes"] for row in samples if row["swapMegabytes"] is not None]
    trees = [row["treeMegabytes"] for row in samples if row["treeMegabytes"] is not None]
    order = ["normal", "warn", "critical"]
    levels = [row["pressure"] for row in samples if row["pressure"] is not None]
    return {
        "samples": len(samples),
        "minimumAvailableMegabytes": min(row["availableMegabytes"] for row in samples),
        "peakUsedMegabytes": max(row["totalMegabytes"] - row["availableMegabytes"] for row in samples),
        "peakSwapMegabytes": max(swaps) if swaps else None,
        "peakTreeMegabytes": max(trees) if trees else None,
        "worstPressure": max(levels, key=order.index) if levels else None,
    }


def verdict(plan_rows, unrouted, outcomes, enforce):
    failing = {row["table"] + "/" + row["suite"] for row in outcomes if row["status"] in ("failed", "timeout", "unrouted")}
    if enforce:
        by_key = {row["table"] + "/" + row["suite"]: row for row in outcomes}
        for row in plan_rows:
            outcome = by_key.get(row["table"] + "/" + row["suite"])
            if outcome is None or outcome["status"] != "passed":
                failing.add(row["table"] + "/" + row["suite"])
        for row in unrouted:
            failing.add(row["table"] + "/" + row["suite"])
    return sorted(failing)


def unobserved(outcomes):
    ran = [row for row in outcomes if row["status"] != "unrouted"]
    sampled = [row for row in ran if row["memory"]["samples"] > 0]
    found = []
    if ran and not sampled:
        found.append("host")
    if sampled and all(row["memory"]["peakTreeMegabytes"] is None for row in sampled):
        found.append("tree")
    return found


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        request = json.load(handle)
    operation = request["op"]
    if operation == "shards":
        with open(request["table"], encoding="utf-8") as handle:
            rows, loads = shards(json.load(handle), request["count"])
        answer = {"shards": rows, "loads": loads}
    elif operation == "targets":
        with open(request["project"], encoding="utf-8") as handle:
            answer = targets(json.load(handle), request["ids"])
    elif operation == "counts":
        answer = {"counts": counts(request["lines"]), "skipped": skipped(request["lines"], request["suite"], request["english"], request["german"])}
    elif operation == "memory":
        answer = memory(request["samples"])
    elif operation == "unobserved":
        answer = unobserved(request["outcomes"])
    elif operation == "verdict":
        answer = verdict(request["rows"], request["unrouted"], request["outcomes"], request["enforce"])
    else:
        raise SystemExit("unknown operation " + operation)
    json.dump(answer, sys.stdout, ensure_ascii=False)


main()
