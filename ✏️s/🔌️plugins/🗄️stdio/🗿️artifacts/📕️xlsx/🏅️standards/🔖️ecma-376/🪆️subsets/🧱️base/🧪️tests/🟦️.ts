import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

const base = resolve(import.meta.dir, "..");
const fixtures = resolve(import.meta.dir, "🧫️fixtures");
const schema = resolve(base, "🧬️schema");

function readJson(directory: string, file: string) {
  return JSON.parse(readFileSync(resolve(directory, file), "utf8"));
}

test("worksheet viewport dimensions match the shared viewer fixture", () => {
  const fixture = readJson(fixtures, "🪟️viewer-cell-window/🔣️.json");
  expect(fixture.expectedKeys).toHaveLength(fixture.rows);
  expect(fixture.expectedCells).toHaveLength(fixture.rows);
  for (const row of fixture.expectedCells) expect(row).toHaveLength(fixture.columns);
});

test("unchanged workbook cell drafts satisfy the set-cell value schema", () => {
  const fixture = readJson(fixtures, "✍️unchanged-cell-draft/🔣️.json");
  const cell = readJson(schema, "🧬️mutations/✍️set-cell/🧬️schema/🔣️.json");
  const ajv = new Ajv({ strict: true, allErrors: true })
    .addKeyword({ keyword: "x-semio-state", schemaType: "string" })
    .addKeyword({ keyword: "x-semio-ui", schemaType: "object" })
    .addSchema(readJson(base, "../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"))
    .addSchema(readJson(schema, "📸️snapshot/🔣️.json"))
    .addSchema(readJson(schema, "🧬️mutations/🧭️cell-address/🔣️.json"))
    .addSchema(cell);
  const validate = ajv.compile({ $ref: cell.$id + "#/$defs/XlsxCellValue" });
  for (const entry of fixture.cases) expect(validate(entry.value), ajv.errorsText(validate.errors)).toBe(true);
  expect(new Set(fixture.cases.map((entry: { id: string }) => entry.id)).size).toBe(fixture.cases.length);
  expect(fixture.sharedStringConflict.index).toBeLessThan(fixture.sharedStrings.length);
});
