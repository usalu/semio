/** 🧪️ Every IFC4 mutation agrees with an independent JSON patch and restores its retained pre-state. */
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {test} from "node:test";
import {applyPatch, type Operation} from "fast-json-patch";
import {applyIfcMutation, diffIfcMutation, inverseIfcMutation, ifcMutationInverseRows, type IfcMutation} from "../🟦️.ts";
import type {IfcSnapshot} from "../../📸️snapshot/🟦️.ts";

const fixtureRoot = new URL("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/", import.meta.url);
const read = async (relative: string): Promise<unknown> => JSON.parse(await readFile(new URL(relative, fixtureRoot), "utf8"));

/** 🔢️ Decodes language-neutral JSON scalars into the owned bigint and Binary64 model. */
function model(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(model);
  if (value && typeof value === "object") {
    const row = value as Record<string, unknown>;
    return Object.fromEntries(Object.entries(row).map(([key, entry]) => {
      if (key === "id" || key === "value" && ["integer", "reference"].includes(row.kind as string)) return [key, BigInt(entry as number | string)];
      if (key === "value" && row.kind === "real") { const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, entry as number); return [key, {bits: view.getBigUint64(0)}]; }
      return [key, model(entry)];
    }));
  }
  return value;
}

/** 🔤️ Projects owned test scalars back to the language-neutral fixture representation. */
function wire(value: unknown): unknown {
  if (typeof value === "bigint") return Number(value);
  if (Array.isArray(value)) return Array.from(value, wire);
  if (value && typeof value === "object") {
    const row = value as Record<string, unknown>;
    if (row.kind === "real") { const view = new DataView(new ArrayBuffer(8)); view.setBigUint64(0, (row.value as {bits: bigint}).bits); return {kind: "real", value: view.getFloat64(0)}; }
    return Object.fromEntries(Object.entries(row).map(([key, entry]) => [key, wire(entry)]));
  }
  return value;
}

/** 📸️ Restores default omitted constituent lists for structural comparisons. */
function normalized(snapshot: IfcSnapshot): unknown {
  const output = wire(snapshot) as {entities: {complex?: unknown[]}[]};
  for (const entity of output.entities) entity.complex ??= [];
  return output;
}

/** 🧫️ The committed raw fixture as a complete owned IFC4 snapshot. */
async function before(): Promise<IfcSnapshot> {
  const snapshot = model(await read("📸️snapshot/⬅️before/🔣️.json")) as IfcSnapshot;
  for (const entity of snapshot.entities) entity.complex ??= [];
  return snapshot;
}

test("all ten mutation kinds match independent patches and restore the pre-state", async () => {
  const base = await before(), complex = await read("🦠️mutation/🔣️.json") as {complex: unknown[]};
  const cases: {mutation: unknown; patch: Operation[]}[] = [
    {mutation: {mutation: "setFileDescription", values: []}, patch: [{op: "replace", path: "/header/fileDescription", value: []}]},
    {mutation: {mutation: "setFileName", values: []}, patch: [{op: "replace", path: "/header/fileName", value: []}]},
    {mutation: {mutation: "setFileSchema", values: []}, patch: [{op: "replace", path: "/header/fileSchema", value: []}]},
    {mutation: {mutation: "insertEntity", index: 1, entity: {id: 99, name: "IFCSITE", args: [], complex: []}}, patch: [{op: "add", path: "/entities/1", value: {id: 99, name: "IFCSITE", args: [], complex: []}}]},
    {mutation: {mutation: "removeEntity", id: 1}, patch: [{op: "remove", path: "/entities/0"}]},
    {mutation: {mutation: "setEntityName", id: 1, name: "IFCSLAB"}, patch: [{op: "replace", path: "/entities/0/name", value: "IFCSLAB"}]},
    {mutation: {mutation: "setEntityArg", id: 1, index: 0, value: {kind: "string", value: "Changed"}}, patch: [{op: "replace", path: "/entities/0/args/0", value: {kind: "string", value: "Changed"}}]},
    {mutation: {mutation: "insertEntityArg", id: 1, index: 1, value: {kind: "reference", value: 2}}, patch: [{op: "add", path: "/entities/0/args/1", value: {kind: "reference", value: 2}}]},
    {mutation: {mutation: "removeEntityArg", id: 1, index: 0}, patch: [{op: "remove", path: "/entities/0/args/0"}]},
    {mutation: {mutation: "setEntityComplex", id: 1, complex: complex.complex}, patch: [{op: "replace", path: "/entities/0/complex", value: complex.complex}]},
  ];
  for (const item of cases) {
    const mutation = model(item.mutation) as IfcMutation, original = normalized(base);
    const reference = applyPatch(original, item.patch, true, false).newDocument;
    let actual = applyIfcMutation(base, mutation);
    assert.deepEqual(normalized(actual), JSON.parse(JSON.stringify(reference)));
    const inverses = inverseIfcMutation(base, mutation);
    assert.equal(inverses.length, ifcMutationInverseRows(mutation));
    for (const inverse of inverses) actual = applyIfcMutation(actual, inverse);
    assert.deepEqual(normalized(actual), original);
    assert.deepEqual(normalized(base), original);
  }
});

test("the committed complex mutation produces the exact fixture diff and post-state", async () => {
  const base = await before(), mutation = model(await read("🦠️mutation/🔣️.json")) as IfcMutation;
  assert.deepEqual(wire(diffIfcMutation(base, mutation)), await read("🔺️diff/🔣️.json"));
  assert.deepEqual(normalized(applyIfcMutation(base, mutation)), normalized(model(await read("📸️snapshot/➡️after/🔣️.json")) as IfcSnapshot));
});

test("omitted ordinary constituents, binary values and wide IDs survive mutation undo", async () => {
  const base = await before();
  delete (base.entities[0] as {complex?: unknown[]}).complex;
  const mutation: IfcMutation = {mutation: "setEntityComplex", id: 1n, complex: [{name: "IFCOBJECT", args: [{kind: "binary", value: "0F"}]}]};
  let actual = applyIfcMutation(base, mutation);
  assert.deepEqual(wire(actual.entities[0]!.complex), wire(mutation.complex));
  for (const inverse of inverseIfcMutation(base, mutation)) actual = applyIfcMutation(actual, inverse);
  assert.deepEqual(normalized(actual), normalized(base));
  base.entities[0]!.id = 18446744073709551615n;
  const removed = diffIfcMutation(base, {mutation: "removeEntity", id: base.entities[0]!.id}).entities!.removed![0]!;
  assert.equal(removed.id, 18446744073709551615n);
  assert.equal(removed.index, 0);
});

test("missing targets, invalid positions and duplicate identities are rejected", async () => {
  const base = await before();
  assert.throws(() => applyIfcMutation(base, {mutation: "removeEntity", id: 999n}));
  assert.throws(() => applyIfcMutation(base, {mutation: "setEntityArg", id: 1n, index: 999, value: {kind: "unset"}}));
  assert.throws(() => applyIfcMutation(base, {mutation: "insertEntity", index: 999, entity: {id: 99n, name: "IFCSITE", args: [], complex: []}}));
  assert.throws(() => applyIfcMutation(base, {mutation: "insertEntity", index: 0, entity: structuredClone(base.entities[0]!)}));
});
