import {execFile,spawn} from "node:child_process";
import {randomBytes} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,readdirSync,realpathSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {readFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {canonicalRootV1,stateDirectoryV1} from "../📨️protocol/📍️endpoint/🟨️.mjs";
import {createChangeJournalClientV1,type ChangeJournalClientV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {CHANGE_FILE_SYSTEM_POLICY_V1,inspectChangeFileSystemV1,type ChangeFileSystemPortsV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🗻️mount/🟦️.ts";
import {
 CHANGE_JOURNAL_FILES_V1,CHANGE_JOURNAL_POLICY_V1,changeHeartbeatAgeV1,createChangeLogWriterV1,readChangeStateV1,touchChangeFileV1,writeChangeStateV1,
 type ChangeJournalPolicyV1,type ChangeRowV1,type ChangeStateV1,
} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {formatJournalSweepV1,sweepAbandonedJournalStatesV1} from "./🧹️abandoned/🟦️.ts";
import {refuseBuildV1} from "../../📖️texts/🟦️.ts";

/** 🔭️ The slice of the native watch supervisor this service needs: it is injected, so the service owns no watcher of its own. */
export interface ChangeWatchSupervisorV1{start():Promise<{backend:string;trust:string;reason?:string}>;stop():Promise<void>}
/** 🏭️ Builds the supervisor from the options the existing `createWatchSupervisorV1` takes. */
export type ChangeWatchFactoryV1=(options:Readonly<{
 root:string;policy:Record<string,unknown>;filter:{excluded(path:string):boolean};rooted:readonly string[];names:readonly string[];binary?:string;backend:"auto"|"native"|"portable";latencyMs:number;fenceDirectory:string;instance:string;
 onBatch:(rows:readonly(readonly[string,number,string])[],at:number)=>Promise<void>;onLost:(scope:string,reason:string)=>void;onTrust:(state:{backend:string;trust:string;reason:string})=>void;log:(kind:string,detail:string)=>void;
}>)=>ChangeWatchSupervisorV1;

/** 🎛️ Everything one service run needs. */
export type ChangeJournalServiceOptionsV1=Readonly<{
 root:string;directory:string;fence:string;exclude:Readonly<{rooted:readonly string[];names:readonly string[]}>;binary?:string;watcherPolicy:Record<string,unknown>;createSupervisor:ChangeWatchFactoryV1;
 policy?:ChangeJournalPolicyV1;now?:()=>number;log?:(kind:string,detail:string)=>void;residentBytes?:()=>number;
}>;
/** 🛑️ Another live service already owns the journal directory. */
export class ChangeJournalRunningErrorV1 extends Error{constructor(directory:string){super("A change journal service already runs for "+directory);this.name="ChangeJournalRunningErrorV1";}}

/** 🪪️ The identity of the directory behind a path (device, inode, birth time), or nothing where there is none: watches follow the directory they were set on, not the path. */
const directoryIdentityV1=(path:string):string|undefined=>{try{const facts=statSync(path,{bigint:true});return facts.dev+":"+facts.ino+":"+facts.birthtimeNs;}catch{return undefined;}};

const excludedBy=(exclude:ChangeJournalServiceOptionsV1["exclude"])=>(path:string):boolean=>{
 if(!path)return false;
 for(const prefix of exclude.rooted)if(path===prefix||path.startsWith(prefix+"/"))return true;
 return exclude.names.length>0&&path.split("/").some(part=>exclude.names.includes(part));
};

/** 🔒️ Whether the lock of a journal directory still belongs to a starting or running service. A recorded process id is only a process id when it is a positive safe integer (`kill(0)` signals the caller's own process group and so always "finds" a process on POSIX, while on Windows it finds none); an owner that is not readable yet (the lock directory exists, `owner.json` is not written) holds the lock while the directory itself is younger than the start timeout. https://pubs.opengroup.org/onlinepubs/9799919799/functions/kill.html */
export function changeLockHeldV1(facts:Readonly<{pid:unknown;alive:boolean;heartbeatAgeMilliseconds:number;ageMilliseconds:number}>,policy:Pick<ChangeJournalPolicyV1,"heartbeatStaleMilliseconds"|"startTimeoutMilliseconds">):boolean{
 if(!(Number.isSafeInteger(facts.pid)&&(facts.pid as number)>0))return facts.ageMilliseconds<policy.startTimeoutMilliseconds;
 return facts.alive&&(facts.heartbeatAgeMilliseconds<policy.heartbeatStaleMilliseconds||facts.ageMilliseconds<policy.startTimeoutMilliseconds);
}

export function takeChangeLockV1(directory:string,policy:ChangeJournalPolicyV1,now:number):void{
 const lock=join(directory,CHANGE_JOURNAL_FILES_V1.lock);
 for(let attempt=0;attempt<2;attempt++){
  try{mkdirSync(lock,{mode:0o700});writeFileSync(join(lock,"owner.json"),JSON.stringify({pid:process.pid,at:now}),{mode:0o600});return;}
  catch(error){
   if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   let owner:{pid?:unknown;at?:unknown}={};
   try{owner=JSON.parse(readFileSync(join(lock,"owner.json"),"utf8"));}catch{}
   const pid=owner.pid,alive=Number.isSafeInteger(pid)&&(pid as number)>0&&(()=>{try{process.kill(pid as number,0);return true;}catch(failure){return(failure as NodeJS.ErrnoException).code==="EPERM";}})();
   const at=typeof owner.at==="number"?owner.at:(()=>{try{return statSync(lock).mtimeMs;}catch{return 0;}})();
   if(changeLockHeldV1({pid,alive,heartbeatAgeMilliseconds:changeHeartbeatAgeV1(directory,now),ageMilliseconds:now-at},policy))throw new ChangeJournalRunningErrorV1(directory);
   rmSync(lock,{recursive:true,force:true});
  }
 }
 throw new ChangeJournalRunningErrorV1(directory);
}

/** 📜️ Runs the change journal service of one workspace: one new epoch, one writer, one watcher supervisor, a heartbeat, idle shutdown. It keeps no membership and no task state, only the log, so its memory is the runtime plus the supervisor's queue. */
export async function runChangeJournalServiceV1(options:ChangeJournalServiceOptionsV1):Promise<Readonly<{epoch:string;stop(reason:string):Promise<void>;closed:Promise<string>}>>{
 const {root,directory,fence,exclude,binary,watcherPolicy,createSupervisor,policy=CHANGE_JOURNAL_POLICY_V1,now=Date.now,log=()=>{},residentBytes=()=>process.memoryUsage().rss}=options;
 mkdirSync(directory,{recursive:true,mode:0o700});
 takeChangeLockV1(directory,policy,now());
 try{const swept=sweepAbandonedJournalStatesV1(directory,now());if(swept.removed.length)log("sweep",formatJournalSweepV1(swept,"en")+" / "+formatJournalSweepV1(swept,"de"));}catch{}
 const epoch=randomBytes(8).toString("hex"),instance=randomBytes(4).toString("hex"),startedAt=now(),excluded=excludedBy(exclude),rootIdentity=directoryIdentityV1(root);
 rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.unavailable),{force:true});rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.heartbeat),{force:true});rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.state),{force:true});
 const writer=createChangeLogWriterV1({directory,epoch,policy});
 let current:ChangeStateV1={version:1,epoch,instance,pid:process.pid,startedAt,root,fence,exclude:{rooted:[...exclude.rooted],names:[...exclude.names]},backend:"none",trust:"none",ready:false};
 const publish=(change:Partial<ChangeStateV1>):void=>{current={...current,...change};writeChangeStateV1(directory,current);};
 let closeResolve!:(reason:string)=>void,stopping:Promise<void>|undefined,timer:ReturnType<typeof setInterval>|undefined,supervisor:ChangeWatchSupervisorV1|undefined,trust="none";
 const closed=new Promise<string>(resolve=>{closeResolve=resolve;});
 const stop=(reason:string):Promise<void>=>stopping??=(async()=>{
  if(timer)clearInterval(timer);
  try{await Promise.race([supervisor?.stop(),new Promise<void>(resolve=>{const bound=setTimeout(resolve,policy.stopWaitMilliseconds);bound.unref?.();})]);}catch{}
  writer.close();
  try{publish({ready:false,trust:"none",reason});}catch{}
  rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.heartbeat),{force:true});rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.lock),{recursive:true,force:true});
  closeResolve(reason);
 })();
 const unavailable=async(reason:string):Promise<never>=>{
  writeFileSync(join(directory,CHANGE_JOURNAL_FILES_V1.unavailable),JSON.stringify({at:now(),reason}),{mode:0o600});
  await stop(reason);
  throw refuseBuildV1("refused","journal.cannot-vouch",{reason});
 };
 try{
  supervisor=createSupervisor({
   root,policy:{...watcherPolicy,watcherLatencyMilliseconds:policy.watcherLatencyMilliseconds},filter:{excluded},rooted:exclude.rooted,names:exclude.names,binary,backend:binary?"native":"portable",latencyMs:policy.watcherLatencyMilliseconds,fenceDirectory:fence,instance,log,
   onBatch:async rows=>{writer.append(rows as readonly ChangeRowV1[],now());},
   onLost:(scope,reason)=>{writer.lost(reason,scope,now());},
   onTrust:state=>{
    const lost=trust==="exact"&&state.trust!=="exact";trust=state.trust;
    if(lost)writer.lost("trust-"+(state.reason||state.trust),"",now());
    if(current.ready)publish({trust:state.trust as ChangeStateV1["trust"],backend:state.backend as ChangeStateV1["backend"],reason:state.reason});
   },
  });
  const started=await supervisor.start();
  trust=started.trust;
  if(!["fsevents","inotify","rdcw"].includes(started.backend)||started.trust!=="exact")await unavailable(started.reason||started.backend+"/"+started.trust);
  publish({backend:started.backend as ChangeStateV1["backend"],trust:started.trust as ChangeStateV1["trust"],ready:true,reason:undefined});
 }catch(error){
  if(!stopping)await stop("failed");
  throw error;
 }
 touchChangeFileV1(directory,CHANGE_JOURNAL_FILES_V1.heartbeat,now());touchChangeFileV1(directory,CHANGE_JOURNAL_FILES_V1.activity,now());
 let swept=0;
 timer=setInterval(()=>{
  const at=now();
  touchChangeFileV1(directory,CHANGE_JOURNAL_FILES_V1.heartbeat,at);
  let activity=at;
  try{activity=statSync(join(directory,CHANGE_JOURNAL_FILES_V1.activity)).mtimeMs;}catch{}
  const here=directoryIdentityV1(root);
  if(here===undefined||here!==rootIdentity){void stop("root-gone").then(()=>rmSync(directory,{recursive:true,force:true}));return;}
  if(!existsSync(directory)){void stop("directory-gone");return;}
  if(at-activity>policy.idleShutdownMilliseconds){void stop("idle");return;}
  if(residentBytes()>policy.maximumResidentBytes){void stop("memory");return;}
  if(at-swept>10000){
   swept=at;
   const base=join(root,...fence.split("/"));
   try{for(const name of readdirSync(base))if(at-statSync(join(base,name)).mtimeMs>30000)rmSync(join(base,name),{force:true});}catch{}
  }
 },policy.heartbeatMilliseconds);
 timer.unref?.();
 log("ready",epoch);
 return{epoch,stop,closed};
}

/** 🏠️ The directory of the host process of this user: one detached process serves the change journals of every workspace, requests arrive as files below `requests`. */
export function changeJournalHostDirectoryV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 return join(dirname(stateDirectoryV1(canonicalRootV1(root),environment)),"journal-host");
}

/** 📨️ One request to open the journal of a workspace in the host. */
export type ChangeJournalOpenRequestV1=Readonly<{root:string;directory:string}>;

/** 🏠️ Everything one host run needs: the services it opens share one watcher factory (and so one native watcher process). */
export type ChangeJournalHostOptionsV1=Readonly<{
 directory:string;fence:string;exclude:ChangeJournalServiceOptionsV1["exclude"];binary?:string;watcherPolicy:Record<string,unknown>;createSupervisor:ChangeWatchFactoryV1;
 policy?:ChangeJournalPolicyV1;now?:()=>number;log?:(kind:string,detail:string)=>void;maximumRoots?:number;idleHostMilliseconds?:number;pollMilliseconds?:number;residentBytes?:()=>number;
}>;

const readOpenRequest=(file:string):ChangeJournalOpenRequestV1|undefined=>{
 try{
  const value=JSON.parse(readFileSync(file,"utf8")) as Partial<ChangeJournalOpenRequestV1>;
  return typeof value.root==="string"&&typeof value.directory==="string"&&value.root.length>0&&value.directory.length>0&&!value.root.includes("\0")&&!value.directory.includes("\0")?{root:value.root,directory:value.directory}:undefined;
 }catch{return undefined;}
};

/** 📨️ Asks the host to open the journal of a workspace; the file appears atomically. */
export function requestChangeJournalV1(hostDirectory:string,request:ChangeJournalOpenRequestV1):void{
 const requests=join(hostDirectory,"requests");
 mkdirSync(requests,{recursive:true,mode:0o700});
 const name=randomBytes(6).toString("hex"),temporary=join(requests,name+".tmp");
 writeFileSync(temporary,JSON.stringify(request),{mode:0o600});renameSync(temporary,join(requests,name+".json"));
}

/** 🏠️ Runs the host: one lock, one heartbeat, and a journal service per requested workspace, all in this process and sharing the native watcher of the supplied factory. The host ends after a quiet period without any workspace. */
export async function runChangeJournalHostV1(options:ChangeJournalHostOptionsV1):Promise<Readonly<{stop(reason:string):Promise<void>;closed:Promise<string>;services:ReadonlyMap<string,unknown>}>>{
 const {directory,policy=CHANGE_JOURNAL_POLICY_V1,now=Date.now,log=()=>{},maximumRoots=64,idleHostMilliseconds=60000,pollMilliseconds=100}=options;
 mkdirSync(directory,{recursive:true,mode:0o700});
 takeChangeLockV1(directory,policy,now());
 rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.heartbeat),{force:true});
 const requests=join(directory,"requests"),services=new Map<string,Awaited<ReturnType<typeof runChangeJournalServiceV1>>>();
 mkdirSync(requests,{recursive:true,mode:0o700});
 let closeResolve!:(reason:string)=>void,stopping:Promise<void>|undefined,timer:ReturnType<typeof setInterval>|undefined,quietSince=now(),busy=false;
 const closed=new Promise<string>(resolve=>{closeResolve=resolve;});
 const stop=(reason:string):Promise<void>=>stopping??=(async()=>{
  if(timer)clearInterval(timer);
  await Promise.allSettled([...services.values()].map(service=>service.stop(reason)));
  rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.heartbeat),{force:true});rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.lock),{recursive:true,force:true});
  closeResolve(reason);
 })();
 const open=async(request:ChangeJournalOpenRequestV1)=>{
  if(services.has(request.directory)||services.size>=maximumRoots)return;
  try{
   const service=await runChangeJournalServiceV1({root:request.root,directory:request.directory,fence:options.fence,exclude:options.exclude,binary:options.binary,watcherPolicy:options.watcherPolicy,createSupervisor:options.createSupervisor,policy,now,log,...(options.residentBytes?{residentBytes:options.residentBytes}:{residentBytes:()=>0})});
   services.set(request.directory,service);
   void service.closed.then(()=>{services.delete(request.directory);});
  }catch(error){if(!(error instanceof ChangeJournalRunningErrorV1))log("open-failed",request.root+": "+String((error as Error).message));}
 };
 const poll=async()=>{
  if(busy||stopping)return;busy=true;
  try{
   const at=now();touchChangeFileV1(directory,CHANGE_JOURNAL_FILES_V1.heartbeat,at);
   let names:string[]=[];try{names=readdirSync(requests).filter(name=>name.endsWith(".json")).sort();}catch{}
   for(const name of names){
    const file=join(requests,name),request=readOpenRequest(file);
    rmSync(file,{force:true});
    if(request)await open(request);
   }
   if(services.size>0)quietSince=at;
   else if(at-quietSince>idleHostMilliseconds)void stop("idle");
  }finally{busy=false;}
 };
 touchChangeFileV1(directory,CHANGE_JOURNAL_FILES_V1.heartbeat,now());
 timer=setInterval(()=>{void poll();},pollMilliseconds);timer.unref?.();
 await poll();
 log("host-ready",directory);
 return{stop,closed,services};
}

/** 🚀️ Makes the journal of a workspace run: asks the live host to open it, or starts the detached host first, and waits until its state is ready; resolves quietly when it cannot (the client then walks the tree). */
export async function startChangeJournalServiceV1(options:Readonly<{directory:string;root:string;execPath?:string;script?:string;arguments?:readonly string[];policy?:ChangeJournalPolicyV1;now?:()=>number;environment?:Readonly<Record<string,string|undefined>>}>):Promise<void>{
 const {directory,root,execPath=process.execPath,script=fileURLToPath(new URL("./⌨️main/🟦️.ts",import.meta.url)),policy=CHANGE_JOURNAL_POLICY_V1,now=Date.now,environment=process.env}=options;
 const live=():boolean=>{const state=readChangeStateV1(directory);return !!state&&changeHeartbeatAgeV1(directory,now())<policy.heartbeatStaleMilliseconds&&state.ready;};
 if(live())return;
 const hostDirectory=changeJournalHostDirectoryV1(root,environment);
 mkdirSync(directory,{recursive:true,mode:0o700});
 const hostLive=():boolean=>changeHeartbeatAgeV1(hostDirectory,now())<policy.heartbeatStaleMilliseconds;
 requestChangeJournalV1(hostDirectory,{root,directory});
 let exited=false;
 if(!hostLive()){
  const child=spawn(execPath,[script,"--host",hostDirectory,...(options.arguments??[])],{cwd:root,env:{...environment},stdio:"ignore",detached:true,windowsHide:true});
  child.unref();child.once("exit",()=>{exited=true;});
 }
 const deadline=now()+policy.startTimeoutMilliseconds;
 while(now()<deadline&&!(exited&&!hostLive())){
  if(live())return;
  await new Promise(resolve=>setTimeout(resolve,policy.startPollMilliseconds));
 }
}

/** 🏁️ The command line of the detached host process: `--host <host directory> [--binary <watcher>]`. */
export async function runChangeJournalMainV1(argv:readonly string[],dependencies:Readonly<{createSupervisor:ChangeWatchFactoryV1;watcherPolicy:Record<string,unknown>;exclude:ChangeJournalServiceOptionsV1["exclude"];fence:string;locate:()=>Promise<string|undefined>;release?:()=>Promise<void>}>):Promise<number>{
 const value=(name:string):string|undefined=>{const at=argv.indexOf(name);return at<0?undefined:argv[at+1];};
 const directory=value("--host");
 if(!directory)throw refuseBuildV1("usage","journal.arguments-required");
 const binary=value("--binary")??await dependencies.locate();
 let host;
 try{host=await runChangeJournalHostV1({directory,fence:dependencies.fence,exclude:dependencies.exclude,binary,watcherPolicy:dependencies.watcherPolicy,createSupervisor:dependencies.createSupervisor,residentBytes:()=>process.memoryUsage().rss});}
 catch(error){return error instanceof ChangeJournalRunningErrorV1?0:1;}
 for(const signal of["SIGTERM","SIGINT"] as const)process.once(signal,()=>{void host.stop(signal);setTimeout(()=>process.exit(1),CHANGE_JOURNAL_POLICY_V1.stopWaitMilliseconds*3).unref();});
 await host.closed;
 await dependencies.release?.();
 return 0;
}

/** 📍️ The journal directory of a workspace: below the private state directory of the worktree and user, which no `TMPDIR` moves, so a nested build action reaches the journal of its parent. */
export function changeJournalDirectoryV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 return join(stateDirectoryV1(canonicalRootV1(root),environment),"journal");
}

/** 🗻️ The host ports of the file-system question: the platform, a text file and one program's output (absent where either fails or takes longer than the declared bound). */
export const CHANGE_HOST_PORTS_V1:ChangeFileSystemPortsV1=Object.freeze({
 platform:process.platform,
 readText:async path=>{try{return await readFile(path,"utf8");}catch{return undefined;}},
 run:(program,args)=>new Promise<string|undefined>(resolve=>{try{execFile(program,[...args],{encoding:"utf8",timeout:CHANGE_FILE_SYSTEM_POLICY_V1.probeTimeoutMilliseconds,windowsHide:true,maxBuffer:16*1024*1024},(error,stdout)=>resolve(error?undefined:stdout));}catch{resolve(undefined);}}),
});

/** 🔌️ The client every consumer uses: it starts the service on demand and falls back (by refusing) whenever the journal cannot vouch. */
export function connectChangeJournalV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):ChangeJournalClientV1{
 const canonical=canonicalRootV1(root),directory=changeJournalDirectoryV1(canonical,environment);
 return createChangeJournalClientV1({directory,root:canonical,environment,start:()=>startChangeJournalServiceV1({directory,root:canonical,environment}),fileSystem:root=>inspectChangeFileSystemV1((()=>{try{return realpathSync(root);}catch{return root;}})(),CHANGE_HOST_PORTS_V1)});
}
