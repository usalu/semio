import{machinePathV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {appendFileSync,mkdirSync,readdirSync,readFileSync,renameSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {randomBytes} from "node:crypto";
import {FLEET_POLICY_V1} from "../🪪️identity/🟦️.ts";
import {readKeySamplesRefV1,type KeySamplesRefV1} from "../🔑️key-samples/🟦️.ts";

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

const SKIP=/^[a-z][a-z0-9-]{0,40}$/u,skipDirectory=(workspace:string)=>join(directory(workspace),"skip");

/** 🚫️ Counts one unit that was compiled here and not shared with the fleet, by the reason name (`breaker-denied`, `breaker-down`, `read-only`, `path-dependent`, `root-key-failed`, `diverged`, `upload-dropped`, `upload-failed`, `publish-refused`, `bypass-native-link`, ...): the `skipped` count grows by one and so does the reason's file, both single appended bytes, so the reasons always add up to `skipped`. A name that is no reason is counted as `other`. Never throws. */
export function countSkipV1(workspace:string|undefined,reason:string):void{
 if(!workspace)return;
 countUnitV1(workspace,"skipped");
 try{const base=skipDirectory(workspace),name=SKIP.test(reason)?reason:"other";try{appendFileSync(join(base,name),"1");}catch{mkdirSync(base,{recursive:true});appendFileSync(join(base,name),"1");}}catch{}
}

/** 🏷️ The reason name of a free-text reason: its first clause, lower case, anything but letters and digits as one dash (`incomplete: a.rlib` is `incomplete`). */
export const skipNameV1=(text:string):string=>text.split(/[:,]/u)[0]!.trim().toLowerCase().replace(/[^a-z0-9]+/gu,"-").replace(/^-+|-+$/gu,"").slice(0,40)||"other";

/** 📖️ The skipped units of a workspace by reason (the size of each reason file), the answer to "why does this machine not share". */
export function readSkipReasonsV1(workspace:string):Readonly<Record<string,number>>{
 const found:Record<string,number>={};
 try{for(const name of readdirSync(skipDirectory(workspace)))if(SKIP.test(name))try{const size=statSync(join(skipDirectory(workspace),name)).size;if(size>0)found[name]=size;}catch{}}catch{}
 return Object.freeze(found);
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

/** 🧾️ The skip reasons so that they always add up to the `skipped` count: units counted before their reason was recorded (an older wrapper wrote only the count) appear as `unrecorded`, so a skip is never invisible. */
export function accountedSkipsV1(skipped:number,reasons:Readonly<Record<string,number>>):Readonly<Record<string,number>>{
 const missing=skipped-Object.values(reasons).reduce((total,count)=>total+count,0);
 return Object.freeze(missing>0?{...reasons,unrecorded:missing}:{...reasons});
}

/** 📖️ The counters as package D's load reporter reads them (`Counters` of the fleet schema): the sizes of the counter files and the skipped units by reason, the time since the first count and zero offload counts, which the offload client owns. */
export function readBuildCountersV1(workspace:string):Readonly<{version:1;wallMs:number;unit:Record<UnitCountKindV1,number>;skipReasons:Readonly<Record<string,number>>;keySamples?:KeySamplesRefV1;offload:Readonly<{out:number;refused:number;fellBack:number}>}>{
 const base=directory(workspace),size=(name:string)=>{try{return statSync(join(base,name)).size;}catch{return 0;}};
 let since=Date.now();try{since=Number(readFileSync(join(base,"since"),"utf8"))||since;}catch{}
 const keySamples=readKeySamplesRefV1(workspace);
 let offload={out:0,refused:0,fellBack:0};try{const previous=JSON.parse(readFileSync(machinePathV1(workspace,".cache","f","build-counters.json"),"utf8")) as {offload?:typeof offload};if(previous.offload)offload={out:Number(previous.offload.out)||0,refused:Number(previous.offload.refused)||0,fellBack:Number(previous.offload.fellBack)||0};}catch{}
 return Object.freeze({version:1,wallMs:Math.max(0,Date.now()-since),unit:Object.fromEntries(KINDS.map(kind=>[kind,size(kind)])) as Record<UnitCountKindV1,number>,skipReasons:accountedSkipsV1(size("skipped"),readSkipReasonsV1(workspace)),...keySamples===undefined?{}:{keySamples},offload});
}

/** 💾️ Writes `<worktree>/.cache/f/build-counters.json` (atomically) from the counter files; the file keeps the `offload` object another writer put there. */
export function writeBuildCountersV1(workspace:string):void{
 try{
  const path=machinePathV1(workspace,".cache","f","build-counters.json"),temporary=path+"."+randomBytes(3).toString("hex");
  mkdirSync(machinePathV1(workspace,".cache","f"),{recursive:true});writeFileSync(temporary,JSON.stringify(readBuildCountersV1(workspace)));renameSync(temporary,path);
 }catch{}
}

/** 🧮️ What one run did with compile units: the growth of the five counters of this machine between two readings, so every run says how many units it restored from this machine, restored from the fleet and compiled. */
export type UnitLedgerV1=Readonly<{restoredLocal:number;restoredFleet:number;compiled:number;stored:number;skipped:number}>;

/** 🧮️ The growth of the counters from `before` to `after` (never negative: a counter directory that was emptied in between counts as zero). */
export function unitLedgerV1(before:Readonly<Record<UnitCountKindV1,number>>,after:Readonly<Record<UnitCountKindV1,number>>):UnitLedgerV1{
 const grow=(kind:UnitCountKindV1)=>Math.max(0,(after[kind]??0)-(before[kind]??0));
 return Object.freeze({restoredLocal:grow("localHit"),restoredFleet:grow("remoteHit"),compiled:grow("miss"),stored:grow("stored"),skipped:grow("skipped")});
}

/** 🧾️ The two lines of a run's unit ledger (English, German); empty when the run touched no unit. */
export function unitLedgerLinesV1(ledger:UnitLedgerV1):readonly string[]{
 if(ledger.restoredLocal+ledger.restoredFleet+ledger.compiled+ledger.skipped===0)return [];
 return [
  `[units] ${ledger.restoredLocal} restored local, ${ledger.restoredFleet} restored fleet, ${ledger.compiled} compiled (${ledger.stored} stored, ${ledger.skipped} not shared)`,
  `[units] ${ledger.restoredLocal} lokal wiederhergestellt, ${ledger.restoredFleet} aus der Flotte wiederhergestellt, ${ledger.compiled} kompiliert (${ledger.stored} gespeichert, ${ledger.skipped} nicht geteilt)`,
 ];
}
