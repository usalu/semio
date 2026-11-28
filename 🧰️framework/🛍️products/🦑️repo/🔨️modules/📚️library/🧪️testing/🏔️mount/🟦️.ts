import {bindCatalogV1,textV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import texts from "./📖️texts/🔣️.json" with {type:"json"};

/** 📐️ The closed policy of every mount plan: phase count, phase and total deadlines, detail length.
 * @see ./🧬️schema/🔣️.json */
export const MOUNT_POLICY_V1=schema.definitions.Policy.const;

/** 📖️ The English and German texts of mount verdicts.
 * @see ./📖️texts/🔣️.json */
export const MOUNT_TEXTS_V1=bindCatalogV1("mount",texts);

/** 🌍️ The two languages of every mount reason. */
export type MountLanguageV1="en"|"de";

/** 🧭️ One phase of a plan: its id and the milliseconds it may take. */
export type MountPhaseV1=Readonly<{id:string;budgetMs:number}>;

/** 🗺️ What a test must mount, in order: only the phases the test touches. */
export type MountPlanV1=Readonly<{version:1;app:string;phases:readonly MountPhaseV1[]}>;

/** 👁️ How one phase ended, as measured. */
export type MountObservationV1=Readonly<{kind:"done"|"refused"|"deadline"|"cancelled";elapsedMs:number;detail?:string;detailDe?:string}>;

/** 🗣️ A refusal a step can throw with its detail in both languages; any other error is one language-neutral detail. */
export class MountRefusalV1 extends Error{constructor(readonly en:string,readonly de:string){super(`${en} / ${de}`);}}

/** 🧾️ The first phase that failed, with the reason in both languages. */
export type MountFailureV1=Readonly<{phase:string;code:"refused"|"deadline"|"cancelled";elapsedMs:number;budgetMs:number;detail:string;en:string;de:string}>;

/** 📜️ The receipt of one mount: every phase with its status, the total time and the first failure. */
export type MountReceiptV1=Readonly<{version:1;app:string;passed:boolean;elapsedMs:number;phases:readonly Readonly<{id:string;status:"passed"|"failed"|"skipped";elapsedMs:number;budgetMs:number}>[];failure?:MountFailureV1}>;

/** 🪜️ The work of one phase; it must stop when `signal` aborts. */
export type MountStepV1<C>=(context:C,signal:AbortSignal)=>Promise<void>;

/** 🛂️ Validates a plan against the schema and the policy and returns it, or throws one bilingual reason. */
export function parseMountPlanV1(value:unknown):MountPlanV1{
 const errors=validateJsonSchemaSubset({$ref:"#/definitions/Plan"},value,schema as never);
 if(errors.length>0)throw Error(both("plan.invalid",{detail:String(errors[0]).slice(0,MOUNT_POLICY_V1.detailCharacters)}));
 const plan=value as MountPlanV1,ids=new Set(plan.phases.map(phase=>phase.id)),total=plan.phases.reduce((sum,phase)=>sum+phase.budgetMs,0);
 if(ids.size!==plan.phases.length)throw Error(both("plan.invalid",{detail:"duplicate phase id"}));
 if(total>MOUNT_POLICY_V1.maximumTotalMs)throw Error(both("plan.total",{app:plan.app,total,maximum:MOUNT_POLICY_V1.maximumTotalMs}));
 return plan;
}

/** 🧮️ Folds the observations of the phases that ran into a receipt: phases pass in order until the first observation that is not `done`, which fails with its reason in both languages, and every later phase is skipped. Pure, so the language-neutral vectors and the independent oracle check exactly this.
 * @see ./🧪️tests/🔣️.json */
export function foldMountPhasesV1(plan:MountPlanV1,observations:readonly MountObservationV1[]):MountReceiptV1{
 const phases:{id:string;status:"passed"|"failed"|"skipped";elapsedMs:number;budgetMs:number}[]=[];
 let failure:MountFailureV1|undefined,elapsedMs=0;
 plan.phases.forEach((phase,index)=>{
  const seen=observations[index];
  if(failure!==undefined||seen===undefined){phases.push({id:phase.id,status:"skipped",elapsedMs:0,budgetMs:phase.budgetMs});return;}
  elapsedMs+=seen.elapsedMs;
  const kind=seen.kind==="done"&&seen.elapsedMs>phase.budgetMs?"deadline":seen.kind;
  phases.push({id:phase.id,status:kind==="done"?"passed":"failed",elapsedMs:seen.elapsedMs,budgetMs:phase.budgetMs});
  if(kind==="done")return;
  const values={app:plan.app,phase:phase.id,elapsed:Math.round(seen.elapsedMs),budget:phase.budgetMs,detail:(seen.detail??"").slice(0,MOUNT_POLICY_V1.detailCharacters)};
  const key=kind==="deadline"&&values.detail!==""?"failure.deadline-detail":`failure.${kind}`,used=(detail:string):Record<string,string|number>=>kind==="refused"?{...values,detail}:kind==="cancelled"?{app:values.app,phase:values.phase,elapsed:values.elapsed}:key==="failure.deadline-detail"?{app:values.app,phase:values.phase,budget:values.budget,detail}:{app:values.app,phase:values.phase,budget:values.budget};
  failure={phase:phase.id,code:kind,elapsedMs:seen.elapsedMs,budgetMs:phase.budgetMs,detail:values.detail,en:text("en",key,used(values.detail)),de:text("de",key,used((seen.detailDe??values.detail).slice(0,MOUNT_POLICY_V1.detailCharacters)))};
 });
 return Object.freeze({version:1,app:plan.app,passed:failure===undefined&&phases.every(row=>row.status==="passed"),elapsedMs,phases,...failure===undefined?{}:{failure}});
}

/** 🏔️ Mounts an app phase by phase: each phase gets its own deadline and its own abort signal (aborted at the deadline, at cancellation of `signal`, and after it settles), the first phase that throws, overruns or is cancelled ends the run, the later phases never start. `onPhase` reports progress after every phase.
 * @see foldMountPhasesV1 */
export async function runMountPlanV1<C>(plan:MountPlanV1,steps:Readonly<Record<string,MountStepV1<C>>>,context:C,options:Readonly<{signal?:AbortSignal;now?:()=>number;onPhase?:(row:MountReceiptV1["phases"][number],index:number,count:number)=>void}>={}):Promise<MountReceiptV1>{
 const phaseIds=plan.phases.map(phase=>phase.id),stepIds=Object.keys(steps);
 if(phaseIds.length!==stepIds.length||phaseIds.some(id=>steps[id]===undefined))throw Error(both("plan.steps",{app:plan.app,phases:phaseIds.join(","),steps:stepIds.join(",")}));
 const now=options.now??(()=>performance.now()),observations:MountObservationV1[]=[];
 for(const [index,phase] of plan.phases.entries()){
  const started=now(),controller=new AbortController(),outer=options.signal,abort=():void=>{controller.abort(outer?.reason);};
  if(outer?.aborted){observations.push({kind:"cancelled",elapsedMs:0});break;}
  outer?.addEventListener("abort",abort,{once:true});
  let timer:ReturnType<typeof setTimeout>|undefined;
  const deadline=new Promise<MountObservationV1>(resolve=>{timer=setTimeout(()=>{resolve({kind:"deadline",elapsedMs:now()-started});controller.abort(Error("deadline"));},phase.budgetMs);});
  const cancelled=new Promise<MountObservationV1>(resolve=>{controller.signal.addEventListener("abort",()=>{if(outer?.aborted)resolve({kind:"cancelled",elapsedMs:now()-started});},{once:true});});
  const work=steps[phase.id]!(context,controller.signal).then(():MountObservationV1=>({kind:"done",elapsedMs:now()-started}),(error:unknown):MountObservationV1=>({kind:outer?.aborted?"cancelled":"refused",elapsedMs:now()-started,...error instanceof MountRefusalV1?{detail:error.en,detailDe:error.de}:{detail:error instanceof Error?error.message:String(error)}}));
  let seen=await Promise.race([work,deadline,cancelled]);
  clearTimeout(timer);outer?.removeEventListener("abort",abort);controller.abort();
  if(seen.kind==="deadline"){const last=await Promise.race([work,new Promise<undefined>(resolve=>setTimeout(resolve,100))]);if(last?.detail!==undefined)seen={...seen,detail:last.detail,...last.detailDe===undefined?{}:{detailDe:last.detailDe}};}
  observations.push(seen);
  const row=foldMountPhasesV1(plan,observations).phases[index]!;
  options.onPhase?.(row,index,plan.phases.length);
  if(row.status!=="passed")break;
 }
 return foldMountPhasesV1(plan,observations);
}

/** 🚨️ Throws the first failure of a receipt with its English and German reason; a passed receipt returns. */
export function assertMountedV1(receipt:MountReceiptV1):void{
 if(receipt.failure!==undefined)throw Error(`${receipt.failure.en} / ${receipt.failure.de}`);
}

/** 📈️ Renders one progress line of a phase in a language. */
export function mountProgressLineV1(language:MountLanguageV1,app:string,row:MountReceiptV1["phases"][number],index:number,count:number):string{
 return text(language,"progress.phase",{app,index:index+1,count,phase:row.id,status:row.status,elapsed:Math.round(row.elapsedMs)});
}

const text=(language:MountLanguageV1,key:string,values:Readonly<Record<string,string|number>>):string=>textV1(MOUNT_TEXTS_V1.catalog,key,language,values);
const both=(key:string,values:Readonly<Record<string,string|number>>):string=>`${text("en",key,values)} / ${text("de",key,values)}`;
