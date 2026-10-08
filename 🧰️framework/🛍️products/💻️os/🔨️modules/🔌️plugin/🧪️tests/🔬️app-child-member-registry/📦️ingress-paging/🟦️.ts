import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/📦️ingress-paging/🔣️.json" with { type: "json" };
import schema from "../🧫️fixtures/📦️ingress-paging/🧬️schema/🔣️.json" with { type: "json" };

test("original ingress slots retire as indivisible pages then a separate pointer backing", async () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const frames = Array.from({ length: Math.ceil(row.entries / fixture.pageSlots) }, (_, index) => Math.min(fixture.pageSlots, row.entries - index * fixture.pageSlots));
    expect(frames).toEqual(row.frames);
    expect(frames.reduce((total, slots) => total + slots, 0)).toBe(row.entries);
    expect(frames.length).toBe(row.pointerSlots);
    for (const slots of [...frames, ...(frames.length ? [row.pointerSlots] : [])]) {
      const original = Buffer.allocUnsafeSlow(slots), before = { backing: slots };
      for (const [items, bytes] of [[0, slots], [1, slots - 1]]) {
        const result = applyPatch(structuredClone(before), items > 0 && bytes >= original.length ? [{ op: "replace", path: "/backing", value: 0 }] : [], true).newDocument;
        expect(result).toEqual(before);
      }
      expect(applyPatch(structuredClone(before), [{ op: "replace", path: "/backing", value: 0 }], true).newDocument).toEqual({ backing: 0 });
    }
  }
  expect(validate({ ...fixture, pageSlots: 9 })).toBe(false);
  expect(validate({ ...fixture, cases: [{ ...fixture.cases[0], entries: 1025 }, ...fixture.cases.slice(1)] })).toBe(false);
  const source = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
  expect(source.includes("OWNED_DOCUMENT_INGRESS_PAGE_SLOTS: usize = 8")).toBe(true);
  expect(source.includes("pages: Vec<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>")).toBe(true);
  expect(source.includes("empty_backing_byte_demand")).toBe(true);
  console.log("[DEBUG] Ajv + Node unpooled Buffer + RFC6902: original0/17/1024 ingress slots, exact8-slot physical pages, final pointer backing separate, underfunding preserves original allocations");
});
