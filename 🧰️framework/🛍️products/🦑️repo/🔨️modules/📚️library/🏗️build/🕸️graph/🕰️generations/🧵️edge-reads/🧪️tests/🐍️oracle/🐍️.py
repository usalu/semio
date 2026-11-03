"""🧵️ Independent model of which owners keep their edges when the environment moves (stdlib only, no indexes, every rule as a plain predicate over every pair).

Reads `{"worlds": [{previous, current, rows, tables}, ...]}` on stdin and prints `{"answers": [...]}`, one answer per world. A world is: `previous` and `current` are environment parts (`owners` as `[name, root, native root or null]`, `packages`, `modules` and `workspaces` as pairs, `lock`, `taxonomy`), `rows` maps each candidate owner to `{root, nativeRoot?, reads?}`, `tables` is the old and the new owner table (name to `[root, native root]`). An answer is `{"all": bool, "affected": [...]}` with the sorted names of the owners that may not keep their edges."""
import json
import sys


def inside(root, path):
    """A path is inside a root when it equals it or lies below it; the root `.` contains everything."""
    return root == "." or path == root or path.startswith(root + "/")


def directory_of(path):
    return path.rsplit("/", 1)[0] if "/" in path else "."


def pairs_by_key(pairs):
    found = {}
    for key, value in pairs:
        found.setdefault(key, set()).add(value)
    return found


def changed_keys(before, after):
    a, b = pairs_by_key(before), pairs_by_key(after)
    return {key for key in set(a) | set(b) if a.get(key, set()) != b.get(key, set())}


def model(previous, current, rows, tables):
    if previous is None or previous["taxonomy"] != current["taxonomy"]:
        return True, sorted(rows)
    old = {row[0]: row for row in previous["owners"]}
    new = {row[0]: row for row in current["owners"]}
    names, roots = set(), set()
    for name in set(old) | set(new):
        if name in old and name in new and old[name][1:] == new[name][1:]:
            continue
        names.add(name)
        for row in (old.get(name), new.get(name)):
            if row:
                roots.update(root for root in row[1:] if root is not None)
    packages = changed_keys(previous["packages"], current["packages"])
    modules = changed_keys(previous["modules"], current["modules"])
    before, after = dict(previous["workspaces"]), dict(current["workspaces"])
    workspaces = {directory_of(path) for path in set(before) | set(after) if before.get(path) != after.get(path)}
    lock = previous["lock"] != current["lock"]
    affected = set()
    for name, row in rows.items():
        reads = row.get("reads")
        if reads is None:
            affected.add(name)
            continue
        own = [root for root in (row["root"], row.get("nativeRoot")) if root]
        resolved = reads.get("owners", [])
        territory = set()
        for target in resolved:
            territory.update(root for root in [rows.get(target, {}).get("root"), rows.get(target, {}).get("nativeRoot")] if root)
            for table in tables:
                territory.update(root for root in table.get(target, []) if root)
        if names & set(resolved) or packages & set(reads.get("packages", [])) or modules & set(reads.get("modules", [])) or (lock and reads.get("lock")):
            affected.add(name)
            continue
        for root in roots:
            if any(inside(mine, root) for mine in own) or any(inside(owned, root) for owned in territory) or any(inside(d, root) or inside(root, d) for d in reads.get("paths", [])):
                affected.add(name)
                break
        if name in affected:
            continue
        reads_cargo = "workspaces" in reads
        for directory in workspaces:
            if directory in reads.get("workspaces", []) or (reads_cargo and any(inside(mine, directory) or inside(directory, mine) for mine in own)):
                affected.add(name)
                break
    return False, sorted(affected)


if __name__ == "__main__":
    answers = []
    for world in json.load(sys.stdin)["worlds"]:
        everything, affected = model(world["previous"], world["current"], world["rows"], world["tables"])
        answers.append({"all": everything, "affected": affected})
    print(json.dumps({"answers": answers}))
