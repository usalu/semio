"""Independent oracle: the cold peak of a third-party crate from the admission policy numbers, `max(min(base + perKloc * kloc, budget), floor)`, for every measured crate (argv[1] = corpus, argv[2] = admission schema)."""
import json, sys

corpus = json.load(open(sys.argv[1], encoding="utf-8"))
policy = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]["Policy"]["const"]
out = []
for row in corpus["measured"]:
    estimate = policy["coldExternalBaseBytes"] + policy["coldExternalBytesPerKloc"] * row["lines"] / 1000
    out.append(max(min(-(-estimate // 1), corpus["budgetMiB"] * 1048576), policy["floorBytes"]))
print(json.dumps([int(value) for value in out]))
