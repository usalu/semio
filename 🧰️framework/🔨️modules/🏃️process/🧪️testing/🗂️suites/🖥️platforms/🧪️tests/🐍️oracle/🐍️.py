"""🔎️ Independent Python reading of the platform evidence rules of the suite route tables: no code of the TypeScript module is used, only the closed policy document and the test files.

Usage:
  python3 -I 🐍️.py evidence <policy.json> <texts.json>   texts.json = list of test file texts; prints one {unconditioned, conditioned, gated} per text
  python3 -I 🐍️.py suites <policy.json> <suites.json>    suites.json = list of {id, files: [absolute path]}; prints {id: [unconditioned causes]}
  python3 -I 🐍️.py findings <table.json>                 prints the suites without a verdict, without an explanation and with a needless explanation
"""
import bisect, json, re, sys


def compile_policy(policy):
    flags = re.ASCII
    return {
        "header": re.compile(policy["header"], flags),
        "gated": re.compile(policy["gatedModifier"], flags),
        "line": re.compile(policy["lineGate"], flags),
        "condition": re.compile(policy["condition"], flags),
        "before_none": policy["window"]["linesBeforeWithoutHeader"],
        "before_marker": policy["window"]["linesBeforeMarker"],
        "causes": [(c["id"], c["lineGate"], [re.compile(m, flags) for m in c["markers"]]) for c in policy["causes"]],
    }


def evidence(text, rules):
    lines = text.split("\n")
    heads = []
    for index, line in enumerate(lines):
        found = list(rules["header"].finditer(line))
        if found:
            heads.append((index, bool(rules["gated"].search(found[-1].group(1) or ""))))
    where = [h[0] for h in heads]
    result = {"unconditioned": set(), "conditioned": set(), "gated": set()}
    for index, line in enumerate(lines):
        for cause, line_gate, patterns in rules["causes"]:
            if not any(p.search(line) for p in patterns):
                continue
            if line_gate and rules["line"].search(line):
                continue
            at = bisect.bisect_right(where, index) - 1
            if at >= 0 and heads[at][1]:
                result["gated"].add(cause)
                continue
            low = where[at] if at >= 0 else max(0, index - rules["before_none"])
            high = where[at + 1] if at + 1 < len(where) else len(lines)
            block = "\n".join(lines[low:high]) + "\n" + "\n".join(lines[max(0, index - rules["before_marker"]):index + 1])
            result["conditioned" if rules["condition"].search(block) else "unconditioned"].add(cause)
    order = [c[0] for c in rules["causes"]]
    return {key: [c for c in order if c in value] for key, value in result.items()}


def suite_causes(files, rules):
    found = set()
    for path in files:
        with open(path, encoding="utf-8", errors="replace") as handle:
            found.update(evidence(handle.read(), rules)["unconditioned"])
    return [c[0] for c in rules["causes"] if c[0] in found]


def findings(table):
    hosts = {"darwin", "linux", "win32"}
    out = {"unjudged": [], "unexplained": [], "overexplained": []}
    for suite in table["suites"]:
        if "platforms" not in suite:
            out["unjudged"].append(suite["id"])
            continue
        full = hosts <= set(suite["platforms"])
        causes = suite.get("platformCauses", [])
        if not full and not causes:
            out["unexplained"].append(suite["id"])
        if full and causes:
            out["overexplained"].append(suite["id"])
    return out


if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "findings":
        print(json.dumps(findings(json.load(open(sys.argv[2], encoding="utf-8")))))
    else:
        rules = compile_policy(json.load(open(sys.argv[2], encoding="utf-8")))
        rows = json.load(open(sys.argv[3], encoding="utf-8"))
        if mode == "evidence":
            print(json.dumps([evidence(text, rules) for text in rows]))
        elif mode == "suites":
            print(json.dumps({row["id"]: suite_causes(row["files"], rules) for row in rows}))
        else:
            sys.exit(2)
