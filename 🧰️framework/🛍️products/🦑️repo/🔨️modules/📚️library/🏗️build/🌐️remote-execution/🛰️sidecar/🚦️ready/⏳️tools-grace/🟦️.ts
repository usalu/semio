import budget from "../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {nativeImageRootV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {leaseRegistryFolderV1} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";
import {machinePathV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {closeSync,constants,fstatSync,lstatSync,openSync,opendirSync,readSync} from "node:fs";
import {dirname,isAbsolute,join,resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
const policy=schema.definitions.ToolsGracePolicy.const;
export type ToolsProof=Readonly<{pid:number;birth:number;parentSince:number;sourceSince:number}>;
export type ToolsSample=Readonly<{proof:ToolsProof;phase:string;series:number;done:number;updatedAt:number}>;
export type ToolsCounter=Readonly<{phase:string;series:number;done:number;previousAt:number;advancedAt:number;reset:boolean}>;
export type ToolsCheckpoint=Readonly<{proof:ToolsProof|null;counters:readonly ToolsCounter[];current:string|null;sampleAt:number;registryAt:number}>;
type OwnedProcess=Readonly<{pid:number;startMs:number}>;
const natural=(value:unknown):value is number=>typeof value==="number"&&Number.isSafeInteger(value)&&value>0;
const same=(a:ToolsProof,b:ToolsProof)=>a.pid===b.pid&&a.birth===b.birth&&a.parentSince===b.parentSince&&a.sourceSince===b.sourceSince;
const stamp=(row:ReturnType<typeof facts>)=>[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");
const facts=(path:string)=>lstatSync(path,{bigint:true});
function readEntry(path:string,device:bigint):unknown{
 const named=facts(path);
 if(named.dev!==device||!named.isFile()||named.isSymbolicLink()||named.nlink!==1n||named.size<1n||named.size>BigInt(policy.maxRegistryBytes))return;
 const file=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));
 try{
  const before=fstatSync(file,{bigint:true});
  if(!before.isFile()||stamp(before)!==stamp(named))return;
  const bytes=Buffer.alloc(Number(named.size)+1);let count=0;
  while(count<bytes.length){const n=readSync(file,bytes,count,bytes.length-count,count);if(n===0)break;count+=n;}
  if(count!==Number(named.size)||stamp(fstatSync(file,{bigint:true}))!==stamp(named)||stamp(facts(path))!==stamp(named))return;
  return JSON.parse(bytes.subarray(0,count).toString("utf8"));
 }finally{closeSync(file);}
}
function sampleOf(value:unknown,pid:number,birth:number,input:{directory:string;owner:string;parent:string;source:string;started:number;now:number}):ToolsSample|undefined{
 if(!value||typeof value!=="object")return;
 const row=value as Partial<{version:number;pid:number;startedAtMs:number;updatedAtMs:number;directory:string;owner:string;holds:readonly {resource?:string;mode?:string;sinceMs?:number}[];standing:{version?:number;phase?:string;done?:number;phaseStartedAtMs?:number}}>;
 if(row.version!==1||row.pid!==pid||row.startedAtMs!==birth||row.directory!==input.directory||row.owner!==input.owner||!natural(row.updatedAtMs)||
   row.updatedAtMs<Math.max(input.started,birth,input.now-policy.maxRegistryAgeMs)||row.updatedAtMs>input.now||!Array.isArray(row.holds)||row.holds.length>64)return;
 const parent=row.holds.filter(hold=>hold?.resource===input.parent),source=row.holds.filter(hold=>hold?.resource===input.source);
 if(parent.length!==1||source.length!==1||parent[0]?.mode!=="exclusive"||source[0]?.mode!=="exclusive")return;
 const parentSince=parent[0].sinceMs,sourceSince=source[0].sinceMs,proof=row.standing;
 if(!natural(parentSince)||!natural(sourceSince)||parentSince<input.started||sourceSince<input.started||proof?.version!==1||
  typeof proof.phase!=="string"||!schema.definitions.ToolsGracePhase.enum.includes(proof.phase)||!natural(proof.done)||!natural(proof.phaseStartedAtMs)||
  proof.phaseStartedAtMs<Math.max(parentSince,sourceSince)||proof.phaseStartedAtMs>row.updatedAtMs)return;
 return {proof:{pid,birth,parentSince,sourceSince},phase:proof.phase,series:proof.phaseStartedAtMs,done:proof.done,updatedAt:row.updatedAtMs};
}
export function sampleToolsProgress(input:{folder:string;directory:string;owner:string;parent:string;source:string;started:number;now:number;tolerance:number;members:readonly OwnedProcess[]}):ToolsSample|undefined{
 try{
  const folder=facts(input.folder),registry=facts(dirname(input.folder));
  if(process.platform!=="win32"&&((folder.mode|registry.mode)&0o022n)!==0n)return;
  if(!folder.isDirectory()||folder.isSymbolicLink()||!registry.isDirectory()||registry.isSymbolicLink())return;
  const members=new Map(input.members.filter(row=>natural(row.pid)&&natural(row.startMs)&&row.startMs+input.tolerance>=input.started).map(row=>[row.pid,row.startMs]));
  if(members.size!==input.members.length)return;
  const entries=opendirSync(input.folder);let total=0,selected:ToolsSample|undefined;
  try{
   for(let entry;(entry=entries.readSync())!==null;){
    if(++total>policy.maxRegistryNames)return;
    const match=/^([1-9][0-9]{0,15})-([1-9][0-9]{0,15})\.json$/u.exec(entry.name);
    if(!match)continue;
    const pid=Number(match[1]),birth=Number(match[2]),kernelBirth=members.get(pid);
    if(kernelBirth===undefined||!natural(birth)||Math.abs(kernelBirth-birth)>input.tolerance)continue;
    const row=sampleOf(readEntry(join(input.folder,entry.name),folder.dev),pid,birth,input);
    if(row){if(selected)return;selected=row;}
   }
  }finally{entries.closeSync();}
  const after=facts(input.folder),parent=facts(dirname(input.folder));
  if(after.dev!==folder.dev||after.ino!==folder.ino||after.mode!==folder.mode||parent.dev!==registry.dev||parent.ino!==registry.ino||parent.mode!==registry.mode)return;
  return selected;
 }catch{return;}
}
export function recordToolsProgress(previous:ToolsCheckpoint|null,value:ToolsSample|undefined,now:number):ToolsCheckpoint{
 const interrupted=()=>({proof:previous?.proof??null,counters:previous?.counters.map(row=>({...row,previousAt:0,advancedAt:0}))??[],current:null,sampleAt:now,registryAt:previous?.registryAt??0});
 if(value===undefined||value.updatedAt>now||value.series>value.updatedAt)return interrupted();
 const retained=previous?.proof!==null&&previous?.proof!==undefined&&same(previous.proof,value.proof)?previous:undefined;
 if(retained&&value.updatedAt<retained.registryAt)return interrupted();
 const counters=retained?.counters??[],old=counters.find(row=>row.phase===value.phase);
 if(old!==undefined&&(value.series<old.series||value.series>old.series&&value.updatedAt===retained?.registryAt))return interrupted();
 const counter:ToolsCounter=old===undefined||value.series>old.series&&value.phase!=="javascript-source-provider"?
  {phase:value.phase,series:value.series,done:value.done,previousAt:0,advancedAt:0,reset:false}:
  old.reset||value.series!==old.series||value.done<old.done?{...old,reset:true}:
  value.done>old.done&&value.updatedAt>retained!.registryAt?{...old,done:value.done,previousAt:old.advancedAt,advancedAt:now}:old;
 return {proof:value.proof,counters:[...counters.filter(row=>row.phase!==value.phase),counter],current:value.phase,sampleAt:now,registryAt:value.updatedAt};
}
export type ToolsGraceDenial="not-eligible"|"budget-exhausted"|"progress-quiet"|"sample-missing"|"sample-unqualified"|"sample-stale"|"series-unqualified"|"series-needs-advances";
type ToolsGraceWindow=Readonly<{stage:string;started:number;now:number;soft:number;quiet:number;sampleLimit:number}>;

/** 🚦️ Which evaluated gate prevents an additional tools interval. */
export function toolsGraceDenial(state:ToolsCheckpoint|null,input:ToolsGraceWindow&Readonly<{eligible:boolean;progressing:boolean}>):ToolsGraceDenial|undefined{
 const elapsed=input.now-input.started;
 if(!input.eligible||input.stage!==policy.stage||elapsed<=input.soft*policy.baseMultiplier)return "not-eligible";
 if(elapsed>input.soft*(policy.baseMultiplier+policy.extraIntervals))return "budget-exhausted";
 if(!input.progressing)return "progress-quiet";
 if(state===null)return "sample-missing";
 if(state.proof===null||state.current===null||state.sampleAt>input.now)return "sample-unqualified";
 if(input.now-state.sampleAt>input.sampleLimit)return "sample-stale";
 const counter=state.counters.at(-1);
 if(counter?.phase!==state.current||counter.reset)return "series-unqualified";
 const advanced=(row:ToolsCounter)=>!row.reset&&row.previousAt>=input.started&&row.previousAt<row.advancedAt&&
  row.advancedAt<=input.now&&input.now-row.previousAt<=input.quiet;
 if(advanced(counter))return;
 const previous=state.counters.at(-2);
 if(counter.phase==="javascript-source-provider"&&previous?.phase==="native-artifact-input-read"&&advanced(previous))return;
 return "series-needs-advances";
}

export function toolsGraceActive(state:ToolsCheckpoint|null,input:ToolsGraceWindow):boolean{
 return toolsGraceDenial(state,{...input,eligible:true,progressing:true})===undefined;
}

export function toolsProgressPaths(root:string,registry:string){
 if(resolve(root)!==root||!isAbsolute(registry))return;
 const parent=machinePathV1(root,budget.partitions.dependencies.directory),directory=join(nativeImageRootV1(machinePathV1(root,budget.partitions.leases.directory)),"data");
 return {folder:leaseRegistryFolderV1(registry,directory),directory,parent:"native-fixed-image:"+parent,source:"javascript-dependency-bindings:"+root};
}
export function currentToolsCheckpoint(value:unknown,started:number,now:number):value is ToolsCheckpoint{
 if(validateJsonSchemaSubset(schema.definitions.ToolsGraceCheckpoint,value,schema).length)return false;
 const state=value as ToolsCheckpoint;
 if(state.sampleAt<started||state.sampleAt>now||state.registryAt>state.sampleAt||state.registryAt!==0&&state.registryAt<started||
  (state.proof===null?(state.current!==null||state.counters.length!==0||state.registryAt!==0):
   state.proof.birth<started||state.proof.parentSince<started||state.proof.sourceSince<started||state.proof.birth>state.registryAt)||
  new Set(state.counters.map(row=>row.phase)).size!==state.counters.length||state.current!==null&&state.counters.at(-1)?.phase!==state.current)return false;
 return state.counters.every(row=>row.series>=started&&row.series<=state.registryAt&&row.advancedAt<=state.sampleAt&&row.previousAt<=row.advancedAt&&
  (row.previousAt===0||row.previousAt>=started&&row.previousAt<row.advancedAt)&&(row.advancedAt===0||row.advancedAt>=row.series));
}
