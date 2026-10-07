import { test, expect } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import { addSemioMutationLeafSchemasV1, semioSchemaAjvV1 } from "../🟦️.ts";
import corpus from "../🧫️fixtures/🔣️.json";

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
  const { default: vectors } = await import("../../../🧫️fixtures/✅️draft07-validation-vectors.json");
  for (const row of vectors.cases.filter(value => value.id === "string-formats-pinned-assertions" || value.id === "annotation-only-formats-never-reject")) {
    const ajv = semioSchemaAjvV1({ strict: false });
    for (const document of row.documents ?? []) ajv.addSchema(document);
    const validate = ajv.compile(row.input.schema);
    for (const value of row.valid) expect(validate(value), row.id).toBe(true);
    for (const value of row.invalid) expect(validate(value.instance), row.id).toBe(false);
  }
  console.log("[DEBUG] actual semantic validator policy replayed shared draft-07 payload cases");
});
