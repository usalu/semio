import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import corpus from "../🧫️fixtures/🎯️set-laws/🔣️.json";
import schema from "../../🧬️schema/🎯️set-laws/🔣️.json";
import { domainTopologyAncestors, domainTopologyDescendantClosure, nextHover, nextSelection, validateState, type DomainTopology, type InteractionDefinition, type MergeMode, type SelectionSpec } from "../../🟦️.ts";

/** 🧮️ Verifies shared interaction set algebra, bounded traversal and large-batch work against SQLite. */
export function testInteractionSetLaws(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(corpus), JSON.stringify(validate.errors));
  const database = new Database(":memory:");
  database.exec("CREATE TABLE nodes(id TEXT PRIMARY KEY, parent TEXT, ordinal INTEGER); CREATE TABLE seed(id TEXT PRIMARY KEY, ordinal INTEGER); CREATE TABLE incoming(id TEXT PRIMARY KEY, ordinal INTEGER)");
  const load = (topology: DomainTopology): void => {
    database.exec("DELETE FROM nodes");
    topology.ordered.forEach((node, index) => database.query("INSERT INTO nodes VALUES (?, ?, ?)").run(node.id, node.parent ?? null, index));
  };
  const closure = (root: string): string[] => database.query("WITH RECURSIVE reachable(id) AS (SELECT id FROM nodes WHERE id = ? UNION SELECT nodes.id FROM nodes JOIN reachable ON nodes.parent = reachable.id) SELECT id FROM reachable").all(root).map(row => (row as { id: string }).id);
  const orderedClosure = (root: string): string[] => database.query("WITH RECURSIVE reachable(id, path) AS (SELECT id, printf('%08d', ordinal) FROM nodes WHERE id = ? UNION ALL SELECT nodes.id, reachable.path || '.' || printf('%08d', nodes.ordinal) FROM nodes JOIN reachable ON nodes.parent = reachable.id) SELECT id FROM reachable ORDER BY path").all(root).map(row => (row as { id: string }).id);
  const spec: SelectionSpec = { modes: ["multiple"], methods: ["pick", "rectangle"], merges: ["replace", "additive", "subtractive", "invertive"], transitive: false, broadcast: true };
  try {
    load({ ordered: corpus.topology });
    for (const row of corpus.cases) {
      database.exec("DELETE FROM seed; DELETE FROM incoming");
      row.seed.forEach((id, index) => database.query("INSERT OR IGNORE INTO seed VALUES (?, ?)").run(id, index));
      let ordinal = 0;
      for (const target of row.targets) {
        const expanded = row.transitive ? orderedClosure(target) : [target];
        for (const id of expanded.length ? expanded : [target]) database.query("INSERT OR IGNORE INTO incoming VALUES (?, ?)").run(id, ordinal++);
      }
      const keep = row.merge === "replace" ? "0" : row.merge === "additive" ? "1" : "NOT EXISTS (SELECT 1 FROM incoming WHERE incoming.id = seed.id)";
      const add = row.merge === "subtractive" ? "0" : row.merge === "replace" ? "1" : "NOT EXISTS (SELECT 1 FROM seed WHERE seed.id = incoming.id)";
      const oracle = database.query(`SELECT id FROM (SELECT id, 0 AS part, ordinal FROM seed WHERE ${keep} UNION ALL SELECT id, 1 AS part, ordinal FROM incoming WHERE ${add}) ORDER BY part, ordinal`).all().map(row => (row as { id: string }).id);
      const topology = { ordered: corpus.topology };
      const next = nextSelection({ ...spec, transitive: row.transitive }, { granularity: "node", ids: row.seed }, topology, { targets: row.targets.map(id => ({ granularity: "node", id })), merge: row.merge as MergeMode, mode: "multiple" });
      assert.deepEqual(next.ids, row.expected, row.id);
      assert.deepEqual(next.ids, oracle, `${row.id}: SQLite ordered set oracle`);
      assert.equal(next.anchorId, row.targets.at(-1), row.id);
    }
    for (const row of corpus.traversals) {
      const topology = { ordered: row.topology };
      load(topology);
      assert.deepEqual(domainTopologyDescendantClosure(topology, row.root), row.descendants, row.id);
      assert.deepEqual([...row.descendants].sort(), closure(row.root).sort(), `${row.id}: SQLite recursive union`);
      assert.deepEqual(domainTopologyAncestors(topology, row.root), row.ancestors, row.id);
    }
    const size = corpus.scale;
    const ids = Array.from({ length: size }, (_, index) => `n${index}`);
    const topology = { ordered: ids.map((id, index) => ({ id, granularity: "node", ...(index ? { parent: ids[index - 1]! } : {}) })) };
    assert.equal(domainTopologyDescendantClosure(topology, ids[0]!).length, size);
    assert.equal(domainTopologyAncestors(topology, ids.at(-1)!).length, size - 1);
    const targets = ids.flatMap(id => [{ id, granularity: "node" }, { id, granularity: "node" }]);
    assert.deepEqual(nextSelection(spec, { granularity: "node", ids }, topology, { targets, merge: "invertive", mode: "multiple" }).ids, []);
    assert.equal(nextHover({ enabled: true, transitive: true, channels: ["pointer"], broadcast: true }, topology, { targets, channel: "pointer" }).ids.length, size);
    let reads = 0;
    const counted = new Proxy(topology.ordered, { get(target, key, receiver) { if (typeof key === "string" && /^\d+$/.test(key)) reads++; return Reflect.get(target, key, receiver); } });
    const label = { native: { en: "Mesh", de: "Netz" }, reuse: { en: "Mesh", de: "Netz" } };
    const def: InteractionDefinition = { id: "mesh", label, granularities: [{ id: "node", label, iconId: "circle" }], hierarchy: { kind: "flat" }, selection: spec, hover: { enabled: true, transitive: false, channels: ["pointer"], broadcast: true } };
    const validated = validateState([def], { domains: { mesh: { ordered: counted } } }, { selection: { mesh: { granularity: "node", ids } }, hover: { mesh: { channel: "pointer", ids } }, activeMode: {}, activeGranularity: {} });
    assert.equal(validated.selection.mesh!.ids.length, size);
    assert(reads <= size * 3, `Membership validation read ${reads} nodes for ${size} identities`);
    for (const domain of corpus.domains) {
      const definitions = [{ ...def, id: domain }];
      const empty = validateState(definitions, { domains: {} }, { selection: {}, hover: {}, activeMode: {}, activeGranularity: {} });
      assert(!Object.hasOwn(empty.selection, domain));
      const present = validateState(definitions, { domains: {} }, { selection: { [domain]: { granularity: "node", ids: ["n0"] } }, hover: { [domain]: { channel: "pointer", ids: ["n0"] } }, activeMode: {}, activeGranularity: {} });
      assert(Object.hasOwn(present.selection, domain));
      assert.deepEqual(present.selection[domain]!.ids, ["n0"]);
      assert.deepEqual(present.hover[domain]!.ids, ["n0"]);
      assert.equal(Object.getPrototypeOf(present.selection), null);
    }
    console.log(`[DEBUG] Interaction set laws: ${corpus.cases.length} merges, ${corpus.traversals.length} traversals and ${size} identities agree with Ajv and SQLite`);
  } finally { database.close(); }
}
