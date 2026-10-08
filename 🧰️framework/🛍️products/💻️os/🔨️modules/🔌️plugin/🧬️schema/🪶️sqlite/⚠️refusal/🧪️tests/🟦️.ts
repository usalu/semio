import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("guest SQLite rejection carries all eight intrinsic causes independently of prose and VM cancellation", () => {
  const ajv = new Ajv({ strict: true });
  const validate = ajv.compile(schema.$defs.rejection);
  const diagnostics = Buffer.from(fixture.diagnostic.message, "utf8");
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE refusal(kind TEXT PRIMARY KEY, message TEXT NOT NULL, diagnostics BLOB NOT NULL)");
    for (const kind of fixture.kinds) {
      const carrier = { kind, message: fixture.message, diagnostics: [...diagnostics] };
      expect(validate(carrier)).toBe(true);
      expect(validate({ ...carrier, kind: "OwnershipLimit" })).toBe(false);
      expect(validate({ message: carrier.message, diagnostics: carrier.diagnostics })).toBe(false);
      expect(validate({ ...carrier, guessedKind: kind })).toBe(false);
      database.query("INSERT INTO refusal VALUES (?, ?, ?)").run(kind, fixture.message, diagnostics);
    }
    const rows = database.query("SELECT kind, message, diagnostics FROM refusal ORDER BY rowid").all() as { kind: string; message: string; diagnostics: Uint8Array }[];
    expect(rows.map(row => row.kind)).toEqual(fixture.kinds);
    for (const row of rows) {
      expect(row.message).toBe(fixture.message);
      expect(Buffer.from(row.diagnostics)).toEqual(diagnostics);
    }
  } finally { database.close(); }
  const wit = readFileSync(resolve(import.meta.dir, "../../../📜️.wit"), "utf8");
  const cases = wit.match(/enum value-refusal-kind\s*\{([^}]+)\}/)?.[1].split(",").map(value => value.trim()).filter(Boolean);
  expect(cases).toEqual(fixture.kinds.map(kind => kind.replace(/[A-Z]/g, letter => `-${letter.toLowerCase()}`)));
  expect(wit.match(/record snapshot-rejection\s*\{([^}]+)\}/)?.[1]).toMatch(/kind:\s*value-refusal-kind/);
  expect(fixture.providerCancellation).toBe("canceled");
  expect(fixture.vmCancellation).toBe("turnFaultCancelled");
});
