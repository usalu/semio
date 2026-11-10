import opaqueRows from "./📥️opaque.json";
import startupRows from "./📥️startup.json";
import opaqueSchema from "./🔣️opaque.json";
import{SOURCE_OPACITY_PROTOCOL_V1}from"../../../../../🔍️discovery/📂️source/🟦️.ts";
import{repositoryFilesystemJournalPolicyV1}from"../../../../🔏️seal/📥️membership/📥️journal/🟦️.ts";
import{expect,test}from"bun:test";
import{mkdir,mkdtemp,rm,symlink}from"node:fs/promises";
import{join}from"node:path";
import{mkdirSync,writeFileSync,realpathSync}from"node:fs";
import{tmpdir}from"node:os";
import{writeChangeStateV1,type ChangeStateV1}from"../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import{changeJournalDirectoryV1}from"../../../../../../🏗️build/🏡️resident/📜️journal/🟦️.ts";
import{RESIDENT_EXCLUDES_V1}from"../../../../../../🏗️build/🏡️resident/⌨️command/🟨️.mjs";
import{createRepositoryWorkspaceWatcherV1,requireJournalCoversSourceV1}from"../🟦️.ts";
import * as workspaceRuntime from "../🟦️.ts";
import schema from"../🔣️schema.json";
import Ajv from"ajv";
import{Database}from"bun:sqlite";

const policy={roots:[".git",".cache",".r",".🧬semio"],names:[".nx","node_modules","target","__pycache__",".venv"]};
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
/** 🧫️ Each bounded law owns its physical workspace and a generic observation control. */
async function fixture(body:(root:string,control:any,abort:AbortController,environment:Record<string,string>)=>Promise<void>){
 const base=await mkdtemp(join(realpathSync(tmpdir()),"w-")),root=join(base,"root"),abort=new AbortController(),control={signal:abort.signal,remainingMilliseconds:()=>30000,publish:async()=>{},yieldContinuation:async()=>{}};
 await mkdir(root);const environment={...process.env,XDG_RUNTIME_DIR:base,SEMIO_REPO_CHANGES:"on"} as Record<string,string>;
 try{await body(root,control,abort,environment);}finally{await rm(base,{recursive:true,force:true});}
}
const cursor={epoch:"0123456789abcdef",seq:1};
const publish=(root:string,environment:Record<string,string>,exclude=RESIDENT_EXCLUDES_V1)=>{
 const directory=changeJournalDirectoryV1(root,environment);mkdirSync(directory,{recursive:true});
 const state:ChangeStateV1={version:1,epoch:cursor.epoch,instance:"i",pid:process.pid,startedAt:0,root,fence:".f",exclude:{rooted:[...exclude.rooted],names:[...exclude.names]},backend:"inotify",trust:"exact",ready:true};writeChangeStateV1(directory,state);
};
const fakeClient=(answers:any[])=>{const calls={since:0};return{calls,client:{excluder:()=>()=>false,cursor:async()=>({...cursor}),since:async()=>{calls.since++;return answers.length?answers.shift():{exact:true,cursor,rows:[]};}} as never};};
test("workspace watch protocol remains closed and independent of Native runtime types",()=>{
 expect(new Ajv({strict:true}).validate(schema.definitions.Protocol,schema.definitions.Protocol.const)).toBe(true);
 expect(schema.definitions.Protocol.const).toMatchObject({maximumPathUnits:256,maximumPendingEvents:200000,failure:"terminal",custody:"host-journal"});
});
test("the host journal exclusions are covered by the real Source policy of this repository, and a narrower policy is refused",async()=>{
 const real=await repositoryFilesystemJournalPolicyV1(new URL("../../../../../../../../../../../../../../",import.meta.url).pathname);
 expect(()=>requireJournalCoversSourceV1(RESIDENT_EXCLUDES_V1,real)).not.toThrow();
 expect(()=>requireJournalCoversSourceV1(RESIDENT_EXCLUDES_V1,{roots:[".nx"],names:[]})).toThrow("never reports");
 expect(()=>requireJournalCoversSourceV1({rooted:[],names:["node_modules"]},{roots:[],names:["node_modules"]})).not.toThrow();
});
test("workspace watcher refuses a symlinked root and a journal that cannot vouch before any delivery",()=>fixture(async(root,control,_abort,environment)=>{
 const link=root+"-link";await symlink(root,link,process.platform==="win32"?"junction":"dir");
 await expect(createRepositoryWorkspaceWatcherV1(link,policy,"native",control,{environment})).rejects.toThrow("No-follow");
 const refused={cursor:async()=>({exact:false,reason:"untrusted",detail:"portable"}),since:async()=>({exact:false,reason:"untrusted"}),excluder:()=>()=>false} as never;
 await expect(createRepositoryWorkspaceWatcherV1(root,policy,"native",control,{environment,journal:refused})).rejects.toThrow("cannot vouch");
}));
test("workspace startup cancellation settles without awaiting the journal",()=>fixture(async(root,control,abort,environment)=>{
 const reason=Object.freeze({boundary:"original-cancel"});let entered!:()=>void;const entering=new Promise<void>(done=>entered=done);
 const journal={cursor:async()=>{entered();await new Promise(()=>{});},since:async()=>({exact:false,reason:"gap"}),excluder:()=>()=>false} as never;
 const pending=createRepositoryWorkspaceWatcherV1(root,policy,"native",control,{environment,journal});
 await entering;abort.abort(reason);await expect(pending).rejects.toBe(reason);
}));
test("workspace enrollment retires its startup clock while observation keeps its service owner",()=>fixture(async(root,control,abort,environment)=>{
 control.remainingMilliseconds=()=>null;publish(root,environment,{rooted:[],names:[]});
 const startup={signal:abort.signal,remainingMilliseconds:()=>1000},{client}=fakeClient([]);
 const watcher=await createRepositoryWorkspaceWatcherV1(root,policy,"native",control,{environment,journal:client,startupControl:startup});
 try{expect(await watcher.forceFlushPending()).toEqual([]);}finally{await watcher.stop();}
 await expect(createRepositoryWorkspaceWatcherV1(root,policy,"native",control,{environment,journal:client,startupControl:{signal:abort.signal,remainingMilliseconds:()=>0}})).rejects.toThrow("Finite");
}));
test("rows become create, update and delete events; excluded and opaque paths never arrive; a flush is a fenced read",()=>fixture(async(root,control,_abort,environment)=>{
 publish(root,environment);writeFileSync(join(root,"kept.ts"),"k");
 const{client,calls}=fakeClient([{exact:true,cursor,rows:[["a",0,"new.ts"],["m",0,"kept.ts"],["r",0,"old.ts"],["n",0,"kept.ts"],["n",0,"moved-away.ts"],["m",0,".nx/x"],["a",1,".p0123abcd"],["m",0,"a/node_modules/q"]]}]);
 const watcher=await createRepositoryWorkspaceWatcherV1(root,policy,"native",control,{environment,journal:client});
 try{const events=await watcher.forceFlushPending();expect(calls.since).toBe(1);expect(events).toEqual([{type:"create",path:"new.ts"},{type:"create",path:"kept.ts"},{type:"delete",path:"old.ts"},{type:"delete",path:"moved-away.ts"}]);expect(await watcher.forceFlushPending()).toEqual([]);}finally{await watcher.stop();}
}));
test("a registered receiver gets events by polling and the flush returns what it has not seen",()=>fixture(async(root,control,_abort,environment)=>{
 publish(root,environment);const{client}=fakeClient([]),received:any[]=[];
 const answers=[{exact:true,cursor,rows:[["m",0,"a.ts"]]}];(client as any).since=async()=>answers.length?answers.shift():{exact:true,cursor,rows:[]};
 const watcher=await createRepositoryWorkspaceWatcherV1(root,policy,"native",control,{environment,journal:client});
 try{watcher.watch((error,events)=>{received.push([error,events]);});await new Promise(done=>setTimeout(done,500));expect(received).toEqual([[null,[{type:"update",path:"a.ts"}]]]);expect(await watcher.forceFlushPending()).toEqual([]);}finally{await watcher.stop();}
}));
test("workspace loss is terminal: the receiver hears it once and every later flush refuses",()=>fixture(async(root,control,_abort,environment)=>{
 publish(root,environment);const{client}=fakeClient([{exact:false,reason:"lost",path:"src",detail:"must-scan"}]),events:any[]=[];
 const watcher=await createRepositoryWorkspaceWatcherV1(root,policy,"native",control,{environment,journal:client});
 try{await expect(watcher.forceFlushPending()).rejects.toThrow("cannot vouch");watcher.watch((error,rows)=>{events.push([error,rows]);});await new Promise(done=>setTimeout(done,50));expect(events).toHaveLength(1);expect(events[0][0].message).toContain("src");await expect(watcher.forceFlushPending()).rejects.toThrow("cannot vouch");}finally{await watcher.stop();}
}));
test("opaque source paths are excluded exactly as the schema-derived SQL oracle says",()=>fixture(async(root,control,_abort,environment)=>{
 const db=new Database(":memory:");
 try{
  expect(new Ajv({strict:true}).validate(opaqueSchema,opaqueRows)).toBe(true);
  const actual=await repositoryFilesystemJournalPolicyV1(root);expect(actual.opaqueBasenames).toEqual(SOURCE_OPACITY_PROTOCOL_V1);
  publish(root,environment,{rooted:[],names:[]});
  const{client}=fakeClient([{exact:true,cursor,rows:opaqueRows.map(row=>["m",0,row.path] as const)}]),watcher=await createRepositoryWorkspaceWatcherV1(root,policy,"native",control,{environment,journal:client});
  try{
   const delivered=new Set((await watcher.forceFlushPending()).map(event=>event.path));
   for(const row of opaqueRows){const parts=row.path.split(/[\\/]/u),oracle=db.query("SELECT EXISTS(SELECT 1 FROM json_each(?1) part,json_each(?2) prefix WHERE substr(part.value,1,length(prefix.value))=prefix.value AND length(part.value)=length(prefix.value)+?3 AND substr(part.value,length(prefix.value)+1) NOT GLOB '*[^0-9a-f]*') excluded").get(JSON.stringify(parts),JSON.stringify(SOURCE_OPACITY_PROTOCOL_V1.prefixes),SOURCE_OPACITY_PROTOCOL_V1.digestHex)as{excluded:number};expect(Boolean(oracle.excluded),row.path).toBe(row.excluded);expect(delivered.has(row.path),row.path).toBe(!row.excluded&&!policy.names.some(name=>row.path.split(/[\\/]/u).includes(name)));}
  }finally{await watcher.stop();}
 }finally{db.close();}
}));
