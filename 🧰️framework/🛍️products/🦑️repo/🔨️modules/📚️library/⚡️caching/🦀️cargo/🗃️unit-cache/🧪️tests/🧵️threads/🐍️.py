"""🧵️ Independent Python oracle of the per-unit front-end thread decision (standard library only): reads the corpus and the constrained thread count from the policy file and prints one JSON row per case."""
import json, sys

def decide(case, budget, constrained):
    args = case["args"]
    if "--crate-name" not in args or "--out-dir" not in args or not any(a == "-Z" or a.startswith("-Z") for a in args):
        return None
    peak = case.get("peakMegabytes")
    if peak is not None and peak > budget:
        return {"threads": constrained, "reason": "peak"}
    host = case.get("host")
    if host is not None and (host["level"] != "calm" or host["availableMegabytes"] < host["guardMegabytes"]):
        return {"threads": constrained, "reason": "host-memory"}
    return None

corpus = json.load(open(sys.argv[1], encoding="utf-8"))
policy = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]["Policy"]["const"]["threads"]
print(json.dumps([decide(case, corpus["budgetMegabytes"], policy["constrained"]) for case in corpus["cases"]]))
