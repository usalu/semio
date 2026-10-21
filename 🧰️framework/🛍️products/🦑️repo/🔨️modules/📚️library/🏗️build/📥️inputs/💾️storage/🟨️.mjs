import {repositoryCacheBudgetV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import {buildGraphWriterResourceV1} from "./🔒️writer/🟨️.mjs";
import {requireNativePlatformImageWriterV1,requireNativePlatformImageBindingV1,withNativePlatformImageOperationV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import{captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1}from"../🔏️revision/🟨️.mjs";
import{fileURLToPath}from"node:url";
import {join,resolve,dirname,relative,sep,isAbsolute} from "node:path";
import {randomBytes} from "node:crypto";
import {AsyncLocalStorage} from "node:async_hooks";
import {statSync,lstatSync,mkdirSync,rmSync,unlinkSync} from "node:fs";
import{nativeStatfsSyncV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {readdir,lstat,unlink} from "node:fs/promises";
import {withRepositoryCacheStoreV1,withRequiredRepositoryCacheStoreConsumerV1,requireCurrentRepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {checkScriptInvocation,requireIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {openBuildGraphOptionalLedgerV1} from "./📒️ledger/🟨️.mjs";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {acquireResourceLease,acquireQueuedResourceLease,checkResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {openRepositoryCachePartitionV1,repositoryLeaseDirectoryV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
const graphConsumerURLsV1=[new URL(physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),new URL("./🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),new URL("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),new URL("./🔒️writer/🟨️.mjs",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),new URL("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL))],graphConsumerCapturesV1=graphConsumerURLsV1.map((url,index)=>index===0?captureBoundedSourceRevisionV1(fileURLToPath(url),8388608):captureBoundedSourceRevisionV1(fileURLToPath(url),65536,true)),graphConsumerRevisionsV1=graphConsumerCapturesV1.map(row=>row.sha256),schema=JSON.parse(graphConsumerCapturesV1[1].text??""),policy=JSON.parse(graphConsumerCapturesV1[2].text??"");
/** 🧷️ Refuses stale Graph mapping, privately captured policies or its defining Store owner. */
export function requireCurrentRepositoryBuildGraphConsumerV1(){if(implementationPinnedV1())return;requireCurrentRepositoryCacheStoreV1();if(graphConsumerURLsV1.some((url,index)=>captureBoundedSourceRevisionV1(fileURLToPath(url),index===0?8388608:65536).sha256!==graphConsumerRevisionsV1[index]))throw Error("Stale loaded Graph consumer implementation; restart required");}
const ledgerSchema=schema.definitions.OptionalLedger,quotaSchema=schema.definitions.OptionalQuota,datasetSchema=schema.definitions.InferenceDataset;
const writers=new AsyncLocalStorage(),datasets=new AsyncLocalStorage(),selections=new AsyncLocalStorage();

/** 📏️ Observes bounded quota refusal DATA without granting publication capacity. */
function optionalGraphQuotaObservationV1(state){
 try{const object=state!==null&&typeof state==="object"&&!Array.isArray(state),kind=state===null?"null":Array.isArray(state)?"array":["object","number","string","boolean","undefined"].includes(typeof state)?typeof state:"other",schemaValid=validateJsonSchemaSubset(quotaSchema,state,schema).length===0,value=key=>object&&typeof state[key]==="number"&&Number.isFinite(state[key])?state[key]:null;return Object.freeze({version:1,observed:true,kind,fieldCount:object?Object.keys(state).length:null,schemaValid,maximumBytes:value("maximumBytes"),usedBytes:value("usedBytes"),availableBytes:value("availableBytes"),minimumAvailableBytes:value("minimumAvailableBytes"),reason:!schemaValid?"shape":state.usedBytes>state.maximumBytes?"retained":null,headroomSatisfied:schemaValid?state.minimumAvailableBytes<=state.availableBytes:null});}catch{return Object.freeze({version:1,observed:false});}
}

/** 🧮️ Reserves exact temporary peaks before optional retained cache bytes. */
export function createBuildGraphOptionalQuotaV1(state,check=()=>{}){
 if(validateJsonSchemaSubset(quotaSchema,state,schema).length||state.usedBytes>state.maximumBytes)throw Error("Closed optional graph cache quota required: "+JSON.stringify(optionalGraphQuotaObservationV1(state)));
 let used=state.usedBytes,available=state.availableBytes,pending=0;
 return Object.freeze({get usedBytes(){return used;},get pendingBytes(){return pending;},reserve(request){
  check();if(!request||Object.keys(request).sort().join()!=="peakBytes,retainedDeltaBytes"||!Number.isSafeInteger(request.peakBytes)||request.peakBytes<0||!Number.isSafeInteger(request.retainedDeltaBytes)||request.retainedDeltaBytes>request.peakBytes||used+request.retainedDeltaBytes<0)throw Error("Exact optional graph publication peak required");
  if(used+pending+request.peakBytes>state.maximumBytes||available-pending-request.peakBytes<state.minimumAvailableBytes)return undefined;
  pending+=request.peakBytes;let live=true;return Object.freeze({commit(){check();if(!live)throw Error("Original optional graph reservation released");pending-=request.peakBytes;used+=request.retainedDeltaBytes;available-=request.retainedDeltaBytes;live=false;},release(){if(live){pending-=request.peakBytes;live=false;}}});
 }});
}

/** 🏷️ Identifies only recomputable optional leaves, never producer or graph publications. */
export function isBuildGraphOptionalCachePathV1(path){
 return typeof path==="string"&&new RegExp(quotaSchema.optionalPath,"u").test(path);
}

/** 🗂️ Restricts isolated writers to immutable closures and their own inference directory. */
export function buildGraphWriterDirectoryAdmittedV1(scope,namespace){
 if(validateJsonSchemaSubset(ledgerSchema.definitions.WriterDirectory,{scope,namespace},schema).length)throw Error("Closed isolated graph writer directory required");return namespace!=="admission";
}

/** 🧷️ Refuses another writer's mutable optional cache namespace. */
export function buildGraphWriterOptionalPathAdmittedV1(scope,path){
 if(validateJsonSchemaSubset(ledgerSchema.definitions.WriterScope,scope,schema).length||typeof path!=="string")throw Error("Closed isolated graph optional path required");return path.startsWith((scope.kind==="selection"?"selections":"contexts")+"/"+scope.identity+"/")&&isBuildGraphOptionalCachePathV1(path);
}

/** 🪪️ Projects the retained writer namespace without accepting a caller supplied authority. */
function isolatedGraphWriterScopeV1(current){if(!current.namespace)return undefined;const [kind,identity]=current.namespace.split(":");return {kind,identity};}

/** 📏️ Inventories optional regular leaves without following another physical owner. */
async function optionalInventoryV1(image,control){
 const directory=join(image.root,"inference"),entries=[];let count=0;
 const walk=async path=>{checkResourceLeaseControl(control);if(++count>ledgerSchema.protocol.inventoryEntryLimit)throw Error("Finite optional graph inventory exceeded");if(count%64===0){await control.publish({resource:"repository:graph-cache-inventory",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}let status;try{status=await lstat(path);}catch(error){if(error.code==="ENOENT")return;throw error;}if(status.isSymbolicLink()||status.dev!==statSync(image.root).dev)throw Error("Original graph cache physical ancestry required");if(status.isDirectory()){for(const name of(await readdir(path)).sort())await walk(join(path,name));return;}const name=relative(directory,path).split(sep).join("/");if(!isBuildGraphOptionalCachePathV1(name))return;if(!status.isFile()||status.nlink!==1)throw Error("Original optional graph cache leaf required");entries.push({path,bytes:status.size,modified:status.mtimeMs,device:status.dev,inode:status.ino});};
 await walk(directory);return entries;
}

/** 📒️ Awaits original ledger initialization custody without retaining it through producer work. */
async function openOriginalGraphLedgerV1(workspace,original,control){
 const lease=await acquireQueuedResourceLease({directory:repositoryLeaseDirectoryV1(workspace),resource:"repository-graph-optional-ledger:"+resolve(workspace),mode:"exclusive",owner:randomBytes(16).toString("base64url"),control:control,signal:control.signal});
 try{for(;;){try{return await openBuildGraphOptionalLedgerV1({image:original.image,control:control,leaseDirectory:repositoryLeaseDirectoryV1(workspace),maximumBytes:quotaSchema.policy.maximumBytes,minimumAvailableBytes:quotaSchema.policy.minimumAvailableBytes});}catch(error){if(error?.code!=="SQLITE_BUSY"&&error?.errcode!==5&&error?.errno!==5)throw error;checkResourceLeaseControl(control);await control.publish({resource:"repository-graph-optional-ledger",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}}}finally{lease.release();}
}

/** 🚰️ Requests low-water optional headroom before pressure can deny later publication peaks. */
export function buildGraphOptionalRetirementDueV1(usedBytes,availableBytes){
 const policy=quotaSchema.policy;if(!Number.isSafeInteger(usedBytes)||usedBytes<0||!Number.isSafeInteger(availableBytes)||availableBytes<0)throw Error("Exact optional graph cache usage required");
 return usedBytes>policy.lowWaterBytes||availableBytes<policy.minimumAvailableBytes;
}

/** 🚰️ Applies the same pressure recovery to every producer kind. */
export function buildGraphWriterMaintenanceDueV1(request){
 if(validateJsonSchemaSubset(ledgerSchema.definitions.Maintenance,request,schema).length)throw Error("Closed graph writer pressure required");return buildGraphOptionalRetirementDueV1(request.usedBytes,request.availableBytes);
}

/** 🧹️ Reports optional reclamation pressure under already held exclusive custody. */
async function retireOptionalGraphHeadroomV1(image,control,request){
 checkResourceLeaseControl(control);const minimumAvailableBytes=request.minimumAvailableBytes,policy=quotaSchema.policy,started=Date.now();let retiredBytes=0,retiredFiles=0,partial=false;
 await image.recheck();const entries=await optionalInventoryV1(image,control);let remaining=entries.reduce((sum,row)=>sum+row.bytes,0);
 for(const row of entries.sort((a,b)=>a.modified-b.modified||Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)))){checkResourceLeaseControl(control);const free=nativeStatfsSyncV1(image.root);if(remaining<=policy.lowWaterBytes&&free.bavail*free.bsize>=minimumAvailableBytes)break;if(Date.now()-started>=policy.retirementWindowMilliseconds){partial=true;break;}if(retiredFiles%64===0)await image.recheck();const actual=await lstat(requireNativeStoragePathV1(image.root,row.path));if(!actual.isFile()||actual.isSymbolicLink()||actual.nlink!==1||actual.dev!==row.device||actual.ino!==row.inode||actual.size!==row.bytes)throw Error("Quiescent optional graph cache identity changed");const allocated=actual.blocks*512;if(!Number.isSafeInteger(allocated)||allocated<0)throw Error("Exact optional graph allocated bytes required");await unlink(row.path);remaining-=row.bytes;retiredBytes+=allocated;retiredFiles++;if(retiredFiles%64===0){await control.publish({resource:"repository:graph-cache-retirement",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}}
 checkResourceLeaseControl(control);const free=nativeStatfsSyncV1(image.root);return Object.freeze({retainedOptionalBytes:remaining,availableBytes:free.bavail*free.bsize,requiredAvailableBytes:minimumAvailableBytes,retiredBytes,retiredFiles,...(partial?{partial:true}:{})});
}

/** 🔐️ Retires optional leaves only under the original canonical image writer, without acquiring it again. */
export async function retireBuildGraphOptionalHeadroomV1(image,custody,request){requireNativePlatformImageWriterV1(image,custody);if(validateJsonSchemaSubset(quotaSchema.definitions.Reclamation,request,schema).length)throw Error("Closed optional graph reclamation request required");return retireOptionalGraphHeadroomV1(image,custody.control,request);}

/** 🧹️ Reclaims explicit optional leaves under actual exclusive custody and defers while readers remain. */
export async function prepareBuildGraphOptionalHeadroomV1(image,control,request){
 if(validateJsonSchemaSubset(quotaSchema.definitions.Headroom,request,schema).length)throw Error("Closed optional graph headroom request required");await image.recheck();let retired=false;for(;;){const result=await image.withExclusiveMaintenance({version:1,wait:false},async(_port,custody)=>retireOptionalGraphHeadroomV1(image,custody.kind==="kernel-ex"?custody.control:control,request));if(!result.acquired)return retired;const value=result.value;if(value.retiredFiles>0)retired=true;if(value.partial){await control.yieldContinuation();continue;}if(value.retainedOptionalBytes>quotaSchema.policy.maximumBytes||value.availableBytes<value.requiredAvailableBytes)throw Error("Graph cache cannot admit original required publication headroom: "+JSON.stringify(value));return retired;}
}

/** 🚰️ Preserves the ordinary producer low-water and mandatory publication floor. */
export async function maintainBuildGraphOptionalCachesV1(image,control){return prepareBuildGraphOptionalHeadroomV1(image,control,{minimumAvailableBytes:quotaSchema.policy.minimumAvailableBytes});}

/** 📊️ Copies only the current writer's finite accounting DATA while its original custody remains live. */
export function buildGraphWriterOptionalQuotaFactsV1(workspace){requireRepositoryBuildGraphWriterV1(workspace);const quota=writers.getStore().optionalQuota;return Object.freeze({version:1,usedBytes:quota.usedBytes,pendingBytes:quota.pendingBytes});}

/** 📦️ Admits optional cache bytes only within the current original serial writer. */
export function withBuildGraphOptionalPublicationV1(path,plan,write){
 const scope=writers.getStore();if(!scope){if(path.split(sep).join("/").includes("/.cache/b/g/m/"))throw Error("Original optional graph writer required");write();return true;}
 requireRepositoryBuildGraphWriterV1(scope.workspace);requireNativeStoragePathV1(scope.store.root,resolve(path));const name=relative(join(scope.store.root,"inference"),resolve(path)).split(sep).join("/");if(!isBuildGraphOptionalCachePathV1(name))throw Error("Explicit optional graph cache leaf required");const isolated=isolatedGraphWriterScopeV1(scope);if(isolated&&!buildGraphWriterOptionalPathAdmittedV1(isolated,name))throw Error("Original isolated graph optional namespace required");const free=nativeStatfsSyncV1(scope.store.root);if(free.bavail*free.bsize-plan.peakBytes<quotaSchema.policy.minimumAvailableBytes)return false;const reservation=scope.optionalQuota.reserve(plan);if(!reservation)return false;try{if(!scope.ledger.publish(plan,write))return false;reservation.commit();return true;}finally{reservation.release();}
}

/** 🔌️ Closes retained cache descriptors before releasing original image custody. */
export function registerBuildGraphOptionalHandleV1(close){const scope=writers.getStore();if(!scope)return()=>{};scope.handles.add(close);return()=>{scope.handles.delete(close);};}

/** 🪪️ Resolves only the source root admitted by a live original dataset. */
function graphDatasetWorkspaceV1(workspace){
 const root=resolve(workspace),dataset=datasets.getStore();if(!dataset)return root;
 if(!dataset.live||root!==dataset.workspace&&root!==dataset.sourceRoot)throw Error("Original inference dataset lifetime and source root required");return dataset.workspace;
}

/** 🗂️ Names graph facts inside their sole authored finite backing. */
export function repositoryBuildGraphDataDirectoryV1(workspace,namespace){
 if(!schema.enum.includes(namespace))throw Error("Declared bounded graph fact namespace required");
 const root=join(graphDatasetWorkspaceV1(workspace),policy.partitions.graph.directory,"m",namespace),dataset=datasets.getStore(),selection=selections.getStore();if(namespace==="inference"&&selection){if(!selection.live||selection.workspace!==graphDatasetWorkspaceV1(workspace))throw Error("Original selected inference lifetime required");return join(root,"selections",selection.identity);}if(namespace!=="inference"||!dataset)return root;
 if(!dataset.live||dataset.workspace!==graphDatasetWorkspaceV1(workspace))throw Error("Original inference dataset lifetime required");return join(root,"contexts",dataset.identity);
}

/** 🧭️ Bounds namespace writer lock catalogs independently of graph identity growth. */
export {buildGraphWriterResourceV1,buildGraphPublicationWriterResourceV1} from "./🔒️writer/🟨️.mjs";

/** 📥️ Retains an already-issued Graph reader through its actual private dataset mapping and consumer frame. */
export function withRepositoryBuildGraphConsumerV1(workspace,control,body){requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);requireCurrentRepositoryBuildGraphConsumerV1();return withRequiredRepositoryCacheStoreConsumerV1(graphDatasetWorkspaceV1(workspace),"graph",control,async(store,actual)=>{requireCurrentRepositoryBuildGraphConsumerV1();const value=await body(store,actual);requireCurrentRepositoryBuildGraphConsumerV1();return value;});}

/** 🎟️ Holds enforcing storage before any synchronous graph fact producer runs. */
export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return withGraphFactWriterV1(workspace,invocation,phaseControl,body);}

/** 🔒️ Retains one physical inference namespace while shared quota commits stay short. */
async function withGraphFactWriterV1(workspace,invocation,phaseControl,body,namespace){
 requireCurrentRepositoryBuildGraphConsumerV1();requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
 const current=writers.getStore(),owner=graphDatasetWorkspaceV1(workspace);
 if(current?.workspace===owner&&current.store.invocation===invocation&&(!namespace||current.namespace===namespace)){requireRepositoryBuildGraphWriterV1(workspace);requireIssuedResourceControlRefinementV1(current.phaseControl,phaseControl);const nested={...current,live:true,phaseControl};try{return await writers.run(nested,async()=>{checkResourceLeaseControl(phaseControl);const value=await body(current.store,phaseControl);requireRepositoryBuildGraphWriterV1(workspace);return value;});}finally{nested.live=false;}}
 if(datasets.getStore()||current?.workspace===owner&&current.store.invocation!==invocation)throw Error("Original graph writer invocation identity changed");
 const leasePhase=phaseControl,leases=await openRepositoryCachePartitionV1(workspace,schema.definitions.GraphWriterLeaseProtocol.const.partition,leasePhase);let leaseFailed=false;
 try{
  await leases.recheck();const check=()=>{checkResourceLeaseControl(phaseControl);checkResourceLeaseControl(leasePhase);};check();let maintained=false;
  const maintain=async force=>{const image=await openRepositoryCachePartitionV1(workspace,"graph",leasePhase);let failed=false;try{await image.recheck();check();const space=nativeStatfsSyncV1(image.root);if(force||space.bavail*space.bsize<quotaSchema.policy.minimumAvailableBytes){maintained=true;await maintainBuildGraphOptionalCachesV1(image,leasePhase);check();return;}await image.recheck();check();}catch(error){failed=true;throw error;}finally{try{await image.close();}catch(error){if(!failed)throw error;}}};
  if(!current)await maintain(false);
  const resource=namespace?buildGraphWriterResourceV1(workspace,...namespace.split(":")):"repository-graph-facts:"+resolve(workspace),request={directory:repositoryLeaseDirectoryV1(workspace),resource,mode:"exclusive",owner:randomBytes(16).toString("base64url"),control:leasePhase,signal:leasePhase.signal};
  for(;;){
   const result=await withRepositoryCacheStoreV1(workspace,"graph",invocation,leasePhase,async(original,actualControl)=>{
    const checkActual=()=>{check();requireIssuedResourceControlRefinementV1(leasePhase,actualControl);checkResourceLeaseControl(actualControl);};checkActual();
    let held;try{held=await acquireQueuedResourceLease({...request,timeoutMs:0});}catch(error){checkActual();if(error instanceof Error&&error.message==="Resource lease timed out: "+resource){if(current)throw Error("Nested graph writer cannot wait for a different namespace");return{waiting:true};}throw error;}
    let primary;
    try{
     await leases.recheck();checkActual();
     await original.recheck();checkActual();const status=statSync(original.root),ledger=await openOriginalGraphLedgerV1(workspace,original,actualControl);let failed=false;
     try{
      checkActual();const observed=ledger.observe(),space=nativeStatfsSyncV1(original.root);
      if(!current&&!maintained&&buildGraphOptionalRetirementDueV1(observed.usedBytes,space.bavail*space.bsize))return{again:true};
      const optionalQuota=current?.store.invocation===invocation?current.optionalQuota:createBuildGraphOptionalQuotaV1({maximumBytes:quotaSchema.policy.maximumBytes,usedBytes:observed.usedBytes,availableBytes:space.bavail*space.bsize,minimumAvailableBytes:quotaSchema.policy.minimumAvailableBytes},()=>{checkActual();requireRepositoryBuildGraphWriterV1(workspace);}),store=original,scope={workspace:resolve(workspace),store,device:status.dev,inode:status.ino,live:true,handles:new Set(),ledger,optionalQuota,namespace,phaseControl:actualControl};
      try{return{value:await writers.run(scope,async()=>{let failed=false;try{const value=await body(store,actualControl);checkActual();return value;}catch(error){failed=true;throw error;}finally{let closeFailed=false,failure;for(const close of scope.handles)try{close();}catch(error){if(!closeFailed){closeFailed=true;failure=error;}}if(closeFailed&&!failed)throw failure;}})};}finally{scope.live=false;}
     }catch(error){failed=true;throw error;}finally{try{ledger.close();}catch(error){if(!failed)throw error;}}
    }catch(error){primary={error};throw error;}finally{try{held.release();}catch(error){if(!primary)throw error;if(primary.error instanceof Error&&Object.isExtensible(primary.error))try{Object.defineProperty(primary.error,"graphWriterLeaseReleaseFailure",{value:error});}catch{}}}
   });
   await leases.recheck();check();
   if(result.waiting){let primary;const held=await acquireQueuedResourceLease(request);try{check();}catch(error){primary={error};throw error;}finally{try{held.release();}catch(error){if(!primary)throw error;}}continue;}
   if(result.again){await maintain(true);continue;}
   return result.value;
  }
 }catch(error){leaseFailed=true;throw error;}finally{try{await leases.close();}catch(error){if(!leaseFailed)throw error;}}
}

/** 🔒️ Refuses graph fact publication outside live original fixed-volume custody. */
export function requireRepositoryBuildGraphWriterV1(workspace){
 requireCurrentRepositoryBuildGraphConsumerV1();const current=writers.getStore();if(!current?.live||current.workspace!==graphDatasetWorkspaceV1(workspace))throw Error("Original bounded graph writer custody required");checkScriptInvocation(current.store.invocation);requireIssuedResourceControlInvocationV1(current.phaseControl,current.store.invocation);requireIssuedResourceControlRefinementV1(current.store.control,current.phaseControl);checkResourceLeaseControl(current.phaseControl);const actual=statSync(current.store.root);if(actual.dev!==current.device||actual.ino!==current.inode)throw Error("Original bounded graph writer volume changed");
}

/** 📁️ Admits the closed fact directory on the original mounted device before a writer follows it. */
export function repositoryBuildGraphWriterDirectoryV1(workspace,namespace){
 requireRepositoryBuildGraphWriterV1(workspace);const current=writers.getStore(),isolated=isolatedGraphWriterScopeV1(current);if(isolated&&!buildGraphWriterDirectoryAdmittedV1(isolated,namespace))throw Error("Original isolated graph directory required");const path=repositoryBuildGraphDataDirectoryV1(workspace,namespace);requireNativeStoragePathV1(current.store.root,path);try{mkdirSync(path);}catch(error){if(error?.code!=="EEXIST")throw error;}const status=lstatSync(path);if(!status.isDirectory()||status.isSymbolicLink()||status.dev!==current.device)throw Error("Original bounded graph fact directory required");return path;
}

/** 🎯️ Separates mutable selected facts inside the original fixed-volume writer custody. */
export async function withRepositoryBuildGraphSelectionV1(workspace,invocation,phaseControl,identity,body){
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
 if(!/^[a-f0-9]{64}$/u.test(identity)||selections.getStore()||datasets.getStore())throw Error("Closed original selected graph scope required");
 return withGraphFactWriterV1(workspace,invocation,phaseControl,async(store,actualControl)=>{
  const base=repositoryBuildGraphWriterDirectoryV1(workspace,"inference"),directories=[join(base,"selections"),join(base,"selections",identity)];
  for(const path of directories){requireNativeStoragePathV1(store.root,path);try{mkdirSync(path);}catch(error){if(error?.code!=="EEXIST")throw error;}const status=lstatSync(path);if(!status.isDirectory()||status.isSymbolicLink()||status.dev!==writers.getStore().device)throw Error("Original selected graph directory required");}
  const selection={workspace:resolve(workspace),identity,live:true};try{return await selections.run(selection,()=>body(store,actualControl));}finally{selection.live=false;}
 },"selection:"+identity);
}

/** 🧬️ Admits a portable dataset identity without accepting filesystem or ambient authority. */
export function readBuildInferenceDatasetV1(value){
 if(validateJsonSchemaSubset(datasetSchema,value,schema).length)throw Error("Closed original inference dataset required");return Object.freeze({version:1,identity:value.identity,sourceRoot:value.sourceRoot});
}

/** 🧷️ Holds one logical inference namespace inside the actual original graph backing. */
export async function withRepositoryBuildInferenceDatasetV1(workspace,invocation,phaseControl,value,body){
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
 const dataset=readBuildInferenceDatasetV1(value);if(datasets.getStore())throw Error("Nested inference dataset refused");const actual=resolve(workspace),sourceRoot=resolve(actual,dataset.sourceRoot),distance=relative(actual,sourceRoot);if(sourceRoot.length>256||[...sourceRoot].length>256||!distance||distance===".."||distance.startsWith(".."+sep)||isAbsolute(distance)||dataset.sourceRoot.split("/").some(part=>!part||part==="."||part===".."))throw Error("Original inference dataset source root required");for(let path=sourceRoot;;path=dirname(path)){const status=lstatSync(path);if(!status.isDirectory()||status.isSymbolicLink())throw Error("Original inference dataset real source ancestry required");if(path===actual)break;}
 return withGraphFactWriterV1(workspace,invocation,phaseControl,async(store,actualControl)=>{
  const base=repositoryBuildGraphWriterDirectoryV1(workspace,"inference"),directory=join(base,"contexts",dataset.identity);
  {for(const path of[join(base,"contexts"),directory]){requireNativeStoragePathV1(store.root,path);try{mkdirSync(path);}catch(error){if(error?.code!=="EEXIST")throw error;}const status=lstatSync(path);if(!status.isDirectory()||status.isSymbolicLink()||status.dev!==writers.getStore().device)throw Error("Original inference dataset directory required");}
   const scope={workspace:actual,sourceRoot,identity:dataset.identity,live:true},pending=new Set();let retirement;
   const retire=()=>{requireRepositoryBuildGraphWriterV1(workspace);if(!scope.live||datasets.getStore()!==scope)throw Error("Original inference dataset retirement lifetime required");if(retirement)return retirement;const operation=(async()=>{requireNativeStoragePathV1(store.root,directory);const status=lstatSync(directory);if(!status.isDirectory()||status.isSymbolicLink()||status.dev!==writers.getStore().device)throw Error("Original inference dataset retirement directory required");const writer=writers.getStore(),lease=await acquireQueuedResourceLease({directory:repositoryLeaseDirectoryV1(workspace),resource:"repository-graph-optional-ledger:"+actual,mode:"exclusive",owner:randomBytes(16).toString("base64url"),control:actualControl,signal:actualControl.signal});try{for(;;){try{writer.ledger.retire(()=>rmSync(directory,{recursive:true}));break;}catch(error){if(error?.code!=="SQLITE_BUSY"&&error?.errcode!==5&&error?.errno!==5)throw error;checkResourceLeaseControl(actualControl);await actualControl.publish({resource:"repository-graph-optional-retirement",mode:"exclusive",elapsedMs:0});await actualControl.yieldContinuation();}}}finally{lease.release();}})();retirement=operation;pending.add(operation);void operation.then(()=>pending.delete(operation),()=>{});return operation;};
   try{return await datasets.run(scope,async()=>{let failed=false;try{return await body({root:directory,retire},actualControl);}catch(error){failed=true;throw error;}finally{const results=await Promise.allSettled(pending);if(!failed){const failure=results.find(row=>row.status==="rejected");if(failure)throw failure.reason;}}});}finally{scope.live=false;}

  }
 },"dataset:"+dataset.identity);
}

function optionalPhysicalStampV1(row){return[row.dev,row.ino,row.size,row.nlink,row.mtimeNs,row.ctimeNs].join(":");}
async function optionalScopedCandidatesV1(image,control){
 const entries=[],device=lstatSync(image.root,{bigint:true}).dev;let count=0;
 for(const[group,kind]of[["selections","selection"],["contexts","dataset"]]){const base=requireNativeStoragePathV1(image.root,join(image.root,"inference",group));let names;try{const row=await lstat(base,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||row.dev!==device)throw Error("Original optional namespace ancestry required");names=await readdir(base);}catch(error){if(error?.code==="ENOENT")continue;throw error;}for(const identity of names.sort()){checkResourceLeaseControl(control);if(++count>ledgerSchema.protocol.inventoryEntryLimit)throw Error("Finite optional namespace inventory required");if(!/^[a-f0-9]{64}$/u.test(identity))continue;const directory=requireNativeStoragePathV1(image.root,join(base,identity));let row;try{row=await lstat(directory,{bigint:true});}catch(error){if(error?.code==="ENOENT")continue;throw error;}if(!row.isDirectory()||row.isSymbolicLink()||row.dev!==device)continue;entries.push({directory,kind,identity,modified:row.mtimeNs});}}
 return entries.sort((a,b)=>a.modified<b.modified?-1:a.modified>b.modified?1:Buffer.compare(Buffer.from(a.directory),Buffer.from(b.directory)));
}
async function optionalScopedInventoryV1(image,candidate,control){
 const base=join(image.root,"inference"),device=lstatSync(image.root,{bigint:true}).dev,pending=[candidate.directory],directories=[],rows=[];let count=0;
 while(pending.length){checkResourceLeaseControl(control);if(++count>ledgerSchema.protocol.inventoryEntryLimit)throw Error("Finite scoped optional inventory required");if(count%64===0){await control.publish({resource:"repository:graph-scoped-inventory",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}const path=requireNativeStoragePathV1(image.root,pending.pop());let row;try{row=await lstat(path,{bigint:true});}catch(error){if(error?.code==="ENOENT")return undefined;throw error;}if(row.isSymbolicLink()||row.dev!==device)return undefined;if(row.isDirectory()){directories.push({path,identity:[row.dev,row.ino].join(":")});for(const name of(await readdir(path)).sort())pending.push(join(path,name));continue;}const name=relative(base,path).split(sep).join("/");if(!row.isFile()||row.nlink!==1n||row.size>BigInt(quotaSchema.policy.maximumBytes)||!isBuildGraphOptionalCachePathV1(name))return undefined;const allocated=Number(row.blocks)*512;if(!Number.isSafeInteger(allocated)||allocated<0)throw Error("Exact scoped optional allocation required");rows.push({path,stamp:optionalPhysicalStampV1(row),bytes:Number(row.size),allocated,modified:row.mtimeNs});}
 rows.sort((a,b)=>a.modified<b.modified?-1:a.modified>b.modified?1:Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)));return{directories,rows};
}
function unchangedOptionalScopeV1(image,inventory){for(const original of inventory.directories){const row=lstatSync(requireNativeStoragePathV1(image.root,original.path),{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||[row.dev,row.ino].join(":")!==original.identity)throw Error("Original optional namespace directory changed");}}
function unchangedOptionalLeafV1(image,original,parents){for(let path=dirname(original.path);parents.has(path);path=dirname(path)){const row=lstatSync(requireNativeStoragePathV1(image.root,path),{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||[row.dev,row.ino].join(":")!==parents.get(path))throw Error("Original optional namespace directory changed");}const row=lstatSync(requireNativeStoragePathV1(image.root,original.path),{bigint:true});if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1n||optionalPhysicalStampV1(row)!==original.stamp)throw Error("Original scoped optional leaf changed");}
/** 🧹️ Reclaims only quiescent isolated optional namespaces under their actual fact writers and durable ledger. */
export async function recoverBuildGraphOptionalScopesHeadroomV1(workspace,image,control,request){
 requireCurrentRepositoryBuildGraphConsumerV1();if(validateJsonSchemaSubset(quotaSchema.definitions.Headroom,request,schema).length)throw Error("Closed scoped optional headroom required");request=Object.freeze({minimumAvailableBytes:request.minimumAvailableBytes});if(validateJsonSchemaSubset(quotaSchema.definitions.Headroom,request,schema).length)throw Error("Closed scoped optional headroom required");const partition=repositoryCacheBudgetV1(workspace).partitions.graph,leaseDirectory=repositoryLeaseDirectoryV1(workspace);requireNativePlatformImageBindingV1(image,control,{directory:partition.directory,leaseDirectory,filesystemBytes:partition.filesystemBytes,maximumPhysicalBytes:partition.capacityBytes});
 return withNativePlatformImageOperationV1(image,control,async actual=>{
  let retiredBytes=0,retiredFiles=0;const available=()=>{const free=nativeStatfsSyncV1(image.root),value=free.bavail*free.bsize;if(!Number.isSafeInteger(value)||value<0)throw Error("Actual optional graph space required");return value;},result=()=>{const value=Object.freeze({acquired:true,availableBytes:available(),retiredBytes,retiredFiles});if(validateJsonSchemaSubset(schema.definitions.ScopedOptionalRecoveryResult,value,schema).length)throw Error("Closed scoped optional recovery result required");return value;},busy=error=>error?.code==="SQLITE_BUSY"||error?.errcode===5||error?.errno===5;
  const acquire=async resource=>{checkResourceLeaseControl(actual);try{return await acquireResourceLease({directory:leaseDirectory,resource,mode:"exclusive",owner:randomBytes(16).toString("base64url"),timeoutMs:0,control:actual,signal:actual.signal});}catch(error){checkResourceLeaseControl(actual);if(error instanceof Error&&error.message==="Resource lease timed out: "+resource)return undefined;throw error;}};
  checkResourceLeaseControl(actual);await image.recheck();if(available()>=request.minimumAvailableBytes)return result();
  for(const candidate of await optionalScopedCandidatesV1(image,actual)){if(available()>=request.minimumAvailableBytes)break;const held=await acquire(buildGraphWriterResourceV1(workspace,candidate.kind,candidate.identity));if(!held)continue;let ledger,ledgerLease,failed=false;const before=retiredFiles;
   try{await image.recheck();const inventory=await optionalScopedInventoryV1(image,candidate,actual);if(!inventory?.rows.length)continue;ledgerLease=await acquire("repository-graph-optional-ledger:"+resolve(workspace));if(!ledgerLease)continue;ledger=await openBuildGraphOptionalLedgerV1({image,control:actual,leaseDirectory,maximumBytes:quotaSchema.policy.maximumBytes,minimumAvailableBytes:quotaSchema.policy.minimumAvailableBytes});ledger.retire(()=>{checkResourceLeaseControl(actual);unchangedOptionalScopeV1(image,inventory);const parents=new Map(inventory.directories.map(row=>[row.path,row.identity]));for(const row of inventory.rows){if(available()>=request.minimumAvailableBytes)break;checkResourceLeaseControl(actual);unchangedOptionalLeafV1(image,row,parents);unlinkSync(row.path);retiredBytes+=row.allocated;retiredFiles++;}});
   }catch(error){failed=true;checkResourceLeaseControl(actual);if(!busy(error)||retiredFiles!==before)throw error;failed=false;}finally{let closing=false,failure;for(const close of[()=>ledger?.close(),()=>ledgerLease?.release(),()=>held.release()])try{close();}catch(error){if(!closing){closing=true;failure=error;}}if(closing&&!failed)throw failure;}
   await image.recheck();await actual.publish({resource:"repository:graph-scoped-optional-retirement",mode:"exclusive",elapsedMs:0});await actual.yieldContinuation();
  }
  checkResourceLeaseControl(actual);await image.recheck();return result();
 });
}
