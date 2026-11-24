"""🧢️ Independent Python oracle of the unit ceiling (standard library only): a unit key is `<package>@<crate>|<target>|<profile>|<crate types>` (exactly four `|` parts, the first holds an `@`, crate and target are not empty; any other key is ignored). The peak of a unit is the largest `peakRssBytes` of its rows, rounded up to whole MiB. A unit is exceeded when that peak is above the policy ceiling; it is listed when its (crate, target) is on the carving list. A carving entry is resolved when some row of its (crate, target) exists and the largest peak of all its units is at or below the ceiling. `ok` means no unlisted exceeded unit and no resolved entry. Order: exceeded and resolved by peak descending, then key (resolved: crate, then target). usage: 🐍️.py <schema.json> <corpus.json> prints the expected verdict of every case as JSON."""
import json, sys

MIB = 1048576


def parse(key):
    parts = key.split("|")
    if len(parts) != 4 or "@" not in parts[0]:
        return None
    crate = parts[0].partition("@")[2]
    return (crate, parts[1]) if crate and parts[1] else None


def verdict(policy, rows):
    unit, group = {}, {}
    for row in rows:
        ident = parse(row["key"])
        if ident is None:
            continue
        peak = -(-row["peakRssBytes"] // MIB)
        unit[row["key"]] = max(unit.get(row["key"], 0), peak)
        group[ident] = max(group.get(ident, 0), peak)
    ceiling = policy["megabytes"]
    listed = {(e["crate"], e["target"]) for e in policy["carving"]}
    exceeded = [{"key": k, "crate": parse(k)[0], "target": parse(k)[1], "peakMegabytes": p, "listed": parse(k) in listed} for k, p in unit.items() if p > ceiling]
    exceeded.sort(key=lambda e: (-e["peakMegabytes"], e["key"]))
    resolved = [{"crate": e["crate"], "target": e["target"], "peakMegabytes": group[(e["crate"], e["target"])]} for e in policy["carving"] if (e["crate"], e["target"]) in group and group[(e["crate"], e["target"])] <= ceiling]
    resolved.sort(key=lambda e: (-e["peakMegabytes"], e["crate"], e["target"]))
    return {"version": 1, "ceilingMegabytes": ceiling, "exceeded": exceeded, "resolved": resolved, "ok": all(e["listed"] for e in exceeded) and not resolved}


if __name__ == "__main__":
    policy = json.load(open(sys.argv[1], encoding="utf-8"))["definitions"]["Policy"]["const"]["unitCeiling"]
    print(json.dumps([verdict(policy, c["observations"]) for c in json.load(open(sys.argv[2], encoding="utf-8"))["cases"]], ensure_ascii=False))
