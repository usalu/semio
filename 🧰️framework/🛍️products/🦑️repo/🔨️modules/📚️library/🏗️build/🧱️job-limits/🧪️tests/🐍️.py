"""🐍️ Independent model of the job limits: limit of a class, the learned peak after a breach, the memory of a process tree. Reads JSON on argv[2]."""
import json
import sys

RULES = {"headroom": 12500, "growth": 12500, "minimum": 64}


def rounded(value):
    return int(value + 0.5)


def limit(case):
    learned = case.get("learnedPeakMegabytes")
    grown = 0 if learned is None else rounded(learned * RULES["headroom"] / 10000)
    return max(grown, case["floorMegabytes"], RULES["minimum"])


def learned_peak(case):
    if case["kind"] != "memory-exceeded":
        return case["peakMegabytes"]
    return max(case["peakMegabytes"], rounded(case["limitMegabytes"] * RULES["growth"] / 10000))


def tree(case):
    rows = {row["pid"]: row for row in case["rows"]}
    stack, seen, total = [case["root"]], set(), 0
    while stack:
        pid = stack.pop()
        if pid in seen:
            continue
        seen.add(pid)
        if pid in rows:
            total += rows[pid]["kilobytes"]
        stack.extend(row["pid"] for row in case["rows"] if row["ppid"] == pid)
    return -(-total // 1024)


MODES = {"limit": limit, "learned": learned_peak, "tree": tree}
print(json.dumps([MODES[sys.argv[1]](case) for case in json.loads(sys.argv[2])]))
