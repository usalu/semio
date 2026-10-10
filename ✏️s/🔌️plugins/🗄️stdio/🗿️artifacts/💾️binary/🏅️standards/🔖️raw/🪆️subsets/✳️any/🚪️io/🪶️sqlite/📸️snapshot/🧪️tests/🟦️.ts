/** 🧫️ Shared Binary semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import { binarySnapshotToSqliteDatabase, binarySnapshotFromSqliteDatabase, BINARY_SQLITE_SCHEMA } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";
import { parseBinarySnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";

test("Binary rejects independently edited invalid flags and ordinal gaps", async () => {
  const input = fixture;
  const db = Database.deserialize(await exportSqliteDatabase(await binarySnapshotToSqliteDatabase(input)));
  try {
    db.run("PRAGMA ignore_check_constraints=ON");
    db.run("UPDATE binary_byte SET value=256 WHERE ordinal=0");
    await expect(binarySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("byte");
    db.run("UPDATE binary_byte SET value=0 WHERE ordinal=0");
    db.run("UPDATE binary_byte SET ordinal=-1 WHERE id=1");
    await expect(binarySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("contiguous");
  } finally { db.close(); }
});


test("binary borrowed native preflight contract has literal octet and refusal authorities", async()=>{
 
 expect(controlFixture["preflight"]["encodings"]).toEqual(["binary","text"]);expect(controlFixture["preflight"]["ownershipBytes"]).toEqual(0);expect(controlFixture["preflight"]["byteAuthority"]).toEqual("actualLiteralOutputUTF8OrOctets");expect(controlFixture["preflight"]["refusalKinds"]["file"]).toEqual("ownershipLimit");expect(controlFixture["preflight"]["refusalKinds"]["cancellation"]).toEqual("canceled");expect(controlFixture["preflight"]["cancelAt"]).toEqual("firstBorrowedCheckpoint");expect(controlFixture["preflight"]["retirementRefund"]).toEqual(false);expect(controlFixture["expectedRows"]).toEqual(1025);expect(controlFixture["allocationRole"]).toEqual("cumulativeOwnedBacking");
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


test("Binary semantic SQLite schema, native roundtrip and independent SQL edit", async () => {
  expect(BINARY_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  const input = fixture;
  const database = await binarySnapshotToSqliteDatabase(input);
  expect(await binarySnapshotFromSqliteDatabase(database)).toEqual(input);
  const bytes = await exportSqliteDatabase(database);
  const db = Database.deserialize(bytes);
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT value FROM binary_byte ORDER BY ordinal").all()).toEqual(fixture.bytes.map((value) => ({ value })));
    db.run("UPDATE binary_byte SET value=254 WHERE ordinal=0");
    const result = await binarySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(result.bytes[0]).toBe(254);
    db.run("UPDATE binary_byte SET document_id=999 WHERE ordinal=0");
    await expect(binarySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("unknown");
  } finally { db.close(); }
});

test("Binary resource limits apply before semantic projection and reconstruction", async () => {
  const input = fixture;
  const database = await binarySnapshotToSqliteDatabase(input);
  await expect(binarySnapshotToSqliteDatabase(input, { maxRows: 0 })).rejects.toThrow("row limit");
  await expect(binarySnapshotToSqliteDatabase(input, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  await expect(binarySnapshotFromSqliteDatabase(database, { maxRows: 0 })).rejects.toThrow("row limit");
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    const independent = await importSqliteDatabase(new Uint8Array(db.serialize()));
    await expect(binarySnapshotFromSqliteDatabase(independent, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  } finally { db.close(); }
});

test("Binary projection and reconstruction cancellation", async () => {
  const input = fixture;
  const database = await binarySnapshotToSqliteDatabase(input);
  for (const direction of ["project", "reconstruct"]) {
    const controller = new AbortController();
    const options = { signal: controller.signal, onProgress: () => controller.abort() };
    await expect(direction === "project" ? binarySnapshotToSqliteDatabase(input, options) : binarySnapshotFromSqliteDatabase(database, options)).rejects.toMatchObject({ kind: "canceled" });
  }
});

test("Binary snapshot typed boundary requires integer byte arrays", () => {
  expect(parseBinarySnapshot(fixture)).toEqual(fixture);
  for (const byte of [-1, 256, 1.5, "0"]) expect(() => parseBinarySnapshot({ schema: fixture.schema, bytes: [byte] })).toThrow();
  expect(() => parseBinarySnapshot({ schema: fixture.schema, bytes: "00ff" })).toThrow();
});

import controlFixture from "../🧫️fixtures/🛬️native-control/🔣️.json";

import Ajv2020 from "ajv/dist/2020";
test("Binary native control corpus retains complete literal owned fields independently of external format",async()=>{
 
 const owned={schema:controlFixture.ownedSchema,bytes:Array.from({length:controlFixture.workItems},(_,i)=>controlFixture.bytePattern[i%4]!)};
 const database=await binarySnapshotToSqliteDatabase(owned);const file=await exportSqliteDatabase(database);const independent=Database.deserialize(file);expect(database.tables.reduce((sum,table)=>sum+table.rows.length,0)).toBe(controlFixture.expectedRows);
 try{expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT schema FROM binary_document").get()).toEqual({schema:controlFixture.ownedSchema});}finally{independent.close();}
 expect(await binarySnapshotFromSqliteDatabase(await importSqliteDatabase(file))).toEqual(owned);
});

import {NativeDecodeControl as BudgetDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BudgetAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import SemanticAjv2020 from "ajv/dist/2020";
import completeSemantic from "../🧫️fixtures/🛂️semantic/🔣️.json";

test("binary closed complete semantic cells agree with independent third-party SQL",async()=>{
 expect(completeSemantic["tableWidths"]["binary_document"]).toEqual(2);expect(completeSemantic["tableWidths"]["binary_byte"]).toEqual(4);
 for(const sample of completeSemantic.cases){const snapshot=sample.snapshot as Parameters<typeof binarySnapshotToSqliteDatabase>[0];const database=await binarySnapshotToSqliteDatabase(snapshot);expect(await binarySnapshotFromSqliteDatabase(database)).toEqual(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);let rows=0,bytes=0;const quote=(v:string)=>'"'+v.replaceAll('"','""')+'"';
  for(const[table,width]of Object.entries(completeSemantic.tableWidths)){const fields=db.query("PRAGMA table_info("+quote(table)+")").all()as{name:string}[];expect(fields.length).toBe(width);const cells=fields.map(({name})=>{const f=quote(name);return "CASE typeof("+f+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+f+" AS BLOB)) WHEN 'blob' THEN length("+f+") ELSE 0 END";}).join("+");const extent=db.query("SELECT COUNT(*) AS rows,COALESCE(SUM("+cells+"),0) AS bytes FROM "+quote(table)).get()as{rows:number;bytes:number};rows+=extent.rows;bytes+=extent.bytes;}expect(rows).toBe(sample.rows);expect(bytes).toBe(sample.bytes);
 }finally{db.close();}}
});

test("Original Binary receiving vectors have no whole-trial authority", async () => {
 const { existsSync } = await import("node:fs");
 expect(existsSync(new URL("../🫴️receiving/🧬️schema/🔣️.json",import.meta.url))).toBe(false);
});


test("Original Binary required paid receiver keeps the raw carrier and canonical supplied Grant", async () => {
 const { readFileSync } = await import("node:fs");
 const { resolve } = await import("node:path");
 const repo=resolve(import.meta.dir,"../../../../../../../../../../../../..");
 const original=JSON.parse(readFileSync(new URL("../🫴️receiving/🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const { semioSchemaAjvV1 } = await import(resolve(repo,"🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts"));
 const grant=JSON.parse(readFileSync(resolve(repo,"🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json"),"utf8"));
 expect(semioSchemaAjvV1({strict:true}).compile(grant)(original.original.grant)).toBe(true);
 for(const row of original.shapeCases.filter((row:any)=>row.carrier==="binary")){
  const raw=Uint8Array.from(row.raw),expected=Buffer.from(row.expected);
  expect(Buffer.from(raw)).toEqual(expected);expect(parseBinarySnapshot({schema:original.original.canonicalSchema,bytes:[...raw]}).bytes).toEqual([...expected]);
 }
 const source=readFileSync(new URL("../../../💾️binary/📸️snapshot/🦀️.rs",import.meta.url),"utf8");
 expect(source.includes("impl store::ArtifactPackReceiving for BinarySnapshot")).toBe(true);
 expect(source.includes("owner.receive::<Self,Self>")).toBe(true);
 expect(source.includes("bind_raw_pack(bytes,slot,native,body)")).toBe(true);
 expect(source.includes("NativeDecodeControl::new")).toBe(false);
 console.log("[DEBUG] Original required Binary receiver uses canonical supplied Grant and independent Buffer/Uint8Array raw-carrier output; actual System cancellation proof is separate");
});

test("current Binary examples have no whole-trial schema authority", async()=>{const {existsSync}=await import("node:fs");expect(existsSync(new URL("../🫴️receiving/🧫️fixtures/🧬️schema/🔣️.json",import.meta.url))).toBe(false);});
