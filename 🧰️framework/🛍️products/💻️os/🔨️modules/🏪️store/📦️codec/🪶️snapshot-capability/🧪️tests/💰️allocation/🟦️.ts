/** 💰️ Independently checks literal byte ownership and cumulative stage settlement. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { isUtf8 } from "node:buffer";

import fixture from "./🧫️fixtures/🔣️.json";
import Ajv from "ajv";
import grantSchema from "../../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
test("original snapshot producer spans debit four physical currencies without replenishing depth",()=>{
 const neutral=JSON.parse(readFileSync(resolve(import.meta.dir,"../../../../../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧫️fixtures/🔣️.json"),"utf8"));const policy=neutral.producer;const validate=new Ajv({strict:true,allErrors:true}).compile(grantSchema);expect(validate(policy.walletGrant)).toBe(true);expect(validate(policy.remainingGrant)).toBe(true);expect(policy.custody).toBe("originalDestinationBeforeCopy");expect(policy.receipt).toBe("actualDestinationOnSuccessAndRefusal");expect(policy.publication).toBe("moveOriginal");
 const db=new Database(":memory:");try{db.exec("CREATE TABLE producer_span(ordinal INTEGER PRIMARY KEY,items INTEGER NOT NULL,copy_bytes INTEGER NOT NULL,capacity_bytes INTEGER NOT NULL,release_bytes INTEGER NOT NULL)");for(const[index,span]of policy.receipts.entries())db.query("INSERT INTO producer_span VALUES(?,?,?,?,?)").run(index,span.copiedItems,span.copiedBytes,span.retainedCapacityBytes,span.releasedBytes);const remaining=db.query("SELECT ?1-SUM(items) AS maximumItems,?2-SUM(copy_bytes) AS maximumCopyBytes,?3-SUM(capacity_bytes) AS maximumCapacityBytes,?4-SUM(release_bytes) AS maximumReleaseBytes,?5 AS maximumDepth FROM producer_span").get(policy.walletGrant.maximumItems,policy.walletGrant.maximumCopyBytes,policy.walletGrant.maximumCapacityBytes,policy.walletGrant.maximumReleaseBytes,policy.walletGrant.maximumDepth);expect(remaining).toEqual(policy.remainingGrant);expect(policy.copyBytes).toBe(new TextEncoder().encode(neutral.copy).length);for(const axis of grantSchema.required){const missing={...policy.walletGrant};delete missing[axis];expect(validate(missing)).toBe(false);}}finally{db.close();}
 console.error("[DEBUG] Original producer neutral spans match independent SQLite subtraction and closed five-axis Ajv policy; native parser body accounting remains separately unqualified");
});

test("original buffer constructor custody uses an independent closed five-axis caller policy",()=>{
 const contract=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🪶️native-decoding/🧫️fixtures/🔣️.json"),"utf8"));
 const validate=new Ajv({strict:true,allErrors:true}).compile(grantSchema);expect(validate(contract.callerGrant)).toBe(true);
 expect(contract.constructorCustody).toEqual({partial:"originalRecipient",register:"beforeFirstCopy",output:"moveOriginal",cancelAfterBuffers:1});
 for(const axis of grantSchema.required){const missing={...contract.callerGrant};delete missing[axis];expect(validate(missing)).toBe(false);expect(validate({...contract.callerGrant,[axis]:-1})).toBe(false);expect(validate({...contract.callerGrant,[axis]:0.5})).toBe(false);expect(validate({...contract.callerGrant,[axis]:0})).toBe(true);}
 expect(validate({...contract.callerGrant,extra:1})).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE buffer_owner(case_id TEXT NOT NULL,ordinal INTEGER NOT NULL,octets INTEGER NOT NULL,PRIMARY KEY(case_id,ordinal))");const insert=db.query("INSERT INTO buffer_owner VALUES(?,?,?)");for(const row of contract.cases){for(const[index,bytes]of row.bufferBytes.entries())insert.run(row.id,index,bytes);const total=db.query("SELECT COALESCE(SUM(octets),0) AS bytes FROM buffer_owner WHERE case_id=?").get(row.id) as {bytes:number};expect(total.bytes).toBe(row.bufferBytes.reduce((n:number,b:number)=>n+b,0));expect(contract.callerGrant.maximumCapacityBytes).not.toBe(row.maximumBytes);}}finally{db.close();}
 console.error("[DEBUG] Original buffer constructor neutral custody and independent five-axis grant agree with Ajv/SQLite; expansion ceilings remain separate");
});



test("borrowed validation and admitted binding have distinct cancellation frontiers", () => {
  const literal = fixture.literal.unit.repeat(fixture.literal.repeat);
  const bytes = new TextEncoder().encode(literal);
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE literal(text TEXT NOT NULL)");
    database.query("INSERT INTO literal VALUES(?)").run(literal);
    const measured = database.query("SELECT length(CAST(text AS BLOB)) AS bytes FROM literal").get() as { bytes: number };
    expect(isUtf8(bytes)).toBe(true);
    expect(measured.bytes).toBe(fixture.literal.utf8Bytes);
    for (const stage of fixture.cancellation.stages) {
      const borrowed = stage.scope === "borrowed-validation";
      const admitted = fixture.cancellation.priorAdmissionBytes + (borrowed ? 0 : measured.bytes);
      expect(stage.ownedLiteralBytes).toBe(borrowed ? 0 : measured.bytes);
      let remaining = admitted - fixture.cancellation.priorAdmissionBytes;
      if (borrowed) {
        expect(isUtf8(bytes)).toBe(true);
        expect(remaining).toBe(0);
        expect(measured.bytes).toBeGreaterThan(remaining);
      } else {
        expect(measured.bytes).toBeLessThanOrEqual(remaining);
        remaining -= measured.bytes;
        expect(remaining).toBe(0);
        expect(measured.bytes).toBeGreaterThan(remaining);
      }
    }
  } finally { database.close(); }
});

test("independent SQLite preserves UTF-8 semantics separately from allocation stages", () => {
  const literal = fixture.literal.unit.repeat(fixture.literal.repeat);
  const bytes = new TextEncoder().encode(literal);
  expect(bytes.length).toBe(fixture.literal.utf8Bytes);
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE literal(id INTEGER PRIMARY KEY, text TEXT NOT NULL)");
    database.query("INSERT INTO literal VALUES(1,?)").run(literal);
    expect(database.query("SELECT length(CAST(text AS BLOB)) AS bytes FROM literal").get()).toEqual({ bytes: bytes.length });
    expect(database.query("SELECT text FROM literal").get()).toEqual({ text: literal });
    let boundary = 65536;
    while ((bytes[boundary] & 0xc0) === 0x80) boundary--;
    expect(boundary).toBeGreaterThan(fixture.literal.cancelAfterBytes);
    expect(boundary).toBeLessThan(bytes.length);
    expect(new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(0, boundary))).toBe(literal.slice(0, new TextDecoder().decode(bytes.subarray(0, boundary)).length));
    for (const item of fixture.ledgerCases) {
      let remaining = item.maximum;
      for (const stage of item.stages) {
        expect(stage.owned).toBeLessThanOrEqual(remaining);
        remaining -= stage.owned;
      }
      expect(remaining).toBe(item.remaining);
    }
  } finally { database.close(); }
});

import refusalFixture from "../../../../../../../../🔨️modules/🌱️value/⚠️refusal/🧫️fixtures/🔣️.json";
import deflateFixture from "../../../../../../../../🔨️modules/🎒️pack/⚠️error/🧫️fixtures/🧭️cause/📡️codec/🔣️.json";
import {deflateRawSync,inflateRawSync} from "node:zlib";
import {PackError} from "../../../../../../../../🔨️modules/🎒️pack/⚠️error/🟦️.ts";
import {ValueError} from "../../../../../../../../🔨️modules/🌱️value/⚠️refusal/🟦️.ts";

test("retained Deflate physical credit has an explicit closed ownership authority",()=>{
 const admission=deflateFixture.retainedAdmission;
 const physical=admission.physicalHistory,semantic=admission.semanticLength;
 expect(physical.expectedRawBytes).toBeLessThanOrEqual(physical.semanticSegmentBytes);
 expect(physical.historyTargetBytes).toBeGreaterThan(physical.maximumAllocationBytes);
 expect(semantic.expectedRawBytes).toBeGreaterThan(semantic.semanticSegmentBytes);
 const database=new Database(":memory:");
 try{
  for(const row of [physical,semantic]){
   const bytes=Buffer.alloc(row.expectedRawBytes,97),stored=deflateRawSync(bytes);
   expect(inflateRawSync(stored)).toEqual(bytes);
   const reference=database.query("SELECT CASE ?1 WHEN 'ownedPhysicalCeiling' THEN 'ownershipLimit' WHEN 'semanticSegmentLength' THEN 'workLimit' END AS kind").get(row.origin) as {kind:string};
   expect(reference.kind).toBe(row.expectedKind);
   if(row.expectedKind!=="ownershipLimit"&&row.expectedKind!=="workLimit")throw Error("unknown authored retained admission kind");
   const cause=new ValueError(row.expectedKind,"same deliberately misleading cancellation/depth prose"),error=PackError.fromValue(cause);
   expect(error.cause).toBe(cause);
   expect(cause.kind).toBe(reference.kind);
  }
 }finally{database.close();}
});

test("eight neutral refusal authorities retain literal identity in independent SQLite",()=>{
 const cases=refusalFixture.cases.filter(item=>item.operation==="construct");expect(cases.length).toBe(9);expect(new Set(cases.map(item=>item.kind)).size).toBe(8);const database=new Database(":memory:");try{database.exec("CREATE TABLE refusal(id INTEGER PRIMARY KEY,kind TEXT NOT NULL,message TEXT NOT NULL)");cases.forEach((item,id)=>database.query("INSERT INTO refusal VALUES(?,?,?)").run(id,item.kind,item.message));expect(database.query("SELECT kind,message FROM refusal ORDER BY id").all()).toEqual(cases.map(item=>({kind:item.kind,message:item.message})));}finally{database.close();}
});
