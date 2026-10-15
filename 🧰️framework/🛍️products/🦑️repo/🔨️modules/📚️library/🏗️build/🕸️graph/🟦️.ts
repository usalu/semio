import {repositoryBuildGraphDataDirectoryV1,withRepositoryBuildGraphFactStoreV1,withRepositoryBuildGraphSelectionV1,buildGraphWriterResourceV1} from "../📥️inputs/💾️storage/🟨️.mjs";
import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,type BuildGraphSelectionV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "./🚪️admission/🟦️.ts";
import { readdir, readFile, mkdir, writeFile, rename, lstat, rm } from "node:fs/promises";
import {readFileSync} from "node:fs";
import { join, dirname, relative, isAbsolute } from "node:path";
import {createRequire} from "node:module";
import { randomUUID, createHash } from "node:crypto";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { checkScriptInvocation, advanceScriptInvocation, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {acquireQueuedResourceLease} from "../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import type {RepositoryCacheStoreV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

/** 📖️ Reads each declaration through the same exact source fact boundary as inference. */
function graphDeclarationV1(path:string):any{return JSON.parse(createRequire(import.meta.url)("../👁️observation/🟨️.mjs").observedFs.readFileSync(new URL(path,import.meta.url),"utf8"));}

/** 🧾️ Binds loaded graph behavior and its publication declaration to their original bytes. */
export interface BuildGraphModuleAuthorityV1{implementation:string;publication:string}
const graphModuleAuthorityV1=():BuildGraphModuleAuthorityV1=>{const implementation=createHash("sha256");for(const path of[new URL(import.meta.url),new URL("./🚪️admission/🟦️.ts",import.meta.url),new URL("../🧬️schema/🚪️selection.json",import.meta.url),new URL("../🧬️schema/📋️owners.json",import.meta.url),new URL("../🧬️schema/🚪️admission.json",import.meta.url),new URL("../📥️inputs/💾️storage/🟨️.mjs",import.meta.url),new URL("../📥️inputs/💾️storage/🧬️schema/💾️quota.json",import.meta.url),new URL("../📥️inputs/💾️storage/📒️ledger/🟨️.mjs",import.meta.url),new URL("../📥️inputs/💾️storage/🧬️schema/📒️ledger.json",import.meta.url)])implementation.update(readFileSync(path));return {implementation:implementation.digest("hex"),publication:createHash("sha256").update(readFileSync(new URL("../🧬️schema/📣️publication.json",import.meta.url))).digest("hex")};};
const loadedGraphModuleAuthorityV1=graphModuleAuthorityV1();

/** 🔒️ Refuses a loaded implementation or declaration superseded while an original invocation waited. */
export function requireBuildGraphModuleAuthorityV1(original:BuildGraphModuleAuthorityV1,current:BuildGraphModuleAuthorityV1):void{
 const schema=graphDeclarationV1("../🧬️schema/📥️authority.json").properties.module;
 if(validateJsonSchemaSubset(schema,original).length||validateJsonSchemaSubset(schema,current).length||original.implementation!==current.implementation||original.publication!==current.publication)throw Error("Loaded build graph source changed; restart the owned build invocation "+JSON.stringify({loaded:original,current}));
}
const requireCurrentGraphModuleV1=():void=>requireBuildGraphModuleAuthorityV1(loadedGraphModuleAuthorityV1,graphModuleAuthorityV1());

/** 🧷️ Admits only source receipts issued by the exact loaded producer protocol. */
export function buildGraphProducerAuthorityV1(receipt:unknown,module:BuildGraphModuleAuthorityV1):boolean{
 if(validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/📥️authority.json"),receipt).length)return false;
 const admitted=receipt as {module:BuildGraphModuleAuthorityV1};return admitted.module.implementation===module.implementation&&admitted.module.publication===module.publication;
}

/** ⚙️ Describes portable process execution independently of any build implementation. */
export interface BuildExecutionV1 { kind: "command" | "noop"; command?: string; commands?: (string | {command:string;forwardAllArgs?:boolean})[]; args?:string[]; cwd?:string; env?:Record<string,string>; parallel?:boolean; forwardAllArgs?:boolean; [key:string]:unknown }
/** 🎯️ Declares one build operation and its configuration, inputs and products. */
export interface BuildTargetV1 { execution:BuildExecutionV1; configurations?:Record<string,Record<string,unknown>>; dependsOn?: (string | {target:string;projects?:string[]|string;dependencies?:boolean;params?:string})[]; inputs?:unknown[]; outputs?:string[]; cache?:boolean; continuous?:boolean; [key:string]:unknown }
/** 🧩️ Describes an owned project with its complete inferred operations. */
export interface BuildProjectV1 { name:string; root:string; targets:Record<string,BuildTargetV1>; tags?:string[]; namedInputs?:Record<string,unknown[]>; [key:string]:unknown }
/** 🕸️ Defines the authoritative portable workspace graph. */
export interface BuildGraphV1 { version:1; projects:Record<string,BuildProjectV1>; dependencies:{source:string;target:string;type?:string;sourceFile?:string}[]; namedInputs?:Record<string,unknown[]>; externalNodes?:Record<string,unknown>; unavailable?:BuildUnavailableProjectV1[]; admission?:{selection:BuildGraphSelectionV1;registry:string;closure:string} }
/** 🚧️ Names one project withheld from the snapshot because a source its inference consumed does not parse. */
export interface BuildUnavailableProjectV1 { root:string; project?:string; source:string; sha256:string; reason:string }

/** ✅️ Admits the language-neutral graph contract and cross-project identities. */
export function readBuildGraphV1(value:unknown):BuildGraphV1 {
  const errors=validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/🔣️.json"),value);
  if(errors.length)throw Error(`Invalid build graph: ${JSON.stringify(errors.slice(0,4))}`);
  const graph=value as BuildGraphV1;
  if(graph.admission)readBuildGraphSelectionV1(graph.admission.selection);
  const roots=new Map<string,string>();
  for(const[name,project]of Object.entries(graph.projects)){if(name!==project.name)throw Error(`Build project identity differs: ${name}`);const previous=roots.get(project.root);if(previous)throw Error(`Duplicate build project root ${project.root}: ${previous}, ${name}`);roots.set(project.root,name);}
  for(const edge of graph.dependencies)if(!graph.projects[edge.source]&&!graph.externalNodes?.[edge.source]||!graph.projects[edge.target]&&!graph.externalNodes?.[edge.target])throw Error(`Unknown build dependency ${edge.source} -> ${edge.target}`);
  return graph;
}

/** 🔄️ Removes implementation-specific executor names from an inferred operation. */
export function neutralBuildTargetV1(value:Record<string,any>,identity?:string):BuildTargetV1 {
  const {executor,command,options:declaredOptions={},configurations,...rest}=value;
  if(command!==undefined&&(typeof command!=="string"||declaredOptions.command!==undefined||declaredOptions.commands!==undefined))throw Error("Build command shorthand requires one command form");
  const options=command===undefined?declaredOptions:{...declaredOptions,command};
  requireBuildArgumentsV1(options,identity);
  if(executor!==undefined&&!["repo:script","repo:noop"].includes(executor))throw Error(`Unsupported project executor${identity?` for ${identity}`:""}: ${executor}`);
  const normalize=(configuration:Record<string,any>)=>{requireBuildConfigurationV1(options,configuration,identity);return {execution:configuration};};
  return {...rest,execution:{kind:executor==="repo:noop"?"noop":"command",...options},...(configurations?{configurations:Object.fromEntries(Object.entries(configurations).map(([name,configuration])=>[name,normalize(configuration as Record<string,any>)]))}:{})};
}


/** 🔐️ Keeps configured execution options inside the same task and command identity. */
function requireBuildConfigurationV1(base:Record<string,unknown>,configuration:Record<string,unknown>,identity?:string):void {
  if(!configuration||typeof configuration!=="object"||Array.isArray(configuration))throw Error("Build configuration execution options required");
  if(configuration.env!==undefined&&(!configuration.env||typeof configuration.env!=="object"||Array.isArray(configuration.env)||Object.values(configuration.env).some(value=>typeof value!=="string")))throw Error("Build configuration environment strings required");
  requireBuildArgumentsV1(configuration,identity);
  const fixed=["kind","ownerArguments","nativeOwnerCommand","executor","inputs","outputs","dependsOn","cache","continuous","parallelism","metadata","defaultConfiguration","configurations","options"];
  if(Object.keys(configuration).some(key=>fixed.includes(key)))throw Error("Build configuration cannot change task identity");
  if(configuration.command!==undefined&&(configuration.commands!==undefined||base.commands!==undefined)||configuration.commands!==undefined&&base.command!==undefined)throw Error("Build configuration cannot change command form");
}

/** 🔡️ Keeps authored argument boundaries literal through every backend's execution protocol. */
function requireBuildArgumentsV1(options:Record<string,unknown>,identity?:string):void {
  if(options.args!==undefined&&(!Array.isArray(options.args)||options.args.some(value=>typeof value!=="string")))throw Error(`Build execution arguments must be literal string arrays${identity?` for ${identity}`:""}`);
}

/** 🗃️ Locates the sole published portable graph. */
export function buildGraphPathV1(root:string):string { const publication=graphDeclarationV1("../🧬️schema/📣️publication.json").properties;return join(repositoryCachePartitionRootV1(root,publication.partition.const),publication.graph.const); }

/** 📋️ Describes observed root entries without granting mutation or storage custody. */
export interface BuildGraphInvalidationInventoryV1 {device:number;entries:{name:string;kind:"file"|"directory"|"symlink"|"other";links:number;bytes:number;device:number}[]}

/** 🗑️ Selects only owned current publications and their bounded staging files. */
export function planBuildGraphInvalidationV1(inventory:BuildGraphInvalidationInventoryV1):string[] {
  const schema=graphDeclarationV1("../🧬️schema/🗑️reset.json");if(validateJsonSchemaSubset(schema,inventory).length)throw Error("Build graph reset inventory required");
  const owned=new RegExp(schema.$defs.OwnedName.pattern,"u"),selected=inventory.entries.filter(entry=>owned.test(entry.name));
  for(const entry of selected)if(validateJsonSchemaSubset(schema.$defs.OwnedEntry,entry,schema).length||entry.device!==inventory.device)throw Error("Owned regular bounded graph publication required");
  return selected.map(entry=>entry.name).sort();
}

/** 🔒️ Serializes graph publication and invalidation inside the original enforcing store. */
async function withBuildGraphStateV1<T>(root:string,invocation:ScriptInvocation,body:(store:RepositoryCacheStoreV1)=>Promise<T>,scope?:string):Promise<T> {
  checkScriptInvocation(invocation);requireCurrentGraphModuleV1();
  const run=async(store:RepositoryCacheStoreV1)=>{
    const writer=await acquireQueuedResourceLease({directory:join(root,".cache/b/l/m/data"),resource:"build-graph-state:"+store.root+(scope?":selected:"+buildGraphWriterResourceV1(root,"selection",scope).split(":").at(-1):""),mode:"exclusive",owner:randomUUID(),control:store.control,signal:store.control.signal});
    try{requireCurrentGraphModuleV1();return await body(store);}finally{writer.release();}
  };
  return scope?withRepositoryBuildGraphSelectionV1(root,invocation,scope,run):withRepositoryBuildGraphFactStoreV1(root,invocation,run);
}

/** ♻️ Invalidates current graph authority while retaining charged storage, immutable pins and source facts. */
export async function invalidateBuildGraphV1(root:string,invocation:ScriptInvocation):Promise<void> {
  checkScriptInvocation(invocation);await advanceScriptInvocation(invocation,"build:reset","running");
  await withBuildGraphStateV1(root,invocation,async store=>{
    await store.recheck();if(store.root!==dirname(buildGraphPathV1(root)))throw Error("Original graph publication store required");
    const device=(await lstat(store.root)).dev,entries:BuildGraphInvalidationInventoryV1["entries"]=[];
    for(const name of await readdir(store.root)){
      checkScriptInvocation(invocation);const status=await lstat(join(store.root,name));entries.push({name,kind:status.isSymbolicLink()?"symlink":status.isFile()?"file":status.isDirectory()?"directory":"other",links:status.nlink,bytes:status.size,device:status.dev});
      if(entries.length%64===0)await advanceScriptInvocation(invocation,"build:reset-inventory","running");
    }
    for(const name of planBuildGraphInvalidationV1({device,entries})){
      checkScriptInvocation(invocation);await store.recheck();const path=requireNativeStoragePathV1(store.root,join(store.root,name)),status=await lstat(path);planBuildGraphInvalidationV1({device,entries:[{name,kind:status.isSymbolicLink()?"symlink":status.isFile()?"file":status.isDirectory()?"directory":"other",links:status.nlink,bytes:status.size,device:status.dev}]});await rm(path);await advanceScriptInvocation(invocation,"build:reset-publication","running");
    }
  });
}

const inventoryDirectoriesV1=new WeakMap<string[],string[]>();

/** 📁️ Discovers real workspace authority while yielding cancellation and progress. */
export async function workspaceBuildFilesV1(root:string,invocation?:ScriptInvocation):Promise<string[]> {
  const cachePolicy=graphDeclarationV1("../../⚡️caching/🔣️policy.json"),ownership=graphDeclarationV1("../../🗂️workspaces/🔣️policy.json"),excluded=new Set([...cachePolicy.generatedDirectories,".git",".🧬semio",".nx",".cache","node_modules","dist","target",".venv","venv","__pycache__",".pytest_cache",".next",".turbo","temp","🗑️generated"]),pending=[""],files:string[]=[],directories:string[]=[];
  let visited=0;
  while(pending.length){
    if(invocation)checkScriptInvocation(invocation);
    const batch=pending.splice(-32),listings=await Promise.all(batch.map(async directory=>{
      try{return {directory,entries:await readdir(join(root,directory),{withFileTypes:true})};}
      catch(error){if(directory&&["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw Object.assign(Error(`Build directory removed during discovery: ${directory}`),{code:"BUILD_MEMBERSHIP_CHANGED"});throw error;}
    }));
    if(invocation)checkScriptInvocation(invocation);
    for(const {directory,entries}of listings){
      directories.push(directory||".");
      for(const entry of entries){
        if(entry.isSymbolicLink())continue;
        const path=join(directory,entry.name).replaceAll("\\","/");
        if(entry.isDirectory()){if(!excluded.has(entry.name)&&!entry.name.startsWith("extension-host-build-")&&!(ownership.collections.names.includes(entry.name)&&directory.split("/").at(-1)!==ownership.collections.moduleMember)&&path!=="compose")pending.push(path);}
        else if(entry.isFile())files.push(path);
      }
    }
    const previous=visited;visited+=batch.length;if(Math.floor(visited/128)!==Math.floor(previous/128)&&invocation)await advanceScriptInvocation(invocation,"build:discover","running");
  }
  files.sort();inventoryDirectoriesV1.set(files,directories.sort());if(invocation)await advanceScriptInvocation(invocation,"build:discover","complete");return files;
}

/** 📥️ Expands private input compaction into portable source declarations. */
function portableInputs(root:string,value:unknown,invocation?:ScriptInvocation):unknown|Promise<unknown> {
  const token=(item:unknown)=>{
    if(!item||typeof item!=="object"||Array.isArray(item))return undefined;
    const object=item as Record<string,unknown>;if(Object.keys(object).length!==1||typeof object.env!=="string")return undefined;
    const match=/^SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})$/u.exec(object.env);
    return match?{key:object.env,digest:match[2]!,directory:match[1]==="SOURCE_CLOSURE"?"build-input-closures":"build-lock-closures"}:undefined;
  };
  const pending:unknown[]=[value],seen=new WeakSet<object>(),tokens=new Map<string,NonNullable<ReturnType<typeof token>>>();let visited=0;
  while(pending.length){
    if(invocation&&visited++%1024===0)checkScriptInvocation(invocation);
    const item=pending.pop();if(!item||typeof item!=="object"||seen.has(item))continue;seen.add(item);
    const compact=token(item);if(compact)tokens.set(compact.key,compact);else for(const child of Object.values(item))if(child&&typeof child==="object")pending.push(child);
  }
  if(!tokens.size)return value;
  const expand=async()=>{
    const entries=[...tokens.values()],expanded=new Map<string,string[]>(),read=createRequire(import.meta.url)("../👁️observation/🟨️.mjs").observedFs.readFile;let cursor=0,stopped=false;
    const workers=Array.from({length:Math.min(16,entries.length)},async()=>{
      while(!stopped&&cursor<entries.length){
        if(invocation)checkScriptInvocation(invocation);
        const compact=entries[cursor++]!,bytes=await read(join(repositoryBuildGraphDataDirectoryV1(root,compact.directory as "build-input-closures"|"build-lock-closures"),compact.digest+".json"),"utf8");
        if(invocation)checkScriptInvocation(invocation);
        if(createHash("sha256").update(bytes).digest("hex")!==compact.digest)throw Error(`Build input closure content differs: ${compact.key}`);
        const contract=JSON.parse(bytes);expanded.set(compact.key,compact.directory==="build-input-closures"?contract.files.map((file:string)=>"{workspaceRoot}/"+file):["{workspaceRoot}/Cargo.lock","{workspaceRoot}/"+contract.manifest]);
      }
    });
    await Promise.all(workers).catch(async error=>{stopped=true;await Promise.allSettled(workers);throw error;});
    const copies=new WeakMap<object,unknown>();visited=0;
    const transform=(item:unknown):unknown=>{
      if(invocation&&visited++%1024===0)checkScriptInvocation(invocation);
      if(!item||typeof item!=="object")return item;
      const compact=token(item);if(compact)return expanded.get(compact.key)!;
      if(copies.has(item))return copies.get(item);
      if(Array.isArray(item)){
        let result:unknown[]|undefined;
        for(let index=0;index<item.length;index++){const child=item[index],next=transform(child),flatten=token(child)!==undefined;if(!result&&(flatten||next!==child))result=item.slice(0,index);if(result){if(flatten)for(const entry of next as unknown[])result.push(entry);else result.push(next);}}
        copies.set(item,result??item);return result??item;
      }
      let result:Record<string,unknown>|undefined;
      for(const[key,child]of Object.entries(item)){const next=transform(child);if(next!==child&&!result)result={...item};if(result)result[key]=next;}
      copies.set(item,result??item);return result??item;
    };
    return transform(value);
  };
  return expand();
}

/** 🪪️ Refuses caller registry data superseded before the current source boundary. */
export function requireBuildOwnerRegistryAuthorityV1(original:unknown,current:unknown):void{
 if(JSON.stringify(readBuildOwnerRegistryV1(original))!==JSON.stringify(readBuildOwnerRegistryV1(current)))throw Object.assign(Error("Original build owner registry changed before source consumption"),{code:"BUILD_SOURCE_AUTHORITY"});
}

/** 📚️ Assembles all owned inference contributions without installing or invoking Nx. */
export async function discoverBuildGraphV1(root:string,selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1,invocation?:ScriptInvocation,suppliedFiles?:string[],admission?:BuildOwnerAdmissionV1):Promise<BuildGraphV1> {
  selection=readBuildGraphSelectionV1(selection);registry=readBuildOwnerRegistryV1(registry);
  requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(createRequire(import.meta.url)("../👁️observation/🟨️.mjs").observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8")));
  if(selection.kind==="tasks"&&!admission)throw Error("Closed selected owner admission required");
  const files=suppliedFiles??(selection.kind==="all"?buildGraphMembersV1(await workspaceBuildFilesV1(root,invocation)):admission!.files);
  const require=createRequire(import.meta.url),owner=require("./📥️inference/🟨️.mjs"),tests=require("../../../🧪️test/🟨️.mjs"),policies=require("./📥️inference/🛡️policy/🟨️.mjs");
  const policy=graphDeclarationV1("../🔣️policy.json"),options={analyzeLockfile:true,catalogProviders:selection.kind==="all"?registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})):admission!.catalogProviders,...(selection.kind==="tasks"?{ownerAdmission:admission}:{})};
  const primary=await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||file==="bun.lock"||file.endsWith(".patch")),options,{workspaceRoot:root,invocation,registry});
  if(invocation)await advanceScriptInvocation(invocation,"build:infer","running");
  const contributions=[...primary,...await tests.discoverTestBuildProjectsV1(files.filter(file=>file.endsWith(".feature")),{}, {workspaceRoot:root,invocation})];
  const policyOwners=Object.assign({},...contributions.map(([,contribution])=>contribution.projects??{}));
  contributions.push(...await policies.default.createNodesV2[1](files.filter(file=>file.endsWith("📜️script.ts")),{}, {workspaceRoot:root,projects:policyOwners}));
  const projects:Record<string,any>={},externalNodes:Record<string,unknown>={},unavailable=new Map<string,BuildUnavailableProjectV1>();
  for(const[,contribution]of contributions){
    Object.assign(externalNodes,contribution.externalNodes);
    for(const row of (contribution.unavailable??[]) as BuildUnavailableProjectV1[])unavailable.set(row.root,row);
    for(const[name,project]of Object.entries(contribution.projects??{}) as [string,any][]){
      if(projects[name]&&projects[name].root!==project.root)throw Error(`Duplicate build project ${name}`);
      projects[name]={...projects[name],...project,name,targets:{...projects[name]?.targets,...project.targets}};
    }
  }
  const ordered=Object.entries(projects).sort(([,a],[,b])=>b.root.length-a.root.length),projectFileMap:Record<string,{file:string}[]>=Object.fromEntries(Object.keys(projects).map(name=>[name,[]]));
  const withheld=[...unavailable.keys()];
  for(const file of files){const match=ordered.find(([,project])=>project.root==="."||file.startsWith(project.root+"/"));if(match&&!withheld.some(root=>file.startsWith(root+"/")&&root.length>match[1].root.length)){const entries=projectFileMap[match[0]];if(!entries)throw Error("Discovered project file owner is missing");entries.push({file});}}
  const context={workspaceRoot:root,projects,fileMap:{projectFileMap},invocation,selection,registry,completeSourceImports:selection.kind==="tasks"};
  const edges=[...await owner.discoverBuildDependenciesV1(options,context),...await tests.discoverTestBuildDependenciesV1({},context)];
  const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{name,type:"lib",data}]))},options,context);
  for(const[name,value]of Object.entries(metadata) as [string,any][])projects[name].metadata={...projects[name].metadata,...value.metadata,semio:{...projects[name].metadata?.semio,...value.metadata?.semio}};
  const neutral=Object.fromEntries(Object.entries(projects).map(([name,project])=>[name,{...project,targets:Object.fromEntries(Object.entries(project.targets??{}).map(([target,value])=>[target,neutralBuildTargetV1(value as Record<string,any>,`${name}:${target}`)]))}]));
  const dependencies=[...new Map(edges.map(edge=>[JSON.stringify(edge),edge])).values()].sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)));
  if(invocation)await advanceScriptInvocation(invocation,"build:graph","complete");
  return readBuildGraphV1({version:1,projects:await portableInputs(root,neutral,invocation),dependencies,namedInputs:policy.namedInputs,externalNodes,...(unavailable.size?{unavailable:[...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0)}:{}),admission:{selection,registry:createHash("sha256").update(JSON.stringify(registry)).digest("hex"),closure:createHash("sha256").update(JSON.stringify(admission??files)).digest("hex")}});
}

/** 🧺️ Keeps exactly the discovered files whose presence the portable graph consumes, as declared by the build policy. */
export function buildGraphMembersV1(files:readonly string[]):string[] {
  const {basenames,suffixes}=graphDeclarationV1("../🔣️policy.json").graphMembers as {basenames:string[];suffixes:string[]},names=new Set(basenames);
  return files.filter(file=>names.has(file.slice(file.lastIndexOf("/")+1))||suffixes.some(suffix=>file.endsWith(suffix)));
}

/** 🧾️ Identifies complete admitted physical membership, types and modes independently of implementation bytes. */
export async function buildGraphMembershipV1(root:string,files:string[],invocation?:ScriptInvocation):Promise<string> {
  const records:string[]=[],entries=[...files.map(file=>({file,directory:false})),...(inventoryDirectoriesV1.get(files)??[]).map(file=>({file,directory:true}))].filter(({file})=>![".nxignore","nx.json"].includes(file)).sort((a,b)=>a.file.localeCompare(b.file));
  for(let index=0;index<entries.length;index+=64){
    if(invocation)checkScriptInvocation(invocation);
    records.push(...await Promise.all(entries.slice(index,index+64).map(async({file,directory})=>{let info;try{info=await lstat(join(root,file));}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw Object.assign(Error(`Build member removed during discovery: ${file}`),{code:"BUILD_MEMBERSHIP_CHANGED"});throw error;}if(info.isSymbolicLink()||(directory?!info.isDirectory():!info.isFile()))throw Object.assign(Error(`Build membership changed during discovery: ${file}`),{code:"BUILD_MEMBERSHIP_CHANGED"});return JSON.stringify([file,info.isDirectory()?"directory":"file",info.mode]);})));
  }
  return createHash("sha256").update(records.join("\n")).digest("hex");
}

/** 🔏️ Admits content-aware observations through the same owned inference mechanism. */
export async function buildGraphSourceFactsHoldV1(observations:unknown[],invocation?:ScriptInvocation,onStale?:(row:{kind:string;path:string;fact:unknown})=>void):Promise<boolean>{return createRequire(import.meta.url)("../👁️observation/🟨️.mjs").observationsHold(observations,64,new Set(),new Set(),invocation?.control.signal,onStale);}

/** 📥️ Resolves one candidate while keeping every candidate's memory within its own finite attempt. */
async function loadBuildGraphAttemptV1(root:string,invocation:ScriptInvocation|undefined,options:{selection:BuildGraphSelectionV1;reuse?:boolean},registry?:BuildOwnerRegistryV1,scope?:string):Promise<BuildGraphV1|undefined> {
  requireCurrentGraphModuleV1();
  const profile=process.env.SEMIO_NX_PROFILE_INPUTS==="1",timing=(phase:string,started:number,details:Record<string,unknown>={})=>{if(profile)console.log("[DEBUG] Build graph Source phase "+JSON.stringify({phase,milliseconds:performance.now()-started,...details}));};
  const image=async(source:string)=>{let started=performance.now();const decoded=JSON.parse(source);timing("graph-json-parse",started,{bytes:Buffer.byteLength(source)});started=performance.now();const expanded=await portableInputs(root,decoded,invocation);timing("graph-input-expand",started);started=performance.now();const graph=readBuildGraphV1(expanded);timing("graph-schema-validation",started,{projects:Object.keys(graph.projects).length,dependencies:graph.dependencies.length});return graph;};
  if(options.reuse){
    const pin={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST};if(validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/📥️pin.json"),pin).length||!isAbsolute(pin.path!))throw Error("Original immutable build graph path and digest required");
    const source=await readFile(pin.path!,"utf8"),digest=createHash("sha256").update(source).digest("hex");
    if(digest!==pin.digest)throw Error("Pinned build graph digest differs from original authority");
    const graph=await image(source);if(!graph.admission||JSON.stringify(readBuildGraphSelectionV1(graph.admission.selection))!==JSON.stringify(readBuildGraphSelectionV1(options.selection)))throw Error("Original selected build graph admission required");return graph;
  }
  if(!registry)throw Error("Original canonical graph owner registry required");
  const selected=options.selection.kind==="tasks",path=scope?join(dirname(buildGraphPathV1(root)),"selected",scope+".json"):buildGraphPathV1(root),authorityPath=scope?join(dirname(path),scope+".authority.json"):join(dirname(path),graphDeclarationV1("../🧬️schema/📣️publication.json").properties.authority.const);
  const observation=createRequire(import.meta.url)("../👁️observation/🟨️.mjs"),producer=createRequire(import.meta.url)("./📥️inference/🟨️.mjs");createRequire(import.meta.url)("../../../🧪️test/🟨️.mjs");createRequire(import.meta.url)("./📥️inference/🛡️policy/🟨️.mjs");
  const inventory=async()=>{let started=performance.now();const captured=selected?await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{const current=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(buildGraphScopeKeyV1(options.selection,current)!==scope)throw Error("Original selected owner registry changed");const nativeCache=new Map(),nativeClosures=new Map();return closeBuildOwnerAdmissionV1(options.selection,registry!,async(owner,targets,inputs)=>{if(invocation)checkScriptInvocation(invocation);return producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,invocation,selection:options.selection,nativeCache,nativeClosures},inputs);});},{signal:invocation?.control.signal})):undefined,admission=captured?.value as BuildOwnerAdmissionV1|undefined,files=admission?.files??buildGraphMembersV1(await workspaceBuildFilesV1(root,invocation));timing("workspace-catalog-lookup",started,{files:files.length,directories:inventoryDirectoriesV1.get(files)?.length??0});started=performance.now();const digest=await buildGraphMembershipV1(root,files,invocation);timing("workspace-membership-hash",started);return {files,digest,admission,observations:captured?.observations??[]};};
  let initial:Awaited<ReturnType<typeof inventory>>;try{initial=await inventory();}catch(error){if((error as {code?:string}).code==="BUILD_MEMBERSHIP_CHANGED")return undefined;throw error;}
  const {files,digest:before}=initial;
  let phaseStarted=performance.now();
  await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();timing("source-parser-bootstrap",phaseStarted);
  let cachedReceipt:any;try{cachedReceipt=JSON.parse(await readFile(authorityPath,"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"&&!(error instanceof SyntaxError))throw error;}
  if(cachedReceipt)try{
    phaseStarted=performance.now();timing("source-authority-read-parse",phaseStarted,{observations:cachedReceipt.observations?.length??0});phaseStarted=performance.now();
    const cachedReceiptValid=buildGraphProducerAuthorityV1(cachedReceipt,loadedGraphModuleAuthorityV1)&&cachedReceipt.membership===before;timing("source-authority-schema",phaseStarted,{admitted:cachedReceiptValid});phaseStarted=performance.now();const factsHold=cachedReceiptValid&&await buildGraphSourceFactsHoldV1(cachedReceipt.observations,invocation);timing("source-authority-facts",phaseStarted,{holds:factsHold});
    if(factsHold){requireCurrentGraphModuleV1();
      phaseStarted=performance.now();const source=await readFile(path,"utf8");timing("graph-file-read",phaseStarted,{bytes:Buffer.byteLength(source)});phaseStarted=performance.now();const digest=createHash("sha256").update(source).digest("hex");timing("graph-content-hash",phaseStarted);if(digest===cachedReceipt.graph){const graph=await image(source);requireCurrentGraphModuleV1();if(selected&&JSON.stringify(graph.admission?.selection)!==JSON.stringify(options.selection))throw Error("Original selected publication scope differs");if(await buildGraphSourceFactsHoldV1([...initial.observations,...cachedReceipt.observations],invocation))return graph;return undefined;}
    }
  }catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  let captured:{value:BuildGraphV1;observations:unknown[]},failedFacts:unknown[]=[],failedAuthority:unknown[]=[];
  phaseStarted=performance.now();
  try{captured=await observation.captureObservedSourceInputsV1(async()=>{
    for(const source of [new URL(import.meta.url),new URL("../🧬️schema/📣️publication.json",import.meta.url),new URL("../🧬️schema/🔣️.json",import.meta.url),new URL("../🧬️schema/📥️authority.json",import.meta.url),new URL("../🔣️policy.json",import.meta.url),new URL("../../⚡️caching/🔣️policy.json",import.meta.url),new URL("../../🗂️workspaces/🔣️policy.json",import.meta.url),new URL("../📤️arguments/🧬️schema/🔣️.json",import.meta.url),new URL("../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json",import.meta.url),new URL("../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🧬️schema/🔣️.json",import.meta.url)])observation.observedFs.readFileSync(source);
    const admittedRegistry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(JSON.stringify(admittedRegistry)!==JSON.stringify(registry))throw Error("Original canonical owner registry changed");
    for(const source of [new URL("../🧬️schema/🚪️selection.json",import.meta.url),new URL("../🧬️schema/📋️owners.json",import.meta.url),new URL("../🧬️schema/🚪️admission.json",import.meta.url),new URL("./🚪️admission/🟦️.ts",import.meta.url)])observation.observedFs.readFileSync(source);
    return discoverBuildGraphV1(root,options.selection,registry!,invocation,files,initial.admission);
  },(rows:unknown[],authority:unknown[])=>{failedFacts=rows;failedAuthority=authority;});}
  catch(error){if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true")console.error("[DEBUG] original graph inference failure "+String(error));if(invocation)checkScriptInvocation(invocation);if((error as {code?:string}).code==="BUILD_AUTHORITY_CHANGED")throw error;let current;try{current=await inventory();}catch(recheck){if((recheck as {code?:string}).code==="BUILD_MEMBERSHIP_CHANGED")return undefined;throw recheck;}const stale:{kind:string;path:string;fact:unknown}[]=[],holds=await buildGraphSourceFactsHoldV1(failedFacts,invocation,row=>{if(stale.length<8)stale.push(row);});if(current.digest===before&&holds)throw error;if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true")console.error("[DEBUG] invalidated graph inference "+JSON.stringify({error:String(error),membershipChanged:current.digest!==before,stale:stale.map(row=>({kind:row.kind,path:row.path,observed:failedFacts.filter((original:any)=>original.kind===row.kind&&original.path===row.path).slice(0,4).map((original:any)=>({fact:original.fact,moduleLoad:failedAuthority.some((loaded:any)=>loaded.kind===original.kind&&loaded.path===original.path&&JSON.stringify(loaded.fact)===JSON.stringify(original.fact))}))}))}));return undefined;}
  timing("graph-original-inference",phaseStarted,{observations:captured.observations.length});
  let final:Awaited<ReturnType<typeof inventory>>;try{final=await inventory();}catch(error){if((error as {code?:string}).code==="BUILD_MEMBERSHIP_CHANGED")return undefined;throw error;}
  const after=final.digest;
  captured.observations=[...initial.observations,...captured.observations];
  const stale:{kind:string;path:string;fact:unknown}[]=[],holds=await buildGraphSourceFactsHoldV1(captured.observations,invocation,row=>{if(stale.length<8)stale.push(row);});
  if(selected&&JSON.stringify(initial.admission)!==JSON.stringify(final.admission))return undefined;
  if(before!==after||!holds){if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true"){const oldFiles=new Set(files),newFiles=new Set(final.files);console.error("[DEBUG] invalidated completed graph "+JSON.stringify({membershipChanged:before!==after,added:final.files.filter(path=>!oldFiles.has(path)).slice(0,8),removed:files.filter(path=>!newFiles.has(path)).slice(0,8),stale}));}return undefined;}
  const graph=captured.value,source=JSON.stringify(graph),digest=createHash("sha256").update(source).digest("hex"),authority=JSON.stringify({version:3,module:loadedGraphModuleAuthorityV1,membership:after,graph:digest,observations:captured.observations});if(Buffer.byteLength(source)>536870912||Buffer.byteLength(authority)>536870912)throw Error("Build graph cache exceeds its finite publication limit");await mkdir(dirname(path),{recursive:true});
  const temporary=path+"."+randomUUID(),receipt=authorityPath+"."+randomUUID();try{await writeFile(temporary,source,{flag:"wx"});await writeFile(receipt,authority,{flag:"wx"});if(invocation)checkScriptInvocation(invocation);requireCurrentGraphModuleV1();if(!await buildGraphSourceFactsHoldV1(captured.observations,invocation))return undefined;await rename(temporary,path);await rename(receipt,authorityPath);}finally{await Promise.all([rm(temporary,{force:true}),rm(receipt,{force:true})]);}
  return graph;
}
/** 💾️ Publishes only a current graph and releases each invalidated candidate before retrying. */
export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation|undefined,options:{selection:BuildGraphSelectionV1;reuse?:boolean}):Promise<BuildGraphV1> {
  if(invocation)checkScriptInvocation(invocation);
  options={...options,selection:readBuildGraphSelectionV1(options.selection)};
  if(options.reuse)return await loadBuildGraphAttemptV1(root,invocation,options)??Promise.reject(Error("Reusable build graph unavailable"));
  if(!invocation)throw Error("Original build graph producer invocation required");
  const registry=readBuildOwnerRegistryV1(JSON.parse(createRequire(import.meta.url)("./📥️inference/🟨️.mjs").readBuildOwnerRegistrySourceV1(root))),scope=options.selection.kind==="tasks"?buildGraphScopeKeyV1(options.selection,registry):undefined;
  return withBuildGraphStateV1(root,invocation,async store=>{
  const staging=scope?join(store.root,"selected"):store.root;await store.recheck();requireNativeStoragePathV1(store.root,staging);await mkdir(staging,{recursive:true});const selectedStage=scope?new RegExp("^"+scope+"(?:\\.authority)?\\.json\\.[a-f0-9-]{36}$","u"):undefined;
  for(const name of await readdir(staging)){if(!(selectedStage?.test(name)??/^(?:canonical|authority)\.json\.[a-f0-9-]{36}$/u.test(name)))continue;await store.recheck();const path=join(staging,name),status=await lstat(path);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>536870912)throw Error("Owned bounded graph staging file required");await rm(path);}
  for(let attempt=0;attempt<=8;attempt++){
    if(invocation)checkScriptInvocation(invocation);
    await store.recheck();
    const currentRegistry=options.selection.kind==="all"?readBuildOwnerRegistryV1(JSON.parse(await readFile(join(root,"📋️build-owners.json"),"utf8"))):registry;
    const graph=await loadBuildGraphAttemptV1(root,invocation,options,currentRegistry,scope);if(graph)return graph;
    if(invocation)await advanceScriptInvocation(invocation,"build:refresh","running");
  }
  throw Error("Build source authority keeps changing during inference");
  },scope);
}

/** 🎚️ Resolves one configuration with environment inherited from the declared execution. */
export function resolveBuildTargetV1(target:BuildTargetV1,configuration?:string):BuildTargetV1 {
  if(!configuration)return target;
  const configured=target.configurations?.[configuration];
  if(!configured)throw Error(`Unknown build configuration: ${configuration}`);
  if(Object.keys(configured).some(key=>key!=="execution")||!configured.execution)throw Error("Build configuration execution options required");
  const overrides=configured.execution as Partial<BuildExecutionV1>;requireBuildConfigurationV1(target.execution,overrides);
  return {...target,execution:{...target.execution,...overrides,env:{...target.execution.env,...overrides.env}}};
}


/** 📥️ Expands owned named inputs while preserving explicit dependency and runtime declarations. */
export function resolveBuildInputsV1(graph:BuildGraphV1,project:string|BuildProjectV1,inputs:readonly unknown[]):unknown[] {
  const owner=typeof project==="string"?graph.projects[project]:project;
  if(!owner)throw Error(`Unknown build input owner: ${project}`);
  const named={...graph.namedInputs,...owner.namedInputs as Record<string,unknown[]>|undefined},active=new Set<string>();
  const expand=(values:readonly unknown[]):unknown[]=>values.flatMap(value=>{
    if(typeof value!=="string"||!Object.hasOwn(named,value))return [value];
    if(active.has(value))throw Error(`Build named input cycle: ${[...active,value].join(" -> ")}`);
    active.add(value);const result=expand(named[value]!);active.delete(value);return result;
  });
  return expand(inputs);
}
