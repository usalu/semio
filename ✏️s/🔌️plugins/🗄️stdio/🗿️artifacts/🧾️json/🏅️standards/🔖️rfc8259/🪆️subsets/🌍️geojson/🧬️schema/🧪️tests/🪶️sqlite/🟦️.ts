/** 🌍️ Neutral GeoJSON exact profile admission and independent SQLite edits. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "../../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { validateGeoJsonSnapshotSqliteDialect } from "../../🪶️sqlite/🟦️.ts";
import { jsonSnapshotToSqliteDatabase, jsonSnapshotFromSqliteDatabase } from "../../../../🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
import type { JsonValue } from "../../../../🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";
function value(input: unknown): JsonValue { if (input === null) return { kind: "null" }; if (typeof input === "boolean") return { kind: "bool", value: input }; if (typeof input === "string") return { kind: "string", value: input }; if (typeof input === "number") return { kind: "number", lexeme: String(input) }; if (Array.isArray(input)) return { kind: "array", items: input.map(value) }; return { kind: "object", members: Object.entries(input as Record<string, unknown>).map(([key, input]) => ({ key, value: value(input) })) }; }
test("GeoJSON neutral profile cases admit their borrowed typed JSON and queryable SQLite", async () => {
 for (const item of fixture.cases) {
  const snapshot = { schema: "owned GeoJSON 世界", value: value(JSON.parse(item.text)) };
  const database = await jsonSnapshotToSqliteDatabase(snapshot);
  const diagnostics = await validateGeoJsonSnapshotSqliteDialect(snapshot, { artifactKind: "s.stdio.json", standard: "rfc8259", subset: "geojson" }, database);
  expect(diagnostics.map(d => d.code)).toEqual(item.codes);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try { expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" }); expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]); const restored = await jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize()))); expect(await validateGeoJsonSnapshotSqliteDialect(restored, { artifactKind: "s.stdio.json", standard: "rfc8259", subset: "geojson" }, await jsonSnapshotToSqliteDatabase(restored))).toEqual(diagnostics); } finally { db.close(); }
 }
});
test("GeoJSON independently edited coordinates reject exact named profile while preserving the JSON syntax", async () => {
 const snapshot = { schema: "owned", value: value(JSON.parse(fixture.cases[0]!.text)) }; const db = Database.deserialize(await exportSqliteDatabase(await jsonSnapshotToSqliteDatabase(snapshot)));
 try { db.query("UPDATE json_value SET number_lexeme=?,number_value=CAST(? AS REAL) WHERE number_lexeme='12'").run(fixture.editCoordinateLexeme,fixture.editCoordinateLexeme); const restored = await jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize()))); expect((await validateGeoJsonSnapshotSqliteDialect(restored, { artifactKind: "s.stdio.json", standard: "rfc8259", subset: "geojson" }, await jsonSnapshotToSqliteDatabase(restored))).map(d => d.code)).toEqual(["stdio.json.geojson.not-rfc7946"]); } finally { db.close(); }
});

test("GeoJSON exact document identity and cancellable foreign-member traversal", async () => {
 const snapshot = { schema: "owned", value: value({ type: "Feature", geometry: null, properties: { retained: Array.from({ length: fixture.cancelNodes }, () => 1) } }) }; const database = await jsonSnapshotToSqliteDatabase(snapshot); const dialect = { artifactKind: "s.stdio.json", standard: "rfc8259", subset: "geojson" };
 for (const invalid of [{ ...dialect, artifactKind: "s.stdio.xml" }, { ...dialect, standard: "1.0" }, { ...dialect, subset: "*" }]) await expect(validateGeoJsonSnapshotSqliteDialect(snapshot, invalid, database)).rejects.toThrow("subset");
 await expect(validateGeoJsonSnapshotSqliteDialect({ ...snapshot, schema: "different" }, dialect, database)).rejects.toThrow("identity");
 const before = new AbortController(); before.abort(); await expect(validateGeoJsonSnapshotSqliteDialect(snapshot, dialect, database, { signal: before.signal })).rejects.toMatchObject({ name: "AbortError" });
 const during = new AbortController(); let visited = 0; await expect(validateGeoJsonSnapshotSqliteDialect(snapshot, dialect, database, { signal: during.signal, onProgress: event => { if (event.completed >= 256) { visited = event.completed; during.abort(); } } })).rejects.toMatchObject({ name: "AbortError" }); expect(visited).toBe(256);
});
