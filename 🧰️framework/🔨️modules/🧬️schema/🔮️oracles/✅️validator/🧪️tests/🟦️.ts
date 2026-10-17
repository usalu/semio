import { test, expect } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import { addSemioMutationLeafSchemasV1, compileSchemaScopeExport, semioSchemaAjvV1 } from "../🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json";

test("portable mutation leaf registration binds exact physical directory ownership", () => {
  expect(new Set(corpus.cases.map(row => row.id)).size).toBe(corpus.cases.length);
});

test("actual authored directory URLs register every mutation leaf against an independent Ajv oracle", () => {
  if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("Mutation registration requires caller-owned SEMIO_TEST_ARTIFACT_DIR");
  const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR);
  mkdirSync(artifacts, { recursive: true });
  const root = mkdtempSync(join(artifacts, "mutation-registration-"));
  try {
    for (const row of corpus.cases) {
      const directory = join(root, row.id), leaf = join(directory, row.directory, "🧬️schema");
      mkdirSync(leaf, { recursive: true });
      const leafSchema = { $id: "https://neutral.example/leaf/" + row.kind, type: "object", additionalProperties: false, required: ["kind", "value"], properties: { kind: { const: row.kind }, value: { type: "integer" } } };
      writeFileSync(join(leaf, "🔣️.json"), JSON.stringify(leafSchema));
      const aggregate = { oneOf: [{ $ref: leafSchema.$id }] };
      const oracle = new Ajv({ strict: true }).addSchema(leafSchema).compile(aggregate);
      const authority = pathToFileURL(directory + (row.separator ? "/" : ""));
      const owned = addSemioMutationLeafSchemasV1(semioSchemaAjvV1(), authority);
      const validate = owned.compile(aggregate);
      for (const input of [{ kind: row.kind, value: row.value }, { kind: row.kind }, { kind: row.kind, value: row.value, extra: true }, { kind: "other", value: row.value }]) expect(validate(input), row.id).toBe(oracle(input));
      expect(owned.getSchema(leafSchema.$id), row.id).toBeDefined();
      expect(readFileSync(join(leaf, "🔣️.json"), "utf8"), row.id).toBe(JSON.stringify(leafSchema));
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("actual format policy matches shared asserted and annotation-only format cases", async () => {
  const { default: vectors } = await import("../../../🧪️tests/🧫️fixtures/✅️draft07-validation-vectors.json");
  for (const row of vectors.cases.filter(value => value.id === "string-formats-pinned-assertions" || value.id === "annotation-only-formats-never-reject")) {
    const ajv = semioSchemaAjvV1({ strict: false });
    for (const document of row.documents ?? []) ajv.addSchema(document);
    const validate = ajv.compile(row.input.schema);
    for (const value of row.valid) expect(validate(value), row.id).toBe(true);
    for (const value of row.invalid) expect(validate(value.instance), row.id).toBe(false);
  }
});

test("a scope export validates exactly the hand-listed rows, keeps every annotation and resolves its declared dependency", () => {
  const root = mkdtempSync(join(tmpdir(), "semio-scope-export-"));
  try {
    const dependency = { $schema: "http://json-schema.org/draft-07/schema#", $id: "https://neutral.example/dependency.json", $defs: { Weight: { type: "number", format: "double", "x-semio-state": "artifact" } } };
    const scope = { $schema: "http://json-schema.org/draft-07/schema#", $id: "https://neutral.example/scope.json", $defs: {
      Row: { type: "object", additionalProperties: false, required: ["id", "weight"], "x-semio-formats": ["json"], "x-semio-child-kind": "row", properties: { id: { type: "string", minLength: 1 }, weight: { $ref: "https://neutral.example/dependency.json#/$defs/Weight" } } },
      Flag: { type: "boolean" },
    } };
    mkdirSync(join(root, "schema"), { recursive: true });
    writeFileSync(join(root, "schema/dependency.json"), JSON.stringify(dependency));
    writeFileSync(join(root, "schema/scope.json"), JSON.stringify(scope));
    const row = compileSchemaScopeExport(root, "schema/scope.json", "Row", ["schema/dependency.json"]);
    for (const value of [{ id: "a", weight: 1.5 }, { id: "b", weight: 0 }]) expect(row(value)).toBe(true);
    for (const value of [{ id: "", weight: 1 }, { id: "a" }, { id: "a", weight: "1" }, { id: "a", weight: 1, extra: true }, null, []]) expect(row(value)).toBe(false);
    const flag = compileSchemaScopeExport(root, "schema/scope.json", "Flag");
    expect(flag(true)).toBe(true);
    expect(flag("true")).toBe(false);
    expect(() => compileSchemaScopeExport(root, "schema/scope.json", "Absent")).toThrow();
    expect(() => compileSchemaScopeExport(root, "schema/scope.json", "Row")).toThrow();
    expect(() => compileSchemaScopeExport(root, "schema/absent.json", "Row")).toThrow();
  } finally { rmSync(root, { recursive: true, force: true }); }
});
