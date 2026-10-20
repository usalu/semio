import type graphSnapshotPolicy from"../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json";
import type exactReadSchema from"../📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json";
import{captureBoundedSourceRevisionV1,capturePhysicalSourceTextV1,physicalSourceOriginV1,type PhysicalSourceModuleMetaV1}from"../📥️inputs/🔏️revision/🟨️.mjs";
import{fileURLToPath}from"node:url";
import*as observationV1 from"../👁️observation/🟨️.mjs";
import{withManagedGraphSourceReadBatchV1,readExactSourceInventoryV1}from"../📥️inputs/🧮️digest/📥️read/🟨️.mjs";
import{captureInferenceInventoryReferencesV1,admitInferenceInventoryCohortV1,planInferenceInventoryCohortsV1}from"../👁️observation/📥️reuse/🟨️.mjs";
import{requireCurrentSourceInventoryV1,captureSourceInputInventoryV1,captureCargoLockInventoryV1,admitExactSourceExtentV1}from"../📥️inputs/📋️data/📥️load/🟨️.mjs";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withRepositoryBuildGraphConsumerV1,withRepositoryBuildGraphFactStoreV1,withRepositoryBuildGraphSelectionV1,buildGraphWriterResourceV1} from "../📥️inputs/💾️storage/🟨️.mjs";
import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,type BuildGraphSelectionV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "./🚪️admission/🟦️.ts";
import { readdir, readFile, open, mkdir, writeFile, rename, lstat, rm } from "node:fs/promises";
import {constants} from "node:fs";
import{withArtifactDescriptorV1}from"../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import { join, dirname, relative, isAbsolute } from "node:path";
import { randomUUID, randomBytes, createHash } from "node:crypto";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { checkScriptInvocation, advanceScriptInvocation, requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {acquireQueuedResourceLease} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {RepositoryCacheStoreV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

const graphSnapshotPolicyURLV1=new URL("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),exactReadPolicyURLV1=new URL("../📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),graphSnapshotPolicyCaptureV1=captureBoundedSourceRevisionV1(fileURLToPath(graphSnapshotPolicyURLV1),65536,true),exactReadPolicyCaptureV1=captureBoundedSourceRevisionV1(fileURLToPath(exactReadPolicyURLV1),65536,true);
const graphSnapshotPolicyV1:typeof graphSnapshotPolicy=Object.freeze(JSON.parse(graphSnapshotPolicyCaptureV1.text??"")),exactSourceReadPolicyV1:typeof exactReadSchema.definitions.Policy.const=Object.freeze(JSON.parse(exactReadPolicyCaptureV1.text??"").definitions.Policy.const);
function requireCurrentGraphReadPoliciesV1():void{if(captureBoundedSourceRevisionV1(fileURLToPath(graphSnapshotPolicyURLV1),65536).sha256!==graphSnapshotPolicyCaptureV1.sha256||captureBoundedSourceRevisionV1(fileURLToPath(exactReadPolicyURLV1),65536).sha256!==exactReadPolicyCaptureV1.sha256)throw Error("Stale loaded Graph reader policy; restart required");}

/** 📖️ Reads each declaration through the same exact source fact boundary as inference. */
function graphDeclarationV1(path:string):any{const url=new URL(path,physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),captured=capturePhysicalSourceTextV1(url,65536),observed=observationV1.observedFs.readFileSync(url,"utf8");if(observed!==captured)throw Error("Current graph declaration Source changed");return JSON.parse(observed);}

/** 🧾️ Binds loaded graph behavior and its publication declaration to their original bytes. */
export interface BuildGraphModuleAuthorityV1{implementation:string;publication:string}
const graphModuleAuthorityV1=():BuildGraphModuleAuthorityV1=>{requireCurrentSourceInventoryV1();requireCurrentGraphReadPoliciesV1();const observation=observationV1;if(typeof observation.requireCurrentObservationV1!=="function")throw Error("Loaded observation current-instance guard required");observation.requireCurrentObservationV1();const implementation=createHash("sha256");for(const path of[new URL(physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../👁️observation/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../👁️observation/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🧮️digest/📥️read/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../👁️observation/📥️reuse/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../👁️observation/📥️reuse/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/📋️data/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/📋️data/📥️load/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/📋️data/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🚪️admission/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/🚪️selection.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/📋️owners.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/🚪️admission.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/💾️storage/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/💾️storage/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/💾️storage/📒️ledger/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🔏️revision/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🔏️revision/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),graphSnapshotPolicyURLV1])implementation.update(captureBoundedSourceRevisionV1(fileURLToPath(path),8388608).sha256);return {implementation:implementation.digest("hex"),publication:captureBoundedSourceRevisionV1(fileURLToPath(new URL("../🧬️schema/📣️publication.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),65536).sha256};};
const loadedGraphModuleAuthorityV1=graphModuleAuthorityV1();

/** 🔒️ Refuses a loaded implementation or declaration superseded while an original invocation waited. */
export function requireBuildGraphModuleAuthorityV1(original:BuildGraphModuleAuthorityV1,current:BuildGraphModuleAuthorityV1):void{
 const schema=graphDeclarationV1("../🧬️schema/📥️authority.json").properties.module;
 if(validateJsonSchemaSubset(schema,original).length||validateJsonSchemaSubset(schema,current).length||original.implementation!==current.implementation||original.publication!==current.publication)throw Error("Loaded build graph source changed; restart the owned build invocation "+JSON.stringify({loaded:original,current}));
}
const requireCurrentGraphModuleV1=():void=>requireBuildGraphModuleAuthorityV1(loadedGraphModuleAuthorityV1,graphModuleAuthorityV1());

/** 🔏️ Reads immutable defining authority after checking that the loaded graph is still current. */
export function readBuildGraphModuleAuthorityV1():Readonly<BuildGraphModuleAuthorityV1>{requireCurrentGraphModuleV1();return Object.freeze({...loadedGraphModuleAuthorityV1});}

/** 🧷️ Admits only source receipts issued by the exact loaded producer protocol. */
export function buildGraphProducerAuthorityV1(receipt:unknown,module:BuildGraphModuleAuthorityV1):boolean{
 if(validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/📥️authority.json"),receipt).length)return false;
 const admitted=receipt as {module:BuildGraphModuleAuthorityV1};return admitted.module.implementation===module.implementation&&admitted.module.publication===module.publication;
}

/** ⚙️ Describes portable process execution independently of any build implementation. */
export interface BuildExecutionV1 { kind: "command" | "noop"; program?:string; command?: string; commands?: (string | {command:string;forwardAllArgs?:boolean})[]; args?:string[]; cwd?:string; env?:Record<string,string>; parallel?:boolean; forwardAllArgs?:boolean; [key:string]:unknown }
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
  const fixed=["kind","program","ownerArguments","nativeOwnerCommand","executor","inputs","outputs","dependsOn","cache","continuous","parallelism","metadata","defaultConfiguration","configurations","options"];
  if(Object.keys(configuration).some(key=>fixed.includes(key)))throw Error("Build configuration cannot change task identity");
  if(base.program!==undefined&&(configuration.command!==undefined||configuration.commands!==undefined)||configuration.command!==undefined&&(configuration.commands!==undefined||base.commands!==undefined)||configuration.commands!==undefined&&base.command!==undefined)throw Error("Build configuration cannot change command form");
}

/** 🔡️ Keeps authored argument boundaries literal through every backend's execution protocol. */
function requireBuildArgumentsV1(options:Record<string,unknown>,identity?:string):void {
  if(options.program!==undefined&&(typeof options.program!=="string"||!options.program||options.command!==undefined||options.commands!==undefined))throw Error("Literal build program requires one exclusive nonempty executable");
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
async function withBuildGraphStateV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(store:RepositoryCacheStoreV1,actualControl:ResourceLeaseControl)=>Promise<T>,scope?:string):Promise<T> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);requireCurrentGraphModuleV1();
  const run=async(store:RepositoryCacheStoreV1,actualControl:ResourceLeaseControl)=>{
    const writer=await acquireQueuedResourceLease({directory:join(root,".cache/b/l/m/data"),resource:"build-graph-state:"+store.root+(scope?":selected:"+buildGraphWriterResourceV1(root,"selection",scope).split(":").at(-1):""),mode:"exclusive",owner:randomBytes(16).toString("base64url"),control:actualControl,signal:actualControl.signal});
    try{requireCurrentGraphModuleV1();checkResourceLeaseControl(actualControl);return await body(store,actualControl);}finally{writer.release();}
  };
  return scope?withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,scope,run):withRepositoryBuildGraphFactStoreV1(root,invocation,phaseControl,run);
}

/** ♻️ Invalidates current graph authority while retaining charged storage, immutable pins and source facts. */
export async function invalidateBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,beforeInvalidation?:(actualControl:ResourceLeaseControl)=>Promise<void>):Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);await advanceScriptInvocation(invocation,"build:reset","running");
  await withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl)=>{
    await store.recheck();if(store.root!==dirname(buildGraphPathV1(root)))throw Error("Original graph publication store required");
    const device=(await lstat(store.root)).dev,entries:BuildGraphInvalidationInventoryV1["entries"]=[];
    for(const name of await readdir(store.root)){
      checkScriptInvocation(invocation);const status=await lstat(join(store.root,name));entries.push({name,kind:status.isSymbolicLink()?"symlink":status.isFile()?"file":status.isDirectory()?"directory":"other",links:status.nlink,bytes:status.size,device:status.dev});
      if(entries.length%64===0)await advanceScriptInvocation(invocation,"build:reset-inventory","running");
    }
    const selected=planBuildGraphInvalidationV1({device,entries});checkScriptInvocation(invocation);checkResourceLeaseControl(actualControl);if(beforeInvalidation)await beforeInvalidation(actualControl);checkResourceLeaseControl(actualControl);checkScriptInvocation(invocation);
    for(const name of selected){
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
function portableInputs(root:string,value:unknown,invocation:ScriptInvocation,control:ResourceLeaseControl):unknown|Promise<unknown> {
  const token=(item:unknown)=>{
    if(!item||typeof item!=="object"||Array.isArray(item))return undefined;
    const object=item as Record<string,unknown>;if(Object.keys(object).length!==1||typeof object.env!=="string")return undefined;
    const match=/^SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})$/u.exec(object.env);
    return match?{key:object.env,digest:match[2]!,directory:match[1]==="SOURCE_CLOSURE"?"build-input-closures":"build-lock-closures"}:undefined;
  };
  captureInferenceInventoryReferencesV1(value);
  const pending:unknown[]=[value],seen=new WeakSet<object>(),tokens=new Map<string,NonNullable<ReturnType<typeof token>>>();let visited=0;
  while(pending.length){
    if(invocation&&visited++%1024===0)checkScriptInvocation(invocation);
    const item=pending.pop();if(!item||typeof item!=="object"||seen.has(item))continue;seen.add(item);
    const compact=token(item);if(compact)tokens.set(compact.key,compact);else for(const child of Object.values(item))if(child&&typeof child==="object")pending.push(child);
  }
  if(!tokens.size)return value;
  const expand=async()=>{
    const entries=[...tokens.values()],expanded=new Map<string,string[]>();
    await withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{
      const extents:number[]=[];let bytes=0,readRecords=0;
      const cadence=exactSourceReadPolicyV1.maximumInventoryRecordsPerYield;
      await store.recheck();checkResourceLeaseControl(actual);
      if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");
      for(const compact of entries){
        checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
        const path=join(store.root,compact.directory,compact.digest+".json"),row=await lstat(path);
        checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
        if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1||!Number.isSafeInteger(row.size)||row.size<0||row.size>exactSourceReadPolicyV1.maximumClosureBytes)throw Error("Single-link bounded regular inventory extent required");
        extents.push(Math.max(1,row.size));
        if(extents.length%cadence===0){await store.recheck();await actual.yieldContinuation();checkScriptInvocation(invocation);checkResourceLeaseControl(actual);if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");}
      }
      await store.recheck();checkResourceLeaseControl(actual);
      for(const cohort of planInferenceInventoryCohortsV1(extents)){
        admitInferenceInventoryCohortV1(bytes,cohort.records,cohort.maximumBytes);
        checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
        await withManagedGraphSourceReadBatchV1(root,actual,async batch=>{for(const compact of entries.slice(cohort.offset,cohort.offset+cohort.records)){
          checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
          const file=await readExactSourceInventoryV1(batch,compact.directory as "build-input-closures"|"build-lock-closures",compact.digest,cohort.maximumBytes);
          checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
          if(!file||file.sha256!==compact.digest)throw Error(`Build input closure content differs: ${compact.key}`);
          bytes+=file.byteLength;const contract=file.value;expanded.set(compact.key,compact.directory==="build-input-closures"?captureSourceInputInventoryV1(contract,root).value.files.map(file=>"{workspaceRoot}/"+file):["{workspaceRoot}/Cargo.lock","{workspaceRoot}/"+captureCargoLockInventoryV1(contract,root).value.manifest]);
          if(++readRecords%cadence===0){await actual.yieldContinuation();checkScriptInvocation(invocation);checkResourceLeaseControl(actual);if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");}
        }});
        await store.recheck();checkResourceLeaseControl(actual);if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");
      }
    });
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
    const result=transform(value);if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","complete");return result;
  };
  return expand();
}

/** 🪪️ Refuses caller registry data superseded before the current source boundary. */
export function requireBuildOwnerRegistryAuthorityV1(original:unknown,current:unknown):void{
 if(JSON.stringify(readBuildOwnerRegistryV1(original))!==JSON.stringify(readBuildOwnerRegistryV1(current)))throw Object.assign(Error("Original build owner registry changed before source consumption"),{code:"BUILD_SOURCE_AUTHORITY"});
}

/** 📚️ Assembles all owned inference contributions without installing or invoking Nx. */
export async function discoverBuildGraphV1(root:string,selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,suppliedFiles?:string[],admission?:BuildOwnerAdmissionV1):Promise<BuildGraphV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);selection=readBuildGraphSelectionV1(selection);registry=readBuildOwnerRegistryV1(registry);
  requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8")));
  if(selection.kind==="tasks"&&!admission)throw Error("Closed selected owner admission required");
  const files=suppliedFiles??(selection.kind==="all"?buildGraphMembersV1(await workspaceBuildFilesV1(root,invocation)):admission!.files);
  const [owner,tests,policies]=await Promise.all([import("./📥️inference/🟨️.mjs"),import("../../../🧪️test/🟨️.mjs"),import("./📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
  const policy=graphDeclarationV1("../🔣️policy.json"),options={analyzeLockfile:true,catalogProviders:selection.kind==="all"?registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})):admission!.catalogProviders,...(selection.kind==="tasks"?{ownerAdmission:admission}:{})};
  const primary=await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||file==="bun.lock"||file.endsWith(".patch")),options,{workspaceRoot:root,invocation,phaseControl,registry});
  if(invocation)await advanceScriptInvocation(invocation,"build:infer","running");
  const contributions=[...primary,...await tests.discoverTestBuildProjectsV1(files.filter(file=>file.endsWith(".feature")),{}, {workspaceRoot:root,invocation,phaseControl})];
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
  const context={workspaceRoot:root,projects,fileMap:{projectFileMap},invocation,phaseControl,selection,registry,completeSourceImports:selection.kind==="tasks"};
  const testEdges:unknown=await tests.discoverTestBuildDependenciesV1({},context);if(!Array.isArray(testEdges))throw Error("Test dependency DATA array required");const edges=[...await owner.discoverBuildDependenciesV1(options,context),...testEdges];
  const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{name,type:"lib",data}]))},options,context);
  if(!metadata||typeof metadata!=="object"||Array.isArray(metadata))throw Error("Graph metadata DATA object required");
  for(const[name,value]of Object.entries(metadata)){if(!value||typeof value!=="object"||!("metadata"in value)||!value.metadata||typeof value.metadata!=="object"||Array.isArray(value.metadata))throw Error("Project metadata DATA object required");if("semio"in value.metadata&&value.metadata.semio!==undefined&&(!value.metadata.semio||typeof value.metadata.semio!=="object"||Array.isArray(value.metadata.semio)))throw Error("Project semio metadata DATA object required");projects[name].metadata={...projects[name].metadata,...value.metadata,semio:{...projects[name].metadata?.semio,...("semio"in value.metadata&&value.metadata.semio&&typeof value.metadata.semio==="object"?value.metadata.semio:{})}};}
  const neutral=Object.fromEntries(Object.entries(projects).map(([name,project])=>[name,{...project,targets:Object.fromEntries(Object.entries(project.targets??{}).map(([target,value])=>[target,neutralBuildTargetV1(value as Record<string,any>,`${name}:${target}`)]))}]));
  const dependencies=[...new Map(edges.map(edge=>[JSON.stringify(edge),edge])).values()].sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)));
  if(invocation)await advanceScriptInvocation(invocation,"build:graph","complete");
  return readBuildGraphV1({version:1,projects:await portableInputs(root,neutral,invocation,phaseControl),dependencies,namedInputs:policy.namedInputs,externalNodes,...(unavailable.size?{unavailable:[...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0)}:{}),admission:{selection,registry:createHash("sha256").update(JSON.stringify(registry)).digest("hex"),closure:createHash("sha256").update(JSON.stringify(admission??files)).digest("hex")}});
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
export async function buildGraphSourceFactsHoldV1(observations:readonly unknown[],invocation?:ScriptInvocation,onStale?:(row:{kind:string;path:string;fact:unknown})=>void):Promise<boolean>{return observationV1.observationsHold(observations,64,new Set(),new Set(),invocation?.control.signal,onStale);}

type OwnerRegistryInvalidationV1={readonly kind:"invalidated";readonly reason:"owner-registry"};
const ownerRegistryInvalidationV1:OwnerRegistryInvalidationV1=Object.freeze({kind:"invalidated",reason:"owner-registry"});
const buildGraphErrorCodeV1=(error:unknown):unknown=>error!==null&&typeof error==="object"&&"code" in error?error.code:undefined;
const isOwnerRegistryInvalidationV1=(value:unknown):value is OwnerRegistryInvalidationV1=>value===ownerRegistryInvalidationV1;

/** 📌️ Reads immutable snapshot bytes while the existing original Graph store and exact content reader lease remain held. */
async function withRetainedGraphSnapshotV1<T>(workspace:string,store:RepositoryCacheStoreV1,path:string,control:ResourceLeaseControl,run:(source:string)=>Promise<T>):Promise<T>{
 const policy=graphSnapshotPolicyV1,lease=await acquireQueuedResourceLease({directory:join(workspace,".cache/b/l/m/data"),resource:"build-graph:"+path,mode:"shared",owner:randomBytes(16).toString("base64url"),control,signal:control.signal});let failed=false;
 try{await store.recheck();checkResourceLeaseControl(control);const directory=dirname(path),parent=await lstat(directory,{bigint:true}),before=await lstat(path,{bigint:true});checkResourceLeaseControl(control);if(!parent.isDirectory()||parent.isSymbolicLink()||!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<0n||before.size>BigInt(policy.maximumSnapshotBytes))throw Error("Original bounded Graph snapshot required");
 return await withArtifactDescriptorV1(()=>open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0)),async file=>{const verify=async()=>{await store.recheck();checkResourceLeaseControl(control);requireNativeStoragePathV1(store.root,path);const linked=await lstat(path,{bigint:true}),opened=await file.stat({bigint:true}),actualParent=await lstat(directory,{bigint:true});checkResourceLeaseControl(control);if(!actualParent.isDirectory()||actualParent.isSymbolicLink()||actualParent.dev!==parent.dev||actualParent.ino!==parent.ino||[linked,opened].some(row=>!row.isFile()||row.isSymbolicLink()||["dev","ino","size","nlink","mtimeNs","ctimeNs"].some(key=>row[key as keyof typeof row]!==before[key as keyof typeof before])))throw Error("Retained Graph snapshot descriptor changed");};await verify();const bytes=Buffer.alloc(Number(before.size));try{let offset=0;while(offset<bytes.length){await verify();const row=await file.read(bytes,offset,Math.min(65536,bytes.length-offset),offset);checkResourceLeaseControl(control);if(!row.bytesRead)throw Error("Retained Graph snapshot shortened");offset+=row.bytesRead;await control.yieldContinuation();}if((await file.read(Buffer.alloc(1),0,1,bytes.length)).bytesRead)throw Error("Retained Graph snapshot grew");await verify();const value=await run(new TextDecoder("utf-8",{fatal:true}).decode(bytes));await verify();return value;}finally{bytes.fill(0);}});
 }catch(error){failed=true;throw error;}finally{try{lease.release();}catch(error){if(!failed)throw error;}}
}

/** 🏪️ Delivers the pinned reusable graph only through its existing private Store consumer and retained exact content descriptor. */
export function withRepositoryBuildGraphPublicationV1<T>(root:string,control:ResourceLeaseControl,body:(graph:BuildGraphV1,digest:string,actual:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 requireCurrentGraphModuleV1();const pin={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST};if(validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/📥️pin.json"),pin).length||!isAbsolute(pin.path!))throw Error("Original immutable build graph path and digest required");
 return withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{await store.recheck();checkResourceLeaseControl(actual);const expected=requireNativeStoragePathV1(store.root,join(store.root,"snapshots",pin.digest+".json"));if(pin.path!==expected)throw Error("Original retained Graph snapshot identity required");return withRetainedGraphSnapshotV1(root,store,expected,actual,async source=>{if(createHash("sha256").update(source).digest("hex")!==pin.digest)throw Error("Pinned build graph digest differs from original authority");const graph=readBuildGraphV1(await portableInputs(root,JSON.parse(source),store.invocation,actual));requireCurrentGraphModuleV1();checkResourceLeaseControl(actual);const value=await body(graph,pin.digest!,actual);await store.recheck();checkResourceLeaseControl(actual);requireCurrentGraphModuleV1();return value;});});
}

/** 📥️ Resolves one candidate while keeping every candidate's memory within its own finite attempt. */
async function loadBuildGraphAttemptV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean},registry?:BuildOwnerRegistryV1,scope?:string):Promise<BuildGraphV1|OwnerRegistryInvalidationV1|undefined> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
  const profile=process.env.SEMIO_BUILD_PROFILE_INPUTS==="1",timing=(phase:string,started:number,details:Record<string,unknown>={})=>{if(profile){}};
  const image=async(source:string)=>{let started=performance.now();const decoded=JSON.parse(source);timing("graph-json-parse",started,{bytes:Buffer.byteLength(source)});started=performance.now();const expanded=await portableInputs(root,decoded,invocation,phaseControl);timing("graph-input-expand",started);started=performance.now();const graph=readBuildGraphV1(expanded);timing("graph-schema-validation",started,{projects:Object.keys(graph.projects).length,dependencies:graph.dependencies.length});return graph;};
  if(!registry)throw Error("Original canonical graph owner registry required");
  const selected=options.selection.kind==="tasks",path=scope?join(dirname(buildGraphPathV1(root)),"selected",scope+".json"):buildGraphPathV1(root),authorityPath=scope?join(dirname(path),scope+".authority.json"):join(dirname(path),graphDeclarationV1("../🧬️schema/📣️publication.json").properties.authority.const);
  const observation=observationV1,[producer]=await Promise.all([import("./📥️inference/🟨️.mjs"),import("../../../🧪️test/🟨️.mjs"),import("./📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
  const inventory=async()=>{let started=performance.now();const captured=selected?await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{const current=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(buildGraphScopeKeyV1(options.selection,current)!==scope)return ownerRegistryInvalidationV1;const nativeCache=new Map(),nativeClosures=new Map();return closeBuildOwnerAdmissionV1(options.selection,registry!,async(owner,targets,inputs)=>{if(invocation)checkScriptInvocation(invocation);return producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,invocation,phaseControl,selection:options.selection,nativeCache,nativeClosures},inputs);});},{signal:invocation?.control.signal})):undefined,candidate=captured?.value;if(isOwnerRegistryInvalidationV1(candidate))return candidate;const admission=candidate as BuildOwnerAdmissionV1|undefined,files=admission?.files??buildGraphMembersV1(await workspaceBuildFilesV1(root,invocation));timing("workspace-catalog-lookup",started,{files:files.length,directories:inventoryDirectoriesV1.get(files)?.length??0});started=performance.now();const digest=await buildGraphMembershipV1(root,files,invocation);timing("workspace-membership-hash",started);return {files,digest,admission,observations:captured?.observations??[]};};
  let initial:Awaited<ReturnType<typeof inventory>>;try{initial=await inventory();}catch(error){if(buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return undefined;throw error;}
  if(isOwnerRegistryInvalidationV1(initial))return initial;
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
  let captured:{value:BuildGraphV1|OwnerRegistryInvalidationV1;observations:readonly unknown[]},failedFacts:unknown[]=[],failedAuthority:unknown[]=[];
  phaseStarted=performance.now();
  try{captured=await observation.captureObservedSourceInputsV1(async()=>{
    for(const source of [new URL(physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/📣️publication.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/📥️authority.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../⚡️caching/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🗂️workspaces/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📤️arguments/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))])observation.observedFs.readFileSync(source);
    const admittedRegistry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(JSON.stringify(admittedRegistry)!==JSON.stringify(registry))return ownerRegistryInvalidationV1;
    for(const source of [new URL("../🧬️schema/🚪️selection.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/📋️owners.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/🚪️admission.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🚪️admission/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))])observation.observedFs.readFileSync(source);
    return discoverBuildGraphV1(root,options.selection,registry!,invocation,phaseControl,files,initial.admission);
  },(rows:unknown[],authority:unknown[])=>{failedFacts=rows;failedAuthority=authority;});}
  catch(error){if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true"){}if(invocation)checkScriptInvocation(invocation);if(buildGraphErrorCodeV1(error)==="BUILD_AUTHORITY_CHANGED")throw error;let current;try{current=await inventory();}catch(recheck){if(buildGraphErrorCodeV1(recheck)==="BUILD_MEMBERSHIP_CHANGED")return undefined;throw recheck;}if(isOwnerRegistryInvalidationV1(current))throw error;const stale:{kind:string;path:string;fact:unknown}[]=[],holds=await buildGraphSourceFactsHoldV1(failedFacts,invocation,row=>{if(stale.length<8)stale.push(row);});if(current.digest===before&&holds)throw error;if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true"){}return undefined;}
  if(isOwnerRegistryInvalidationV1(captured.value))return captured.value;
  timing("graph-original-inference",phaseStarted,{observations:captured.observations.length});
  let final:Awaited<ReturnType<typeof inventory>>;try{final=await inventory();}catch(error){if(buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return undefined;throw error;}
  if(isOwnerRegistryInvalidationV1(final))return final;
  const after=final.digest;
  captured.observations=[...initial.observations,...captured.observations];
  const stale:{kind:string;path:string;fact:unknown}[]=[],holds=await buildGraphSourceFactsHoldV1(captured.observations,invocation,row=>{if(stale.length<8)stale.push(row);});
  if(selected&&JSON.stringify(initial.admission)!==JSON.stringify(final.admission))return undefined;
  if(before!==after||!holds){if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true"){const oldFiles=new Set(files),newFiles=new Set(final.files);}return undefined;}
  const graph=captured.value,source=JSON.stringify(graph),digest=createHash("sha256").update(source).digest("hex"),authority=JSON.stringify({version:3,module:loadedGraphModuleAuthorityV1,membership:after,graph:digest,observations:captured.observations});if(Buffer.byteLength(source)>536870912||Buffer.byteLength(authority)>536870912)throw Error("Build graph cache exceeds its finite publication limit");await mkdir(dirname(path),{recursive:true});
  const temporary=path+"."+randomUUID(),receipt=authorityPath+"."+randomUUID();try{await writeFile(temporary,source,{flag:"wx"});await writeFile(receipt,authority,{flag:"wx"});if(invocation)checkScriptInvocation(invocation);requireCurrentGraphModuleV1();if(!await buildGraphSourceFactsHoldV1(captured.observations,invocation))return undefined;await rename(temporary,path);await rename(receipt,authorityPath);}finally{await Promise.all([rm(temporary,{force:true}),rm(receipt,{force:true})]);}
  return graph;
}
/** 💾️ Publishes only a current graph and releases each invalidated candidate before retrying. */
export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean}):Promise<BuildGraphV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);requireCurrentGraphModuleV1();
  options={...options,selection:readBuildGraphSelectionV1(options.selection)};
  if(options.reuse)return withRepositoryBuildGraphPublicationV1(root,phaseControl,async graph=>{checkScriptInvocation(invocation);if(!graph.admission||JSON.stringify(readBuildGraphSelectionV1(graph.admission.selection))!==JSON.stringify(readBuildGraphSelectionV1(options.selection)))throw Error("Original selected build graph admission required");return graph;});
  if(!invocation)throw Error("Original build graph producer invocation required");
  for(let attempt=0;attempt<=8;attempt++){
    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
    const producer=await import("./📥️inference/🟨️.mjs");checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),scope=options.selection.kind==="tasks"?buildGraphScopeKeyV1(options.selection,registry):undefined;
    const graph=await withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl)=>{
      const staging=scope?join(store.root,"selected"):store.root;await store.recheck();checkResourceLeaseControl(actualControl);requireNativeStoragePathV1(store.root,staging);await mkdir(staging,{recursive:true});checkResourceLeaseControl(actualControl);const selectedStage=scope?new RegExp("^"+scope+"(?:\\.authority)?\\.json\\.[a-f0-9-]{36}$","u"):undefined;
      const staged=await readdir(staging);checkResourceLeaseControl(actualControl);for(const name of staged){if(!(selectedStage?.test(name)??/^(?:canonical|authority)\.json\.[a-f0-9-]{36}$/u.test(name)))continue;await store.recheck();checkResourceLeaseControl(actualControl);const path=join(staging,name),status=await lstat(path);checkResourceLeaseControl(actualControl);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>536870912)throw Error("Owned bounded graph staging file required");await rm(path);checkResourceLeaseControl(actualControl);}
      checkResourceLeaseControl(actualControl);await store.recheck();return loadBuildGraphAttemptV1(root,invocation,actualControl,options,registry,scope);
    },scope);
    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
    if(graph&&!isOwnerRegistryInvalidationV1(graph))return graph;
    await advanceScriptInvocation(invocation,"build:refresh","running");
  }
  throw Error("Build source authority keeps changing during inference");
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
