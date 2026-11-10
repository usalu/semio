import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {buildPatternV1,selectBuildProjectsV1} from "../🟦️.ts";
import type {BuildGraphV1} from "../../🕸️graph/📋️model/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
/** 🎯️ One requested task. */
export type TargetRefV1={readonly project:string;readonly target:string};
/** 🎯️ Declares which tasks are demanded and whether the tools running them are part of the scope. */
export type TargetScopeV1={readonly version:1;readonly targets:readonly TargetRefV1[];readonly includeToolClosure:boolean};
/** 🧱️ One prerequisite declaration of a target as authored. */
export type ScopeDependencyV1=string|{readonly target:string;readonly projects?:readonly string[]|string;readonly dependencies?:boolean};
/** 🧩️ The only facts planning needs of one project, read on demand. */
export type ScopeNodeV1={readonly targets:Readonly<Record<string,{readonly dependsOn?:readonly ScopeDependencyV1[]}>>;readonly dependencies:readonly string[];readonly tools?:readonly string[]};
/** 🔌️ Lazy access to project nodes: `select` resolves selector patterns over names only, `node` is the single read that is counted. */
export interface ScopeSourceV1{select(patterns:readonly string[]):readonly string[];node(project:string,signal?:AbortSignal):Promise<ScopeNodeV1|undefined>}
/** 📋️ The planning result of one scope: its closure and nothing else. */
export type ScopedPlanV1={readonly version:1;readonly scope:TargetScopeV1;readonly projects:readonly string[];readonly tasks:readonly TargetRefV1[];readonly edges:readonly {readonly from:TargetRefV1;readonly to:TargetRefV1}[];readonly projectsRead:number};
/** 📈️ Cooperative controls of one planning run. */
export type ScopedPlanOptionsV1={readonly signal?:AbortSignal;readonly maximumTasks?:number;readonly progress?:(update:{readonly tasks:number;readonly projectsRead:number})=>void};

const key=(project:string,target:string)=>project+"\0"+target,ref=(value:string):TargetRefV1=>{const split=value.indexOf("\0");return{project:value.slice(0,split),target:value.slice(split+1)};};
const compare=(a:string,b:string)=>a<b?-1:a>b?1:0;

/** ✅️ Admits only a closed scope. */
export function readTargetScopeV1(value:unknown):TargetScopeV1{
 if(validateJsonSchemaSubset(schema,value,schema).length)throw Error("Closed target scope required\nGeschlossener Zielbereich erforderlich");
 return structuredClone(value) as TargetScopeV1;
}

/** 🕸️ Views a whole graph as a lazy source, so a scoped plan can be compared with the plan of everything. */
export function scopeSourceOfGraphV1(graph:BuildGraphV1):ScopeSourceV1{
 const edges=new Map<string,string[]>();for(const edge of graph.dependencies){const rows=edges.get(edge.source)??[];rows.push(edge.target);edges.set(edge.source,rows);}
 return{select:patterns=>selectBuildProjectsV1(graph,patterns),node:async project=>{const row=graph.projects[project];return row&&{targets:row.targets,dependencies:edges.get(project)??[],...(Array.isArray(row.tools)?{tools:row.tools as string[]}:{})};}};
}

/** 🧭️ Plans exactly the closure of the scope: projects are read when a walk reaches them, so unrelated projects are never touched; the result equals the restriction of the whole-workspace plan to the scope. */
export async function planScopedTargetsV1(source:ScopeSourceV1,scopeValue:unknown,options:ScopedPlanOptionsV1={}):Promise<ScopedPlanV1>{
 const scope=readTargetScopeV1(scopeValue),limit=options.maximumTasks??65536,nodes=new Map<string,ScopeNodeV1|undefined>(),tasks=new Set<string>(),edges=new Set<string>(),queue:string[]=[],bridges=new Set<string>();
 const read=async(project:string)=>{options.signal?.throwIfAborted();if(!nodes.has(project)){nodes.set(project,await source.node(project,options.signal));options.progress?.({tasks:tasks.size,projectsRead:nodes.size});}return nodes.get(project);};
 const add=async(project:string,target:string,from?:string,required=false)=>{
  const node=await read(project);if(!node?.targets[target]){if(required)throw Error("Selected task missing: "+project+":"+target+"\nGewählte Aufgabe fehlt: "+project+":"+target);return;}
  const id=key(project,target);if(from!==undefined)edges.add(from+"\u0001"+id);if(tasks.has(id))return;if(tasks.size>=limit)throw Error("Scoped task capacity exceeded\nKapazität der Aufgaben im Bereich überschritten");tasks.add(id);queue.push(id);
 };
 const through=async(project:string,pattern:string,from:string)=>{
  const pending=[...(await read(project))?.dependencies??[]];
  for(let index=0;index<pending.length;index++){
   const owner=pending[index]!,id=owner+"\0"+pattern+"\0"+from;if(bridges.has(id))continue;bridges.add(id);
   const node=await read(owner),targets=Object.keys(node?.targets??{}).filter(target=>buildPatternV1(pattern,target));
   if(targets.length)for(const target of targets)await add(owner,target,from);else pending.push(...node?.dependencies??[]);
  }
 };
 for(const {project,target} of scope.targets)await add(project,target,undefined,true);
 for(let index=0;index<queue.length;index++){
  const id=queue[index]!,{project,target}=ref(id),node=(await read(project))!;
  for(const dependency of node.targets[target]!.dependsOn??[]){
   if(typeof dependency==="string"){
    if(dependency.startsWith("^")){await through(project,dependency.slice(1),id);continue;}
    const split=dependency.indexOf(":"),named=split>=0&&await read(dependency.slice(0,split))!==undefined,owner=named?dependency.slice(0,split):project,pattern=named?dependency.slice(split+1):dependency;
    for(const candidate of Object.keys((await read(owner))?.targets??{}))if(buildPatternV1(pattern,candidate))await add(owner,candidate,id);
   }else if(dependency.dependencies)await through(project,dependency.target,id);
   else for(const owner of dependency.projects===undefined?[project]:source.select(Array.isArray(dependency.projects)?dependency.projects:[dependency.projects as string]))for(const candidate of Object.keys((await read(owner))?.targets??{}))if(buildPatternV1(dependency.target,candidate))await add(owner,candidate,id);
  }
  if(index%64===63)await Promise.resolve();
 }
 const projects=new Set<string>(),visit=[...tasks].map(id=>ref(id).project);
 for(let index=0;index<visit.length;index++){const name=visit[index]!;if(projects.has(name))continue;projects.add(name);visit.push(...(await read(name))?.dependencies??[]);}
 if(scope.includeToolClosure)for(let index=0;index<visit.length;index++){const name=visit[index]!,node=await read(name);for(const tool of node?.tools??[])if(!projects.has(tool))visit.push(tool);if(!projects.has(name)&&node)projects.add(name);for(const dependency of node?.dependencies??[])if(!projects.has(dependency))visit.push(dependency);}
 const sort=(values:Iterable<string>)=>[...values].sort(compare);
 const plan:ScopedPlanV1={version:1,scope,projects:sort(projects).filter(name=>nodes.get(name)),tasks:sort(tasks).map(ref),edges:sort(edges).map(row=>{const[from,to]=row.split("\u0001");return{from:ref(from!),to:ref(to!)};}),projectsRead:[...nodes.values()].filter(Boolean).length};
 if(validateJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/ScopedPlan"},plan).length)throw Error("Closed scoped plan required");
 return plan;
}
