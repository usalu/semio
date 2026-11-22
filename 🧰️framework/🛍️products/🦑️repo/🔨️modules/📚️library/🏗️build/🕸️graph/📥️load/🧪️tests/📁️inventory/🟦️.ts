import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {buildSliceReportV1,describeSliceReadStepV1,sliceReadMovedAtV1,sliceReadStepV1} from "../../../../🧠️memory/📋️slice-report/🟨️.mjs";
import schema from "../../../../🧠️memory/📋️slice-report/🧬️schema/🔣️.json" with {type:"json"};
import cases from "./🔣️.json" with {type:"json"};
import owners from "./🧭️owners.json" with {type:"json"};
import graphSchema from "../../../🧬️schema/🔣️.json" with {type:"json"};
import {instantiateOwnerClosureV1,prepareOwnerClosureV1} from "../../../../🧪️tests/🕸️graph/🧰️support/🔗️owner-closure/🟦️.ts";

type Phase={phase:string;elapsedMilliseconds:number;observations?:number;completedOwners?:number;verifiedMemoRows?:number;files?:number;published?:boolean};
type Report={active:Phase;completed:Phase[];activeOwner?:{owner:string;stage:string;elapsedMilliseconds:number}};
const verify=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/InventoryAwait"});
const LOADER_URL=new URL("../../🟦️.ts",import.meta.url);
/** 🔬️ Executes the actual inventory closure and its timing owner with independent controlled await ports; every name the owners read that the ports do not supply is bound from the loader module itself. */
async function inventoryFixture(ports:Record<string,unknown>){
 ports={sliceReportSchema:schema,createHash,GRAPH_LIMITS_V1:graphSchema.definitions.Limits.const,checkpointForScope:async()=>undefined,...ports};
 const provided=Object.keys(ports),inventory=await prepareOwnerClosureV1({moduleUrl:LOADER_URL,owner:"inventory",within:"loadBuildGraphAttemptV1",provided}),report=await prepareOwnerClosureV1({moduleUrl:LOADER_URL,owner:"createGraphInventoryAwaitReportV1",provided});
 return {inventory:instantiateOwnerClosureV1(inventory,ports),report:instantiateOwnerClosureV1(report,ports)};
}

for(const vector of cases)test("inventory receipt "+vector.mode,async()=>{
 const db=new Database(":memory:"),signals=new AbortController(),cause=Object.freeze({cancel:"memo publication"}),whole=vector.mode==="whole",admission={files:["kept.ts"]},calls:string[]=[],samples:Report[]=[];
 let now=0;
 const sample=(phase:string)=>{
  const prior=sliceReadMovedAtV1();now+=13;
  const report=(buildSliceReportV1({reason:"interval"}).sources as {inventory?:Report}).inventory;
  expect(verify(report)).toBe(true);expect(report?.active.phase).toBe(phase);expect(report?.active.elapsedMilliseconds).toBe(13);
  expect(describeSliceReadStepV1()?.id).toBe("inventory");expect(sliceReadMovedAtV1()).toBe(prior);
  calls.push(phase);samples.push(report!);
 };
 const owner={name:"fixture owner"},invocation={control:{signal:signals.signal}},ports={
  selected:!whole,scope:whole?undefined:"s",options:{selection:{}},root:"fixture",memoPath:"private",registry:{},invocation,phaseControl:{},loadedGraphAdmissionFormatV1:"v",ownerRegistryInvalidationV1:{},readBuildOwnerRegistryV1:(v:unknown)=>v,buildGraphScopeKeyV1:()=>"s",
  producer:{readBuildOwnerRegistrySourceV1:()=>"{}",discoverBuildOwnerAdmissionFactsV1:async()=>({})},
  readAdmissionMemoV1:async()=>{sample("memo-read");return{admission,observations:[1,2]};},
  selectedAdmissionFactsV1:{entries:()=>[],seed:()=>undefined},ownersOfAdmissionMemoV1:()=>[],
  observation:{captureObservedSourceInputsV1:async(read:()=>Promise<unknown>)=>({value:await read(),observations:[3]}),settleObservedComputationV1:async(run:()=>Promise<unknown>)=>{sample("scoped-discovery");return run();}},
  closeBuildOwnerAdmissionV1:async(_selection:unknown,_registry:unknown,observe:(owner:unknown,targets:unknown,inputs:unknown)=>Promise<unknown>)=>{
   await observe(owner,[],[]);
   if(vector.mode==="write-completed"){
    expect(describeSliceReadStepV1()?.phase).toContain("1 completed owner checks");
    await observe(owner,["another revision"],[]);
    expect(describeSliceReadStepV1()?.phase).toContain("2 completed owner checks");
   }
   return admission;
  },
  observeOwnerFactsMemoizedV1:async(input:{observe:()=>Promise<unknown>;outcome?:(kind:string)=>void})=>{if(vector.mode==="hit"){input.outcome?.("memo");return {};}input.outcome?.("discovery");return input.observe();},isOwnerRegistryInvalidationV1:()=>false,checkScriptInvocation:()=>signals.signal.throwIfAborted(),
  writeAdmissionMemoV1:async()=>{sample("memo-publish");if(vector.mode==="memo-over-size")return false;if(vector.mode==="write-completed")return true;if(vector.mode==="cancel")signals.abort(cause);throw Error("write refused");},
  packAdmissionMemoV1:(v:unknown)=>v,graphSnapshotPolicyV1:{maximumSnapshotBytes:4096},timing:()=>undefined,
  workspaceBuildFilesV1:async()=>{sample("catalog");return admission.files;},buildGraphMembersV1:(files:string[])=>files,inventoryDirectoriesV1:new WeakMap(),
  buildGraphMembershipV1:async()=>{sample("membership-hash");return "digest";},buildGraphSourceFactsHoldV1:async()=>false,
 };
 try{
  const {inventory,report}=await inventoryFixture(ports),activity=report(()=>now),result=sliceReadStepV1("inventory",()=>inventory(activity),{},activity.read);
  if(vector.settlement==="cancelled")await expect(result).rejects.toBe(cause);else expect(await result).toMatchObject({files:admission.files,digest:"digest"});
  expect(db.query("select value from json_each(?1) order by key").all(JSON.stringify(calls))).toEqual(vector.calls.map(value=>({value})));
  expect((buildSliceReportV1({reason:"end"}).sources as {inventory?:Report}).inventory).toBeUndefined();
  if(["write-refused","memo-over-size","write-completed"].includes(vector.mode)){const completed=samples.at(-1)!.completed;expect(completed.find(row=>row.phase==="scoped-discovery")).toMatchObject({elapsedMilliseconds:13,completedOwners:vector.mode==="write-completed"?2:1});const publish=completed.find(row=>row.phase==="memo-publish");expect(publish?.observations).toBe(1);expect(publish?.published).toBe(vector.mode==="write-refused"?undefined:vector.mode==="write-completed");}
  if(vector.mode==="hit")expect(samples.at(-1)!.completed.find(row=>row.phase==="memo-publish")).toBeUndefined();
 }finally{db.close();}
});

for(const vector of owners)test("active admission lookup "+vector.mode,async()=>{
 const db=new Database(":memory:"),signal=new AbortController(),cause=Object.freeze({cancel:"active admission lookup"});
 const owner="@fixture/"+vector.ownerGlyph.repeat(vector.ownerRepeat),facts={files:["kept.ts"]};
 const scope="s",received:NonNullable<Report["activeOwner"]>[]=[];
 const ownerName=db.query<{name:string},[string,number]>("select substr(?1,1,?2) as name").get(owner,vector.ownerBudget)!.name;
 let now=0,memoAttempt=0;const verified:number[]=[];
 const sample=(stage:string,began:number,at:number)=>{
  now=at;
  const moved=sliceReadMovedAtV1(),description=describeSliceReadStepV1();
  const report=(buildSliceReportV1({reason:"interval"}).sources as {inventory?:Report}).inventory;
  expect(verify(report)).toBe(true);expect(report?.active.phase).toBe("scoped-discovery");expect(report?.active.completedOwners).toBe(vector.mode==="memo-hit"?memoAttempt:0);
  const elapsed=db.query<{ms:number},[number,number]>("select ?2 - ?1 as ms").get(began,at)!.ms;
  expect(report?.activeOwner).toEqual({owner:ownerName,stage,elapsedMilliseconds:elapsed});
  expect(describeSliceReadStepV1()).toEqual(description);expect(sliceReadMovedAtV1()).toBe(moved);
  received.push(report!.activeOwner!);
 };
 const read=()=>(buildSliceReportV1({reason:"interval"}).sources as {inventory?:Report}).inventory;
 const ports={
  selected:true,scope,options:{selection:{}},root:"fixture",memoPath:"private",registry:{},invocation:{control:{signal:signal.signal}},phaseControl:{},loadedGraphAdmissionFormatV1:"v",ownerRegistryInvalidationV1:{},readBuildOwnerRegistryV1:(v:unknown)=>v,buildGraphScopeKeyV1:()=>scope,
  producer:{readBuildOwnerRegistrySourceV1:()=>"{}",discoverBuildOwnerAdmissionFactsV1:async(_owner:unknown,_targets:unknown,_registry:unknown,context:{onOwnerDiscoveryPhase:(operation:string)=>void})=>{sample("discovery",vector.discoveryAt,vector.discoverySampleAt);
   const moved=sliceReadMovedAtV1(),prior=read()?.active;
   for(const operation of vector.operations??[]){
    context.onOwnerDiscoveryPhase(operation);
    expect(verify(read())).toBe(true);
    expect(read()?.activeOwner).toMatchObject({stage:"discovery",operation,elapsedMilliseconds:vector.discoverySampleAt-vector.discoveryAt});
    expect(read()?.active).toEqual(prior);expect(sliceReadMovedAtV1()).toBe(moved);
   }
   if(vector.mode==="cancel"){signal.abort(cause);signal.signal.throwIfAborted();}return facts;}},
  readAdmissionMemoV1:async()=>undefined,selectedAdmissionFactsV1:{entries:()=>[]},
  buildGraphSourceFactsHoldV1:async(_rows:unknown,_invocation:unknown,_onStale:unknown,onVerified?:(rows:number)=>void)=>{
   sample("memo-check",memoAttempt===0?0:vector.discoveryAt,memoAttempt===0?vector.memoSampleAt:vector.discoverySampleAt);
   for(const count of vector.verified?.[memoAttempt]??[]){
    onVerified?.(count);
    expect(describeSliceReadStepV1()?.phase).toContain((read()?.active.verifiedMemoRows??0)+" verified memo rows");
    verified.push(read()?.active.verifiedMemoRows??0);
   }
   memoAttempt++;await Promise.resolve();now=vector.discoveryAt;return vector.mode==="memo-hit";
  },
  observation:{captureObservedSourceInputsV1:async(read:()=>Promise<unknown>)=>({value:await read(),observations:[3]}),settleObservedComputationV1:(run:()=>Promise<unknown>)=>run(),retainObservedRowsV1:()=>undefined},
  closeBuildOwnerAdmissionV1:async(_selection:unknown,_registry:unknown,observe:(owner:unknown,targets:unknown,inputs:unknown)=>Promise<unknown>)=>{const value=await observe({name:owner},[],[]);expect(read()?.activeOwner).toBeUndefined();expect(read()?.active.completedOwners).toBe(1);if(vector.verified)await observe({name:owner},[],[]);return value;},
  ownerFactsMemoV1:new Map([[scope+"\0v\0"+owner+"\0[[],[]]",{facts,rows:[]}]]),rememberOwnerFactsV1:()=>undefined,
  isOwnerRegistryInvalidationV1:()=>false,checkScriptInvocation:()=>signal.signal.throwIfAborted(),
  writeAdmissionMemoV1:async()=>{expect(read()?.activeOwner).toBeUndefined();return true;},packAdmissionMemoV1:(v:unknown)=>v,graphSnapshotPolicyV1:{maximumSnapshotBytes:4096},timing:()=>undefined,
  inventoryDirectoriesV1:new WeakMap(),buildGraphMembershipV1:async()=> "digest",
 };
 try{
  const {inventory,report}=await inventoryFixture(ports),activity=report(()=>now),result=sliceReadStepV1("inventory",()=>inventory(activity),{},activity.read);
  if(vector.mode==="cancel")await expect(result).rejects.toBe(cause);else expect(await result).toMatchObject({files:facts.files,digest:"digest"});
  expect(activity.read().activeOwner).toBeUndefined();expect(read()).toBeUndefined();
  expect(received.map(row=>row.stage)).toEqual(vector.stages);
  const independent=db.query<{rows:number},[string]>("select max(value) over (order by key) as rows from json_each(?1) order by key").all(JSON.stringify(vector.verified?.flat()??[]));
  expect(verified).toEqual(independent.map(row=>row.rows));expect(verified).toEqual(vector.highWater??[]);
 }finally{db.close();}
});

test("settled inventory frames cannot overwrite another running frame",async()=>{
 const active=()=>(buildSliceReportV1({reason:"interval"}).sources as {inventory?:Report}).inventory?.active.phase;
 const diagnostics=(phase:string)=>()=>({version:1,active:{phase,elapsedMilliseconds:0},completed:[]});
 await sliceReadStepV1("inventory",async()=>{
  const end=Promise.withResolvers<void>(),inner=sliceReadStepV1("inventory",()=>end.promise,{},diagnostics("memo-read"));
  try{expect(active()).toBe("memo-read");}finally{end.resolve();await inner;}
  expect(active()).toBe("catalog");
 },{},diagnostics("catalog"));
 expect(active()).toBeUndefined();
});
