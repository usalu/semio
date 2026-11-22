import {test,expect} from "bun:test";
import {recordToolsProgress,sampleToolsProgress,toolsGraceActive} from "../../🟦️.ts";
import {join} from "node:path";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,renameSync,unlinkSync,symlinkSync} from "node:fs";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {readLeaseRegistryEntriesV1,leaseProcessStartedAtMsV1,leaseRegistryFolderV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-remote-execution",maximumElapsedMilliseconds:30000});
test("issued tools progress publishes through the existing held census",()=>withBuildTestInvocationV1(async original=>{
 const previousRegistry=process.env.SEMIO_LEASE_REGISTRY_DIRECTORY,previousOwner=process.env.SEMIO_LEASE_OWNER;
 try{
 const base=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"r-")),registry=join(base,"r"),directory=join(base,"l"),root=join(base,"s");
 mkdirSync(root);
 process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=registry;
 process.env.SEMIO_LEASE_OWNER="ready:abcdefgh12:tools:"+Date.now();
 const phase=createIssuedResourceControlV1(original,"repository:command",0),native="native-fixed-image:"+join(root,"d"),bindings="javascript-dependency-bindings:"+root;
 const acquire=(resource:string)=>acquireResourceLease({control:phase,signal:phase.signal,directory,resource,mode:"exclusive",timeoutMs:0,maximumDatabaseBytes:16384});
 const folder=leaseRegistryFolderV1(registry,directory),sample=(patch:Partial<Parameters<typeof sampleToolsProgress>[0]>={})=>sampleToolsProgress({folder,directory,owner:process.env.SEMIO_LEASE_OWNER!,parent:native,source:bindings,started:leaseProcessStartedAtMsV1,now:Date.now(),tolerance:2000,members:[{pid:process.pid,startMs:leaseProcessStartedAtMsV1}],...patch});
 const read=()=>readLeaseRegistryEntriesV1(registry,directory,Date.now(),false).find(row=>row.pid===process.pid);
 const wait=async(check:()=>boolean)=>{
  for(let i=0;i<80;i++){if(check())return;await Bun.sleep(20);}
  throw Error("Expected live registry progress missing");
 };
 const parent=await acquire(native),gate=await acquire(bindings);
 try{
  await phase.publish({resource:"dependencies:funded-ownership:state:501",mode:"exclusive",elapsedMs:0});
  await wait(()=>read()?.standing.done===501);
  const row=read()!,first=sample();expect(first?.done).toBe(501);
  expect(row.owner).toBe(process.env.SEMIO_LEASE_OWNER);expect(row.startedAtMs).toBe(leaseProcessStartedAtMsV1);
  expect(row.holds.map(x=>[x.resource,x.mode]).sort()).toEqual([[bindings,"exclusive"],[native,"exclusive"]]);
  expect(row.standing.phase).toBe("dependencies:funded-ownership:state");
  await Bun.sleep(5100);
  await phase.publish({resource:"dependencies:funded-ownership:state:565",mode:"exclusive",elapsedMs:0});
  await wait(()=>read()?.standing.done===565);
  expect(read()!.standing.phaseStartedAtMs).toBe(row.standing.phaseStartedAtMs);
  expect(sample()?.done).toBe(565);
  expect(sample({owner:"unrelated-owner"})).toBeUndefined();
  expect(sample({directory:join(base,"unrelated")})).toBeUndefined();
  expect(sample({members:[{pid:process.pid,startMs:leaseProcessStartedAtMsV1+5000}]})).toBeUndefined();
  expect(sample({now:Date.now()+310000})).toBeUndefined();
  const file=join(folder,process.pid+"-"+leaseProcessStartedAtMsV1+".json"),bytes=readFileSync(file),data=JSON.parse(bytes.toString());
  writeFileSync(file,JSON.stringify({...data,standing:{...data.standing,phase:"dependencies:funded-ownership:unknown"}}));
  try{expect(sample()).toBeUndefined();}finally{writeFileSync(file,bytes);}
  const duplicate=join(folder,process.pid+"-"+(leaseProcessStartedAtMsV1+1)+".json");
  writeFileSync(duplicate,JSON.stringify({...data,startedAtMs:leaseProcessStartedAtMsV1+1}));try{expect(sample()).toBeUndefined();}finally{unlinkSync(duplicate);}
  if(process.platform!=="win32"){
   const kept=file+".keep";
   renameSync(file,kept);symlinkSync(kept,file);
   try{expect(sample()).toBeUndefined();}finally{unlinkSync(file);renameSync(kept,file);}
  }
  expect(sample()?.done).toBe(565);
  const binding="dependencies:sync-bindings",provider="javascript-source-provider";
  await phase.publish({resource:binding+":312/323:canonical:238:changed:74",mode:"exclusive",elapsedMs:0});
  await wait(()=>read()?.standing.phase===binding&&read()?.standing.done===312);
  const bindingFirst=sample();expect(bindingFirst?.done).toBe(312);
  expect(bindingFirst?.proof).toEqual(first?.proof);
  expect(read()?.standing.total).toBe(323);
  await Bun.sleep(5100);
  await phase.publish({resource:binding+":313/323:canonical:239:changed:74",mode:"exclusive",elapsedMs:0});
  await wait(()=>read()?.standing.done===313);
  const bindingNext=sample();expect(bindingNext?.series).toBe(bindingFirst?.series);
  expect(bindingNext?.done).toBe(313);
  await phase.publish({resource:provider+":519",mode:"shared",elapsedMs:0});
  await wait(()=>read()?.standing.phase===provider);
  expect(sample()).toMatchObject({phase:provider,done:519,proof:bindingNext!.proof});

  const phaseNative="native-artifact-input-read";
  const report=async(resource:string,done:number)=>{
   await phase.publish({resource:resource+":"+done,mode:resource===provider?"shared":"exclusive",elapsedMs:0});
   await wait(()=>read()?.standing.phase===resource&&read()?.standing.done===done);
   const observed=sample();
   if(observed===undefined||observed.phase!==resource||observed.done!==done)throw Error("Expected live dual-exclusive progress sample");
   return {observed,now:Date.now()};
  };
  const initialNative=await report(phaseNative,11436);
  let progress=recordToolsProgress(null,initialNative.observed,initialNative.now);
  await Bun.sleep(5100);
  const advanceNative=await report(phaseNative,11438);
  progress=recordToolsProgress(progress,advanceNative.observed,advanceNative.now);
  await Bun.sleep(5100);
  const finalNative=await report(phaseNative,11440);
  progress=recordToolsProgress(progress,finalNative.observed,finalNative.now);
  const thenProvider=await report(provider,519);
  const limit={stage:"tools",started:leaseProcessStartedAtMsV1,now:thenProvider.now,soft:Math.floor((thenProvider.now-leaseProcessStartedAtMsV1)/4.3),quiet:20000,sampleLimit:10250};
  expect(initialNative.observed.proof).toEqual(thenProvider.observed.proof);
  expect(finalNative.observed.proof).toEqual(thenProvider.observed.proof);
  expect(toolsGraceActive(progress,limit)).toBe(true);
  const switched=recordToolsProgress(progress,thenProvider.observed,thenProvider.now);
  expect(switched.counters.find(row=>row.phase===phaseNative)).toEqual(progress.counters.find(row=>row.phase===phaseNative));
  expect(toolsGraceActive(switched,limit)).toBe(true);
  expect(toolsGraceActive(recordToolsProgress(switched,sample({owner:"foreign-stage"}),thenProvider.now),limit)).toBe(false);
  const alias="dependencies:Source-aliases";
  await phase.publish({resource:alias+":9016",mode:"shared",elapsedMs:0});
  await wait(()=>read()?.standing.phase===alias);
  expect(sample()).toMatchObject({phase:alias,done:9016,proof:thenProvider.observed.proof});
  gate.release();await wait(()=>read()?.holds.length===1);
  expect(sample()).toBeUndefined();
  expect(toolsGraceActive(recordToolsProgress(switched,sample(),thenProvider.now),limit)).toBe(false);
 }finally{try{gate.release();}catch{}parent.release();}
 await wait(()=>read()===undefined);
 }finally{
  if(previousRegistry===undefined)delete process.env.SEMIO_LEASE_REGISTRY_DIRECTORY;else process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=previousRegistry;
  if(previousOwner===undefined)delete process.env.SEMIO_LEASE_OWNER;else process.env.SEMIO_LEASE_OWNER=previousOwner;
 }
}),28500);
