import {appendFileSync,mkdirSync,readFileSync,renameSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {randomBytes} from "node:crypto";

/** 📊️ The four unit counts a machine keeps for `repo fleet-status`. */
export type UnitCountKindV1="localHit"|"remoteHit"|"miss"|"stored"|"skipped";
const KINDS:readonly UnitCountKindV1[]=["localHit","remoteHit","miss","stored","skipped"];
const directory=(workspace:string)=>join(workspace,".cache","f","unit-counters");

/** 🔢️ Counts one decision: a single byte appended to the kind's file, which POSIX makes atomic across any number of processes, so the hot path takes no lock and loses nothing. Never throws. */
export function countUnitV1(workspace:string|undefined,kind:UnitCountKindV1):void{
 if(!workspace)return;
 try{
  const base=directory(workspace);
  try{appendFileSync(join(base,kind),"1");}catch{mkdirSync(base,{recursive:true});appendFileSync(join(base,kind),"1");}
  try{statSync(join(base,"since"));}catch{writeFileSync(join(base,"since"),String(Date.now()),{flag:"wx"});}
 }catch{}
}

/** 📖️ The counters as package D's load reporter reads them (`Counters` of the fleet schema): the sizes of the four files, the time since the first count and zero offload counts, which the offload client owns. */
export function readBuildCountersV1(workspace:string):Readonly<{version:1;wallMs:number;unit:Record<UnitCountKindV1,number>;offload:Readonly<{out:number;refused:number;fellBack:number}>}>{
 const base=directory(workspace),size=(name:string)=>{try{return statSync(join(base,name)).size;}catch{return 0;}};
 let since=Date.now();try{since=Number(readFileSync(join(base,"since"),"utf8"))||since;}catch{}
 let offload={out:0,refused:0,fellBack:0};try{const previous=JSON.parse(readFileSync(join(workspace,".cache","f","build-counters.json"),"utf8")) as {offload?:typeof offload};if(previous.offload)offload={out:Number(previous.offload.out)||0,refused:Number(previous.offload.refused)||0,fellBack:Number(previous.offload.fellBack)||0};}catch{}
 return Object.freeze({version:1,wallMs:Math.max(0,Date.now()-since),unit:Object.fromEntries(KINDS.map(kind=>[kind,size(kind)])) as Record<UnitCountKindV1,number>,offload});
}

/** 💾️ Writes `<worktree>/.cache/f/build-counters.json` (atomically) from the counter files; the file keeps the `offload` object another writer put there. */
export function writeBuildCountersV1(workspace:string):void{
 try{
  const path=join(workspace,".cache","f","build-counters.json"),temporary=path+"."+randomBytes(3).toString("hex");
  mkdirSync(join(workspace,".cache","f"),{recursive:true});writeFileSync(temporary,JSON.stringify(readBuildCountersV1(workspace)));renameSync(temporary,path);
 }catch{}
}
