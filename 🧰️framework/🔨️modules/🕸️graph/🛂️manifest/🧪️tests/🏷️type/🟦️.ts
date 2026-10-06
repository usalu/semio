/** 🕸️ Proves Graph-owned type acceptance with its actual canonical lower type snapshot. */
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import fixture from "../../🧫️fixtures/🏷️type/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../..");
const ajv = new Ajv({strict: true, allErrors: true});

test("Graph has a closed property corpus with the exact canonical type snapshot", () => {
  const lower = JSON.parse(readFileSync(resolve(root, "🧰️framework/🔨️modules/🌱️value/🏷️type/🧫️fixtures/🔣️.json"), "utf8"));
  expect(fixture.types).toEqual(lower.types);
  
});

test("Graph binds the canonical lower type without a Neural product API", () => {
  const graph = readFileSync(resolve(root, "🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs"), "utf8");
  expect(graph).not.toMatch(/neural_engine|property_value_to_neural/u);
  expect(graph).toContain("if matches!(expected, ValueType::Any)");
  expect(graph).toContain("PropertyValue::Object(_) if matches!(expected, ValueType::Schema(_)) => true");
  const manifest = toml.parse(readFileSync(resolve(root, "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml"), "utf8"));
  expect((manifest.dependencies as Record<string, unknown>).neural_engine).toBeUndefined();
});

test("Graph retains its explicit property acceptance in an independent schema corpus", () => {
  expect(new Set(fixture.graphCases.map((row: { type: number; value: number }) => row.type + ":" + row.value)).size).toBe(63);
  for (const row of fixture.graphCases) {
    const candidate = fixture.types[row.type];
    expect(candidate).toBeDefined();
    if (!candidate) throw new Error("Graph type fixture index is outside its declared corpus");
    const type = candidate.type;
    const reference = ajv.compile(type.kind === "any" ? {} : type.kind === "boolean" ? { type: "boolean" } : type.kind === "decimal" ? { anyOf: [{ type: "boolean" }, { type: "number" }] } : type.kind === "text" ? { type: "string" } : type.kind === "schema" ? { type: "object" } : false);
    expect(reference(fixture.graphValues[row.value]), row.name).toBe(row.accepted);
  }
});
