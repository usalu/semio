import { readdir, readFile, mkdir, writeFile, rename, lstat } from "node:fs/promises";
import { join, dirname, relative } from "node:path";
import { randomUUID, createHash } from "node:crypto";
import schema from "../🧬️schema/🔣️.json";
import policy from "../🔣️policy.json";
import cachePolicy from "../../⚡️caching/🔣️policy.json";
import ownership from "../../🗂️workspaces/🔣️policy.json";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { checkScriptInvocation, advanceScriptInvocation, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** ⚙️ Describes portable process execution independently of any build implementation. */
export interface BuildExecutionV1 { kind: "command" | "noop"; command?: string; commands?: (string | {command:string;forwardAllArgs?:boolean})[]; cwd?:string; env?:Record<string,string>; parallel?:boolean; forwardAllArgs?:boolean; [key:string]:unknown }
/** 🎯️ Declares one build operation and its configuration, inputs and products. */
export interface BuildTargetV1 { execution:BuildExecutionV1; configurations?:Record<string,Record<string,unknown>>; dependsOn?: (string | {target:string;projects?:string[]|string;dependencies?:boolean;params?:string})[]; inputs?:unknown[]; outputs?:string[]; cache?:boolean; continuous?:boolean; [key:string]:unknown }
/** 🧩️ Describes an owned project with its complete inferred operations. */
export interface BuildProjectV1 { name:string; root:string; targets:Record<string,BuildTargetV1>; [key:string]:unknown }
/** 🕸️ Defines the authoritative portable workspace graph. */
export interface BuildGraphV1 { version:1; projects:Record<string,BuildProjectV1>; dependencies:{source:string;target:string;type?:string;sourceFile?:string}[]; namedInputs?:Record<string,unknown[]>; externalNodes?:Record<string,unknown> }

/** ✅️ Admits the language-neutral graph contract and cross-project identities. */
export function readBuildGraphV1(value:unknown):BuildGraphV1 {
  const errors=validateJsonSchemaSubset(schema,value);
  if(errors.length)throw Error(`Invalid build graph: ${JSON.stringify(errors.slice(0,4))}`);
  const graph=value as BuildGraphV1;
  for(const[name,project]of Object.entries(graph.projects))if(name!==project.name)throw Error(`Build project identity differs: ${name}`);
  for(const edge of graph.dependencies)if(!graph.projects[edge.source]&&!graph.externalNodes?.[edge.source]||!graph.projects[edge.target]&&!graph.externalNodes?.[edge.target])throw Error(`Unknown build dependency ${edge.source} -> ${edge.target}`);
  return graph;
}

/** 🔄️ Removes implementation-specific executor names from an inferred operation. */
export function neutralBuildTargetV1(value:Record<string,any>):BuildTargetV1 {
  const {executor,options={},configurations,...rest}=value;
  if(executor!==undefined&&!["nx:run-commands","repo:script","repo:noop","nx:noop","@semio-tech/repo-lib:owner-command"].includes(executor))throw Error(`Unsupported project executor: ${executor}`);
  const normalize=(configuration:Record<string,any>)=>{
    const {options:configOptions,...properties}=configuration,keys=new Set(["outputs","inputs","dependsOn","cache","continuous","metadata","defaultConfiguration","configurations","description"]),target:Record<string,unknown>={},execution:Record<string,unknown>={...configOptions};
    for(const[key,value]of Object.entries(properties))if(keys.has(key))target[key]=value;else execution[key]=value;
    return {...target,execution};
  };
  return {...rest,execution:{kind:(executor==="nx:noop"||executor==="repo:noop")?"noop":"command",...options,...(executor==="@semio-tech/repo-lib:owner-command"?{ownerArguments:true}:{})},...(configurations?{configurations:Object.fromEntries(Object.entries(configurations).map(([name,configuration])=>[name,normalize(configuration as Record<string,any>)]))}:{})};
}

/** 🔌️ Projects the portable operation onto Nx's command executor contract. */
export function nxBuildTargetV1(value:BuildTargetV1):Record<string,unknown> {
  const {execution,configurations,...rest}=value,{kind,ownerArguments,...options}=execution;
  return {...rest,executor:kind==="noop"?"nx:noop":ownerArguments?"@semio-tech/repo-lib:owner-command":"nx:run-commands",...(kind==="command"?{options}:{}),...(configurations?{configurations:Object.fromEntries(Object.entries(configurations).map(([name,value])=>[name,value.execution?{...Object.fromEntries(Object.entries(value).filter(([key])=>key!=="execution")),...value.execution as object}:value]))}:{})};
}

/** 🗃️ Locates the sole published portable graph. */
export function buildGraphPathV1(root:string):string { return join(root,".🧬semio/🦑️repo/⚡️cache/build/graph.json"); }

/** 📁️ Discovers real workspace authority while yielding cancellation and progress. */
export async function workspaceBuildFilesV1(root:string,invocation?:ScriptInvocation):Promise<string[]> {
  const excluded=new Set([...cachePolicy.generatedDirectories,".git",".🧬semio",".nx",".cache","node_modules","dist","target",".venv","venv","__pycache__",".pytest_cache",".next",".turbo","temp","🗑️generated"]),pending=[""],files:string[]=[];
  let visited=0;
  while(pending.length){
    if(invocation)checkScriptInvocation(invocation);
    const directory=pending.pop()!;
    for(const entry of await readdir(join(root,directory),{withFileTypes:true})){
      if(entry.isSymbolicLink())continue;
      const path=join(directory,entry.name).replaceAll("\\","/");
      if(entry.isDirectory()){if(!excluded.has(entry.name)&&!entry.name.startsWith("extension-host-build-")&&!(ownership.collections.names.includes(entry.name)&&directory.split("/").at(-1)!==ownership.collections.moduleMember)&&path!=="compose")pending.push(path);}
      else if(entry.isFile())files.push(path);
    }
    if(++visited%128===0&&invocation)await advanceScriptInvocation(invocation,"build:discover","running");
  }
  return files.sort();
}

/** 📥️ Expands private input compaction into portable source declarations. */
async function portableInputs(root:string,value:unknown,memo=new Map<string,Promise<unknown>>()):Promise<any> {
  if(Array.isArray(value)){const values=await Promise.all(value.map(item=>portableInputs(root,item,memo)));return values.flat();}
  if(!value||typeof value!=="object")return value;
  const object=value as Record<string,unknown>,environment=typeof object.env==="string"?object.env:undefined,source=environment?.match(/^SEMIO_SOURCE_CLOSURE_([a-f0-9]{64})$/u),lock=environment?.match(/^SEMIO_CARGO_LOCK_([a-f0-9]{64})$/u);
  if(source||lock){
    const key=environment!;
    if(!memo.has(key))memo.set(key,(async()=>{
      const path=join(root,".🧬semio/🦑️repo/⚡️cache",source?"nx-input-closures":"nx-lock-closures",(source??lock)![1]+".json"),contract=JSON.parse(await readFile(path,"utf8"));
      return source?contract.files.map((file:string)=>"{workspaceRoot}/"+file):["{workspaceRoot}/Cargo.lock","{workspaceRoot}/"+contract.manifest];
    })());
    return memo.get(key)!;
  }
  return Object.fromEntries(await Promise.all(Object.entries(object).map(async([key,item])=>[key,await portableInputs(root,item,memo)])));
}

/** 📚️ Assembles all owned inference contributions without installing or invoking Nx. */
export async function discoverBuildGraphV1(root:string,invocation?:ScriptInvocation,suppliedFiles?:string[]):Promise<BuildGraphV1> {
  const files=suppliedFiles??await workspaceBuildFilesV1(root,invocation);
  const owner=await import("./📥️inference/🟨️.mjs"),tests=await import("../../../🧪️test/🟨️.mjs"),policies=await import("../../🔌️nx-plugin/🟨️.mjs");
  const options={analyzeLockfile:true,catalogProviders:policy.catalogProviders};
  const primary=await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||file==="bun.lock"||file.endsWith(".patch")),options,{workspaceRoot:root});
  if(invocation)await advanceScriptInvocation(invocation,"build:infer","running");
  const contributions=[...primary,...await tests.discoverTestBuildProjectsV1(files.filter(file=>file.endsWith(".feature")),{}, {workspaceRoot:root}),...await policies.default.createNodesV2[1](files.filter(file=>file.endsWith("📜️script.ts")),{}, {workspaceRoot:root})];
  const projects:Record<string,any>={},externalNodes:Record<string,unknown>={};
  for(const[,contribution]of contributions){
    Object.assign(externalNodes,contribution.externalNodes);
    for(const[name,project]of Object.entries(contribution.projects??{}) as [string,any][]){
      if(projects[name]&&projects[name].root!==project.root)throw Error(`Duplicate build project ${name}`);
      projects[name]={...projects[name],...project,name,targets:{...projects[name]?.targets,...project.targets}};
    }
  }
  const ordered=Object.entries(projects).sort(([,a],[,b])=>b.root.length-a.root.length),projectFileMap:Record<string,{file:string}[]>=Object.fromEntries(Object.keys(projects).map(name=>[name,[]]));
  for(const file of files){const match=ordered.find(([,project])=>project.root==="."||file.startsWith(project.root+"/"));if(match)projectFileMap[match[0]].push({file});}
  const context={workspaceRoot:root,projects,fileMap:{projectFileMap}};
  const edges=[...await owner.discoverBuildDependenciesV1(options,context),...await tests.discoverTestBuildDependenciesV1({},context)];
  const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{name,type:"lib",data}]))},options,context);
  for(const[name,value]of Object.entries(metadata) as [string,any][])projects[name].metadata={...projects[name].metadata,...value.metadata,semio:{...projects[name].metadata?.semio,...value.metadata?.semio}};
  const neutral=Object.fromEntries(Object.entries(projects).map(([name,project])=>[name,{...project,targets:Object.fromEntries(Object.entries(project.targets??{}).map(([target,value])=>[target,neutralBuildTargetV1(value as Record<string,any>)]))}]));
  const dependencies=[...new Map(edges.map(edge=>[JSON.stringify(edge),edge])).values()].sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)));
  if(invocation)await advanceScriptInvocation(invocation,"build:graph","complete");
  return readBuildGraphV1({version:1,projects:await portableInputs(root,neutral),dependencies,namedInputs:policy.namedInputs,externalNodes});
}

/** 🧾️ Identifies current physical source membership and revision without parsing project code. */
async function buildAuthority(root:string,files:string[],invocation?:ScriptInvocation):Promise<string> {
  const records:string[]=[];
  for(let index=0;index<files.length;index+=64){
    if(invocation)checkScriptInvocation(invocation);
    const batch=await Promise.all(files.slice(index,index+64).map(async file=>{const info=await lstat(join(root,file));if(!info.isFile()||info.isSymbolicLink())throw Error(`Build authority changed during discovery: ${file}`);return /(?:\.(?:[cm]?[jt]sx?|json|toml|lock|ya?ml|go|rs|py|cs|graphql|proto|feature)|(?:^|\/)(?:go\.mod|go\.sum|CMakeLists\.txt))$/u.test(file)?JSON.stringify([file,info.size,info.mtimeMs,info.ctimeMs,info.mode]):JSON.stringify([file,info.mode]);}));
    records.push(...batch);
  }
  return createHash("sha256").update(records.join("\n")).digest("hex");
}

/** 📌️ Pins a complete graph snapshot for concurrent backend consumers. */
export async function pinBuildGraphV1(root:string,graph:BuildGraphV1):Promise<{path:string;digest:string}> {
  const source=JSON.stringify(graph),digest=createHash("sha256").update(source).digest("hex"),path=join(root,".🧬semio/🦑️repo/⚡️cache/build/graphs",digest+".json");
  await mkdir(dirname(path),{recursive:true});
  try{await writeFile(path,source,{flag:"wx"});}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}
  return {path,digest};
}

/** 💾️ Validates source authority before reusing or atomically publishing the portable graph. */
export async function loadBuildGraphV1(root:string,invocation?:ScriptInvocation,options:{reuse?:boolean;attempt?:number}={}):Promise<BuildGraphV1> {
  const path=buildGraphPathV1(root),authorityPath=join(dirname(path),"authority.json");
  let existing:BuildGraphV1|undefined;
  try{existing=readBuildGraphV1(await portableInputs(root,JSON.parse(await readFile(path,"utf8"))));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  if(existing&&options.reuse)return existing;
  const files=await workspaceBuildFilesV1(root,invocation),before=await buildAuthority(root,files,invocation);
  if(existing){try{const receipt=JSON.parse(await readFile(authorityPath,"utf8")),digest=createHash("sha256").update(JSON.stringify(existing)).digest("hex");if(receipt.version===1&&receipt.authority===before&&receipt.graph===digest)return existing;}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}}
  const graph=await discoverBuildGraphV1(root,invocation,files),afterFiles=await workspaceBuildFilesV1(root,invocation),after=await buildAuthority(root,afterFiles,invocation);
  if(before!==after){if((options.attempt??0)<2){if(invocation)await advanceScriptInvocation(invocation,"build:refresh","running");return loadBuildGraphV1(root,invocation,{attempt:(options.attempt??0)+1});}throw Error("Build source authority keeps changing during inference");}
  const snapshot=await pinBuildGraphV1(root,graph);
  await mkdir(dirname(path),{recursive:true});
  const temporary=path+"."+randomUUID();
  await writeFile(temporary,JSON.stringify(graph));await rename(temporary,path);
  const receipt=authorityPath+"."+randomUUID();await writeFile(receipt,JSON.stringify({version:1,authority:after,graph:snapshot.digest}));await rename(receipt,authorityPath);
  return graph;
}
/** 🎚️ Resolves one configuration with environment inherited from the declared execution. */
export function resolveBuildTargetV1(target:BuildTargetV1,configuration?:string):BuildTargetV1 {
  if(!configuration)return target;
  const configured=target.configurations?.[configuration];
  if(!configured)throw Error(`Unknown build configuration: ${configuration}`);
  const {execution={},...properties}=configured;
  const overrides=execution as Partial<BuildExecutionV1>;
  return {...target,...properties,execution:{...target.execution,...overrides,env:{...target.execution.env,...overrides.env}}};
}
