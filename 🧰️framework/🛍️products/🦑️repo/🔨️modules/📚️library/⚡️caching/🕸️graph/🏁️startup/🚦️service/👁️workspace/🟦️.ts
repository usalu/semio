import{isOpaqueSourcePathV1}from"../../../../🔍️discovery/📂️source/🟦️.ts";
import schema from"./🔣️schema.json"with{type:"json"};
import{lstat}from"node:fs/promises";
import{join,resolve}from"node:path";
import{changeJournalDirectoryV1,connectChangeJournalV1}from"../../../../../🏗️build/🏡️resident/📜️journal/🟦️.ts";
import{renderChangeRefusalV1,readChangeStateV1,type ChangeRowV1}from"../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type{ChangeJournalClientV1}from"../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import type{FilesystemJournalChangeV1,FilesystemJournalControlV1}from"../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📥️journal/🟦️.ts";

/** 🗺️ Closed Source exclusions of the sealing consumer; the host journal reports everything else. */
export type RepositoryWorkspaceWatchPolicyV1=Readonly<{roots:readonly string[];names:readonly string[]}>;
/** ⏱️ Narrows enrollment to the existing readiness route and both live owner clocks. */
export function repositoryWorkspaceWatchStartupMillisecondsV1(control:Pick<FilesystemJournalControlV1,"signal"|"remainingMilliseconds">,parentDeadline:number|null=null,now:()=>number=Date.now):number{
 control.signal.throwIfAborted();const remaining=control.remainingMilliseconds();
 if(remaining!==null&&(!Number.isFinite(remaining)||remaining<=0)||parentDeadline!==null&&!Number.isSafeInteger(parentDeadline))throw Error("Live workspace startup funding required");
 const maximum=Math.floor(Math.min(schema.definitions.Protocol.const.maximumStartupMilliseconds,remaining??Infinity,parentDeadline===null?Infinity:parentDeadline-now()));
 if(!Number.isFinite(maximum)||maximum<=0)throw Error("Original workspace startup deadline exhausted");return maximum;
}
/** 🪟️ The owned workspace observation retains Nx delivery and flush semantics without a vendor type. */
export interface RepositoryWorkspaceWatcherV1{
 watch(receive:(error:unknown,events:readonly FilesystemJournalChangeV1[])=>void|Promise<void>):void;
 forceFlushPending():Promise<readonly FilesystemJournalChangeV1[]>;
 stop():Promise<void>;
}

/** 🛡️ The sealing policy as a consumer of the host journal: every region the journal never reports must be a region the Source policy excludes too (or a declared blind path), otherwise a change could go unseen and sealing refuses. */
export function requireJournalCoversSourceV1(exclude:Readonly<{rooted:readonly string[];names:readonly string[]}>,policy:RepositoryWorkspaceWatchPolicyV1):void{
 const excluded=(path:string)=>isOpaqueSourcePathV1(path)||policy.roots.some(prefix=>path===prefix||path.startsWith(prefix+"/"))||path.split("/").some(part=>policy.names.includes(part));
 const blind=schema.definitions.Protocol.const.blindRootedPaths as readonly string[];
 const hidden=[...exclude.rooted.filter(path=>!excluded(path)&&!blind.includes(path)),...exclude.names.filter(name=>!excluded(name))];
 if(hidden.length)throw Error("The host journal never reports regions the Source policy observes: "+hidden.slice(0,5).join(", "));
}

/** 🔭️ Observes the workspace through the one host change journal (started on demand, shared with every other consumer): delivery polls the log, a flush is a fenced read, and a journal that cannot vouch is a terminal failure of the observation. */
export async function createRepositoryWorkspaceWatcherV1(root:string,policy:RepositoryWorkspaceWatchPolicyV1,binary:string,control:FilesystemJournalControlV1,ports:Readonly<{journal?:ChangeJournalClientV1;environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;startupControl?:Pick<FilesystemJournalControlV1,"signal"|"remainingMilliseconds">}>={}):Promise<RepositoryWorkspaceWatcherV1>{
 const signal=ports.signal??control.signal,limits=schema.definitions.Protocol.const,workspace=resolve(root),environment={...(ports.environment??process.env),SEMIO_REPO_WATCH_BINARY:binary};
 const excluded=(path:string)=>isOpaqueSourcePathV1(path)||policy.roots.some(prefix=>path===prefix||path.startsWith(prefix+"/"))||path.split("/").some(part=>policy.names.includes(part));
 const check=()=>{control.signal.throwIfAborted();signal.throwIfAborted();const remaining=control.remainingMilliseconds();if(remaining!==null&&(!Number.isFinite(remaining)||remaining<=0))throw Error("Original workspace observation deadline exhausted");};
 check();const rootStatus=await lstat(workspace);if(rootStatus.isSymbolicLink()||!rootStatus.isDirectory())throw Error("No-follow workspace observation root required");
 const startup=ports.startupControl??control;startup.signal.throwIfAborted();{const remaining=startup.remainingMilliseconds();if(remaining===null||!Number.isFinite(remaining)||remaining<=0)throw Error("Finite live workspace startup phase required");}
 const client=ports.journal??connectChangeJournalV1(workspace,environment);
 const refuse=(answer:{reason:string}&Record<string,unknown>)=>Error("Workspace journal cannot vouch: "+renderChangeRefusalV1(answer as never,"en"));
 await control.publish({resource:"workspace-watch:subscribe",mode:"shared",elapsedMs:0});check();
 let interrupted!:(reason:unknown)=>void;const aborted=new Promise<never>((_accept,reject)=>{interrupted=reject;}),onAbort=()=>interrupted(signal.reason);signal.addEventListener("abort",onAbort,{once:true});
 let cursor;
 try{cursor=await Promise.race([client.cursor(startup.signal),aborted]);}finally{signal.removeEventListener("abort",onAbort);}
 check();if(cursor.exact===false)throw refuse(cursor);
 const state=readChangeStateV1(changeJournalDirectoryV1(workspace,environment));
 if(!state)throw Error("Workspace journal state required");
 requireJournalCoversSourceV1(state.exclude,policy);
 let stopped=false,terminal:unknown,failed=false,receiver:((error:unknown,events:readonly FilesystemJournalChangeV1[])=>void|Promise<void>)|undefined,pulling:Promise<void>|undefined,timer:ReturnType<typeof setInterval>|undefined;
 const pending=new Map<string,FilesystemJournalChangeV1>(),drain=()=>{const rows=[...pending.values()];pending.clear();return rows;};
 const fail=(error:unknown)=>{if(failed||stopped)return;failed=true;terminal=error;if(timer)clearInterval(timer);if(receiver)void Promise.resolve().then(()=>receiver!(error,[])).catch(()=>undefined);};
 const typeOf=async(row:ChangeRowV1):Promise<FilesystemJournalChangeV1["type"]>=>{
  if(row[0]==="a")return"create";if(row[0]==="r")return"delete";if(row[0]==="m")return"update";
  try{await lstat(join(workspace,row[2]));return"create";}catch(error){if(!["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw error;return"delete";}
 };
 const pull=():Promise<void>=>pulling??=(async()=>{
  try{
   if(stopped||failed)return;check();
   const answer=await client.since(cursor as never,[""],signal);
   if(!answer.exact)throw refuse(answer);
   cursor=answer.cursor;
   for(const row of answer.rows){
    const path=row[2];if(!path||excluded(path))continue;
    pending.set(path,{type:await typeOf(row),path});if(pending.size>limits.maximumPendingEvents)throw Error("Workspace observation event capacity exceeded");
   }
   check();if(receiver&&pending.size)await receiver(null,drain());
  }catch(error){fail(error);}finally{pulling=undefined;}
 })();
 const stop=async()=>{stopped=true;if(timer)clearInterval(timer);signal.removeEventListener("abort",abort);await pulling;pending.clear();};
 const abort=()=>{void stop().catch(()=>undefined);};
 signal.addEventListener("abort",abort,{once:true});
 return{
  watch(receive){check();if(receiver)throw Error("One workspace observation receiver required");receiver=receive;if(failed)void Promise.resolve().then(()=>receive(terminal,[])).catch(()=>undefined);else{if(pending.size)void Promise.resolve().then(()=>receive(null,drain())).catch(fail);timer=setInterval(()=>{void pull();},limits.pollMilliseconds);timer.unref?.();}},
  async forceFlushPending(){check();if(failed)throw terminal;await pulling;await pull();check();if(failed)throw terminal;return drain();},
  stop,
 };
}
