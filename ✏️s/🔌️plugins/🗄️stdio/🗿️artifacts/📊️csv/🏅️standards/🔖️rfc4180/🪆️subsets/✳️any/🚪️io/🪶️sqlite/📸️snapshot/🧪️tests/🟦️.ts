/** 🧫️ Shared Csv semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import { csvSnapshotToSqliteDatabase, csvSnapshotFromSqliteDatabase, CSV_SQLITE_SCHEMA } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

test("Csv rejects independently edited invalid flags and ordinal gaps", async () => {
  const input = fixture;
  const db = Database.deserialize(await exportSqliteDatabase(await csvSnapshotToSqliteDatabase(input)));
  try {
    db.run("PRAGMA ignore_check_constraints=ON");
    db.run("UPDATE csv_document SET has_header=2");
    await expect(csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("boolean");
    db.run("UPDATE csv_document SET has_header=0");
    db.run("UPDATE csv_field SET ordinal=-1 WHERE id=1");
    await expect(csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("contiguous");
  } finally { db.close(); }
});


test("csv borrowed native preflight contract has literal octet and refusal authorities", async()=>{
 
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


test("Csv semantic SQLite schema, native roundtrip and independent SQL edit", async () => {
  expect(CSV_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  const input = fixture;
  const database = await csvSnapshotToSqliteDatabase(input);
  expect(await csvSnapshotFromSqliteDatabase(database)).toEqual(input);
  const bytes = await exportSqliteDatabase(database);
  const db = Database.deserialize(bytes);
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT f.value,f.quoted FROM csv_field f JOIN csv_record r ON r.id=f.record_id ORDER BY r.ordinal,f.ordinal").all()).toEqual(fixture.records.flatMap((record) => record.fields.map((field) => ({ value: field.value, quoted: Number(field.quoted) }))));
    db.run("UPDATE csv_field SET value='edited semantic field' WHERE id=1");
    const result = await csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(result.records[0]!.fields[0]!.value).toBe("edited semantic field");
    db.run("UPDATE csv_field SET record_id=999 WHERE id=1");
    await expect(csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("unknown");
  } finally { db.close(); }
});

test("Csv resource limits apply before semantic projection and reconstruction", async () => {
  const input = fixture;
  const database = await csvSnapshotToSqliteDatabase(input);
  await expect(csvSnapshotToSqliteDatabase(input, { maxRows: 0 })).rejects.toThrow("row limit");
  await expect(csvSnapshotToSqliteDatabase(input, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  await expect(csvSnapshotFromSqliteDatabase(database, { maxRows: 0 })).rejects.toThrow("row limit");
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    const independent = await importSqliteDatabase(new Uint8Array(db.serialize()));
    await expect(csvSnapshotFromSqliteDatabase(independent, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  } finally { db.close(); }
});

test("Csv projection and reconstruction cancellation", async () => {
  const countingController = new AbortController();
  let countEvents = 0;
  await expect(csvSnapshotToSqliteDatabase({ ...fixture, records: Array.from({ length: 600 }, () => fixture.records[0]!) }, { signal: countingController.signal, onProgress: (progress) => { if (progress.completed === 0 && ++countEvents === 2) countingController.abort(); } })).rejects.toMatchObject({ kind: "canceled" });
  expect(countEvents).toBe(2);
  const input = fixture;
  const database = await csvSnapshotToSqliteDatabase(input);
  for (const direction of ["project", "reconstruct"]) {
    const controller = new AbortController();
    const options = { signal: controller.signal, onProgress: () => controller.abort() };
    await expect(direction === "project" ? csvSnapshotToSqliteDatabase(input, options) : csvSnapshotFromSqliteDatabase(database, options)).rejects.toMatchObject({ kind: "canceled" });
  }
});

import controlFixture from "../🧫️fixtures/🛬️native-control/🔣️.json";

import Ajv2020 from "ajv/dist/2020";
test("Csv native control corpus retains complete literal owned fields independently of external format",async()=>{
 
 const owned={schema:controlFixture.ownedSchema,hasHeader:false,records:[{fields:[]},{fields:Array.from({length:controlFixture.workItems},()=>({value:controlFixture.fieldText,quoted:false}))},{fields:[{value:"",quoted:true}]},{fields:[]}]};
 const database=await csvSnapshotToSqliteDatabase(owned);const file=await exportSqliteDatabase(database);const independent=Database.deserialize(file);expect(database.tables.reduce((sum,table)=>sum+table.rows.length,0)).toBe(controlFixture.expectedRows);
 try{expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT schema FROM csv_document").get()).toEqual({schema:controlFixture.ownedSchema});}finally{independent.close();}
 expect(await csvSnapshotFromSqliteDatabase(await importSqliteDatabase(file))).toEqual(owned);
});


test("csv one literal Unicode field remains exact through independent SQLite storage",async()=>{
 
 const text=controlFixture.copyTextUnit.repeat(controlFixture.copyTextRepeat),bytes=new TextEncoder().encode(text);
 expect(bytes.length).toBe(controlFixture.copyTextBytes);
 expect(new TextDecoder("utf-8",{fatal:true}).decode(bytes.subarray(0,controlFixture.copyTextBoundary))).toBe(text.slice(0,controlFixture.copyTextBoundary/7*3));
 const owned={schema:controlFixture.ownedSchema,hasHeader:false,records:[{fields:[{value:text,quoted:true}]}]};
 const file=await exportSqliteDatabase(await csvSnapshotToSqliteDatabase(owned));const independent=Database.deserialize(file);
 try{
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(independent.query("SELECT value,length(CAST(value AS BLOB)) AS bytes FROM csv_field").get()).toEqual({value:text,bytes:controlFixture.copyTextBytes});
  expect(independent.query("SELECT count(*) AS count FROM csv_record").get()).toEqual({count:1});
  expect(await csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(owned);
 }finally{independent.close();}
});


import backingFixture from "../🧫️fixtures/💰️backing/🔣️.json";

import {NativeDecodeControl as BackingDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BackingAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
test("csv authored relation indexes and literal octet admission have independent authorities",async()=>{
 expect(backingFixture["owner"]).toEqual("csv");expect(backingFixture["policy"]["zeroOwnershipBytes"]).toEqual(0);expect(backingFixture["policy"]["refusalKind"]).toEqual("ownershipLimit");expect(backingFixture["policy"]["retirementRefund"]).toEqual(false);expect(backingFixture["policy"]["byteAuthority"]).toEqual("literalUTF8Bytes");expect(backingFixture["policy"]["indexAuthority"]).toEqual("authoredEntityRelations");
 const snapshot={schema:backingFixture.schema,hasHeader:false,records:backingFixture.records.map(record=>({fields:backingFixture.fields.filter(field=>field.recordId===record.id).sort((a,b)=>a.ordinal-b.ordinal).map(field=>({value:field.value,quoted:field.quoted}))}))};
 const independent=Database.deserialize(await exportSqliteDatabase(await csvSnapshotToSqliteDatabase(snapshot)));
 try{independent.exec("DELETE FROM csv_field;DELETE FROM csv_record;");for(const record of backingFixture.records)independent.query("INSERT INTO csv_record VALUES(?,1,?)").run(record.id,record.ordinal);for(const field of backingFixture.fields)independent.query("INSERT INTO csv_field VALUES(?,?,?,?,?)").run(field.id,field.recordId,field.ordinal,field.value,Number(field.quoted));
  expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT r.id AS recordId,f.ordinal FROM csv_record r JOIN csv_field f ON f.record_id=r.id ORDER BY r.ordinal,f.ordinal").all()).toEqual([{recordId:7,ordinal:0},{recordId:2,ordinal:0},{recordId:2,ordinal:1}]);
  for(const field of backingFixture.fields)expect((independent.query("SELECT length(CAST(value AS BLOB)) AS bytes FROM csv_field WHERE id=?").get(field.id)as{bytes:number}).bytes).toBe(Buffer.byteLength(field.value,"utf8"));
  expect(await csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(snapshot);
 }finally{independent.close();}
 const bytes=Buffer.from(backingFixture.fields[0]!.value,"utf8");expect(bytes.length).toBeGreaterThan(0);
 const zero=new BackingAllocationControl({maxAllocationBytes:backingFixture.policy.zeroOwnershipBytes});await expect(zero.stage(maximum=>new BackingDecodeControl(maximum,()=>true),control=>control.copyBytes(bytes))).rejects.toMatchObject({kind:backingFixture.policy.refusalKind});expect(zero.remainingBytes()).toBe(0);
 const exact=new BackingAllocationControl({maxAllocationBytes:bytes.length});{const copied=await exact.stage(maximum=>new BackingDecodeControl(maximum,()=>true),control=>control.copyBytes(bytes));expect(Buffer.from(copied)).toEqual(bytes);}expect(exact.remainingBytes()).toBe(0);await expect(exact.stage(maximum=>new BackingDecodeControl(maximum,()=>true),control=>control.copyBytes(Uint8Array.of(1)))).rejects.toMatchObject({kind:backingFixture.policy.refusalKind});expect(exact.remainingBytes()).toBe(0);
});

import {NativeDecodeControl as BudgetDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BudgetAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import logicalOwnerFixture from "../🧫️fixtures/🧠️logical-owner/🔣️.json";

import { parseCsvSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
test("Csv complete logical owner preserves authored metadata and empty records in independent SQLite",async()=>{
 expect(logicalOwnerFixture["contract"]).toEqual("completeCsvLogicalOwner");expect(logicalOwnerFixture["encodings"]).toEqual(["binary","text"]);expect(logicalOwnerFixture["policy"]).toEqual({"preserveSchema":true,"preserveHasHeader":true,"preserveQuoted":true,"preserveEmptyRecord":true,"nativeOwner":"declaredLogicalRecord","naturalFile":"rfc4180"});expect(logicalOwnerFixture["edit"]).toEqual({"query":"UPDATE csv_document SET has_header=1","snapshotIndex":0});
 
 
 for(const [index,owner]of logicalOwnerFixture.snapshots.entries()){
  expect(parseCsvSnapshot(owner)).toEqual(owner);
  const file=await exportSqliteDatabase(await csvSnapshotToSqliteDatabase(owner));const independent=Database.deserialize(file);
  try{
   expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
   expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
   expect(independent.query("SELECT schema,has_header FROM csv_document").get()).toEqual({schema:owner.schema,has_header:Number(owner.hasHeader)});
   expect(independent.query("SELECT r.ordinal,count(f.id) AS fields FROM csv_record r LEFT JOIN csv_field f ON f.record_id=r.id GROUP BY r.id ORDER BY r.ordinal").all()).toEqual(owner.records.map((record,ordinal)=>({ordinal,fields:record.fields.length})));
   expect(independent.query("SELECT f.value,f.quoted FROM csv_field f JOIN csv_record r ON r.id=f.record_id ORDER BY r.ordinal,f.ordinal").all()).toEqual(owner.records.flatMap(record=>record.fields.map(field=>({value:field.value,quoted:Number(field.quoted)}))));
   expect(await csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(owner);
   if(index===logicalOwnerFixture.edit.snapshotIndex){independent.run(logicalOwnerFixture.edit.query);expect(await csvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual({...owner,hasHeader:true});}
  }finally{independent.close();}
 }
 console.log("[DEBUG] Csv complete logical owners retained literal schema, header flag, quote flags and zero-field record occurrences through independent SQLite");
});

import {readClosedRecordPack,type ClosedValue} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🫳️preflight/🔮️pack/🟦️.ts";
import {readFileSync as readLogicalAsset} from "node:fs";
import {createRequire as requireIndependentTable} from "node:module";
const independentTable:{csvParseRows:(text:string)=>string[][]}=requireIndependentTable(import.meta.url)("d3-dsv");
const readExternalRows=independentTable.csvParseRows;
function readCSVLogicalAsset(bytes:Uint8Array){const wire=readClosedRecordPack(bytes,"stdio.csv");const records=wire[2] as {[id:number]:ClosedValue}[];return{schema:wire[0],hasHeader:wire[1],records:records.map(row=>({fields:(row[0] as {[id:number]:ClosedValue}[]).map(field=>({value:field[0],quoted:field[1]}))}))};}
test("CSV canonical producer assets retain complete owner fields and remain separate from authored raw tables",()=>{
 for(const [index,owner]of logicalOwnerFixture.snapshots.entries()){
  const pack=readLogicalAsset(new URL("../🧫️fixtures/🧠️logical-owner/🎒️"+index+".pack.semio",import.meta.url));
  expect(readCSVLogicalAsset(pack)).toEqual(owner);
  const text=readLogicalAsset(new URL("../🧫️fixtures/🧠️logical-owner/🗣️"+index+".dsl.semio",import.meta.url),"utf8");
  expect(text.startsWith("semio stdio.csv.dsl v1\n")).toBe(true);
  expect(text).toContain("schema=");
 }
 const assets=new URL("../../../../📚️examples/🎬️demo/🖼️assets",import.meta.url);
 const raw=readLogicalAsset(new URL("🧪️example/📊️.csv",assets),"utf8"),rows=readExternalRows(raw);
 const demo=readCSVLogicalAsset(readLogicalAsset(new URL("🎒️.pack.semio",assets)));
 expect(demo.schema).toBe("stdio.csv");
 expect(demo.records.map(row=>row.fields.map(field=>field.value))).toEqual(rows);
 console.log("[DEBUG] CSV independent DEFLATE, CRC32C and external table reader retained real canonical producer assets");
});

import SemanticAjv2020 from "ajv/dist/2020";
import completeSemantic from "../🧫️fixtures/🛂️semantic/🔣️.json";

test("csv closed complete semantic cells agree with independent third-party SQL",async()=>{
 expect(completeSemantic["tableWidths"]["csv_document"]).toEqual(3);expect(completeSemantic["tableWidths"]["csv_record"]).toEqual(3);expect(completeSemantic["tableWidths"]["csv_field"]).toEqual(5);
 for(const sample of completeSemantic.cases){const snapshot=sample.snapshot as Parameters<typeof csvSnapshotToSqliteDatabase>[0];const database=await csvSnapshotToSqliteDatabase(snapshot);expect(await csvSnapshotFromSqliteDatabase(database)).toEqual(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);let rows=0,bytes=0;const quote=(v:string)=>'"'+v.replaceAll('"','""')+'"';
  for(const[table,width]of Object.entries(completeSemantic.tableWidths)){const fields=db.query("PRAGMA table_info("+quote(table)+")").all()as{name:string}[];expect(fields.length).toBe(width);const cells=fields.map(({name})=>{const f=quote(name);return "CASE typeof("+f+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+f+" AS BLOB)) WHEN 'blob' THEN length("+f+") ELSE 0 END";}).join("+");const extent=db.query("SELECT COUNT(*) AS rows,COALESCE(SUM("+cells+"),0) AS bytes FROM "+quote(table)).get()as{rows:number;bytes:number};rows+=extent.rows;bytes+=extent.bytes;}expect(rows).toBe(sample.rows);expect(bytes).toBe(sample.bytes);
 }finally{db.close();}}
});
