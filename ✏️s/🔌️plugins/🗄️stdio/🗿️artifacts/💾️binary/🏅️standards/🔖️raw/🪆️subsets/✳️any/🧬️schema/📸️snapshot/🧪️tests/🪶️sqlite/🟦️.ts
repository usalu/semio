/** 🧫️ Shared Binary semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { binarySnapshotToSqliteDatabase, binarySnapshotFromSqliteDatabase, BINARY_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";
import { parseBinarySnapshot } from "../../🟦️.ts";

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
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(controlSchema);
 expect(validate(controlFixture)).toBe(true);
 const policy=controlFixture.preflight;
 expect(policy).toEqual({"encodings":["binary","text"],"ownershipBytes":0,"byteAuthority":"actualLiteralOutputUTF8OrOctets","refusalKinds":{"file":"ownershipLimit","cancellation":"canceled"},"cancelAt":"firstBorrowedCheckpoint","retirementRefund":false});
 expect(validate({...controlFixture,preflight:{...policy,ownershipBytes:1}})).toBe(false);
 expect(validate({...controlFixture,preflight:{...policy,refusalKinds:{...policy.refusalKinds,file:"workLimit"}}})).toBe(false);
 const literal=controlFixture.fieldText,octets=Buffer.from(literal,"utf8");
 expect(Buffer.byteLength(literal,"utf8")).toBe(new TextEncoder().encode(literal).length);
 expect(octets.toString("utf8")).toBe(literal);
 const independent=new Database(":memory:");
 try{independent.run("CREATE TABLE literal_output(value TEXT NOT NULL)");independent.run("INSERT INTO literal_output VALUES(?)",[literal]);expect(independent.query("SELECT length(CAST(value AS BLOB)) AS bytes FROM literal_output").get()).toEqual({bytes:octets.byteLength});}finally{independent.close();}
 expect(policy.ownershipBytes).toBe(0);expect(policy.retirementRefund).toBe(false);
 expect(controlFixture.allocationRole).toBe("cumulativeOwnedBacking");
 expect(controlFixture.maxAllocationBytes).toBe(1);
 expect(validate({...controlFixture,allocationRole:"semanticValueBytes"})).toBe(false);
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
  expect(BINARY_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
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

import controlFixture from "../../🧫️fixtures/🪶️sqlite/🛬️native-control/🔣️.json";
import controlSchema from "../../🧫️fixtures/🪶️sqlite/🛬️native-control/🧬️schema/🔣️.json";
import Ajv2020 from "ajv/dist/2020";
test("Binary native control corpus retains complete literal owned fields independently of external format",async()=>{
 expect(new Ajv2020({allErrors:true,strict:true}).compile(controlSchema)(controlFixture)).toBe(true);
 const owned={schema:controlFixture.ownedSchema,bytes:Array.from({length:controlFixture.workItems},(_,i)=>controlFixture.bytePattern[i%4]!)};
 const database=await binarySnapshotToSqliteDatabase(owned);const file=await exportSqliteDatabase(database);const independent=Database.deserialize(file);expect(database.tables.reduce((sum,table)=>sum+table.rows.length,0)).toBe(controlFixture.expectedRows);
 try{expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT schema FROM binary_document").get()).toEqual({schema:controlFixture.ownedSchema});}finally{independent.close();}
 expect(await binarySnapshotFromSqliteDatabase(await importSqliteDatabase(file))).toEqual(owned);
});

import {NativeDecodeControl as BudgetDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BudgetAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
