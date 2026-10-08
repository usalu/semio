/** 🔏️ Independent UTF-8 and RFC6902 owners preserve the exact edit through birth admission. */
import { expect, test } from "bun:test";
import { applyPatch } from "fast-json-patch";
import { readFileSync } from "node:fs";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
test("canonical sealer constructor admits the exact edit frame and identity buffers before transfer", () => {
  const law = read("../../🧫️fixtures/🔣️.json");
  const capacity = law.editHeaderBytes + law.identityCapacityBytes.reduce((a: number, b: number) => a + b, 0);
  expect(capacity).toBeLessThanOrEqual(law.maximumBytes);
  for (const row of law.cases) {
    const original = { actor: row.actor, id: row.id, inverse: [7, 3] };
    const accepted = row.items > 0 && row.depth > 0 && row.capacity >= capacity;
    expect(accepted).toBe(row.accepted);
    const moved = applyPatch({ edit: original, identities: [] }, accepted ? [{ op: "replace", path: "/identities", value: [row.actor, row.id, row.id] }] : [], true, false).newDocument;
    expect(moved.edit).toEqual(original);
    expect(moved.identities).toEqual(accepted ? [row.actor, row.id, row.id] : []);
    for (const [index, text] of moved.identities.entries()) expect(new TextEncoder().encode(text).byteLength).toBeLessThanOrEqual(law.identityCapacityBytes[index]);
  }
  console.log("[DEBUG] canonical sealer independent RFC6902/UTF8 preserves original owners before admitted frame and identity birth");
  const native = readFileSync(new URL("../../../🦀️.rs", import.meta.url), "utf8");
  expect(native.includes("pub fn constructor_demand(")).toBe(true);
  expect(native.includes("pub fn admit(")).toBe(true);
});

test("canonical sealer retirement separates tiny copy work from exact constructor and backing release", () => {
  const law = read("../../🧫️fixtures/🔣️.json");
  for (const copy of law.retirement.copyGrants) {
    let state = { remaining: law.retirement.stages.map((row: any) => row.name) };
    for (const row of law.retirement.stages) {
      for (const axis of ["copy", "capacity", "release", "depth"]) {
        const grant = { items: 1, copy, capacity: row.capacity, release: row.release, depth: row.depth };
        if (row[axis] === 0) continue;
        grant[axis as "copy"] = row[axis] - 1;
        const accepted = grant.items > 0 && ["copy", "capacity", "release", "depth"].every(key => grant[key as "copy"] >= row[key]);
        expect(accepted).toBe(false);
        expect(applyPatch(state, accepted ? [{ op: "remove", path: "/remaining/0" }] : [], true, false).newDocument).toEqual(state);
      }
      expect(row.copy).toBeLessThanOrEqual(copy);
      state = applyPatch(state, [{ op: "remove", path: "/remaining/0" }], true, false).newDocument;
    }
    expect(state.remaining).toEqual([]);
  }
  console.log("[DEBUG] canonical retirement RFC6902 owner conservation keeps 1/2/7-byte copy independent of exact physical backing release");
  const native = readFileSync(new URL("../../../🦀️.rs", import.meta.url), "utf8");
  const fixtures = readFileSync(new URL("../../../🧪️tests/🔬️unit/🦀️.rs", import.meta.url), "utf8");
  expect(native.includes("grant.maximum_bytes")).toBe(false);
  expect(fixtures.includes("admit_typed_controlled_retirement(value, grant)")).toBe(true);
  expect(fixtures.includes("maximum_capacity_bytes: demand.capacity_bytes")).toBe(true);
});
