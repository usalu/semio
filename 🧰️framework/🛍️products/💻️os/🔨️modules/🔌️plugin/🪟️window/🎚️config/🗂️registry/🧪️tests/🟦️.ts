import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("original registry publication and cancellation match independent SQLite transactions", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const database = new Database(":memory:");
    try {
      database.exec("CREATE TABLE original_address(address TEXT PRIMARY KEY, value INTEGER NOT NULL)");
      for (const address of row.before) database.query("INSERT INTO original_address VALUES (?, 1)").run(address);
      database.exec("BEGIN");
      database.query("INSERT INTO original_address VALUES (?, ?)").run(fixture.key, fixture.value);
      expect(database.query("SELECT address FROM original_address ORDER BY address").all().map((item: any) => item.address)).toEqual(row.after);
      database.exec("ROLLBACK");
      expect(database.query("SELECT address FROM original_address ORDER BY address").all().map((item: any) => item.address)).toEqual(row.before);
    } finally {
      database.close();
    }
  }
});
