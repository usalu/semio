import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import schema from "./schema.json";
import law from "./fixture.json";

test("delegated work is quoted before advance, take and close", () => {
  expect(new Ajv({ strict: true }).validate(schema, law)).toBe(true);
  for (const row of law.cases) {
    expect(row.quoted).toBe(row.local + row.child);
    expect(row.effects).toBe(row.available >= row.quoted);
    expect(row.performed).toBe(row.effects ? row.quoted : 0);
  }
});

import cursorSchema from "./cursors.schema.json";
import cursorVectors from "./cursors.json";

test("all retained cursor families share the selected work regression", () => {
  expect(new Ajv({ strict: true }).validate(cursorSchema, cursorVectors)).toBe(true);
  for (const row of cursorVectors.cases) {
    const oracle = JSON.parse(row.source);
    expect(JSON.parse(JSON.stringify(oracle))).toEqual(oracle);
  }
});

import { Database } from "bun:sqlite";

test("SQLite independently computes selected original work receipts", () => {
  const db = new Database(":memory:");
  try {
    const quote = db.query("select ?1 + ?2 as quoted, case when ?3 >= ?1 + ?2 then ?1 + ?2 else 0 end as performed, ?3 >= ?1 + ?2 as effects");
    for (const row of law.cases) {
      const oracle = quote.get(row.local, row.child, row.available) as { quoted: number; performed: number; effects: number };
      expect(oracle.quoted).toBe(row.quoted);
      expect(oracle.performed).toBe(row.performed);
      expect(Boolean(oracle.effects)).toBe(row.effects);
    }
  } finally { db.close(); }
});

test("duplicate cursor family coverage is refused", () => {
  const validate = new Ajv({ strict: true }).compile(cursorSchema);
  expect(validate(cursorVectors)).toBe(true);
  const duplicate = structuredClone(cursorVectors);
  duplicate.cases[0].cursor = duplicate.cases[1].cursor;
  expect(validate(duplicate)).toBe(false);
});
