import opaqueRows from "./📥️opaque.json";
import startupRows from "./📥️startup.json";
import opaqueSchema from "./🔣️opaque.json";
import{SOURCE_OPACITY_PROTOCOL_V1}from"../../../../../🔍️discovery/📂️source/🟦️.ts";
import{repositoryFilesystemJournalPolicyV1}from"../../../../🔏️seal/📥️membership/📥️journal/🟦️.ts";
import{expect,test}from"bun:test";
import{mkdir,mkdtemp,rm,symlink}from"node:fs/promises";
import{join}from"node:path";
import{createRepositoryWorkspaceWatcherV1}from"../🟦️.ts";
import * as workspaceRuntime from "../🟦️.ts";
import schema from"../🔣️schema.json";
import Ajv from"ajv";
import{Database}from"bun:sqlite";

const policy={roots:[],names:[".nx"]};
test("workspace startup funding is bounded by route, actual control and starting parent",()=>{
 const budget=(workspaceRuntime as Record<string,unknown>).repositoryWorkspaceWatchStartupMillisecondsV1 as (control:any,parent:number|null,now:()=>number)=>number;
 expect(typeof budget).toBe("function");const db=new Database(":memory:");
 try{for(const row of startupRows){
  const expected=db.query("select case when ?1=0 and (?2 is null or ?2>0) and (?3 is null or ?3>0) then min(?4,coalesce(?2,?4),coalesce(?3,?4)) end budget").get(Number(row.cancelled),row.remaining,row.parentRemaining,schema.definitions.Protocol.const.maximumStartupMilliseconds) as {budget:number|null};
  expect(expected.budget).toBe(row.budget);const abort=new AbortController();if(row.cancelled)abort.abort(Error("cancelled"));
  const invoke=()=>budget({signal:abort.signal,remainingMilliseconds:()=>row.remaining},row.parentRemaining===null?null:1000+row.parentRemaining,()=>1000);
  if(row.budget===null)expect(invoke).toThrow();else expect(invoke()).toBe(row.budget);
 }}finally{db.close();}
});
/** 🧫️ Each bounded law owns its physical artifact tree and generic observation control. */
async function fixture(body:(root:string,control:any,abort:AbortController)=>Promise<void>){
 const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifact)throw Error("Caller-owned workspace watcher artifacts required");await mkdir(artifact,{recursive:true});const root=await mkdtemp(join(artifact,"w-")),abort=new AbortController(),control={signal:abort.signal,remainingMilliseconds:()=>30000,publish:async()=>{},yieldContinuation:async()=>{}};
 try{await body(root,control,abort);}finally{await rm(root,{recursive:true,force:true});}
}
test("workspace watch protocol remains closed and independent of Native runtime types",()=>{
 const db=new Database(":memory:");try{expect(new Ajv({strict:true}).validate(schema.definitions.Protocol,schema.definitions.Protocol.const)).toBe(true);const row=db.query("SELECT ?1=256 AND ?2=200000 AND ?3='terminal' admitted").get(schema.definitions.Protocol.const.maximumPathUnits,schema.definitions.Protocol.const.maximumPendingEvents,schema.definitions.Protocol.const.failure)as{admitted:number};expect(row.admitted).toBe(1);}finally{db.close();}
});
test("workspace watcher rejects a Source-visible or foreign physical barrier before launch",()=>fixture(async(root,control)=>{
 let launches=0;const ports={createSupervisor:()=>{launches++;throw Error("Unexpected launch");}}as never;
 await expect(createRepositoryWorkspaceWatcherV1(root,policy,join(root,"source"),"native",control,ports)).rejects.toThrow("excluded");
 await expect(createRepositoryWorkspaceWatcherV1(root,policy,join(root,"..","foreign"),"native",control,ports)).rejects.toThrow("Funded");
 expect(launches).toBe(0);
}));
test("workspace watcher refuses a symlink in private barrier ancestry",()=>fixture(async(root,control)=>{
 await mkdir(join(root,"ordinary"));await symlink(join(root,"ordinary"),join(root,".nx"),process.platform==="win32"?"junction":"dir");let launches=0;
 await expect(createRepositoryWorkspaceWatcherV1(root,policy,join(root,".nx","f"),"native",control,{createSupervisor:()=>{launches++;throw Error("Unexpected launch");}}as never)).rejects.toThrow("No-follow");expect(launches).toBe(0);
}));
test("workspace startup preserves primary and independent settlement failures",()=>fixture(async(root,control)=>{
 const primary=Object.freeze({boundary:"native-start"}),secondary=Object.freeze({boundary:"native-stop"});let stops=0;
 try{await createRepositoryWorkspaceWatcherV1(root,policy,join(root,".nx","f"),"native",control,{createSupervisor:()=>({start:async()=>{throw primary;},stop:async()=>{stops++;throw secondary;}})}as never);throw Error("Expected startup refusal");}
 catch(error){expect(error).toBeInstanceOf(AggregateError);expect((error as AggregateError).errors).toEqual([primary,secondary]);}expect(stops).toBe(1);
}));
test("workspace startup cancellation settles the same process without awaiting native readiness",()=>fixture(async(root,control,abort)=>{
 let started!:()=>void;const entering=new Promise<void>(done=>started=done),reason=Object.freeze({boundary:"original-cancel"});let stops=0;
 const pending=createRepositoryWorkspaceWatcherV1(root,policy,join(root,".nx","f"),"native",control,{createSupervisor:()=>({start:async()=>{started();await new Promise(()=>{});},stop:async()=>{stops++;}})}as never);
 await entering;abort.abort(reason);await expect(pending).rejects.toBe(reason);expect(stops).toBe(1);
}));
test("workspace enrollment retires its startup clock while observation keeps its service owner",()=>fixture(async(root,control,abort)=>{
 control.remainingMilliseconds=()=>null;let remaining=1000,options:any;const startup={signal:abort.signal,remainingMilliseconds:()=>remaining},events:any[]=[];
 const watcher=await createRepositoryWorkspaceWatcherV1(root,policy,join(root,".nx","f"),"native",control,{startupControl:startup,createSupervisor:(value:any)=>{options=value;return{start:async()=>({trust:"exact"}),stop:async()=>{},fence:async()=>({ok:true}),drained:async()=>{}};}}as never);
 try{expect(options.startupControl).toBe(startup);remaining=0;watcher.watch((error,rows)=>{events.push([error,rows]);});await options.onBatch([["a",0,"live.ts"]],0);expect(events).toEqual([[null,[{type:"create",path:"live.ts"}]]]);expect(await watcher.forceFlushPending()).toEqual([]);}finally{await watcher.stop();}
}));
test("workspace loss signals shutdown without joining its own Native delivery",()=>fixture(async(root,control)=>{
 let options:any,inside=false,stopped=0;const events:any[]=[];
 const watcher=await createRepositoryWorkspaceWatcherV1(root,policy,join(root,".nx","f"),"native",control,{createSupervisor:(value:any)=>{options=value;return{start:async()=>({trust:"exact"}),stop:async()=>{expect(inside).toBe(false);stopped++;},fence:async()=>({ok:true}),drained:async()=>{}};}}as never);
 let completion:Promise<void>|undefined;watcher.watch((error,rows)=>{events.push([error,rows]);if(error)completion=watcher.stop();});
 inside=true;options.onLost("source","registration-error");inside=false;await Promise.resolve();await completion;
 expect(events).toHaveLength(1);expect(events[0][0].message).toContain("registration-error");expect(stopped).toBe(1);await expect(watcher.forceFlushPending()).rejects.toThrow("coverage");
}));

test("reserved capsule policy and adapter agree with schema-derived SQL eligibility",()=>fixture(async(root,control)=>{
 const rows=opaqueRows,db=new Database(":memory:");let options:any;
 try{
  expect(new Ajv({strict:true}).validate(opaqueSchema,rows)).toBe(true);
  const actual=await repositoryFilesystemJournalPolicyV1(root);expect(actual.opaqueBasenames).toEqual(SOURCE_OPACITY_PROTOCOL_V1);
  const watcher=await createRepositoryWorkspaceWatcherV1(root,policy,join(root,".nx","f"),"native",control,{createSupervisor:(value:any)=>{options=value;return{start:async()=>({trust:"exact"}),stop:async()=>{},fence:async()=>({ok:true}),drained:async()=>{}};}}as never);
  try{expect(options.opaqueBasenames).toEqual(SOURCE_OPACITY_PROTOCOL_V1);for(const row of rows){const parts=row.path.split(/[\\/]/u),oracle=db.query("SELECT EXISTS(SELECT 1 FROM json_each(?1) part,json_each(?2) prefix WHERE substr(part.value,1,length(prefix.value))=prefix.value AND length(part.value)=length(prefix.value)+?3 AND substr(part.value,length(prefix.value)+1) NOT GLOB '*[^0-9a-f]*') excluded").get(JSON.stringify(parts),JSON.stringify(SOURCE_OPACITY_PROTOCOL_V1.prefixes),SOURCE_OPACITY_PROTOCOL_V1.digestHex)as{excluded:number};expect(Boolean(oracle.excluded),row.path).toBe(row.excluded);expect(options.filter.excluded(row.path),row.path).toBe(row.excluded)}}finally{await watcher.stop()}
 }finally{db.close()}
}));
