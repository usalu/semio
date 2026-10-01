/** 🧫️ Shared Csv semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { csvSnapshotToSqliteDatabase, csvSnapshotFromSqliteDatabase, CSV_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

test("Csv rejects independently edited invalid flags and ordinal gaps", async () => {
  const input = fixture;
  const db = Database.deserialize(await exportSqliteDatabase(await csvSnapshotToSqliteDatabase(input)));
  try {
    db.run("PRAGMA ignore_check_constraints=ON");
    db.run("UPDATE csv_document SET has_header=2");
    await expect(csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("boolean");
    db.run("UPDATE csv_document SET has_header=0");
    db.run("UPDATE csv_field SET ordinal=-1 WHERE id=1");
    await expect(csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("contiguous");
  } finally { db.close(); }
});


test("Csv semantic SQLite schema, native roundtrip and independent SQL edit", async () => {
  expect(CSV_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const input = fixture;
  const database = await csvSnapshotToSqliteDatabase(input);
  expect(await csvSnapshotFromSqliteDatabase(database)).toEqual(input);
  const bytes = await exportSqliteDatabase(database);
  const db = Database.deserialize(bytes);
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT f.value,f.quoted FROM csv_field f JOIN csv_record r ON r.id=f.record_id ORDER BY r.ordinal,f.ordinal").all()).toEqual(fixture.records.flatMap((record) => record.fields.map((field) => ({ value: field.value, quoted: Number(field.quoted) }))));
    db.run("UPDATE csv_field SET value='edited semantic field' WHERE id=1");
    const result = await csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(result.records[0]!.fields[0]!.value).toBe("edited semantic field");
    db.run("UPDATE csv_field SET record_id=999 WHERE id=1");
    await expect(csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("unknown");
  } finally { db.close(); }
});

test("Csv resource limits apply before semantic projection and reconstruction", async () => {
  const input = fixture;
  const database = await csvSnapshotToSqliteDatabase(input);
  await expect(csvSnapshotToSqliteDatabase(input, { maxRows: 0 })).rejects.toThrow("row limit");
  await expect(csvSnapshotToSqliteDatabase(input, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  await expect(csvSnapshotFromSqliteDatabase(database, { maxRows: 0 })).rejects.toThrow("row limit");
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    const independent = await importSqliteDatabase(new Uint8Array(db.serialize()));
    await expect(csvSnapshotFromSqliteDatabase(independent, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  } finally { db.close(); }
});

test("Csv projection and reconstruction cancellation", async () => {
  const countingController = new AbortController();
  let countEvents = 0;
  await expect(csvSnapshotToSqliteDatabase({ ...fixture, records: Array.from({ length: 600 }, () => fixture.records[0]!) }, { signal: countingController.signal, onProgress: (progress) => { if (progress.completed === 0 && ++countEvents === 2) countingController.abort(); } })).rejects.toMatchObject({ name: "AbortError" });
  expect(countEvents).toBe(2);
  const input = fixture;
  const database = await csvSnapshotToSqliteDatabase(input);
  for (const direction of ["project", "reconstruct"]) {
    const controller = new AbortController();
    const options = { signal: controller.signal, onProgress: () => controller.abort() };
    await expect(direction === "project" ? csvSnapshotToSqliteDatabase(input, options) : csvSnapshotFromSqliteDatabase(database, options)).rejects.toMatchObject({ name: "AbortError" });
  }
});
