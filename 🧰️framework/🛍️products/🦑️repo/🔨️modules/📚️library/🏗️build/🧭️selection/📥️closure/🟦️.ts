import type { BuildGraphV1, BuildTargetV1 } from "../../🕸️graph/📋️model/🟦️.ts";
import {resolveBuildTargetV1} from "../../🕸️graph/🟦️.ts";
import {buildPatternV1,selectBuildProjectsV1} from "../🟦️.ts";
import {checkScriptInvocation,advanceScriptInvocation,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
/** 🎯️ Declares one finite prerequisite walk under an explicit compiler intent. */
export interface BuildPrerequisiteRequestV1{readonly version:1;readonly tasks:readonly string[];readonly mode:"prototype"|"release";readonly excludeDependencies:boolean;readonly maximumTasks:number;}
/** 🧱️ Keeps the requested compiler intent and effective owned target together. */
export interface SelectedBuildPrerequisiteV1{readonly project:string;readonly target:string;readonly mode:"prototype"|"release";readonly declaration:BuildTargetV1;readonly effective:BuildTargetV1;}
/** 🕸️ Traverses actual task prerequisites, including dependency projects without the requested target. */
export async function selectedBuildPrerequisitesV1(graph:BuildGraphV1,request:BuildPrerequisiteRequestV1,original:ScriptInvocation):Promise<readonly SelectedBuildPrerequisiteV1[]>{
 if(validateJsonSchemaSubset(schema,request).length)throw Error("Complete prerequisite request required");checkScriptInvocation(original);
 const queue:SelectedBuildPrerequisiteV1[]=[],seen=new Set<string>(),bridges=new Set<string>(),edges=new Map<string,string[]>();for(const edge of graph.dependencies){const owners=edges.get(edge.source)??[];owners.push(edge.target);edges.set(edge.source,owners);}
 const add=(project:string,target:string,mode:string,required=false)=>{const declaration=graph.projects[project]?.targets[target];if(!declaration){if(required)throw Error("Selected prerequisite task missing: "+project+":"+target);return;}if(mode!=="prototype"&&mode!=="release")throw Error("Explicit prerequisite mode required");const key=project+":"+target+":"+mode;if(seen.has(key))return;if(seen.size>=request.maximumTasks)throw Error("Selected prerequisite capacity exceeded");seen.add(key);queue.push({project,target,mode,declaration,effective:declaration.configurations?.[mode]?resolveBuildTargetV1(declaration,mode):declaration});};
 for(const task of request.tasks){const parts=task.split(":");if(parts.length<2||parts.length>3||parts.some(part=>!part))throw Error("Selected prerequisite task missing: "+task);add(parts[0]!,parts[1]!,parts[2]??request.mode,true);}
 if(request.excludeDependencies)return queue;
 const dependencyTargets=(project:string,pattern:string,mode:"prototype"|"release")=>{const pending=[...(edges.get(project)??[])];for(let index=0;index<pending.length;index++){checkScriptInvocation(original);const owner=pending[index]!,key=owner+"\0"+pattern+"\0"+mode;if(bridges.has(key))continue;if(bridges.size>=request.maximumTasks)throw Error("Selected prerequisite bridge capacity exceeded");bridges.add(key);const targets=Object.keys(graph.projects[owner]?.targets??{}).filter(target=>buildPatternV1(pattern,target));if(targets.length)for(const target of targets)add(owner,target,mode);else pending.push(...(edges.get(owner)??[]));}};
 for(let index=0;index<queue.length;index++){
  checkScriptInvocation(original);const row=queue[index]!;
  for(const dependency of row.effective.dependsOn??[]){
   if(typeof dependency==="string"){
    if(dependency.startsWith("^")){dependencyTargets(row.project,dependency.slice(1),row.mode);continue;}
    const split=dependency.indexOf(":"),owner=split>=0&&graph.projects[dependency.slice(0,split)]?dependency.slice(0,split):row.project,pattern=owner===row.project&&!(split>=0&&graph.projects[dependency.slice(0,split)])?dependency:dependency.slice(split+1);
    for(const target of Object.keys(graph.projects[owner]?.targets??{}))if(buildPatternV1(pattern,target))add(owner,target,row.mode);
   }else if(dependency.dependencies)dependencyTargets(row.project,dependency.target,row.mode);
   else{const patterns=Array.isArray(dependency.projects)?dependency.projects:typeof dependency.projects==="string"?[dependency.projects]:[row.project];for(const owner of selectBuildProjectsV1(graph,patterns))for(const target of Object.keys(graph.projects[owner]!.targets))if(buildPatternV1(dependency.target,target))add(owner,target,row.mode);}
  }
  if(index%64===63)await advanceScriptInvocation(original,"build:prerequisite-closure","running");
 }
 checkScriptInvocation(original);return queue;
}
