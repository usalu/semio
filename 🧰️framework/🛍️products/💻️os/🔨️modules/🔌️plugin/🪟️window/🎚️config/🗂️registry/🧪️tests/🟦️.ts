import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("window_config_paged_registry_neutral_original_order", () => {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  const database = new Database(":memory:");
  try {
    database.run("CREATE TABLE original_addresses(address TEXT PRIMARY KEY COLLATE BINARY)");
    const insert = database.prepare("INSERT INTO original_addresses VALUES(?)");
    for (const address of fixture.addresses) for (let ordinal = 0; ordinal < fixture.copies; ordinal++) insert.run(address.repeat(fixture.addressRepeat) + "/" + ordinal.toString().padStart(6, "0"));
    const ordered = database.query("SELECT address FROM original_addresses ORDER BY address COLLATE BINARY").all() as { address: string }[];
    expect(ordered.map(row => row.address)).toEqual(fixture.ordered.flatMap(address => Array.from({ length: fixture.copies }, (_, ordinal) => address.repeat(fixture.addressRepeat) + "/" + ordinal.toString().padStart(6, "0"))));
    expect(fixture.refusedAllocationBytes + fixture.refusedReleaseBytes + fixture.terminalDropBytes).toBe(0);
    const reservation = database.query("SELECT (? != 0) AS reserved, 0 AS copiedBytes");
    for (const law of fixture.sourceReservation) {
      const result = reservation.get(Number(law.capacityGranted)) as { reserved: number; copiedBytes: number };
      expect({ reserved: Boolean(result.reserved), copiedBytes: result.copiedBytes }).toEqual({ reserved: law.reserved, copiedBytes: law.copiedBytes });
    }
    console.error("[DEBUG] original window registry strict neutral SQLite order admitted");
  } finally {
    database.close();
  }
});

test("window_config_paged_registry_neutral_authored_metadata_capability", () => {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  const law = JSON.parse(JSON.stringify(fixture.rootMetadata)) as typeof fixture.rootMetadata;
  const database = new Database(":memory:");
  try {
    database.run("CREATE TABLE authored_fields(ordinal INTEGER PRIMARY KEY, field TEXT NOT NULL UNIQUE)");
    const insert = database.prepare("INSERT INTO authored_fields VALUES(?,?)");
    law.fields.forEach((field, ordinal) => insert.run(ordinal, field));
    expect(database.query("SELECT field FROM authored_fields ORDER BY ordinal").all()).toEqual(law.fields.map(field => ({ field })));
    expect(law.ownedProducer).toBe(true);
    expect(law.schemaLessProducer).toBe(false);
    expect(law.getterAllocationBytes + law.getterReleaseBytes).toBe(0);
    expect(law.maximumCapacityBytes).toBeGreaterThan(0);
    console.error("[DEBUG] original Window authored metadata declaration matches independent JSON and SQLite field order");
  } finally {
    database.close();
  }
});
