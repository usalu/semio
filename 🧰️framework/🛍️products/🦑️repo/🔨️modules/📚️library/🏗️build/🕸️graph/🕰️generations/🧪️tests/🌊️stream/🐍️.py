"""🐍️ Independent model of the graph assembly (stdlib only): which nodes, dependencies, external nodes and unavailable projects the graph of a set of owners contains, computed from the declared corpus alone.

usage: 🐍️.py expect FIXTURE   prints the expected assembly of every case as JSON
       🐍️.py write FIXTURE    writes the corpus and its expectations into FIXTURE (the only way the checked-in expectations are produced)"""
import json, sys


def edge_key(edge):
    return json.dumps(edge, separators=(",", ":"), ensure_ascii=False)


def named_dependencies(value):
    if isinstance(value, list):
        for item in value:
            yield from named_dependencies(item)
    elif isinstance(value, dict):
        for key, item in value.items():
            if key == "externalDependencies" and isinstance(item, list):
                yield from (name for name in item if isinstance(name, str))
            else:
                yield from named_dependencies(item)


def assemble(corpus, names):
    owners = {row["name"]: row for row in corpus["owners"]}
    wanted = sorted(set(names)) if names is not None else sorted(owners)
    keep = set(wanted) if names is not None else None
    glob = corpus["global"]
    external_nodes = glob["externalNodes"]
    edges = {}
    for name in wanted:
        for edge in owners[name]["edges"]:
            if keep is None or edge["target"] in keep or edge["target"] in external_nodes:
                edges[edge_key(edge)] = edge
    reached = set()
    if keep is None:
        for edge in glob["edges"]:
            edges[edge_key(edge)] = edge
    else:
        reached = {edge["target"] for edge in edges.values() if edge["target"] in external_nodes}
        for name in wanted:
            for dependency in named_dependencies(owners[name]["node"]):
                key = dependency if dependency in external_nodes else "npm:" + dependency
                if key in external_nodes:
                    reached.add(key)
        grown = True
        while grown:
            grown = False
            for edge in glob["edges"]:
                if (edge["source"] in keep or edge["source"] in reached) and (edge["target"] in keep or edge["target"] in external_nodes):
                    edges[edge_key(edge)] = edge
                    if edge["target"] in external_nodes and edge["target"] not in reached:
                        reached.add(edge["target"])
                        grown = True
    roots = [owners[name]["root"] for name in wanted]
    rows = list(glob["unavailable"]) + [owners[name]["node"] for name in wanted if owners[name]["kind"] == "unavailable"]
    if keep is not None:
        rows = [row for row in rows if any(row["root"] == root or row["root"].startswith(root + "/") for root in roots)]
    return {
        "projects": {name: owners[name]["node"] for name in wanted if owners[name]["kind"] == "project"},
        "dependencies": [edges[key] for key in sorted(edges)],
        "externalNodes": {key: value for key, value in external_nodes.items() if keep is None or key in reached},
        "unavailable": sorted(rows, key=lambda row: row["root"]),
    }


def corpus():
    def project(name, root, targets):
        return {"name": name, "root": root, "kind": "project", "node": {"name": name, "root": root, "targets": {target: {"execution": {"kind": "command", "command": "run " + name + " " + target}} for target in targets}}, "edges": []}
    owners = [project("a", "pkg/a", ["build", "test"]), project("b", "pkg/b", ["build"]), project("c", "pkg/c", ["build"]), project("d", "pkg/d/inner", ["build"]), project("e", "pkg/e", [])]
    owners.append(project("f", "pkg/f", ["build"]))
    owners[-1]["node"]["targets"]["build"]["inputs"] = ["{projectRoot}/src.ts", {"externalDependencies": ["npm:named", "npm:missing"]}]
    owners.append(project("g", "pkg/g", ["build"]))
    owners[-1]["node"]["namedInputs"] = {"commandSources0": ["{workspaceRoot}/x.ts", {"externalDependencies": ["npm:grouped"]}]}
    owners[-1]["node"]["targets"]["build"]["inputs"] = ["commandSources0"]
    owners.append({"name": "broken", "root": "pkg/broken", "kind": "unavailable", "node": {"root": "pkg/broken", "project": "broken", "source": "pkg/broken/project.json", "sha256": "0" * 64, "reason": "unparsable"}, "edges": []})
    owners.append({"name": "nested", "root": "pkg/d/inner/nested", "kind": "unavailable", "node": {"root": "pkg/d/inner/nested", "project": "nested", "source": "pkg/d/inner/nested/project.json", "sha256": "1" * 64, "reason": "unparsable"}, "edges": []})
    owners.append(project("h", "pkg/h", ["build"]))
    owners[-1]["node"]["targets"]["build"]["inputs"] = ["{projectRoot}/src.ts", {"externalDependencies": ["plain", "npm:named"]}]
    by = {row["name"]: row for row in owners}
    by["a"]["edges"] = [{"source": "a", "target": "b", "type": "static"}, {"source": "a", "target": "b", "type": "static"}, {"source": "a", "target": "npm:left", "type": "static"}]
    by["b"]["edges"] = [{"source": "b", "target": "c", "type": "static"}, {"source": "b", "target": "npm:right", "type": "static"}]
    by["c"]["edges"] = [{"source": "c", "target": "a", "type": "static"}]
    by["d"]["edges"] = [{"source": "d", "target": "e", "type": "implicit"}]
    return {
        "owners": owners,
        "global": {
            "externalNodes": {"npm:left": {"type": "npm", "name": "npm:left"}, "npm:right": {"type": "npm", "name": "npm:right"}, "npm:unused": {"type": "npm", "name": "npm:unused"}, "npm:named": {"type": "npm", "name": "npm:named"}, "npm:plain": {"type": "npm", "name": "npm:plain"}, "npm:below": {"type": "npm", "name": "npm:below"}, "npm:grouped": {"type": "npm", "name": "npm:grouped"}},
            "edges": [{"source": "npm:left", "target": "npm:right", "type": "static"}, {"source": "npm:right", "target": "npm:unused", "type": "static"}, {"source": "e", "target": "a", "type": "static"}, {"source": "npm:unused", "target": "c", "type": "static"}, {"source": "npm:named", "target": "npm:below", "type": "static"}],
            "unavailable": [{"root": "pkg/loose", "source": "pkg/loose/project.json", "sha256": "2" * 64, "reason": "unparsable"}],
        },
        "cases": [
            {"name": "everything", "names": None},
            {"name": "one-project-with-an-external-closure", "names": ["a"]},
            {"name": "a-ring-without-its-neighbours", "names": ["c", "b"]},
            {"name": "unavailable-below-a-selected-root", "names": ["d", "nested"]},
            {"name": "unavailable-outside-the-selection-is-dropped", "names": ["e"]},
            {"name": "duplicates-in-the-selection-count-once", "names": ["a", "a", "b"]},
            {"name": "an-external-dependency-named-by-an-input-without-an-edge", "names": ["f"]},
            {"name": "an-external-dependency-named-by-an-input-group-without-an-edge", "names": ["g"]},
            {"name": "an-external-dependency-named-by-its-package-name-without-an-edge", "names": ["h"]},
        ],
    }


def expectations(fixture):
    return [assemble(fixture, case["names"]) for case in fixture["cases"]]


if __name__ == "__main__":
    command, path = sys.argv[1], sys.argv[2]
    if command == "write":
        fixture = {"version": 1, **corpus()}
        fixture["expect"] = expectations(fixture)
        open(path, "w", encoding="utf8").write(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n")
    elif command == "expect":
        print(json.dumps(expectations(json.load(open(path, encoding="utf8"))), ensure_ascii=False))
    else:
        raise SystemExit("unknown command")
