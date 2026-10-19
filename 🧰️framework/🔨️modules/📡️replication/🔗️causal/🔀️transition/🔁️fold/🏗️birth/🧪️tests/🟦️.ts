import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { applyPatch } from "fast-json-patch";
import Ajv from "ajv";
import { blake3 } from "@noble/hashes/blake3.js";
import { blake3Hex } from "@semio-tech/framework/hash";

test("fold backing birth preserves every independent refusal before allocating source storage", () => {
  const law = JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  expect(new Ajv({ strict: true }).validate(schema, law)).toBe(true);
  for (const count of law.lengths) for (const width of law.elementBytes) {
    const bytes = count * width;
    expect(bytes).toBe(Buffer.alloc(count * width).byteLength);
    const source = { items: 1, copy: 48, capacity: bytes + 128, depth: 1 };
    for (const axis of law.axes) {
      const denied = applyPatch(source, [{ op: "replace", path: `/${axis}`, value: 0 }], true, false).newDocument;
      expect(denied[axis as keyof typeof source]).toBe(0);
      expect(source[axis as keyof typeof source]).toBeGreaterThan(0);
    }
  }
  const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  const birth = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("pending_construction_capacity");
  expect(birth).toContain("struct FoldBackingBirth");
  expect(source).toContain("control.vector(");
  expect(source).not.toMatch(/control\.track\((?:Vec(?:::<[^>]+>)?|String)::with_capacity/);
  expect(source).not.toContain("chain.reserve_exact(source.len())");
  expect(birth).toContain("control.hash_start().await");
  expect(birth).toContain("control.hash_bytes(&mut hash");
  expect(birth).toContain("control.hash_finish(&hash).await");
  const original = readFileSync(new URL("../🎟️original/🦀️.rs", import.meta.url), "utf8");
  expect(original).toContain("native.receive_construction_step_typed");
  expect(original).toContain("runner: &mut Option<R>");
  expect(original).toContain("self.original_output = Some(ControlledRetirement::retain(value))");
  expect(original).toContain("primary: result.err(), quote_refusal, allocation_refusal");
  expect(original).toContain("take_original_output");
  expect(original).toContain("native.ledger_identity(), native.retirement_recipient_identity()");
  expect(original).not.toContain("observe_heap_allocations_on_this_thread");
  console.log("[DEBUG] Node Buffer and RFC6902 independently validate all fold backing extents and denied currencies; actual native allocator laws are separate");
});

test("streamed trunk framing preserves first-party and independent BLAKE3 identities", () => {
  for (const text of ["", "artifact", "🧩️/ä/共同", "x".repeat(65537)]) {
    const parts = ["semio.history.trunk", text].map(value => Buffer.from(value, "utf8"));
    const wire: Buffer[] = [];
    const independent = blake3.create();
    for (const part of parts) {
      let length = BigInt(part.byteLength);
      const prefix: number[] = [];
      do { prefix.push(Number(length & 127n) | (length >= 128n ? 128 : 0)); length >>= 7n; } while (length);
      wire.push(Buffer.from(prefix), part);
      independent.update(Buffer.from(prefix));
      for (let at = 0; at < part.byteLength; at += 4096) independent.update(part.subarray(at, at + 4096));
    }
    const actual = `trunk-${blake3Hex(Buffer.concat(wire)).slice(0, 16)}`;
    const expected = `trunk-${Buffer.from(independent.digest()).toString("hex").slice(0, 16)}`;
    expect(actual).toBe(expected);
    expect(actual).toHaveLength(22);
  }
  console.log("[DEBUG] first-party BLAKE3 and noble streaming agree on exact persisted trunk wire identities including Unicode and multipage input");
});
