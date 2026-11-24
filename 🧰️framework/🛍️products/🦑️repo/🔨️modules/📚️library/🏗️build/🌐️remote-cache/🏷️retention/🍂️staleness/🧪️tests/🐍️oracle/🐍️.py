import hashlib
import json
import sys


def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def judge(scenario):
    entries = {row["key"]: dict(row, declared=row["declared"] and row.get("version", 2) == 2) for row in scenario["entries"]}
    tree = scenario["tree"]
    toolchains = scenario["toolchains"]
    retired = {digest(name) for name in scenario["retired"]}
    now, grace = scenario["now"], scenario["grace"]
    holders = {}
    for key, row in entries.items():
        for blob in row["refs"]:
            holders.setdefault(digest(blob), set()).add(key)
    named = {}
    for key, row in entries.items():
        if row["declared"] and row.get("self") is not None:
            named.setdefault(row["self"], set()).add(key)
    stale = {}
    for key, row in entries.items():
        if not row["declared"]:
            continue
        if toolchains is not None and row["toolchain"] not in toolchains:
            stale[key] = "toolchain"
            continue
        for path, content in sorted(row["sources"].items()):
            if path not in tree:
                stale[key] = "deleted"
                break
            if digest(tree[path]) != digest(content):
                stale[key] = "edited"
                break
    changed = True
    while changed:
        changed = False
        for key, row in entries.items():
            if not row["declared"] or key in stale:
                continue
            for blob in row["externs"]:
                names = holders.get(digest(blob), set()) - {key}
                if (not names and digest(blob) in retired) or (names and all(name in stale for name in names)):
                    stale[key] = "extern"
                    changed = True
                    break
            if key in stale:
                continue
            for name in row.get("units", []):
                names = named.get(name, set()) - {key}
                if names and all(other in stale for other in names):
                    stale[key] = "extern"
                    changed = True
                    break
    young = lambda row: now - row["storedAt"] < grace
    result = {}
    roots = set()
    for key, row in entries.items():
        if not row["declared"]:
            result[key] = "undeclared"
        elif row.get("kind") == "graph":
            result[key] = "live"
            roots.add(key)
        elif key in stale and not young(row):
            result[key] = "stale:" + stale[key]
        elif row["sources"] or row.get("kind") == "stage-output":
            result[key] = "grace" if key in stale else "live"
            roots.add(key)
    reached, frontier = set(), list(roots)
    while frontier:
        row = entries[frontier.pop()]
        found = set()
        for blob in row["externs"]:
            found |= holders.get(digest(blob), set())
        for unit in row.get("units", []):
            found |= named.get(unit, set())
        for name in found:
            other = entries[name]
            if name in result or name in reached or not other["declared"] or other["sources"] or other.get("kind") in ("stage-output", "graph"):
                continue
            reached.add(name)
            frontier.append(name)
    for key, row in entries.items():
        if key in result:
            continue
        result[key] = "live" if key in reached else "grace" if young(row) else "unreached"
    return result


if __name__ == "__main__":
    document = json.load(sys.stdin)
    json.dump([judge(scenario) for scenario in document["scenarios"]], sys.stdout)
