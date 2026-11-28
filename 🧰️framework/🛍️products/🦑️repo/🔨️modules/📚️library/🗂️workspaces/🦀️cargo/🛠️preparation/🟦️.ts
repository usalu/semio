import {concurrentEditV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import{physicalSourceOriginV1,type PhysicalSourceModuleMetaV1}from"../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import {parseCargoPreparationStorageV1,type CargoPreparationStorageV1} from "./📦️storage/🟦️.ts";
import {randomBytes,createHash} from "node:crypto";
import {mkdirSync,readFileSync,rmSync,lstatSync,writeFileSync,renameSync,existsSync} from "node:fs";
import synchronizationSchema from "./🧾️synchronization/🧬️schema/🔣️.json" with { type: "json" };
import {fileURLToPath} from "node:url";
import {join,resolve} from "node:path";
import {acquireQueuedResourceLease,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { cargoWorkspaceForManifest, cargoWorkspaceMembers, prepareCargoOwnersControlledV1, publishSelectedCargoWorkspaceMembershipV1, parseCargoPreparationSelectionV1 } from "../🟦️.ts";
import { type CargoPreparationControlV1 } from "./🎛️control/🟦️.ts";
import {cargoPreparationInputV1,assertCargoPreparationObservationCurrentV1} from "./🧾️custody/🟦️.ts";
import {cargoPreparationReplanPolicyV1,CargoPreparationManifestChangedV1} from "./🔁️replan/🟦️.ts";
import {cargoPhysicalObservationsHoldV1,readCargoListingSnapshotV1,seedCargoPriorListingsV1,writeCargoListingSnapshotV1,cargoPreparationCompilerEnvironmentV1,cargoPreparationGlobalConfigurationV1,cargoPreparationRecordPathsV1,cargoToolStampV1,writeCargoPreparationWitnessV1,type CargoPhysicalObservationsV1} from "./👁️witness/🟦️.ts";

export type CargoDependencyPairRunnerV1=(args:string[],cwd:string,signal:AbortSignal)=>Promise<void>;
/** 🔎️ Receives bounded identity acquisition separately from the unchanged update and fetch pair. */
export type CargoDependencyIdentityV1=(root:string,signal:AbortSignal)=>Promise<string>;
const sha=(path:string):string=>{const info=lstatSync(path);if(info.isSymbolicLink()||!info.isFile())throw Error("Preparation source must be a physical file");return createHash("sha256").update(readFileSync(path)).digest("hex");};

function requirePreparedCargoSources(sources: readonly { path: string; sha256: string }[]): void {
  for (const source of sources) {
    let current: string;
    try {
      current = sha(source.path);
    } catch (error) {
      if (error instanceof Error && "code" in error && error.code === "ENOENT") {
        throw concurrentEditV1("vanished", source.path, "Prepared source disappeared", "build");
      }
      throw error;
    }
    if (current !== source.sha256) throw concurrentEditV1("bytes", source.path, "Prepared source changed", "build");
  }
}

/** 🔗️ Replans pre-command manifest edits, then owns update and locked fetch under original exclusive custody. Every run plans, hashes and confirms its receipt under the shared lease, so concurrent runs overlap; only a pass that must run a recipe, write a membership or renew the receipt takes the exclusive lease, and a renewal does not plan again there: it re-verifies the inputs it planned, finds the receipt a peer renewed meanwhile or runs update and locked fetch alone, so the exclusive section is the mutation, not the planning; a selection known to run recipes goes straight to the exclusive lease. A launcher gets a witness of the held receipt when the identity names its tool files.
 * @see ./👁️witness/🟦️.ts */
export async function withPreparedCargoDependencyPairV1(storage:CargoPreparationStorageV1,root:string,manifest:string,control:ResourceLeaseControl,runner:CargoDependencyPairRunnerV1,identity:CargoDependencyIdentityV1,preparation:CargoPreparationControlV1,sourcePaths:readonly string[]=[],packages:readonly string[]=[],toolInputs:readonly string[]=[]):Promise<void> {
 let commit:CargoPreparationCommitV1|undefined;
 if(!existsSync(cargoPreparationRecordPathsV1(storage,root,manifest,packages).receipt+recipeMarkerV1))try{return await preparedPair(storage,root,manifest,control,runner,identity,preparation,sourcePaths,packages,toolInputs,"shared");}catch(error){if(!(error instanceof CargoPreparationExclusiveV1))throw error;commit=error.commit;}
 return preparedPair(storage,root,manifest,control,runner,identity,preparation,sourcePaths,packages,toolInputs,"exclusive",commit);
}
/** 🏷️ Names the hint beside a receipt that the last exclusive pass of its selection ran recipes. */
const recipeMarkerV1=".recipes";
/** 🏷️ Remembers beside the receipt whether the last exclusive pass of a selection ran recipes, so the next run goes straight to the exclusive lease instead of running them under the shared lease first; a hint that cannot be written only costs one shared pass. */
function markCargoPreparationRecipesV1(receiptPath:string,recipes:boolean):void{try{if(recipes)writeFileSync(receiptPath+recipeMarkerV1,"1");else rmSync(receiptPath+recipeMarkerV1,{force:true});}catch{}}
/** 🪜️ Finishes a planned renewal under the exclusive lease in the directory of that pass: it verifies the planned inputs still hold (else it throws the replan signal and the exclusive pass plans again), accepts a receipt a peer wrote meanwhile or runs update and locked fetch. */
type CargoPreparationCommitV1=(directory:string)=>Promise<void>;
/** 🚧️ Signals that a shared pass found work only the exclusive holder may do: a recipe to run, a membership to write, or a receipt to renew (then it carries the planned renewal). */
class CargoPreparationExclusiveV1 extends Error{readonly commit?:CargoPreparationCommitV1;constructor(commit?:CargoPreparationCommitV1){super("Cargo preparation needs the exclusive lease");this.commit=commit;}}
async function preparedPair(storage:CargoPreparationStorageV1,root:string,manifest:string,control:ResourceLeaseControl,runner:CargoDependencyPairRunnerV1,identity:CargoDependencyIdentityV1,preparation:CargoPreparationControlV1,sourcePaths:readonly string[],packages:readonly string[],toolInputs:readonly string[],mode:"shared"|"exclusive",commit?:CargoPreparationCommitV1):Promise<void> {
 const signal=control.signal,started=performance.now(),check=():void=>{signal.throwIfAborted();const remaining=control.remainingMilliseconds();if(remaining===null||!Number.isFinite(remaining)||remaining<=0)throw Error("Original Cargo preparation deadline exhausted");};
 check();if(preparation.signal!==signal)throw Error("Original paired Cargo preparation signal required");parseCargoPreparationStorageV1(storage);parseCargoPreparationSelectionV1({manifest,packages});await control.publish({resource:`cargo-preparation:${root}`,mode,elapsedMs:performance.now()-started});check();await control.yieldContinuation();check();const directory=join(storage.directory,randomBytes(16).toString("base64url")),lease=await acquireQueuedResourceLease({directory:storage.leaseDirectory,resource:`cargo-preparation:${root}`,mode,owner:randomBytes(16).toString("base64url"),signal,control});
 try {
 mkdirSync(directory,{recursive:true});if(commit){try{await commit(directory);return;}catch(error){if(!(error instanceof CargoPreparationManifestChangedV1))throw error;}}
 const sources=[...new Set([fileURLToPath(physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),fileURLToPath(new URL("../🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),fileURLToPath(new URL("./📜️script.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),fileURLToPath(new URL("./🧾️custody/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),fileURLToPath(new URL("./🧾️custody/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),fileURLToPath(new URL("./🧾️synchronization/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),fileURLToPath(new URL("./🔁️replan/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),fileURLToPath(new URL("./🔁️replan/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),fileURLToPath(new URL("../🧬️schema/🏃️invocation/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),fileURLToPath(new URL("../🧬️schema/🛠️preparation/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),...sourcePaths])].map(path=>({path,sha256:sha(path)}));
 const priors=readCargoListingSnapshotV1(storage,root);
 for(let attempt=1;;attempt++){
 try{
 check();requirePreparedCargoSources(sources);
 const observations:CargoPhysicalObservationsV1=new Map();seedCargoPriorListingsV1(observations,priors);const scope=cargoWorkspaceForManifest(root,manifest,observations);
 const prepared=await prepareCargoOwnersControlledV1(root,scope,preparation,packages,directory,observations,mode==="shared"?()=>{throw new CargoPreparationExclusiveV1();}:undefined),scopes=prepared.owners.map(owner=>cargoWorkspaceForManifest(root,owner==="."?"Cargo.toml":owner+"/Cargo.toml",observations));check();if(mode==="shared"&&prepared.recipes.length)throw new CargoPreparationExclusiveV1();const membership=cargoWorkspaceForManifest(root,scope.manifest),published=prepared.recipes.some(recipe=>recipe.outputs.some(output=>resolve(output.path)===resolve(root,membership.manifest)));if(published){try{publishSelectedCargoWorkspaceMembershipV1(root,membership,"check",scopes);}catch(error){if(error instanceof Error&&error.message===`Cargo source membership is stale: ${membership.manifest}`)throw Error(`Prepared output membership conflict: ${membership.manifest}`,{cause:error});throw error;}}else{try{publishSelectedCargoWorkspaceMembershipV1(root,membership,mode==="shared"?"check":"write",scopes);}catch(error){if(mode==="shared"&&error instanceof Error&&error.message===`Cargo source membership is stale: ${membership.manifest}`)throw new CargoPreparationExclusiveV1();throw error;}}
 const ownerRoster=JSON.stringify(scopes.map(scope=>scope.manifest)),members=prepared.owners.map(owner=>{const scope=scopes.find(scope=>scope.directory===owner);if(!scope)throw Error("Prepared workspace disappeared");return{owner,roster:JSON.stringify(prepared.inventories[owner]??cargoWorkspaceMembers(root,scope,observations))};}),inputs=[...new Set([...prepared.manifests,...members.flatMap(member=>(JSON.parse(member.roster) as {manifest:string}[]).map(pkg=>pkg.manifest))])].map(path=>cargoPreparationInputV1(root,resolve(root,path),"file"));
 const current=async():Promise<void>=>{check();requirePreparedCargoSources(sources);for(const input of inputs)if(cargoPreparationInputV1(root,input.path,input.kind).sha256!==input.sha256)throw new CargoPreparationManifestChangedV1(`Prepared manifest changed: ${input.path}`);const currentScopes=scopes.map(scope=>cargoWorkspaceForManifest(root,scope.manifest));if(JSON.stringify(currentScopes.map(scope=>scope.manifest))!==ownerRoster)throw new CargoPreparationManifestChangedV1("Prepared workspace roster changed");for(const member of members)if(!currentScopes.some(scope=>scope.directory===member.owner))throw new CargoPreparationManifestChangedV1("Prepared member roster changed");if(!cargoPhysicalObservationsHoldV1(root,[...observations.values()]))throw new CargoPreparationManifestChangedV1("Prepared member roster changed");for(const recipe of prepared.recipes)await assertCargoPreparationObservationCurrentV1(recipe,preparation);check();};
 await current();writeCargoListingSnapshotV1(storage,root,directory,observations);
 const configuration=[".cargo/config",".cargo/config.toml","rust-toolchain","rust-toolchain.toml"].flatMap(name=>[resolve(root,name),resolve(root,scope.directory,name)]).map(path=>cargoPreparationInputV1(root,path,"file"));
 const tool=await identity(root,signal);await current();if(!tool.trim())throw Error("Cargo tool identity unavailable");
 const hashed={sources,inputs,ownerRoster,members,recipes:prepared.recipes,configuration,globalConfiguration:cargoPreparationGlobalConfigurationV1(process.env),tool,compilerEnvironment:cargoPreparationCompilerEnvironmentV1(process.env)};
 const inputsSha256=createHash("sha256").update(JSON.stringify(hashed)).digest("hex"),receiptPath=cargoPreparationRecordPathsV1(storage,root,manifest,packages).receipt;
 if(mode==="exclusive")markCargoPreparationRecipesV1(receiptPath,prepared.recipes.length>0);
 const settle=async(directory:string,exclusive:boolean):Promise<void>=>{
 const lockBefore=cargoPreparationInputV1(root,resolve(root,scope.lock),"file"),witness=(lockSha256:string):void=>{if(toolInputs.length&&!prepared.recipes.length)writeCargoPreparationWitnessV1(storage,root,manifest,packages,directory,{version:1,receipt:{inputsSha256,lockSha256},lock:resolve(root,scope.lock),hashed,observations:[...observations.values()],tools:toolInputs.map(cargoToolStampV1)});};
 if(existsSync(receiptPath)){
  let receipt:unknown;try{receipt=JSON.parse(readFileSync(receiptPath,"utf8"));}catch{}
  const row=receipt as Record<string,unknown>|undefined,hash=synchronizationSchema.properties.inputsSha256.pattern;
  if(row&&Object.keys(row).sort().join(",")==="inputsSha256,lockSha256,version"&&row.version===1&&typeof row.inputsSha256==="string"&&typeof row.lockSha256==="string"&&new RegExp(hash).test(row.inputsSha256)&&new RegExp(hash).test(row.lockSha256)&&row.inputsSha256===inputsSha256&&row.lockSha256===lockBefore.sha256){witness(row.lockSha256);return;}
 }
 if(!exclusive)throw new CargoPreparationExclusiveV1(async next=>{await current();await settle(next,true);});
 await runner(["update","--workspace","--manifest-path",manifest],root,signal);await current();const lock=cargoPreparationInputV1(root,resolve(root,scope.lock),"file");if(!lock.sha256)throw Error("Prepared update did not publish a lock");await runner(["fetch","--locked","--manifest-path",manifest],root,signal);await current();if(cargoPreparationInputV1(root,lock.path,"file").sha256!==lock.sha256)throw Error("Locked fetch changed its prepared lock");
 const temporary=join(directory,"synchronization.json");writeFileSync(temporary,JSON.stringify({version:1,inputsSha256,lockSha256:lock.sha256}));renameSync(temporary,receiptPath);witness(lock.sha256);
 };
 await settle(directory,mode==="exclusive");return;
 }catch(error){
 signal.throwIfAborted();
 if(!(error instanceof CargoPreparationManifestChangedV1)||attempt>=cargoPreparationReplanPolicyV1.maximumAttempts)throw error;
 console.error(`[cargo-preparation] replan ${attempt}: ${error.message}`);check();await control.publish({resource:`cargo-preparation-replan:${root}:${attempt}`,mode,elapsedMs:performance.now()-started});check();await control.yieldContinuation();check();
 }
 }
 } finally {rmSync(directory,{recursive:true,force:true});lease.release();}
}
