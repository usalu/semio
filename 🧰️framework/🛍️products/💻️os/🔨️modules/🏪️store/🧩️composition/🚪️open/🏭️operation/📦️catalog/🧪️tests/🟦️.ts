import { expect, test } from "bun:test";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("member opening admits one original catalog before snapshot decoding and retains it on cancellation", async () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const catalog = fixture.disposer + fixture.factories.reduce((sum, factory) => sum + factory.arc + factory.ticket, 0);
  const exact = fixture.page + fixture.snapshotOpen + catalog;
  for (const row of fixture.cases) {
    const bytes = row.bytes === "exact" ? exact : exact - 1;
    const born = row.items === 1 && bytes >= exact;
    expect(born).toBe(row.born);
    expect(applyPatch({ catalog: false }, born ? [{ op: "replace", path: "/catalog", value: true }] : [], true).newDocument.catalog).toBe(row.born);
  }
  const store = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const source = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
  expect(store.includes("fn member_store_owners_birth_bytes() -> usize;")).toBe(true);
  expect(source.includes("checked_add(P::member_store_owners_birth_bytes())")).toBe(true);
  expect(source.includes("owners: ManuallyDrop::new(Some(P::member_store_owners()))")).toBe(true);
  expect(source.match(/P::member_store_owners\(\)/g)?.length).toBe(1);
  expect(source.includes("self.owners.take().expect(")).toBe(true);
  console.log("[DEBUG] Ajv/Node additive concrete Arc+ticket+Box constructor law and RFC6902 ownership; exactquery before one original birth, no cancellation catalog allocation");
});
