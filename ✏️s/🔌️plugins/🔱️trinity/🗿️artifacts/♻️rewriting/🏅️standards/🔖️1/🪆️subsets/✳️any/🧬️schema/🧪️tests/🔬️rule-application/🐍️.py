"""🔬️ Check the rule vector against SQLite's relational graph projection."""

import json
import sqlite3
from pathlib import Path


def test_rule_application_oracle():
    here = Path(__file__).parent
    vector = json.loads((here / "🔣️.json").read_text(encoding="utf-8"))
    fixture = here.parents[2] / "🖼️assets" / "🎬️demo" / "🪆️working-graph" / "🔣️.json"
    nodes = json.loads(fixture.read_text(encoding="utf-8"))["nodes"]
    rule = vector["rule"]
    lhs = rule["lhs"]
    create, = rule["rhs"]["create"]
    assignment, = rule["rhs"]["set"]

    with sqlite3.connect(":memory:") as graph:
        graph.executescript("""
            CREATE TABLE node (id TEXT PRIMARY KEY, kind TEXT, name TEXT);
            CREATE TABLE edge (source TEXT, target TEXT, kind TEXT);
        """)
        graph.executemany("INSERT INTO node VALUES (?, ?, ?)", (
            (node["id"]["value"], node["kind"], node["label"]) for node in nodes
        ))
        _, match_name = lhs["whereClause"].split(" = ")
        graph.execute(
            "INSERT INTO edge SELECT id, ?, ? FROM node WHERE kind = ? AND name = ?",
            (create["rightVar"], create["edgeKind"], lhs["pattern"]["leftKind"], match_name.strip("'")),
        )
        graph.execute("INSERT INTO node VALUES (?, ?, ?)", (
            create["rightVar"], create["rightKind"], create["rightVar"],
        ))
        graph.execute("UPDATE node SET name = ? WHERE id = ?", (
            vector["bindings"][assignment["value"].removeprefix("$")], assignment["var"],
        ))
        result = graph.execute("""
            SELECT node.name, node.kind, edge.source, edge.kind
            FROM edge JOIN node ON node.id = edge.target
        """).fetchall()

    assert result == [(
        vector["expectedName"], vector["expectedKind"],
        vector["expectedSourceId"], vector["expectedEdgeKind"],
    )]


if __name__ == "__main__":
    test_rule_application_oracle()
