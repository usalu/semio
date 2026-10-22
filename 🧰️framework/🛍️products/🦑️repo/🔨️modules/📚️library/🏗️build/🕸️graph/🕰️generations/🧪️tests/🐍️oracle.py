#!/usr/bin/env python3
"""Independent model of the synthetic workspace of the generation laws (stdlib only): the graph a full recomputation yields, which owners an edit must refresh and what a publish must be.

usage: oracle.py graph DIR              canonical graph of a real workspace directory
       oracle.py check FIXTURE.json     recompute every scenario of the fixture and compare with its expectations (exit 1 on any difference)
The model shares no code with the TypeScript implementation: hashlib for digests, json.dumps(sort_keys) for canonical bytes, posixpath for paths."""
import hashlib, json, os, posixpath, re, sys


def canon(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def sha(data):
    if isinstance(data, str):
        data = data.encode("utf8")
    return hashlib.sha256(data).hexdigest()


def load_dir(directory):
    files = {}
    for base, _, names in os.walk(directory):
        for name in names:
            path = os.path.relpath(os.path.join(base, name), directory).replace(os.sep, "/")
            if path.startswith(".") or "/." in path:
                continue
            with open(os.path.join(base, name), "rb") as handle:
                files[path] = handle.read().decode("utf8")
    return files


def model(files):
    owners = []
    for path in sorted(files):
        if path.startswith("pkg/") and posixpath.basename(path) == "project.json":
            data = json.loads(files[path])
            owners.append({"name": data["name"], "root": posixpath.dirname(path), "config": path, "package": data.get("package"), "uses": data.get("uses", [])})
    owners.sort(key=lambda owner: owner["name"])
    by_package = {owner["package"]: owner["name"] for owner in owners if owner["package"]}
    by_root = sorted(owners, key=lambda owner: -len(owner["root"]))
    nodes, edges, inputs = {}, {}, {}
    for owner in owners:
        source_dir = owner["root"] + "/src"
        names = sorted(path[len(source_dir) + 1:] for path in files if path.startswith(source_dir + "/") and "/" not in path[len(source_dir) + 1:])
        listing = names if any(path.startswith(source_dir + "/") for path in files) else None
        sources, owner_edges = [], {}
        for name in [n for n in names if n.endswith(".ts")]:
            path = source_dir + "/" + name
            sources.append({"path": path, "sha256": sha(files[path])})
            for line in files[path].split("\n"):
                match = re.match(r'^import "(.*)";$', line)
                if not match:
                    continue
                spec, target = match.group(1), None
                if spec.startswith("@syn/"):
                    target = by_package.get(spec)
                elif spec.startswith("."):
                    resolved = posixpath.normpath(posixpath.join(posixpath.dirname(path), spec))
                    target = next((row["name"] for row in by_root if resolved.startswith(row["root"] + "/")), None)
                if target and target != owner["name"]:
                    edge = {"source": owner["name"], "sourceFile": path, "target": target, "type": "static"}
                    owner_edges[canon(edge)] = edge
        uses = [{"path": path, "sha256": sha(files[path]) if path in files else None} for path in owner["uses"]]
        nodes[owner["name"]] = {"name": owner["name"], "root": owner["root"], "package": owner["package"], "sources": sources, "uses": uses}
        edges[owner["name"]] = [owner_edges[key] for key in sorted(owner_edges)]
        inputs[owner["name"]] = sha(canon({"config": files[owner["config"]], "listing": listing, "files": {s["path"]: s["sha256"] for s in sources}, "uses": {u["path"]: u["sha256"] for u in uses}}))
    env = sha(canon({"owners": sorted([owner["name"], owner["root"]] for owner in owners), "packages": sorted([owner["package"], owner["name"]] for owner in owners if owner["package"])}))
    members = sorted([path for path in files if path.startswith("pkg/") and (path.endswith("/project.json") or path.endswith(".ts"))])
    return {"owners": owners, "nodes": nodes, "edges": edges, "inputs": inputs, "env": env, "members": members}


def graph(files, names=None):
    state = model(files)
    keep = set(names) if names is not None else set(state["nodes"])
    dependencies = sorted((edge for name in keep for edge in state["edges"][name] if edge["target"] in keep), key=canon)
    return {"projects": {name: state["nodes"][name] for name in sorted(keep)}, "dependencies": dependencies, "externalNodes": {}, "unavailable": []}


def closure(files, roots):
    state, seen, queue = model(files), set(), list(roots)
    while queue:
        name = queue.pop()
        if name in seen:
            continue
        seen.add(name)
        queue.extend(edge["target"] for edge in state["edges"][name])
    return sorted(seen)


def apply_ops(files, ops):
    for op in ops:
        if op["op"] == "write":
            files[op["path"]] = op["content"]
        elif op["op"] == "delete":
            for path in [p for p in files if p == op["path"] or p.startswith(op["path"] + "/")]:
                del files[path]
        elif op["op"] == "rename":
            for path in [p for p in files if p == op["from"] or p.startswith(op["from"] + "/")]:
                files[op["to"] + path[len(op["from"]):]] = files.pop(path)


def directories(files):
    found = set()
    for path in files:
        if path.startswith("pkg/"):
            parts = path.split("/")
            for cut in range(1, len(parts)):
                found.add("/".join(parts[:cut]))
    return found


def owner_of(path, roots):
    cut = path.rfind("/")
    while cut > 0:
        if path[:cut] in roots:
            return roots[path[:cut]]
        cut = path.rfind("/", 0, cut)
    return None


def plan(previous, files, state):
    """What the store must decide: the owners (by old name) to refresh, the configs of new owners, the removed owners and whether the environment facts moved."""
    roots = {owner["root"]: owner["name"] for owner in previous["owners"]}
    configs = {owner["config"]: owner["name"] for owner in previous["owners"]}
    dirty, added, removed, env_stale = set(), [], set(), False
    for name in previous["inputs"]:
        if previous["inputs"][name] != state["inputs"].get(name) or name not in state["inputs"]:
            dirty.add(name)
    before, after = set(previous["members"]), set(state["members"])
    changes = [(path, "added", False) for path in after - before] + [(path, "removed", False) for path in before - after]
    changes += [(path, "removed", True) for path in previous["directories"] - directories(files)]
    for path, kind, directory in sorted(changes):
        config = (not directory) and posixpath.basename(path) == "project.json"
        if config and kind == "added":
            added.append(path)
        elif config and kind == "removed" and path in configs:
            removed.add(configs[path])
        elif directory and kind != "removed":
            continue
        else:
            owner = owner_of(path, roots)
            if owner is None:
                env_stale = True
            else:
                dirty.add(owner)
    if previous["env_fields"] != state["env_fields"]:
        env_stale = True
    dirty -= removed
    return dirty, added, removed, env_stale


def simulate(initial, steps):
    files, previous, generation, out = dict(initial), None, -1, []
    for step in steps:
        apply_ops(files, step["ops"])
        state = model(files)
        state["directories"] = directories(files)
        state["env_fields"] = {owner["config"]: (owner["name"], owner["package"]) for owner in state["owners"]}
        if previous is None:
            full, edges, built = sorted(state["nodes"]), [], True
        else:
            dirty, added, removed, env_stale = plan(previous, files, state)
            old_roots = {owner["name"]: owner["root"] for owner in previous["owners"]}
            roots = {old_roots[name] for name in dirty} | {posixpath.dirname(path) for path in added}
            now = {owner["root"]: owner["name"] for owner in state["owners"]}
            full = sorted(now[root] for root in roots if root in now)
            edges = sorted(name for name in state["nodes"] if name not in full) if state["env"] != previous["env"] else []
            built = bool(full or edges or removed or env_stale or added)
        if built:
            generation += 1
        out.append({"label": step["label"], "full": full, "edges": edges, "built": built, "generation": generation, "graph": graph(files)})
        previous = state
    return out


def check(path):
    fixture = json.load(open(path, encoding="utf8"))
    bad = 0
    for row in fixture["documentSizes"]:
        size = len(canon(row["value"]).encode("utf8"))
        if size != row["expandedBytes"] or (size > row["maximumBytes"]) != (row["refusal"] == "expanded"):
            bad += 1
            print("DIFF document bytes", row["name"])
    for scenario in fixture["scenarios"]:
        expected = simulate(scenario["initial"], scenario["steps"])
        for step, row in zip(scenario["steps"], expected):
            if step["expect"] != row:
                bad += 1
                print("DIFF", scenario["name"], step["label"])
    print("scenarios checked: %d, differences: %d" % (len(fixture["scenarios"]), bad))
    return 1 if bad else 0


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "graph":
        print(canon(graph(load_dir(sys.argv[2]))))
    elif len(sys.argv) == 3 and sys.argv[1] == "check":
        sys.exit(check(sys.argv[2]))
    else:
        print(__doc__)
        sys.exit(2)
