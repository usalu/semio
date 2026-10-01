/** 🧫️ Shared JSON semantic corpus with independent SQL editing and ownership validation. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import type { JsonSnapshot, JsonValue } from "../../🟦️.ts";
import { parseJsonSnapshot } from "../../🟦️.ts";
import { jsonSnapshotToSqliteDatabase, jsonSnapshotFromSqliteDatabase, JSON_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input: JsonSnapshot = { schema: fixture.schema, value: { kind: "object", members: [
  { key: fixture.memberKeys[0]!, value: { kind: "number", lexeme: fixture.numberLexemes[0]! } },
  { key: fixture.memberKeys[1]!, value: { kind: "array", items: [{ kind: "bool", value: true }, { kind: "bool", value: false }, { kind: "null" }, { kind: "string", value: "Grüße 🌠\u0000" }, { kind: "object", members: [] }, { kind: "array", items: [] }] } },
  { key: fixture.memberKeys[2]!, value: { kind: "number", lexeme: fixture.numberLexemes[1]! } },
] } };

test("JSON native lexemes and ordered duplicate members expose independent relational SQL", async () => {
  expect(parseJsonSnapshot(input)).toEqual(input);
  expect(JSON_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const database = await jsonSnapshotToSqliteDatabase(input);
  expect(await jsonSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query(fixture.query).all()).toEqual([{ key: "z", kind: "number", number_lexeme: fixture.numberLexemes[0] }, { key: "a", kind: "array", number_lexeme: null }, { key: "z", kind: "number", number_lexeme: fixture.numberLexemes[1] }]);
    expect(db.query("SELECT count(*) AS count FROM json_value").get()).toEqual({ count: fixture.valueCount });
    expect(db.query("SELECT count(*) AS count FROM json_object_member").get()).toEqual({ count: fixture.memberCount });
    expect(db.query("SELECT count(*) AS count FROM json_array_element").get()).toEqual({ count: fixture.elementCount });
    db.query("UPDATE json_value SET string_value=? WHERE kind='string'").run(fixture.editedString);
    const edited = await jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect((edited.value as Extract<JsonValue, { kind: "object" }>).members[1]!.value).toEqual({ kind: "array", items: [{ kind: "bool", value: true }, { kind: "bool", value: false }, { kind: "null" }, { kind: "string", value: fixture.editedString }, { kind: "object", members: [] }, { kind: "array", items: [] }] });
  } finally { db.close(); }
});

test("JSON independently edited dangling, multiple-owner, disconnected-cycle and ordinal relations reject", async () => {
  for (const edit of [
    "UPDATE json_document SET root_value_id=999",
    "UPDATE json_array_element SET value_id=1 WHERE ordinal=0",
    "UPDATE json_array_element SET ordinal=99 WHERE ordinal=0",
    "UPDATE json_object_member SET object_value_id=3 WHERE ordinal=0",
    "INSERT INTO json_value VALUES (99,'array',NULL,NULL,NULL); INSERT INTO json_array_element VALUES (99,99,0,99)",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await jsonSnapshotToSqliteDatabase(input)));
    try { db.run(edit); await expect(jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("JSON exact number grammar and constrained primitive columns reject independent bad edits", async () => {
  for (const lexeme of ["01", "+1", "1.", "1e", " 1", "1\n", "NaN"]) {
    await expect(jsonSnapshotToSqliteDatabase({ schema: input.schema, value: { kind: "number", lexeme } })).rejects.toThrow("number");
    const db = Database.deserialize(await exportSqliteDatabase(await jsonSnapshotToSqliteDatabase(input)));
    try { db.query("UPDATE json_value SET number_lexeme=? WHERE kind='number'").run(lexeme); await expect(jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("number"); }
    finally { db.close(); }
  }
  const db = Database.deserialize(await exportSqliteDatabase(await jsonSnapshotToSqliteDatabase(input)));
  try { db.run("PRAGMA ignore_check_constraints=ON"); db.run("UPDATE json_value SET boolean_value=2 WHERE kind='boolean'"); await expect(jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
  finally { db.close(); }
});

test("JSON iterative trees honor bounds and cancellation without native serialization", async () => {
  const database = await jsonSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 }, { maxValueBytes: 0 }]) {
    await expect(jsonSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(jsonSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
  let value: JsonValue = { kind: "null" };
  for (let depth = 0; depth < 3000; depth++) value = { kind: "array", items: [value] };
  const nested = await jsonSnapshotFromSqliteDatabase(await jsonSnapshotToSqliteDatabase({ schema: "deep", value }));
  let rebuilt = nested.value;
  let count = 0;
  while (rebuilt.kind === "array") { count++; rebuilt = rebuilt.items[0]!; }
  expect(count).toBe(3000);
  const controller = new AbortController();
  let events = 0;
  await expect(jsonSnapshotToSqliteDatabase({ schema: "deep", value }, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ name: "AbortError" });
  const reconstruction = new AbortController();
  await expect(jsonSnapshotFromSqliteDatabase(database, { signal: reconstruction.signal, onProgress: () => reconstruction.abort() })).rejects.toMatchObject({ name: "AbortError" });
});
