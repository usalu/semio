import { pdfIndirectFromNativeJson } from "../../../../📝️text/📸️snapshot/🪪️native-json/📄️document/🟦️.ts";
import { pdfCosFromNativeJson,pdfDictionaryFromNativeJson } from "../../../../📝️text/📸️snapshot/🪪️native-json/🟦️.ts";
/** 🧩️ PDF COS owner laws against neutral vectors and an independent SQLite engine. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import type { PdfObject, PdfStreamFilter } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { parsePdfObject, parsePdfDictEntry, parsePdfIndirectObject } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PDF_COS_SQLITE_SCHEMA, pdfObjectToSqliteDatabase, pdfObjectFromSqliteDatabase } from "../../🧩️cos/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

async function fixture(): Promise<{ root: PdfObject; depth: number }> {
  const value = await Bun.file(new URL("../../🧩️cos/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const filters: PdfStreamFilter[] = value.filters;
  const items: PdfObject[] = [{ kind: "null" }, { kind: "bool", value: false }, ...value.integers.map((integer: string) => ({ kind: "int" as const, value: BigInt(integer) })), { kind: "real", ...value.decimal }, { kind: "str", value: value.bytes }, { kind: "name", value: value.name }, { kind: "ref", ...value.reference }, { kind: "array", value: [] }, { kind: "dict", value: [{ key: "duplicate", value: { kind: "null" } }, { key: "duplicate", value: { kind: "bool", value: true } }] }, { kind: "stream", data: value.bytes, filters, dict: [{ key: "Size", value: { kind: "int", value: 4n } }] }];
  return { root: { kind: "array", value: items }, depth: value.depth };
}

test("PDF COS complete neutral model is SQL-interpretable and independently editable", async () => {
  const { root } = await fixture();
  const bytes = await exportSqliteDatabase(await pdfObjectToSqliteDatabase(root));
  const sql = Database.deserialize(bytes);
  expect(sql.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
  expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(sql.query("SELECT CAST(integer_value AS TEXT) AS value FROM pdf_cos_value WHERE kind='integer' ORDER BY id").all()).toEqual([{ value: "-9223372036854775808" }, { value: "9223372036854775807" }, { value: "4" }]);
  expect(sql.query("SELECT kind FROM pdf_stream_filter ORDER BY ordinal").all()).toEqual(["flate", "lzw", "asciiHex", "ascii85", "runLength", "dct", "jpx", "ccitt", "jbig2", "crypt"].map(kind => ({ kind })));
  expect(await pdfObjectFromSqliteDatabase(await importSqliteDatabase(bytes), 1n)).toEqual(root);
  sql.run("UPDATE pdf_cos_value SET integer_value=-17 WHERE integer_value=9223372036854775807");
  const edited = await pdfObjectFromSqliteDatabase(await importSqliteDatabase(sql.serialize()), 1n);
  expect(edited.kind).toBe("array");
  if (edited.kind === "array") expect(edited.value[3]).toEqual({ kind: "int", value: -17n });
  sql.close();
});

test("PDF COS nesting is iterative and cancellation bounds actual ownership copies", async () => {
  const { depth } = await fixture();
  let root: PdfObject = { kind: "int", value: 9223372036854775807n };
  for (let index = 0; index < depth; index++) root = { kind: "array", value: [root] };
  const bytes = await exportSqliteDatabase(await pdfObjectToSqliteDatabase(root));
  let decoded = await pdfObjectFromSqliteDatabase(await importSqliteDatabase(bytes), 1n);
  for (let index = 0; index < depth; index++) { expect(decoded.kind).toBe("array"); if (decoded.kind !== "array") throw new Error("COS nesting changed"); decoded = decoded.value[0]!; }
  expect(decoded).toEqual({ kind: "int", value: 9223372036854775807n });
  await expect(pdfObjectToSqliteDatabase(root, { maxRows: 4 })).rejects.toThrow();
  await expect(pdfObjectToSqliteDatabase({ kind: "str", value: new Array(100).fill(0) }, { maxValueBytes: 20 })).rejects.toThrow();
  const controller = new AbortController();
  await expect(pdfObjectToSqliteDatabase(root, { signal: controller.signal, onProgress: event => { if (event.completed >= 256) controller.abort(); } })).rejects.toThrow();
  const cancelled = new AbortController(); cancelled.abort();
  await expect(pdfObjectFromSqliteDatabase(await importSqliteDatabase(bytes), 1n, { signal: cancelled.signal })).rejects.toThrow();
});

test("PDF COS rejects cycles, orphan rows, variant spoofing and weakened SQL", async () => {
  const cyclic: PdfObject = { kind: "array", value: [] }; cyclic.value.push(cyclic);
  await expect(pdfObjectToSqliteDatabase(cyclic)).rejects.toThrow();
  const valid = await pdfObjectToSqliteDatabase({ kind: "array", value: [{ kind: "null" }] });
  const cycle = { tables: valid.tables.map(table => table.name === "pdf_cos_array_element" ? { ...table, rows: table.rows.map(row => ({ ...row, values: row.values.map((value, index) => index === 3 ? 1n : value) })) } : table) };
  await expect(pdfObjectFromSqliteDatabase(cycle, 1n)).rejects.toThrow();
  const spoof = { tables: valid.tables.map(table => table.name === "pdf_cos_value" ? { ...table, rows: table.rows.map(row => row.rowid === 2n ? { ...row, values: row.values.map((value, index) => index === 8 ? "hidden" : value) } : row) } : table) };
  await expect(pdfObjectFromSqliteDatabase(spoof, 1n)).rejects.toThrow();
  await expect(pdfObjectFromSqliteDatabase(valid, 2n)).rejects.toThrow();
  const weakened = { tables: valid.tables.map(table => ({ ...table, sql: table.sql.replace("chain_id INTEGER NOT NULL", "chain_id INTEGER \"NOT NULL\"") })) };
  await expect(pdfObjectFromSqliteDatabase(weakened, 1n)).rejects.toThrow();
  expect(PDF_COS_SQLITE_SCHEMA.split("CREATE TABLE")).toHaveLength(9);
});

test("PDF COS native JSON admission constructs the canonical owned bigint model", () => {
  expect(parsePdfObject(pdfCosFromNativeJson({ kind: "int", value: 17 }))).toEqual({ kind: "int", value: 17n });
  expect(parsePdfDictEntry(pdfDictionaryFromNativeJson([{ key: "Integer", value: { kind: "array", value: [{ kind: "int", value: -17 }] } }])[0])).toEqual({ key: "Integer", value: { kind: "array", value: [{ kind: "int", value: -17n }] } });
  expect(parsePdfIndirectObject(pdfIndirectFromNativeJson({ id: { num: 7, gen: 0 }, value: { kind: "int", value: 1 } }))).toEqual({ id: { num: 7, gen: 0 }, value: { kind: "int", value: 1n } });
  expect(() => parsePdfObject(pdfCosFromNativeJson({ kind: "int", value: 9223372036854775807 }))).toThrow();
});

test("PDF retained text and date own typed SQL values against neutral admission cases",async()=>{
  const cases=await Bun.file(new URL("../../../../🧫️fixtures/🪪️retained-text/🔣️.json",import.meta.url)).json();
  for(const vector of cases.cases){
    const root:PdfObject={kind:"array",value:[{kind:"text",value:vector.text},{kind:"date",value:vector.ownedDate}]};
    expect(parsePdfObject(root)).toEqual(root);
    const bytes=await exportSqliteDatabase(await pdfObjectToSqliteDatabase(root)),sql=Database.deserialize(bytes);
    expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(sql.query("SELECT name AS value FROM pdf_cos_value WHERE kind='text'").get()).toEqual({value:vector.text});
    expect(sql.query("SELECT d.year,d.month,d.day,d.offset_minutes AS offsetMinutes FROM pdf_cos_value v JOIN pdf_date d ON d.id=v.date_id WHERE v.kind='date'").get()).toEqual({year:vector.ownedDate.year,month:vector.ownedDate.month,day:vector.ownedDate.day,offsetMinutes:vector.ownedDate.offsetMinutes});
    expect(await pdfObjectFromSqliteDatabase(await importSqliteDatabase(bytes),1n)).toEqual(root);
    sql.run("UPDATE pdf_cos_value SET name='independent owned text' WHERE kind='text'");sql.run("UPDATE pdf_date SET year=2031");
    expect(await pdfObjectFromSqliteDatabase(await importSqliteDatabase(sql.serialize()),1n)).toEqual({kind:"array",value:[{kind:"text",value:"independent owned text"},{kind:"date",value:{...vector.ownedDate,year:2031}}]});
    sql.close();
  }
});
