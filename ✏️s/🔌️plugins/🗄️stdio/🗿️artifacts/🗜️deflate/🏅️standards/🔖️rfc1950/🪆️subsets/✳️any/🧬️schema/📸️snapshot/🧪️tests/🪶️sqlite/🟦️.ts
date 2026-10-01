import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { parseDeflateSnapshot } from "../../🟦️.ts";
import { DEFLATE_SQLITE_SCHEMA, deflateSnapshotToSqliteDatabase, deflateSnapshotFromSqliteDatabase } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

test("Deflate declared header and decompressed byte entities expose shared editable SQL", async () => {
  expect(DEFLATE_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const snapshot = parseDeflateSnapshot(fixture);
  const database = await deflateSnapshotToSqliteDatabase(snapshot);
  expect(await deflateSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT d.dictionary_adler32,b.value FROM deflate_document d JOIN deflate_payload_byte b ON b.document_id=d.id WHERE b.ordinal=2").get()).toEqual({ dictionary_adler32: 4294967295, value: 255 });
    db.run("UPDATE deflate_payload_byte SET value=42 WHERE ordinal=2");
    db.run("UPDATE deflate_document SET dictionary_adler32=NULL");
    const edited = await deflateSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited).toEqual({ ...snapshot, dictId: undefined, payload: [0,65,42,128,17] });
  } finally { db.close(); }
});

test("Deflate scalar widths, ownership, budgets and cancellation remain bounded", async () => {
  const snapshot = parseDeflateSnapshot(fixture);
  await expect(deflateSnapshotToSqliteDatabase({ ...snapshot, compressionMethod: 16 })).rejects.toThrow();
  await expect(deflateSnapshotToSqliteDatabase({ ...snapshot, payload: [256] })).rejects.toThrow();
  await expect(deflateSnapshotToSqliteDatabase(snapshot, { maxValueBytes: 0 })).rejects.toThrow();
  const database = await deflateSnapshotToSqliteDatabase(snapshot);
  await expect(deflateSnapshotFromSqliteDatabase(database, { maxValueBytes: 0 })).rejects.toThrow();
  for (const alteration of ["UPDATE deflate_payload_byte SET document_id=999 WHERE ordinal=0", "UPDATE deflate_payload_byte SET ordinal=0 WHERE ordinal=1", "UPDATE deflate_document SET compression_level_hint=4"]) {
    const db = Database.deserialize(await exportSqliteDatabase(database));
    try { db.run("PRAGMA ignore_check_constraints=ON"); db.run(alteration); await expect(deflateSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); } finally { db.close(); }
  }
  const controller = new AbortController();
  await expect(deflateSnapshotToSqliteDatabase({ ...snapshot, payload: new Array<number>(2000).fill(1) }, { signal: controller.signal, onProgress: event => { if (event.completed >= 256) controller.abort(); } })).rejects.toHaveProperty("name", "AbortError");
});
