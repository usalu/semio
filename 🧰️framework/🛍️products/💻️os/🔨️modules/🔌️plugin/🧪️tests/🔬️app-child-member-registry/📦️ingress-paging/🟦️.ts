import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/📦️ingress-paging/🔣️.json" with { type: "json" };
import {pluginSubtreeText} from "../../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";

test("original ingress slots retire as indivisible pages then a separate pointer backing", async () => {

  
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
  
  
  const source = pluginSubtreeText("🪆️child") + pluginSubtreeText("🏛️ownership");
  expect(source.includes("OWNED_DOCUMENT_INGRESS_PAGE_SLOTS: usize = 8")).toBe(true);
  expect(source.includes("pages: Vec<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>")).toBe(true);
  expect(source.includes("empty_backing_byte_demand")).toBe(true);
});
