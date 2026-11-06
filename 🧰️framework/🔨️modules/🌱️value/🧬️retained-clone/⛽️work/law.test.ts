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
