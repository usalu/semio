import json, math, sys

law = json.load(open(sys.argv[1], encoding="utf-8"))["properties"]["plans"]["const"]
policy = json.load(open(sys.argv[2], encoding="utf-8"))["stretch"]
low, high = policy["minimumBasisPoints"] / 10000, policy["maximumBasisPoints"] / 10000
out = []
for case in json.load(sys.stdin):
    plan = law[case["plan"]]
    factor = min(high, max(low, case["ratio"])) if plan["scaling"] == "load" else 1
    body, drain = (math.ceil(plan[name] * factor) for name in ("bodyMilliseconds", "drainMilliseconds"))
    out.append({"totalMilliseconds": body + drain, "bodyMilliseconds": body, "drainMilliseconds": drain})
json.dump(out, sys.stdout)
