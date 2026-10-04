import { Database } from "bun:sqlite";
import {deflateRawSync as independentFixedDeflate,deflateSync as independentZlib,inflateSync as independentInflate,constants as independentZlibConstants} from "node:zlib";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { parseDeflateSnapshot } from "../../🟦️.ts";
import { DEFLATE_SQLITE_SCHEMA, deflateSnapshotToSqliteDatabase, deflateSnapshotFromSqliteDatabase } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

test("Deflate declared header and decompressed byte entities expose shared editable SQL", async () => {
  expect(DEFLATE_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const snapshot = parseDeflateSnapshot(fixture);
  const database = await deflateSnapshotToSqliteDatabase(snapshot);
  expect(await deflateSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT d.dictionary_adler32,b.value FROM deflate_document d JOIN deflate_payload_byte b ON b.document_id=d.id WHERE b.ordinal=2").get()).toEqual({ dictionary_adler32: 4294967295, value: 255 });
    db.run("UPDATE deflate_payload_byte SET value=42 WHERE ordinal=2");
    db.run("UPDATE deflate_document SET dictionary_adler32=NULL");
    const edited = await deflateSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited).toEqual({ ...snapshot, dictId: undefined, payload: [0,65,42,128,17] });
  } finally { db.close(); }
});


test("deflate borrowed native preflight contract has literal octet and refusal authorities", async()=>{
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(controlSchema);
 expect(validate(controlFixture)).toBe(true);
 const policy=controlFixture.preflight;
 expect(policy).toEqual({"encodings":["binary","text"],"ownershipBytes":0,"byteAuthority":"fixedHuffmanCapacityAtLeastLiteralOutput","refusalKinds":{"file":"ownershipLimit","cancellation":"canceled"},"cancelAt":"firstBorrowedCheckpoint","retirementRefund":false});
 expect(validate({...controlFixture,preflight:{...policy,ownershipBytes:1}})).toBe(false);
 expect(validate({...controlFixture,preflight:{...policy,refusalKinds:{...policy.refusalKinds,file:"workLimit"}}})).toBe(false);
 const literal=controlFixture.fieldText,octets=Buffer.from(literal,"utf8");
 expect(Buffer.byteLength(literal,"utf8")).toBe(new TextEncoder().encode(literal).length);
 expect(octets.toString("utf8")).toBe(literal);
 const payload=Buffer.from(Array.from({length:controlFixture.workItems},(_,index)=>controlFixture.bytePattern[index%controlFixture.bytePattern.length]!));
 const bound=Math.floor((9*payload.length+17)/8),compressed=independentFixedDeflate(payload,{strategy:independentZlibConstants.Z_FIXED});
 expect(compressed.length).toBeLessThanOrEqual(bound);
 expect(controlFixture.nativeInput).toEqual({compressionMethod:8,windowBits:7,dictionaryPresent:false,dictionaryRefusal:"unsupportedOwner"});
 expect(validate({...controlFixture,nativeInput:{...controlFixture.nativeInput,dictionaryPresent:true}})).toBe(false);
 const framed=independentZlib(payload);expect(framed[0]!&15).toBe(controlFixture.nativeInput.compressionMethod);expect(framed[0]!>>4).toBe(controlFixture.nativeInput.windowBits);expect((framed[1]!&32)!==0).toBe(controlFixture.nativeInput.dictionaryPresent);expect(independentInflate(framed)).toEqual(payload);
 const dictionaryFrame=Buffer.concat([framed.subarray(0,2),Buffer.from([255,255,255,255]),framed.subarray(2)]);const high=(dictionaryFrame[1]!&0xc0)|32;dictionaryFrame[1]=high+(31-(dictionaryFrame[0]!*256+high)%31)%31;
 expect(()=>independentInflate(dictionaryFrame)).toThrow();expect(controlFixture.dictionaryAdler32).toBe(4294967295);

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

test("Deflate scalar widths, ownership, budgets and cancellation remain bounded", async () => {
  const snapshot = parseDeflateSnapshot(fixture);
  await expect(deflateSnapshotToSqliteDatabase({ ...snapshot, compressionMethod: 16 })).rejects.toThrow();
  await expect(deflateSnapshotToSqliteDatabase({ ...snapshot, payload: [256] })).rejects.toThrow();
  await expect(deflateSnapshotToSqliteDatabase(snapshot, { maxValueBytes: 0 })).rejects.toThrow();
  const database = await deflateSnapshotToSqliteDatabase(snapshot);
  await expect(deflateSnapshotFromSqliteDatabase(database, { maxValueBytes: 0 })).rejects.toThrow();
  for (const alteration of ["UPDATE deflate_payload_byte SET document_id=999 WHERE ordinal=0", "UPDATE deflate_payload_byte SET ordinal=0 WHERE ordinal=1", "UPDATE deflate_document SET compression_level_hint=4"]) {
    const db = Database.deserialize(await exportSqliteDatabase(database));
    try { db.run("PRAGMA ignore_check_constraints=ON"); db.run(alteration); await expect(deflateSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); } finally { db.close(); }
  }
  const controller = new AbortController();
  await expect(deflateSnapshotToSqliteDatabase({ ...snapshot, payload: new Array<number>(2000).fill(1) }, { signal: controller.signal, onProgress: event => { if (event.completed >= 256) controller.abort(); } })).rejects.toHaveProperty("kind", "canceled");
});

import controlFixture from "../../🧫️fixtures/🪶️sqlite/🛬️native-control/🔣️.json";
import controlSchema from "../../🧫️fixtures/🪶️sqlite/🛬️native-control/🧬️schema/🔣️.json";
import Ajv2020 from "ajv/dist/2020";
test("Deflate neutral owned headers and dictionary retain reserved literal fields through independent SQLite",async()=>{
 expect(new Ajv2020({allErrors:true,strict:true}).compile(controlSchema)(controlFixture)).toBe(true);
 const owned=parseDeflateSnapshot({schema:controlFixture.ownedSchema,compressionMethod:controlFixture.compressionMethod,windowBits:controlFixture.windowBits,compressionLevelHint:"maximum",dictId:controlFixture.dictionaryAdler32,payload:Array.from({length:controlFixture.workItems},(_,i)=>controlFixture.bytePattern[i%4]!)});
 const database=await deflateSnapshotToSqliteDatabase(owned);const file=await exportSqliteDatabase(database);const independent=Database.deserialize(file);expect(database.tables.reduce((sum,table)=>sum+table.rows.length,0)).toBe(controlFixture.expectedRows);
 try{expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT schema,compression_method,window_bits,dictionary_adler32 FROM deflate_document").get()).toEqual({schema:controlFixture.ownedSchema,compression_method:15,window_bits:15,dictionary_adler32:4294967295});}finally{independent.close();}
 expect(await deflateSnapshotFromSqliteDatabase(await importSqliteDatabase(file))).toEqual(owned);
});

test("Deflate long literal schema admits interior cancellation in both relational directions",async()=>{
 const snapshot=parseDeflateSnapshot({...fixture,schema:"x".repeat(controlFixture.copyBytes)});
 const database=await deflateSnapshotToSqliteDatabase(snapshot);
 for(const phase of ["projectSnapshot","reconstructSnapshot"] as const){
  const controller=new AbortController();let observed=false;
  const options={signal:controller.signal,onProgress:(event:{phase:string;completed:number;total:number})=>{if(event.phase===phase&&event.completed>=controlFixture.copyCancelAfter&&event.completed<event.total){observed=true;controller.abort();}}};
  await expect(phase==="projectSnapshot"?deflateSnapshotToSqliteDatabase(snapshot,options):deflateSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(observed).toBe(true);
 }
});

import {NativeDecodeControl as BudgetDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BudgetAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
