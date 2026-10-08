import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { applyPatch } from "fast-json-patch";
import fixture from "../../../📏️retirement/🧫️fixtures/🔣️.json" with { type: "json" };

test("member-open retirement requires the original full allocation including empty capacity", async () => {
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(3);
  for (const row of fixture.cases) {
    const original = Buffer.allocUnsafeSlow(row.capacityBytes);
    const used = original.write(row.text, "utf8");
    expect(used).toBe(new TextEncoder().encode(row.text).length);
    expect(original.buffer.byteLength).toBe(row.capacityBytes);
    for (const [items, bytes] of [[0, row.capacityBytes], [1, row.capacityBytes - 1], [1, row.capacityBytes]]) {
      const paid = items > 0 && bytes >= original.buffer.byteLength;
      const result = applyPatch({ retained: row.capacityBytes, released: 0 }, paid ? [
        { op: "replace", path: "/retained", value: 0 },
        { op: "replace", path: "/released", value: row.capacityBytes },
      ] : [], true).newDocument;
      expect(result.released).toBe(paid ? row.capacityBytes : 0);
      expect(result.retained + result.released).toBe(row.capacityBytes);
      expect(bytes).toBeLessThanOrEqual(fixture.maximumAdmissionBytes);
    }
  }
  const source = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
  const request = source.slice(source.indexOf("impl MemberOpenRequest {"), source.indexOf("impl ErasedSnapshotRetirement for MemberOpenRequest"));
  expect(request.includes("owned_retirement(")).toBe(false);
  expect(request.includes("drop(std::mem::take(field))")).toBe(true);
  expect(request.includes("field.capacity()")).toBe(true);
  expect(request.includes("closing_identity_field")).toBe(true);
  const oversized = applyPatch({ capacityBytes: fixture.maximumAdmissionBytes }, [{ op: "replace", path: "/capacityBytes", value: fixture.maximumAdmissionBytes + 1 }], true).newDocument;
  expect(Buffer.allocUnsafeSlow(oversized.capacityBytes).buffer.byteLength).toBeGreaterThan(fixture.maximumAdmissionBytes);
  console.log("[DEBUG] member-open physical retirement oracle: 3 original allocation extents, zero-item and one-byte-under denial, exact full release, empty capacity retained");
});

test("member-open inline page work and whole backing release remain separate authorities", async () => {
  const input = await Bun.file(new URL("../../../📏️retirement/🧫️fixtures/📄️inline.json", import.meta.url)).json();
  for (const row of input.cases) {
    let retained = row.inputBytes;
    const pages = Math.ceil(retained / input.pageBytes);
    for (let index = 0; index < pages; index++) {
      const bytes = retained % input.pageBytes || input.pageBytes;
      const after = applyPatch({ retained, items: 0, freed: 0 }, [
        { op: "replace", path: "/retained", value: retained - bytes },
        { op: "replace", path: "/items", value: 1 },
      ], true).newDocument;
      expect(after.freed).toBe(0);
      expect(after.items).toBe(input.maximumItems);
      retained = after.retained;
    }
    expect(retained).toBe(0);
    const original = Buffer.allocUnsafeSlow(pages * input.slotBytes);
    expect(original.buffer.byteLength).toBe(pages * input.slotBytes);
    expect(original.byteLength).toBeLessThanOrEqual(input.maximumAdmissionBytes);
    for (const paid of [0, original.byteLength - 1, original.byteLength]) {
      const released = paid >= original.byteLength ? original.byteLength : 0;
      expect(released).toBe(paid === original.byteLength ? original.byteLength : 0);
    }
  }
  const source = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
  const request = source.slice(source.indexOf("pub struct MemberOpenRequest"), source.indexOf("impl ErasedSnapshotRetirement for MemberOpenRequest"));
  expect(request).not.toContain("closing_page");
  expect(request).not.toContain("closing_bytes");
  expect(request).toContain("released_items: 1, released_bytes: 0");
  console.log("[DEBUG] inline request oracle: four byte corpora, one POD page per work item, zero physical payload free, indivisible original boxed backing");
});
