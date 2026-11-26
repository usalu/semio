import {expect,test} from "bun:test";
import {spawn} from "node:child_process";
import * as ts from "../../🟦️.ts";
import * as js from "../../🟨️.mjs";

/** 👯️ Contract: the plain-JavaScript twin (`🟨️.mjs`, for callers that run without a TypeScript loader) answers every vector exactly like the TypeScript identity module, so there is one definition of how a holder is judged.
 * @see ../../🟨️.mjs */
const T=Date.UTC(2026,10,25,17,25,10);
const tolerance=ts.PROCESS_IDENTITY_POLICY_V1.toleranceMilliseconds;
const reads:(number|"gone"|"unknown")[]=[T,T+tolerance,T+tolerance+1,T-60_000,T+17_000,"gone","unknown"];
const records:unknown[]=[{pid:10,startMs:T},{pid:10,startMs:0},{pid:10},{pid:0,startMs:T},{pid:1.5,startMs:T},{pid:10,startMs:-1},{pid:10,startMs:"1"},{startMs:T},null,"x",[]];
const outcome=(run:()=>unknown):unknown=>{try{return {value:run()};}catch(error){return {error:(error as Error).name,message:(error as Error).message};}};

test("both modules declare the same policy and export the same names",()=>{
 expect(js.PROCESS_IDENTITY_POLICY_V1).toEqual(ts.PROCESS_IDENTITY_POLICY_V1);
 const shared=["PROCESS_IDENTITY_POLICY_V1","ProcessIdentityRecordErrorV1","ownProcessIdentityV1","parsePosixProcessStartV1","parseProcessIdentityV1","processHolderAliveV1","processIdentityDeadV1","processIdentityForPidV1","processIdentityOfV1","processIdentityVerdictV1","processPidExistsV1","processRecordAliveV1","readProcessStartV1","requireProcessIdentityV1"];
 for(const name of shared){expect(name in js,name).toBe(true);expect(name in ts,name).toBe(true);}
});

test("record admission and the verdict agree on every vector",()=>{
 for(const record of records){
  expect(outcome(()=>js.requireProcessIdentityV1(record,"x")),JSON.stringify(record)).toEqual(outcome(()=>ts.requireProcessIdentityV1(record,"x")));
  expect(js.parseProcessIdentityV1(record),JSON.stringify(record)).toEqual(ts.parseProcessIdentityV1(record));
  for(const read of reads){
   expect(outcome(()=>js.processIdentityVerdictV1(record as never,read)),JSON.stringify([record,read])).toEqual(outcome(()=>ts.processIdentityVerdictV1(record as never,read)));
   expect(outcome(()=>js.processIdentityDeadV1(record as never,()=>read)),JSON.stringify([record,read])).toEqual(outcome(()=>ts.processIdentityDeadV1(record as never,()=>read)));
   expect(outcome(()=>js.processHolderAliveV1(record as never,()=>read)),JSON.stringify([record,read])).toEqual(outcome(()=>ts.processHolderAliveV1(record as never,()=>read)));
  }
 }
});

test("record liveness agrees, a record without creation time is judged by its pid",()=>{
 for(const pid of [process.pid,0,-3,1.5,"x",undefined,2_000_000_000])for(const start of [undefined,0,T,"x",process.pid]){
  expect(js.processRecordAliveV1(pid,start),JSON.stringify([pid,start])).toBe(ts.processRecordAliveV1(pid,start));
 }
 expect(ts.processRecordAliveV1(process.pid)).toBe(true);
 expect(ts.processRecordAliveV1(process.pid,ts.ownProcessIdentityV1().startMs)).toBe(true);
 expect(ts.processRecordAliveV1(process.pid,T)).toBe(false);
 expect(ts.processRecordAliveV1(0)).toBe(false);
});

test("the process table text is parsed alike",()=>{
 for(const text of ["Wed Nov 25 17:25:10 2026","  Thu Jan  1 00:00:00 1970 ","Wed Foo 25 17:25:10 2026","garbage",""])expect(js.parsePosixProcessStartV1(text),text).toBe(ts.parsePosixProcessStartV1(text));
});

test("reads of this process, a live child and a gone pid agree",async()=>{
 expect(js.ownProcessIdentityV1()).toEqual(ts.ownProcessIdentityV1());
 expect(js.readProcessStartV1(process.pid)).toBe(ts.readProcessStartV1(process.pid));
 expect(js.processIdentityForPidV1(process.pid)).toEqual(ts.processIdentityForPidV1(process.pid));
 const child=spawn(process.execPath,["-e","setTimeout(()=>{},30000)"],{stdio:"ignore"});
 try{
  await new Promise(resolve=>setTimeout(resolve,300));
  const found=ts.readProcessStartV1(child.pid!);
  expect(typeof found).toBe("number");
  expect(js.readProcessStartV1(child.pid!)).toBe(found);
  expect(js.processHolderAliveV1({pid:child.pid!,startMs:found as number})).toBe(true);
  expect(ts.processHolderAliveV1({pid:child.pid!,startMs:found as number})).toBe(true);
  expect(js.processHolderAliveV1({pid:child.pid!,startMs:(found as number)+3_600_000})).toBe(false);
  expect(ts.processHolderAliveV1({pid:child.pid!,startMs:(found as number)+3_600_000})).toBe(false);
  expect(js.processPidExistsV1(child.pid!)).toBe(ts.processPidExistsV1(child.pid!));
 }finally{child.kill();}
 await new Promise(resolve=>child.once("exit",resolve));
 expect(js.readProcessStartV1(child.pid!)).toBe(ts.readProcessStartV1(child.pid!));
 expect(js.processPidExistsV1(child.pid!)).toBe(false);
});
