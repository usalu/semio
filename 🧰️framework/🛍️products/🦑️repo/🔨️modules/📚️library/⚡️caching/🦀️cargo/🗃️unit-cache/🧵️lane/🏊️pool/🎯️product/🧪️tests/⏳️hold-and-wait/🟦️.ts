import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {acquireProductLaneV1,awaitProductLaneFreeV1,ProductLaneBusyErrorV1,ProductLaneWaitBoundErrorV1,retryWhileProductLaneBusyV1} from "../../🟦️.ts";
import {laneMessageV1} from "../../../../../🚪️launch/🟦️.ts";
import {compilerProductToolchainDigestV1} from "../../../../../../../💾️storage/🔒️enforcement/🏗️compiler/🎯️product/🟦️.ts";
import {createHash} from "node:crypto";
import fixture from "./🔣️.json" with {type:"json"};

const scratch=()=>{const base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});return mkdtempSync(join(base,"hold-and-wait-"));},owner={session:"0123456789ab.w",agent:"waiter"},request={key:fixture.key,crates:["same"],quotaBytes:Number.MAX_SAFE_INTEGER,owner};
const until=async(test:()=>boolean,limit=20000)=>{for(const started=Date.now();!test();){if(Date.now()-started>limit)throw Error("condition not reached");await new Promise(done=>setTimeout(done,20));}};
const startHolder=(root:string,marks:string)=>Bun.spawn([process.execPath,resolve(import.meta.dir,"📥️holder.ts"),root,fixture.key,marks],{stdout:"inherit",stderr:"inherit"});

test("a waiter in another process holds nothing while it queues for the product lane (lane first, scopes second)",async()=>{
 const root=scratch(),marks=join(root,"marks"),log:string[]=[],signal=new AbortController().signal;mkdirSync(marks);
 const holder=startHolder(root,marks);
 try{
  await until(()=>existsSync(join(marks,"holder-ready")));log.push(fixture.events.holderReady);
  let open=0,entered=0;const reports:string[]=[];
  const attempt=async()=>{open++;entered++;log.push(fixture.events.scopeEntered);try{const lane=await acquireProductLaneV1(root,request,{signal,onHeld:"refuse"});log.push(fixture.events.laneTaken);return lane;}finally{open--;log.push(fixture.events.scopeLeft);}};
  const waiting=retryWhileProductLaneBusyV1(attempt,{signal,noticeMilliseconds:100,report:(kind,parameters)=>{reports.push(laneMessageV1(kind,parameters,"en"));expect(open).toBe(0);}});
  await until(()=>reports.length>0);expect(open).toBe(0);expect(reports[0]).toMatch(new RegExp(fixture.holders.pattern));
  writeFileSync(join(marks,"holder-release"),"1");
  const lane=await waiting;lane.release();expect(entered).toBeGreaterThan(1);expect(open).toBe(0);
  const first=log.indexOf(fixture.events.laneTaken);expect(log.slice(0,first).filter(event=>event===fixture.events.scopeEntered).length-1).toBe(log.slice(0,first).filter(event=>event===fixture.events.scopeLeft).length);
  const oracle=Bun.spawnSync(["python3","-c","import json,sys;log=json.loads(sys.argv[1]);depth=0\nfor event in log:\n  depth+=event.endswith('scope-entered')-event.endswith('scope-left')\n  assert depth>=0\n  if event.endswith('lane-taken'): break\nprint(depth)",JSON.stringify(log)]);
  expect(oracle.exitCode).toBe(0);expect(Number(oracle.stdout.toString().trim())).toBe(1);
 }finally{writeFileSync(join(marks,"holder-release"),"1");await holder.exited;rmSync(root,{recursive:true,force:true});}
});

test("the legacy wait, which queues inside its scopes, is the hold-and-wait: it keeps the scope open while the other process holds the lane",async()=>{
 const root=scratch(),marks=join(root,"marks"),signal=new AbortController().signal;mkdirSync(marks);
 const holder=startHolder(root,marks);
 try{
  await until(()=>existsSync(join(marks,"holder-ready")));
  let open=0;const attempt=async()=>{open++;try{return await acquireProductLaneV1(root,request,{signal});}finally{open--;}};
  const waiting=attempt();await new Promise(done=>setTimeout(done,500));expect(open).toBe(1);
  writeFileSync(join(marks,"holder-release"),"1");(await waiting).release();
 }finally{writeFileSync(join(marks,"holder-release"),"1");await holder.exited;rmSync(root,{recursive:true,force:true});}
});

test("a refusing acquisition names the holder and the wait is bounded and reports in both languages",async()=>{
 const root=scratch(),marks=join(root,"marks"),signal=new AbortController().signal;mkdirSync(marks);
 const holder=startHolder(root,marks);
 try{
  await until(()=>existsSync(join(marks,"holder-ready")));
  const error=await acquireProductLaneV1(root,request,{signal,onHeld:"refuse"}).catch(reason=>reason);
  expect(error).toBeInstanceOf(ProductLaneBusyErrorV1);expect(error.holders).toMatch(new RegExp(fixture.holders.pattern));expect(error.holders).toContain(`pid ${holder.pid}`);
  const bound=await awaitProductLaneFreeV1(root,fixture.key,{signal,boundMilliseconds:1000,noticeMilliseconds:50}).catch(reason=>reason);
  expect(bound).toBeInstanceOf(ProductLaneWaitBoundErrorV1);expect(bound.message).toContain("longer than the 1 s allowed");expect(bound.message).toContain("länger als die erlaubten 1 s");expect(bound.message).toContain(`pid ${holder.pid}`);
 }finally{writeFileSync(join(marks,"holder-release"),"1");await holder.exited;rmSync(root,{recursive:true,force:true});}
});

test("a waiter that holds nothing is never bounded by default and keeps reporting the holder until the lane is free",async()=>{
 const root=scratch(),marks=join(root,"marks"),signal=new AbortController().signal,reports:string[]=[];mkdirSync(marks);
 const holder=startHolder(root,marks);
 try{
  await until(()=>existsSync(join(marks,"holder-ready")));
  const waiting=awaitProductLaneFreeV1(root,fixture.key,{signal,noticeMilliseconds:40,report:(kind,parameters)=>{reports.push(laneMessageV1(kind,parameters,"de"));}});
  await until(()=>reports.length>=3);writeFileSync(join(marks,"holder-release"),"1");await waiting;
  expect(reports.every(text=>text.includes(`pid ${holder.pid}`)&&!text.includes("Grenze"))).toBe(true);
 }finally{writeFileSync(join(marks,"holder-release"),"1");await holder.exited;rmSync(root,{recursive:true,force:true});}
});

test("the product toolchain digest derives from the pinned toolchain file alone, before any store is open",()=>{
 const root=scratch();
 try{
  writeFileSync(join(root,"rust-toolchain.toml"),"[toolchain]\nchannel = \"1.90.0\"\n");
  const first=compilerProductToolchainDigestV1(root);
  expect(first).toBe(createHash("sha256").update("semio.cargo.product-toolchain/v1\n").update("[toolchain]\nchannel = \"1.90.0\"\n").update("\n"+process.platform+"\n"+process.arch).digest("hex"));
  expect(compilerProductToolchainDigestV1(root)).toBe(first);
  writeFileSync(join(root,"rust-toolchain.toml"),"[toolchain]\nchannel = \"1.91.0\"\n");
  expect(compilerProductToolchainDigestV1(root)).not.toBe(first);
 }finally{rmSync(root,{recursive:true,force:true});}
});
