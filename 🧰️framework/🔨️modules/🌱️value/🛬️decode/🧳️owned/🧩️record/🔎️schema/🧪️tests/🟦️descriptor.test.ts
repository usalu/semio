import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import law from "../🔣️descriptor.json";
import schema from "../🧬️descriptor.json";

test("descriptor denial preserves the destination before a paid native selection", () => {
  expect(new Ajv({ strict: true }).compile(schema)(law)).toBe(true);
  const db = new Database(":memory:");
  try {
    const row = db.query("SELECT ?1 + ?2 AS calls, ?3 - ?4 AS work, ?5 AS preserved").get(law.deniedCalls, law.selectedCalls, law.work, law.deniedWork, Number(law.retainsDestinationOnDenial));
    expect(row).toEqual({ calls: 1, work: 1, preserved: 1 });
  } finally {
    db.close();
  }
});
