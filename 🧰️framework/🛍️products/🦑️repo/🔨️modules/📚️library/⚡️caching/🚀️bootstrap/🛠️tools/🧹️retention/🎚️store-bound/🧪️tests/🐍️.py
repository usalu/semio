"""Independent oracle of the proactive Nx toolkit store bound: a plain sort-and-walk over rows, no shared code with the TypeScript. Reads {"cases"} (each with its own "policy") from the file in argv[1] and prints one plan per case as JSON. Rule: the generation the workspace activates stays; so does every generation a live reader holds; so do the `keep` newest by (creation time descending, identity ascending), keep being the pressure bound under pressure; so does every generation younger than the minimum age; the rest is listed oldest first by (creation time ascending, identity ascending) and one run retires at most `maximumPerRun` of them."""
import json
import sys


def plan(policy, case):
    keep = policy["keepNewestUnderPressure"] if case["pressure"] else policy["keepNewest"]
    rows = sorted(case["images"], key=lambda r: (-r["createdAt"], r["identity"]))
    newest = {r["identity"] for r in rows[:keep]}
    retained, doomed = [], []
    for r in rows:
        if r["identity"] == case["current"]:
            retained.append({"identity": r["identity"], "reason": "current"})
        elif r["held"]:
            retained.append({"identity": r["identity"], "reason": "held"})
        elif r["identity"] in newest:
            retained.append({"identity": r["identity"], "reason": "newest"})
        elif case["nowMs"] - r["createdAt"] < policy["minimumAgeMs"]:
            retained.append({"identity": r["identity"], "reason": "young"})
        else:
            doomed.append(r)
    doomed.sort(key=lambda r: (r["createdAt"], r["identity"]))
    return {"keepNewest": keep, "retained": retained, "retire": [r["identity"] for r in doomed], "limit": policy["maximumPerRun"]}


if __name__ == "__main__":
    document = json.load(open(sys.argv[1], encoding="utf8"))
    print(json.dumps([plan(case["policy"], case) for case in document["cases"]]))
