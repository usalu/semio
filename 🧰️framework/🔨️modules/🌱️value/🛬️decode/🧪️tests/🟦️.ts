import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {NativeDecodeControl} from "../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import stageFixture from "../🧫️fixtures/🪆️stage/🔣️.json";
import {stageOracle,type StageCase,type StageOperation,type StageResult} from "./🪆️stage/🟦️.ts";

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
