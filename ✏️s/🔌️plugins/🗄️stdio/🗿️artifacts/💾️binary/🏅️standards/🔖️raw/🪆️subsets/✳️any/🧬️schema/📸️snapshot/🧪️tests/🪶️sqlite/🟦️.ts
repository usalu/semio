/** 🧫️ Shared Binary semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { binarySnapshotToSqliteDatabase, binarySnapshotFromSqliteDatabase, BINARY_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";
import { parseBinarySnapshot } from "../../🟦️.ts";

test("Binary rejects independently edited invalid flags and ordinal gaps", async () => {
  const input = fixture;
  const db = Database.deserialize(await exportSqliteDatabase(await binarySnapshotToSqliteDatabase(input)));
  try {
    db.run("PRAGMA ignore_check_constraints=ON");
    db.run("UPDATE binary_byte SET value=256 WHERE ordinal=0");
    await expect(binarySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("byte");
    db.run("UPDATE binary_byte SET value=0 WHERE ordinal=0");
    db.run("UPDATE binary_byte SET ordinal=-1 WHERE id=1");
    await expect(binarySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("contiguous");
  } finally { db.close(); }
});


test("Binary semantic SQLite schema, native roundtrip and independent SQL edit", async () => {
  expect(BINARY_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const input = fixture;
  const database = await binarySnapshotToSqliteDatabase(input);
  expect(await binarySnapshotFromSqliteDatabase(database)).toEqual(input);
  const bytes = await exportSqliteDatabase(database);
  const db = Database.deserialize(bytes);
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT value FROM binary_byte ORDER BY ordinal").all()).toEqual(fixture.bytes.map((value) => ({ value })));
    db.run("UPDATE binary_byte SET value=254 WHERE ordinal=0");
    const result = await binarySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(result.bytes[0]).toBe(254);
    db.run("UPDATE binary_byte SET document_id=999 WHERE ordinal=0");
    await expect(binarySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("unknown");
  } finally { db.close(); }
});

test("Binary resource limits apply before semantic projection and reconstruction", async () => {
  const input = fixture;
  const database = await binarySnapshotToSqliteDatabase(input);
  await expect(binarySnapshotToSqliteDatabase(input, { maxRows: 0 })).rejects.toThrow("row limit");
  await expect(binarySnapshotToSqliteDatabase(input, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  await expect(binarySnapshotFromSqliteDatabase(database, { maxRows: 0 })).rejects.toThrow("row limit");
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    const independent = await importSqliteDatabase(new Uint8Array(db.serialize()));
    await expect(binarySnapshotFromSqliteDatabase(independent, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  } finally { db.close(); }
});

test("Binary projection and reconstruction cancellation", async () => {
  const input = fixture;
  const database = await binarySnapshotToSqliteDatabase(input);
  for (const direction of ["project", "reconstruct"]) {
    const controller = new AbortController();
    const options = { signal: controller.signal, onProgress: () => controller.abort() };
    await expect(direction === "project" ? binarySnapshotToSqliteDatabase(input, options) : binarySnapshotFromSqliteDatabase(database, options)).rejects.toMatchObject({ name: "AbortError" });
  }
});

test("Binary snapshot typed boundary requires integer byte arrays", () => {
  expect(parseBinarySnapshot(fixture)).toEqual(fixture);
  for (const byte of [-1, 256, 1.5, "0"]) expect(() => parseBinarySnapshot({ schema: fixture.schema, bytes: [byte] })).toThrow();
  expect(() => parseBinarySnapshot({ schema: fixture.schema, bytes: "00ff" })).toThrow();
});
