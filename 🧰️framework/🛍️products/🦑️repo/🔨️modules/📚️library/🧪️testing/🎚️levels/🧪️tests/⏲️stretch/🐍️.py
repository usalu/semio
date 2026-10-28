"""Independent model of the stretched inner tool timeouts.

argv: <levels corpus JSON> <contention policy JSON>; stdin: a JSON list of {"level", "ratio"}.
For each case it takes the stretch-1 argument vectors of that level from the corpus, clamps the ratio to the
policy's stretch bounds and multiplies every timeout (milliseconds rounded up, seconds rounded up from them).
It shares no code with the TypeScript implementation.
"""
import json
import math
import sys


def main():
    corpus = json.load(open(sys.argv[1], encoding="utf8"))
    policy = json.load(open(sys.argv[2], encoding="utf8"))
    low = policy["stretch"]["minimumBasisPoints"] / 10000
    high = policy["stretch"]["maximumBasisPoints"] / 10000
    rows = {row["level"]: row for row in corpus["levels"]}
    out = []
    for case in json.load(sys.stdin):
        row = rows[case["level"]]
        stretch = min(high, max(low, case["ratio"]))
        ms = math.ceil(row["playwright"] * stretch)
        seconds = math.ceil(ms / 1000)
        go = list(row["go"]); go[go.index("-timeout") + 1] = "%ds" % seconds
        vitest = [str(ms) if index % 2 else value for index, value in enumerate(row["vitest"])]
        pytest = [("--timeout=%d" % seconds) if value.startswith("--timeout=") else value for value in row["pytest"]]
        dotnet = list(row["dotnet"]); dotnet[-1] = str(ms)
        out.append({"go": go, "vitest": vitest, "bun": ["--timeout", str(ms)], "pytest": pytest, "dotnet": dotnet, "playwright": ms})
    json.dump(out, sys.stdout)


main()
