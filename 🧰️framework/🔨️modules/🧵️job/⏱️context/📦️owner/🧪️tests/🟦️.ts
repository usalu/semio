import { test, expect } from "bun:test";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { readFileSync, existsSync } from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("retained step context neutral ledger alias and whole-frame authority", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const original = { operation: row.operation, generation: row.generation, aliases: 1, ledger: true };
    for (let turn = 0; turn < row.turns; turn++) {
      const held = applyPatch(structuredClone(original), [{ op: "replace", path: "/aliases", value: 2 }], true).newDocument;
      expect(held.operation).toBe(original.operation);
      expect(held.generation).toBe(original.generation);
      expect(applyPatch(held, [{ op: "replace", path: "/aliases", value: 1 }], true).newDocument).toEqual(original);
    }
    expect(applyPatch(structuredClone(original), [{ op: "replace", path: "/ledger", value: false }, { op: "replace", path: "/aliases", value: 0 }], true).newDocument.ledger).toBe(false);
  }
});

test("retained context owns one ledger and uses atomic unique teardown", () => {
  const path = new URL("../🦀️.rs", import.meta.url);
  expect(existsSync(path)).toBe(true);
  if (!existsSync(path)) return;
  const source = readFileSync(path, "utf8");
  for (const method of ["birth_bytes", "context", "next_close_byte_demand", "close_step", "terminal_is_empty"]) expect(source).toContain(method);
  expect(source).toContain("Arc::try_unwrap");
  expect(source).toContain("StepContext::with_payload_ledger");
  expect(source).not.toContain("Arc::strong_count");
});
