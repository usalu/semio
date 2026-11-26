import{freeSpaceSyncV1}from"../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts";
import{MACHINE_STATE_PROTOCOL_V1,machinePathV1}from"../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {spawn,spawnSync} from "node:child_process";
import {randomBytes} from "node:crypto";
import {closeSync,existsSync,lstatSync,mkdirSync,mkdtempSync,openSync,readdirSync,readFileSync,readSync,realpathSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {dirname,join,resolve,sep} from "node:path";
import {fileURLToPath} from "node:url";
import {cloneFrozenRootV1,editFrozenFileV1,freezeSliceLabRootV1,frozenRootV1} from "./🧊️frozen/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {hostEndpointsV1} from "../../🏗️build/🏡️resident/📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {PLAN_STAGE_POLICY_V1} from "../../🏗️build/🧠️memory/🪜️plan-stage/🟨️.mjs";
import {READY_PROBES_V1,READY_STAGES_V1,type ReadyStageV1} from "../../🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";
import {SLICE_LAB_HOME_V1 as HOME,sliceLabAllStagesV1,sliceLabHomeEnvironmentV1,sliceLabHomeSeedsV1,sliceLabKindsOfSliceV1,sliceLabLauncherBuildV1,sliceLabLinkHomeV1,sliceLabLiveDiffV1,sliceLabLiveSnapshotV1,sliceLabObservedStagesV1,sliceLabPinnedGoSeedsV1,sliceLabSeedHomeV1,sliceLabSeedToolsV1,sliceLabRegistryRefusalV1,sliceLabShimsV1,sliceLabSkipStagesV1,sliceLabStageTableV1,sliceLabToolsForKindsV1,type SliceLabHomeReportV1} from "./🏠️home/🟦️.ts";
import {readFootprintsV1} from "../../🏗️build/🌐️remote-execution/🛰️sidecar/🕸️graph/🔬️stall/🌳️tree/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored numbers and texts of the slice lab. @see ./🧬️schema/🔣️.json */
export const SLICE_LAB_POLICY_V1=schema.definitions.Policy.const;
const P=SLICE_LAB_POLICY_V1,MEGABYTE=1048576,here=dirname(fileURLToPath(import.meta.url)),pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));

/** 💽️ Whether the volume of the work directory can carry a lab: the fixed rust cache image is created at its minimum size (`elastic.rust.minimumBytes` of the storage budget) at the first command of the `workspace` stage and macOS refuses an image beyond the free space of the volume, so a lab that starts with less than that plus its own footprint (`Policy.minimumFreeBytes`) would run for half an hour and stop at that one command. @see https://nodejs.org/api/fs.html#fsstatfssyncpath-options */
export function sliceLabDiskPreflightV1(freeBytes:number):Readonly<{ok:boolean;needBytes:number;freeBytes:number}>{
 return Object.freeze({ok:freeBytes>=P.minimumFreeBytes,needBytes:P.minimumFreeBytes,freeBytes});
}
/** 🎛️ What a lab run is asked to do. */
export type SliceLabOptionsV1=Readonly<{machines:number;memoryMegabytes:16384|8192;cores:number;slice:readonly string[];target?:string;timeoutSeconds:number;workspaceArguments?:readonly string[];execute?:boolean;stages?:"workspace"|"all";skip?:readonly string[];warm?:string;kinds?:readonly string[];frozen?:boolean;frozenDirectory?:string;edit?:string;stagger?:number;preload?:string;out?:string;work?:string;language?:"en"|"de"}>;

const provisionsDependenciesV1=(options:SliceLabOptionsV1):boolean=>options.stages==="all"&&options.skip?.includes("dependencies")!==true;
const needsPrivateCheckoutV1=(options:SliceLabOptionsV1):boolean=>provisionsDependenciesV1(options)||options.frozen===true||options.frozenDirectory!==undefined;

/** 📖️ Reads the options of a run and refuses anything the schema does not declare. */
export function readSliceLabOptionsV1(value:unknown):SliceLabOptionsV1{
 const problems=validateJsonSchemaSubset(schema.definitions.Options,value,schema);
 if(problems.length>0)throw Error(`Closed slice lab options required: ${problems[0]}\nGeschlossene Laborparameter erforderlich: ${problems[0]}`);
 const read=value as SliceLabOptionsV1;
 if(read.edit!==undefined&&(!needsPrivateCheckoutV1(read)||read.machines<2||(read.stagger??0)===0))throw Error("An edited file needs a frozen view, two machines and a stagger / Eine bearbeitete Datei braucht eine eingefrorene Sicht, zwei Maschinen und einen Versatz");
 return read;
}

/** 🗣️ A text of the lab in one language, with its fields filled in. */
export function sliceLabTextV1(language:"en"|"de",key:string,fields:Readonly<Record<string,string|number>>={}):string{
 const table=schema.definitions.Texts.const[language] as Record<string,string>;
 return(table[key]??key).replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]??""));
}

/** 🪜️ The stage plan of the lab: the four stages before `workspace` are replaced by `repo --help` (they provision tools and download), the `workspace` stage reads the project graph of the declared slice through the same entry as `repo run workspace:setup` and executes nothing (`--dry-run`). */
export function sliceLabStagesV1(options:SliceLabOptionsV1):readonly ReadyStageV1[]{
 const skipped=P.skippedStages.map(name=>({name,kind:"repo" as const,arguments:[...P.skippedArguments],timeoutMilliseconds:120000,inputs:[] as string[],bootstrapInput:false}));
 const workspace={name:"workspace",kind:"repo" as const,arguments:options.workspaceArguments!==undefined?[...options.workspaceArguments]:["run-many",`--projects=${options.slice.join(",")}`,`--targets=${options.target??P.target}`,...options.execute===true?[]:["--dry-run"],"--lang=en"],timeoutMilliseconds:options.timeoutSeconds*1000,inputs:[] as string[],bootstrapInput:false};
 return Object.freeze([...skipped,workspace]);
}

/** 🗂️ One machine of a run: its own state root (what `<checkout>/.cache` is on a worker), temporary, runtime and capacity directories. */
export type SliceLabPlanV1=Readonly<{origin:string;index:number;id:string;work:string;root:string;state:string;temporary:string;runtime:string;capacity:string;registry:string;cache:string;home:string;store:string;memoryMegabytes:number;cores:number}>;

/** 🗺️ Names the machines of a run; the runtime directory is short (the socket of the resident service must fit 96 bytes) and the store is the one shared directory. With `own` every machine gets a checkout of its own below its directory (`m1/r`, filled by `cloneFrozenRootV1`) instead of the shared `root`: the Nx client finds its installation at `<checkout>/.nx/installation` and the patched Nx compares storage paths with the checkout, so two machines cannot share one checkout. */
export function sliceLabPlansV1(options:SliceLabOptionsV1,id:string,work:string,root:string,own:boolean=false):readonly SliceLabPlanV1[]{
 return Array.from({length:options.machines},(_,at)=>{
  const index=at+1,home=join(work,`m${index}`);
  return Object.freeze({origin:`slice-lab-${id}-m${index}`,index,id,work,root:own?join(home,"r"):root,state:join(home,"s"),temporary:join(home,"t"),runtime:join(home,P.runtimeDirectory),capacity:join(home,"c"),registry:join(home,"l"),cache:join(home,"x"),home:join(home,HOME.directory),store:join(work,"store"),memoryMegabytes:options.memoryMegabytes,cores:options.cores});
 });
}

/** 📍️ Validates the production socket spelling before provisioning. The digest changes the tag, whose byte width is fixed. */
export function sliceLabRuntimeDirectoryV1(plan:SliceLabPlanV1):string{
 return hostEndpointsV1("",{XDG_RUNTIME_DIR:plan.runtime},process.platform,undefined,plan.root).directory;
}

/** 🔗️ Gives the checkout of a machine the `.nx` its Nx client looks at: the build moves `<checkout>/.nx` below the declared state (`<state>/nx`, `MACHINE_STATE_PROTOCOL_V1.moved`), but Nx resolves the installation of its tooling at `<checkout>/.nx/installation` (`getNxInstallationPath` in the Nx package) and without it falls back to the `node_modules/nx` of the dependency store, which has no compiled authorizers (wall 99: `Original immutable Native snapshot authority required`). A checkout that already has a `.nx` keeps it. Returns whether a link was made. @see https://nx.dev/docs/reference/nx-json */
export function sliceLabNxLinkV1(plan:SliceLabPlanV1):boolean{
 const moved=MACHINE_STATE_PROTOCOL_V1.moved.find(row=>row.prefix===".nx");
 if(moved===undefined)throw Error("The machine state protocol moves no .nx / Das Maschinenzustandsprotokoll verschiebt kein .nx");
 const link=join(plan.root,".nx"),target=join(plan.state,...moved.to);
 mkdirSync(target,{recursive:true});
 try{lstatSync(link);return false;}catch{}
 symlinkSync(target,link);
 return true;
}

/** 🛑️ The pids of the resident watchers of a lab: processes of the table whose command is a `semio-repo-watch serve` below the work directory of the run. The watcher daemonizes out of the process group and drops the marker of the lab from its environment, so the lab's own process filter never lists it; with its watched root deleted under it, one spun at 165 percent of a core for over an hour. @see https://man7.org/linux/man-pages/man1/ps.1.html */
export function sliceLabWatcherPidsV1(table:string,work:string):readonly number[]{
 const pids:number[]=[];
 for(const line of table.split("\n")){
  const match=/^\s*(\d+)\s+(.*)$/u.exec(line);
  if(match!==null&&match[2]!.startsWith(work+sep)&&/\/semio-repo-watch serve(?:\s|$)/u.test(match[2]!))pids.push(Number(match[1]));
 }
 return pids;
}

/** 🌿️ The environment of one machine: the state root, the declared memory and cores (the seam every limit of the build derives from), its own temporary, runtime, capacity and lease-registry directories, no launchd service, the fleet identity, and the marker that finds its processes again. */
export function sliceLabEnvironmentV1(plan:SliceLabPlanV1,preload:string|undefined,base:Readonly<Record<string,string|undefined>>=process.env):Record<string,string>{
 const kept=Object.fromEntries(Object.entries(base).filter(([name,value])=>value!==undefined&&!/^SEMIO_(REMOTE_CACHE|FLEET|HOST_CAPACITY|SLICE_LAB|MACHINE_STATE|LEASE_REGISTRY|SERVICE_LAUNCHD)/u.test(name)&&!["TMPDIR","XDG_RUNTIME_DIR","XDG_CACHE_HOME","BUN_OPTIONS"].includes(name))) as Record<string,string>;
 return{...sliceLabHomeEnvironmentV1(kept,{home:plan.home,live:base.HOME??homedir(),downloads:join(plan.work,`m${plan.index}`,HOME.downloadLog)}),SEMIO_WORKSPACE_ROOT:plan.root,SEMIO_FLEET_ORIGIN:plan.origin,SEMIO_FLEET_GROUP:P.group,SEMIO_FLEET_STORE_ROOT:plan.store,SEMIO_FLEET_ROOT:plan.work,SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:String(plan.memoryMegabytes),SEMIO_HOST_CAPACITY_CORES:String(plan.cores),SEMIO_HOST_CAPACITY_DIRECTORY:plan.capacity,SEMIO_LEASE_REGISTRY_DIRECTORY:plan.registry,SEMIO_MACHINE_STATE_DIRECTORY:plan.state,SEMIO_SERVICE_LAUNCHD:"off",SEMIO_BUILD_GRAPH_SLICE_READ:"selected",SEMIO_LANGUAGE:"en",[P.markerEnvironment]:plan.id,[P.machineEnvironment]:plan.origin,TMPDIR:plan.temporary,XDG_RUNTIME_DIR:plan.runtime,XDG_CACHE_HOME:plan.cache,...preload===undefined?{}:{BUN_OPTIONS:`--preload=${preload}`}};
}

/** 🧱️ The footprint limit of a plan on a worker of this size: the `treeShare` of the declared memory, the figure in the refusals of the workers (6553 MB of 16 GB, 3276 MB of 8 GB). */
export const sliceLabLimitMegabytesV1=(memoryMegabytes:number):number=>Math.floor(memoryMegabytes*PLAN_STAGE_POLICY_V1.treeShare);

/** 🔬️ One process of a run read from the process table. */
export type SliceLabProcessV1=Readonly<{pid:number;ppid:number;rssMegabytes:number;cpu:number;nice:number;machine:string;command:string}>;
/** 🔎️ The rows of `ps -A -ww -E -o pid=,ppid=,rss=,pcpu=,nice=,command=` whose environment carries the marker of this run. */
export function sliceLabProcessRowsV1(text:string,id:string):readonly SliceLabProcessV1[]{
 const rows:SliceLabProcessV1[]=[];
 for(const line of text.split("\n")){
  const match=/^\s*(\d+)\s+(\d+)\s+(\d+)\s+([\d.]+)\s+(-?\d+)\s+(.*)$/u.exec(line);
  if(match===null||!new RegExp(`(?:^|\\s)${P.markerEnvironment}=${id}(?:\\s|$)`,"u").test(match[6]!))continue;
  rows.push({pid:Number(match[1]),ppid:Number(match[2]),rssMegabytes:Number(match[3])/1024,cpu:Number(match[4]),nice:Number(match[5]),machine:new RegExp(`(?:^|\\s)${P.machineEnvironment}=(\\S+)`,"u").exec(match[6]!)?.[1]??"",command:match[6]!.split(/\s[A-Z_]+=/u)[0]!.slice(0,160)});
 }
 return rows;
}
/** 🧮️ What the processes of a machine hold now: the physical footprint where the system measures it (macOS `footprint`, the figure the slice guard judges) and the resident size of `ps` for the rest and for reference; resident size counts file pages of the runtime that the guard never counts, so it is not judged. */
export function sliceLabFootprintV1(rows:readonly Pick<SliceLabProcessV1,"pid"|"rssMegabytes">[],footprints:ReadonlyMap<number,number>):Readonly<{footprintMegabytes:number;rssMegabytes:number;measured:number}>{
 return{footprintMegabytes:Math.round(rows.reduce((sum,row)=>sum+(footprints.get(row.pid)??row.rssMegabytes),0)),rssMegabytes:Math.round(rows.reduce((sum,row)=>sum+row.rssMegabytes,0)),measured:rows.filter(row=>footprints.has(row.pid)).length};
}
const watcherTable=():string=>spawnSync("ps",["-A","-ww","-o","pid=,command="],{encoding:"utf8",maxBuffer:64*MEGABYTE}).stdout??"";
const processTable=():string=>{const run=spawnSync("ps",process.platform==="darwin"?["-A","-ww","-E","-o","pid=,ppid=,rss=,pcpu=,nice=,command="]:["-A","-ww","e","-o","pid=,ppid=,rss=,pcpu=,nice=,command="],{encoding:"utf8",maxBuffer:128*MEGABYTE});return run.status===0?run.stdout:"";};

/** 💿️ The disk images of `hdiutil info` whose file lies below a directory, with their devices. */
export function sliceLabImagesV1(text:string,below:string):readonly Readonly<{path:string;device:string}>[]{
 const rows:{path:string;device:string}[]=[];
 for(const block of text.split(/^=+$/mu)){
  const path=/image-path\s*:\s*(.+)/u.exec(block)?.[1]?.trim(),device=/^\/dev\/(disk\d+)\b/mu.exec(block)?.[1];
  if(path!==undefined&&device!==undefined&&path.startsWith(below+sep))rows.push({path,device:`/dev/${device}`});
 }
 return rows;
}
const attachedImages=(below:string):readonly Readonly<{path:string;device:string}>[]=>process.platform!=="darwin"?[]:sliceLabImagesV1(spawnSync("hdiutil",["info"],{encoding:"utf8"}).stdout??"",below);

/** 📖️ The phase of the graph read in a progress line: `Reading the project graph slice (12 s): taken 3 of 40.` */
export function sliceLabPhaseV1(line:string|undefined):Readonly<{phase:string;done:number;total:number}>|undefined{
 const match=/Reading the project graph slice \(\d+ s\): ([a-z -]+?)(?: \(pass [1-9][0-9]*\))? (\d+) of (\d+)/iu.exec(line??"");
 return match===null?undefined:{phase:match[1]!.trim(),done:Number(match[2]),total:Number(match[3])};
}

/** 🧭️ What the resident service was doing, read from the end of its log: the last line that names a tool download or install (`tools`), a command of the build script (`build:source-admission`, `build:selected-declaration`, ...), a missing module (`module-missing`) or a capacity admission (`capacity`); `-` when the tail names none. */
export function sliceLabActivityV1(tail:string):string{
 const lines=tail.split("\n");
 for(let at=lines.length-1;at>=0;at--){
  const line=lines[at]!,command=/"command":"(build:[a-z-]+)"/u.exec(line);
  if(/^Cannot find module /u.test(line))return"module-missing";
  if(/^(?:Downloading the pinned|info: |warn: |Rust is installed)|rustup|toolchain/u.test(line))return"tools";
  if(command!==null)return command[1]!;
  if(/^\[capacity\] admitted /u.test(line))return"capacity";
 }
 return"-";
}

/** 🔖️ One sample of one machine: the stage and phase, the owners the read has, the processes of the machine and the footprint they hold. */
export type SliceLabSampleV1=Readonly<{atSeconds:number;machine:string;stage:string|null;state:string;phase:string|null;done:number|null;total:number|null;owners:Readonly<Record<string,number>>;processes:number;busy:number;footprintMegabytes:number;rssMegabytes?:number;cpu:number;loopLagMs:number|null;activity:string}>;
/** 🧱️ Where and how a run stopped. */
export type SliceLabWallV1=Readonly<{machine:string;stage:string;phase:string|null;code:string;exit:number|null;text:string;process:string;tail:readonly string[]}>;
/** 🧾️ The result a machine driver files when it ends. */
export type SliceLabMachineResultV1=Readonly<{warm?:readonly Readonly<{stage:string;id:string;ok:boolean;method:string;seconds:number;megabytes:number}>[];ready:boolean;failure:null|Readonly<{stage:string;code:string;exit:number|null;tail:readonly string[]}>;timings:readonly Readonly<{stage:string;seconds:number;outcome:string}>[]}>;
/** ⚖️ The verdict of a run. */
export type SliceLabVerdictV1=Readonly<{outcome:"ready"|"wall"|"timeout";pass:boolean;readyAtSeconds:Readonly<Record<string,number>>;walls:readonly SliceLabWallV1[];phaseSeconds:Readonly<Record<string,number>>;stageSeconds:Readonly<Record<string,number>>;peakFootprintMegabytes:Readonly<Record<string,number>>;peakResidentMegabytes:Readonly<Record<string,number>>;limitMegabytes:number;withinLimit:boolean;duplicateOwners:number;liveFiles:readonly string[];lowered:number;leftoverProcesses:number;leftoverImages:number;admitted:number|null;elapsedSeconds:number;load:number;activitySeconds:Readonly<Record<string,number>>;loopLagMaxMs:number|null;lagWithin:boolean;busyShare:number|null;published:readonly string[];churn:Readonly<{count:number;sample:readonly string[]}>;frozen:Readonly<{files:number;megabytes:number;seconds:number;drift:number}>|null;readySeconds:Readonly<Record<string,number>>;ownersSeen:Readonly<Record<string,Readonly<{total:number;present:number;fetched:number;here:number}>>>;detached:number}>;

/** ⚖️ Judges a run: ready on every machine, or the walls with their stage, phase, text, process and last lines; time per phase and per stage, peak footprint against the limit, duplicate work, leftovers. */
export function sliceLabVerdictV1(input:Readonly<{options:SliceLabOptionsV1;machines:readonly string[];samples:readonly SliceLabSampleV1[];results:Readonly<Record<string,SliceLabMachineResultV1|undefined>>;duplicateOwners:number;liveFiles?:readonly string[];lowered?:number;leftoverProcesses:number;leftoverImages:number;elapsedSeconds:number;load:number;timedOut:boolean;published?:readonly string[];starts?:Readonly<Record<string,number>>;detached?:number;churn?:Readonly<{count:number;sample:readonly string[]}>;frozen?:Readonly<{files:number;megabytes:number;seconds:number;drift:number}>}>):SliceLabVerdictV1{
 const step=P.sampleMilliseconds/1000,phaseSeconds:Record<string,number>={},stageSeconds:Record<string,number>={},activitySeconds:Record<string,number>={},peak:Record<string,number>={},resident:Record<string,number>={},readyAt:Record<string,number>={},walls:SliceLabWallV1[]=[],seen:Record<string,{total:number;present:number;fetched:number;here:number}>={},readySeconds:Record<string,number>={};
 for(const sample of input.samples){
  activitySeconds[sample.activity]=Math.round(((activitySeconds[sample.activity]??0)+step/Math.max(1,input.machines.length))*10)/10;
  if(sample.phase!==null)phaseSeconds[sample.phase]=Math.round(((phaseSeconds[sample.phase]??0)+step/Math.max(1,input.machines.length))*10)/10;
  peak[sample.machine]=Math.max(peak[sample.machine]??0,sample.footprintMegabytes);resident[sample.machine]=Math.max(resident[sample.machine]??0,sample.rssMegabytes??sample.footprintMegabytes);
  if(sample.owners.total!==undefined){const was=seen[sample.machine]??{total:0,present:0,fetched:0,here:0};seen[sample.machine]={total:Math.max(was.total,sample.owners.total??0),present:Math.max(was.present,sample.owners.present??0),fetched:Math.max(was.fetched,sample.owners.fetched??0),here:Math.max(was.here,sample.owners.here??0)};}
 }
 for(const machine of input.machines){
  const result=input.results[machine];
  for(const timing of result?.timings??[])stageSeconds[timing.stage]=Math.max(stageSeconds[timing.stage]??0,timing.seconds);
  if(result?.ready===true){readyAt[machine]=Math.max(0,...input.samples.filter(sample=>sample.machine===machine).map(sample=>sample.atSeconds));readySeconds[machine]=Math.round((readyAt[machine]!-(input.starts?.[machine]??0))*10)/10;}
  if(result?.failure!=null){
   const mine=input.samples.filter(sample=>sample.machine===machine),phase=[...mine].reverse().find(sample=>sample.phase!==null)?.phase??null,last=[...mine].reverse().find(sample=>sample.processes>0);
   walls.push({machine,stage:result.failure.stage,phase,code:result.failure.code,exit:result.failure.exit,text:result.failure.tail.at(-1)??"",process:last===undefined?"repo":`${result.failure.stage}: repo (exit ${result.failure.exit??"-"})`,tail:result.failure.tail.slice(-P.tailLines)});
  }
 }
 const limit=sliceLabLimitMegabytesV1(input.options.memoryMegabytes),allReady=input.machines.every(machine=>readyAt[machine]!==undefined),within=Object.values(peak).every(value=>value<=limit);
 const lags=input.samples.map(sample=>sample.loopLagMs).filter((value):value is number=>value!==null),lagMax=lags.length===0?null:Math.max(...lags),lagWithin=lagMax===null||lagMax<P.loopLagLimitMilliseconds,counted=input.samples.filter(sample=>sample.processes>0),busyShare=counted.length===0?null:Math.round(counted.reduce((sum,sample)=>sum+sample.busy/sample.processes,0)/counted.length*100);
 const admitted=Math.max(0,...input.samples.map(sample=>sample.total??0))||null;
 const outcome=allReady?"ready":walls.length>0?"wall":"timeout";
 return Object.freeze({outcome,pass:allReady&&within&&lagWithin&&input.duplicateOwners===0&&(input.liveFiles??[]).length===0&&(input.lowered??0)===0&&input.leftoverProcesses===0&&input.leftoverImages===0&&(input.frozen===undefined||(input.churn?.count??0)===0),readyAtSeconds:readyAt,walls,phaseSeconds,stageSeconds,peakFootprintMegabytes:peak,peakResidentMegabytes:resident,limitMegabytes:limit,withinLimit:within,duplicateOwners:input.duplicateOwners,liveFiles:input.liveFiles??[],lowered:input.lowered??0,leftoverProcesses:input.leftoverProcesses,leftoverImages:input.leftoverImages,admitted,elapsedSeconds:input.elapsedSeconds,load:input.load,activitySeconds,loopLagMaxMs:lagMax,lagWithin,busyShare,published:input.published??[],readySeconds,ownersSeen:seen,detached:input.detached??0,churn:input.churn??{count:0,sample:[]},frozen:input.frozen??null});
}

const table=(rows:readonly (readonly string[])[]):string=>rows.map(row=>"| "+row.join(" | ")+" |").join("\n");
/** 📜️ The timeline and the verdict as markdown in one language. */
export function sliceLabReportV1(language:"en"|"de",options:SliceLabOptionsV1,verdict:SliceLabVerdictV1,samples:readonly SliceLabSampleV1[]):string{
 const t=(key:string,fields:Readonly<Record<string,string|number>>={})=>sliceLabTextV1(language,key,fields),last=verdict.walls[0],stage=last?.stage??samples.at(-1)?.stage??"-",phase=last?.phase??samples.at(-1)?.phase??"-";
 const head=verdict.outcome==="ready"?t("ready"):verdict.outcome==="wall"?t("wall",{stage,phase}):t("timeout",{seconds:verdict.elapsedSeconds,stage,phase});
 const lines=[`# ${head}`,"",`- ${t("slice",{owners:options.slice.length,admitted:verdict.admitted??"?"})}`,`- ${t("skipped",{stages:P.skippedStages.join(", ")})}`,`- ${t("peak",{peak:Math.max(0,...Object.values(verdict.peakFootprintMegabytes)),limit:verdict.limitMegabytes})}`,`- ${t("peakResident",{peak:Math.max(0,...Object.values(verdict.peakResidentMegabytes))})}`,`- ${t("duplicates",{count:verdict.duplicateOwners})}`,`- ${t("leftovers",{processes:verdict.leftoverProcesses,images:verdict.leftoverImages})}`,`- ${t("load",{load:verdict.load})}`,`- ${t("nice",{count:verdict.lowered})}`,`- ${t("lag",{lag:verdict.loopLagMaxMs??"-",limit:P.loopLagLimitMilliseconds})}`,`- ${t("busy",{share:verdict.busyShare??"-"})}`,`- seconds per activity: ${JSON.stringify(verdict.activitySeconds)}`,`- ${t("published",{count:verdict.published.length})}${verdict.published.length>0?": "+verdict.published.join(", "):""}`,`- ${t("owners",{rows:JSON.stringify(Object.fromEntries(Object.entries(verdict.ownersSeen??{}).map(([machine,row])=>[machine,`${row.present} / ${row.fetched} / ${row.here} of ${row.total}`])))})}`,`- ${t("starts",{rows:JSON.stringify(verdict.readySeconds??{})})}`,`- ${t("detached",{count:verdict.detached??0})}`,`- ${t(verdict.frozen===null||verdict.frozen===undefined?"churn":"frozenChurn",{count:verdict.churn?.count??0,sample:(verdict.churn?.sample.length??0)>0?": "+verdict.churn!.sample.join(", "):""})}`,...verdict.frozen===null||verdict.frozen===undefined?[]:[`- ${t("frozen",verdict.frozen)}`],`- ${t("notExercised")}`,`- ${t("live",{count:verdict.liveFiles.length})}${verdict.liveFiles.length>0?": "+verdict.liveFiles.join(", "):""}`,`- seconds per stage: ${JSON.stringify(verdict.stageSeconds)}`,`- seconds per phase: ${JSON.stringify(verdict.phaseSeconds)}`,""];
 for(const wall of verdict.walls)lines.push(`## ${wall.machine}: ${wall.stage} / ${wall.phase??"-"} (${wall.code})`,"",`process: ${wall.process}`,"","```",...wall.tail,"```","");
 lines.push(table([["t (s)","machine","stage","state","phase","done","total","owners","procs","busy","footprint MB","cpu %","loop lag ms"],["---","---","---","---","---","---","---","---","---","---","---","---","---"],...samples.map(sample=>[String(sample.atSeconds),sample.machine,sample.stage??"-",sample.state,sample.phase??"-",String(sample.done??"-"),String(sample.total??"-"),JSON.stringify(sample.owners),String(sample.processes),String(sample.busy),String(Math.round(sample.footprintMegabytes)),String(sample.cpu),String(sample.loopLagMs??"-")])]),"");
 return lines.join("\n");
}

/** 📡️ One request the fleet cache server of the lab answered, as the logging proxy saw it. */
export type SliceLabRequestV1=Readonly<{method:string;key:string|null;kind:"ac"|"cas"|"claim"|"other";origin:string;status:number;atSeconds:number}>;
/** 📡️ The fleet of a run: the real shared cache server with its claim table in temporary directories on loopback, behind one logging proxy; the facts every machine reads are written into the store and into the state root of each machine. Nothing of the live server is touched. */
export async function startSliceLabFleetV1(work:string,plans:readonly SliceLabPlanV1[],seconds:()=>number):Promise<Readonly<{url:string;requests:SliceLabRequestV1[];stop():Promise<void>}>>{
 const{startRemoteCacheServerV1}=await import("../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts"),{readRemoteCachePolicyV1}=await import("../../🏗️build/🌐️remote-cache/🟦️.ts"),{createCasStoreV1}=await import("../../🏗️build/🗄️cas/🗃️store/🟦️.ts");
 const all=readRemoteCachePolicyV1(),policy=all.server,controller=new AbortController(),requests:SliceLabRequestV1[]=[];
 const server=await startRemoteCacheServerV1({hostStore:createCasStoreV1({root:join(work,"cas")}),directory:join(work,"cache"),host:"127.0.0.1",port:0,maximumBytes:policy.maximumBytes,maximumEntryBytes:Math.min(policy.maximumEntryBytes,Math.floor(policy.maximumBytes/2)),maximumAgeDays:policy.maximumAgeDays,graceMilliseconds:policy.graceSeconds*1000,defaultInstance:all.defaults.instance,tokens:{},validation:policy.actionResults as "complete",maximumActionResultBytes:policy.maximumActionResultBytes,maintenanceSeconds:policy.maintenanceSeconds,signal:controller.signal});
 const proxyFor=(origin:string)=>Bun.serve({hostname:"127.0.0.1",port:0,async fetch(request){
  const url=new URL(request.url),body=request.method==="GET"||request.method==="HEAD"?undefined:await request.arrayBuffer(),answer=await fetch(server.url+url.pathname+url.search,{method:request.method,headers:request.headers,...body===undefined?{}:{body}});
  const claim=/\/claim\/ac\/([0-9a-f]+)$/u.exec(url.pathname),ac=/\/ac\/([0-9a-f]+)$/u.exec(url.pathname),cas=/\/cas\/([0-9a-f]+)$/u.exec(url.pathname);
  requests.push({method:request.method,key:(claim??ac??cas)?.[1]??null,kind:claim?"claim":ac?"ac":cas?"cas":"other",origin,status:answer.status,atSeconds:seconds()});
  return new Response(answer.body,{status:answer.status,headers:answer.headers});
 }});
 const proxies=plans.map(plan=>proxyFor(plan.origin)),urls=proxies.map(proxy=>`http://127.0.0.1:${proxy.port}`),url=urls[0]!,now=Date.now();
 const factsFor=(endpoint:string,port:number)=>({version:1,group:P.group,generation:1,publishedAtMs:now,publisher:{origin:`${P.group}-server`,directory:join(work,"cache"),port},cache:{instance:all.defaults.instance,access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1,transport:"trusted-lan"},auth:{scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]}});
 mkdirSync(join(plans[0]!.store,"v1","fleet"),{recursive:true});writeFileSync(join(plans[0]!.store,"v1","fleet","cache.json"),JSON.stringify(factsFor(url,proxies[0]!.port!)));
 for(const[at,plan]of plans.entries()){mkdirSync(join(plan.state,"f"),{recursive:true});writeFileSync(join(plan.state,"f","cache-facts.json"),JSON.stringify({version:1,fetchedAtMs:now,checkedAtMs:now,selectedEndpoint:urls[at]!,candidates:[urls[at]!],selectedExecution:null,facts:factsFor(urls[at]!,proxies[at]!.port!)}),{mode:0o600});}
 return{url,requests,async stop(){controller.abort();for(const proxy of proxies)proxy.stop(true);await server.stop().catch(()=>{});}};
}

/** 🌊️ The files below `framework` that were changed after the marker file was written (the start of the run): the churn of the live tree that invalidates a long read. */
export function treeChurnV1(root:string,marker:string,ignore:readonly string[]=[],whole=false):Readonly<{count:number;sample:readonly string[]}>{
 const workspace=resolve(root),since=statSync(marker,{bigint:true}).mtimeNs,deadline=performance.now()+120_000;
 const directories=[whole?"":"🧰️framework"],excluded=new Set(ignore.map(name=>join(name))),changed:string[]=[];
 const vanished=(error:unknown)=>["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException)?.code??"");
 const checkTime=()=>{if(performance.now()>=deadline)throw Error("Slice lab change scan timed out");};
 let bytes=0,visited=0;
 while(directories.length){
  checkTime();
  const directory=directories.pop()!;
  let entries;
  try{
   const path=join(workspace,directory);
   if(!lstatSync(path).isDirectory())continue;
   entries=readdirSync(path,{withFileTypes:true});
  }
  catch(error){if(directory&&directory!=="🧰️framework"&&vanished(error))continue;throw error;}
  for(const entry of entries){
   if(++visited%256===0)checkTime();
   const name=join(directory,entry.name);
   if(entry.isDirectory()){
    if(entry.name!=="node_modules"&&entry.name!==".cache"&&!(whole&&(entry.name==="target"||name===".git")))directories.push(name);
   }else if(entry.isFile()&&!excluded.has(name)){
    try{
     const current=lstatSync(join(workspace,name),{bigint:true});
     if(!current.isFile()||current.mtimeNs<=since)continue;
    }catch(error){if(vanished(error))continue;throw error;}
    bytes+=Buffer.byteLength(name)+1;
    if(bytes>64*MEGABYTE)throw Error("Slice lab change list exceeded its size limit");
    changed.push(name);
   }
  }
 }
 let rows=changed;
 if(whole&&changed.length){
  const names=changed.map(name=>name.split(sep).join("/"));
  const ignored=spawnSync("git",["-c","core.fsmonitor=false","check-ignore","-z","--stdin"],{windowsHide:true,cwd:workspace,input:names.join("\0")+"\0",encoding:"utf8",maxBuffer:64*MEGABYTE,timeout:Math.max(1,Math.ceil(deadline-performance.now()))});
  if(ignored.error)throw ignored.error;
  if(ignored.status!==0&&ignored.status!==1)throw Error("Slice lab change scan could not read ignore rules: "+ignored.stderr.trim().slice(0,512));
  const omit=new Set(ignored.stdout.split("\0"));
  rows=changed.filter((_,index)=>!omit.has(names[index]!));
 }
 return Object.freeze({count:rows.length,sample:rows.slice(0,5)});
}

/** 🧮️ What the fleet cache server of a run answered, counted by origin, kind (`ac`, `cas`, `claim`), method and status: who stored, who fetched, who claimed. */
export function sliceLabFleetSummaryV1(requests:readonly SliceLabRequestV1[]):Readonly<Record<string,number>>{
 const counts:Record<string,number>={};
 for(const request of requests){const key=`${request.origin} ${request.kind} ${request.method} ${request.status}`;counts[key]=(counts[key]??0)+1;}
 return counts;
}

/** 🔁️ How many action-cache entries two different machines both stored (a successful PUT of one key from more than one origin): the owners two machines both inferred; a machine that stores a key again after a retry of its own read counts nothing. */
export function sliceLabDuplicatesV1(requests:readonly SliceLabRequestV1[]):number{
 const origins=new Map<string,Set<string>>();
 for(const request of requests)if(request.method==="PUT"&&request.kind==="ac"&&request.key!==null&&request.status>=200&&request.status<300)origins.set(request.key,(origins.get(request.key)??new Set()).add(request.origin));
 return[...origins.values()].filter(set=>set.size>1).length;
}

/** 🕳️ The files below the live state of the checkout (`.cache`, `.nx`) that processes of the run hold open for writing (read-only use of the shared tools and dependencies is not a disturbance): a machine of the lab must neither read nor write them. */
export function sliceLabLiveFilesV1(lsof:string,root:string):readonly string[]{
 const prefixes=[machinePathV1(root,".cache")+sep,machinePathV1(root,".nx")+sep],found=new Set<string>();
 let access="";
 for(const line of lsof.split("\n")){
  if(line.startsWith("f"))access="";
  else if(line.startsWith("a"))access=line.slice(1);
  else if(line.startsWith("n")&&(access==="w"||access==="u")&&prefixes.some(prefix=>line.slice(1).startsWith(prefix)))found.add(line.slice(1));
 }
 return[...found].sort();
}
const openFiles=(pids:readonly number[],root:string):readonly string[]=>pids.length===0?[]:sliceLabLiveFilesV1(spawnSync("lsof",["-n","-P","-Fan","-p",pids.join(",")],{encoding:"utf8",maxBuffer:64*MEGABYTE}).stdout??"",root);

const hostActivity=(directory:string):string=>{
 try{
  const file=readdirSync(directory).find(name=>/^host-.*\.log$/u.test(name));
  if(file===undefined)return"-";
  const found=join(directory,file);
  const size=statSync(found).size,length=Math.min(size,16384),buffer=Buffer.alloc(length),fd=openSync(found,"r");
  try{readSync(fd,buffer,0,length,size-length);}finally{closeSync(fd);}
  return sliceLabActivityV1(buffer.toString("utf8"));
 }catch{return"-";}
};
const selectedPublications=(state:string):readonly string[]=>{
 const found:string[]=[],walk=(directory:string,depth:number):void=>{
  if(depth>7)return;
  let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}
  for(const entry of entries){
   const path=join(directory,entry.name);
   if(entry.isDirectory()){if(entry.name==="selected"){for(const file of readdirSync(path))if(/^[0-9a-f]{64}\.json$/u.test(file))found.push(file);}else walk(path,depth+1);}
  }
 };
 walk(join(state,"b"),0);return found;
};
const readJson=<T>(path:string):T|undefined=>{try{return JSON.parse(readFileSync(path,"utf8")) as T;}catch{return undefined;}};

type SliceLabIoV1=Readonly<{say?(line:string):void;keep?:boolean;hold?:boolean;signal?:AbortSignal;root?:string}>;

/** 🔬️ Runs the lab: one temporary work directory, one driver per machine that runs the real readiness loop and the real ports over the lab's stage plan with its own state root, a sampler every 2 s, the verdict, and the clean-up of everything it started (processes, disk images, directories). */
export async function runSliceLabV1(options:SliceLabOptionsV1,io:SliceLabIoV1={}):Promise<Readonly<{workDirectory:string;samples:readonly SliceLabSampleV1[];verdict:SliceLabVerdictV1;home:SliceLabHomeReportV1}>>{
 if(process.platform==="win32")throw Error(`${sliceLabTextV1("en","windows")}\n${sliceLabTextV1("de","windows")}`);
 const started=Date.now(),id=randomBytes(2).toString("hex"),live=realpathSync(io.root??process.env.SEMIO_WORKSPACE_ROOT??process.cwd()),requested=options.work??"/tmp";
 mkdirSync(requested,{recursive:true});
 const base=realpathSync(requested);
 {const info=freeSpaceSyncV1(base);if(!info)throw Error(`${sliceLabTextV1("en","diskUnknown",{path:base})}\n${sliceLabTextV1("de","diskUnknown",{path:base})}`);const disk=sliceLabDiskPreflightV1(info.availableBytes),fields={path:base,free:Math.floor(disk.freeBytes/1073741824),need:Math.ceil(disk.needBytes/1073741824)};if(!disk.ok)throw Error(`${sliceLabTextV1("en","disk",fields)}\n${sliceLabTextV1("de","disk",fields)}`);}
 const work=mkdtempSync(join(base,`${P.prefix}-`)),owned=lstatSync(work);
 try{
  if(work.length>P.maximumWorkDirectoryCharacters||work.startsWith(live+sep))throw Error("A work directory outside the checkout with a short path is required / Ein Arbeitsverzeichnis ausserhalb des Checkouts mit kurzem Pfad ist erforderlich");
  return await runOwnedSliceLabV1(options,io,{started,id,live,work});
 }finally{
  if(io.keep!==true){
   const current=lstatSync(work,{throwIfNoEntry:false});
   if(current?.isDirectory()&&current.dev===owned.dev&&current.ino===owned.ino)rmSync(work,{recursive:true,force:true});
  }
 }
}

async function runOwnedSliceLabV1(options:SliceLabOptionsV1,io:SliceLabIoV1,{started,id,live,work}:Readonly<{started:number;id:string;live:string;work:string}>){
 const ambientDependencies=!provisionsDependenciesV1(options);
 const say=io.say??(()=>{}),own=needsPrivateCheckoutV1(options),plans=sliceLabPlansV1(options,id,work,live,own),runtimeDirectories=plans.map(sliceLabRuntimeDirectoryV1);
 if(options.warm!==undefined&&!existsSync(options.warm))throw Error(`The warm directory ${options.warm} is absent; build it with slice-lab warm --out=<dir> / Das Warm-Verzeichnis ${options.warm} fehlt; mit slice-lab warm --out=<dir> bauen`);
 const driftMarker=join(work,"start.marker");writeFileSync(driftMarker,"");
 const frozen=own?(say("freezing the tree / Baum wird eingefroren"),options.frozenDirectory===undefined?freezeSliceLabRootV1(live,join(work,P.frozen.directory),{ambientDependencies}):frozenRootV1(live,options.frozenDirectory,ambientDependencies)):undefined,view=frozen?.root??live,marker=frozen===undefined?driftMarker:join(work,"frozen.marker");writeFileSync(marker,"");
 if(frozen!==undefined)say(`frozen view ${frozen.files} files, ${frozen.megabytes} MB, ${frozen.seconds} s / eingefrorene Sicht ${frozen.files} Dateien`);
 const kinds=options.stages==="all"?sliceLabKindsOfSliceV1(options.slice,READY_PROBES_V1.projects,options.kinds):[],tools=sliceLabToolsForKindsV1(kinds),labStages=sliceLabStagesV1(options),stages=options.stages==="all"?sliceLabSkipStagesV1(sliceLabAllStagesV1(READY_STAGES_V1,labStages.at(-1)!,tools),options.skip):labStages,load=Number(spawnSync("sysctl",["-n","vm.loadavg"],{encoding:"utf8"}).stdout?.trim().split(/\s+/u)[1]??0)||0;
 for(const plan of plans){
  mkdirSync(plan.runtime,{recursive:true,mode:0o700});
  for(const path of[...P.provision.map(name=>join(plan.state,...name.split("/"))),plan.state,plan.temporary,plan.capacity,plan.registry,plan.cache,plan.store])mkdirSync(path,{recursive:true});
 }
 if(frozen!==undefined)for(const plan of plans){const made=cloneFrozenRootV1(view,live,plan.root,ambientDependencies);say(`machine checkout ${plan.origin} cloned in ${made.seconds} s / Maschinen-Checkout ${plan.origin} geklont`);}
 for(const plan of plans)sliceLabNxLinkV1(plan);
 const liveHome=process.env.HOME??homedir(),liveBefore=sliceLabLiveSnapshotV1(liveHome,live),seedRows:{machine:string;id:string;ok:boolean;method:string;seconds:number}[]=[];
 for(const plan of plans){mkdirSync(plan.home,{recursive:true});sliceLabRegistryRefusalV1(plan.home);sliceLabShimsV1(plan.home,join(plan.work,`m${plan.index}`,HOME.downloadLog));seedRows.push(...sliceLabSeedHomeV1((seedTools=>[...sliceLabHomeSeedsV1(seedTools,liveHome,plan.home,plan.root),...seedTools==="all"||seedTools.includes("go")?sliceLabPinnedGoSeedsV1(live,plan.home):[]])(sliceLabSeedToolsV1(tools,options.stages))).concat((()=>{sliceLabLinkHomeV1(sliceLabSeedToolsV1(tools,options.stages),plan.home);return[];})()).map(row=>({machine:plan.origin,...row})));}
 const buildLauncher=(plan:SliceLabPlanV1)=>{
  const built=sliceLabLauncherBuildV1(plan.root,plan.home);
  say(`launcher ${built.reason} ${built.seconds}s / Launcher ${built.reason}`);
  seedRows.push({machine:plan.origin,id:"launcher-build",ok:built.ok,method:built.reason,seconds:built.seconds});
 };
 if(options.stages==="all")for(const plan of plans)if(options.edit===undefined||plan.index!==plans.length)buildLauncher(plan);
 say(`home seeded: ${seedRows.map(row=>`${row.id} ${row.method} ${row.seconds}s`).join(", ")} / Home gesät`);
 const samples:SliceLabSampleV1[]=[],drivers=new Map<string,ReturnType<typeof spawn>>(),seconds=()=>Math.round((Date.now()-started)/100)/10;
 let restoreEdit:(()=>void)|undefined,timedOut=false,leftoverProcesses=0,leftoverImages=0,detachedImages=0;const started_=new Map<string,number>();
 const fleet=await startSliceLabFleetV1(work,plans,seconds);
 const results:Record<string,SliceLabMachineResultV1|undefined>={},leaks=new Set<string>(),niced=new Set<number>(),published=new Set<string>();let tick=0;
 const stop=async():Promise<void>=>{
  for(const[,child]of drivers)try{child.kill("SIGTERM");}catch{}
  const mine=()=>sliceLabProcessRowsV1(processTable(),id).filter(row=>row.pid!==process.pid);
  const deadline=Date.now()+P.stopGraceMilliseconds;
  while(mine().length>0&&Date.now()<deadline)await pause(250);
  for(const row of mine())try{process.kill(row.pid,"SIGTERM");}catch{}
  await pause(1500);
  leftoverProcesses=mine().length;
  for(const row of mine())try{process.kill(row.pid,"SIGKILL");}catch{}
  for(const pid of sliceLabWatcherPidsV1(watcherTable(),work))try{process.kill(pid,"SIGTERM");}catch{}
  await pause(500);
  for(const pid of sliceLabWatcherPidsV1(watcherTable(),work))try{process.kill(pid,"SIGKILL");}catch{}
  await pause(500);
  const images=io.hold===true?[]:attachedImages(work);detachedImages=images.length;
  for(const image of images)spawnSync("hdiutil",["detach",image.device,"-force"],{encoding:"utf8"});
  await pause(500);leftoverImages=attachedImages(work).length;
 };
 try{
  const launch=(plan:SliceLabPlanV1):void=>{
   if(options.edit!==undefined&&plan.index===plans.length&&restoreEdit===undefined){
    restoreEdit=editFrozenFileV1(plan.root,options.edit);
    if(options.stages==="all")buildLauncher(plan);
   }
   const config={origin:plan.origin,root:plan.root,group:P.group,stages,state:plan.state,...options.warm===undefined?{}:{warm:{source:options.warm,tools,prime:HOME.primeTools}},result:join(plan.work,`m${plan.index}`,"result.json"),timeoutSeconds:options.timeoutSeconds};
   const file=join(plan.work,`m${plan.index}`,"config.json"),log=join(plan.work,`m${plan.index}`,"driver.log");
   writeFileSync(file,JSON.stringify(config));
   const logFd=openSync(log,"a");
   drivers.set(plan.origin,spawn(process.execPath,[join(here,"👷️machine","🟦️.ts"),`--config=${file}`],{windowsHide:true,cwd:plan.root,env:sliceLabEnvironmentV1(plan,options.preload),stdio:["ignore",logFd,logFd]}));
   started_.set(plan.origin,seconds());
   say(`machine ${plan.origin} started / Maschine ${plan.origin} gestartet`);
  };
  const stagger=options.stagger??0;
  for(const plan of plans)if(plan.index===1||stagger===0)launch(plan);
  const deadline=Date.now()+options.timeoutSeconds*1000;
  while(io.signal?.aborted!==true){
   if(Date.now()>deadline){timedOut=true;break;}
   await pause(P.sampleMilliseconds);
   const rows=sliceLabProcessRowsV1(processTable(),id),at=seconds();const footprints=await readFootprintsV1(rows.map(row=>row.pid));
   for(const plan of plans)if(!started_.has(plan.origin)&&(stagger<0?results[plans[0]!.origin]!==undefined:at>=stagger))launch(plan);
   for(const row of rows)if(row.nice!==0)niced.add(row.pid);
   if(tick%5===0)for(const plan of plans)for(const name of selectedPublications(plan.state))published.add(name);
   if(tick++%3===0)for(const path of[...openFiles(rows.map(row=>row.pid),live),...frozen===undefined?[]:openFiles(rows.map(row=>row.pid),view)])if(leaks.size<20)leaks.add(path);
   for(const plan of plans){
    if(!started_.has(plan.origin))continue;
    const dir=join(plan.work,`m${plan.index}`),ready=readJson<{stage:string|null;state:string;ready:boolean;tail:string[]}>(join(plan.state,"f","ready","ready.json"))??readJson<{stage:string|null;state:string;ready:boolean;tail:string[]}>(join(plan.store,"v1","origins",plan.origin,"ready.json")),report=readJson<{phase?:{name:string;done:number;total:number};sources?:{owners?:{ledger?:Record<string,number>};loop?:{lagMilliseconds?:number}}}>(join(plan.state,"f","slice-memory.json")),mine=rows.filter(row=>row.machine===plan.origin);
    const progress=sliceLabPhaseV1(ready?.tail?.at(-1)),phase=progress?.phase??report?.phase?.name??null;
    samples.push({atSeconds:at,machine:plan.origin,stage:ready?.stage??null,state:ready?.state??"starting",phase,done:progress?.done??report?.phase?.done??null,total:progress?.total??report?.phase?.total??null,owners:report?.sources?.owners?.ledger??{},processes:mine.length,busy:mine.filter(row=>row.cpu>=1).length,...(()=>{const held=sliceLabFootprintV1(mine,footprints);return{footprintMegabytes:held.footprintMegabytes,rssMegabytes:held.rssMegabytes};})(),cpu:Math.round(mine.reduce((sum,row)=>sum+row.cpu,0)*10)/10,loopLagMs:report?.sources?.loop?.lagMilliseconds??null,activity:hostActivity(runtimeDirectories[plan.index-1]!)});
    const result=readJson<SliceLabMachineResultV1>(join(dir,"result.json"));if(result!==undefined)results[plan.origin]=result;
   }
   if(plans.every(plan=>results[plan.origin]!==undefined))break;
  }
  await pause(Math.min(P.settleMilliseconds,1500));
 }finally{
  await stop();
  restoreEdit?.();
  await fleet.stop();
  for(const plan of plans){const result=readJson<SliceLabMachineResultV1>(join(plan.work,`m${plan.index}`,"result.json"));if(result!==undefined)results[plan.origin]=result;}
 }
 const elapsed=seconds(),verdict=sliceLabVerdictV1({options,machines:plans.map(plan=>plan.origin),samples,results,duplicateOwners:sliceLabDuplicatesV1(fleet.requests),liveFiles:[...leaks],lowered:niced.size,leftoverProcesses,leftoverImages,elapsedSeconds:elapsed,load,timedOut,published:[...published],starts:Object.fromEntries(started_),detached:detachedImages,churn:treeChurnV1(view,marker,options.edit===undefined?[]:[options.edit],frozen!==undefined),...frozen===undefined?{}:{frozen:{files:frozen.files,megabytes:frozen.megabytes,seconds:frozen.seconds,drift:treeChurnV1(live,driftMarker).count}}});
 const homeReport:SliceLabHomeReportV1=Object.freeze({tools,kinds,seeds:[...seedRows,...plans.flatMap(plan=>(results[plan.origin]?.warm??[]).map(row=>({machine:plan.origin,id:`warm:${row.stage}:${row.id}`,ok:row.ok,method:row.method+(row.megabytes>0?` ${row.megabytes} MB`:""),seconds:row.seconds})))],stageTables:Object.fromEntries(plans.map(plan=>[plan.origin,sliceLabStageTableV1(stages,results[plan.origin]?.timings??[],results[plan.origin]?.failure??null,sliceLabObservedStagesV1(samples,plan.origin,P.sampleMilliseconds/1000))])),liveChanges:sliceLabLiveDiffV1(liveBefore,sliceLabLiveSnapshotV1(liveHome,live)),downloads:plans.flatMap(plan=>{try{return readFileSync(join(plan.work,`m${plan.index}`,HOME.downloadLog),"utf8").split("\n").filter(Boolean);}catch{return[];}})});
 if(options.out!==undefined){
  mkdirSync(options.out,{recursive:true});
  writeFileSync(join(options.out,"home.json"),JSON.stringify(homeReport,null,1));
  writeFileSync(join(options.out,"timeline.jsonl"),samples.map(row=>JSON.stringify(row)).join("\n")+"\n");
  writeFileSync(join(options.out,"verdict.json"),JSON.stringify({options,verdict},null,1));
  for(const plan of plans){try{writeFileSync(join(options.out,`m${plan.index}-driver.log`),readFileSync(join(plan.work,`m${plan.index}`,"driver.log")));}catch{}}
  for(const plan of plans){
   try{const ready=join(plan.state,"f","ready");for(const name of readdirSync(ready))if(name.endsWith(".log"))writeFileSync(join(options.out,`m${plan.index}-ready-${name}`),readFileSync(join(ready,name)));}catch{}
   try{const directory=runtimeDirectories[plan.index-1]!;for(const file of readdirSync(directory))if(/^host-.*\.log$/u.test(file))writeFileSync(join(options.out,`m${plan.index}-${file}`),readFileSync(join(directory,file)));}catch{}
  }
  writeFileSync(join(options.out,"fleet.json"),JSON.stringify(sliceLabFleetSummaryV1(fleet.requests),null,1));
 }
 return Object.freeze({workDirectory:work,samples,verdict,home:homeReport});
}
