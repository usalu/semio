import {lstat,mkdir,readdir,readFile,realpath,rename,rmdir,writeFile} from "node:fs/promises";
import {dirname,join,relative,resolve,sep} from "node:path";
import {fleetReportV1} from "../../🩺️attest/🟦️.ts";
import {adoptV1,planAdoptV1,type AdoptPlanV1,type AdoptResultV1,type AdoptStepV1} from "../🧲️adopt/🟦️.ts";
import {closeReplicaV1,localPathsV1,openReplicaV1,requireReplicaShapeV1,setSyncEnabledV1,verifyV1,type ReplicaOptionsV1,type ReplicaSinkV1,type VerifyReportV1} from "../🟦️.ts";

/** 📍️ The phases of a relocation: `prepare` stops the old tree from publishing, `move` takes the machine identity to the new tree and adopts the store there (the old services must be stopped by then), `finish` verifies the new tree (and the fleet, with a wait) and only then retires the old tree, `all` runs the three in a row. */
export type RelocatePhaseV1="prepare"|"move"|"finish"|"all";

/** 🪜️ The last finished step of a relocation, recorded in `.cache/f/relocate.json` of both trees. */
export type RelocateStepV1="prepared"|"moved"|"adopted"|"finished";

/** 🎛️ The inputs of a relocation: the old and the new worktree, the phase, `dryRun` (plan the adoption in the new tree and change nothing), `force` (adopt without common history), the number of sample paths, the seconds to wait for the old daemon to stop and for the fleet to agree, and a `fault` hook that tests throw from after a finished step. */
export type RelocateOptionsV1=Readonly<{from:string;to:string;phase:RelocatePhaseV1;dryRun:boolean;force:boolean;sample:number;waitSeconds:number;fault?:(step:RelocateStepV1|AdoptStepV1)=>void}>;

/** 🏁️ What a relocation did: the step reached, the plan (dry run), the adoption result and the verification. */
export type RelocateResultV1=Readonly<{step:RelocateStepV1|"planned";plan?:AdoptPlanV1;adopt?:AdoptResultV1;verify?:VerifyReportV1;fleetOk:boolean|null}>;

/** ⛔️ Raised when a relocation must not go on: `same-path`, `nested` (one tree inside the other), `not-joined` (the old tree has no machine identity), `target-joined` (the new tree belongs to another origin), `running` (the old daemon is alive), `order` (a phase before the one that is needed) or `unverified` (the new tree or the fleet disagrees). */
export class RelocateRefusedV1 extends Error{
 readonly reason:"same-path"|"nested"|"not-joined"|"target-joined"|"running"|"order"|"unverified";
 constructor(reason:RelocateRefusedV1["reason"]){super(`Relocation refused: ${reason}`);this.name="RelocateRefusedV1";this.reason=reason;}
}

type Marker=Readonly<{version:1;step:RelocateStepV1;from:string;to:string;originId:string;groupId:string;storeRoot:string;startedMs:number}>;
type Identity=Readonly<{originId:string;groupId:string;storeRoot:string}>;
const STEPS:readonly RelocateStepV1[]=["prepared","moved","adopted","finished"];
const markerOf=(worktree:string):string=>join(localPathsV1(worktree).directory,"relocate.json");

async function readJson(file:string):Promise<unknown>{
 try{return JSON.parse(await readFile(file,"utf8")) as unknown;}catch(error){if((error as {code?:string}).code==="ENOENT")return undefined;throw error;}
}

async function writeJson(file:string,value:unknown):Promise<void>{
 await mkdir(join(file,".."),{recursive:true});
 const temporary=`${file}.tmp`;
 await writeFile(temporary,JSON.stringify(value));
 await rename(temporary,file);
}

async function identityOf(worktree:string):Promise<Identity|undefined>{
 const value=await readJson(localPathsV1(worktree).machine);
 if(value===undefined)return undefined;
 requireReplicaShapeV1("Machine",value,"Closed replica machine required");
 return value as Identity;
}

async function alive(worktree:string):Promise<boolean>{
 const value=await readJson(localPathsV1(worktree).lock) as {pid?:unknown}|undefined;
 if(typeof value?.pid!=="number")return false;
 try{process.kill(value.pid,0);return true;}catch(error){return (error as {code?:string}).code==="EPERM";}
}

const inside=(parent:string,child:string):boolean=>{const path=relative(parent,child);return path!==""&&!path.startsWith("..")&&!path.startsWith(sep)&&resolve(parent,path)===child;};

async function wait(seconds:number,probe:()=>Promise<boolean>,signal?:AbortSignal):Promise<boolean>{
 const deadline=Date.now()+seconds*1000;
 for(;;){
  signal?.throwIfAborted();
  if(await probe())return true;
  if(Date.now()>=deadline)return false;
  await new Promise<void>(done=>{setTimeout(done,Math.min(500,Math.max(0,deadline-Date.now())));});
 }
}

const stronger=(left:{ms:number;counter:number},right:{ms:number;counter:number}):boolean=>left.ms>right.ms||left.ms===right.ms&&left.counter>=right.counter;

async function carry(from:string,to:string,identity:Identity):Promise<void>{
 const target=localPathsV1(to);
 await mkdir(target.directory,{recursive:true});
 await writeJson(target.machine,{version:1,...identity});
 const old=await openReplicaV1({worktree:from,storeRoot:identity.storeRoot,originId:identity.originId,groupId:identity.groupId});
 let cursors:Record<string,unknown>,clock:{ms:number;counter:number};
 try{cursors={...old.cursors};clock={ms:old.clock.ms,counter:old.clock.counter};}finally{await closeReplicaV1(old);}
 const kept=await readJson(target.clock) as {ms:number;counter:number}|undefined;
 if(await readJson(target.cursor)===undefined)await writeJson(target.cursor,{version:1,cursors});
 if(kept===undefined||!stronger(kept,clock))await writeJson(target.clock,{version:1,...clock});
}

/** 🧳️ What became of the build cache images of the old tree: `carried` (their directory moved to the new tree with a relocation hint), `kept` (the new tree has images of its own), `attached` (an image is mounted, nothing moved), `foreign` (another volume), `none` (the old tree has none) or `failed`. */
export type RelocateImagesV1="carried"|"kept"|"attached"|"foreign"|"none"|"failed";

/** 🧳️ Takes the build cache images of the old tree (`.cache/b`) along to a new tree on the same volume that has none: the directory is renamed as a whole, so every image keeps its inodes, and the hint `image-relocation.json` names the move; the image opener of the build re-binds each image from that hint only after it proved the image's identity, so a refusal there costs a cold cache and never a build. Never throws: a relocation does not depend on the build caches. @see ../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🟦️.ts */
export async function carryImagesV1(from:string,to:string):Promise<RelocateImagesV1>{
 try{
  const source=join(await realpath(from),".cache","b"),target=join(await realpath(to),".cache","b"),origin=await lstat(source).catch(()=>undefined);
  if(!origin?.isDirectory())return "none";
  if((await readdir(target).catch(()=>[])).length>0)return "kept";
  if(origin.dev!==(await lstat(to)).dev)return "foreign";
  for(const name of await readdir(source)){const root=await lstat(join(source,name,"m")).catch(()=>undefined);if(root!==undefined&&root.dev!==origin.dev)return "attached";}
  await rmdir(target).catch(()=>{});
  await mkdir(dirname(target),{recursive:true});
  await rename(source,target);
  await writeJson(join(target,"image-relocation.json"),{version:1,movedFrom:source,movedTo:target});
  return "carried";
 }catch{return "failed";}
}

/** 📍️ Relocates the joined machine at `from` to the repository at `to`, crash-safe and re-runnable (the last finished step is in `.cache/f/relocate.json` of both trees): the machine identity (`machine.json`, the cursors, the clock; the origin id never changes and is never reused) is taken to the new tree, the store is adopted there with `adoptV1`, the new tree is verified (and, with `waitSeconds`, the fleet must agree), and only then the old tree is marked as retired (`relocated.json`, synchronisation off); the old tree is never deleted. The old daemon must be stopped before `move`. A dry run plans the adoption in the new tree and writes nothing. */
export async function relocateV1(options:RelocateOptionsV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<RelocateResultV1>{
 const from=resolve(options.from),to=resolve(options.to),identity=await identityOf(from);
 if(identity===undefined)throw new RelocateRefusedV1("not-joined");
 if(from===to)throw new RelocateRefusedV1("same-path");
 if(inside(from,to)||inside(to,from))throw new RelocateRefusedV1("nested");
 const there=await identityOf(to);
 if(there!==undefined&&there.originId!==identity.originId)throw new RelocateRefusedV1("target-joined");
 const replica:ReplicaOptionsV1={worktree:to,storeRoot:identity.storeRoot,originId:identity.originId,groupId:identity.groupId};
 if(options.dryRun)return {step:"planned",plan:await planAdoptV1(replica,{sample:options.sample},signal,sink),fleetOk:null};
 let marker=(await readJson(markerOf(to)) ?? await readJson(markerOf(from))) as Marker|undefined;
 const reached=(step:RelocateStepV1):boolean=>marker!==undefined&&STEPS.indexOf(marker.step)>=STEPS.indexOf(step);
 const record=async(step:RelocateStepV1):Promise<void>=>{
  marker={version:1,step,from,to,originId:identity.originId,groupId:identity.groupId,storeRoot:identity.storeRoot,startedMs:marker?.startedMs??Date.now()};
  await writeJson(markerOf(from),marker);await writeJson(markerOf(to),marker);
  options.fault?.(step);
 };
 const phases=options.phase==="all"?["prepare","move","finish"]:[options.phase];
 let result:RelocateResultV1={step:marker?.step??"prepared",fleetOk:null};
 for(const phase of phases){
  if(phase==="prepare"&&!reached("prepared")){
   await setSyncEnabledV1(from,false);
   await record("prepared");
  }
  if(phase==="move"){
   if(!reached("prepared"))throw new RelocateRefusedV1("order");
   if(!reached("adopted")){
    if(!await wait(options.waitSeconds,async()=>!await alive(from),signal))throw new RelocateRefusedV1("running");
    if(!reached("moved")){await carry(from,to,identity);await record("moved");}
    const adopted=await adoptV1(replica,{dryRun:false,force:options.force,sample:options.sample,fault:step=>options.fault?.(step)},signal,sink);
    result={...result,adopt:adopted};
    await carryImagesV1(from,to);
    await record("adopted");
   }
  }
  if(phase==="finish"){
   if(!reached("adopted"))throw new RelocateRefusedV1("order");
   const report=await verifyV1(replica,signal,sink);
   let fleetOk:boolean|null=null;
   if(options.waitSeconds>0){
    fleetOk=await wait(options.waitSeconds,async()=>(await fleetReportV1(identity.storeRoot,Date.now(),signal)).ok,signal);
   }
   result={...result,verify:report,fleetOk};
   if(!report.ok||fleetOk===false)throw new RelocateRefusedV1("unverified");
   if(!reached("finished")){
    await setSyncEnabledV1(from,false);
    await writeJson(join(localPathsV1(from).directory,"relocated.json"),{version:1,originId:identity.originId,to,wallMs:Date.now()});
    await record("finished");
   }
  }
 }
 return {...result,step:marker?.step??"prepared"};
}

/** 🪧️ Reads the relocation marker of a worktree, or `undefined` when none was written. */
export async function readRelocationV1(worktree:string):Promise<Marker|undefined>{
 return await readJson(markerOf(resolve(worktree))) as Marker|undefined;
}

