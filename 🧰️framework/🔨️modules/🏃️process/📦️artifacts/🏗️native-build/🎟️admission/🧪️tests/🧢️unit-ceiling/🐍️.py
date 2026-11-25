"""🧢️ Independent Python oracle of the compile-memory budget (standard library only). A unit key is `<package>@<crate>|<target>|<profile>|<crate types>` (exactly four `|` parts, the first holds an `@`, crate and target are not empty; any other key is ignored). A key whose crate types hold a linked type (cdylib, bin, dylib, staticlib) is a final link unit with the final budget of the active worker class, every other key a library unit with the library budget. The peak of a unit is the largest `peakRssBytes` of its rows, rounded up to whole MiB. A unit is exceeded when that peak is above the budget of its kind; it is listed when its (crate, target) is on the carving list. A carving entry is resolved when some row of its (crate, target) exists and every unit of it is within its own budget; its reported peak is the largest peak and its budget the largest budget of those units. `ok` means no unlisted exceeded unit and no resolved entry. Order: exceeded and resolved by peak descending, then key (resolved: crate, then target). The fresh rows of a ledger are, per key, the rows that record `lines` and lie within the policy tolerance of the row with the greatest `atMs` (ties: the later row in the file). The runtime ceiling of a running compile unit (`ceilings`) is read from its compiler command line split at white space: the crate is the first `--crate-name` value (`--crate-name X` or `--crate-name=X`; none or empty means no unit, null), the target the first `--target` value (`host` without), the crate types every `--crate-type` value split at commas, plus `bin` when `--test` is present. The unit is `<crate>|<target>`; a linked type among the types makes it a `final` unit with the final budget, else `library` with the library budget; a unit whose (crate, target) is on the carving list is `listed` with `measuredMegabytes` and may reach the larger of its declared `budgetMegabytes` and the budget of its kind. usage: 🐍️.py <schema.json> <corpus.json> prints {"verdicts": [...], "fresh": [...], "ceilings": [...]} for every case as JSON."""
import json, sys

MIB = 1048576
LINKED = {"cdylib", "bin", "dylib", "staticlib"}


def parse(key):
    parts = key.split("|")
    if len(parts) != 4 or "@" not in parts[0]:
        return None
    crate = parts[0].partition("@")[2]
    return (crate, parts[1]) if crate and parts[1] else None


def budget_of(policy, key):
    cls = policy["classes"][policy["workerClass"]]
    return cls["finalMegabytes"] if LINKED & set(key.split("|")[3].split(",")) else cls["libraryMegabytes"]


def verdict(policy, rows):
    unit = {}
    for row in rows:
        if parse(row["key"]) is None:
            continue
        unit[row["key"]] = max(unit.get(row["key"], 0), -(-row["peakRssBytes"] // MIB))
    group = {}
    for k, p in unit.items():
        g = group.setdefault(parse(k), {"peak": 0, "budget": 0, "fits": True})
        g["peak"] = max(g["peak"], p)
        g["budget"] = max(g["budget"], budget_of(policy, k))
        g["fits"] = g["fits"] and p <= budget_of(policy, k)
    listed = {(e["crate"], e["target"]) for e in policy["carving"]}
    cls = policy["classes"][policy["workerClass"]]
    exceeded = [{"key": k, "crate": parse(k)[0], "target": parse(k)[1], "kind": "final" if LINKED & set(k.split("|")[3].split(",")) else "library", "peakMegabytes": p, "budgetMegabytes": budget_of(policy, k), "listed": parse(k) in listed} for k, p in unit.items() if p > budget_of(policy, k)]
    exceeded.sort(key=lambda e: (-e["peakMegabytes"], e["key"]))
    resolved = [{"crate": e["crate"], "target": e["target"], "peakMegabytes": group[(e["crate"], e["target"])]["peak"], "budgetMegabytes": group[(e["crate"], e["target"])]["budget"]} for e in policy["carving"] if (e["crate"], e["target"]) in group and group[(e["crate"], e["target"])]["fits"]]
    resolved.sort(key=lambda e: (-e["peakMegabytes"], e["crate"], e["target"]))
    return {"version": 1, "workerClass": policy["workerClass"], "libraryMegabytes": cls["libraryMegabytes"], "finalMegabytes": cls["finalMegabytes"], "exceeded": exceeded, "resolved": resolved, "ok": all(e["listed"] for e in exceeded) and not resolved}


def fresh(tolerance, rows):
    by = {}
    for i, row in enumerate(rows):
        if "lines" in row:
            by.setdefault(row["key"], []).append((row["atMs"], i, row))
    out = []
    for key, items in by.items():
        latest = max(items, key=lambda x: (x[0], x[1]))[2]["lines"]
        for _, _, row in items:
            lo, hi = sorted((row["lines"], latest))
            ok = row["lines"] == latest if lo == 0 else hi * 10000 <= (10000 + tolerance) * lo
            if ok:
                out.append([key, row["atMs"]])
    return sorted(out)


def flag_values(tokens, flag):
    out = []
    for i, token in enumerate(tokens):
        if token == flag:
            out.append(tokens[i + 1] if i + 1 < len(tokens) else "")
        elif token.startswith(flag + "="):
            out.append(token[len(flag) + 1:])
    return out


def ceiling(policy, command):
    tokens = command.split()
    crate = (flag_values(tokens, "--crate-name") or [""])[0]
    if not crate:
        return None
    target = (flag_values(tokens, "--target") or [""])[0] or "host"
    types = [x for value in flag_values(tokens, "--crate-type") for x in value.split(",") if x] + (["bin"] if "--test" in tokens else [])
    cls = policy["classes"][policy["workerClass"]]
    final = bool(LINKED & set(types))
    base = cls["finalMegabytes"] if final else cls["libraryMegabytes"]
    entry = next((e for e in policy["carving"] if e["crate"] == crate and e["target"] == target), None)
    out = {"unit": crate + "|" + target, "kind": "final" if final else "library", "megabytes": max(entry["budgetMegabytes"], base) if entry else base, "listed": entry is not None}
    if entry:
        out["measuredMegabytes"] = entry["measuredMegabytes"]
    return out


if __name__ == "__main__":
    policy_all = json.load(open(sys.argv[1], encoding="utf-8"))["definitions"]["Policy"]["const"]
    corpus = json.load(open(sys.argv[2], encoding="utf-8"))
    print(json.dumps({"verdicts": [verdict(policy_all["unitCeiling"], c["observations"]) for c in corpus["cases"]], "fresh": [fresh(policy_all["forecastSizeToleranceBasisPoints"], c["rows"]) for c in corpus["freshCases"]], "ceilings": [ceiling(policy_all["unitCeiling"], c["command"]) for c in corpus["commands"]]}, ensure_ascii=False))
