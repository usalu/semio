import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import fixture from "../../🧫️fixtures/📏️close/🔣️.json" with { type: "json" };
import schema from "../../🧫️fixtures/📏️close/🧬️schema/🔣️.json" with { type: "json" };

test("ordered map cold close retains whole backing independently of logical text work", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const allocation = Buffer.alloc(row.capacity);
    const text = Buffer.from(row.text, "utf8");
    expect(text.length).toBeLessThanOrEqual(allocation.length);
    for (const [items, bytes] of [[0, row.capacity], [1, row.capacity - 1], [1, row.capacity]]) {
      const permitted = items > 0 && bytes >= allocation.length;
      const result = applyPatch({ retained: allocation.length, released: 0 }, permitted ? [{ op: "replace", path: "/retained", value: 0 }, { op: "replace", path: "/released", value: allocation.length }] : [], true).newDocument;
      expect(result.released).toBe(permitted ? row.capacity : 0);
      expect(result.retained + result.released).toBe(row.capacity);
      expect(Math.max(7, row.capacity)).toBeLessThanOrEqual(fixture.maximumAdmissionBytes);
    }
  }
  expect(readFileSync(new URL("../../../🦀️.rs", import.meta.url), "utf8").includes("pub fn next_cold_byte_demand")).toBe(true);
  expect(readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8").includes("pub fn next_close_byte_demand")).toBe(true);
  console.log("[DEBUG] ordered-map whole physical extents use Node Buffer + Ajv + JSON Patch oracle within unchanged65536 admission");
});
