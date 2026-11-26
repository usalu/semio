import {createBoundedCacheV1} from "../../../🧊️bounded/🟨️.mjs";
import {ROSTER_RULES_V1} from "../../🧭️roster/🟨️.mjs";
import {assertNotReadParentV1} from "../../../👁️observation/🚧️read-parent/🟨️.mjs";
import {wasmCompileOutputIndexV1,wasmCompileOutputV1} from "./🕸️wasm-outputs/🟦️.ts";
import {typeScriptPackageOutputProducerV1} from "./../../../../⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🏭️producer/🟦️.ts";
/** 🔎️ Discovers canonical owners, their native manifests and the selected executable Source closures. */
import { checkResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { pinnedAuthorityReadV1 } from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import { checkScriptInvocation, requireIssuedResourceControlInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createHash } from "node:crypto";
import { parserTypescriptV1 } from "../../../🚀️bootstrap/📦️parsers/🟨️.mjs";
import { dirname, isAbsolute, join, relative, resolve, posix } from "node:path";
import { loadCatalogBuildProvidersV1, catalogBuildContributionForManifestV1 } from "../../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts";
import { importObservedModuleV1, memoizedObservedAsyncComputationV1, observedSourceAnalysisPairV1, observedRequireResolve, observedTypeScriptHost, observedFs, recordRegistryOwnerV1, memoizedObservedComputationV1, observedFieldV1, registerFieldExtractorV1, withObservationRoundV1, observeGlobalFactV1, observeDetachedSyncV1, observationScopeV1, registerGlobalFactV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { generatorContractReadHoldsV1, trackGeneratorContractsV1 } from "../📜️contract-reads/🟨️.mjs";
import { pathToFileURL } from "node:url";
import * as loadedProjectInputs from "../../../../🏗️build/📥️inputs/🟨️.mjs";
import ownerAdmissionSchema from "../../../🧬️schema/🚪️admission.json" with { type: "json" };
import { askOwnerRegistryV1, observeOwnerRegistryAnswerV1, observedOwnerRegistryViewV1, registerOwnerRegistryQuestionV1 } from "../../🧾️registry-reads/🟨️.mjs";
import * as nodeFs from "node:fs";
import { buildInputSourceRoleV1, buildSourceDemandV1, buildNativeSourceObservationV1 } from "../../🚪️admission/🟦️.ts";
import {refuseBuildV1} from "../../../📖️texts/🟦️.ts";
import {INFERENCE_LIMITS_V1} from "../🧬️schema/🟨️.mjs";
import {observationRoundV1} from "../../../👁️observation/🟨️.mjs";
import{isOpaqueSourcePathV1}from"../../../📥️inputs/👻️opaque-source/🟨️.mjs";
const { existsSync, lstatSync, readFileSync, readdirSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "owner-discovery";
let buildPath, POLICY, WORKSPACE_POLICY, exampleCollectionPath, yieldInferenceV1, readPhysicalSource, requirePhysicalSourceV1, importRevision, PROJECT_BASENAME, nativeDependencyRoots, libraryBootstrap, readToml, createScriptInputCache, cargoTargets, componentBuildTargets, printDocumentTargets, readTaxonomyProjectionV1, owned, playgroundSessionTargets, playgroundPreparationTargets, collectPlaygroundCatalog, playgroundSiteTargetsForAdmissionV1, targetWithDefaults, SCRIPT_BASENAME, loaded, targetScriptSources, projectWithDefaults, withNativePreparation, LIBRARY_ROOT, EXAMPLE_COLLECTIONS, WORKSPACE_OWNERSHIP, relativeScriptInputs, prepareSourceAnalysesV1, buildImportResourceV1, workspacePackageModuleV1;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ buildPath, POLICY, WORKSPACE_POLICY, exampleCollectionPath, readPhysicalSource, requirePhysicalSourceV1, PROJECT_BASENAME, owned, SCRIPT_BASENAME, LIBRARY_ROOT, EXAMPLE_COLLECTIONS, WORKSPACE_OWNERSHIP } = registry.coordinates);
  ({ yieldInferenceV1, createScriptInputCache, targetScriptSources, relativeScriptInputs, buildImportResourceV1, workspacePackageModuleV1 } = registry.scriptInputs);
  ({ importRevision, libraryBootstrap, loaded } = registry.bootstrap);
  ({ nativeDependencyRoots, readToml, withNativePreparation } = registry.nativeManifests);
  ({ cargoTargets, componentBuildTargets, printDocumentTargets } = registry.componentTargets);
  ({ readTaxonomyProjectionV1, prepareSourceAnalysesV1 } = registry.sourceAnalyses);
  ({ playgroundSessionTargets, playgroundPreparationTargets, collectPlaygroundCatalog, playgroundSiteTargetsForAdmissionV1 } = registry.playgroundTargets);
  ({ targetWithDefaults } = registry.targetPolicy);
  ({ projectWithDefaults } = registry.projectAssembly);
}

/** 📜️ The taxonomy file the generator contracts are read from, below the workspace root. */
const TAXONOMY_PATH_V1="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
/** 🏷️ The name under which what an owner admission read of the generator contracts travels as `global` rows. */
export const ADMISSION_GENERATOR_CONTRACTS_FACT_V1="owner-admission-generator-contracts";
const admissionContractsV1=createBoundedCacheV1({name:"inference:admission-contracts",maxEntries:8,maxBytes:8});

/** 📜️ The generator contracts of the workspace taxonomy as they are now, read unobserved and kept while the taxonomy file keeps its size and modification time. */
function currentGeneratorContractsV1(workspaceRoot){
 const path=join(workspaceRoot,TAXONOMY_PATH_V1);
 if(!existsSync(path))return {};
 const status=observedFs.statSync(path),stamp=status.size+":"+status.mtimeMs,held=admissionContractsV1.get(workspaceRoot);
 if(held?.stamp===stamp)return held.contracts;
 const contracts=observeDetachedSyncV1(()=>readTaxonomyProjectionV1(path)).value.generatorContracts;
 admissionContractsV1.set(workspaceRoot,{stamp,contracts});
 return contracts;
}

/** ✅️ Whether one recorded read of the generator contracts has the same answer in the taxonomy as it is now. */
function admissionGeneratorContractReadHoldsV1(workspaceRoot,segments,fact){return generatorContractReadHoldsV1(currentGeneratorContractsV1(workspaceRoot),segments,fact);}
registerGlobalFactV1(ADMISSION_GENERATOR_CONTRACTS_FACT_V1,admissionGeneratorContractReadHoldsV1);

/** 🔭️ The generator contracts one owner admission consults: every read of them (a property, a listing of keys, a probe) is recorded as a `global` row of the owner, instead of the whole taxonomy projection as one row, so an edit of a contract the owner never read keeps its remembered admission valid while a peer edits the taxonomy.
 * @see ../📜️contract-reads/🟨️.mjs
 * @see ./🧪️tests/📜️contract-rows/🟦️.ts */
function ownerGeneratorContractsV1(workspaceRoot){
 const told=new Map();
 return trackGeneratorContractsV1(currentGeneratorContractsV1(workspaceRoot),(segments,fact)=>{
  const scope=observationScopeV1(),key=segments.join("\u0001");
  let scopes=told.get(key);
  if(scopes===undefined)told.set(key,scopes=new WeakSet());
  if(scope!==undefined){if(scopes.has(scope))return;scopes.add(scope);}
  observeGlobalFactV1(ADMISSION_GENERATOR_CONTRACTS_FACT_V1,workspaceRoot,segments,fact);
 }).view;
}

/** 🦀️ Lists every workspace Cargo manifest through taxonomy-respecting discovery. */
export function walkCargoToml(workspaceRoot) {
  const found = [];
  const walk = (directory) => {
    for (const entry of readdirSync(join(workspaceRoot, directory), { withFileTypes: true })) {
      const path = buildPath(join(directory, entry.name));
      if (entry.isSymbolicLink() || isOpaqueSourcePathV1(path) || path === "compose" || path === "temp/compose" || entry.name === ".🧬semio" || POLICY.generatedDirectories.includes(entry.name)) continue;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile() && entry.name === "Cargo.toml") found.push(path);
    }
  };
  walk("");
  return found.sort();
}

/** 🧠️ Discovers the same complete canonical manifest scope through the caller's bounded continuation. */
export async function walkCargoTomlControlledV1(workspaceRoot, control) {
  const found = [], directories = [""];let visited=0;
  while(directories.length){
    control.check();if(visited%128===0)await control.advance("manifest-discovery");
    const directory=directories.pop();
    for(const entry of readdirSync(join(workspaceRoot,directory),{withFileTypes:true})){
      const path=buildPath(join(directory,entry.name));
      if(entry.isSymbolicLink()||isOpaqueSourcePathV1(path)||path==="compose"||path==="temp/compose"||entry.name===".🧬semio"||POLICY.generatedDirectories.includes(entry.name))continue;
      if(entry.isDirectory())directories.push(path);else if(entry.isFile()&&entry.name==="Cargo.toml")found.push(path);
    }
    visited++;
  }
  await control.advance(`manifest-discovery:${visited}`);
  return found.sort();
}

/** 🛡️ Admits callback candidates through fresh raw physical workspace ancestry. */
export function inferenceCandidates(configFiles, workspaceRoot) {
  if (typeof workspaceRoot !== "string" || !workspaceRoot || workspaceRoot.includes("\0") || workspaceRoot.split(/[\\/]/u).some(part => part === "." || part === "..")) throw Error("Build inference requires raw real workspace ancestry");
  const root = resolve(workspaceRoot), ancestors = [];
  for (let path = root; ; path = dirname(path)) { ancestors.push(path); if (path === dirname(path)) break; }
  for (const path of ancestors.reverse()) { const stat = lstatSync(path); if (!stat.isDirectory() || stat.isSymbolicLink()) throw Error(`Build inference requires real workspace ancestry: ${path}`); }
  const directories = new Map([[root, true]]), files = new Set();
  for (const supplied of configFiles) {
    if (!ROSTER_RULES_V1.admitsCandidate(supplied)) continue;
    const path = buildPath(supplied), parts = path.split("/");
    let directory = root, admitted = true;
    for (const part of parts.slice(0, -1)) {
      directory = join(directory, part);
      if (!directories.has(directory)) {
        try { const value = lstatSync(directory); directories.set(directory, value.isDirectory() && !value.isSymbolicLink()); }
        catch (error) { if (!["ENOENT", "ENOTDIR"].includes(error.code)) throw error; directories.set(directory, false); }
      }
      if (!directories.get(directory)) { admitted = false; break; }
    }
    if (!admitted) continue;
    try { const value = lstatSync(join(root, path)); if (value.isFile() && !value.isSymbolicLink()) files.add(path); }
    catch (error) { if (!["ENOENT", "ENOTDIR"].includes(error.code)) throw error; }
  }
  return [...files].sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
}

/** 🌫️ Reads text of a file that replicas may remove while inference runs: a vanished file (ENOENT/ENOTDIR) is absent, every other failure surfaces. */
export function readVanishableTextV1(path){
 try{return readFileSync(path,"utf8");}catch(error){if(error?.code==="ENOENT"||error?.code==="ENOTDIR")return undefined;throw error;}
}

/** 🦉️ Projects native catalog manifest candidates from canonical owner identities. */
export async function catalogNativeManifestCandidatesV1(workspaceRoot,registry,accept,context){
 askOwnerRegistryV1(workspaceRoot,registry,["candidates"]);
 const manifest=root=>root==="."?"Cargo.toml":root+"/Cargo.toml",declared=new Set(registry.owners.flatMap(owner=>[...(owner.manifest.endsWith("Cargo.toml")?[owner.manifest]:[]),...(owner.nativeRoots??[]).map(manifest)])),candidates=[...new Set([...declared,...registry.owners.map(owner=>manifest(owner.root))])].filter(accept).sort(),result=[];let index=0;
 for(const path of candidates){
  await yieldInferenceV1(context,"build:catalog-manifests",++index);
  try{
   const entry=lstatSync(join(workspaceRoot,path));
   if(!entry.isFile()||entry.isSymbolicLink())throw Error("Catalog native manifest must be a real file: "+path);
  }catch(error){
   if(error?.code!=="ENOENT"&&error?.code!=="ENOTDIR")throw error;
   if(!declared.has(path))continue;
   const manifest=path==="Cargo.toml"?"./"+path:path;
   throw Object.assign(Error("Build owner manifest absent: "+manifest+" (required by catalog discovery)",{cause:error}),{code:"BUILD_OWNER_MANIFEST_ABSENT"});
  }
  result.push(path);
 }
 return result;
}

/** 📖️ Loads only the caller-declared optional providers and verifies current producer authority. */
export async function catalogBuildContributionsV1(workspaceRoot,providers) {
  const catalogContributions=await loadCatalogBuildProvidersV1(providers,{
    readText(path){return readFileSync(join(workspaceRoot,path),"utf8");},
    kind(path){try{const stat=lstatSync(join(workspaceRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch(error){if(error.code==="ENOENT")return null;throw error;}},
    async load(path){const file=join(workspaceRoot,path),captured=importObservedModuleV1(file);if(captured!==undefined)return captured;return importRevision(pathToFileURL(file),createHash("sha256").update(readPhysicalSource(file)).digest("hex"));},
  });
  for(const owner of catalogContributions){for(const input of owner.inputs)readFileSync(join(workspaceRoot,input));const project=JSON.parse(readFileSync(join(workspaceRoot,owner.projectRoot,PROJECT_BASENAME),"utf8"));if(project.name!==owner.project||!project.targets?.[owner.generateTarget])throw Error("Catalog build producer is absent from its owner's project");}
  return catalogContributions;
}

const catalogMemosV1=new WeakMap(),catalogStatsV1={computed:0,reused:0};

/** 📊️ How often the shared catalog was computed and how often an owner reused it (the counters of this process). */
export const catalogSharedStatsV1=()=>({...catalogStatsV1});

/** 📚️ The catalog contributions of `providers` and the native manifest candidates they accept, computed once per owner registry and answered from the memo to every other owner and to every later revision of one owner (its targets or inputs grew) while every row of the computation still holds: the 3000 manifests are looked at once, not once per owner and per revision, which was the largest part of the discovery of an owner that consumes the catalog (28 s of the stage `declaration` in the host log).
 * @see ./🧪️tests/📚️shared-catalog/🟦️.ts */
export async function sharedCatalogV1(workspaceRoot,registry,providers,context){
 let cache=catalogMemosV1.get(registry);if(cache===undefined){cache=new Map();catalogMemosV1.set(registry,cache);}
 const key=JSON.stringify([workspaceRoot,providers.map(({ownerRoot,sourcePath})=>[ownerRoot,sourcePath])]),checkpoint=force=>yieldInferenceV1(context,"build:catalog-manifests",0,force===true);let computed=false;
 try{
  const contributions=await memoizedObservedAsyncComputationV1(cache,"contributions\0"+key,()=>{computed=true;return catalogBuildContributionsV1(workspaceRoot,providers.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})));},checkpoint);
  const files=contributions.length?await memoizedObservedAsyncComputationV1(cache,"candidates\0"+key,()=>{computed=true;return catalogNativeManifestCandidatesV1(workspaceRoot,registry,path=>catalogBuildContributionForManifestV1(contributions,path),context);},checkpoint):[];
  return {contributions,files};
 }finally{catalogStatsV1[computed?"computed":"reused"]++;}
}

/** 🧭️ Reads the explicit catalog options declared on this workspace plugin. */
export async function workspaceCatalogBuildContributionsV1(workspaceRoot) {
  const config=JSON.parse(pinnedAuthorityReadV1(join(workspaceRoot,"📋️build-owners.json"),()=>readFileSync(join(workspaceRoot,"📋️build-owners.json"),"utf8")));
  return catalogBuildContributionsV1(workspaceRoot,config.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})));
}

/** 🧩️ Invalidates project assembly only when its actual native owners or component pipeline change. The closure walk runs in its own validation round, so a memoized manifest closure that many dependants reach is checked once per owner and not once per hit (outside a round every hit re-checked every row of the closure: 38.5 s against 3.2 s for 60 Rust owners, and 13 to 26 s per owner look on the 16 GB workers).
 * @see ./🧪️tests/🔁️authority-round/🟦️.ts */
export function projectAssemblyAuthorityV1(workspaceRoot,nativeRoot,projectsByRoot,componentRoots,nativeCache=new Map(),nativeClosures=new Map()){
 assertNotReadParentV1("observe",nativeRoot??"release pipeline");let nativeOwners;
 if(nativeRoot!==undefined){try{nativeOwners=withObservationRoundV1(()=>nativeDependencyRoots(nativeRoot,workspaceRoot,true,nativeCache,nativeClosures)).map(owner=>[owner,projectsByRoot.get(owner)??null]);}catch(error){if(error.code!=="NATIVE_INPUT_ADMISSION")throw error;nativeOwners=[error.manifest,error.message];}}
 return createHash("sha256").update(JSON.stringify([nativeOwners??null,componentRoots??null])).digest("hex");
}

/** 🚧️ The characters a generated import path may not hold, compiled once. */
const UNSAFE_GENERATED_PATH_V1 = /[\\%*?#[\]{}]/u;

/** 🗂️ Indexes ignored output roots once per owned generator contract snapshot. */
const GENERATED_OUTPUT_INDEXES_V1 = new WeakMap();
function generatedOutputIndexV1(contracts,safe){
 let index=GENERATED_OUTPUT_INDEXES_V1.get(contracts);
 if(index===undefined){
  index=new Map();
  for(const contract of Object.values(contracts)){
   if(contract.ownership!=="owned")continue;
   for(const output of contract.outputRoots??[]){
    if(output.inclusion!=="ignored")continue;
    const path=output.path;
    if(!safe(path))throw Error("Canonical generated output root required");
    const producers=index.get(path)??[];
    producers.push(output.producer??contract);
    index.set(path,producers);
   }
  }
  GENERATED_OUTPUT_INDEXES_V1.set(contracts,index);
 }
 return index;
}

/** 🏭️ Resolves only one exact ignored output contract to its physical authored task owner. */
export function readBuildGeneratedCompileOutputV1(path,workspaceRoot,contracts,registry,context){
 const local=buildPath(relative(workspaceRoot,path)),safe=value=>typeof value==="string"&&value!=="."&&value!==""&&!isAbsolute(value)&&posix.normalize(value)===value&&!value.startsWith("../")&&!UNSAFE_GENERATED_PATH_V1.test(value);const refusal=()=>Error("Canonical generated import path required: "+JSON.stringify({path:local.slice(0,256),...(typeof context?.importer==="string"?{importer:context.importer.slice(0,256)}:{}),...(typeof context?.specifier==="string"?{specifier:context.specifier.slice(0,256)}:{})}));if(local==="."||local===""||isAbsolute(local)||posix.normalize(local)!==local||local.startsWith("../")||local.includes("\0"))throw refusal();const matches=[];
 const resolution=ownerAdmissionSchema.$defs.generatedImportResolution.const,index=generatedOutputIndexV1(contracts,safe);
 if(!index.size)return undefined;
 const collect=(output,inferred)=>{
  const producers=index.get(output);
  if(producers)for(const producer of producers)matches.push({output,producer,inferred});
 };
 for(let candidate=local;candidate;){
  collect(candidate,false);
  const separator=candidate.lastIndexOf("/");
  candidate=separator<0?"":candidate.slice(0,separator);
 }
 if(posix.extname(local)==="")for(const suffix of resolution.files)collect(local+suffix,true);
 if(!matches.length)return undefined;if(!safe(local))throw refusal();if(matches.length!==1)throw Error("Ambiguous generated compile output producer");const {output,producer,inferred}=matches[0];if(inferred&&resolution.probes.some(suffix=>{const candidate=local+suffix;if(candidate===output)return false;const absolute=join(workspaceRoot,candidate);return index.has(candidate)||existsSync(absolute)&&!lstatSync(absolute).isDirectory();}))throw Error("Ambiguous generated compile output resolution");const separator=producer.target?.lastIndexOf(":"),project=producer.target?.slice(0,separator),target=producer.target?.slice(separator+1);if(!project||!target||!safe(producer.ownerPath)&&producer.ownerPath!==".")throw Error("Exact generated task owner required");const manifest=buildPath(join(producer.ownerPath,PROJECT_BASENAME)),owner=registry?.owners.find(owner=>owner.name===project);if(registry)recordRegistryOwnerV1(workspaceRoot,project,owner);if(registry&&(!owner||owner.root!==producer.ownerPath||owner.manifest!==manifest))throw Error("Canonical generated task owner absent");const declaration=JSON.parse(readPhysicalSource(join(workspaceRoot,manifest)).toString("utf8"));if(declaration.name!==project||!declaration.targets?.[target])throw Error("Authored generated producer task absent");return Object.freeze({project,target:producer.target,manifest,output});
}

/** 🧵️ Shares observed DFS ranges and identical source questions while keeping each target's input classification and bounded scan checkpoints. */
export async function readBuildSourceInputMatchesV1(scans,workspaceRoot,context){
 const matched=new Set(),compileMatched=new Set(),compileByTarget=new Map(),walked=new Map(),matchers=new Map();
 const globRoots=new Map(),recursiveByRoot=new Map(),ignored=new Set([...POLICY.generatedDirectories,".git",".🧬semio",".nx",".cache",".r","node_modules","dist","target","🗑️generated"]);
 let visited=0;
 for(const scan of scans)for(const pattern of scan.patterns){
  if(pattern.startsWith("!")||globRoots.has(pattern))continue;
  const index=pattern.search(/[?*[{(!]/u);
  if(index<0)continue;
  const prefix=pattern.slice(0,index),root=(prefix.endsWith("/")?prefix.slice(0,-1):dirname(prefix))||".",tail=pattern.slice(index);
  const recursive=tail.includes("/")||tail.includes("**")||/[\[({]/u.test(tail);
  globRoots.set(pattern,{root,recursive});
  recursiveByRoot.set(root,recursive||recursiveByRoot.get(root)===true);
 }
 const questions=new Map(),plans=scans.map(scan=>{
  const exclusions=scan.patterns.filter(value=>value.startsWith("!")).map(value=>value.slice(1)),repeats=new Map();
  for(const pattern of scan.patterns){
   if(pattern.startsWith("!"))continue;
   const key=JSON.stringify([pattern,exclusions]),question=questions.get(key)??{remaining:0,paths:undefined};
   question.remaining++;
   questions.set(key,question);repeats.set(pattern,question);
  }
  return {scan,exclusions,repeats};
 });
 const matcherOf=pattern=>{
  let matcher=matchers.get(pattern);
  if(!matcher){matcher=loadedProjectInputs.createBuildInputPathMatcherV1(pattern);matchers.set(pattern,matcher);}
  return matcher;
 };
 const checkpoint=()=>yieldInferenceV1(context,"build:selected-source-inputs",visited);
 const maximumEntries=INFERENCE_LIMITS_V1.selectedSourceCheckpointEntries;
 const walk=async root=>{
  if(walked.has(root))return walked.get(root);
  if(isOpaqueSourcePathV1(root))return {paths:[],start:0,directEnd:0,end:0};
  const pending=[root],paths=[],recursive=recursiveByRoot.get(root);
  while(pending.length){
   const entry=pending.pop();
   if(typeof entry!=="string"){entry.end=paths.length;continue;}
   if(++visited%maximumEntries===0)await checkpoint();
   const directory=entry,absolute=join(workspaceRoot,directory);
   try{if(!lstatSync(absolute).isDirectory())continue;}catch(error){
    if(error.code==="ENOENT"||error.code==="ENOTDIR"||!existsSync(absolute))continue;
    throw error;
   }
   const range=recursiveByRoot.has(directory)&&!walked.has(directory)?{paths,start:paths.length,directEnd:0,end:0}:undefined;
   if(range)pending.push(range);
   for(const entry of readdirSync(absolute,{withFileTypes:true})){
    if(++visited%maximumEntries===0)await checkpoint();
    const path=buildPath(join(directory,entry.name));
    if(entry.isSymbolicLink()||isOpaqueSourcePathV1(path))continue;
    if(entry.isFile())paths.push(path);
    else if(recursive&&entry.isDirectory()&&!ignored.has(entry.name)&&!(EXAMPLE_COLLECTIONS.includes(entry.name)&&directory.split("/").at(-1)!==WORKSPACE_OWNERSHIP.collections.moduleMember))pending.push(path);
   }
   if(range){range.directEnd=paths.length;walked.set(directory,range);}
  }
  const listing=walked.get(root)??{paths,start:0,directEnd:0,end:0};
  walked.set(root,listing);
  return listing;
 };
 for(const {scan,exclusions,repeats} of plans){
  let targetMatches=scan.compile?compileByTarget.get(scan.target):undefined;
  for(const pattern of scan.patterns){
   if(pattern.startsWith("!"))continue;
   visited++;await checkpoint();
   const glob=globRoots.get(pattern),question=repeats.get(pattern),shared=question?.paths;
   if(question&&--question.remaining===0)question.paths=undefined;
   let candidates=shared,start=0,end;
   if(candidates===undefined){
    if(glob===undefined)candidates=existsSync(join(workspaceRoot,pattern))?[pattern]:[];
    else{const listing=await walk(glob.root);candidates=listing.paths;start=listing.start;end=glob.recursive?listing.end:listing.directEnd;}
   }
   const remember=shared===undefined&&question?.remaining>0?(question.paths=[]):undefined,limit=end??candidates.length;
   let include;
   for(let index=start;index<limit;index++){
    const path=candidates[index];
    if(++visited%maximumEntries===0)await checkpoint();
    if(shared===undefined&&(glob===undefined&&isOpaqueSourcePathV1(path)||!(include??=matcherOf(pattern))(path)||exclusions.some(pattern=>matcherOf(pattern)(path))))continue;
    remember?.push(path);
    matched.add(path);
    if(scan.compile){
     compileMatched.add(path);
     if(targetMatches===undefined)compileByTarget.set(scan.target,targetMatches=new Set());
     targetMatches.add(path);
    }
   }
  }
 }
 await checkpoint();
 return {matched,compileMatched,compileByTarget};
}

/** 🚪️ Observes one canonical owner and only its selected executable source closures. */
export async function discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,context,inputNames=[]){
 await libraryBootstrap;const {workspaceRoot}=context,nativeCache=context.nativeCache??new Map(),nativeClosures=context.nativeClosures??new Map();
 if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkResourceLeaseControl(context.phaseControl);checkScriptInvocation(context.invocation);}
 const stage=async phase=>{context.onOwnerDiscoveryPhase?.(phase);await yieldInferenceV1(context,"build:selected-"+phase,0);};
 const dbgT0=performance.now(),dbgMarks=[],dbg=name=>dbgMarks.push(name+"="+Math.round(performance.now()-dbgT0));
 await stage("declaration");dbg("stage");
 askOwnerRegistryV1(workspaceRoot,registry,["row",owner.name]);
 const path=join(workspaceRoot,owner.manifest),source=readPhysicalSource(path);
 const declaration=owner.manifest.endsWith("Cargo.toml")?readToml(path):JSON.parse(source.toString("utf8"));dbg("manifest");
 const authoredTargets=owner.manifest.endsWith("Cargo.toml")?{}:declaration.targets??{},scripts=createScriptInputCache(context),nativeDeclarations=await cargoTargets(owner.root,workspaceRoot,undefined,scripts),dbgCargo=dbg("cargo-targets"),json=owner.manifest.endsWith("Cargo.toml")?{name:declaration.package?.name,root:owner.root,targets:nativeDeclarations}:{...declaration,root:owner.root};
 if(json.name!==owner.name)throw Error("Admitted canonical build owner differs: "+owner.manifest);
 json.targets={...nativeDeclarations,...json.targets,...componentBuildTargets(owner.root,workspaceRoot),...await printDocumentTargets(json,owner.root,workspaceRoot,scripts)};dbg("component-targets");
 askOwnerRegistryV1(workspaceRoot,registry,["providers",owner.root,targets.length>0,[...inputNames]]);
 const contracts=ownerGeneratorContractsV1(workspaceRoot),providers=registry.catalogProviders.filter(provider=>targets.length&&provider.consumerRoots.some(root=>owned(owner.root,root))||inputNames.includes("{workspaceRoot}/"+provider.sourcePath)),dbgP=dbg("providers"),shared=providers.length?await sharedCatalogV1(workspaceRoot,registry,providers,context):{contributions:[],files:[]},dbgS=dbg("shared-catalog"),catalogContributions=shared.contributions;
 scripts.generatedOutput=(path,context)=>readBuildProducedCompileOutputV1(path,workspaceRoot,contracts,registry,context);
 const catalogFiles=shared.files;
 await stage("catalog-targets");dbg("end");if(performance.now()-dbgT0>500)process.stderr.write("[DEBUG] [owner-declaration] "+owner.name+" "+dbgMarks.join(" ")+"\n");
 if(catalogContributions.length&&targets.length){
  if(json.name==="@semio-tech/plugin-registry")json.targets={...json.targets,...playgroundSessionTargets(catalogFiles,workspaceRoot,catalogContributions)};
  if(json.name==="@semio-tech/framework-os-dev")json.targets={...json.targets,...await playgroundPreparationTargets(catalogFiles,workspaceRoot,owner.root,contracts,catalogContributions,context)};
  const catalog=collectPlaygroundCatalog(catalogFiles,workspaceRoot,catalogContributions);json.targets={...json.targets,...playgroundSiteTargetsForAdmissionV1(owner.root,catalog,workspaceRoot,registry,targets),...authoredTargets};
 }
 const selected=Object.fromEntries(targets.map(name=>{if(!json.targets?.[name])throw Error("Unknown admitted build target: "+owner.name+":"+name);const target=targetWithDefaults(json.targets[name],owner.root,existsSync(join(workspaceRoot,owner.root,SCRIPT_BASENAME))),modes=loaded.targetBuildModes(name,target),configuration=(context.selection?.kind==="tasks"?context.selection.tasks.find(task=>task.project===owner.name&&task.target===name)?.configuration:undefined)??modes.defaultConfiguration,overrides=modes.configurations?.[configuration];if(!overrides)throw Error("Unknown selected build configuration: "+configuration);if(Object.keys(overrides).some(key=>["dependsOn","inputs","outputs","configurations","executor","metadata"].includes(key)))throw Error("Build configuration cannot change task identity");buildInputSourceRoleV1(true,false,[target.metadata?.semio?.sourceInputObservation]);return [name,target];})),files=new Set([owner.manifest]),observedFiles=new Set(),catalogProducers=new Set(),dependencies=new Set(),dependencyInputs=new Map(),caretInputs=new Set(),inputRole=buildInputSourceRoleV1(targets.length>0,false,Object.values(selected).map(target=>target.metadata?.semio?.sourceInputObservation));
 const {byName,byPackage,byRoot,relativeProject}=observedOwnerRegistryViewV1(workspaceRoot,registry);
 const addFile=path=>files.add(path),addDependency=path=>{const name=relativeProject(path);if(name&&name!==json.name)dependencies.add(name);},addInput=(project,input)=>{if(!byName.has(project)||typeof input!=="string"||!input)throw Error("Canonical named input owner required");const selected=dependencyInputs.get(project)??new Set();selected.add(input);dependencyInputs.set(project,selected);};
 for(const provider of providers)addFile(provider.sourcePath);for(const file of catalogFiles){readPhysicalSource(join(workspaceRoot,file));observedFiles.add(file);}for(const contribution of catalogContributions){for(const input of contribution.inputs)addFile(input);addFile(contribution.projectRoot+"/"+PROJECT_BASENAME);if(contribution.project!==json.name)catalogProducers.add(contribution.project);}
 await stage("command-sources");
 const commandModules=new Map(),commandResources=new Map();
 for(const [name,target] of Object.entries(selected)){
  const sources=await targetScriptSources(target,workspaceRoot,scripts,owner.root,owner.manifest.endsWith("Cargo.toml")?owner.manifest:json.metadata?.nativeRoot?buildPath(join(json.metadata.nativeRoot,"Cargo.toml")):undefined);
  const compiled=scripts.compiledNames.get(sources.closure)??new Set(),absolute=path=>join(workspaceRoot,path.slice("{workspaceRoot}/".length));commandModules.set(name,new Set([...compiled].map(absolute)));commandResources.set(name,new Set((sources.closure??[]).filter(path=>typeof path==="string"&&!compiled.has(path)).map(absolute)));
  for(const input of sources.closure??[]){if(typeof input==="string"){if(input.startsWith("{workspaceRoot}/"))addFile(input.slice("{workspaceRoot}/".length));}else if(input!==null&&typeof input==="object"&&!Array.isArray(input)&&Object.keys(input).length===2&&typeof input.input==="string"&&Array.isArray(input.projects)&&input.projects.length>0)for(const project of input.projects)addInput(project,input.input);else throw refuseBuildV1("invalid","graph.command-closure-reference-required",{input:JSON.stringify(input).slice(0,256)});}

 }
 const packageFile=join(workspaceRoot,owner.root,"package.json");const packageText=readVanishableTextV1(packageFile);if(packageText!==undefined){const manifest=JSON.parse(packageText);addFile(buildPath(relative(workspaceRoot,packageFile)));for(const name of inputRole==="compile"?Object.keys({...manifest.dependencies,...manifest.peerDependencies,...manifest.optionalDependencies}):[]){const owner=byPackage.get(name);if(owner&&owner!==json.name)dependencies.add(owner);}}
 const nativeRoot=json.metadata?.nativeRoot??owner.root,nativeManifest=join(workspaceRoot,nativeRoot,"Cargo.toml"),nativeTargets=Object.entries(selected).filter(([,target])=>target.metadata?.semio?.execution?.kind!=="source"),nativeObservation=()=>buildNativeSourceObservationV1({selectedSource:Object.values(selected).some(target=>target.metadata?.semio?.execution?.kind==="source"),selectedNative:nativeTargets.length>0,requestedInputs:inputNames.length>0,dependencyInputs:caretInputs.size>0,inputRole});let nativeObserved=false;
 const observeNative=()=>{if(nativeObserved)return;nativeObserved=true;if(existsSync(nativeManifest))for(const root of nativeDependencyRoots(nativeRoot,workspaceRoot,true,nativeCache,nativeClosures)){addFile(root+"/Cargo.toml");const dependency=byRoot.get(root);if(!dependency)throw Object.assign(Error("Canonical native dependency owner required: "+root),{code:"BUILD_SOURCE_AUTHORITY"});if(dependency!==json.name)dependencies.add(dependency);}};
 await stage("native-closure");if(nativeObservation()!=="none")observeNative();
 const declaredCompileEntries=new Map(),compileModules=new Map();
 if(targets.length||inputNames.length){
  await stage("normalization");const normalized=await projectWithDefaults({...json,targets:{...json.targets,...selected}},owner.root,join(workspaceRoot,owner.root),workspaceRoot,contracts,new Map(),undefined,scripts,undefined,"source",targets,inputNames);await stage("native-preparation");if(nativeTargets.length){const preparing={...normalized,targets:Object.fromEntries(nativeTargets.map(([name])=>[name,normalized.targets[name]]))};withNativePreparation(preparing,workspaceRoot,contracts,nativeCache,byRoot,nativeClosures);normalized.targets={...normalized.targets,...preparing.targets};if(preparing.metadata!==undefined)json.metadata=normalized.metadata=preparing.metadata;for(const[name,target]of Object.entries(preparing.targets))if(!Object.hasOwn(json.targets,name))json.targets[name]=target;}
  const named={...WORKSPACE_POLICY.namedInputs,...normalized.namedInputs},scans=[],active=new Set();
  const expand=(values,patterns,target)=>{for(const value of values){if(typeof value!=="string"){
   if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("Closed selected source input required");
   const keys=Object.keys(value);
   if(typeof value.input==="string"&&keys.every(key=>["input","projects","dependencies"].includes(key))){if(value.projects!==undefined){if(!Array.isArray(value.projects)||value.dependencies!==undefined)throw Error("Canonical named input project list required");for(const project of value.projects)addInput(project,value.input);}else if(value.dependencies===true)caretInputs.add(value.input);else if(value.dependencies===undefined||value.dependencies===false)expand([value.input],patterns,target);else throw Error("Closed named input dependency required");}
   else if(typeof value.fileset==="string"&&keys.length===1)expand([value.fileset],patterns,target);
   else if(keys.length===1&&typeof(value.env??value.runtime)==="string"||keys.length===1&&Array.isArray(value.externalDependencies)&&value.externalDependencies.every(name=>typeof name==="string"))continue;
   else if(typeof value.dependentTasksOutputFiles==="string"&&keys.every(key=>["dependentTasksOutputFiles","transitive"].includes(key))&&(value.transitive===undefined||typeof value.transitive==="boolean"))continue;
   else throw Error("Unsupported selected source input object");
   continue;
  }if(value.startsWith("^")){caretInputs.add(value.slice(1));continue;}if(Object.hasOwn(named,value)){const contract=json.metadata?.sourceInputs?.[value];if(contract){const entries=declaredCompileEntries.get(target)??new Set();for(const entry of loaded.readSourceInputContract(join(workspaceRoot,contract)).entries)entries.add(join(workspaceRoot,entry));declaredCompileEntries.set(target,entries);}if(active.has(value))throw Error("Selected named input cycle: "+value);active.add(value);expand(named[value],patterns,target);active.delete(value);}else if(value.includes("{workspaceRoot}")||value.includes("{projectRoot}"))patterns.push(value.replaceAll("{workspaceRoot}/","").replaceAll("{projectRoot}",owner.root));else throw refuseBuildV1("invalid","graph.unknown-source-input",{owner:owner.name,input:value});}};
  for(const name of targets){json.targets[name]=normalized.targets[name];const patterns=[];expand(normalized.targets[name].inputs??["default"],patterns,name);scans.push({target:name,patterns,compile:buildInputSourceRoleV1(true,false,[json.targets[name].metadata?.semio?.sourceInputObservation])==="compile"});}
  const requested=[];expand(inputNames,requested);scans.push({patterns:requested,compile:false});
  if(nativeObservation()!=="none")observeNative();
  context.onOwnerDiscoveryPhase?.("source-inputs");
  const {matched,compileByTarget}=await readBuildSourceInputMatchesV1(scans,workspaceRoot,context);
  for(const path of matched)observedFiles.add(path);
  await stage("source-closure");
  for(const name of new Set([...compileByTarget.keys(),...declaredCompileEntries.keys()])){const declared=declaredCompileEntries.get(name)??new Set(),resources=commandResources.get(name)??new Set(),compiled=commandModules.get(name)??new Set(),modules=new Set([...compileByTarget.get(name)??[]].filter(path=>/\.[cm]?[jt]sx?$/u.test(path)).map(path=>join(workspaceRoot,path)).filter(path=>declared.has(path)||!resources.has(path)||compiled.has(path)));for(const path of declared)if(/\.[cm]?[jt]sx?$/u.test(path))modules.add(path);compileModules.set(name,modules);}
  const modules=[...new Set([...compileModules.values()].flatMap(paths=>[...paths]))];
  for(const input of modules.length?await relativeScriptInputs(modules,workspaceRoot,scripts):[])if(typeof input==="string"&&input.startsWith("{workspaceRoot}/"))addFile(input.slice("{workspaceRoot}/".length));
 }
 const generatedDependencies={},roles=ownerAdmissionSchema.$defs.sourceDemand.const,generated=(producer,path,phase,name)=>{addFile(producer.manifest);for(const source of producer.sources??[])addFile(source);if(producer.project!==json.name)dependencies.add(producer.project);if(name!==undefined){if(producer.target===json.name+":"+name)throw Error("Generated task cannot consume its own output: "+JSON.stringify({target:producer.target.slice(0,256),importer:buildPath(relative(workspaceRoot,path)).slice(0,256),output:producer.output.slice(0,256),phase}));addInput(producer.project,"default");const rows=generatedDependencies[name]??=[];if(!rows.includes(producer.target))rows.push(producer.target);for(const source of producer.sources??[])if(/\.[cm]?[jt]sx?$/u.test(source))enqueue(name,join(workspaceRoot,source),phase);}};
 const compiler=parserTypescriptV1(),host=observedTypeScriptHost(compiler.sys),pending=[],demands=new Map(),visited=new Map(),enqueue=(target,file,role)=>{const rows=demands.get(target)??new Map(),next=buildSourceDemandV1(rows.get(file),role);if(next!==rows.get(file)){rows.set(file,next);demands.set(target,rows);pending.push({target,file});}};let index=0;
 for(const [target,files] of compileModules)for(const file of files)enqueue(target,file,roles.compileInput);for(const [target,files] of declaredCompileEntries)for(const file of files)enqueue(target,file,roles.declaredEntry);for(const [target,files] of commandModules)for(const file of files)enqueue(target,file,roles.command);
 await stage("compile-closure");
  while(pending.length){await yieldInferenceV1(context,"build:selected-compile-imports",++index);const {target,file:absolute}=pending.pop(),phase=demands.get(target).get(absolute),seen=visited.get(target)??new Map();if(seen.get(absolute)===phase)continue;seen.set(absolute,phase);visited.set(target,seen);const path=buildPath(relative(workspaceRoot,absolute));requirePhysicalSourceV1(absolute);const {module,references}=prepareSourceAnalysesV1(),[facts,{runtime}]=observedSourceAnalysisPairV1(module,references,absolute),imports=phase===roles.compileInput?facts.imports:facts.runtimeImports;
  for(const specifier of imports){
   await yieldInferenceV1(context,"build:selected-compile-imports",++index);
   const relativeImport=specifier.startsWith(".");if(!relativeImport){const name=specifier.startsWith("@")?specifier.split("/").slice(0,2).join("/"):specifier.split("/")[0],dependency=byPackage.get(name);if(!dependency)continue;if(dependency!==json.name)dependencies.add(dependency);}
   const resource=buildImportResourceV1(absolute,specifier),producer=relativeImport?scripts.generatedOutput(resource.specifier,{importer:path,specifier}):undefined;if(producer){generated(producer,absolute,phase,target);continue;}const modes=[...runtime.imports.includes(specifier)?[compiler.ModuleKind.ESNext]:[],...runtime.requires.includes(specifier)?[compiler.ModuleKind.CommonJS]:[]];
   for(const mode of modes.length?modes:[compiler.ModuleKind.ESNext]){let resolved=workspacePackageModuleV1(absolute,resource.specifier,workspaceRoot,compiler,host,mode,phase===roles.compileInput&&modes.length===0,(request)=>scripts.generatedOutput?.(request,{target,phase}));try{resolved??=observedRequireResolve(absolute,resource.specifier);}catch(error){if(error.code!=="MODULE_NOT_FOUND")throw error;resolved=compiler.resolveModuleName(resource.specifier,absolute,{moduleResolution:compiler.ModuleResolutionKind.Bundler,allowJs:true,resolveJsonModule:true},host,undefined,undefined,mode).resolvedModule?.resolvedFileName;if(!resolved||phase===roles.command&&resource.source&&/\.d\.[cm]?ts$/u.test(resolved))throw Error("Selected compile source import missing: "+path+" -> "+specifier,{cause:error});}
    if(phase===roles.command&&resource.source&&/\.d\.[cm]?ts$/u.test(resolved))throw Error("Runtime source cannot be a declaration: "+path+" -> "+specifier);
    const typed=phase===roles.compileInput&&resource.source?(workspacePackageModuleV1(absolute,resource.specifier,workspaceRoot,compiler,host,mode,true,(request)=>scripts.generatedOutput?.(request,{target,phase}))??compiler.resolveModuleName(resource.specifier,absolute,{moduleResolution:compiler.ModuleResolutionKind.Bundler,allowJs:true,resolveJsonModule:true},host,undefined,undefined,mode).resolvedModule?.resolvedFileName):undefined;
    for(const file of new Set([resolved,...typed?[typed]:[]])){await yieldInferenceV1(context,"build:selected-compile-imports",++index);const local=buildPath(relative(workspaceRoot,file));if(local===".."||local.startsWith("../")||isAbsolute(local))throw Error("Selected compile source escapes workspace: "+local);const produced=scripts.generatedOutput(file,{importer:path,specifier});if(produced){generated(produced,absolute,phase,target);continue;}requirePhysicalSourceV1(file);addFile(local);if(resource.source&&/\.[cm]?[jt]sx?$/u.test(local)){addDependency(local);enqueue(target,file,phase);}}
   }
  }
 }
 for(const dependency of dependencies)for(const input of caretInputs)addInput(dependency,input);
 for(const producer of catalogProducers)dependencies.add(producer);
 return {project:json,files:[...files].sort(),...(observedFiles.size?{observedFiles:[...observedFiles].filter(file=>!files.has(file)).sort()}:{}),dependencies:[...dependencies].sort(),...(Object.keys(generatedDependencies).length?{generatedDependencies}:{}),dependencyInputs:Object.fromEntries([...dependencyInputs].sort(([a],[b])=>a.localeCompare(b)).map(([name,inputs])=>[name,[...inputs].sort()]))};
}

/** 📜️ Reads the workspace's canonical owner declaration through original physical Source ancestry. */
/** 🧾️ Reads the pinned registry for inference, or the current physical registry when checking a native owner failure. */
export function readBuildOwnerRegistrySourceV1(workspaceRoot, policy) {
  const path=join(workspaceRoot,"📋️build-owners.json"),read=()=>readPhysicalSource(path).toString("utf8");
  return policy==="current"?read():pinnedAuthorityReadV1(path,read);
}

/** 📦️ Resolves an ignored generator or exact authored compiled-package publication to its actual producer. */
const wasmCompileIndicesV1=new WeakMap();
/** 🔌️ The part of a project declaration the Wasm compile index reads: the project name and, per target, its declared Wasm package producer and outputs; any other edit of the declaration moves nothing. */
const wasmProducerDeclarationV1=text=>{const declaration=JSON.parse(text),targets={};for(const[name,target]of Object.entries(declaration.targets??{})){const producer=target?.metadata?.semio?.wasmPackageSourceV1;if(producer?.compileOutputs!==undefined)targets[name]={metadata:{semio:{wasmPackageSourceV1:producer}},outputs:target.outputs};}return{name:declaration.name,targets};};
registerFieldExtractorV1("wasm-producer-declaration",wasmProducerDeclarationV1);
const wasmPackageContractV1=text=>{const manifest=JSON.parse(text);return{name:manifest.name,exports:manifest.exports};};
registerFieldExtractorV1("wasm-package-contract",wasmPackageContractV1);
/** 🪞️ The digest of a Wasm compile output index: every key with its producer, and the namespaces. */
const wasmIndexDigests=new WeakMap();
const wasmIndexDigestV1=index=>{let known=wasmIndexDigests.get(index);if(known===undefined){known=createHash("sha256").update(JSON.stringify([[...index.outputs].map(([key,row])=>[key,row.project,row.target,row.manifest,row.output,row.declarations??null,row.sources]).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0),[...index.namespaces].sort()])).digest("hex");wasmIndexDigests.set(index,known);}return known;};
const wasmFileTextUncachedV1=file=>{if(file.endsWith("📋️project.json")){try{return JSON.stringify(wasmProducerDeclarationV1(nodeFs.readFileSync(file,"utf8")));}catch{return undefined;}}if(file.endsWith("/package.json")){try{return JSON.stringify(wasmPackageContractV1(nodeFs.readFileSync(file,"utf8")));}catch{return undefined;}}return file.endsWith("/Cargo.toml")?"":nodeFs.readFileSync(file,"utf8");};
/** 🧠️ The text a Wasm compile index reads of one file, kept while the stamp of the file (size, both times, inode) is unchanged and none of its times lies within the racy window, so a verification reads and parses an untouched declaration once per process instead of once per question. */
const wasmTextsV1=new Map();
const wasmFileTextV1=file=>{
 let status;try{status=nodeFs.statSync(file);}catch{return undefined;}
 const taken=Date.now(),stamp=status.size+":"+status.mtimeMs+":"+status.ctimeMs+":"+status.ino,known=wasmTextsV1.get(file);
 if(known!==undefined&&known.stamp===stamp)return known.text;
 const text=wasmFileTextUncachedV1(file);
 if(status.mtimeMs<taken-INFERENCE_LIMITS_V1.wasmTextRacyMilliseconds&&status.ctimeMs<taken-INFERENCE_LIMITS_V1.wasmTextRacyMilliseconds){if(wasmTextsV1.size>=INFERENCE_LIMITS_V1.wasmTextMemoEntries)wasmTextsV1.clear();wasmTextsV1.set(file,{stamp,text});}
 return text;
};
/** 🪞️ The digest of the Wasm compile index of one registry, asked once per validation round: the index is a pure function of the registry and of declarations that do not move inside a round, and a stored record asks it once per `global` row (hundreds per owner, each a pass over every owner of the registry). */
const wasmIndexRoundsV1=new WeakMap();
registerOwnerRegistryQuestionV1("wasm-index",(registry,_arguments,workspaceRoot)=>{
 const round=observationRoundV1();let digests;
 if(round!==undefined&&round!==null&&typeof round==="object"){digests=wasmIndexRoundsV1.get(round);if(digests===undefined){digests=new Map();wasmIndexRoundsV1.set(round,digests);}const known=digests.get(workspaceRoot);if(known!==undefined&&known.registry===registry)return known.digest;}
 const digest=wasmIndexDigestV1(wasmCompileOutputIndexV1(workspaceRoot,registry.owners,wasmFileTextV1));
 digests?.set(workspaceRoot,{registry,digest});return digest;
});
const wasmDeclarationV1=(file,format)=>{requirePhysicalSourceV1(file);try{return JSON.stringify(observedFieldV1(file,format,""));}catch{return undefined;}};
export function readBuildProducedCompileOutputV1(path,workspaceRoot,contracts,registry,context){if(registry){let cache=wasmCompileIndicesV1.get(registry);if(!cache){cache=new Map();wasmCompileIndicesV1.set(registry,cache);}const index=memoizedObservedComputationV1(cache,workspaceRoot,()=>wasmCompileOutputIndexV1(workspaceRoot,registry.owners,file=>!existsSync(file)?undefined:file.endsWith("📋️project.json")?wasmDeclarationV1(file,"wasm-producer-declaration"):file.endsWith("/package.json")?wasmDeclarationV1(file,"wasm-package-contract"):file.endsWith("/Cargo.toml")?(requirePhysicalSourceV1(file),""):readPhysicalSource(file).toString("utf8")));observeOwnerRegistryAnswerV1(workspaceRoot,["wasm-index"],wasmIndexDigestV1(index));const wasm=wasmCompileOutputV1(index,path);if(wasm)return wasm;}
 if(!isAbsolute(path))return undefined;
 const generated=readBuildGeneratedCompileOutputV1(path,workspaceRoot,contracts,registry,context);if(generated)return generated;const local=buildPath(relative(workspaceRoot,path));const produced=typeScriptPackageOutputProducerV1(path,workspaceRoot,file=>existsSync(file)?readPhysicalSource(file).toString("utf8"):undefined);if(!produced)return undefined;
 const manifest=buildPath(relative(workspaceRoot,produced.manifest)),root=buildPath(relative(workspaceRoot,produced.packageRoot)),owner=registry?.owners.find(owner=>owner.name===produced.project);if(registry)recordRegistryOwnerV1(workspaceRoot,produced.project,owner);if(registry&&(!owner||owner.root!==root||owner.manifest!==manifest))throw Error("Canonical TypeScript artifact producer absent");
 const packageManifest=join(produced.packageRoot,"package.json");requirePhysicalSourceV1(packageManifest);return Object.freeze({project:produced.project,target:produced.target,manifest,output:local,sources:[manifest,buildPath(relative(workspaceRoot,packageManifest)),buildPath(relative(workspaceRoot,produced.source))]});}
