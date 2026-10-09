import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {NativeDecodeControl} from "../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import stageFixture from "../🧫️fixtures/🪆️stage/🔣️.json";
import {stageOracle,type StageCase,type StageOperation,type StageResult} from "./🪆️stage/🟦️.ts";

test("original shared actor lease authority preserves UTF8 and admits no replacement frame",async()=>{
 const fixture=(await import("../../📝️shared-utf8/♻️original-lease/🧫️fixtures/🔣️.json")).default;
 const schema=(await import("../../📝️shared-utf8/♻️original-lease/🧬️schema/🔣️.json")).default;
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const bad of [{...fixture,unknown:true},{...fixture,grant:{...fixture.grant,maximumCapacityBytes:1}},{...fixture,values:["changed"]}])expect(validate(bad)).toBe(false);
 for(const text of fixture.values)for(const capacity of fixture.capacities)for(const aliases of fixture.aliases){
  const db=new Database(":memory:");try{
   db.exec("CREATE TABLE actors(value TEXT NOT NULL,leases INTEGER NOT NULL,capacity INTEGER NOT NULL,frames INTEGER NOT NULL,CHECK(leases>=0));");
   db.run("INSERT INTO actors VALUES(?,?,?,0)",[text,aliases+1,capacity]);let finalOwners=0;
   for(let position=0;position<=aliases;position++){db.run("UPDATE actors SET leases=leases-1");const row=db.query("SELECT value,leases,capacity,frames FROM actors").get()as {value:string;leases:number;capacity:number;frames:number};expect(row.value).toBe(text);expect(row.frames).toBe(fixture.expected.closeHeapBytes);if(row.leases===0){finalOwners++;expect(new TextEncoder().encode(row.value).length).toBe(Buffer.byteLength(text,"utf8"));}}
   expect(finalOwners).toBe(1);expect(db.query("SELECT leases,frames FROM actors").get()).toEqual({leases:0,frames:0});
  }finally{db.close();}
 }
 console.log("[DEBUG] original shared actor lease: strict independent authority, SQLite lease custody preserves NUL/Unicode, zero replacement frames and one final original owner");
});


test("borrowed native scopes intersect retained turn capacity before every original allocation",async()=>{
 const fixture=(await import("../📏️maximum/🧫️fixtures/🔣️.json")).default;
 const schema=(await import("../📏️maximum/🧬️schema/🔣️.json")).default;
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const bad of [{...fixture,unknown:true},{...fixture,initialBytes:-1},{...fixture,cases:fixture.cases.slice(1)},{...fixture,cases:fixture.cases.map((row,index)=>index===0?{...row,ownedBytes:-1}:row)}])expect(validate(bad)).toBe(false);
 for(const row of fixture.cases){
  const db=new Database(":memory:");let outcome="complete";
  try{
   db.exec("CREATE TABLE ledger(owned INTEGER NOT NULL,ceiling INTEGER NOT NULL,CHECK(owned<=ceiling));");
   db.run("INSERT INTO ledger VALUES(?,?)",[fixture.initialBytes,fixture.firstTurnCapacityBytes]);
   db.run("UPDATE ledger SET ceiling=min(ceiling,?)",[fixture.scopeMaximumBytes]);
   const turn=()=>db.run("UPDATE ledger SET ceiling=min(?,owned+?)",[fixture.scopeMaximumBytes,fixture.nextTurnCapacityBytes]);
   const charge=(bytes:number)=>db.run("UPDATE ledger SET owned=owned+?",[bytes]);
   try{turn();if(row.id==="repeat"){charge(3);turn();charge(1);}else if(row.id==="nested"){db.run("UPDATE ledger SET ceiling=9");charge(2);db.run("UPDATE ledger SET ceiling=10");charge(1);}else if(row.id==="refuse")charge(4);else if(row.id==="cancel")outcome="canceled";else{charge(3);outcome="unwind";}}catch{outcome="limit";}
   db.run("UPDATE ledger SET ceiling=?",[fixture.firstTurnCapacityBytes]);expect(outcome).toBe(row.outcome);expect(db.query("SELECT owned,ceiling FROM ledger").get()).toEqual({owned:row.ownedBytes,ceiling:fixture.restoredMaximumBytes});
  }finally{db.close();}
 }
 console.log("[DEBUG] native borrowed scope maximum: strict corpus, repeated and nested original turn admissions, SQLite pre-allocation refusal and receipt restoration agree");
});


test("nested native stages preserve parent workloads, cancellation and cumulative ownership",async()=>{
  const classify=(error:unknown)=>{const message=(error as Error).message;const labels=new Map([["native decoding exceeded declared stage workload","overrun"],["native decoding canceled","canceled"],["native decoding ownership exceeds caller limit","limit"],["owned child rejected","rejected"]]);const label=labels.get(message);if(!label)throw error;return label;};
  for(const c of stageFixture.cases as StageCase[]){
    let progress:StageResult["progress"]={completed:0,total:0,ownedBytes:0};const errors:string[]=[];
    const control=new NativeDecodeControl(c.maximumBytes,p=>{progress=p;return !c.cancel||p.total!==c.cancel.total||p.completed<c.cancel.at;});
    const run=async(operations:StageOperation[]):Promise<void>=>{for(const op of operations){
      if(op.kind==="begin")await control.beginStage(op.units);
      else if(op.kind==="advance")await control.advance(op.units);
      else if(op.kind==="charge")await control.charge(op.units);
      else if(op.kind==="reject")throw new Error("owned child rejected");
      else try{await control.scopedStage(()=>run(op.operations));}catch(error){errors.push(classify(error));}
    }};
    try{await run(c.operations);}catch(error){errors.push(classify(error));}await control.checkpoint();
    const actual={progress,errors};expect(actual).toEqual(c.expected);expect(actual).toEqual(stageOracle(c));
  }
});

test("native materialization retains cumulative bounds and interior cancellation",async()=>{
  const database=new Database(":memory:");database.run("CREATE TABLE charge(position INTEGER PRIMARY KEY,bytes INTEGER NOT NULL)");
  try{for(const c of fixture.cases){
    const control=new NativeDecodeControl(c.maximumBytes,()=>true);let acceptedCharges=0;
    for(const bytes of c.charges){try{await control.charge(bytes);}catch{break;}acceptedCharges++;}
    database.run("DELETE FROM charge");let independentAccepted=0,independentBytes=0;
    for(const bytes of c.charges){database.run("INSERT INTO charge VALUES(?,?)",[independentAccepted+1,bytes]);const total=(database.query("SELECT SUM(bytes) AS n FROM charge").get() as {n:number}).n;if(total>c.maximumBytes)break;independentAccepted++;independentBytes=total;}
    expect([acceptedCharges,control.ownedBytes]).toEqual([independentAccepted,independentBytes]);expect([acceptedCharges,control.ownedBytes]).toEqual([c.acceptedCharges,c.ownedBytes]);
  }}finally{database.close();}
  const scopeCase=fixture.cases.find(c=>c.id==="cumulative")!,scoped=new NativeDecodeControl(fixture.workload.total,()=>true);await expect(scoped.scopedMaximum(scopeCase.maximumBytes,async control=>{for(const bytes of scopeCase.charges)await control.charge(bytes);})).rejects.toThrow("caller limit");expect(scoped.ownedBytes).toBe(scopeCase.ownedBytes);expect(scoped.maximumBytes).toBe(fixture.workload.total);await scoped.charge(scopeCase.charges[1]);expect(scoped.ownedBytes).toBe(scopeCase.ownedBytes+scopeCase.charges[1]);await expect(scoped.scopedMaximum(0,async()=>{throw new Error("operation must not run");})).rejects.toThrow("stage limit");expect(scoped.maximumBytes).toBe(fixture.workload.total);
  const copyEvents:number[]=[];const copyCanceled=new NativeDecodeControl(1_000_000,p=>{if(p.total!==fixture.copyWorkload.total)return true;copyEvents.push(p.completed);return p.completed<fixture.copyWorkload.cancelAt;});await expect(copyCanceled.copyBytes(new Uint8Array(fixture.copyWorkload.total))).rejects.toThrow("canceled");expect(copyEvents).toEqual([0,fixture.copyWorkload.cancelAt]);expect(fixture.copyWorkload.cancelAt).toBeLessThan(fixture.copyWorkload.total);
  const denied=new NativeDecodeControl(1_000_000,()=>false);await expect(denied.copyBytes(new Uint8Array(65537))).rejects.toThrow("canceled");expect(denied.ownedBytes).toBe(0);
  const early=new NativeDecodeControl(16,()=>false);await expect(early.copyBytes(new Uint8Array(1))).rejects.toThrow("canceled");expect(early.ownedBytes).toBe(0);
  const observed:number[]=[];const canceled=new NativeDecodeControl(1_000_000,p=>{expect(p.total).toBe(fixture.workload.total);observed.push(p.completed);return p.completed<fixture.workload.cancelAt;});await canceled.beginStage(fixture.workload.total);for(let i=0;i<fixture.workload.cancelAt-1;i++)await canceled.step();await expect(canceled.step()).rejects.toThrow("canceled");expect(observed).toEqual(fixture.workload.checkpoints);expect(fixture.workload.cancelAt).toBeLessThan(fixture.workload.total);
  const limited=new NativeDecodeControl(16,()=>true);await expect(limited.admitSlots(2048,24)).rejects.toThrow("caller limit");expect(limited.ownedBytes).toBe(0);
  await limited.charge(12);await limited.beginStage(4);await expect(limited.charge(8)).rejects.toThrow("caller limit");expect(limited.ownedBytes).toBe(12);
  const signal=new AbortController(),interrupted=new NativeDecodeControl(1_000_000,()=>true,signal.signal);const pending=interrupted.copyBytes(new Uint8Array(65537));signal.abort();await expect(pending).rejects.toThrow("canceled");expect(interrupted.ownedBytes).toBe(0);
  const shared=new NativeDecodeControl(100000,()=>true);const simultaneous=await Promise.allSettled([shared.copyBytes(new Uint8Array(65537)),shared.copyBytes(new Uint8Array(65537))]);expect(simultaneous.filter(result=>result.status==="fulfilled")).toHaveLength(1);expect(shared.ownedBytes).toBe(65537);
  const parent=new NativeDecodeControl(64,()=>true),n=fixture.nestedStage;await parent.beginStage(n.parentTotal);await parent.advance(n.parentBefore);await parent.scopedStage(async child=>{await child.beginStage(n.childTotal);await child.charge(n.childBytes);await child.advance(n.childTotal);});await parent.advance(n.parentAfter);expect(parent.ownedBytes).toBe(n.childBytes);await expect(parent.scopedStage(async child=>{await child.beginStage(n.childTotal);throw new Error("owned child failed");})).rejects.toThrow("owned child failed");await parent.advance(n.parentTotal-n.parentBefore-n.parentAfter);await expect(parent.step()).rejects.toThrow("declared stage workload");
  for(const c of fixture.utf8){const control=new NativeDecodeControl(0,()=>true),bytes=new Uint8Array(c.bytes);let independent=true,actual=true;try{new TextDecoder("utf-8",{fatal:true}).decode(bytes);}catch{independent=false;}try{await control.validateUtf8(bytes);}catch{actual=false;}expect(actual).toBe(independent);expect(actual).toBe(c.accepted);expect(control.ownedBytes).toBe(0);}
  let utf8Interior=false;const utf8Canceled=new NativeDecodeControl(0,p=>{if(p.completed===fixture.copyWorkload.cancelAt&&p.total===fixture.copyWorkload.total){utf8Interior=true;return false;}return true;});await expect(utf8Canceled.validateUtf8(new TextEncoder().encode(fixture.copyWorkload.text.repeat(fixture.copyWorkload.repetitions)))).rejects.toThrow("canceled");expect(utf8Interior).toBe(true);
});


test("borrowed native observers preserve original cancellation, complete cumulative debit and unwind restoration",async()=>{
  const {default:fixture}=await import("../🧫️fixtures/🔭️observer/🔣️.json");
  for(const sample of fixture.cases){
    const parentEvents:number[]=[],observerEvents:number[]=[];let active=false,capturing=false;
    const control=new NativeDecodeControl(sample.maximumBytes,event=>{if(capturing)parentEvents.push(event.ownedBytes);return !active||sample.parentRejectAt===null||event.ownedBytes<sample.parentRejectAt;});
    await control.charge(sample.initialBytes);capturing=true;active=true;let outcome="complete";
    try{await control.scopedObserver(event=>{observerEvents.push(event.ownedBytes);return sample.observerRejectAt===null||event.ownedBytes<sample.observerRejectAt;},async child=>{
      expect(child).toBe(control);await child.beginStage(0);await child.charge(sample.chargeBytes);if(sample.unwindAfterCharge)throw Error("owned observer scope unwound");await child.checkpoint();
    });}catch(error){outcome=sample.unwindAfterCharge?"unwound":(error as {kind:string}).kind;}
    active=false;await control.checkpoint();
    const actual={outcome,ownedBytes:control.ownedBytes,maximumBytes:control.maximumBytes,parentEvents,observerEvents};expect(actual).toEqual(sample.expected);
    const database=new Database(":memory:");try{
      database.exec("CREATE TABLE admission(owned INTEGER NOT NULL, ceiling INTEGER NOT NULL, CHECK(owned<=ceiling))");database.run("INSERT INTO admission VALUES(?,?)",[sample.initialBytes,sample.maximumBytes]);
      const refusedBefore=sample.parentRejectAt!==null&&sample.initialBytes>=sample.parentRejectAt||sample.observerRejectAt!==null&&sample.initialBytes>=sample.observerRejectAt;
      if(!refusedBefore)database.run("UPDATE admission SET owned=owned+? WHERE owned+?<=ceiling",[sample.chargeBytes,sample.chargeBytes]);
      expect(database.query("SELECT owned FROM admission").get()).toEqual({owned:actual.ownedBytes});
    }finally{database.close();}
  }
  console.log("[DEBUG] Five complete borrowed-observer vectors preserve scope identity, cancellation and unwind restoration with independent SQLite ownership admission");
});


test("native_detached_receiving_preserves_original_cumulative_ledger", async () => {
 const fixture=(await import("../🧫️fixtures/🔌️detached/🔣️.json")).default;
 const schema=(await import("../🧬️schema/🔌️detached/🔣️.json")).default;
 expect(new Ajv({strict:true,allErrors:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.run("CREATE TABLE original_receipt(maximum INTEGER, owned INTEGER, recipient INTEGER)");
  db.run("INSERT INTO original_receipt VALUES(?,?,1)",[fixture.maximumBytes,fixture.firstOwnedBytes]);
  const receipt=db.query("SELECT maximum,owned,recipient FROM original_receipt").get();
  db.run("UPDATE original_receipt SET owned=owned+? WHERE recipient=2 AND owned+?<=maximum",[fixture.secondOwnedBytes,fixture.secondOwnedBytes]);
  expect(db.query("SELECT maximum,owned,recipient FROM original_receipt").get()).toEqual(receipt);
  db.run("UPDATE original_receipt SET owned=owned+? WHERE recipient=1 AND owned+?<=maximum",[fixture.secondOwnedBytes,fixture.secondOwnedBytes]);
  expect(db.query("SELECT maximum,owned,recipient FROM original_receipt").get()).toEqual({maximum:fixture.maximumBytes,owned:fixture.finalOwnedBytes,recipient:1});
  expect(JSON.parse(JSON.stringify(fixture))).toEqual(fixture);
  console.error("[DEBUG] original detached native receipt retains ceiling, cumulative ownership and recipient through independent SQLite/JSON");
 }finally{db.close();}
});
