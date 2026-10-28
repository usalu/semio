"""🍷 Independent oracle of the acceptance laws, written from the budget semantics and the Bazel summary format, sharing no code with the TypeScript.
usage: python3 -I oracle.py observe FIXTURE   -> {case: {executed, served}} read from the recorded logs
       python3 -I oracle.py judge GRID        -> one verdict per grid entry {budget, records, outcome}
       python3 -I oracle.py edit CASES        -> [] when every {original, line, edited, restored} (base64) is an exact append and an exact restore"""
import base64, hashlib, json, re, statistics, sys

SUMMARY = re.compile(r"^INFO: (\d+) process(?:es)?: (.*?)\.?$")
SERVED = (re.compile(r"^graph: \d+ projects served from the memo"), re.compile(r"^graph: \d+ Projekte aus dem Memo"))
SATURATED = 1.5
NEED = {"max": 1, "min": 1, "p50": 3}


def expand(rows):
    for row in rows:
        if isinstance(row, str):
            yield row
        else:
            for _ in range(row[1]):
                yield row[0]


def observe(lines):
    total, seen, served = 0, False, False
    for line in lines:
        match = SUMMARY.match(line)
        if match:
            seen = True
            for part in match.group(2).split(", "):
                cell = re.match(r"^(\d+) (.+)$", part.strip())
                if cell and cell.group(2) != "internal" and not cell.group(2).endswith("cache hit"):
                    total += int(cell.group(1))
        if any(pattern.match(line) for pattern in SERVED):
            served = True
    return {"executed": total if seen else None, "served": served}


def value_of(record, metric):
    return {"wallMs": record["wallMs"], "peakFootprintMb": record["peakMb"], "executedActions": record["executed"]}[metric]


def verdict(status, reason=None, value=None, n=0, bound="exact"):
    return {"status": status, "reason": reason, "value": value, "n": n, "bound": bound}


def judge(budget, records, outcome):
    if outcome and outcome.get("status") == "skipped":
        return verdict("unproven", "skipped")
    if not records:
        return verdict("unproven", "run-failed" if outcome and outcome.get("status") == "failed" else "no-samples")
    good = [r for r in records if r["exit"] == 0 and r["killed"] is None]
    if not good:
        return verdict("unproven", "run-failed", n=len(records))
    pairs = [(r, value_of(r, budget["metric"])) for r in good]
    pairs = [(r, v) for r, v in pairs if v is not None]
    if len(pairs) < NEED[budget["statistic"]]:
        return verdict("unproven", "no-samples", n=len(pairs))
    numbers = sorted(v for _, v in pairs)
    stat = numbers[-1] if budget["statistic"] == "max" else numbers[0] if budget["statistic"] == "min" else statistics.median(numbers)
    slack = max([r["clock"]["grainMs"] for r, _ in pairs if r["clock"]["kind"] == "poll"] + [0]) if budget["metric"] == "wallMs" else 0
    resident = budget["metric"] == "peakFootprintMb" and any(r["measure"] == "resident" for r, _ in pairs)
    bound = "upper" if slack > 0 or resident else "exact"
    n = len(pairs)
    if stat <= budget["maximum"]:
        return verdict("pass", None, stat, n, bound)
    if resident:
        return verdict("unproven", "resident-only", stat, n, bound)
    if slack > 0 and stat - slack <= budget["maximum"]:
        return verdict("unproven", "clock-grain", stat, n, bound)
    if budget["metric"] == "wallMs" and any(r["loadPerCore"] is not None and r["loadPerCore"] >= SATURATED for r, _ in pairs):
        return verdict("unproven", "host-saturated", stat, n, bound)
    return verdict("fail", None, stat, n, bound)


def check_edit(case):
    original, edited, restored = (base64.b64decode(case[k]) for k in ("original", "edited", "restored"))
    tail = (b"\n" if original and not original.endswith(b"\n") else b"") + case["line"].encode("utf8")
    problems = []
    if not edited.startswith(original):
        problems.append("the edit changed an existing byte")
    if edited[len(original):] != tail:
        problems.append("the appended tail is not the newline-if-missing plus the line")
    if edited == original:
        problems.append("the edit changed nothing")
    if restored != original or hashlib.sha256(restored).hexdigest() != hashlib.sha256(original).hexdigest():
        problems.append("the restore is not byte-exact")
    return problems


def main(argv):
    mode, path = argv[1], argv[2]
    data = json.load(open(path, encoding="utf8"))
    if mode == "observe":
        print(json.dumps({case["id"]: observe(expand(case["log"])) for case in data["cases"]}))
    elif mode == "judge":
        print(json.dumps([judge(entry["budget"], entry["records"], entry.get("outcome")) for entry in data]))
    elif mode == "edit":
        print(json.dumps([problem for case in data for problem in check_edit(case)]))
    else:
        sys.exit("mode: observe|judge|edit")


main(sys.argv)
