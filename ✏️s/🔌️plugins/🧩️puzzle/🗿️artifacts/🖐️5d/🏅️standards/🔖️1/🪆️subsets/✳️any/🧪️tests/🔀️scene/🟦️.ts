import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { mutationInputAudit } from "@semio-tech/framework-manifest/mutation-input-schema";
import Ajv from "ajv";
import artifactSchema from "../../🧬️schema/🔣️.json";
import batchSchema from "../../🧬️schema/🧬️mutations/🔀️splice-scene-row-batch/🧬️schema/🔣️.json";
import spliceSchema from "../../🧬️schema/🧬️mutations/🔀️splice-scene-rows/🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔀️scene/🔣️.json";

type Scene = typeof fixture.before;
type Splice = (typeof fixture.steps)[number]["forward" | "inverse"];
type Table = "parts" | "fasteners" | "kindCompatibility";

test("The history input resolver reads every localized splice field", () => {
  const audit = mutationInputAudit(spliceSchema, id => id === artifactSchema.$id ? artifactSchema : undefined);
  expect(audit.findings).toEqual([]);
  expect(audit.inputs.map(input => input.id)).toEqual(["/table", "/start", "/deleteCount", "/parts", "/fasteners", "/kindCompatibility"]);
});

test("AJV applies the same scene splice contract to forward and inverse mutations", () => {
  const validate = new Ajv({ strict: false }).addSchema(artifactSchema).compile(spliceSchema);
  for (const { forward, inverse } of fixture.steps) {
    expect(validate(forward)).toBe(true);
    expect(validate(inverse)).toBe(true);
  }
  const edit = fixture.steps[0].forward;
  expect(validate({ ...edit, start: -1 })).toBe(false);
  expect(validate({ ...edit, deleteCount: 65 })).toBe(false);
  expect(validate({ ...edit, parts: Array(65).fill(fixture.before.parts[0]) })).toBe(false);
  expect(validate({ ...edit, fasteners: fixture.before.fasteners })).toBe(false);
});

test("AJV accepts the forward and inverse groups and enforces their group and row bounds", () => {
  const audit = mutationInputAudit(batchSchema, id => id === artifactSchema.$id ? artifactSchema : undefined);
  expect(audit.findings).toEqual([]);
  expect(audit.inputs.map(input => input.id)).toEqual(["/edits"]);
  const validate = new Ajv({ strict: false }).addSchema(artifactSchema).compile(batchSchema);
  const batch = (edits: Splice[]) => ({
    mutation: "spliceSceneRowBatch",
    edits: edits.map(({ mutation, ...edit }) => edit),
  });
  const forward = fixture.steps.map(step => step.forward);
  const inverse = fixture.steps.map(step => step.inverse).reverse();
  expect(validate(batch(forward))).toBe(true);
  expect(validate(batch(inverse))).toBe(true);
  expect(validate(batch([]))).toBe(false);
  expect(validate(batch(Array(batchSchema.properties.edits.maxItems + 1).fill(forward[0])))).toBe(false);
  expect(validate(batch([{ ...forward[0], deleteCount: 65 }]))).toBe(false);
  expect(validate(batch([{ ...forward[0], table: "fasteners" }]))).toBe(false);
});

test("SQLite matches scene splice order and restores the exact scene on undo", () => {
  using database = new Database(":memory:");
  const splice = database.query<{ value: string }, [number, number, string, string]>(`
    WITH edits(start, stop, original, replacement) AS (VALUES (?1, ?1 + ?2, ?3, ?4))
    SELECT value FROM (
      SELECT 0 AS region, key, value FROM edits, json_each(original) WHERE key < start
      UNION ALL
      SELECT 1, key, value FROM edits, json_each(replacement)
      UNION ALL
      SELECT 2, key, value FROM edits, json_each(original) WHERE key >= stop
    ) ORDER BY region, key
  `);
  const rowsFor = (value: Scene | Splice, table: Table): readonly unknown[] => {
    switch (table) {
      case "parts": return "parts" in value && Array.isArray(value.parts) ? value.parts : [];
      case "fasteners": return "fasteners" in value && Array.isArray(value.fasteners) ? value.fasteners : [];
      case "kindCompatibility": return "kindCompatibility" in value && Array.isArray(value.kindCompatibility) ? value.kindCompatibility : [];
    }
  };
  const apply = (scene: Scene, mutation: Splice): Scene => {
    const table = mutation.table as Table;
    const rows = splice.all(mutation.start, mutation.deleteCount, JSON.stringify(rowsFor(scene, table)), JSON.stringify(rowsFor(mutation, table)));
    return { ...scene, [table]: rows.map(({ value }) => JSON.parse(value)) };
  };
  const after = fixture.steps.reduce((scene, { forward }) => apply(scene, forward), fixture.before);
  expect(after).toEqual(fixture.after);
  const undone = fixture.steps.reduceRight((scene, { inverse }) => apply(scene, inverse), after);
  expect(undone).toEqual(fixture.before);
});
