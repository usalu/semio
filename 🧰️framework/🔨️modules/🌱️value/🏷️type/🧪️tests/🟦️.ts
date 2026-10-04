import { test, expect } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

const owner = resolve(import.meta.dir, "..");
const read = (path: string): string => readFileSync(resolve(owner, path), "utf8");
const fixture = JSON.parse(read("🧫️fixtures/🔣️.json")), schema = JSON.parse(read("🧬️schema/🔣️.json"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const typeValid = ajv.getSchema(schema.$id + "#/$defs/valueType")!;

test("canonical type corpus closes every wire shape and classification pair", () => {
  expect(ajv.getSchema(schema.$id)!(fixture)).toBe(true);
  expect(new Set(fixture.cases.map((row: { type: number; kind: number }) => row.type + ":" + row.kind)).size).toBe(63);
  for (const row of fixture.wire) expect(typeValid(row)).toBe(true);
  for (const row of fixture.refused) expect(typeValid(row)).toBe(false);
});

test("actual neutral types match an independent schema validator", async () => {
  expect(existsSync(resolve(owner, "🟦️.ts"))).toBe(true);
  const subject = await import("../🟦️.ts");
  const control = { checkpoint() {} };
  for (const row of fixture.wire) expect(subject.readValueType(row, control)).toEqual(row);
  for (const row of fixture.refused) expect(() => subject.readValueType(row, control)).toThrow();
  for (const row of fixture.types) expect(subject.valueTypeId(row.type)).toBe(row.id);
  for (const row of fixture.cases) {
    const type = fixture.types[row.type].type, kind = fixture.kinds[row.kind];
    const accepted = type.kind === "any" ? ["boolean", "integer", "decimal", "text", "dictionary"] : type.kind === "decimal" ? ["boolean", "integer", "decimal"] : [type.kind];
    const reference = ajv.compile(type.kind === "list" || type.kind === "schema"
      ? { type: "object", properties: { kind: { const: "dictionary" }, schema: { const: type.kind === "list" ? "list" : type.of } }, required: ["kind", "schema"], additionalProperties: false }
      : { type: "object", properties: { kind: { enum: accepted }, schema: { type: "string" } }, required: ["kind"], additionalProperties: false });
    expect(reference(kind), row.name).toBe(row.accepted);
    expect(subject.valueTypeMatches(type, kind), row.name).toBe(row.accepted);
  }
});

test("typed reader refuses inherited fields, accessors and cycles with caller cancellation", async () => {
  const subject = await import("../🟦️.ts"), control = { checkpoint() {} };
  const inherited = Object.create({ kind: "boolean" }, { of: { value: null, enumerable: true } });
  const concealed = Object.defineProperty({ of: null }, "kind", { value: "boolean" });
  for (const row of [inherited, concealed]) {
    expect(typeValid(JSON.parse(JSON.stringify(row)))).toBe(false);
    expect(() => subject.readValueType(row, control)).toThrow();
  }
  let calls = 0;
  const accessor = Object.defineProperty({}, "kind", { get() { calls++; return "boolean"; }, enumerable: true });
  expect(() => subject.readValueType(accessor, control)).toThrow();
  expect(calls).toBe(0);
  const cycle: { kind: "list"; of?: unknown } = { kind: "list" };
  cycle.of = cycle;
  expect(() => subject.readValueType(cycle, control)).toThrow();
  let nested: unknown = { kind: "text" };
  for (let index = 0; index < 300; index++) nested = { kind: "list", of: nested };
  const progress: string[] = [];
  subject.readValueType(nested, { checkpoint(completed, phase) { progress.push(phase + ":" + completed); } });
  expect(progress.length).toBe(601);
  expect(progress[300]).toBe("read:300");
  expect(progress.at(-1)).toBe("construct:299");
  let checked = 0;
  expect(() => subject.readValueType(nested, { checkpoint() { if (++checked === 5) throw Error("cancelled"); } })).toThrow("cancelled");
  expect(checked).toBe(5);
});

test("canonical neutral enums and package exports have one lower owner", () => {
  expect(existsSync(resolve(owner, "🦀️.rs"))).toBe(true);
  const neutral = read("🦀️.rs");
  expect(neutral).toContain("pub enum ValueType");
  expect(neutral).toContain("pub enum ValueKind");
  expect(neutral).not.toMatch(/neural_engine|semio_framework_os|serde::/u);
  expect(read("../📦️packages/🦀️rust/🦀️.rs")).toContain("pub use types::{ValueKind, ValueType}");
});
