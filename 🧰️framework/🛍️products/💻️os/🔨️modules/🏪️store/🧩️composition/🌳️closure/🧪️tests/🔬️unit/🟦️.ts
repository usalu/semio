/** 🧪️ Graphlib topology and Ajv shape independently verify recursive document closure. */
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import Ajv from "ajv";
import vectors from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import ioSchema from "../../../../../../../../🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import { OwnedDocumentClosure, type OwnedDocumentClosureInput } from "../../🟦️.ts";
import ownerSchema from "../../../../🪆️child/🏠️owner/🧬️schema/🔣️.json" with { type: "json" };
import { parseOwnerRef } from "../../../../🪆️child/🏠️owner/🧬️schema/🟦️.ts";
const graphlib = createRequire(import.meta.url)("graphlib");

function oracle(input: OwnedDocumentClosureInput): boolean {
  const graph = new graphlib.Graph({ directed: true });
  const nodes = [input.root, ...input.members];
  if (input.members.length > 1024 || nodes.some((node) => node.children.length > 64)) return false;
  for (const node of nodes) {
    if (graph.hasNode(node.reference.artifactId)) return false;
    graph.setNode(node.reference.artifactId);
  }
  for (const member of input.members) {
    if (!graph.hasNode(member.owner.parent.artifactId) || member.owner.childId !== member.reference.artifactId) return false;
    graph.setEdge(member.owner.parent.artifactId, member.reference.artifactId);
  }
  if (!graphlib.alg.isAcyclic(graph) || graphlib.alg.preorder(graph, input.root.reference.artifactId).length !== nodes.length) return false;
  for (const node of nodes) {
    const declared = node.children.map((child) => JSON.stringify([child.slot, child.childId, child.target]));
    const owned = input.members.filter((member) => member.owner.parent.artifactId === node.reference.artifactId);
    if (owned.some((member) => JSON.stringify(member.owner.parent) !== JSON.stringify(node.reference))) return false;
    const actual = owned.map((member) => JSON.stringify([member.owner.slot, member.reference.artifactId, member.reference]));
    if (JSON.stringify(declared.sort()) !== JSON.stringify(actual.sort())) return false;
  }
  return true;
}

function chain(count: number): OwnedDocumentClosureInput {
  const reference = (index: number) => ({ artifactId: "node-" + index, dialect: { artifactKind: "s.stdio.semio", standard: "v1", subset: "object" } });
  const children = (index: number) => index < count ? [{ slot: "children", childId: "node-" + (index + 1), target: reference(index + 1) }] : [];
  return { root: { reference: reference(0), children: children(0) }, members: Array.from({ length: count }, (_, offset) => ({ reference: reference(offset + 1), owner: { parent: reference(offset), slot: "children", childId: "node-" + (offset + 1) }, children: children(offset + 1) })) };
}

function validate(input: OwnedDocumentClosureInput, fuel: number): boolean {
  const source = { ...input, generation: 7 };
  const cursor = new OwnedDocumentClosure(11, 13, 7, 100);
  const zero = cursor.step(source, { operation: 11, generation: 13, maximumItems: 0, nowMicros: 1, cancelled: false });
  assert.equal(cursor.progress.steps, 0);
  assert.equal(zero.status, input.members.length > 1024 ? "rejected" : "pending");
  if (zero.status === "rejected") return false;
  for (let turn = 0; turn < 20000; turn += 1) {
    const before = cursor.progress.steps;
    const outcome = cursor.step(source, { operation: 11, generation: 13, maximumItems: fuel, nowMicros: 1, cancelled: false });
    assert(cursor.progress.steps - before <= fuel);
    if (outcome.status !== "pending") return outcome.status === "complete";
  }
  throw new Error("closure validation did not terminate");
}

/** 🌳️ A chain may exceed the per-parent child bound while respecting the global registry bound. */
export function testOwnedDocumentClosureOracle(): void {
  const ajv = new Ajv({ strict: true }).addSchema(ioSchema).addSchema(ownerSchema);
  const shape = ajv.compile(schema);
  const ownerShape = ajv.getSchema(ownerSchema.$id)!;
  for (const row of vectors.continuationCases) {
    const unchanged = ajv.compile({ const: row.beforeMembers });
    assert.equal(unchanged(row.afterMembers), false);
    const source = { ...chain(row.beforeMembers), generation: 7 };
    const cursor = new OwnedDocumentClosure(11, 13, 7, 100);
    const grant = { operation: 11, generation: 13, maximumItems: 1, nowMicros: 1, cancelled: false };
    assert.equal(cursor.step(source, grant).status, "pending");
    const before = { ...cursor.progress };
    source.members = chain(row.afterMembers).members;
    assert.deepEqual(cursor.step(source, grant), { status: "rejected", reason: row.reason });
    assert.deepEqual(cursor.progress, before);
  }
  for (const owner of vectors.ownerCases.valid) { assert(ownerShape(owner)); assert.deepEqual(parseOwnerRef(owner), owner); }
  for (const owner of vectors.ownerCases.invalid) { assert.equal(ownerShape(owner), false); assert.throws(() => parseOwnerRef(owner)); }
  for (const row of vectors.cases) {
    assert(shape(row.input), JSON.stringify(shape.errors));
    assert.equal(oracle(row.input), row.accepted, row.id + " graph oracle");
    for (const fuel of [1, 7, 64]) assert.equal(validate(row.input, fuel), row.accepted, row.id);
  }
  for (const row of vectors.chainCases) {
    const input = chain(row.members);
    assert.equal(oracle(input), row.accepted);
    assert.equal(validate(input, 1), row.accepted);
  }
  for (const row of vectors.breadthCases) {
    const input = chain(row.children);
    input.root.children = input.members.map((member) => ({ slot: "children", childId: member.reference.artifactId, target: member.reference }));
    for (const member of input.members) { member.owner.parent = input.root.reference; member.children = []; }
    assert.equal(oracle(input), row.accepted);
    assert.equal(validate(input, 1), row.accepted);
  }
  for (const patch of [{ cancelled: true }, { operation: 12 }, { generation: 14 }, { nowMicros: 100 }]) {
    const cursor = new OwnedDocumentClosure(11, 13, 7, 100);
    assert.equal(cursor.step({ ...vectors.cases[0]!.input, generation: 7 }, { operation: 11, generation: 13, maximumItems: 1, nowMicros: 1, cancelled: false, ...patch }).status, "rejected");
  }
  const stale = new OwnedDocumentClosure(11, 13, 7, 100);
  assert.equal(stale.step({ ...vectors.cases[0]!.input, generation: 8 }, { operation: 11, generation: 13, maximumItems: 1, nowMicros: 1, cancelled: false }).status, "rejected");
  console.log("[DEBUG] Recursive closure: 16 neutral graphs x3 grants, four chain limits, two breadth limits and five authority refusals agree with Ajv/Graphlib");
}
