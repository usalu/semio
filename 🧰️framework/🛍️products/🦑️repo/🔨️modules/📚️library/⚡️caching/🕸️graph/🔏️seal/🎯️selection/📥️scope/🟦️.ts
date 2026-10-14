import {createHash} from "node:crypto";
import {readFile} from "node:fs/promises";
import {join} from "node:path";
import {validateSelectedGraphValueV1,type SelectedGraphScopeV1} from "../🟦️.ts";

/** 🎯️ One public run selection is frozen before any reuse proof. */
export interface SelectedTaskSelectionV1 {readonly project:string;readonly target:string;readonly configuration:"prototype"|"release";readonly excludeTaskDependencies:boolean;}
/** 🔌️ Installed Nx resolves task edges and expanded input contracts without leaking SDK types. */
export interface SelectedTaskPlannerV1 {tasks(graph:unknown,selection:SelectedTaskSelectionV1,nx:unknown):readonly {id:string;project:string;target:string}[];inputs(graph:unknown,project:string,target:string,nx:unknown,input?:string):Readonly<{self:readonly Readonly<{fileset?:string;env?:string}>[];dependencies:readonly string[];projectInputs:number;dependencyFilesets:readonly string[]}>;}

/** 📥️ Scopes original native task/dependency inputs, refusing an orchestration-wide glob. */
export async function selectedTaskScopeV1(root:string,value:unknown,selection:SelectedTaskSelectionV1,planner:SelectedTaskPlannerV1,signal?:AbortSignal):Promise<SelectedGraphScopeV1|null>{
 validateSelectedGraphValueV1("TaskSelectionV1",selection);
 const graph=value as {nodes:Record<string,{data:{root:string;targets:Record<string,unknown>}}> ;dependencies:Record<string,readonly {target:string}[]>},nx=JSON.parse(await readFile(join(root,"nx.json"),"utf8"));
 if(!graph?.nodes?.[selection.project]?.data?.targets?.[selection.target])return null;
 const tasks=planner.tasks(graph,selection,nx),roots=new Set<string>(),files=new Set<string>(),visited=new Set<string>();
 const addFileset=(value:string,project:string):boolean=>{if(value.startsWith("!"))return true;const owner=graph.nodes[project]?.data.root;if(!owner)return false;const normalized=value.replaceAll("{projectRoot}",owner).replaceAll("{workspaceRoot}/","");if(normalized.includes("{"))return false;const index=normalized.search(/[*?[!]/u),prefix=index<0?normalized:normalized.slice(0,index);if(index<0)files.add(prefix);else{const path=prefix.endsWith("/")?prefix.slice(0,-1):prefix.slice(0,prefix.lastIndexOf("/"));if(!path||path===".")return false;roots.add(path);}return true;};
 const inputs=async(project:string,target:string,input?:string):Promise<boolean>=>{
  signal?.throwIfAborted();const identity=project+"\0"+target+"\0"+(input??"");if(visited.has(identity))return true;visited.add(identity);
  const expanded=planner.inputs(graph,project,target,nx,input);if(expanded.projectInputs)return false;
  for(const row of expanded.self){if(row.fileset&&!addFileset(row.fileset,project))return false;const match=row.env?.match(/^SEMIO_SOURCE_CLOSURE_([a-f0-9]{64})$/u);if(match){const bytes=await readFile(join(root,".🧬semio/🦑️repo/⚡️cache/nx-input-closures",match[1]+".json"));if(createHash("sha256").update(bytes).digest("hex")!==match[1])throw Error("Original source input inventory custody changed");const value=JSON.parse(bytes.toString());if(value.version!==1||!Array.isArray(value.files)||Object.keys(value).length!==2)throw Error("Original source inventory required");for(const file of value.files)files.add(file);}}
  for(const edge of graph.dependencies[project]??[]){if(!graph.nodes[edge.target])continue;for(const dependencyInput of expanded.dependencies)if(!await inputs(edge.target,target,dependencyInput))return false;for(const fileset of expanded.dependencyFilesets)if(!addFileset(fileset,edge.target))return false;}
  return true;
 };
 for(const task of tasks){const path=graph.nodes[task.project]?.data.root;if(!path||path===".")return null;roots.add(path);if(!await inputs(task.project,task.target))return null;}
 const ordered=[...roots].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))),minimal=ordered.filter((path,index)=>!ordered.some((ancestor,other)=>other!==index&&path.startsWith(ancestor+"/"))),scope:SelectedGraphScopeV1={version:1,tasks:tasks.map(task=>task.id).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))),roots:minimal,files:[...files].filter(path=>!minimal.some(owner=>path===owner||path.startsWith(owner+"/"))).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)))};validateSelectedGraphValueV1("ScopeV1",scope);return scope;
}
