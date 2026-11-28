import {appendFileSync,mkdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {isRepoErrorV1,renderRepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {landAppendLandingV1,landApplyV1,landAreaOfV1,landBaseProblemsV1,landCacheDirectoryV1,landOverlayBaseV1,landChangesetV1,landLiveHashV1,landLockV1,landMergeGitV1,landRebaseV1,landRecordRollbackV1,landRollbackChangesetV1,landSlotOverrunV1,landSlotStateV1,readLandPolicyV1,validateLandDocumentV1,type LandApplyPortsV1,type LandBaseProblemV1,type LandChangesetV1,type LandMergeV1,type LandOptionsV1,type LandPolicyV1} from "../🟦️.ts";
import {landBuildOverlayV1,landOverlayRootV1,landRunGatesV1,landTestsV1,type LandGatePortsV1,type LandGateRowV1} from "../🔬️gates/🟦️.ts";
import {landTextV1,refuseLandV1,type LandLanguageV1,type LandTextKeyV1} from "../📖️texts/🟦️.ts";

/** 📤️ The structured result of one run. */
export type LandResultV1=Readonly<{version:1;mode:"land"|"check"|"rollback";status:"landed"|"checked"|"failed"|"refused"|"queued"|"rolled-back";id:string;lane:string;seconds:number;areas:readonly string[];slot:Readonly<{bound:boolean;open:boolean;next:string}>;entries:readonly Readonly<{path:string;op:"write"|"delete";base:string|null;after:string|null;area:string}>[];gates:readonly LandGateRowV1[];rollback:string;landings:boolean;reason?:string}>;
/** 🔌️ The seams of a run; tests replace the clock, the waits, the merge, the gates and the apply. */
export type LandFlowPortsV1=LandGatePortsV1&Readonly<{cacheDirectory?:string;merge?:LandMergeV1;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;apply?:Partial<Omit<LandApplyPortsV1,"lockRoot">>;lockPollMilliseconds?:number}>;
/** 🎬️ What one run is asked to do. */
export type LandRequestV1=Readonly<{mode:"land"|"check"|"rollback";directory?:string;id?:string;options:LandOptionsV1;language:LandLanguageV1;environment:NodeJS.ProcessEnv}>;

const pause=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((finish,fail)=>{
 if(signal?.aborted){fail(new Error("aborted"));return;}
 const timer=setTimeout(finish,milliseconds);
 signal?.addEventListener("abort",()=>{clearTimeout(timer);fail(new Error("aborted"));},{once:true});
});
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🔒️ Takes the overlay of one lane for this process; an overlay held by a live process of another run is refused, one left by a dead process is taken over. */
function takeOverlayV1(overlay:string):()=>void{
 const lock=overlay+".lock";
 mkdirSync(resolve(overlay,".."),{recursive:true});
 for(let attempt=0;attempt<2;attempt++){
  try{mkdirSync(lock);writeFileSync(join(lock,"pid"),String(process.pid));return ()=>rmSync(lock,{recursive:true,force:true});}
  catch(error){
   if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   let pid=0;try{pid=Number(readFileSync(join(lock,"pid"),"utf8"));}catch{}
   if(pid>0&&alive(pid)&&pid!==process.pid)throw refuseLandV1("refused","overlay.busy",{lane:overlay.split(/[\\/]/u).pop()!});
   rmSync(lock,{recursive:true,force:true});
  }
 }
 throw refuseLandV1("refused","overlay.busy",{lane:overlay.split(/[\\/]/u).pop()!});
}

/** 🧭️ Runs one landing: stale-base check (or 3-way rebase), overlay build, the gates against the overlay, the slot wait, the area locks, the atomic apply, the rollback point and the landings entry. Refusals and failed gates apply nothing; a cancellation before the apply applies nothing; the apply itself is never interrupted. */
export async function landRunV1(root:string,request:LandRequestV1,ports:LandFlowPortsV1={},limits:LandPolicyV1=readLandPolicyV1()):Promise<LandResultV1>{
 const now=ports.now??Date.now,begin=now(),language=request.language,notice=ports.notice??(()=>{}),sleep=ports.sleep??pause,text=(key:LandTextKeyV1,values:Record<string,string|number>={})=>landTextV1(language,key,values),lockRoot=resolve(root,limits.locks.root),cacheDirectory=ports.cacheDirectory??landCacheDirectoryV1(root,request.environment,limits);
 let changeset:LandChangesetV1,gates:readonly LandGateRowV1[]=[],id="",lane=request.options.lane??request.environment.SEMIO_LAND_LANE??"agent";
 const done=(status:LandResultV1["status"],extra:Partial<LandResultV1>={}):LandResultV1=>({version:1,mode:request.mode,status,id,lane,seconds:Math.round((now()-begin)/10)/100,areas:[...new Set(changeset?.entries.map(entry=>entry.area)??[])].sort(),slot:slotInfo(),entries:changeset?.entries.map(entry=>({...entry}))??[],gates,rollback:"",landings:false,...extra});
 let bound=false;const deferred:string[]=[];
 const slotInfo=()=>{const state=landSlotStateV1(new Date(now()),limits);return {bound,open:state.open,next:state.next};};
 const problemLines=(problems:readonly LandBaseProblemV1[]):string[]=>problems.map(problem=>text(problem.kind==="stale"?"base.stale":problem.kind==="exists"?"base.exists":problem.kind==="gone"?"base.gone":"base.deleted-changed",{path:problem.path}));
 if(request.mode==="rollback"){
  changeset=landRollbackChangesetV1(root,cacheDirectory,request.id!,limits);
  lane=request.options.lane??changeset.stage.lane;
 }else{
  changeset=landChangesetV1(request.directory!,limits);
  if(!changeset.entries.length)throw refuseLandV1("refused","changeset.empty");
  lane=request.options.lane??changeset.stage.lane;
 }
 id=changeset.stage.id;
 bound=changeset.entries.some(entry=>landAreaOfV1(entry.path,limits).slot);
 const verify=async():Promise<LandResultV1|undefined>=>{
  if(request.mode==="rollback")return undefined;
  notice(text("progress.bases",{count:changeset.entries.length}));
  let problems=landBaseProblemsV1(root,changeset);
  if(problems.length&&request.options.rebase){
   const rebased=landRebaseV1(root,request.directory!,changeset,problems,ports.merge??landMergeGitV1);
   changeset=rebased.changeset;problems=rebased.remaining;
   for(const path of rebased.merged)notice(text("rebase.merged",{path}));
   if(rebased.unavailable)notice(text("rebase.unavailable",{reason:"git merge-file"}));
   for(const problem of problems)if(problem.kind==="stale"&&!rebased.unavailable)notice(text("rebase.conflict",{path:problem.path}));
  }
  if(problems.length)return done("refused",{reason:problemLines(problems).join("\n")});
  const overlay=landOverlayRootV1(ports.cacheDirectory===undefined?landOverlayBaseV1(root,request.environment,limits):join(cacheDirectory,"overlay"),lane),release=takeOverlayV1(overlay);
  try{
   notice(text("progress.overlay",{directory:overlay}));
   await landBuildOverlayV1(root,overlay,changeset,{...ports,notice},limits);
   gates=await landRunGatesV1({root,overlay,cacheDirectory,changeset,language,bench:request.options.bench},{...ports,notice,deferred},limits);
  }finally{release();}
  const failed=gates.filter(row=>row.status==="failed");
  if(failed.length)return done("failed",{reason:failed.map(row=>row.id+": "+row.detail.join(" | ")).join("\n")});
  return undefined;
 };
 try{
  const first=await verify();
  if(first)return first;
  if(request.mode==="check"||request.options.dryRun)return done("checked");
  const guard=(): void=>{if(ports.signal?.aborted)throw refuseLandV1("cancelled","cancelled");};
  guard();
  let state=landSlotStateV1(new Date(now()),limits);
  if(bound&&!request.options.outage&&!state.open&&!landSlotOverrunV1(begin,now(),limits)){
   if(request.options.noWait){
    appendFileSync(join(lockRoot,limits.locks.slotQueueFile),`${lane} ${id} ${new Date(now()).toTimeString().slice(0,8)} until ${state.next}\n`);
    notice(text("slot.queued",{next:state.next}));
    return done("queued",{reason:text("totals.queued",{id,next:state.next})});
   }
   const limit=now()+limits.limits.maximumSlotWaitMinutes*60000;
   while(!state.open&&!landSlotOverrunV1(begin,now(),limits)){
    guard();
    notice(text("progress.slot-waiting",{minutes:Math.max(1,Math.ceil((state.nextMs-now())/60000)),next:state.next}));
    await sleep(Math.min(60000,Math.max(1000,state.nextMs-now())),ports.signal);
    state=landSlotStateV1(new Date(now()),limits);
    if(now()>limit)break;
   }
   notice(text("progress.slot-open"));
   notice(text("progress.recheck"));
   const second=await verify();
   if(second)return second;
  }
  guard();
  notice(text("progress.lock",{areas:[...new Set(changeset.entries.map(entry=>entry.area))].sort().join(", ")}));
  const locks=await landLockV1(lockRoot,changeset.entries.map(entry=>entry.area),lane+" land",{now,sleep,signal:ports.signal,notice:(_key,values)=>notice(text("lock.waiting",values))},limits);
  let rollback="";
  try{
   if(request.mode==="land"){const late=landBaseProblemsV1(root,changeset);if(late.length)return done("refused",{reason:problemLines(late).join("\n")});}
   guard();
   rollback=landRecordRollbackV1(root,changeset,cacheDirectory,now());
   notice(text("progress.apply",{count:changeset.entries.length}));
   landApplyV1(root,changeset,{lockRoot,...ports.apply},limits);
   notice(text("progress.verify"));
   for(const entry of changeset.entries)if(landLiveHashV1(root,entry.path)!==entry.after)throw refuseLandV1("failed","apply.verify-failed",{path:entry.path});
  }finally{locks.release();}
  notice(text("rollback.recorded",{id:rollback}));
  const wave=request.options.wave??request.environment.SEMIO_LAND_WAVE??limits.landings.defaultWave;
  const file=landAppendLandingV1(root,wave,changeset,{rollback,areas:locks.areas,gates:gates.map(row=>row.id+" "+row.status),heldMilliseconds:locks.heldMilliseconds(),atMs:now()},limits);
  notice(text("landings.appended",{file}));
  if(deferred.length&&request.mode==="land"){
   const live=await (ports.tests??landTestsV1)({overlay:root,files:[...new Set(deferred)],signal:ports.signal});
   if(!live.passed){
    for(const row of live.failures??[])notice(text("tests.live-red",{file:row.file}));
    const back=await landRunV1(root,{...request,mode:"rollback",id:rollback,options:{...request.options,outage:true}},ports,limits);
    return done("failed",{reason:[text("tests.live-red",{file:deferred.join(", ")}),...live.lines].join("\n"),rollback,landings:true,areas:[...locks.areas],...(back.status==="rolled-back"?{}:{})});
   }
  }
  return done(request.mode==="rollback"?"rolled-back":"landed",{rollback,landings:true,areas:[...locks.areas]});
 }catch(error){
  if(isRepoErrorV1(error)&&(error as {code?:string}).code==="cancelled"){notice(renderRepoErrorV1(error,language));return done("refused",{reason:text("cancelled")});}
  throw error;
 }
}

/** 🔎️ Whether a result document satisfies its closed schema; the problems are returned. */
export const landResultProblemsV1=(result:LandResultV1):readonly string[]=>validateLandDocumentV1("Result",result);
