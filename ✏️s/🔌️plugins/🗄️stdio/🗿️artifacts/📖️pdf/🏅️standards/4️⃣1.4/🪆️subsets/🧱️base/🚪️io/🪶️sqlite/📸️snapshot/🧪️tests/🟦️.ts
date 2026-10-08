/** 📖️ PDF1.4 owned dimensions, ordered page text and independent SQL laws. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { parsePdfSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PDF14_SQLITE_SCHEMA, pdf14SnapshotToSqliteDatabase, pdf14SnapshotFromSqliteDatabase } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("PDF1.4 complete concrete backing contract keeps all page words and literal text", async () => {
  const plan = await Bun.file(new URL("../🧫️fixtures/🔢ieee.json", import.meta.url)).json();
  expect(plan.backing).toEqual({ authority: "completeSystemAllocatorRequests", phases: ["projectSnapshot", "reconstructSnapshot"], ceilings: ["zero", "exact", "oneBelow", "cumulative"], cancellation: ["start", "materializedInterior"], diagnosticOwnership: "actualCapacity", retirementRefund: false });
  const { default: Ajv } = await import("ajv/dist/2020");
  
  
  expect(plan.backing["authority"]).toEqual("completeSystemAllocatorRequests");expect(plan.backing["phases"]).toEqual(["projectSnapshot","reconstructSnapshot"]);expect(plan.backing["ceilings"]).toEqual(["zero","exact","oneBelow","cumulative"]);expect(plan.backing["cancellation"]).toEqual(["start","materializedInterior"]);expect(plan.backing["diagnosticOwnership"]).toEqual("actualCapacity");expect(plan.backing["retirementRefund"]).toEqual(false);
  for (const invalid of [{ ...plan.backing, extra: true }, { ...plan.backing, retirementRefund: true }, { ...plan.backing, authority: "estimatedSlots" }, { ...plan.backing, cancellation: ["start"] }]) 
  for (const item of plan.binary64) {
    const raw = BigInt(`0x${item.bits}`);
    const buffer = Buffer.alloc(8); buffer.writeBigUInt64BE(raw);
    const decoded = buffer.readDoubleBE();
    expect(Number.isNaN(decoded) ? "nan" : decoded === Infinity ? "positiveInfinity" : decoded === -Infinity ? "negativeInfinity" : "finite").toBe(item.class);
    const expected = { schema: "literal\0世界", pages: [{ width: { bits: raw }, height: { bits: raw }, text: "page\0😀" }] };
    const sql = Database.deserialize(await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(expected)), { safeIntegers: true });
    try {
      expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
      expect(sql.query("SELECT ordinal,text,CAST(width_bits AS TEXT) AS bits FROM pdf14_page").get()).toEqual({ ordinal: 0n, text: expected.pages[0]!.text, bits: BigInt.asIntN(64, raw).toString() });
      expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()))).toEqual(expected);
    } finally { sql.close(); }
  }
});

test("PDF1.4 native-neutral fixture is complete and independently editable", async () => {
  expect(PDF14_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  const snapshot = parsePdfSnapshot(await Bun.file(new URL("../🧫️fixtures/🔣️.json", import.meta.url)).json());
  const bytes = await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(snapshot));
  const sql = Database.deserialize(bytes);
  expect(sql.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
  expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(sql.query("SELECT ordinal,text FROM pdf14_page ORDER BY ordinal").all()).toEqual(snapshot.pages.map((page, ordinal) => ({ ordinal, text: page.text })));
  expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(snapshot);
  sql.run("UPDATE pdf14_page SET text='Independent Ω edit' WHERE ordinal=1");
  const restored = await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()));
  expect(restored.schema).toBe(snapshot.schema);
  expect(restored.pages[1]!.text).toBe("Independent Ω edit");
  sql.close();
});

test("PDF1.4 preserves every neutral IEEE word without numeric canonicalization", async () => {
  const cases = await Bun.file(new URL("../🧫️fixtures/🔢ieee.json", import.meta.url)).json();
  for (const item of cases.binary64) {
    const scalar = { bits: BigInt(`0x${item.bits}`) };
    const snapshot = { schema: "pdf14.custom-schema", pages: [{ width: scalar, height: scalar, text: item.bits }] };
    const bytes = await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(snapshot));
    expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(snapshot);
    const sql = Database.deserialize(bytes);
    expect(sql.query("SELECT CAST(width_bits AS TEXT) AS bits,width_class AS class,width IS NULL AS nullQuery FROM pdf14_page").get()).toEqual({ bits: BigInt.asIntN(64,scalar.bits).toString(), class: item.class, nullQuery: item.class === "nan" ? 1 : 0 });
    sql.close();
  }
});

test("PDF1.4 rejects noncontiguous pages, weakened schema and bounded cancellation", async () => {
  const snapshot = parsePdfSnapshot(await Bun.file(new URL("../🧫️fixtures/🔣️.json", import.meta.url)).json());
  await expect(pdf14SnapshotToSqliteDatabase(snapshot, { maxRows: 1 })).rejects.toThrow();
  await expect(pdf14SnapshotToSqliteDatabase(snapshot, { maxValueBytes: 1 })).rejects.toThrow();
  const controller = new AbortController(); controller.abort();
  await expect(pdf14SnapshotToSqliteDatabase(snapshot, { signal: controller.signal })).rejects.toThrow();
  const valid = await pdf14SnapshotToSqliteDatabase(snapshot);
  const changed = { tables: valid.tables.map(table => table.name === "pdf14_page" ? { ...table, rows: table.rows.map((row, index) => index === 0 ? { ...row, values: row.values.map((value, index) => index === 2 ? 999n : value) } : row) } : table) };
  await expect(pdf14SnapshotFromSqliteDatabase(changed)).rejects.toThrow();
  const weakened = { tables: valid.tables.map(table => ({ ...table, sql: table.sql.replace("schema TEXT NOT NULL", "schema TEXT \"NOT NULL\"") })) };
  await expect(pdf14SnapshotFromSqliteDatabase(weakened)).rejects.toThrow();
});

import { Buffer } from "node:buffer";
test("PDF14 populated page UI corpus is closed and independently SQL editable",async()=>{
  const fixture=await Bun.file(new URL("../../../../✏️editor/\u{1f9eb}️fixtures/\u{1f4c4}️resolved-page-domain/\u{1f523}️.json",import.meta.url)).json();
  
  expect(fixture["dialect"]).toEqual({"artifactKind":"s.stdio.pdf","standard":"1.4","subset":"*"});expect(fixture["schema"]).toEqual("stdio.pdf");expect(fixture["pages"]).toEqual([{"width":200,"height":300,"text":"Semio page one"},{"width":400,"height":500,"text":"Seite zwei"}]);expect(fixture["setPage"]).toEqual({"page":0,"item":0,"text":"Edited own14 text"});expect(fixture["geometryEdit"]).toEqual({"index":1,"width":8,"height":9});expect(fixture["invalidAddress"]).toEqual({"page":2,"item":0});expect(fixture["invalidItem"]).toEqual(1);expect(fixture["nativeHeader"]).toEqual("%PDF-1.4");expect(fixture["forbiddenOwnFields"]).toEqual(["mediaBox","cropBox","rotate","objects","trailer"]);expect(fixture["requiredActionVocabulary"]).toEqual(["set-page","insert-page","remove-page","move-page","set-page-size","replace-page-text"]);expect(fixture["artifactSchemaId"]).toEqual("s.stdio.pdf");expect(fixture["sqlEdit"]).toEqual({"page":1,"width":8,"height":9,"text":"Independent page edit"});
  
  const owner=parsePdfSnapshot(fixture),sql=Database.deserialize(await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(owner)));
  try{
    expect(sql.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(sql.query("SELECT p.ordinal,p.width,p.height,p.text FROM pdf14_document d JOIN pdf14_page p ON p.document_id=d.id ORDER BY p.ordinal").all()).toEqual(fixture.pages.map((p:{width:number;height:number;text:string},ordinal:number)=>({ordinal,...p})));
    const expected=parsePdfSnapshot({...fixture,pages:fixture.pages.map((p:unknown,index:number)=>index===fixture.sqlEdit.page?{width:fixture.sqlEdit.width,height:fixture.sqlEdit.height,text:fixture.sqlEdit.text}:p)});
    const word=(value:number)=>{const bytes=Buffer.alloc(8);bytes.writeDoubleLE(value);return bytes.readBigInt64LE().toString();};
    sql.query("UPDATE pdf14_page SET width=?,height=?,text=?,width_bits=CAST(? AS INTEGER),height_bits=CAST(? AS INTEGER),width_class='finite',height_class='finite' WHERE ordinal=?").run(fixture.sqlEdit.width,fixture.sqlEdit.height,fixture.sqlEdit.text,word(fixture.sqlEdit.width),word(fixture.sqlEdit.height),fixture.sqlEdit.page);
    expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()))).toEqual(expected);
    expect(owner).toEqual(parsePdfSnapshot(fixture));
  }finally{sql.close();}
});
