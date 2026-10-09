import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {CargoController,type CargoControl,type CargoAccounting} from "../🟦️.ts";

const owner=join(import.meta.dir,".."),fixture=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")) as {limits:{maximumUnits:number;maximumOwnedBytes:number;maximumDepth:number};cases:{id:string;units:number;bytes:number;accepted:boolean}[]},oracle=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8")));

test("all original portable Cargo work and ownership vectors agree with independent schema",async()=>{
 for(const row of fixture.cases){
  const signal=new AbortController(),accounting:CargoAccounting={completed:0,ownedBytes:0},deadline=performance.now()+60000;let progress=0,yields=0;
  const operation:CargoControl={signal:signal.signal,...fixture.limits,remainingMilliseconds:()=>deadline-performance.now(),onProgress:()=>{progress++;},yieldContinuation:async()=>{yields++;await new Promise<void>(done=>setImmediate(done));}},control=new CargoController(operation,accounting);let accepted=true;
  try{await control.step("portable",row.id,row.units,row.bytes);}catch{accepted=false;}
  expect(oracle({units:row.units,bytes:row.bytes})).toBe(row.accepted);expect(accepted).toBe(row.accepted);expect(accounting).toEqual(row.accepted?{completed:row.units,ownedBytes:row.bytes}:{completed:0,ownedBytes:0});
  if(row.units===0&&row.accepted){expect(progress).toBe(1);expect(yields).toBe(1);}console.log("[DEBUG] Cargo finite control "+row.id+" admitted="+accepted);
 }
});

test("Cargo control refusal retains cumulative accounting and real cancellation at a yield",async()=>{
 let progress=0;const signal=new AbortController(),accounting:CargoAccounting={completed:0,ownedBytes:0},deadline=performance.now()+60000,control=new CargoController({signal:signal.signal,...fixture.limits,remainingMilliseconds:()=>deadline-performance.now(),onProgress:()=>{progress++;},yieldContinuation:async()=>{signal.abort();await new Promise<void>(done=>setImmediate(done));}},accounting);
 await control.step("first","owned",4,32);expect(accounting).toEqual({completed:4,ownedBytes:32});
 await expect(control.step("over","owned",7,0)).rejects.toThrow();expect(accounting).toEqual({completed:4,ownedBytes:32});
 await expect(control.finish("finish","owned")).rejects.toThrow();expect(signal.signal.aborted).toBe(true);expect(progress).toBe(1);expect(accounting).toEqual({completed:4,ownedBytes:32});
});

test("Cargo remaining-time authority refuses before ownership and after real yielding",async()=>{for(const remaining of[0,-1,NaN,Infinity]){const accounting={completed:0,ownedBytes:0},control=new CargoController({signal:new AbortController().signal,...fixture.limits,remainingMilliseconds:()=>remaining,onProgress:()=>{},yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))},accounting);await expect(control.step("deadline","owned",1,32)).rejects.toThrow();expect(accounting).toEqual({completed:0,ownedBytes:0});}let remaining=60000;const accounting={completed:0,ownedBytes:0},control=new CargoController({signal:new AbortController().signal,...fixture.limits,remainingMilliseconds:()=>remaining,onProgress:()=>{},yieldContinuation:async()=>{await new Promise<void>(done=>setImmediate(done));remaining=0;}},accounting);await expect(control.finish("deadline","owned")).rejects.toThrow();expect(accounting).toEqual({completed:0,ownedBytes:0});});
