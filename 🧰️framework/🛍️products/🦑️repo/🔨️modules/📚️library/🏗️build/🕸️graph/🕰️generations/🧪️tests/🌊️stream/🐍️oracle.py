"""🐍️ Independent model of the graph assembly (stdlib only): which nodes, dependencies, external nodes and unavailable projects the graph of a set of owners contains, computed from the declared corpus alone.

usage: oracle.py expect FIXTURE   prints the expected assembly of every case as JSON
       oracle.py write FIXTURE    writes the corpus and its expectations into FIXTURE (the only way the checked-in expectations are produced)"""
import json, sys


def edge_key(edge):
    return json.dumps(edge, separators=(",", ":"), ensure_ascii=False)


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
    owners.append({"name": "broken", "root": "pkg/broken", "kind": "unavailable", "node": {"root": "pkg/broken", "project": "broken", "source": "pkg/broken/project.json", "sha256": "0" * 64, "reason": "unparsable"}, "edges": []})
    owners.append({"name": "nested", "root": "pkg/d/inner/nested", "kind": "unavailable", "node": {"root": "pkg/d/inner/nested", "project": "nested", "source": "pkg/d/inner/nested/project.json", "sha256": "1" * 64, "reason": "unparsable"}, "edges": []})
    by = {row["name"]: row for row in owners}
    by["a"]["edges"] = [{"source": "a", "target": "b", "type": "static"}, {"source": "a", "target": "b", "type": "static"}, {"source": "a", "target": "npm:left", "type": "static"}]
    by["b"]["edges"] = [{"source": "b", "target": "c", "type": "static"}, {"source": "b", "target": "npm:right", "type": "static"}]
    by["c"]["edges"] = [{"source": "c", "target": "a", "type": "static"}]
    by["d"]["edges"] = [{"source": "d", "target": "e", "type": "implicit"}]
    return {
        "owners": owners,
        "global": {
            "externalNodes": {"npm:left": {"type": "npm", "name": "npm:left"}, "npm:right": {"type": "npm", "name": "npm:right"}, "npm:unused": {"type": "npm", "name": "npm:unused"}},
            "edges": [{"source": "npm:left", "target": "npm:right", "type": "static"}, {"source": "npm:right", "target": "npm:unused", "type": "static"}, {"source": "e", "target": "a", "type": "static"}, {"source": "npm:unused", "target": "c", "type": "static"}],
            "unavailable": [{"root": "pkg/loose", "source": "pkg/loose/project.json", "sha256": "2" * 64, "reason": "unparsable"}],
        },
        "cases": [
            {"name": "everything", "names": None},
            {"name": "one-project-with-an-external-closure", "names": ["a"]},
            {"name": "a-ring-without-its-neighbours", "names": ["c", "b"]},
            {"name": "unavailable-below-a-selected-root", "names": ["d", "nested"]},
            {"name": "unavailable-outside-the-selection-is-dropped", "names": ["e"]},
            {"name": "duplicates-in-the-selection-count-once", "names": ["a", "a", "b"]},
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
