import{SOURCE_OPACITY_PROTOCOL_V1,isOpaqueSourcePathV1}from"../../../../🔍️discovery/📂️source/🟦️.ts";
import schema from"./🔣️schema.json"with{type:"json"};
import{lstat,mkdir,rmdir}from"node:fs/promises";
import{isAbsolute,join,relative,resolve,sep}from"node:path";
import{randomBytes}from"node:crypto";
import{createWatchSupervisorV1}from"../../../../../🏗️build/🏡️resident/🔭️watch/🟨️.mjs";
import{POLICY_V1}from"../../../../../🏗️build/🏡️resident/📨️protocol/🎚️policy/🟨️.mjs";
import type{FilesystemJournalChangeV1,FilesystemJournalControlV1}from"../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📥️journal/🟦️.ts";

/** 🗺️ Closed Source exclusions remain independent of the exact private physical barrier. */
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
/** 🔭️ Prunes native registration with the closed Source policy and proves each flush through an independently subscribed funded directory. */
export async function createRepositoryWorkspaceWatcherV1(root:string,policy:RepositoryWorkspaceWatchPolicyV1,fence:string,binary:string,control:FilesystemJournalControlV1,ports:Readonly<{createSupervisor?:typeof createWatchSupervisorV1;signal?:AbortSignal;startupControl?:Pick<FilesystemJournalControlV1,"signal"|"remainingMilliseconds">}>={}):Promise<RepositoryWorkspaceWatcherV1>{
 const signal=ports.signal??control.signal,limits=schema.definitions.Protocol.const,workspace=resolve(root),distance=relative(workspace,resolve(fence));
 if(!isAbsolute(fence)||!distance||distance===".."||distance.startsWith(".."+sep)||isAbsolute(distance)||fence.length>limits.maximumPathUnits)throw Error("Funded private workspace barrier required");
 const privatePath=distance.replaceAll("\\","/"),excluded=(path:string)=>isOpaqueSourcePathV1(path)||policy.roots.some(prefix=>path===prefix||path.startsWith(prefix+"/"))||path.split("/").some(part=>policy.names.includes(part));
 if(!excluded(privatePath))throw Error("Private workspace barrier must remain excluded from Source");
 const check=()=>{control.signal.throwIfAborted();signal.throwIfAborted();const remaining=control.remainingMilliseconds();if(remaining!==null&&(!Number.isFinite(remaining)||remaining<=0))throw Error("Original workspace observation deadline exhausted");};
 check();const rootStatus=await lstat(workspace);if(rootStatus.isSymbolicLink()||!rootStatus.isDirectory())throw Error("No-follow workspace observation root required");const created:string[]=[];let directory=workspace;for(const part of distance.split(sep)){directory=join(directory,part);if(directory.length>limits.maximumPathUnits)throw Error("Bounded private barrier ancestry required");try{await mkdir(directory,{mode:0o700});created.push(directory);}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}check();const status=await lstat(directory);if(status.isSymbolicLink()||!status.isDirectory())throw Error("No-follow private barrier ancestry required");}check();
 let stopped=false,terminal:unknown,failed=false,receiver:((error:unknown,events:readonly FilesystemJournalChangeV1[])=>void|Promise<void>)|undefined;
 const pending=new Map<string,FilesystemJournalChangeV1>(),drain=()=>{const rows=[...pending.values()];pending.clear();return rows;},fail=(error:unknown)=>{if(failed||stopped)return;failed=true;terminal=error;if(receiver)void Promise.resolve().then(()=>receiver!(error,[])).catch(()=>undefined);};
 const startup=ports.startupControl??control,checkStartup=()=>{startup.signal.throwIfAborted();const remaining=startup.remainingMilliseconds();if(remaining===null||!Number.isFinite(remaining)||remaining<=0)throw Error("Finite live workspace startup phase required");};
 const supervisor=(ports.createSupervisor??createWatchSupervisorV1)({root:workspace,policy:POLICY_V1,filter:{excluded},rooted:policy.roots,names:policy.names,opaqueBasenames:SOURCE_OPACITY_PROTOCOL_V1,binary,backend:"native",startupControl:startup,privateFence:true,failureMode:"terminal",fenceDirectory:privatePath,instance:randomBytes(16).toString("hex"),
  async onBatch(rows){
   if(stopped)return;check();
   for(const[kind,,path]of rows){
    if(!path||excluded(path))continue;
    let type:FilesystemJournalChangeV1["type"]=kind==="a"?"create":kind==="r"?"delete":"update";
    if(kind==="n"){try{await lstat(join(workspace,path));type="create";}catch(error){if(!["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw error;type="delete";}}
    pending.set(path,{type,path});if(pending.size>limits.maximumPendingEvents)throw Error("Workspace observation event capacity exceeded");
   }
   check();if(receiver&&pending.size)await receiver(null,drain());
  },
  onLost:(scope,reason)=>fail(Error("Workspace native coverage lost: "+reason+" ("+scope+")"))
 });
 let closing:Promise<void>|undefined;
 const stop=():Promise<void>=>closing??=(async()=>{stopped=true;signal.removeEventListener("abort",abort);await supervisor.stop();pending.clear();for(const path of created.toReversed())try{await rmdir(path);}catch(error){if(!["ENOTEMPTY","EEXIST"].includes((error as NodeJS.ErrnoException).code??""))throw error;}})().catch(error=>{closing=undefined;throw error;});
 const abort=()=>{void stop().catch(()=>undefined);};
 signal.addEventListener("abort",abort,{once:true});
 const watcher:RepositoryWorkspaceWatcherV1={
  watch(receive){check();if(receiver)throw Error("One workspace observation receiver required");receiver=receive;if(failed)void Promise.resolve().then(()=>receive(terminal,[])).catch(()=>undefined);else if(pending.size)void Promise.resolve().then(()=>receive(null,drain())).catch(fail);},
  async forceFlushPending(){check();if(failed)throw terminal;const result=await supervisor.fence();await supervisor.drained();check();if(failed)throw terminal;if(!result.ok)throw Error("Workspace physical observation barrier failed: "+result.reason);return drain();},
  stop
 };
 try{checkStartup();await control.publish({resource:"workspace-watch:subscribe",mode:"shared",elapsedMs:0});check();let rejectAbort!:(reason:unknown)=>void;const interrupted=new Promise<never>((_accept,reject)=>{rejectAbort=reject;}),interrupt=()=>rejectAbort(signal.reason);signal.addEventListener("abort",interrupt,{once:true});try{check();await Promise.race([supervisor.start(),interrupted]);}finally{signal.removeEventListener("abort",interrupt);}check();checkStartup();if(failed)throw terminal;await control.publish({resource:"workspace-watch:ready",mode:"shared",elapsedMs:0});check();return watcher;}
 catch(error){try{await stop();}catch(cleanup){throw new AggregateError([error,cleanup],"Workspace observation startup and stop failed");}throw error;}
}
