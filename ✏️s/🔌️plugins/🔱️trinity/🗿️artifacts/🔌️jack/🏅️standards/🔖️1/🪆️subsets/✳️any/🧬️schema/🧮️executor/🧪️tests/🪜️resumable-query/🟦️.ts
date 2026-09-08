import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import fixture from "./🔣️.json";

/** 🔎️ SQLite independently validates the neutral graph-query results and mutation counts. */
export function testResumableQueryOracle(): void {
  for (const test of fixture.cases) {
    const database = new Database(":memory:");
    try {
      database.run("CREATE TABLE nodes (id TEXT PRIMARY KEY, kind TEXT, name TEXT, label TEXT)");
      database.run("CREATE TABLE edges (id TEXT PRIMARY KEY, kind TEXT, source TEXT, target TEXT)");
      for (const node of fixture.graph.nodes) database.run("INSERT INTO nodes (id, kind, name) VALUES (?, ?, ?)", [node.id, node.kind, node.name]);
      for (const edge of fixture.graph.edges) database.run("INSERT INTO edges VALUES (?, ?, ?, ?)", [edge.id, edge.kind, edge.source.split("@")[0], edge.target.split("@")[0]]);
      const oracle = test.oracle;
      const mutations = "mutate" in oracle ? database.run(oracle.mutate).changes : 0;
      const rows = "select" in oracle ? database.query(oracle.select).values() : [];
      assert.deepEqual(rows, test.rows, test.query);
      assert.equal(mutations, test.mutations, test.query);
      if ("nodes" in oracle && "nodeIds" in test) assert.deepEqual(database.query(oracle.nodes).values().flat(), test.nodeIds, test.query);
      if ("edges" in oracle && "edgeIds" in test) assert.deepEqual(database.query(oracle.edges).values().flat(), test.edgeIds, test.query);
    } finally {
      database.close();
    }
  }
  console.log(`[DEBUG] SQLite matched ${fixture.cases.length} neutral query result and mutation cases`);
}
