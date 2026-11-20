import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {AsyncResource} from "node:async_hooks";
import {createNativePhysicalGateV1} from "../../🍎️image/📡️roster/🟦️.ts";
import {bindNativeStorageVerificationObservationV1,createNativeStorageVerificationTimingsV1,observeNativeStorageVerificationAwaitV1,withNativeStorageVerificationObservationV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import vectors from "./🔣️.json" with {type:"json"};

const verify=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/Snapshot"});

test("verification timing isolates retained asynchronous readers and preserves cancellation",async()=>{
 let now=0,late:(()=>Promise<void>)|undefined;
 const mounted=createNativeStorageVerificationTimingsV1(()=>now),fixed=createNativeStorageVerificationTimingsV1(()=>now),a=Promise.withResolvers<void>(),b=Promise.withResolvers<void>(),reason=Object.freeze({cancelled:true}),db=new Database(":memory:");
 try{
  const first=withNativeStorageVerificationObservationV1("mounted-image",mounted.observe,()=>{late=AsyncResource.bind(()=>observeNativeStorageVerificationAwaitV1("journalRead",async()=>{}));return observeNativeStorageVerificationAwaitV1("imageInventory",()=>a.promise);});
  now=3;
  const second=withNativeStorageVerificationObservationV1("directory-partition",fixed.observe,()=>observeNativeStorageVerificationAwaitV1("allocatedUsage",()=>b.promise));
  await observeNativeStorageVerificationAwaitV1("imageInventory",async()=>{});
  now=9;
  const pending={...mounted.stats(),...fixed.stats()};
  expect(verify(pending)).toBe(true);expect(pending).toEqual(vectors.pending);
  expect(Object.keys(mounted.stats())).toEqual(["mounted-image"]);expect(Object.keys(fixed.stats())).toEqual(["directory-partition"]);
  expect(db.query("select sum(json_extract(value,'$.milliseconds')) n from json_tree(?1) where type='object' and path like '$.%'").get(JSON.stringify(pending))).toEqual({n:15});
  const rejected=second.catch(error=>error);a.resolve();b.reject(reason);
  await first;expect(await rejected).toBe(reason);await late!();
  expect({...mounted.stats(),...fixed.stats()}).toEqual(vectors.settled);
 }finally{a.resolve();b.resolve();db.close();}
});

test("native gate reports existing full, joined and cached checks without extra calls",async()=>{
 let now=0,cheap=0,full=0;const timing=createNativeStorageVerificationTimingsV1(()=>now),gate=createNativePhysicalGateV1(()=>now),deferred=Promise.withResolvers<void>();
 await withNativeStorageVerificationObservationV1("mounted-image",timing.observe,async()=>{
  const call=()=>gate.run(500,()=>{cheap++;},()=>{full++;return deferred.promise;});
  const first=call();now=9;const joined=call();
  try{
   const during=timing.stats()["mounted-image"]!;
   expect(during.physicalGateVerify).toEqual({started:1,active:1,milliseconds:9});
   expect(during.physicalGateJoin).toEqual({started:1,active:1,milliseconds:0});
  }finally{now=10;deferred.resolve();await first;await joined;}
  await call();
 });
 const stats=timing.stats()["mounted-image"]!;
 expect(verify(timing.stats())).toBe(true);
 expect(Object.fromEntries(Object.keys(vectors.gate).map(phase=>[phase,stats[phase]!.started]))).toEqual(vectors.gate);
 expect(full).toBe(1);expect(cheap).toBe(3);
});

test("a diagnostic refusal cannot alter the verified result or replace a primary error",async()=>{
 const cause=Object.freeze({primary:true});
 const bad=()=>()=>{throw Error("diagnostics");};
 expect(await withNativeStorageVerificationObservationV1("mounted-image",bad,()=>observeNativeStorageVerificationAwaitV1("volumeIdentity",async()=>7))).toBe(7);
 await expect(withNativeStorageVerificationObservationV1("mounted-image",()=>{throw Error("observer");},()=>observeNativeStorageVerificationAwaitV1("imageInventory",async()=>{throw cause;}))).rejects.toBe(cause);
});

test("a deferred Native callback retains its observer or its explicit absence when another caller pumps it",async()=>{
 const owner=createNativeStorageVerificationTimingsV1(),pumper=createNativeStorageVerificationTimingsV1(),release=Promise.withResolvers<void>();
 const unobserved=bindNativeStorageVerificationObservationV1(()=>observeNativeStorageVerificationAwaitV1("journalRead",async()=>{}));
 let deferred:(()=>Promise<void>)|undefined;
 const work=withNativeStorageVerificationObservationV1("mounted-image",owner.observe,()=>{deferred=bindNativeStorageVerificationObservationV1(()=>observeNativeStorageVerificationAwaitV1("imageInventory",async()=>{}));return release.promise;});
 try{
  await withNativeStorageVerificationObservationV1("directory-partition",pumper.observe,async()=>{await unobserved();await deferred!();});
  expect(pumper.stats()).toEqual({});expect(Object.keys(owner.stats()["mounted-image"]!)).toEqual(["imageInventory"]);
  expect(owner.stats()["mounted-image"]!.imageInventory?.started).toBe(1);
 }finally{release.resolve();await work;}
 await deferred!();expect(owner.stats()["mounted-image"]!.imageInventory?.started).toBe(1);
});
