import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import fixture from "../../🧫️fixtures/🪜️resumable-query/🔣️.json";

/** 🔗️ The node half of an `<node>@<port>` endpoint — the whole value when it names no port. */
const endpointNode = (endpoint: string): string => endpoint.slice(0, endpoint.indexOf("@") === -1 ? endpoint.length : endpoint.indexOf("@"));

/** 🔎️ SQLite independently validates the neutral graph-query results and mutation counts. */
export function testResumableQueryOracle(): void {
  const executor = readFileSync(resolve(import.meta.dir, "../../🦀️.rs"), "utf8");
  const execution = readFileSync(resolve(import.meta.dir, "../../🪜️execution/🦀️.rs"), "utf8");
  const text = readFileSync(resolve(import.meta.dir, "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"), "utf8");
  assert.match(executor, /pub fn execute\(/u);
  assert.doesNotMatch(text, /fn (?:execute|match_patterns|match_pattern|eval_expr|build_return)\(/u);
  assert.doesNotMatch(execution, /json_string_bytes|property_json_upper_bound|metadata_output_upper_bound|saturating_mul\(6\)/u);
  const ownership = new Database(":memory:");
  try {
    const result = fixture.resultOwnership;
    result.texts.forEach((text, index) => {
      const cell = text.repeat(result.cellBytes);
      const [owned, encoded] = ownership.query("SELECT length(CAST(? AS BLOB)), length(json_quote(?))").values(cell, cell)[0] as [number, number];
      assert.equal(owned, result.cellBytes);
      assert.equal(encoded <= result.maximumEncodedBytes, result.textAdmission[index]);
      assert.equal(result.typedAdmission[index], true);
      assert.equal(JSON.stringify(cell).length, encoded);
    });
  } finally { ownership.close(); }
  for (const test of fixture.cases) {
    const database = new Database(":memory:");
    try {
      database.run("CREATE TABLE nodes (id TEXT PRIMARY KEY, kind TEXT, name TEXT, label TEXT)");
      database.run("CREATE TABLE edges (id TEXT PRIMARY KEY, kind TEXT, source TEXT, target TEXT)");
      for (const node of fixture.graph.nodes) database.run("INSERT INTO nodes (id, kind, name) VALUES (?, ?, ?)", [node.id, node.kind, node.name]);
      for (const edge of fixture.graph.edges) database.run("INSERT INTO edges VALUES (?, ?, ?, ?)", [edge.id, edge.kind, endpointNode(edge.source), endpointNode(edge.target)]);
      // 🧭️ The six fixture cases carry four different oracle shapes, so the imported JSON's element
      // type declares every statement OPTIONAL. `"mutate" in oracle` answers the key's presence but
      // leaves `oracle.mutate` `string | undefined`; reading it through a local binding is what makes
      // the absent case a branch the compiler can see rather than an `undefined` handed to SQLite.
      const oracle: Partial<Record<"cascade" | "mutate" | "select" | "nodes" | "edges", string>> = test.oracle;
      if (oracle.cascade !== undefined) database.run(oracle.cascade);
      const mutations = oracle.mutate === undefined ? 0 : database.run(oracle.mutate).changes;
      const rows = oracle.select === undefined ? [] : database.query(oracle.select).values();
      assert.deepEqual(rows, test.rows, test.query);
      assert.equal(mutations, test.mutations, test.query);
      if ("postNodeCount" in test) assert.equal(database.query("SELECT COUNT(*) FROM nodes").values()[0][0], test.postNodeCount);
      if ("postEdgeCount" in test) assert.equal(database.query("SELECT COUNT(*) FROM edges").values()[0][0], test.postEdgeCount);
      if (oracle.nodes !== undefined && "nodeIds" in test) assert.deepEqual(database.query(oracle.nodes).values().flat(), test.nodeIds, test.query);
      if (oracle.edges !== undefined && "edgeIds" in test) assert.deepEqual(database.query(oracle.edges).values().flat(), test.edgeIds, test.query);
    } finally {
      database.close();
    }
  }
  const database = new Database(":memory:");
  try {
    database.run("CREATE TABLE selections (window_id TEXT PRIMARY KEY, start INTEGER, end INTEGER)");
    for (const selection of fixture.interleaving.selections) database.run("INSERT INTO selections VALUES (?, ?, ?)", [selection.windowId, selection.start, selection.end]);
    assert.deepEqual(database.query("SELECT window_id, start, end FROM selections ORDER BY window_id").values(), fixture.interleaving.expected.selectionRows);
    assert.equal(fixture.interleaving.expected.appTransientPublications, fixture.interleaving.expected.queryResults);
    assert.equal(fixture.interleaving.expected.windowTransientPublications, fixture.interleaving.selections.length);
    assert.deepEqual(Object.values(fixture.interleaving.expected.windowTransientGenerationById), fixture.interleaving.selections.map(() => 1));
  } finally {
    database.close();
  }
  const retained = new Database(":memory:");
  try {
    retained.run("CREATE TABLE source (id INTEGER PRIMARY KEY, live INTEGER NOT NULL)");
    retained.run("CREATE TABLE progress (unit INTEGER PRIMARY KEY)");
    retained.run("CREATE TABLE checkpoint_owners (operation_id INTEGER NOT NULL, generation INTEGER NOT NULL, PRIMARY KEY (operation_id, generation))");
    retained.run("INSERT INTO source VALUES (1, 1)");
    for (let unit = 1; unit <= fixture.retainedExecution.cancelAfterWorkUnits; unit += fixture.retainedExecution.maximumWorkUnitsPerStep) retained.run("INSERT INTO progress VALUES (?)", [unit]);
    assert.equal(retained.query("SELECT COUNT(*) FROM progress").values()[0][0], fixture.retainedExecution.cancelAfterWorkUnits);
    retained.run("DELETE FROM progress");
    assert.equal(retained.query("SELECT live FROM source WHERE id = 1").values()[0][0], fixture.retainedExecution.sourceRemainsLive ? 1 : 0);
    assert.equal(fixture.retainedExecution.retirementItemsPerStep, 1);
    assert.equal(fixture.retainedExecution.retirementBytesPerStep, fixture.retainedExecution.maximumQueryBytes);
    const checkpoint = fixture.retainedExecution.checkpointOwnership;
    retained.run("INSERT INTO checkpoint_owners VALUES (?, ?), (?, ?)", [checkpoint.operationId, checkpoint.generation, checkpoint.collisionOperationId, checkpoint.collisionGeneration]);
    assert.equal(retained.query("SELECT COUNT(*) FROM checkpoint_owners").values()[0][0], 2);
    const rotate = (value: bigint, shift: bigint): bigint => ((value << shift) | (value >> (64n - shift))) & ((1n << 64n) - 1n);
    const tool = 0x7275_6e51_7565_7279n;
    const identity = (operation: number, generation: number): bigint => tool ^ rotate(BigInt(operation), 19n) ^ rotate(BigInt(generation), 41n);
    assert.equal(identity(checkpoint.operationId, checkpoint.generation), identity(checkpoint.collisionOperationId, checkpoint.collisionGeneration));
    assert.equal(checkpoint.exactTupleRequired, true);
    const replayMaximum = checkpoint.maximumFactors.map(BigInt).reduce((product, factor) => product * factor) + 1_000_000n;
    assert.ok(BigInt(checkpoint.legalReplayProgress) < replayMaximum);
    const entity = fixture.retainedExecution.entityAdmission;
    assert.ok(Number(retained.query("SELECT length(?)").values("x".repeat(entity.oversizedPropertyBytes))[0][0]) > entity.maximumBytesPerStep);
    assert.equal(entity.nodeRejectedBeforeClone && entity.edgeRejectedBeforeClone, true);
    retained.run("CREATE TABLE output_cells (row_id INTEGER PRIMARY KEY, value TEXT NOT NULL)");
    const output = fixture.retainedExecution.outputAdmission;
    retained.run("WITH RECURSIVE rows(id) AS (SELECT 1 UNION ALL SELECT id + 1 FROM rows WHERE id < ?) INSERT INTO output_cells SELECT id, ? FROM rows", [output.rowCount, "x".repeat(output.cellBytes)]);
    const encodedCells = Number(retained.query("SELECT length(json_group_array(value)) FROM output_cells").values()[0][0]);
    assert.ok(encodedCells > output.maximumBytes);
    const ownedText = Number(retained.query("SELECT sum(length(CAST(value AS BLOB))) FROM output_cells").values()[0][0]);
    assert.ok(ownedText > output.maximumBytes);
    assert.equal(output.oversizedTableRejected, true);
  } finally {
    retained.close();
  }
  console.log("[DEBUG] Jack query neutral ownership + graph mutations oracle=SQLite/JSON1");
}
