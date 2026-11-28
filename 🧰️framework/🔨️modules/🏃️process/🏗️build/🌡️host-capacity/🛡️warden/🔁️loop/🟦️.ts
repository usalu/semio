import {execFile,spawn,spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {processPidExistsV1,readProcessStartV1} from "../../../../🔖️identity/🟦️.ts";
import {appendFileSync,closeSync,mkdirSync,openSync,readdirSync,readFileSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {currentHostProbeV1,observeTreeMegabytesV1,readHostSampleV1,readHostTextV1,readLinuxChargesV1,type HostSampleRecordV1,type HostTextReaderV1} from "../../📡️sampling/🟦️.ts";
import {systemDarwinFootprintPortV1,type DarwinFootprintPortV1} from "../../🍏️native-footprint/🟦️.ts";
import type {HostCapacityPolicyV1} from "../../📏️decision/🟦️.ts";
import {readHostProcessRowsV1} from "../../🧺️unmanaged/🟦️.ts";
import {buildScriptOutputDirectoryV1,type CompileBudgetPolicyV1} from "../../../../📦️artifacts/🏗️native-build/🎟️admission/🏔️ceiling/🟦️.ts";
import {HOST_WARDEN_PLATFORMS_V1,HOST_WARDEN_POLICY_V1,HOST_WARDEN_SWAP_V1,HOST_WARDEN_TABLES_V1,HOST_WARDEN_TABLE_V1,buildHostWardenAlarmV1,classifyWardenSubjectsV1,decideHostWardenV1,discoverWardenWorkspacesV1,wardenTreeV1,parseWardenSwapV1,formatHostWardenCancelFailureV1,formatHostWardenNoticeV1,applyWardenClaimsV1,formatHostWardenRequeueV1,hostWardenCancelRetryableV1,formatHostWardenStopReasonV1,isHostWardenV1,parseWardenProcessTableV1,planWardenSignalsV1,requireHostWardenV1,topWardenOthersV1,grantedWardenSubjectsV1,foreignWardenHoldersV1,type HostWardenGrantV1,type HostWardenAlarmV1,type HostWardenCancelV1,type HostWardenClaimV1,type HostWardenHostV1,type HostWardenMirrorV1,type HostWardenOtherV1,type HostWardenPolicyV1,type HostWardenPreviousV1,type HostWardenStateV1,type HostWardenRowV1,type HostWardenSubjectV1,type HostWardenTablesV1} from "../🟦️.ts";

const MEGABYTE=1048576;

/** 📡️ Everything the loop reaches the machine through, injectable: the host sample, the process table, the footprint of a process tree (what it holds including what the OS compressed or swapped out of it; absent: the resident set of the table is the measure), signals, the clock, the standard error line, the directory of the warden, the sleep and the mode. */
export type HostWardenPortsV1=Readonly<{sample:()=>Promise<HostWardenHostV1>;table:()=>Promise<readonly HostWardenRowV1[]>;signal:(pid:number,name:"SIGSTOP"|"SIGCONT"|"SIGKILL"|"SIGTERM")=>void;now:()=>number;footprint?:(pid:number)=>Promise<number|null>;write:(line:string)=>void;directory:string;sleep:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;mode:()=>"observe"|"enforce"}>;
/** 🎛️ What one run of the loop may be told besides its ports. */
export type HostWardenLoopOptionsV1=Readonly<{ports:HostWardenPortsV1;policy?:HostWardenPolicyV1;tables?:HostWardenTablesV1;workspaces?:readonly string[];within?:number;prefixes?:readonly string[];maxTicks?:number;signal?:AbortSignal;code?:string;platform?:NodeJS.Platform;sources?:()=>string|Promise<string>;ceilings?:CompileBudgetPolicyV1;grants?:()=>readonly HostWardenGrantV1[];handover?:number;successor?:()=>void}>;
/** 🏁️ How a run ended and what it did. */
export type HostWardenLoopResultV1=Readonly<{reason:"idle"|"code"|"signal"|"locked"|"limit";ticks:number;counters:Readonly<{suspended:number;resumed:number;cancelled:number}>}>;

const POLICY=HOST_WARDEN_POLICY_V1;
const OWN_WORKSPACE=fileURLToPath(import.meta.url).split("/🧰️framework/")[0]!;
const defaultSleep=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise(done=>{if(signal?.aborted)return done();const timer=setTimeout(done,milliseconds);signal?.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});});
const wardenLocale=(platform:NodeJS.Platform):Record<string,string>=>{const locale=(HOST_WARDEN_TABLE_V1.locale as Record<string,string>)[platform];return locale===undefined?{}:{LC_ALL:locale};};

/** ⏱️ Reads the start text of one process (`ps -o lstart= -p`); the empty text where the platform has no such command or the process is gone. */
export function wardenProcessStartV1(pid:number,platform:NodeJS.Platform=process.platform):string{
 if(platform==="win32"){const read=readProcessStartV1(pid,platform);return typeof read==="number"?String(read):"";}
 const table=HOST_WARDEN_TABLE_V1.start,done=spawnSync(table.command,[...table.arguments,String(pid)],{windowsHide:true,encoding:"utf8",env:{...process.env,...wardenLocale(platform)},timeout:POLICY.commandMilliseconds});
 return done.status===0?done.stdout.trim():"";
}

/** 🧬️ The digest of the sources of the warden (its pure module, its loop, its entry, its schema, and the compile-memory policy and law of the admission it enforces as runtime ceilings): a running warden whose digest differs from the code on disk ends itself, the next caller starts the new code. */
export function hostWardenCodeDigestV1():string{
 const hash=createHash("sha256");
 for(const path of["../🟦️.ts","🟦️.ts","../⌨️main/🟦️.ts","../🧬️schema/🔣️.json","../../📡️sampling/🟦️.ts","../../../../📦️artifacts/🏗️native-build/🎟️admission/🏔️ceiling/🟦️.ts","../../../../📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json"])hash.update(readFileSync(fileURLToPath(new URL(path,import.meta.url))));
 return hash.digest("hex").slice(0,16);
}

/** 🗂️ The directory of the warden below the machine-wide host capacity directory. */
export function hostWardenDirectoryV1(hostCapacityDirectory?:string):string{
 const base=hostCapacityDirectory??(createRequire(import.meta.url)("../../🎫️gate/📖️status/🟦️.ts") as typeof import("../../🎫️gate/📖️status/🟦️.ts")).hostCapacityDirectoryV1();
 return join(base,POLICY.directory);
}

const readJson=(path:string):unknown=>{try{return JSON.parse(readFileSync(path,"utf8"));}catch{return undefined;}};
const writeAtomic=(path:string,text:string):void=>{const temporary=`${path}.${process.pid}.tmp`;writeFileSync(temporary,text);renameSync(temporary,path);};
const alive=(pid:number):boolean=>processPidExistsV1(pid);

/** 📖️ Reads the state the running warden wrote (validated); null when absent, invalid or older than `staleMilliseconds`. */
export function readHostWardenStateV1(directory?:string,nowMs:number=Date.now()):HostWardenStateV1|null{
 const value=readJson(join(hostWardenDirectoryV1(directory),POLICY.stateFile));
 if(value===undefined||!isHostWardenV1("State",value))return null;
 const state=value as HostWardenStateV1;
 return nowMs-state.atMs>POLICY.staleMilliseconds?null:state;
}

/** 🎟️ Reads the live grants of the capacity gate (`grants/<id>.json` beside the warden directory; the `.attached`, `.phases` and `.upgrade` files of a grant are no grants): the id, the pid that holds it, the megabytes reserved and the start of that process. A file that is not a grant is skipped. */
export function readHostWardenGrantsV1(wardenDirectory:string):readonly HostWardenGrantV1[]{
 const folder=join(dirname(wardenDirectory),"grants"),found:HostWardenGrantV1[]=[];
 try{for(const name of readdirSync(folder)){
  if(!name.endsWith(".json"))continue;
  const value=readJson(join(folder,name)) as {id?:unknown;pid?:unknown;reservedMegabytes?:unknown;startMs?:unknown}|undefined;
  if(value!==undefined&&value!==null&&typeof value.id==="string"&&Number.isInteger(value.pid)&&(value.pid as number)>1&&typeof value.reservedMegabytes==="number"&&value.reservedMegabytes>=0)found.push({id:value.id,pid:value.pid as number,reservedMegabytes:Math.floor(value.reservedMegabytes),...typeof value.startMs==="number"?{startMs:value.startMs}:{}});
 }}catch{}
 return found;
}

/** 🌐️ What the fresh state of the warden says every process outside the subjects holds (0 when no warden writes a fresh state): the memory admission counts against the floor besides its grants. */
export function foreignHostMegabytesV1(directory?:string,nowMs:number=Date.now()):number{
 return readHostWardenStateV1(directory,nowMs)?.foreign?.megabytes??0;
}

const listCancels=(directory:string|undefined,nowMs:number):HostWardenCancelV1[]=>{
 const folder=join(hostWardenDirectoryV1(directory),POLICY.cancelledDirectory),found:HostWardenCancelV1[]=[];
 try{for(const name of readdirSync(folder)){
  if(!name.endsWith(".json"))continue;
  const value=readJson(join(folder,name));
  if(isHostWardenV1("Cancel",value)&&nowMs-(value as HostWardenCancelV1).atMs<=POLICY.cancelRecordMaximumAgeMilliseconds)found.push(value as HostWardenCancelV1);
 }}catch{}
 return found;
};

/** 🛟️ Writes the claim of an owner on one of its processes (schema `Claim`): the memory it holds for it at the capacity gate and whether the owner itself stopped it while it waits for more. The warden shelters a process within its grant and does not treat a process the owner stopped as suspended by it; the owner calls this again when the grant or the stop changes and {@link releaseHostWardenClaimV1} when the process ends. */
export function claimHostWardenSubjectV1(pid:number,claim:Readonly<{grantMegabytes?:number;stopped:boolean;owner?:string}>,directory?:string,nowMs:number=Date.now()):void{
 const folder=join(hostWardenDirectoryV1(directory),POLICY.claimsDirectory);
 mkdirSync(folder,{recursive:true});
 writeAtomic(join(folder,`${pid}.json`),JSON.stringify({version:1,pid,atMs:nowMs,stopped:claim.stopped,...claim.grantMegabytes===undefined?{}:{grantMegabytes:Math.floor(claim.grantMegabytes)},...claim.owner===undefined?{}:{owner:claim.owner}}));
}

/** 🛟️ Removes the claim of a process that ended. */
export function releaseHostWardenClaimV1(pid:number,directory?:string):void{
 rmSync(join(hostWardenDirectoryV1(directory),POLICY.claimsDirectory,`${pid}.json`),{force:true});
}

/** 🛟️ Reads the claims of the owners by pid (validated, not older than `claimMaximumMilliseconds`). */
export function readHostWardenClaimsV1(directory?:string,nowMs:number=Date.now()):ReadonlyMap<number,HostWardenClaimV1>{
 return readClaimsOfV1(hostWardenDirectoryV1(directory),nowMs);
}

const readClaimsOfV1=(wardenDirectory:string,nowMs:number):ReadonlyMap<number,HostWardenClaimV1>=>{
 const folder=join(wardenDirectory,POLICY.claimsDirectory),found=new Map<number,HostWardenClaimV1>();
 try{for(const name of readdirSync(folder)){
  const value=readJson(join(folder,name));
  if(isHostWardenV1("Claim",value)&&nowMs-(value as HostWardenClaimV1).atMs<=POLICY.claimMaximumMilliseconds)found.set((value as HostWardenClaimV1).pid,value as HostWardenClaimV1);
 }}catch{}
 return found;
};

/** 🚫️ Reads the cancel record of a pid when the warden cancelled it less than `cancelRecordMaximumAgeMilliseconds` ago, so its owner can requeue instead of failing; null otherwise. */
export function readHostWardenCancelV1(pid:number,directory?:string,nowMs:number=Date.now()):HostWardenCancelV1|null{
 let best:HostWardenCancelV1|null=null;
 for(const record of listCancels(directory,nowMs))if(record.pid===pid&&(best===null||record.atMs>best.atMs))best=record;
 return best;
}

/** 🚫️ Reads every cancel record written at or after `sinceMs` (newest first): the warden cancels the leaf (`rustc`), not the root of a task, so an owner matches the records of its task by the `command` they carry. */
export function readHostWardenCancelsSinceV1(sinceMs:number,directory?:string,nowMs:number=Date.now()):readonly HostWardenCancelV1[]{
 return listCancels(directory,nowMs).filter(record=>record.atMs>=sinceMs).sort((a,b)=>b.atMs-a.atMs);
}

/** 🧯️ Reads every cancel record written at or after `sinceMs` for a process below `ancestor` (newest first): the owner of a build tells the cancels that hit its own process tree by its pid, whatever the command lines say. */
export function readHostWardenCancelsBelowV1(ancestor:number,sinceMs:number,directory?:string,nowMs:number=Date.now()):readonly HostWardenCancelV1[]{
 return readHostWardenCancelsSinceV1(sinceMs,directory,nowMs).filter(record=>record.ancestors.includes(ancestor));
}

/** 🧯️ The typed failure of a build whose processes the warden cancelled (`code` `host-warden-cancelled`): its message is the English and the German text of `formatHostWardenCancelFailureV1`, `cancels` the records and `retries` the times it was already queued again (see {@link retryAfterHostWardenCancelV1}). */
export class HostWardenCancelledErrorV1 extends Error{
 readonly code="host-warden-cancelled" as const;
 readonly cancels:readonly HostWardenCancelV1[];
 readonly retries:number;
 constructor(cancels:readonly HostWardenCancelV1[],retries:number=0){super(["en","de"].map(language=>formatHostWardenCancelFailureV1(language as "en"|"de",cancels,retries)).join("\n"));this.name="HostWardenCancelledErrorV1";this.cancels=cancels;this.retries=retries;}
}

/** 🧯️ Throws {@link HostWardenCancelledErrorV1} when the warden cancelled a process below `ancestor` since `sinceMs`; returns otherwise. */
export function requireNoHostWardenCancelBelowV1(ancestor:number,sinceMs:number,directory?:string,nowMs:number=Date.now()):void{
 const cancels=readHostWardenCancelsBelowV1(ancestor,sinceMs,directory,nowMs);
 if(cancels.length>0)throw new HostWardenCancelledErrorV1(cancels);
}

/** 🔁️ What {@link retryAfterHostWardenCancelV1} needs besides the work: the warden directory, the report line sink, the cancel signal and the clock and sleep (tests replace them). */
export type HostWardenRetryOptionsV1=Readonly<{directory?:string;report:(line:string)=>void;signal?:AbortSignal;policy?:HostWardenPolicyV1;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>}>;

const sleepAbortable=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const abort=():void=>{clearTimeout(timer);reject(signal!.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",abort);resolve();},milliseconds);
 signal?.addEventListener("abort",abort,{once:true});
});

/** 🔁️ Runs `run` and, when the warden cancelled a process of the run for the memory the host lacked ({@link hostWardenCancelRetryableV1}), queues it again instead of failing: the run reports the bilingual line, waits for a calm host (the state of the warden is polled every `requeuePollMilliseconds`, progress is reported each poll and the wait is cancellable through `signal`, at most `requeueCalmMaximumSeconds`; an absent warden counts as calm) and calls `run` again, which admits itself through the capacity gate again. After `requeueMaximumAttempts` attempts, for a cancel of a cap or a ceiling and for every other error, the failure is thrown (the typed failure carries the number of retries). `run` receives the attempt number and the time that attempt started, the `sinceMs` of {@link requireNoHostWardenCancelBelowV1}.
 * @see ./../🧬️schema/🔣️.json */
export async function retryAfterHostWardenCancelV1<T>(run:(attempt:number,startedMs:number)=>Promise<T>,options:HostWardenRetryOptionsV1):Promise<T>{
 const policy=options.policy??POLICY,now=options.now??Date.now,sleep=options.sleep??sleepAbortable,maximum=policy.requeueMaximumAttempts;
 for(let attempt=1;;attempt++){
  const startedMs=now();
  try{return await run(attempt,startedMs);}
  catch(error){
   if(!(error instanceof HostWardenCancelledErrorV1))throw error;
   if(attempt>=maximum||!hostWardenCancelRetryableV1(error.cancels))throw attempt>1?new HostWardenCancelledErrorV1(error.cancels,attempt-1):error;
   const kinds=[...new Set(error.cancels.map(record=>record.kind))].join(", ");
   options.report(formatHostWardenRequeueV1("wait",{count:error.cancels.length,kinds,attempt:attempt+1,maximum}));
   const limit=policy.requeueCalmMaximumSeconds,waitStart=now();
   for(;;){
    if(options.signal?.aborted)throw options.signal.reason;
    const state=readHostWardenStateV1(options.directory,now());
    if(state===null||state.level==="calm"&&!state.hold)break;
    const seconds=Math.floor((now()-waitStart)/1000);
    if(seconds>=limit)break;
    options.report(formatHostWardenRequeueV1("waiting",{seconds,limit}));
    await sleep(policy.requeuePollMilliseconds,options.signal);
   }
   options.report(formatHostWardenRequeueV1("admit",{attempt:attempt+1,maximum}));
  }
 }
}

/** 🪞️ The record a worker mirrors to the group store: the fresh state of the warden as written, else `{"version":1,"atMs":<now>,"absent":true}`, so the coordinator tells no warden from no record. */
export function currentHostWardenMirrorV1(directory?:string,nowMs:number=Date.now()):HostWardenMirrorV1{
 return readHostWardenStateV1(directory,nowMs)??{version:1,atMs:nowMs,absent:true};
}

/** 📖️ Reads a mirrored `memory.json` (validated against schema `Mirror`); null when absent or invalid. */
export function readHostWardenMirrorV1(path:string):HostWardenMirrorV1|null{
 const value=readJson(path);
 return value!==undefined&&isHostWardenV1("Mirror",value)?value as HostWardenMirrorV1:null;
}

/** 🏷️ Replaces the resident set a `ps` row carries by what the kernel charges the process (schema `ProcessRow`): on macOS the `phys_footprint` of every readable process through the native port (about two microseconds per process, a process it cannot read keeps its resident set), on Linux the proportional plus swapped proportional set size of the processes whose resident set reaches `chargeMinimumKilobytes` (the kernel walks the page tables per read, and below the minimum the difference is smaller than the noise of the cap), anywhere else the rows as they are. */
export async function chargeWardenRowsV1(platform:NodeJS.Platform,rows:readonly HostWardenRowV1[],ports:Readonly<{darwin?:DarwinFootprintPortV1|null;read?:HostTextReaderV1;minimumKilobytes?:number}>={}):Promise<readonly HostWardenRowV1[]>{
 if(platform==="darwin"){
  const port=ports.darwin===undefined?systemDarwinFootprintPortV1(platform):ports.darwin;
  if(port===null)return rows;
  return rows.map(row=>{
   const bytes=port.footprint(row.pid);
   if(bytes===null)return row;
   const memoryKilobytes=Math.floor(bytes/1024);
   return {...row,memoryKilobytes,nonresidentKilobytes:Math.max(0,memoryKilobytes-row.memoryKilobytes)};
  });
 }
 if(platform==="linux"){
  const minimum=ports.minimumKilobytes??HOST_WARDEN_TABLE_V1.chargeMinimumKilobytes,charged=await readLinuxChargesV1(rows.filter(row=>row.stopped||row.memoryKilobytes>=minimum).map(row=>row.pid),ports.read??readHostTextV1);
  return rows.map(row=>{
   const charge=charged.get(row.pid);
   if(charge===undefined)return row;
   return {...row,memoryKilobytes:charge.resident+(charge.swapped??0),...charge.swapped===undefined?{}:{nonresidentKilobytes:charge.swapped}};
  });
 }
 return rows;
}

/** 🏭️ The default ports: the real machine. The host sample reads the counters through the authored host capacity policy (no `hdiutil`, no `footprint` tool: the loop must stay cheap), the table is one `ps` whose resident sets are replaced by what the kernel charges (`chargeWardenRowsV1`; the native process walk on Windows, which has no state, start or command line, carries the private usage), signals go through `process.kill`, the mode is the content of the `mode` file or forced by `observe`. */
export function defaultHostWardenPortsV1(options:Readonly<{hostPolicy:HostCapacityPolicyV1;directory?:string;observe?:boolean;platform?:NodeJS.Platform}>):HostWardenPortsV1{
 const platform=options.platform??process.platform,directory=hostWardenDirectoryV1(options.directory);
 let previous:Readonly<{atMs:number;swappedOutMegabytes:number|null}>|undefined,swapRead:Readonly<{atMs:number;facts:Pick<HostWardenHostV1,"swapFreeMegabytes"|"swapVolumeFreeMegabytes"|"swapFileMegabytes">}>|undefined;const commands=new Map<string,string>();
 const probe=currentHostProbeV1(),quick={...probe,run:(command:string,args:readonly string[],signal?:AbortSignal)=>command.endsWith("hdiutil")||args.includes("kern.num_files")?Promise.reject(Error("skipped")):probe.run(command,args,signal)};
 return Object.freeze({
  directory,now:Date.now,sleep:defaultSleep,footprint:pid=>observeTreeMegabytesV1(pid),write:(line:string)=>{process.stderr.write(line+"\n");},
  mode:()=>options.observe?"observe" as const:(()=>{try{const text=readFileSync(join(directory,POLICY.modeFile),"utf8").trim();if(text==="observe"||text==="enforce")return text;}catch{}return POLICY.defaultMode as "observe"|"enforce";})(),
  sample:async()=>{
   const record:HostSampleRecordV1=await readHostSampleV1(options.hostPolicy,quick,previous),interval=previous===undefined?0:Math.round(record.host.atMs-previous.atMs);
   previous={atMs:record.host.atMs,swappedOutMegabytes:record.swappedOutMegabytes};
   const host=record.host as typeof record.host&{compressedMegabytes?:number|null};
   const now=Date.now();
   if(swapRead===undefined||now-swapRead.atMs>=HOST_WARDEN_SWAP_V1.intervalMilliseconds||now<swapRead.atMs)swapRead={atMs:now,facts:await readWardenSwapV1(platform,probe)};
   return{totalMegabytes:host.totalMegabytes,availableMegabytes:host.availableMegabytes,swapUsedMegabytes:host.swapUsedMegabytes,swapOutMegabytesPerSecond:host.swapOutMegabytesPerSecond,compressedMegabytes:host.compressedMegabytes??null,pressure:host.pressure,...swapRead.facts,...interval>0?{intervalMilliseconds:interval}:{}};
  },
  table:async()=>{
   if(platform==="win32")return(await readHostProcessRowsV1(platform)).map(row=>({pid:row.pid,parent:row.ppid,group:row.pid,memoryKilobytes:row.kilobytes,stopped:false,elapsedSeconds:0,start:"",command:row.name}));
   const table=HOST_WARDEN_TABLE_V1,rows=parseWardenProcessTableV1(await runPs(table.quickArguments,platform)),key=(row:HostWardenRowV1)=>`${row.pid}.${row.start}`;
   const young=new Map<string,string>(),missing=rows.filter(row=>!commands.has(key(row)));
   if(missing.length>0){
    const found=new Map<number,string>();
    for(const line of(await runPs(missing.length>=table.commandAllMinimum?["-A",...table.commandArguments]:[...table.commandArguments,"-p",missing.map(row=>row.pid).join(",")],platform,true)).split("\n")){const match=/^\s*(\d+)\s(.*)$/u.exec(line);if(match)found.set(Number(match[1]),match[2]!);}
    for(const row of missing){const command=found.get(row.pid)??"";if(row.elapsedSeconds>=table.commandSettledSeconds&&!command.startsWith("("))commands.set(key(row),command);else young.set(key(row),command);}
   }
   const live=new Set(rows.map(key));for(const known of[...commands.keys()])if(!live.has(known))commands.delete(known);
   return chargeWardenRowsV1(platform,rows.map(row=>({...row,command:commands.get(key(row))??young.get(key(row))??""})));
  },
  signal:(pid:number,name:"SIGSTOP"|"SIGCONT"|"SIGKILL"|"SIGTERM")=>{try{process.kill(pid,name);}catch{}},
 });
}

/** 💱️ Reads the free swap and the free space of the swap volume of this machine through the commands of schema `Swap`; a command that fails leaves its number null. */
export async function readWardenSwapV1(platform:NodeJS.Platform,probe:Readonly<{run:(command:string,args:readonly string[])=>Promise<string>;read:(path:string)=>Promise<string|null>}>):Promise<Pick<HostWardenHostV1,"swapFreeMegabytes"|"swapVolumeFreeMegabytes"|"swapFileMegabytes">>{
 const swap=HOST_WARDEN_SWAP_V1;
 if(platform==="darwin"){
  const usage=await probe.run(swap.darwin.usage[0]!,swap.darwin.usage.slice(1)).catch(()=>null);
  const fileSize=await probe.run(swap.darwin.fileSize[0]!,swap.darwin.fileSize.slice(1)).catch(()=>null);
  let disk:string|null=null;
  for(const volume of swap.darwin.volumes){disk=await probe.run(swap.darwin.disk[0]!,[...swap.darwin.disk.slice(1),volume]).catch(()=>null);if(disk!==null)break;}
  return parseWardenSwapV1(platform,{usage,disk,fileSize});
 }
 if(platform==="linux")return parseWardenSwapV1(platform,{meminfo:await probe.read(swap.linux.meminfo).catch(()=>null)});
 return parseWardenSwapV1(platform,{});
}
const runPs=(args:readonly string[],platform:NodeJS.Platform,vanished=false):Promise<string>=>new Promise((done,fail)=>execFile(HOST_WARDEN_TABLE_V1.command,[...args],{windowsHide:true,maxBuffer:64*MEGABYTE,encoding:"utf8",timeout:POLICY.commandMilliseconds,env:{...process.env,...wardenLocale(platform)}},(error,stdout)=>error&&stdout.length===0&&!vanished?fail(error):done(stdout)));
const percentile=(values:readonly number[],share:number):number=>{if(!values.length)return 0;const sorted=[...values].sort((a,b)=>a-b);return sorted[Math.min(sorted.length-1,Math.floor(sorted.length*share))]!;};
const round=(value:number,digits=2):number=>Math.round(value*10**digits)/10**digits;

type Record1=Readonly<{id:string;pid:number;start:string;kind:string;sinceMs:number}>;

/** 🔁️ Runs the host warden until it has no subject for `idleMilliseconds`, its sources changed, the signal aborts, another warden holds the lock or `maxTicks` ticks ran. Every tick samples the host and the process table, classifies the subjects, decides, and for each action writes its record first (`state.json` with the suspended set; the `cancelled/<pid>.<start digest>.json` record for a cancel), then signals (suspend: SIGSTOP to the tree, parent first; resume: SIGCONT; cancel: SIGCONT, then the signal of the kind), then appends the alarm to `alarms.jsonl` and writes it to the standard error in English and German. A resume writes the state after its signal, so a crash never loses a stopped process. In observe mode, and on a platform that cannot suspend, nothing is signalled and alarms are marked observed. A starting warden resumes every process a dead predecessor left suspended, and every exit resumes what this run suspended.
 * @see ../🧬️schema/🔣️.json */
export async function runHostWardenLoopV1(options:HostWardenLoopOptionsV1):Promise<HostWardenLoopResultV1>{
 const ports=options.ports,policy=options.policy??POLICY,tables=options.tables??HOST_WARDEN_TABLES_V1,platform=options.platform??process.platform,directory=ports.directory;

 const workspaces=options.workspaces??[OWN_WORKSPACE],stateFile=join(directory,policy.stateFile),lockFile=join(directory,policy.lockFile),alarmsFile=join(directory,policy.alarmsFile),cancelled=join(directory,policy.cancelledDirectory);
 mkdirSync(cancelled,{recursive:true});
 const code=options.code??hostWardenCodeDigestV1(),observeOnly=(HOST_WARDEN_PLATFORMS_V1.observeOnly as readonly string[]).includes(platform),selfStart=wardenProcessStartV1(process.pid,platform);
 const done=(reason:HostWardenLoopResultV1["reason"],ticks:number,counters:HostWardenLoopResultV1["counters"]):HostWardenLoopResultV1=>{
  for(const language of["en","de"] as const)ports.write(formatHostWardenNoticeV1(language,"stopped",{reason:formatHostWardenStopReasonV1(language,reason,{minutes:Math.round(policy.idleMilliseconds/60000)})}));
  return{reason,ticks,counters};
 };
 const tryLock=():number|undefined=>{
  for(let attempt=0;attempt<3;attempt++){
   try{writeFileSync(lockFile,JSON.stringify({pid:process.pid,start:selfStart}),{flag:"wx"});return undefined;}catch{}
   const held=readJson(lockFile);
   if(isHostWardenV1("Lock",held)){const lock=held as {pid:number;start:string};if(alive(lock.pid)&&(lock.start===""||wardenProcessStartV1(lock.pid,platform)===lock.start))return lock.pid;}
   rmSync(lockFile,{force:true});
  }
  return 0;
 };
 const lockDeadline=ports.now()+(options.handover===undefined?0:policy.handoverWaitMilliseconds);
 let locked=tryLock();
 while(locked!==undefined&&options.handover!==undefined&&ports.now()<lockDeadline&&!options.signal?.aborted){await ports.sleep(100,options.signal);locked=tryLock();}
 if(locked!==undefined){for(const language of["en","de"] as const)ports.write(formatHostWardenNoticeV1(language,"locked",{pid:locked}));return done("locked",0,{suspended:0,resumed:0,cancelled:0});}
 const reported=new Set<string>(),counters={suspended:0,resumed:0,cancelled:0,ticks:0},suspended=new Map<string,Record1>(),since=new Map<string,number>(),lastAlarms:HostWardenAlarmV1[]=[],durations:number[]=[],announced=new Set<string>(),marks:{wall:number;cpu:number}[]=[{wall:performance.now(),cpu:(process.cpuUsage().user+process.cpuUsage().system)/1000}];
 let lastResumeMs:number|undefined,previous:HostWardenPreviousV1={level:"calm",calmTicks:0},nearCeiling=false,level:"calm"|"guard"|"danger"="calm",lastSubjectAt=ports.now(),lastCodeCheck=ports.now(),stopReason:HostWardenLoopResultV1["reason"]|undefined;
 const mode=():"observe"|"enforce"=>observeOnly?"observe":ports.mode();
 const exclusion=(rows:readonly HostWardenRowV1[]):Set<number>=>{const byPid=new Map(rows.map(row=>[row.pid,row] as const)),found=new Set<number>([process.pid]);for(let row=byPid.get(process.pid);row!==undefined&&!found.has(row.parent)&&row.parent>0;row=byPid.get(row.parent))found.add(row.parent);return found;};
 const signalTree=(rows:readonly HostWardenRowV1[],subject:Readonly<{pid:number;scope?:"process"|"group"|"tree"}>,names:readonly("SIGSTOP"|"SIGCONT"|"SIGKILL"|"SIGTERM")[]):void=>{
  const pids=planWardenSignalsV1(rows,subject,tables,exclusion(rows));
  for(const name of names)for(const pid of pids)ports.signal(pid,name);
 };
 const appendAlarm=(alarm:HostWardenAlarmV1):void=>{
  try{
   appendFileSync(alarmsFile,JSON.stringify(alarm)+"\n");
   if(statSync(alarmsFile).size>policy.alarmsMaximumBytes){const lines=readFileSync(alarmsFile,"utf8").split("\n").filter(Boolean),kept=lines.slice(Math.floor(lines.length/2));writeAtomic(alarmsFile,kept.join("\n")+"\n");}
  }catch{}
  lastAlarms.push(alarm);if(lastAlarms.length>policy.lastAlarms)lastAlarms.splice(0,lastAlarms.length-policy.lastAlarms);
  ports.write(alarm.text.en);ports.write(alarm.text.de);
 };
 let others:readonly HostWardenOtherV1[]=[],foreign:Readonly<{megabytes:number;processes:number;heavy:number}>={megabytes:0,processes:0,heavy:0},booked:Readonly<{count:number;megabytes:number}>={count:0,megabytes:0};
 const repoCpu:{known:boolean;total:number;at?:number;seen:ReadonlyMap<string,number>}={known:false,total:0,seen:new Map()};
 let lastTree:ReturnType<typeof wardenTreeV1>=[];
 let last:{host:HostWardenHostV1;decision:ReturnType<typeof decideHostWardenV1>;subjects:readonly HostWardenSubjectV1[]}|undefined;
 const accountCpu=(owned:ReadonlyMap<string,Readonly<{cpuSeconds:number;elapsedSeconds:number}>>,now:number):void=>{
  if(owned.size>0)repoCpu.known=true;
  const span=repoCpu.at===undefined?0:(now-repoCpu.at)/1000+policy.cpuYoungSlackSeconds;
  for(const [key,row] of owned){
   const before=repoCpu.seen.get(key);
   if(before!==undefined)repoCpu.total+=Math.max(0,row.cpuSeconds-before);
   else if(repoCpu.at!==undefined&&row.elapsedSeconds<=span)repoCpu.total+=row.cpuSeconds;
  }
  repoCpu.seen=new Map([...owned].map(([key,row])=>[key,row.cpuSeconds]));repoCpu.at=now;
 };
 const writeState=(host:HostWardenHostV1,decision:Pick<ReturnType<typeof decideHostWardenV1>,"trigger"|"guardMegabytes"|"dangerMegabytes"|"resumeMegabytes"|"hold"|"swapRoomMegabytes"|"headroomMegabytes"|"headroomGuardMegabytes"|"headroomDangerMegabytes">,subjects:readonly HostWardenSubjectV1[]):void=>{
  const byKind:Record<string,{count:number;megabytes:number}>={};
  const own=subjects.filter(subject=>subject.kind!=="granted");
  for(const subject of own){const row=byKind[subject.kind]??={count:0,megabytes:0};row.count++;row.megabytes+=subject.megabytes;}
  const wall=marks.length>1?marks.at(-1)!.wall-marks[0]!.wall:0,cpu=marks.length>1?marks.at(-1)!.cpu-marks[0]!.cpu:0;
  const state={version:1,atMs:ports.now(),pid:process.pid,mode:mode(),code,host,level,hold:decision.hold,trigger:decision.trigger,others,foreign,grants:booked,guardMegabytes:decision.guardMegabytes,dangerMegabytes:decision.dangerMegabytes,resumeMegabytes:decision.resumeMegabytes,swapRoomMegabytes:decision.swapRoomMegabytes,headroomMegabytes:decision.headroomMegabytes,headroomGuardMegabytes:decision.headroomGuardMegabytes,headroomDangerMegabytes:decision.headroomDangerMegabytes,subjects:{count:own.length,megabytes:own.reduce((sum,subject)=>sum+subject.megabytes,0),...repoCpu.known?{cpuSeconds:Math.round(repoCpu.total*100)/100}:{},byKind},suspended:[...suspended.values()],counters:{suspended:counters.suspended,resumed:counters.resumed,cancelled:counters.cancelled,ticks:counters.ticks},lastAlarms:[...lastAlarms],...lastTree.length>0?{tree:lastTree}:{},overhead:{tickMillisecondsP50:round(percentile(durations,0.5)),tickMillisecondsP95:round(percentile(durations,0.95)),cpuBasisPoints:wall>0?round(cpu/wall*10000):0,residentMegabytes:round(process.memoryUsage().rss/MEGABYTE)}};
  if(!isHostWardenV1("State",state)){ports.write(`[warden] state invalid / Zustand ungültig: ${JSON.stringify(state).slice(0,200)}`);return;}
  try{writeAtomic(stateFile,JSON.stringify(state));}catch{}
 };
 const flaps=new Map<string,{suspends:number[];resumedAtMs?:number}>();
 const withHistory=(list:readonly HostWardenSubjectV1[],now:number):readonly HostWardenSubjectV1[]=>{
  const live=new Set(list.map(subject=>subject.id));for(const id of[...flaps.keys()])if(!live.has(id))flaps.delete(id);
  return list.map(subject=>{
   const known=flaps.get(subject.id);if(known===undefined)return subject;
   known.suspends=known.suspends.filter(at=>now-at<policy.flapWindowSeconds*1000);
   const resumed=known.resumedAtMs===undefined?undefined:Math.max(0,Math.floor((now-known.resumedAtMs)/1000));
   return{...subject,...known.suspends.length>0?{suspensions:known.suspends.length}:{},...resumed!==undefined&&resumed<policy.flapWindowSeconds?{resumedSeconds:resumed}:{}};
  });
 };
 const footprints=new Map<string,Readonly<{atMs:number;megabytes:number}>>();
 const measure=async(subjects:readonly HostWardenSubjectV1[],now:number):Promise<readonly HostWardenSubjectV1[]>=>{
  const probe=ports.footprint;
  if(probe===undefined)return subjects;
  const due=subjects.filter(subject=>subject.capMegabytes!==undefined&&subject.canCancel).sort((left,right)=>right.megabytes-left.megabytes||left.pid-right.pid).slice(0,policy.capProbeMaximum);
  const live=new Set(due.map(subject=>subject.id));
  for(const id of[...footprints.keys()])if(!live.has(id))footprints.delete(id);
  await Promise.all(due.map(async subject=>{
   const known=footprints.get(subject.id);
   const near=subject.ceiling!==undefined&&Math.max(subject.megabytes,known?.megabytes??0)*10000>=subject.ceiling.megabytes*policy.ceilingNearBasisPoints;
   if(known!==undefined&&!near&&now>=known.atMs&&now-known.atMs<policy.capMilliseconds)return;
   const megabytes=await probe(subject.pid).catch(()=>null);
   if(megabytes!==null)footprints.set(subject.id,{atMs:now,megabytes});
  }));
  return subjects.map(subject=>{const known=footprints.get(subject.id);return known!==undefined&&known.megabytes>subject.megabytes?{...subject,megabytes:known.megabytes}:subject;});
 };
 const ancestorsOf=(rows:readonly HostWardenRowV1[],pid:number):number[]=>{const byPid=new Map(rows.map(row=>[row.pid,row] as const)),found:number[]=[];for(let at=byPid.get(byPid.get(pid)?.parent??0);at!==undefined&&at.pid>0&&found.length<64&&!found.includes(at.pid);at=byPid.get(at.parent))found.push(at.pid);return found;};
 const resumeAll=async(rows:readonly HostWardenRowV1[]):Promise<void>=>{
  for(const entry of[...suspended.values()]){
   const row=rows.find(candidate=>candidate.pid===entry.pid&&candidate.start===entry.start);
   if(row!==undefined)signalTree(rows,{pid:entry.pid,scope:tables.kinds.find(kind=>kind.name===entry.kind)?.scope??"tree"},["SIGCONT"]);
   suspended.delete(entry.id);
  }
 };
 const predecessor=readJson(stateFile);
 if(isHostWardenV1("State",predecessor)&&mode()==="enforce"&&!observeOnly){
  const left=(predecessor as {suspended:readonly Record1[]}).suspended;
  if(left.length>0){
   const rows=await ports.table().catch(()=>[] as readonly HostWardenRowV1[]);
   for(const entry of left){
    const row=rows.find(candidate=>candidate.pid===entry.pid&&candidate.start===entry.start);
    if(row===undefined)continue;
    signalTree(rows,{pid:entry.pid,scope:tables.kinds.find(kind=>kind.name===entry.kind)?.scope??"tree"},["SIGCONT"]);
    for(const language of["en","de"] as const)ports.write(formatHostWardenNoticeV1(language,"left",{kind:entry.kind,pid:entry.pid}));
   }
  }
 }
 if(observeOnly)for(const language of["en","de"] as const)ports.write(formatHostWardenNoticeV1(language,"observeOnly",{platform}));
 for(const language of["en","de"] as const)ports.write(formatHostWardenNoticeV1(language,"started",{pid:process.pid,mode:mode(),code,directory}));
 const protectList=():readonly string[]=>{const value=readJson(join(directory,policy.protectFile));return isHostWardenV1("Protect",value)?value as string[]:[];};
 try{
  for(;;){
   if(options.signal?.aborted){stopReason="signal";break;}
   if(options.maxTicks!==undefined&&counters.ticks>=options.maxTicks){stopReason="limit";break;}
   const began=performance.now();
   try{
    const[host,rows]=await Promise.all([ports.sample(),ports.table()]),now=ports.now();
    const owned=new Map<string,Readonly<{cpuSeconds:number;elapsedSeconds:number}>>(),protect=protectList(),found=classifyWardenSubjectsV1(rows,tables,{owned,protect,workspaces:discoverWardenWorkspacesV1(rows,workspaces),...options.within===undefined?{}:{within:options.within},prefixes:options.prefixes??[],nowMs:now,suspendedSince:since,...options.ceilings===undefined?{}:{ceilings:options.ceilings}});
    lastTree=wardenTreeV1(rows,discoverWardenWorkspacesV1(rows,workspaces),12,found.map(subject=>subject.pid));
    const stopped=new Map(rows.filter(row=>row.stopped).map(row=>[row.pid,row]));
    const excluded=stopped.size>0?exclusion(rows):undefined;
    const live=new Set(rows.map(row=>row.pid)),listed=(options.grants??(()=>readHostWardenGrantsV1(directory)))().filter(grant=>live.has(grant.pid)),claimed=grantedWardenSubjectsV1(rows,applyWardenClaimsV1(found,readClaimsOfV1(directory,now),now,policy),listed,now,undefined,since),subjects=withHistory(await measure(claimed,now),now).map(subject=>{
     if(!subject.suspended||excluded===undefined)return subject;
     const members=planWardenSignalsV1(rows,{pid:subject.pid,scope:subject.scope??"tree"},tables,excluded).flatMap(pid=>stopped.get(pid)??[]);
     if(members.some(row=>row.nonresidentKilobytes===undefined)){
      const footprint=members.reduce((sum,row)=>sum+row.memoryKilobytes,0);
      return {...subject,nonresidentMegabytes:null,stoppedFootprintMegabytes:Math.ceil(footprint/1024)};
     }
     const kilobytes=members.reduce((sum,row)=>sum+row.nonresidentKilobytes!,0);
     return {...subject,nonresidentMegabytes:Math.ceil(kilobytes/1024)};
    });
    accountCpu(owned,now);
    const ids=new Set(subjects.map(subject=>subject.id));
    nearCeiling=subjects.some(subject=>subject.ceiling!==undefined&&subject.megabytes*10000>=subject.ceiling.megabytes*policy.ceilingNearBasisPoints);
    for(const subject of subjects){if(subject.suspended){if(!since.has(subject.id))since.set(subject.id,suspended.get(subject.id)?.sinceMs??now);}else since.delete(subject.id);}
    for(const id of[...since.keys()])if(!ids.has(id))since.delete(id);
    let dirty=false;
    for(const id of[...suspended.keys()]){const subject=subjects.find(candidate=>candidate.id===id);if(subject===undefined||!subject.suspended){suspended.delete(id);dirty=true;}}
    booked={count:listed.length,megabytes:listed.reduce((sum,grant)=>sum+grant.reservedMegabytes,0)};foreign=foreignWardenHoldersV1(rows,subjects,policy.foreignHeavyMegabytes,exclusion(rows));
    if(subjects.length>0||listed.length>0||foreign.heavy>0)lastSubjectAt=now;
    const decision=decideHostWardenV1(policy,host,subjects,lastResumeMs===undefined?previous:{...previous,resumedSeconds:Math.max(0,Math.floor((now-lastResumeMs)/1000))}),effective=mode(),byId=new Map(subjects.map(subject=>[subject.id,subject] as const)),keys=new Set(decision.actions.map(action=>`${action.action}.${action.id}`));
    level=decision.level;others=topWardenOthersV1(rows,subjects,policy.othersMaximum,options.prefixes??[]);previous={level:decision.level,calmTicks:decision.calmTicks,swapMilliseconds:decision.swapMilliseconds};last={host,decision,subjects:subjects};
    if(dirty)writeState(host,decision,subjects);
    for(const action of decision.actions){
     const subject=byId.get(action.id);if(subject===undefined)continue;
     const alarm=buildHostWardenAlarmV1({atMs:ports.now(),observed:effective==="observe",level:decision.level,action,subject,host,decision});
     if(action.action==="report"){
      if(!reported.has(subject.id)){reported.add(subject.id);appendAlarm(alarm);}
      continue;
     }
     if(effective==="enforce"){
      const start=subject.start??"",scope=subject.scope??"tree";
      if(action.action==="suspend"){
       suspended.set(subject.id,{id:subject.id,pid:subject.pid,start,kind:subject.kind,sinceMs:now});since.set(subject.id,now);{const known=flaps.get(subject.id)??{suspends:[]};known.suspends.push(now);flaps.set(subject.id,known);}writeState(host,decision,subjects);
       signalTree(rows,{pid:subject.pid,scope},["SIGSTOP"]);counters.suspended++;
      }else if(action.action==="resume"){
       signalTree(rows,{pid:subject.pid,scope},["SIGCONT"]);suspended.delete(subject.id);since.delete(subject.id);counters.resumed++;{const known=flaps.get(subject.id)??{suspends:[]};known.resumedAtMs=now;flaps.set(subject.id,known);}lastResumeMs=now;writeState(host,decision,subjects);
      }else{
       const command=rows.find(row=>row.pid===subject.pid)?.command??"",buildScriptOutput=buildScriptOutputDirectoryV1(command);
       const record:HostWardenCancelV1={version:1,pid:subject.pid,start,kind:subject.kind,owner:subject.owner??"",command:command.slice(0,512),megabytes:subject.megabytes,atMs:now,reason:action.reason,ancestors:ancestorsOf(rows,subject.pid),...buildScriptOutput&&buildScriptOutput.length<=256?{buildScriptOutput}:{},...action.capMegabytes===undefined?{}:{capMegabytes:action.capMegabytes},...subject.grantMegabytes===undefined?{}:{grantMegabytes:subject.grantMegabytes},...action.reason==="ceiling"&&subject.ceiling!==undefined?{ceiling:subject.ceiling}:{}};
       writeAtomic(join(cancelled,`${subject.pid}.${createHash("sha256").update(start).digest("hex").slice(0,8)}.json`),JSON.stringify(record));
       const signal=subject.signal??"SIGKILL";
       signalTree(rows,{pid:subject.pid,scope},signal==="SIGTERM"?[signal,"SIGCONT"]:[signal]);
       suspended.delete(subject.id);since.delete(subject.id);counters.cancelled++;writeState(host,decision,subjects);
      }
     }
     if(effective==="enforce"||!announced.has(`${action.action}.${action.id}`))appendAlarm(alarm);
    }
    {const now=new Set(decision.actions.filter(action=>action.action==="report").map(action=>action.id));for(const id of[...reported])if(!now.has(id))reported.delete(id);}
    announced.clear();if(effective==="observe")for(const key of keys)announced.add(key);
    counters.ticks++;
    const elapsed=performance.now()-began;durations.push(elapsed);if(durations.length>policy.tickWindow)durations.shift();
    {const used=process.cpuUsage();marks.push({wall:performance.now(),cpu:(used.user+used.system)/1000});}if(marks.length>policy.tickWindow)marks.shift();
    writeState(host,decision,subjects);
    if(now-lastSubjectAt>=policy.idleMilliseconds){stopReason="idle";break;}
    if(ports.now()-lastCodeCheck>=policy.codeCheckMilliseconds){
     lastCodeCheck=ports.now();
     const current=options.sources===undefined?hostWardenCodeDigestV1():await options.sources();
     if(current!==code){stopReason="code";try{options.successor?.();}catch{}break;}
     try{for(const name of readdirSync(cancelled)){const value=readJson(join(cancelled,name));if(!isHostWardenV1("Cancel",value)||ports.now()-(value as HostWardenCancelV1).atMs>policy.cancelRecordMaximumAgeMilliseconds)rmSync(join(cancelled,name),{force:true});}}catch{}
    }
   }catch(error){ports.write(`[warden] tick failed / Tick fehlgeschlagen: ${String((error as Error)?.message??error).split("\n")[0]}`);counters.ticks++;}
   await ports.sleep(observeOnly?policy.observeOnlyMilliseconds:level==="calm"&&!nearCeiling?policy.calmMilliseconds:policy.tightMilliseconds,options.signal);
  }
 }finally{
  if(mode()==="enforce"&&!observeOnly&&suspended.size>0){const rows=await ports.table().catch(()=>[] as readonly HostWardenRowV1[]);await resumeAll(rows);if(last!==undefined)writeState(last.host,last.decision,last.subjects);}
  try{const held=readJson(lockFile);if(isHostWardenV1("Lock",held)&&(held as {pid:number}).pid===process.pid)rmSync(lockFile,{force:true});}catch{}
 }
 return done(stopReason??"limit",counters.ticks,{suspended:counters.suspended,resumed:counters.resumed,cancelled:counters.cancelled});
}

/** 🤝️ Starts the successor of a warden whose sources changed before it ends, so the machine is never unguarded: the successor (`--handover <pid>`) waits up to `handoverWaitMilliseconds` for the lock of this process and takes over within one tick of its release; the successor reads the new sources, this process ends right after. Never throws. */
export function spawnHostWardenSuccessorV1(options:Readonly<{directory?:string;observe?:boolean;main?:string;within?:number;spawn?:(command:string,args:readonly string[],log:number)=>void}>={}):void{
 try{
  const directory=hostWardenDirectoryV1(options.directory);
  mkdirSync(directory,{recursive:true});
  const log=openSync(join(directory,POLICY.logFile),"a"),main=options.main??fileURLToPath(new URL("../⌨️main/🟦️.ts",import.meta.url)),args=[main,"--handover",String(process.pid),...options.directory===undefined?[]:["--directory",options.directory],...options.observe?["--observe"]:[],...options.within===undefined?[]:["--within",String(options.within)]];
  try{
   if(options.spawn)options.spawn(process.execPath,args,log);
   else spawn(process.execPath,args,{detached:true,stdio:["ignore",log,log],windowsHide:true}).unref();
  }finally{closeSync(log);}
 }catch{}
}

let digestMemory:string|undefined;
const ensured=new Map<string,Readonly<{atMs:number;result:Readonly<{started:boolean;reason:"running"|"replacing"|"spawned"|"failed"}>}>>();

/** 🚀️ Makes sure a warden runs on this machine, cheaply: the source digest is read once per process, a process that ensured less than `ensureMilliseconds` ago (same directory and code, `memoise` not `false`) returns its earlier answer without any file read, otherwise one small file read (`state.json`) and one `kill(pid,0)`; no spawn while a holder with a fresh state is alive (a holder whose code differs from the caller's digest is left to end itself), else it spawns the entry detached with its output in `warden.log` (rotated at `logMaximumBytes`) and `unref`s it. Never throws. A process under a test runner (`NODE_ENV=test`) that names neither a directory nor a spawn port never spawns the real warden of the machine. */
export function ensureHostWardenV1(options:Readonly<{directory?:string;code?:string;nowMs?:number;spawn?:(command:string,args:readonly string[],log:number)=>void;observe?:boolean;main?:string;memoise?:boolean}>={}):Readonly<{started:boolean;reason:"running"|"replacing"|"spawned"|"failed"}>{
 try{
  if(options.spawn===undefined&&options.directory===undefined&&process.env.NODE_ENV==="test")return{started:false,reason:"failed"};
  const now=options.nowMs??Date.now(),code=options.code??(digestMemory??=hostWardenCodeDigestV1()),key=`${options.directory??""}\n${code}`,known=options.memoise===false?undefined:ensured.get(key);
  if(known!==undefined&&now-known.atMs<POLICY.ensureMilliseconds&&now>=known.atMs)return known.result;
  const result=ensureOnce(options,now,code);
  if(options.memoise!==false&&result.reason!=="failed")ensured.set(key,{atMs:now,result:{started:false,reason:result.reason==="spawned"?"running":result.reason}});
  return result;
 }catch{return{started:false,reason:"failed"};}
}

const ensureOnce=(options:Readonly<{directory?:string;spawn?:(command:string,args:readonly string[],log:number)=>void;observe?:boolean;main?:string}>,now:number,code:string):Readonly<{started:boolean;reason:"running"|"replacing"|"spawned"|"failed"}>=>{
 try{
  const directory=hostWardenDirectoryV1(options.directory),state=readJson(join(directory,POLICY.stateFile)) as {atMs?:unknown;pid?:unknown;code?:unknown}|undefined;
  if(state!==undefined&&typeof state.atMs==="number"&&typeof state.pid==="number"&&now-state.atMs<=POLICY.staleMilliseconds&&alive(state.pid))return{started:false,reason:state.code===code?"running":"replacing"};
  mkdirSync(directory,{recursive:true});
  const log=join(directory,POLICY.logFile);
  try{if(statSync(log).size>=POLICY.logMaximumBytes)renameSync(log,`${log}.1`);}catch{}
  const handle=openSync(log,"a"),main=options.main??fileURLToPath(new URL("../⌨️main/🟦️.ts",import.meta.url)),args=[main,...options.directory===undefined?[]:["--directory",options.directory],...options.observe?["--observe"]:[]];
  try{
   if(options.spawn)options.spawn(process.execPath,args,handle);
   else spawn(process.execPath,args,{detached:true,stdio:["ignore",handle,handle],windowsHide:true}).unref();
  }finally{closeSync(handle);}
  return{started:true,reason:"spawned"};
 }catch{return{started:false,reason:"failed"};}
};
