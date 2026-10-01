/** 🧫️ Shared Txt semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { txtSnapshotToSqliteDatabase, txtSnapshotFromSqliteDatabase, TXT_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

test("Txt rejects independently edited invalid flags and ordinal gaps", async () => {
  const input = fixture as { schema: string; lines: string[]; trailingNewline: boolean; lineEnding: "lf" | "crLf" };
  const db = Database.deserialize(await exportSqliteDatabase(await txtSnapshotToSqliteDatabase(input)));
  try {
    db.run("PRAGMA ignore_check_constraints=ON");
    db.run("UPDATE text_document SET trailing_newline=2");
    await expect(txtSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("boolean");
    db.run("UPDATE text_document SET trailing_newline=0");
    db.run("UPDATE text_line SET ordinal=-1 WHERE id=1");
    await expect(txtSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("contiguous");
  } finally { db.close(); }
});


test("Txt semantic SQLite schema, native roundtrip and independent SQL edit", async () => {
  expect(TXT_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const input = fixture as { schema: string; lines: string[]; trailingNewline: boolean; lineEnding: "lf" | "crLf" };
  const database = await txtSnapshotToSqliteDatabase(input);
  expect(await txtSnapshotFromSqliteDatabase(database)).toEqual(input);
  const bytes = await exportSqliteDatabase(database);
  const db = Database.deserialize(bytes);
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT content FROM text_line ORDER BY ordinal").all()).toEqual(fixture.lines.map((content) => ({ content })));
    db.run("UPDATE text_line SET content='edited semantic line' WHERE ordinal=0");
    const result = await txtSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(result.lines[0]).toBe("edited semantic line");
    db.run("UPDATE text_line SET document_id=999 WHERE ordinal=0");
    await expect(txtSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("unknown");
  } finally { db.close(); }
});

test("Txt resource limits apply before semantic projection and reconstruction", async () => {
  const input = fixture as { schema: string; lines: string[]; trailingNewline: boolean; lineEnding: "lf" | "crLf" };
  const database = await txtSnapshotToSqliteDatabase(input);
  await expect(txtSnapshotToSqliteDatabase(input, { maxRows: 0 })).rejects.toThrow("row limit");
  await expect(txtSnapshotToSqliteDatabase(input, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  await expect(txtSnapshotFromSqliteDatabase(database, { maxRows: 0 })).rejects.toThrow("row limit");
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    const independent = await importSqliteDatabase(new Uint8Array(db.serialize()));
    await expect(txtSnapshotFromSqliteDatabase(independent, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  } finally { db.close(); }
});

test("Txt projection and reconstruction cancellation", async () => {
  const countingController = new AbortController();
  let countEvents = 0;
  await expect(txtSnapshotToSqliteDatabase({ ...fixture, lineEnding: "lf" as const, lines: Array.from({ length: 600 }, () => "a") }, { signal: countingController.signal, onProgress: (progress) => { if (progress.completed === 0 && ++countEvents === 2) countingController.abort(); } })).rejects.toMatchObject({ name: "AbortError" });
  expect(countEvents).toBe(2);
  const input = fixture as { schema: string; lines: string[]; trailingNewline: boolean; lineEnding: "lf" | "crLf" };
  const database = await txtSnapshotToSqliteDatabase(input);
  for (const direction of ["project", "reconstruct"]) {
    const controller = new AbortController();
    const options = { signal: controller.signal, onProgress: () => controller.abort() };
    await expect(direction === "project" ? txtSnapshotToSqliteDatabase(input, options) : txtSnapshotFromSqliteDatabase(database, options)).rejects.toMatchObject({ name: "AbortError" });
  }
});
