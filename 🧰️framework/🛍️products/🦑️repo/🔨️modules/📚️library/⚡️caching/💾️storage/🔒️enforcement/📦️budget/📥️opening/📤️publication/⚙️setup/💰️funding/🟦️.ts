import {processPidExistsV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts"
import{readIssuedScriptProcessEnvelopeV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{readNativeCallerAuthorityV1}from"../../../../../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import{createHash,randomInt}from"node:crypto";
import{AsyncLocalStorage}from"node:async_hooks";
import{join}from"node:path";
import type{ScriptInvocation}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{observeNativePlatformProtectedInitializationV1,withNativePlatformProtectedMaintenanceParentV1,withNativePlatformProtectedMaintenanceReadV1,readNativePlatformProtectedMaintenanceJournalPresenceV1,withNativePlatformProtectedFundingReplacementV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🟦️.ts";
import{nativeImageRootV1,type NativeProtectedMaintenanceParentV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import type{NativeStorageAllocationV1,NativeStoragePolicyV1,NativeStorageStateV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import{readNativeImageRetirementFileV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📄️retirement-file/🟦️.ts";
import{withArtifactRelativeDirectoryV1}from"../../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import{spawnSync}from"node:child_process";
import{readdirSync,readFileSync,rmSync}from"node:fs";
import{leaseElapsedSecondsV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts";
import{leaseProcessStartedAtMsV1,leaseRegistryFolderV1,leaseRegistryRootV1,type LeaseRegistryEntryV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";
import{repositoryCacheImagePublicationOptionsV1}from"../../🟦️.ts";
import setupSchema from"../🧬️schema.json"with{type:"json"};
import{isRepoErrorV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import{openRepositoryCachePartitionV1}from"../../../♻️reset/🟦️.ts";
import{refuseCachingV1,sayCachingV1}from"../../../../../../../📖️texts/🟦️.ts";
import{javascriptInstallSchemaV1 as dependencyInstallSchema}from"../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧱️schemas/🟦️.ts";
import{observeRepositoryDependencyMaintenanceParentV1}from"../../📸️observation/🟦️.ts";
import{readRepositoryDependencyAllocationMarkerV1}from"../📋️contract/🟦️.ts";
const owner="repository:dependencies-store";
type Marker=ReturnType<typeof readRepositoryDependencyAllocationMarkerV1>;
type Observation=Readonly<{presence:"initial-empty";currentBytes:number;authority:false}>|Readonly<{presence:"present";state:NativeStorageStateV1;marker:Marker|undefined;markerSha256:string|undefined;markerIntent:boolean;currentBytes:number}>;
/** ⏳️ How a child waits for the dependency store a sibling holds: polls start at `first`, double up to `maximum` with a quarter of jitter so two waiting children do not stay in step, say a line every `progress`, and give up after the queue bound of the store sync. */
export const DEPENDENCY_STORE_WAIT_V1=Object.freeze({firstMilliseconds:200,maximumMilliseconds:2000,progressMilliseconds:10000,ancestorGraceMilliseconds:15000,verifiedMaintenanceMilliseconds:20000,holderStartToleranceMilliseconds:5000,heartbeatStaleMilliseconds:300000,limitMilliseconds:dependencyInstallSchema.$defs.SourceOwnerStoreSyncProtocol.const.maximumQueueWaitMilliseconds});
const dependencyStoreWaitScopeV1=new AsyncLocalStorage<number>();
/** ⏱️ Runs `body` with every dependency store wait of this async scope bounded to `milliseconds` (the smaller of it and the bound of the wait ports), so an optional step never waits as long as a required one. */
export function withDependencyStoreWaitBoundV1<T>(milliseconds:number,body:()=>Promise<T>):Promise<T>{return dependencyStoreWaitScopeV1.run(milliseconds,body);}
/** 🔍️ What the lock-free evidence says about the dependency store: `unverified` (envelope, funding or journal not provably current: the exclusive observation is needed), `occupied` (provably current and every live holder is shared: nothing needs exclusive access and none is possible, so nobody is waited for) or `verified` (provably current and no shared-only holder: maintenance is optional, tried within a bound). */
export function dependencyStoreVerdictV1(evidence:Readonly<{envelope:boolean;pending:boolean;funding:boolean;holders:DependencyStoreHoldersV1}>):"unverified"|"occupied"|"verified"{
  if(!evidence.envelope||evidence.pending||!evidence.funding)return"unverified";
  return evidence.holders.live.length>0&&evidence.holders.live.every(holder=>holder.mode==="shared")?"occupied":"verified";
}
/** 🪪️ One process that holds the dependency store lease, as the lease registry published it: who (`pid`, `command`), since when (`sinceMs`), in which mode, and whether it is this process or one of its ancestors, whose release this process can never wait for. */
export type DependencyStoreHolderV1=Readonly<{pid:number;command:string;sinceMs:number;silentMs?:number;mode:"shared"|"exclusive";ancestor:boolean}>;
/** 👁️ What one look at the registry found: the holders that are alive, and those whose process is gone and whose entry was removed by that look. */
export type DependencyStoreHoldersV1=Readonly<{live:readonly DependencyStoreHolderV1[];gone:readonly DependencyStoreHolderV1[]}>;
export type DependencyStoreWaitPortsV1=Readonly<{now():number;sleep(milliseconds:number,signal:AbortSignal):Promise<void>;say(seconds:number,limit:number,holder?:DependencyStoreHolderV1):void;limitMilliseconds:number;observe?():DependencyStoreHoldersV1;sayGone?(holder:DependencyStoreHolderV1):void}>;
/** 🔌️ What the holder lookup touches outside its decisions: the registry entries of one lease directory with their files, process liveness, the ancestors of this process, the removal of an entry and the process start time. */
export type DependencyStoreRegistryPortsV1=Readonly<{entries(directory:string):readonly Readonly<{path:string;entry:LeaseRegistryEntryV1}>[];alive(pid:number):boolean;osStartedAtMs?(pid:number):number|undefined;now?():number;ancestors():readonly number[];remove(path:string):void;pid:number;startedAtMs:number}>;
/** 🧬️ Walks the parents of a process upward, nearest first, until a root, a repeat or the bound; a parent that cannot be read ends the walk. */
export function processAncestorsV1(parentOf:(pid:number)=>number|undefined,start:number,limit=64):readonly number[]{
  const found:number[]=[];
  for(let pid=start;Number.isSafeInteger(pid)&&pid>1&&!found.includes(pid)&&found.length<limit;pid=parentOf(pid)??0)found.push(pid);
  return Object.freeze(found);
}
const parentOfProcessV1=(pid:number):number|undefined=>{
  const run=process.platform==="win32"?spawnSync("powershell",["-NoProfile","-Command","(Get-CimInstance Win32_Process -Filter \"ProcessId="+pid+"\").ParentProcessId"],{windowsHide:true,encoding:"utf8"}):spawnSync("ps",["-o","ppid=","-p",String(pid)],{encoding:"utf8",env:{...process.env,LC_ALL:"C"}}),value=Number.parseInt(run.stdout,10);
  return run.status===0&&Number.isSafeInteger(value)?value:undefined;
};
/** 🧬️ Whether a live pid is not the process that registered the hold: the operating system started it more than the tolerance after the registered start, so the pid was reused (Windows reuses pids within hours); an unknown start never counts as reused. */
export function dependencyStorePidReusedV1(registeredStartedAtMs:number|undefined,osStartedAtMs:number|undefined):boolean{return osStartedAtMs!==undefined&&Number.isFinite(osStartedAtMs)&&registeredStartedAtMs!==undefined&&Number.isFinite(registeredStartedAtMs)&&osStartedAtMs-registeredStartedAtMs>DEPENDENCY_STORE_WAIT_V1.holderStartToleranceMilliseconds;}
/** 🫀️ Whether a registered holder is still a holder: its process is alive, is not a later process under a reused pid, and, when the operating system could not tell its start (a loaded Windows host often times out the lookup, which left the dead owner of a reused pid in the way for hours), has renewed its registration within `heartbeatStaleMilliseconds`; a holder whose start is known and matches stays whatever its heartbeat, it only reports how long it has been silent, and an ancestor of this process is never judged by its heartbeat (it blocks in a synchronous spawn while its child waits here). */
export function dependencyStoreHolderPresentV1(alive:boolean,registeredStartedAtMs:number|undefined,osStartedAtMs:number|undefined,silentMs:number):boolean{return alive&&!dependencyStorePidReusedV1(registeredStartedAtMs,osStartedAtMs)&&!(osStartedAtMs===undefined&&silentMs>DEPENDENCY_STORE_WAIT_V1.heartbeatStaleMilliseconds);}
/** 🗝️ The text a wait line uses: no holder known, a holder that renews its registration, or one that has been silent past the heartbeat bound and may be blocked. */
export function dependencyStoreWaitKeyV1(holder?:DependencyStoreHolderV1):"dependency-store.waiting"|"dependency-store.waiting-for-holder"|"dependency-store.waiting-for-silent-holder"{return holder===undefined?"dependency-store.waiting":(holder.silentMs??0)>DEPENDENCY_STORE_WAIT_V1.heartbeatStaleMilliseconds?"dependency-store.waiting-for-silent-holder":"dependency-store.waiting-for-holder";}
/** 📝️ The key and the exact values of one wait line: only the silent text takes `silent`, because the catalog refuses a value without a placeholder. */
export function dependencyStoreWaitLineV1(seconds:number,limit:number,holder?:DependencyStoreHolderV1):Readonly<{key:ReturnType<typeof dependencyStoreWaitKeyV1>;values:Readonly<Record<string,string|number>>}>{const key=dependencyStoreWaitKeyV1(holder);return holder===undefined?{key,values:{seconds,limit}}:{key,values:{seconds,limit,...holderValuesV1(holder),...key==="dependency-store.waiting-for-silent-holder"?{silent:Math.round((holder.silentMs??0)/1000)}:{}}};}
const osStartedAtMsOfProcessV1=(pid:number):number|undefined=>{
  if(!Number.isSafeInteger(pid)||pid<1)return undefined;
  if(process.platform==="win32"){const run=spawnSync("powershell",["-NoProfile","-Command","[DateTimeOffset]::new((Get-Process -Id "+pid+" -ErrorAction Stop).StartTime).ToUnixTimeMilliseconds()"],{windowsHide:true,encoding:"utf8",timeout:15000}),value=Number.parseInt(run.stdout,10);return run.status===0&&Number.isSafeInteger(value)?value:undefined;}
  const run=spawnSync("ps",["-o","etime=","-p",String(pid)],{encoding:"utf8",env:{...process.env,LC_ALL:"C"},timeout:15000}),seconds=leaseElapsedSecondsV1(run.stdout);return run.status===0&&seconds!==undefined?Date.now()-seconds*1000:undefined;
};
const processAliveV1=(pid:number):boolean=>processPidExistsV1(pid);
const registryPortsV1:DependencyStoreRegistryPortsV1=Object.freeze({
  entries:directory=>{
    const folder=leaseRegistryFolderV1(leaseRegistryRootV1(),directory),rows:Array<Readonly<{path:string;entry:LeaseRegistryEntryV1}>>=[];
    let names:string[];try{names=readdirSync(folder).filter(name=>/^\d+-\d+\.json$/u.test(name));}catch{return rows;}
    for(const name of names)try{const path=join(folder,name),entry=JSON.parse(readFileSync(path,"utf8")) as LeaseRegistryEntryV1;if(entry?.version===1&&Number.isSafeInteger(entry.pid)&&Array.isArray(entry.holds)&&entry.directory===directory)rows.push(Object.freeze({path,entry}));}catch{}
    return rows;
  },
  alive:processAliveV1,
  osStartedAtMs:osStartedAtMsOfProcessV1,
  ancestors:()=>processAncestorsV1(parentOfProcessV1,process.ppid),
  remove:path=>{try{rmSync(path,{force:true});}catch{}},
  pid:process.pid,
  startedAtMs:leaseProcessStartedAtMsV1,
});
/** 🕵️ Reads who holds one lease from the registry: a holder is an ancestor when its process is this process or one of its parents and it started before this process (a reused pid of a dead holder is not an ancestor); the entry of a holder whose process is gone is removed at once, so a claim that nobody can release does not outlive its process and is reported by name. */
export function observeDependencyStoreHoldersV1(directory:string,resource:string,ports:DependencyStoreRegistryPortsV1=registryPortsV1):DependencyStoreHoldersV1{
  const parents=new Set<number>(ports.ancestors()),live:DependencyStoreHolderV1[]=[],gone:DependencyStoreHolderV1[]=[];
  for(const{path,entry}of ports.entries(directory)){
    const hold=entry.holds.find(row=>row.resource===resource);if(!hold)continue;
    const self=entry.pid===ports.pid,silentMs=Number.isFinite(entry.updatedAtMs)?Math.max(0,(ports.now?.()??Date.now())-entry.updatedAtMs):0,holder=(ancestor:boolean):DependencyStoreHolderV1=>Object.freeze({pid:entry.pid,command:entry.command.slice(0,160),sinceMs:hold.sinceMs,silentMs,mode:hold.mode,ancestor});
    if(!self&&!dependencyStoreHolderPresentV1(ports.alive(entry.pid),entry.startedAtMs,ports.alive(entry.pid)?ports.osStartedAtMs?.(entry.pid):undefined,parents.has(entry.pid)?0:silentMs)){ports.remove(path);gone.push(holder(false));continue;}
    live.push(holder(self||parents.has(entry.pid)&&entry.startedAtMs<=ports.startedAtMs));
  }
  return Object.freeze({live:Object.freeze(live.sort((left,right)=>left.sinceMs-right.sinceMs)),gone:Object.freeze(gone)});
}
const holderValuesV1=(holder:DependencyStoreHolderV1)=>({pid:holder.pid,command:holder.command,since:new Date(holder.sinceMs).toISOString()});
const dependencyStoreWaitPorts:DependencyStoreWaitPortsV1=Object.freeze({now:()=>Date.now(),sleep:(milliseconds,signal)=>new Promise<void>((done,fail)=>{signal.throwIfAborted();const stop=()=>{clearTimeout(timer);fail(signal.reason);},timer=setTimeout(()=>{signal.removeEventListener("abort",stop);done();},milliseconds);signal.addEventListener("abort",stop,{once:true});}),say:(seconds,limit,holder)=>{const line=dependencyStoreWaitLineV1(seconds,limit,holder);sayCachingV1(line.key,line.values);},sayGone:holder=>sayCachingV1("dependency-store.holder-gone",holderValuesV1(holder),"warn"),limitMilliseconds:DEPENDENCY_STORE_WAIT_V1.limitMilliseconds});
/** 🧭️ The wait ports for one dependency store: the defaults plus a look at the registry for the holder of its image lease; the lease is named lazily, so a lookup that cannot even name it simply finds no holder. */
export function dependencyStoreWaitPortsForV1(lease:()=>Readonly<{directory:string;resource:string}>,registry:DependencyStoreRegistryPortsV1=registryPortsV1):DependencyStoreWaitPortsV1{
  return Object.freeze({...dependencyStoreWaitPorts,observe:()=>{try{const named=lease();return observeDependencyStoreHoldersV1(named.directory,named.resource,registry);}catch{return Object.freeze({live:Object.freeze([]),gone:Object.freeze([])});}}});
}
/** 🔁️ Repeats an attempt that found the dependency store parent held by a sibling until it is acquired: a busy parent means wait, never fail. Cancellation and the deadline of the control end the wait at once; so does the bound, with a bilingual refusal. */
export async function retryWhileDependencyStoreBusyV1<T>(control:ResourceLeaseControl,attempt:()=>Promise<Readonly<{acquired:true;value:T}>|Readonly<{acquired:false}>>,ports:DependencyStoreWaitPortsV1=dependencyStoreWaitPorts):Promise<T>{
  const started=ports.now();let step:number=DEPENDENCY_STORE_WAIT_V1.firstMilliseconds,said:number|undefined,ancestorSince:number|undefined;
  for(;;){
    checkResourceLeaseControl(control);const selected=await attempt();if(selected.acquired)return selected.value;
    const now=ports.now(),waited=now-started,seen=ports.observe?.(),live=seen?.live??[],limit=Math.min(ports.limitMilliseconds,dependencyStoreWaitScopeV1.getStore()??Infinity);
    for(const holder of seen?.gone??[])ports.sayGone?.(holder);
    if(live.length>0&&live.every(holder=>holder.ancestor)){ancestorSince??=now;if(now-ancestorSince>=DEPENDENCY_STORE_WAIT_V1.ancestorGraceMilliseconds)throw refuseCachingV1("refused","dependency-store.ancestor-holds",{seconds:Math.round((now-ancestorSince)/1000),...holderValuesV1(live[0]!)});}else ancestorSince=undefined;
    if(waited>=limit)throw refuseCachingV1("timeout","dependency-store.busy-timeout",{seconds:Math.round(waited/1000)});
    if(said===undefined||waited-said>=DEPENDENCY_STORE_WAIT_V1.progressMilliseconds){ports.say(Math.round(waited/1000),Math.round(limit/1000),live.find(holder=>!holder.ancestor)??live[0]);said=waited;}
    await ports.sleep(Math.min(Math.round(step*randomInt(75,126)/100),limit-waited),control.signal);step=Math.min(step*2,DEPENDENCY_STORE_WAIT_V1.maximumMilliseconds);
  }
}
/** 🛡️ The holder that shelters this process: the store is held, every holder is an ancestor (or this process) and every hold is shared, each verified by the registry entry of a live process, so exclusive access is impossible for the life of this command and none is needed when the funding is current. */
export function dependencyStoreShelteredByAncestorV1(holders:DependencyStoreHoldersV1):DependencyStoreHolderV1|undefined{
  return holders.live.length>0&&holders.live.every(holder=>holder.ancestor&&holder.mode==="shared")?holders.live[0]:undefined;
}
/** 🟢️ Whether the dependency funding is current, judged without any exclusive access from what the image shows to a reader: no funding journal is pending beside the image and the marker inside it names the dependencies store with the whole declared envelope. A journal row that disagrees with the marker is found only by the exclusive observation, which a command beneath an ancestor's hold cannot take. */
export function readRepositoryDependencyFundingCurrentV1(options:Readonly<{directory:string;filesystemBytes:number}>,check:()=>void):boolean{
  try{
    const root=nativeImageRootV1(options.directory);check();
    if(readRepositoryDependencyMarkerIntentFileV1(root,check))return false;
    const record=readNativeImageRetirementFileV1(join(root,"dependencies.json"),2048),marker=readRepositoryDependencyAllocationMarkerV1(new TextDecoder("utf-8",{fatal:true}).decode(record.bytes));check();
    return marker.owner===owner&&marker.bytes===options.filesystemBytes;
  }catch{check();return false;}
}
/** 🌊️ How a funding is re-run when the shared tree moved under it: bounded by the schema, with the pauses it declares. */
export type DependencyFundingMovePortsV1=Readonly<{now?():number;sleep(milliseconds:number,signal:AbortSignal):Promise<void>;say(attempt:number,limit:number,what:string,path:string,seconds:number):void}>;
const fundingMovement=setupSchema.definitions.FundingMovementProtocol.const,fundingMovePorts:DependencyFundingMovePortsV1=Object.freeze({sleep:(milliseconds,signal)=>dependencyStoreWaitPorts.sleep(milliseconds,signal),say:(attempt,limit,what,path,seconds)=>sayCachingV1("dependency-store.funding-rerun",{attempt,limit,what,path,seconds})});
/** 🔭️ Whether a failure of the funding says that something it consumes moved under it, found in the typed movement refusal or in the stable text of the input recheck anywhere in the chain of causes and aggregate members; names what moved. */
export function dependencyFundingMovedV1(error:unknown):Readonly<{what:string;path:string}>|undefined{
  const seen=new Set<unknown>(),walk=(value:unknown,depth:number):Readonly<{what:string;path:string}>|undefined=>{
    if(depth>16||value===null||typeof value!=="object"||seen.has(value))return undefined;seen.add(value);
    if(isRepoErrorV1(value)&&value.key===fundingMovement.key)return Object.freeze({what:String(value.values.what),path:String(value.values.path)});
    const row=value as{message?:unknown;cause?:unknown;errors?:unknown};
    if(typeof row.message==="string"&&fundingMovement.signals.some(text=>(row.message as string).includes(text)))return Object.freeze({what:"inputs",path:row.message.slice(0,160)});
    for(const next of[row.cause,...Array.isArray(row.errors)?row.errors:[]]){const found=walk(next,depth+1);if(found)return found;}
    return undefined;
  };
  return walk(error,0);
}
/** 🔁️ Runs a funding again when what it consumes moved under it (a change of content, never of identity or times), for at most `maximumMilliseconds` with the declared pauses (the last repeats), each run resuming from the progress already published; anything else ends it at once, and past the bound it ends in a typed bilingual exit 75 refusal that names what moved. */
export async function retryWhileDependencyFundingMovesV1<T>(control:ResourceLeaseControl,attempt:()=>Promise<T>,ports:DependencyFundingMovePortsV1=fundingMovePorts):Promise<T>{
  const clock=ports.now??Date.now,began=clock();
  for(let run=0;;run++){
    checkResourceLeaseControl(control);
    try{return await attempt();}catch(error){
      const moved=dependencyFundingMovedV1(error);if(!moved)throw error;
      if(clock()-began>=fundingMovement.maximumMilliseconds)throw refuseCachingV1("unserved","dependency-store.moving",{attempts:run+1,what:moved.what,path:moved.path},{cause:error});
      const pause=fundingMovement.pausesMilliseconds[Math.min(run,fundingMovement.pausesMilliseconds.length-1)]!;ports.say(run+1,Math.round(fundingMovement.maximumMilliseconds/60000),moved.what,moved.path,Math.round(pause/1000));await ports.sleep(pause,control.signal);
    }
  }
}
/** 🔒️ Owns one freshly captured genuine parent for one attempt and closes its infrastructure before the next phase; a parent held by a sibling reports `acquired:false` without running the body. */
async function attemptFundingParentV1<T>(invocation:ScriptInvocation,control:ResourceLeaseControl,body:(parent:NativeProtectedMaintenanceParentV1,actual:ResourceLeaseControl,policy:NativeStoragePolicyV1)=>Promise<T>):Promise<Readonly<{acquired:true;value:T}>|Readonly<{acquired:false}>>{
  const observed=await observeRepositoryDependencyMaintenanceParentV1(invocation,control),options=observed.options,root=nativeImageRootV1(options.directory),policy={version:1 as const,root,leaseDirectory:options.leaseDirectory,capacityBytes:options.maximumPhysicalBytes,maximumWorkspaceBytes:options.filesystemBytes,maximumArtifactBytes:options.filesystemBytes},infrastructure=await openRepositoryCachePartitionV1(readNativeCallerAuthorityV1(readIssuedScriptProcessEnvelopeV1(invocation).capabilities).repositoryRoot,"leases",control);let result:Readonly<{acquired:true;value:T}>|Readonly<{acquired:false}>|undefined;const failures:unknown[]=[];
  try{result=await withNativePlatformProtectedMaintenanceParentV1({directory:options.directory,leaseDirectory:options.leaseDirectory,filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:options.maximumPhysicalBytes,control},observed.parent,(parent,actual)=>body(parent,actual,policy));}catch(error){failures.push(error);}try{await infrastructure.close();}catch(error){failures.push(error);}if(failures.length>1)throw new AggregateError(failures,"Dependency funding and infrastructure close failed");if(failures.length)throw failures[0];return result!;
}
/** 🪢️ Runs the body under the funding parent, waiting for a sibling that holds it. */
function withFundingParentV1<T>(invocation:ScriptInvocation,control:ResourceLeaseControl,body:(parent:NativeProtectedMaintenanceParentV1,actual:ResourceLeaseControl,policy:NativeStoragePolicyV1)=>Promise<T>):Promise<T>{return retryWhileDependencyStoreBusyV1(control,()=>attemptFundingParentV1(invocation,control,body),dependencyStoreWaitPortsForV1(()=>{const options=repositoryCacheImagePublicationOptionsV1(invocation,control,"dependencies");return{directory:options.leaseDirectory,resource:"native-fixed-image:"+options.directory};}));}
/** 📖️ Reads actual retained charges before any current-envelope shortcut can return. */
async function observeFundingV1(parent:NativeProtectedMaintenanceParentV1,control:ResourceLeaseControl,policy:NativeStoragePolicyV1):Promise<Observation>{
 const journal=await readNativePlatformProtectedMaintenanceJournalPresenceV1(parent,control,policy);if(journal.presence!=="present"||journal.state.allocations.length===0){await observeNativePlatformProtectedInitializationV1(parent,control,policy);return Object.freeze({presence:"initial-empty",currentBytes:policy.maximumArtifactBytes,authority:false});}return withNativePlatformProtectedMaintenanceReadV1(parent,control,async(read,actual)=>{read.check();if(read.root!==policy.root||read.associations.length)throw Error("Exact leaf Dependencies funding parent required");const check=()=>{checkResourceLeaseControl(actual);read.check();},intent=readRepositoryDependencyMarkerIntentFileV1(read.root,check),path=join(read.root,"dependencies.json");let record:ReturnType<typeof readNativeImageRetirementFileV1>;try{record=readNativeImageRetirementFileV1(path,2048);}catch(error){check();if((error as NodeJS.ErrnoException).code!=="ENOENT"||!intent)throw error;return Object.freeze({presence:"present",state:journal.state,marker:undefined,markerSha256:undefined,markerIntent:true,currentBytes:policy.maximumArtifactBytes});}const marker=readRepositoryDependencyAllocationMarkerV1(new TextDecoder("utf-8",{fatal:true}).decode(record.bytes));check();if(readNativeImageRetirementFileV1(path,2048).stamp!==record.stamp)throw Error("Dependencies funding marker changed");return Object.freeze({presence:"present",state:journal.state,marker,markerSha256:createHash("sha256").update(record.bytes).digest("hex"),markerIntent:!!intent,currentBytes:policy.maximumArtifactBytes});});
}
/** 🧮️ Exposes detached actual funding facts; they issue no replacement or readiness permission. */
export function observeRepositoryDependencyBootstrapFundingV1(invocation:ScriptInvocation,control:ResourceLeaseControl):Promise<Observation>{return withFundingParentV1(invocation,control,(parent,actual,policy)=>observeFundingV1(parent,actual,policy));}
/** 💰️ Reconciles actual postpublication funding and marker CAS through the private Native issuer. */
export function reconcileRepositoryDependencyBootstrapFundingV1(invocation:ScriptInvocation,control:ResourceLeaseControl):Promise<void>{return withFundingParentV1(invocation,control,async(parent,actual,policy)=>{
 const observed=await observeFundingV1(parent,actual,policy);if(observed.presence!=="present")throw Error("Initial empty funding must use the existing canonical allocator");const rows=observed.state.allocations,marker=observed.marker,exact=(row:NativeStorageAllocationV1|undefined)=>!!marker&&!!row&&row.id===marker.allocation&&row.owner===marker.owner&&row.bytes===marker.bytes&&row.kind==="artifact-publication"&&row.phase==="committed"&&!row.child&&!row.namespace;
 if(rows.some(row=>row.owner!==owner||row.kind!=="artifact-publication"||row.child||row.namespace)||rows.length<1||rows.length>2)throw Error("Exact bounded Dependencies funding association required");const matching=rows.find(row=>exact(row));if(!observed.markerIntent&&rows.length===1&&matching&&matching.bytes===policy.maximumArtifactBytes)return;
 await withNativePlatformProtectedFundingReplacementV1(parent,actual,control,policy,async issuer=>{
 let pending=readRepositoryDependencyMarkerIntentFileV1(policy.root,()=>checkResourceLeaseControl(actual)),contextSha256=pending?.baseline?.contextSha256??observed.markerSha256;if(!contextSha256)throw Error("Exact original Dependencies marker context required");let claim:import("../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts").NativeStorageFundingReplacementV1,settled:NativeStorageAllocationV1;const associated=rows.find(row=>row.funding?.phase==="reserved")??rows.find(row=>!!row.funding&&row.id!==marker?.allocation)??rows.find(row=>!!row.funding);
 if(associated){const funding=associated.funding!,predecessor=marker&&funding.predecessor.id===marker.allocation&&funding.predecessor.owner===marker.owner&&funding.predecessor.bytes===marker.bytes;if(funding.contextSha256!==contextSha256||funding.targetBytes!==policy.maximumArtifactBytes||(pending?(pending.baseline?associated.id!==pending.baseline.allocation.id:!predecessor)||funding.phase!=="settled"||rows.length!==1:(!predecessor||funding.phase==="reserved"&&(!matching||rows.length!==2)||funding.phase==="settled"&&rows.length!==1)))throw Error("Exact interrupted Dependencies funding predecessor required");claim=await issuer.recover({version:1,expectedSequence:observed.state.sequence,id:associated.id,owner,contextSha256});settled=funding.phase==="reserved"?await issuer.settle(claim):issuer.read(claim).allocation;}
 else{if(pending||rows.length!==1||!matching||matching.bytes>=policy.maximumArtifactBytes)throw Error("Actual smaller committed Dependencies funding required");claim=await issuer.reserve({version:1,expectedSequence:observed.state.sequence,predecessor:{id:matching.id,owner:matching.owner,kind:"artifact-publication",phase:"committed",bytes:matching.bytes},contextSha256});settled=await issuer.settle(claim);}
 if(settled.kind!=="artifact-publication"||settled.phase!=="committed"||settled.owner!==owner||settled.bytes!==policy.maximumArtifactBytes)throw Error("Actual issued current Dependencies funding required");const currentFunding=issuer.read(claim),bytes=repositoryDependencyFundingMarkerBytesV1(settled),sha256=createHash("sha256").update(bytes).digest("hex");const extension=defineNativeFundingMetadataRoleV1(control,{version:1,id:"repository-dependencies-funding-marker",owner,targetName:"dependencies.json",intentName:"funding.json",preparedPurpose:"publish-current-dependency-funding-marker",maximumTargetBytes:2048,policy},repositoryDependencyFundingMarkerBytesV1),metadata=issuer.metadataInfo(extension,claim);if(pending&&!pending.intent){const retired=await issuer.retireUnpreparedMetadata(extension,claim);if(!retired.intentRemoved)await removeRepositoryDependencyMarkerIntentFileV1(policy.root,pending,()=>checkResourceLeaseControl(actual));pending=undefined;}let intentFile:RepositoryDependencyMarkerIntentFileV1|undefined=pending;
 await withNativePlatformProtectedMaintenanceReadV1(parent,actual,async(read,current)=>{const check=()=>{checkResourceLeaseControl(current);read.check();};return withArtifactRelativeDirectoryV1(read.root,check,async directory=>{
 const same=(left:ArtifactReadFileIdentityV1,right:ArtifactReadFileIdentityV1)=>["dev","ino","mode","size","nlink","mtimeNs","ctimeNs"].every(key=>left[key as keyof ArtifactReadFileIdentityV1]===right[key as keyof ArtifactReadFileIdentityV1]);let published:Awaited<ReturnType<typeof directory.publishFile>>;
 if(intentFile?.intent){requireRepositoryDependencyMarkerPublicationBindingV1(intentFile.intent,currentFunding,directory.stat());const recovery=await directory.captureFileRecovery(intentFile.intent.prepared);published=await directory.recoverFile(recovery);}
 else{const names=await directory.names();if(names.some(name=>artifactRelativeFilePublicationStageNameV1("dependencies.json",name)))throw Error("Unprepared marker publication stage remains retained");const expected=await directory.captureFilePublication("dependencies.json",2048);if(expected.kind!=="file"||!expected.identity||expected.sha256!==contextSha256)throw Error("Exact predecessor Dependencies marker CAS required");const plan=directory.filePublicationPlan(expected);if(!plan.retainedName||plan.windows&&!plan.windows.prior)throw Error("Actual predecessor marker reservation required");const identity=(row:ArtifactReadFileIdentityV1)=>Object.freeze(Object.fromEntries((["dev","ino","mode","size","nlink","mtimeNs","ctimeNs"]as const).map(key=>[key,row[key].toString()])))as Readonly<Record<"dev"|"ino"|"mode"|"size"|"nlink"|"mtimeNs"|"ctimeNs",string>>;const baseline={version:1 as const,purpose:"plan-funded-system-metadata"as const,roleSha256:metadata.sha256,allocation:settled,contextSha256,backingStamp:currentFunding.backingStamp,expectedSequence:currentFunding.expectedSequence,parent:identity(directory.stat()),prior:{identity:identity(expected.identity),sha256:contextSha256},names,plan:{...plan,retainedName:plan.retainedName,windows:plan.windows?{parent:plan.windows.parent,prior:plan.windows.prior!}:null}};intentFile=await writeRepositoryDependencyMarkerBaselineFileV1(read.root,baseline,check);published=await directory.publishFile(expected,bytes,{prepared:async prepared=>{check();if(!intentFile||intentFile.intent)throw Error("One actual marker publication reserved event required");intentFile=await writeRepositoryDependencyMarkerIntentFileV1(read.root,{version:1,purpose:"publish-current-dependency-funding-marker",allocation:settled,contextSha256,backingStamp:currentFunding.backingStamp,expectedSequence:currentFunding.expectedSequence,prepared:artifactRelativeFilePublicationPreparedDataV1(prepared)},check);}});}
 if(!intentFile||published.sha256!==sha256)throw Error("Actual durable current Dependencies marker required");if(published.retained){const retained=await directory.entry(published.retained.name);if(!retained||retained.kind!=="file"||!same(retained,published.retained.identity)||published.retained.sha256!==contextSha256)throw Error("Exact issued retained predecessor marker required");await directory.remove(published.retained.name,retained);await directory.sync();}check();const marker=readNativeImageRetirementFileV1(join(read.root,"dependencies.json"),2048);if(createHash("sha256").update(marker.bytes).digest("hex")!==sha256)throw Error("Exact current Dependencies marker final readback required");await removeRepositoryDependencyMarkerIntentFileV1(read.root,intentFile,check);check();
 });});});checkResourceLeaseControl(actual);
});}
import{readRepositoryDependencyMarkerIntentFileV1,writeRepositoryDependencyMarkerBaselineFileV1,writeRepositoryDependencyMarkerIntentFileV1,removeRepositoryDependencyMarkerIntentFileV1,type RepositoryDependencyMarkerIntentFileV1}from"./📋️intent/📜️journal/🟦️.ts";
import{repositoryDependencyFundingMarkerBytesV1,requireRepositoryDependencyMarkerPublicationBindingV1}from"./📋️intent/🟦️.ts";
import{artifactRelativeFilePublicationPreparedDataV1,artifactRelativeFilePublicationStageNameV1,type ArtifactReadFileIdentityV1}from"../../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";

import{defineNativeFundingMetadataRoleV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
