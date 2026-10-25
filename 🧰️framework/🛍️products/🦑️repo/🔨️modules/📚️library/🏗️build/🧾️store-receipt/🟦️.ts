import type { BuildGraphV1 } from "../🕸️graph/📋️model/🟦️.ts";
import {selectBuildTasksV1} from "../🧭️selection/🟦️.ts";
import {selectedBuildPrerequisitesV1} from "../🧭️selection/📥️closure/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkStoreReceiptFileV1,declaredStoreReceiptV1,type StoreReceiptModeV1,type StoreReceiptVerdictV1} from "./🟨️.mjs";

/** 🧾️ One selected task that declares a store receipt, with the task identity a re-execution selects. */
export interface StoreReceiptTaskV1{readonly id:string;readonly receipt:string}
/** 📋️ Keeps a first-run failure beside the receipt state it left behind. */
interface RunOutcomeV1{readonly failed:boolean;readonly error?:unknown}

/** 🔎️ Lists the tasks a `run` or `run-many` selection reaches that declare a store receipt. */
export async function storeReceiptTasksV1(root:string,graph:BuildGraphV1,args:readonly string[],invocation:ScriptInvocation,mode:"prototype"|"release"):Promise<StoreReceiptTaskV1[]>{
 const boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary);
 if(!["run","run-many"].includes(args[0]??"")||head.some(argument=>argument==="--dry-run"||argument==="--dry-run=true"||argument==="--graph"||argument.startsWith("--graph=")))return [];
 const selection=await selectBuildTasksV1(root,graph,args,invocation);
 if(!selection.tasks.length)return [];
 const rows=await selectedBuildPrerequisitesV1(graph,{version:1,tasks:selection.tasks,mode,excludeDependencies:selection.excludeDependencies,maximumTasks:65536},invocation),tasks=new Map<string,StoreReceiptTaskV1>();
 for(const row of rows){
  const project=graph.projects[row.project]!,receipt=declaredStoreReceiptV1({workspaceRoot:root,projectRoot:project.root,target:row.target,outputs:row.effective.outputs});
  if(receipt!==undefined){const id=row.project+":"+row.target+(row.declaration.configurations?.[row.mode]?":"+row.mode:"");tasks.set(id,{id,receipt});}
 }
 return [...tasks.values()];
}

/** 🩺️ Verifies the receipt each task left behind against its store. */
export async function checkStoreReceiptTasksV1(root:string,tasks:readonly StoreReceiptTaskV1[],mode:StoreReceiptModeV1="digest"):Promise<ReadonlyMap<string,StoreReceiptVerdictV1>>{
 const verdicts=new Map<string,StoreReceiptVerdictV1>();
 for(const task of tasks)verdicts.set(task.id,await checkStoreReceiptFileV1(root,task.receipt,mode));
 return verdicts;
}

const describe=(id:string,verdict:StoreReceiptVerdictV1)=>`${id}: ${verdict.state}${verdict.findings.length?" ("+verdict.findings.slice(0,3).map(row=>row.reason+" "+row.path).join(", ")+(verdict.findings.length>3?", ...":"")+")":""}`;

/** 🔁️ Treats a cache hit whose receipt names missing or changed store content as a miss: the task re-executes uncached and replaces its cache entry. */
export async function withStoreReceiptsV1(options:{readonly root:string;readonly tasks:readonly StoreReceiptTaskV1[];readonly run:()=>Promise<void>;readonly repair:(task:StoreReceiptTaskV1)=>Promise<void>;readonly mode?:StoreReceiptModeV1;readonly report?:(line:string)=>void}):Promise<void>{
 const report=options.report??((line:string)=>console.log("[store-receipt] "+line)),execute=async():Promise<RunOutcomeV1>=>{try{await options.run();return {failed:false};}catch(error){return {failed:true,error};}};
 if(!options.tasks.length){await options.run();return;}
 const first=await execute(),verdicts=await checkStoreReceiptTasksV1(options.root,options.tasks,options.mode),stale=options.tasks.filter(task=>{const verdict=verdicts.get(task.id)!;return verdict.state==="invalid"||verdict.state==="malformed"||verdict.state==="unwritten"&&!first.failed;});
 if(!stale.length){if(first.failed)throw first.error;return;}
 for(const task of stale){report("re-executing "+describe(task.id,verdicts.get(task.id)!));await options.repair(task);}
 const repaired=await checkStoreReceiptTasksV1(options.root,stale,options.mode),broken=stale.filter(task=>repaired.get(task.id)!.state!=="valid");
 if(broken.length)throw Error("Declared store receipt still not valid after re-execution: "+broken.map(task=>describe(task.id,repaired.get(task.id)!)).join("; "));
 if(first.failed){const second=await execute();if(second.failed)throw second.error;}
}
