/** 🧫️ Shared TSV semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { tsvSnapshotToSqliteDatabase, tsvSnapshotFromSqliteDatabase, TSV_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input = { ...fixture, lineEnding: "crlf" as const };

test("TSV shared handcrafted schema, semantic SQL joins and independent edits", async () => {
  expect(TSV_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const database = await tsvSnapshotToSqliteDatabase(input);
  expect(await tsvSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT f.value FROM tsv_record r JOIN tsv_field f ON f.record_id=r.id ORDER BY r.ordinal,f.ordinal").all()).toEqual(input.records.flatMap(record => record.map(value => ({ value }))));
    db.run("UPDATE tsv_field SET value='edited TSV entity 🌠' WHERE id=1");
    db.run("UPDATE tsv_document SET line_ending='lf', trailing_newline=0");
    const edited = await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited.records[0]![0]).toBe("edited TSV entity 🌠");
    expect(edited.lineEnding).toBe("lf");
    expect(edited.trailingNewline).toBe(false);
    db.run("UPDATE tsv_field SET record_id=999 WHERE id=1");
    await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("unknown");
  } finally { db.close(); }
});

test("TSV independently edited ordinals and flags reject", async () => {
  const db = Database.deserialize(await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(input)));
  try {
    db.run("PRAGMA ignore_check_constraints=ON");
    db.run("UPDATE tsv_document SET trailing_newline=2");
    await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("boolean");
    db.run("UPDATE tsv_document SET trailing_newline=1,line_ending='invalid'");
    await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("line ending");
    db.run("UPDATE tsv_document SET line_ending='crlf'");
    db.run("UPDATE tsv_field SET ordinal=-1 WHERE id=1");
    await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("contiguous");
  } finally { db.close(); }
});

test("TSV row and value limits apply to both semantic directions", async () => {
  const database = await tsvSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 }, { maxValueBytes: 0 }]) {
    await expect(tsvSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(tsvSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
});

test("TSV cancellation reaches counting scans and reconstruction", async () => {
  const controller = new AbortController();
  let events = 0;
  await expect(tsvSnapshotToSqliteDatabase({ ...input, records: [Array.from({ length: 1000 }, () => "cell")] }, { signal: controller.signal, onProgress: progress => { if (progress.completed === 0 && ++events === 2) controller.abort(); } })).rejects.toMatchObject({ name: "AbortError" });
  expect(events).toBe(2);
  const reconstruction = new AbortController();
  const database = await tsvSnapshotToSqliteDatabase(input);
  await expect(tsvSnapshotFromSqliteDatabase(database, { signal: reconstruction.signal, onProgress: () => reconstruction.abort() })).rejects.toMatchObject({ name: "AbortError" });
});

test("TSV empty records and intrinsic scalar text preserve native semantics", async () => {
  const value = { ...input, records: [[], ["", "\u0000", "漢🌠"]], lineEnding: "lf" as const };
  const database = await tsvSnapshotToSqliteDatabase(value);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("SELECT count(*) AS records FROM tsv_record").get()).toEqual({ records: 2 });
    expect(db.query("SELECT value FROM tsv_field ORDER BY ordinal").all()).toEqual(value.records[1]!.map(value => ({ value })));
    expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(value);
  } finally { db.close(); }
});

