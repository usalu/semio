import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020.js";
import { Database } from "bun:sqlite";
import { test } from "bun:test";
import { ValueError, type ValueRefusalKind } from "../🟦️.ts";
import { NativeDecodeControl } from "../../🛬️decode/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
const decorate = (error: ValueError, path: readonly (string | number)[]): ValueError => path.reduceRight((error, segment) => error.under(segment), error);
test("owned typed refusal construction preserves all kinds and paths against Ajv and SQLite", () => {
  const valid = new Ajv({ strict: true }).compile(schema);
  const db = new Database(":memory:"); db.run("CREATE TABLE path(position INTEGER PRIMARY KEY,segment TEXT NOT NULL)");
  try {
    for (const row of fixture.cases) {
      db.run("DELETE FROM path"); row.path.forEach((segment, index) => db.run("INSERT INTO path VALUES(?,?)", [index, String(segment)]));
      const prefix = (db.query("SELECT GROUP_CONCAT(segment,'.') AS prefix FROM(SELECT segment FROM path ORDER BY position)").get() as { prefix: string | null }).prefix;
      const display = (db.query("SELECT COALESCE(? || '.', '') || ? AS display").get(prefix, row.message) as { display: string }).display;
      const error = decorate(new ValueError(row.kind as ValueRefusalKind, row.message), row.path);
      assert(valid({ kind: error.kind, display: error.toString() }), row.id);
      assert.equal(error.kind, row.expected.kind, row.id); assert.equal(error.toString(), display, row.id); assert.equal(error.toString(), row.expected.display, row.id);
    }
  } finally { db.close(); }
});
test("actual portable NativeDecodeControl preserves cancellation ownership workload and UTF8 authority", async () => {
  const cases = [
    { id: "decode cancellation", run: () => new NativeDecodeControl(0, () => false).checkpoint() },
    { id: "decode ownership", run: () => new NativeDecodeControl(0, () => true).charge(1) },
    { id: "decode workload", run: async () => { const control = new NativeDecodeControl(0, () => true); await control.beginStage(1); await control.advance(2); } },
    { id: "borrowed malformed UTF8", run: () => new NativeDecodeControl(0, () => true).validateUtf8(new Uint8Array([255])) },
  ];
  for (const operation of cases) {
    const row = fixture.cases.find(row => row.id === operation.id); assert(row);
    let refusal: unknown; try { await operation.run(); } catch (error) { refusal = error; }
    assert(refusal instanceof ValueError, operation.id);
    const error = decorate(refusal, row.path); assert.equal(error.kind, row.expected.kind); assert.equal(error.toString(), row.expected.display);
  }
  console.log("[DEBUG] portable typed refusal construction and actual NativeDecodeControl retain authority and path");
});
