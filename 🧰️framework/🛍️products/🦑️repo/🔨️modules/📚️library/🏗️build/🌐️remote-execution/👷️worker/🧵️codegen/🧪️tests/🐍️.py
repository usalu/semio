import json, math, pathlib, sys

def threads(case, policy):
    busy = max(max(0, math.floor(case["others"])), math.ceil(max(0, case["load"])))
    by_cores = math.floor(case["cores"]) - policy["reserveCores"] - busy
    by_memory = 1 + math.floor(max(0, case["spare"] - max(0, case["peak"])) / policy["extraMegabytesPerThread"])
    return max(1, min(policy["maximumThreads"], by_cores, by_memory))

data = json.loads((pathlib.Path(__file__).parent / "🔣️.json").read_text())
bad = [c["id"] for c in data["cases"] if threads(c, data["policy"]) != c["threads"]]
print(json.dumps({"checked": len(data["cases"]), "bad": bad}))
sys.exit(1 if bad else 0)
