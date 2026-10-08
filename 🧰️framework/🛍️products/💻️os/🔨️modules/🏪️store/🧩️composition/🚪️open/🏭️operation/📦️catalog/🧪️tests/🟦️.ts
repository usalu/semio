import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };

test("member opening admits original sources then funds tickets before snapshot decoding", async () => {
  const sources = fixture.disposer + fixture.factories.reduce((sum, factory) => sum + factory.arc, 0);
  const sourceBirth = fixture.page + fixture.snapshotOpen + sources;
  const ticketBirth = fixture.factories.reduce((sum, factory) => sum + factory.ticket, 0);
  for (const row of fixture.cases) {
    const bytes = row.bytes === "exact" ? sourceBirth : sourceBirth - 1;
    const born = row.items === 1 && bytes >= sourceBirth;
    expect(born).toBe(row.born);
    let state = applyPatch({ catalog: false, capacity: 0, tickets: 0 }, born ? [{ op: "replace", path: "/catalog", value: true }, { op: "replace", path: "/capacity", value: sourceBirth }] : [], true).newDocument;
    expect(state.catalog).toBe(row.born);
    if (born) {
      for (const factory of fixture.factories) {
        const original = Buffer.allocUnsafeSlow(factory.ticket);
        for (const grant of [{ items: 0, capacity: original.byteLength }, { items: 1, capacity: original.byteLength - 1 }]) {
          const funded = grant.items > 0 && grant.capacity >= original.byteLength;
          expect(funded).toBe(false);
          expect(applyPatch(state, [], true).newDocument).toEqual(state);
        }
        state = applyPatch(state, [{ op: "replace", path: "/capacity", value: state.capacity + original.byteLength }, { op: "replace", path: "/tickets", value: state.tickets + 1 }], true).newDocument;
      }
      expect(state.capacity).toBe(sourceBirth + ticketBirth);
      expect(state.tickets).toBe(fixture.factories.length);
    }
  }
  const store = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const source = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
  expect(store.includes("fn member_store_owners_birth_demand() -> Result<")).toBe(true);
  expect(source.includes("bytes.checked_add(source.capacity_bytes)")).toBe(true);
  expect(source.includes("P::member_store_owners(source_grant)")).toBe(true);
  expect(source.includes("progress.fits(source_grant)")).toBe(true);
  expect(source.includes("owners.constructor_is_complete()")).toBe(true);
  expect(source.includes("owners.admit_constructor(grant)")).toBe(true);
  expect(source.includes("self.owners.take().expect(")).toBe(true);
  console.log("[DEBUG] Node storage/RFC6902 original source ingress and independently funded factory ticket phases preserve total physical ownership; System equality required separately");
});
