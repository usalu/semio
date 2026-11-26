import type {NativeBuildJobV1,NativeBuildPlanV1} from "../📋️contract/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
const policy=structuredClone(schema);
/** 🗺️ Orders finite job-only prerequisites in declaration-stable dependency waves. */
export function nativeBuildPrerequisiteOrderV1(jobs:readonly Readonly<{id:string;after:readonly string[]}>[]):readonly string[]{
 const rows=jobs.map(({id,after})=>({id,after:[...after]}));if(validateJsonSchemaSubset(policy,rows).length)throw Error("Finite job-only prerequisite graph required");const ids=new Set(rows.map(row=>row.id));if(ids.size!==rows.length||rows.some(row=>row.after.some(dependency=>!ids.has(dependency.slice(4)))))throw Error("Exact unique prerequisite job dependencies required");const remaining=new Map(rows.map(row=>[row.id,new Set(row.after.map(value=>value.slice(4)))])),ordered:string[]=[];
 while(remaining.size){const ready=rows.filter(row=>remaining.get(row.id)?.size===0);if(!ready.length)throw Error("Acyclic prerequisite job graph required");for(const row of ready){remaining.delete(row.id);ordered.push(row.id);}for(const dependencies of remaining.values())for(const row of ready)dependencies.delete(row.id);}
 return Object.freeze(ordered);
}

/** 🚦 Finds the jobs required before Cargo input capture, given jobs in dependency order. */
export function nativeBuildPassPrerequisitesV1<T extends Readonly<{id:string;after:readonly string[];lane?:string}>>(jobs:readonly T[],stageAfter:Readonly<Record<string,readonly string[]>>):readonly T[]{
 const stageJobs=new Set(Object.values(stageAfter).flat().filter(name=>name.startsWith("job:")).map(name=>name.slice(4)));
 const passJobs=new Set(jobs.filter(job=>job.lane!==undefined||!stageJobs.has(job.id)).map(job=>job.id));
 for(const job of [...jobs].reverse())if(passJobs.has(job.id))for(const dependency of job.after)passJobs.add(dependency.slice(4));
 return jobs.filter(job=>passJobs.has(job.id));
}

/** 🛫 Keeps independently admitted repository jobs beside the component on multi-core hosts and before it on a single-core host. */
export function nativeBuildIndependentJobPlanV1(plan:NativeBuildPlanV1,cores:number):Readonly<{plan:NativeBuildPlanV1;before:readonly NativeBuildJobV1[];independentJobs:boolean}>{
 const jobs=plan.jobs??[],owners=jobs.filter(job=>job.command[0]==="repo");
 if(cores>1||!owners.length)return Object.freeze({plan,before:Object.freeze([]),independentJobs:owners.length>0});
 const byId=new Map(owners.map(job=>[job.id,job]));
 const completed=new Set(owners.map(job=>`job:${job.id}`));
 if(owners.some(job=>job.after.some(dependency=>!completed.has(dependency))))throw Error("Single-core repository prerequisites need their own prerequisites before compiler admission");
 const before=nativeBuildPrerequisiteOrderV1(jobs).flatMap(id=>{const owner=byId.get(id);return owner?[owner]:[];});
 const after=(dependencies:readonly string[])=>dependencies.filter(dependency=>!completed.has(dependency));
 const remaining=jobs.filter(job=>!byId.has(job.id)).map(job=>({...job,after:after(job.after)}));
 const stageAfter=Object.fromEntries(Object.entries(plan.stageAfter??{}).map(([stage,dependencies])=>[stage,after(dependencies)]));
 return Object.freeze({plan:{...plan,jobs:remaining,stageAfter},before:Object.freeze(before),independentJobs:false});
}
