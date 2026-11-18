import {isAbsolute,relative} from "node:path";
import {digestTaskViewInputsV1,openTaskViewV1,taskRootModeV1,taskViewCargoEnvironmentV1,taskViewInputsDigestV1,type TaskViewEnvironmentV1,type TaskViewInputV1,type TaskViewOutcomeV1,type TaskViewPolicyV1,type TaskViewRequestV1,type TaskViewV1} from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🏃️ The bounds of re-pinning and rerunning of a task on a view.
 * @see ./🧬️schema/🔣️.json */
export const TASK_VIEW_EXECUTOR_POLICY_V1=schema.definitions.Policy.const;
/** 🏃️ The texts of the executor, English and German. */
export const TASK_VIEW_EXECUTOR_TEXTS_V1=schema.definitions.Texts.const;

const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]??""));
const say=(key:keyof typeof TASK_VIEW_EXECUTOR_TEXTS_V1["en"],values:Readonly<Record<string,string|number>>)=>({en:fill(TASK_VIEW_EXECUTOR_TEXTS_V1.en[key],values),de:fill(TASK_VIEW_EXECUTOR_TEXTS_V1.de[key],values)});

/** 🧭️ Where one run of a task happens: the live tree or a pinned view, the environment that keeps its outputs at their declared locations, and how it came about. */
export type TaskRootRunV1=Readonly<{mode:"live"|"view";root:string;hash:string|null;environment:Readonly<Record<string,string>>;view:TaskViewV1|null}>;
export type TaskRootNoticeV1=Readonly<{kind:"repinned"|"pins-exhausted"|"refused"|"rerun"|"reruns-exhausted";en:string;de:string}>;
export type TaskRootRequestV1<T>=Readonly<{
 workspace:string;directory:string;targetDirectory:string;outputs?:readonly string[];taskClass?:string;environment?:TaskViewEnvironmentV1;viewPolicy?:TaskViewPolicyV1;
 paths():Promise<readonly string[]>;run(context:TaskRootRunV1):Promise<T>;
 digest?(source:string,paths:readonly string[]):Promise<readonly TaskViewInputV1[]>;open?(request:TaskViewRequestV1):Promise<TaskViewOutcomeV1>;
 notify?(notice:TaskRootNoticeV1):void;signal?:AbortSignal;policy?:typeof TASK_VIEW_EXECUTOR_POLICY_V1;
}>;
export type TaskRootResultV1<T>=Readonly<{value:T;mode:"live"|"view";root:string;hash:string|null;pins:number;reruns:number;fallback:"none"|"refused"|"pins-exhausted"|"reruns-exhausted"}>;

/** 🏃️ Runs a task on the root its class asks for. `live` (the default of every class but `readiness-probe`) calls `run` on the workspace with nothing changed. `view` hashes the declared input files, pins them (`moved`: hashes and pins again, at most `maximumPins` times, before any compile minute is spent), runs on the view with the shared target directory outside it, checks the view at the end (a hard-link view edited in place runs again on a new view, at most `maximumReruns` times) and answers for the start hash. A view that cannot be made, or inputs that never settle, run live with a notice: never a failure.
 * @see ../🟦️.ts */
export async function runOnTaskRootV1<T>(request:TaskRootRequestV1<T>):Promise<TaskRootResultV1<T>>{
 const policy=request.policy??TASK_VIEW_EXECUTOR_POLICY_V1,notify=request.notify??(()=>undefined);
 const live=async(fallback:TaskRootResultV1<T>["fallback"],pins:number,reruns:number):Promise<TaskRootResultV1<T>>=>({value:await request.run({mode:"live",root:request.workspace,hash:null,environment:{},view:null}),mode:"live",root:request.workspace,hash:null,pins,reruns,fallback});
 if(taskRootModeV1({taskClass:request.taskClass,environment:request.environment,policy:request.viewPolicy})==="live")return live("none",0,0);
 const digest=request.digest??digestTaskViewInputsV1,open=request.open??openTaskViewV1;
 let pins=0,reruns=0;
 for(;;){
  let ready:TaskViewV1|undefined,hash="";
  while(ready===undefined){
   if(pins>=policy.maximumPins){const notice=say("pinsExhausted",{maximum:policy.maximumPins});notify({kind:"pins-exhausted",...notice});return live("pins-exhausted",pins,reruns);}
   pins++;
   const inputs=await digest(request.workspace,await request.paths());hash=taskViewInputsDigestV1(inputs);
   const outcome=await open({source:request.workspace,directory:request.directory,hash,inputs,...request.viewPolicy?{policy:request.viewPolicy}:{},...request.signal?{signal:request.signal}:{}});
   if(outcome.kind==="ready")ready=outcome.view;
   else if(outcome.kind==="refused"){const notice=say("refused",{reason:outcome.en});notify({kind:"refused",en:notice.en,de:fill(TASK_VIEW_EXECUTOR_TEXTS_V1.de.refused,{reason:outcome.de})});return live("refused",pins,reruns);}
   else{const notice=say("repinned",{attempt:pins,maximum:policy.maximumPins,first:outcome.paths[0]??""});notify({kind:"repinned",...notice});}
  }
  const view=ready;
  for(const path of [request.targetDirectory,...request.outputs??[]]){
   const inside=relative(view.root,path);
   if(!inside.startsWith("..")&&!isAbsolute(inside)){await view.release();throw Object.assign(Error(say("outputInside",{path,root:view.root}).en+" | "+say("outputInside",{path,root:view.root}).de),{code:"task-view-output-inside",...say("outputInside",{path,root:view.root})});}
  }
  let value:T;
  try{value=await request.run({mode:"view",root:view.root,hash,environment:taskViewCargoEnvironmentV1({root:view.root,targetDirectory:request.targetDirectory}),view});}
  catch(error){await view.release();throw error;}
  const verdict=await view.verify();
  await view.release();
  if(verdict.ok)return{value,mode:"view",root:view.root,hash,pins,reruns,fallback:"none"};
  if(reruns>=policy.maximumReruns){const notice=say("rerunsExhausted",{maximum:policy.maximumReruns});notify({kind:"reruns-exhausted",...notice});return live("reruns-exhausted",pins,reruns);}
  reruns++;pins=0;
  const notice=say("rerun",{first:verdict.paths[0]??"",rerun:reruns,maximum:policy.maximumReruns});notify({kind:"rerun",...notice});
 }
}

