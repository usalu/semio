import{machinePathV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {appendFileSync,mkdirSync,readdirSync,readFileSync,renameSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {randomBytes} from "node:crypto";
import {FLEET_POLICY_V1} from "../🪪️identity/🟦️.ts";

/** 📊️ The four unit counts a machine keeps for `repo fleet-status`. */
export type UnitCountKindV1="localHit"|"remoteHit"|"miss"|"stored"|"skipped";
const KINDS:readonly UnitCountKindV1[]=["localHit","remoteHit","miss","stored","skipped"];
const directory=(workspace:string)=>machinePathV1(workspace,".cache","f","unit-counters");

/** 🔢️ Counts one decision: a single byte appended to the kind's file, which POSIX makes atomic across any number of processes, so the hot path takes no lock and loses nothing. Never throws. */
export function countUnitV1(workspace:string|undefined,kind:UnitCountKindV1):void{
 if(!workspace)return;
 try{
  const base=directory(workspace);
  try{appendFileSync(join(base,kind),"1");}catch{mkdirSync(base,{recursive:true});appendFileSync(join(base,kind),"1");}
  try{statSync(join(base,"since"));}catch{writeFileSync(join(base,"since"),String(Date.now()),{flag:"wx"});}
 }catch{}
}

const REASON=/^[a-z][a-z0-9-]{0,40}$/u,fallbackDirectory=(root:string)=>join(root,FLEET_POLICY_V1.layout.state,FLEET_POLICY_V1.layout.fallbacks);

/** 🧯️ Counts one unit that went to a local compile although the fleet tier was on or wanted, by the reason name (`claims-down`, `await-fault`, `holder-slow`, `breaker-down`, `path-dependent`, ...): one byte appended to the reason's file below the unit cache root, which POSIX makes atomic across processes. A name that is no reason (lower case letters, digits, dashes) is ignored. Never throws. */
export function countFallbackV1(root:string|undefined,reason:string):void{
 if(!root||!REASON.test(reason))return;
 try{const base=fallbackDirectory(root);try{appendFileSync(join(base,reason),"1");}catch{mkdirSync(base,{recursive:true});appendFileSync(join(base,reason),"1");}}catch{}
}

/** 📖️ The fall-back counts of a unit cache root by reason (the size of each reason file). */
export function readFallbacksV1(root:string):Readonly<Record<string,number>>{
 const found:Record<string,number>={};
 try{for(const name of readdirSync(fallbackDirectory(root)))if(REASON.test(name))try{found[name]=statSync(join(fallbackDirectory(root),name)).size;}catch{}}catch{}
 return Object.freeze(found);
}

/** 📖️ The counters as package D's load reporter reads them (`Counters` of the fleet schema): the sizes of the four files, the time since the first count and zero offload counts, which the offload client owns. */
export function readBuildCountersV1(workspace:string):Readonly<{version:1;wallMs:number;unit:Record<UnitCountKindV1,number>;offload:Readonly<{out:number;refused:number;fellBack:number}>}>{
 const base=directory(workspace),size=(name:string)=>{try{return statSync(join(base,name)).size;}catch{return 0;}};
 let since=Date.now();try{since=Number(readFileSync(join(base,"since"),"utf8"))||since;}catch{}
 let offload={out:0,refused:0,fellBack:0};try{const previous=JSON.parse(readFileSync(machinePathV1(workspace,".cache","f","build-counters.json"),"utf8")) as {offload?:typeof offload};if(previous.offload)offload={out:Number(previous.offload.out)||0,refused:Number(previous.offload.refused)||0,fellBack:Number(previous.offload.fellBack)||0};}catch{}
 return Object.freeze({version:1,wallMs:Math.max(0,Date.now()-since),unit:Object.fromEntries(KINDS.map(kind=>[kind,size(kind)])) as Record<UnitCountKindV1,number>,offload});
}

/** 💾️ Writes `<worktree>/.cache/f/build-counters.json` (atomically) from the counter files; the file keeps the `offload` object another writer put there. */
export function writeBuildCountersV1(workspace:string):void{
 try{
  const path=machinePathV1(workspace,".cache","f","build-counters.json"),temporary=path+"."+randomBytes(3).toString("hex");
  mkdirSync(machinePathV1(workspace,".cache","f"),{recursive:true});writeFileSync(temporary,JSON.stringify(readBuildCountersV1(workspace)));renameSync(temporary,path);
 }catch{}
}
