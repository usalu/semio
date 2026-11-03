/** 🗜️ The canonical layout of a schema document: indented while the indented text fits `SCHEMA_INDENT_LINES`, compact (one line per definition) beyond it or when the file already is compact; pure, idempotent and value-preserving. The vectors in `🔣️.json` are shared with the Python mirror `🐍️.py`, which writes the same text with the independent `json` library.
 * @see ../../🧩️model/🟦️.ts
 */
import { expect, test } from "bun:test";
import vectors from "./🔣️.json";
import { SCHEMA_INDENT_LINES, serializeSchema } from "../../🧩️model/🟦️.ts";

test("the limit of the vectors is the limit of the model", () => {
  expect(vectors.limitLines).toBe(SCHEMA_INDENT_LINES);
});

for (const row of vectors.cases) {
  test(row.id, () => {
    const text = serializeSchema(row.document, row.original);
    expect(text).toBe(row.expected);
    expect(JSON.parse(text)).toEqual(row.document);
    expect(serializeSchema(JSON.parse(text), text)).toBe(text);
  });
}

test("a document whose indented text exceeds the limit is laid out compact and stays so", () => {
  const properties = Object.fromEntries(Array.from({ length: SCHEMA_INDENT_LINES }, (_, at) => [`p${at}`, { type: "number" }]));
  const document = { $schema: "http://json-schema.org/draft-07/schema#", title: "Large", type: "object", properties };
  const text = serializeSchema(document, `${JSON.stringify(document, null, 2)}\n`);
  expect(text.split("\n").length).toBeLessThan(10);
  expect(JSON.parse(text)).toEqual(document);
  expect(serializeSchema(JSON.parse(text), text)).toBe(text);
});

test("a document at the limit stays indented", () => {
  const fixed = 8;
  const properties = Object.fromEntries(Array.from({ length: Math.floor((SCHEMA_INDENT_LINES - fixed) / 4) }, (_, at) => [`p${at}`, { type: "number" }]));
  const document = { $schema: "http://json-schema.org/draft-07/schema#", title: "Edge", type: "object", properties };
  const indented = `${JSON.stringify(document, null, 2)}\n`;
  expect(indented.split("\n").length).toBeLessThanOrEqual(SCHEMA_INDENT_LINES);
  expect(serializeSchema(document, indented)).toBe(indented);
});
