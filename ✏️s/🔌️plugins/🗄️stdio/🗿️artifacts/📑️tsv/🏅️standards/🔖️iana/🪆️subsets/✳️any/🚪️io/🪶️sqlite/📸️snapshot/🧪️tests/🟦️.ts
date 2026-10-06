/** 🧫️ Shared TSV semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import { tsvSnapshotToSqliteDatabase, tsvSnapshotFromSqliteDatabase, TSV_SQLITE_SCHEMA } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input = { ...fixture, lineEnding: "crlf" as const };

test("TSV shared handcrafted schema, semantic SQL joins and independent edits", async () => {
  expect(TSV_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
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


test("tsv borrowed native preflight contract has literal octet and refusal authorities", async()=>{
 
 expect(controlFixture["preflight"]["encodings"]).toEqual(["binary","text"]);expect(controlFixture["preflight"]["ownershipBytes"]).toEqual(0);expect(controlFixture["preflight"]["byteAuthority"]).toEqual("actualLiteralOutputUTF8OrOctets");expect(controlFixture["preflight"]["refusalKinds"]["file"]).toEqual("ownershipLimit");expect(controlFixture["preflight"]["refusalKinds"]["cancellation"]).toEqual("canceled");expect(controlFixture["preflight"]["cancelAt"]).toEqual("firstBorrowedCheckpoint");expect(controlFixture["preflight"]["retirementRefund"]).toEqual(false);expect(controlFixture["expectedRows"]).toEqual(1030);expect(controlFixture["copyTextUnit"]).toEqual("文🌠");expect(controlFixture["copyTextRepeat"]).toEqual(20000);expect(controlFixture["copyTextBytes"]).toEqual(140000);expect(controlFixture["copyTextBoundary"]).toEqual(65534);
 const policy=controlFixture.preflight;
 expect(policy).toEqual({"encodings":["binary","text"],"ownershipBytes":0,"byteAuthority":"actualLiteralOutputUTF8OrOctets","refusalKinds":{"file":"ownershipLimit","cancellation":"canceled"},"cancelAt":"firstBorrowedCheckpoint","retirementRefund":false});
 
 
 const literal=controlFixture.fieldText,octets=Buffer.from(literal,"utf8");
 expect(Buffer.byteLength(literal,"utf8")).toBe(new TextEncoder().encode(literal).length);
 expect(octets.toString("utf8")).toBe(literal);
 const independent=new Database(":memory:");
 try{independent.run("CREATE TABLE literal_output(value TEXT NOT NULL)");independent.run("INSERT INTO literal_output VALUES(?)",[literal]);expect(independent.query("SELECT length(CAST(value AS BLOB)) AS bytes FROM literal_output").get()).toEqual({bytes:octets.byteLength});}finally{independent.close();}
 expect(policy.ownershipBytes).toBe(0);expect(policy.retirementRefund).toBe(false);
 expect(controlFixture.allocationRole).toBe("cumulativeOwnedBacking");
 expect(controlFixture.maxAllocationBytes).toBe(1);
 
 const primitive=Buffer.from(controlFixture.bytePattern.slice(0,controlFixture.maxAllocationBytes+1));
 expect(primitive.byteLength).toBe(controlFixture.maxAllocationBytes+1);
 const ownership=new BudgetAllocationControl({maxAllocationBytes:controlFixture.maxAllocationBytes});
 await expect(ownership.stage(maximum=>new BudgetDecodeControl(maximum,()=>true),native=>native.copyBytes(primitive))).rejects.toMatchObject({kind:"ownershipLimit"});
 expect(ownership.remainingBytes()).toBe(controlFixture.maxAllocationBytes);
 const semanticLimits={maxAllocationBytes:primitive.byteLength,maxValueBytes:0};const semantic=new BudgetAllocationControl(semanticLimits);
 const copied=await semantic.stage(maximum=>new BudgetDecodeControl(maximum,()=>true),native=>native.copyBytes(primitive));
 expect(Buffer.from(copied)).toEqual(primitive);expect(semantic.remainingBytes()).toBe(0);

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
  await expect(tsvSnapshotToSqliteDatabase({ ...input, records: [Array.from({ length: 1000 }, () => "cell")] }, { signal: controller.signal, onProgress: progress => { if (progress.completed === 0 && ++events === 2) controller.abort(); } })).rejects.toMatchObject({ kind: "canceled" });
  expect(events).toBe(2);
  const reconstruction = new AbortController();
  const database = await tsvSnapshotToSqliteDatabase(input);
  await expect(tsvSnapshotFromSqliteDatabase(database, { signal: reconstruction.signal, onProgress: () => reconstruction.abort() })).rejects.toMatchObject({ kind: "canceled" });
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


import controlFixture from "../🧫️fixtures/🛬️native-control/🔣️.json";

import Ajv2020 from "ajv/dist/2020";
test("Tsv native control corpus retains complete literal owned fields independently of external format",async()=>{
 
 const owned={schema:controlFixture.ownedSchema,records:[[],Array.from({length:controlFixture.workItems},()=>controlFixture.fieldText),[""],[]],trailingNewline:true,lineEnding:"crlf" as const};
 const database=await tsvSnapshotToSqliteDatabase(owned);const file=await exportSqliteDatabase(database);const independent=Database.deserialize(file);expect(database.tables.reduce((sum,table)=>sum+table.rows.length,0)).toBe(controlFixture.expectedRows);
 try{expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT schema FROM tsv_document").get()).toEqual({schema:controlFixture.ownedSchema});}finally{independent.close();}
 expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(file))).toEqual(owned);
});


test("tsv one literal Unicode field remains exact through independent SQLite storage",async()=>{
 
 const text=controlFixture.copyTextUnit.repeat(controlFixture.copyTextRepeat),bytes=new TextEncoder().encode(text);
 expect(bytes.length).toBe(controlFixture.copyTextBytes);
 expect(new TextDecoder("utf-8",{fatal:true}).decode(bytes.subarray(0,controlFixture.copyTextBoundary))).toBe(text.slice(0,controlFixture.copyTextBoundary/7*3));
 const owned={schema:controlFixture.ownedSchema,records:[[text]],trailingNewline:true,lineEnding:"crlf" as const};
 const file=await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(owned));const independent=Database.deserialize(file);
 try{
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(independent.query("SELECT value,length(CAST(value AS BLOB)) AS bytes FROM tsv_field").get()).toEqual({value:text,bytes:controlFixture.copyTextBytes});
  expect(independent.query("SELECT count(*) AS count FROM tsv_record").get()).toEqual({count:1});
  expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(owned);
 }finally{independent.close();}
});


import backingFixture from "../🧫️fixtures/💰️backing/🔣️.json";

import {NativeDecodeControl as BackingDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BackingAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
test("tsv authored relation indexes and literal octet admission have independent authorities",async()=>{
 expect(backingFixture["owner"]).toEqual("tsv");expect(backingFixture["policy"]["zeroOwnershipBytes"]).toEqual(0);expect(backingFixture["policy"]["refusalKind"]).toEqual("ownershipLimit");expect(backingFixture["policy"]["retirementRefund"]).toEqual(false);expect(backingFixture["policy"]["byteAuthority"]).toEqual("literalUTF8Bytes");expect(backingFixture["policy"]["indexAuthority"]).toEqual("authoredEntityRelations");
 const snapshot={schema:backingFixture.schema,trailingNewline:true,lineEnding:"crlf" as const,records:backingFixture.records.map(record=>backingFixture.fields.filter(field=>field.recordId===record.id).sort((a,b)=>a.ordinal-b.ordinal).map(field=>field.value))};
 const independent=Database.deserialize(await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(snapshot)));
 try{independent.exec("DELETE FROM tsv_field;DELETE FROM tsv_record;");for(const record of backingFixture.records)independent.query("INSERT INTO tsv_record VALUES(?,1,?)").run(record.id,record.ordinal);for(const field of backingFixture.fields)independent.query("INSERT INTO tsv_field VALUES(?,?,?,?)").run(field.id,field.recordId,field.ordinal,field.value);
  expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT r.id AS recordId,f.ordinal FROM tsv_record r JOIN tsv_field f ON f.record_id=r.id ORDER BY r.ordinal,f.ordinal").all()).toEqual([{recordId:7,ordinal:0},{recordId:2,ordinal:0},{recordId:2,ordinal:1}]);
  for(const field of backingFixture.fields)expect((independent.query("SELECT length(CAST(value AS BLOB)) AS bytes FROM tsv_field WHERE id=?").get(field.id)as{bytes:number}).bytes).toBe(Buffer.byteLength(field.value,"utf8"));
  expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(snapshot);
 }finally{independent.close();}
 const bytes=Buffer.from(backingFixture.fields[0]!.value,"utf8");expect(bytes.length).toBeGreaterThan(0);
 const zero=new BackingAllocationControl({maxAllocationBytes:backingFixture.policy.zeroOwnershipBytes});await expect(zero.stage(maximum=>new BackingDecodeControl(maximum,()=>true),control=>control.copyBytes(bytes))).rejects.toMatchObject({kind:backingFixture.policy.refusalKind});expect(zero.remainingBytes()).toBe(0);
 const exact=new BackingAllocationControl({maxAllocationBytes:bytes.length});{const copied=await exact.stage(maximum=>new BackingDecodeControl(maximum,()=>true),control=>control.copyBytes(bytes));expect(Buffer.from(copied)).toEqual(bytes);}expect(exact.remainingBytes()).toBe(0);await expect(exact.stage(maximum=>new BackingDecodeControl(maximum,()=>true),control=>control.copyBytes(Uint8Array.of(1)))).rejects.toMatchObject({kind:backingFixture.policy.refusalKind});expect(exact.remainingBytes()).toBe(0);
});

import {NativeDecodeControl as BudgetDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BudgetAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import logicalOwnerFixture from "../🧫️fixtures/🧠️logical-owner/🔣️.json";

import {parseTsvSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
test("TSV complete logical owner preserves empty row and document metadata in independent SQLite",async()=>{
 expect(logicalOwnerFixture["contract"]).toEqual("completeTsvLogicalOwner");expect(logicalOwnerFixture["encodings"]).toEqual(["binary","text"]);expect(logicalOwnerFixture["policy"]).toEqual({"preserveSchema":true,"preserveTrailingNewline":true,"preserveLineEnding":true,"preserveEmptyRecord":true,"nativeOwner":"declaredLogicalRecord","naturalFile":"ianaTsv"});expect(logicalOwnerFixture["edit"]).toEqual({"query":"UPDATE tsv_document SET line_ending='lf',trailing_newline=0","snapshotIndex":3});
 
 
 for(const[index,owner]of logicalOwnerFixture.snapshots.entries()){
  const snapshot=parseTsvSnapshot(owner);expect(owner).toEqual(snapshot);
  const file=await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(snapshot));const independent=Database.deserialize(file);
  try{
   expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
   expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
   expect(independent.query("SELECT schema,trailing_newline,line_ending FROM tsv_document").get()).toEqual({schema:owner.schema,trailing_newline:Number(owner.trailingNewline),line_ending:owner.lineEnding});
   expect(independent.query("SELECT r.ordinal,count(f.id) AS fields FROM tsv_record r LEFT JOIN tsv_field f ON f.record_id=r.id GROUP BY r.id ORDER BY r.ordinal").all()).toEqual(owner.records.map((record,ordinal)=>({ordinal,fields:record.length})));
   expect(independent.query("SELECT f.value FROM tsv_field f JOIN tsv_record r ON f.record_id=r.id ORDER BY r.ordinal,f.ordinal").all()).toEqual(owner.records.flatMap(record=>record.map(value=>({value}))));
   expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(snapshot);
   if(index===logicalOwnerFixture.edit.snapshotIndex){independent.run(logicalOwnerFixture.edit.query);expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual({...snapshot,lineEnding:"lf",trailingNewline:false});}
  }finally{independent.close();}
 }
 console.log("[DEBUG] TSV complete logical owners preserved schema, empty rows, literal fields and independent newline metadata through physical SQLite");
});

import {readClosedRecordPack,type ClosedValue} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🫳️preflight/🔮️pack/🟦️.ts";
import {readFileSync as readLogicalAsset} from "node:fs";
import {createRequire as requireIndependentTable} from "node:module";
const independentTable:{tsvParseRows:(text:string)=>string[][]}=requireIndependentTable(import.meta.url)("d3-dsv");
const readExternalRows=independentTable.tsvParseRows;
function readTSVLogicalAsset(bytes:Uint8Array){const wire=readClosedRecordPack(bytes,"stdio.tsv");return{schema:wire[0],records:wire[1],trailingNewline:wire[2],lineEnding:wire[3]===0?"lf":"crlf"};}
test("TSV canonical producer assets retain complete owner fields and remain separate from authored raw tables",()=>{
 for(const [index,owner]of logicalOwnerFixture.snapshots.entries()){
  const pack=readLogicalAsset(new URL("../🧫️fixtures/🧠️logical-owner/🎒️"+index+".pack.semio",import.meta.url));
  expect(readTSVLogicalAsset(pack)).toEqual(owner);
  const text=readLogicalAsset(new URL("../🧫️fixtures/🧠️logical-owner/🗣️"+index+".dsl.semio",import.meta.url),"utf8");
  expect(text.startsWith("semio stdio.tsv.dsl v1\n")).toBe(true);
  expect(text).toContain("schema=");
 }
 const assets=new URL("../../../../📚️examples/🎬️demo/🖼️assets",import.meta.url);
 const raw=readLogicalAsset(new URL("📊️.tsv",assets),"utf8"),rows=readExternalRows(raw);
 const demo=readTSVLogicalAsset(readLogicalAsset(new URL("🎒️.pack.semio",assets)));
 expect(demo.schema).toBe("stdio.tsv");
 expect(demo.records).toEqual(rows);
 console.log("[DEBUG] TSV independent DEFLATE, CRC32C and external table reader retained real canonical producer assets");
});

import SemanticAjv2020 from "ajv/dist/2020";
import completeSemantic from "../🧫️fixtures/🛂️semantic/🔣️.json";

test("tsv closed complete semantic cells agree with independent third-party SQL",async()=>{
 expect(completeSemantic["tableWidths"]["tsv_document"]).toEqual(4);expect(completeSemantic["tableWidths"]["tsv_record"]).toEqual(3);expect(completeSemantic["tableWidths"]["tsv_field"]).toEqual(4);
 for(const sample of completeSemantic.cases){const snapshot=sample.snapshot as Parameters<typeof tsvSnapshotToSqliteDatabase>[0];const database=await tsvSnapshotToSqliteDatabase(snapshot);expect(await tsvSnapshotFromSqliteDatabase(database)).toEqual(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);let rows=0,bytes=0;const quote=(v:string)=>'"'+v.replaceAll('"','""')+'"';
  for(const[table,width]of Object.entries(completeSemantic.tableWidths)){const fields=db.query("PRAGMA table_info("+quote(table)+")").all()as{name:string}[];expect(fields.length).toBe(width);const cells=fields.map(({name})=>{const f=quote(name);return "CASE typeof("+f+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+f+" AS BLOB)) WHEN 'blob' THEN length("+f+") ELSE 0 END";}).join("+");const extent=db.query("SELECT COUNT(*) AS rows,COALESCE(SUM("+cells+"),0) AS bytes FROM "+quote(table)).get()as{rows:number;bytes:number};rows+=extent.rows;bytes+=extent.bytes;}expect(rows).toBe(sample.rows);expect(bytes).toBe(sample.bytes);
 }finally{db.close();}}
});
