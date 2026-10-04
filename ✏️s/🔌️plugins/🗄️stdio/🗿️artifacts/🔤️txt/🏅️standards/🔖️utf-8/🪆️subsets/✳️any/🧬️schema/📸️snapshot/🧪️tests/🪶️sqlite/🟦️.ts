import refusalFixture from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧫️fixtures/🔣️.json";
const canceledKind=refusalFixture.cases.find(item=>item.id==="canceled-projection")!.expectedKind;
/** 🧫️ Shared Txt semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import nativeFixture from "../../🧫️fixtures/🪶️sqlite/🛂️native.json";
import { txtSnapshotToSqliteDatabase, txtSnapshotFromSqliteDatabase, TXT_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

test("Txt semantic metadata and surrogate identities remain literal", async () => {
  for (const schema of nativeFixture.literalSchemas) {
    const snapshot = { schema, lines: ["任意\0\nembedded", ""], trailingNewline: false, lineEnding: "lf" as const };
    const database = await txtSnapshotToSqliteDatabase(snapshot);
    const key = BigInt(nativeFixture.controls.rootIdentity);
    database.tables.find((table) => table.name === "text_document")!.rows[0]!.rowid = key;
    database.tables.find((table) => table.name === "text_document")!.rows[0]!.values[0] = key;
    for (const row of database.tables.find((table) => table.name === "text_line")!.rows) row.values[1] = key;
    expect(await txtSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
    const bytes = await exportSqliteDatabase(database);
    const independent = Database.deserialize(bytes);
    try {
      expect(independent.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
      expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
      expect(independent.query("SELECT schema FROM text_document").get()).toEqual({ schema });
      expect(independent.query("SELECT content FROM text_line ORDER BY ordinal").all()).toEqual(snapshot.lines.map((content) => ({ content })));
    } finally { independent.close(); }
  }
});

test("Txt long literal fields expose bounded Unicode traversal cancellation", async () => {
  const text = nativeFixture.controls.longText.repeat(nativeFixture.controls.longRepeat);
  for (const schemaField of [false, true]) {
    const snapshot = { schema: schemaField ? text : "stdio.txt", lines: schemaField ? [] : [text], trailingNewline: false, lineEnding: "lf" as const };
    const database = await txtSnapshotToSqliteDatabase(snapshot);
    for (const phase of ["projectSnapshot", "reconstructSnapshot"] as const) {
      const controller = new AbortController();
      let interior = false;
      const options = { signal: controller.signal, onProgress: (progress: { phase: string; completed: number; total: number }) => { if (progress.phase === phase && progress.completed >= 16_384 && progress.completed < progress.total) { interior = true; controller.abort(); } } };
      await expect(phase === "projectSnapshot" ? txtSnapshotToSqliteDatabase(snapshot, options) : txtSnapshotFromSqliteDatabase(database, options)).rejects.toHaveProperty("kind",canceledKind);
      expect(interior).toBe(true);
    }
  }
});

test("Txt semantic cumulative bytes and ordered wide relationships enforce exact bounds", async () => {
  const snapshot = { ...fixture, lineEnding: "crLf" as const };
  const bytes = new TextEncoder().encode(snapshot.schema).byteLength + 20 + snapshot.lines.reduce((sum, line) => sum + 24 + new TextEncoder().encode(line).byteLength, 0);
  const database = await txtSnapshotToSqliteDatabase(snapshot, { maxRows: nativeFixture.controls.domainRows, maxValueBytes: bytes });
  expect(await txtSnapshotFromSqliteDatabase(database, { maxRows: nativeFixture.controls.domainRows, maxValueBytes: bytes })).toEqual(snapshot);
  await expect(txtSnapshotToSqliteDatabase(snapshot, { maxValueBytes: bytes - 1 })).rejects.toThrow("value limit");
  await expect(txtSnapshotFromSqliteDatabase(database, { maxValueBytes: bytes - 1 })).rejects.toThrow("value limit");
  const wide = { ...snapshot, lines: Array.from({ length: nativeFixture.controls.wideLines }, (_, ordinal) => String(ordinal)) };
  const wideDatabase = await txtSnapshotToSqliteDatabase(wide);
  const controller = new AbortController();
  let interior = false;
  await expect(txtSnapshotFromSqliteDatabase(wideDatabase, { signal: controller.signal, onProgress: (progress) => { if (progress.phase === "reconstructSnapshot" && progress.completed >= 256 && progress.completed < progress.total) { interior = true; controller.abort(); } } })).rejects.toHaveProperty("kind",canceledKind);
  expect(interior).toBe(true);
});

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
  await expect(txtSnapshotToSqliteDatabase({ ...fixture, lineEnding: "lf" as const, lines: Array.from({ length: 600 }, () => "a") }, { signal: countingController.signal, onProgress: (progress) => { if (progress.completed === 0 && ++countEvents === 2) countingController.abort(); } })).rejects.toHaveProperty("kind",canceledKind);
  expect(countEvents).toBe(2);
  const input = fixture as { schema: string; lines: string[]; trailingNewline: boolean; lineEnding: "lf" | "crLf" };
  const database = await txtSnapshotToSqliteDatabase(input);
  for (const direction of ["project", "reconstruct"]) {
    const controller = new AbortController();
    const options = { signal: controller.signal, onProgress: () => controller.abort() };
    await expect(direction === "project" ? txtSnapshotToSqliteDatabase(input, options) : txtSnapshotFromSqliteDatabase(database, options)).rejects.toHaveProperty("kind",canceledKind);
  }
});
