"""🐍️ Independent model of the graph memo retention: the newest slices up to the count, the largest prefix of them that fits the byte budget (never fewer than the minimum), every unnamed file past the grace. usage: python3 -I oracle.py <scenarios.json>  ->  one plan per scenario on stdout (JSON)."""
import json, sys

def plan(case):
    retention, now = case["retention"], case["now"]
    ordered = sorted(case["records"], key=lambda row: (-row["mtimeMs"], row["name"]))
    files = case["files"]
    def cost(prefix):
        names = set()
        for row in prefix:
            names.update(row["owned"])
        return sum(row["bytes"] for row in prefix) + sum(files[name]["bytes"] for name in names if name in files)
    top = ordered[: retention["recordsKept"]]
    keep = len(top)
    while keep > max(retention["minimumRecordsKept"], 1) and cost(top[:keep]) > retention["maximumDirectoryBytes"]:
        keep -= 1
    kept = top[:keep]
    dropped = ordered[keep:]
    alive = {name for row in kept for name in row["owned"]}
    stale = sorted(name for name, row in files.items() if name not in alive and now - row["mtimeMs"] > retention["graceMilliseconds"])
    removed = sum(row["bytes"] for row in dropped) + sum(files[name]["bytes"] for name in stale)
    return {
        "removeRecords": sorted(row["name"] for row in dropped),
        "removeFiles": stale,
        "keptRecords": len(kept),
        "keptBytes": cost(kept),
        "removedBytes": removed,
        "overBudget": len(top) - keep,
    }

if __name__ == "__main__":
    cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
    json.dump([plan(case) for case in cases], sys.stdout)
