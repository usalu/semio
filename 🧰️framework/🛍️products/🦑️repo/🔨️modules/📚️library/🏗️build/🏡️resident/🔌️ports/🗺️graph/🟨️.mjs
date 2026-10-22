/** 🗺 Maps the neutral build graph (projects, targets, dependencies) onto the owners and tasks the resident state tracks. Pure; no tool names. */
import {canonicalJsonV1} from "../../📨️protocol/🔑️capability/🟨️.mjs";

const ROOT_TOKENS=[["{workspaceRoot}/",""],["{workspaceRoot}",""]];

function outputPath(raw,project){
 if(typeof raw!=="string")return undefined;
 let text=raw.split("{projectRoot}").join(project.root).split("{projectName}").join(project.name);
 for(const[token,replacement]of ROOT_TOKENS)text=text.split(token).join(replacement);
 text=text.replace(/^\.\//u,"").replace(/\/+$/u,"");
 if(text===""||text.startsWith("/")||text.split("/").includes("..")||/^[A-Za-z]:/u.test(text)||text.includes("{")||text.includes("*"))return undefined;
 return text;
}

function dependsOnIds(entry,project,dependencies,graph){
 const exists=(owner,target)=>graph.projects[owner]?.targets?.[target]!==undefined;
 const own=target=>exists(project.name,target)?[project.name+":"+target]:[];
 const deps=target=>dependencies.get(project.name)?.filter(dep=>exists(dep,target)).map(dep=>dep+":"+target)??[];
 if(typeof entry==="string")return entry.startsWith("^")?deps(entry.slice(1)):own(entry);
 if(entry&&typeof entry==="object"&&typeof entry.target==="string"){
  const out=[];
  if(entry.dependencies===true)out.push(...deps(entry.target));
  const projects=entry.projects===undefined?(entry.dependencies===true?[]:["self"]):Array.isArray(entry.projects)?entry.projects:[entry.projects];
  for(const name of projects){
   if(name==="self")out.push(...own(entry.target));
   else if(name==="dependencies")out.push(...deps(entry.target));
   else if(exists(name,entry.target))out.push(name+":"+entry.target);
  }
  return out;
 }
 return[];
}

/** 🗺 Builds the state configuration from a neutral graph: owners with direct dependencies and tasks with dependsOn ids, outputs and cacheability. */
export function stateConfigurationFromBuildGraphV1(graph,{environmentPaths=[],environmentPrefixes=[]}={}){
 const dependencies=new Map();
 for(const edge of graph.dependencies??[])if(graph.projects[edge.source]&&graph.projects[edge.target]&&edge.source!==edge.target){
  const list=dependencies.get(edge.source)??[];if(!list.includes(edge.target))list.push(edge.target);dependencies.set(edge.source,list);
 }
 const owners=[];
 for(const name of Object.keys(graph.projects).sort()){
  const project=graph.projects[name],tasks=[];
  for(const target of Object.keys(project.targets??{}).sort()){
   const definition=project.targets[target],ids=new Set();
   for(const entry of definition.dependsOn??[])for(const id of dependsOnIds(entry,project,dependencies,graph))if(id!==name+":"+target)ids.add(id);
   tasks.push({name:target,command:[canonicalJsonV1({execution:definition.execution,configurations:definition.configurations??null})],dependsOn:[...ids].sort(),outputs:(definition.outputs??[]).map(raw=>outputPath(raw,project)).filter(Boolean).sort(),cacheable:definition.cache!==false&&definition.continuous!==true});
  }
  owners.push({name,root:project.root==="."?"":project.root.replace(/\/+$/u,""),deps:[...(dependencies.get(name)??[])].sort(),tasks});
 }
 return{owners,env:{paths:environmentPaths,prefixes:environmentPrefixes}};
}

/** 🔗 The projects a client that named `named` may have built on its behalf: every project reachable from them through task dependencies. */
export function taskClosureV1(graph,named){
 const tasks=new Map(),projectOf=id=>id.split(":")[0];
 for(const owner of stateConfigurationFromBuildGraphV1(graph).owners)for(const task of owner.tasks)tasks.set(owner.name+":"+task.name,task.dependsOn);
 const projects=new Set(named),pending=[...tasks.keys()].filter(id=>projects.has(projectOf(id))),seen=new Set(pending);
 while(pending.length){
  for(const next of tasks.get(pending.pop())??[]){projects.add(projectOf(next));if(!seen.has(next)){seen.add(next);pending.push(next);}}
 }
 return projects;
}

/** 🔌 The graph port of the service over a loader that returns a neutral graph and a salt. */
export function createBuildGraphPortV1({loadGraph,salt,listing,environmentPaths,environmentPrefixes,generation}){
 return{
  async load(){const graph=await loadGraph();return{generation:generation?.(graph)??null,...stateConfigurationFromBuildGraphV1(graph,{environmentPaths,environmentPrefixes})};},
  async environmentDigest(){return salt();},
  listing,
 };
}
