import {join,resolve,dirname,relative,sep,isAbsolute} from "node:path";
import {randomBytes,createHash} from "node:crypto";
import {AsyncLocalStorage} from "node:async_hooks";
import {statSync,lstatSync,mkdirSync,rmSync,statfsSync} from "node:fs";
import {readdir,lstat,unlink} from "node:fs/promises";
import {withRepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {checkScriptInvocation,requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import policy from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {openBuildGraphOptionalLedgerV1} from "./📒️ledger/🟨️.mjs";
import ledgerSchema from "./🧬️schema/📒️ledger.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import quotaSchema from "./🧬️schema/💾️quota.json" with {type:"json"};
import datasetSchema from "./🧬️schema/📦️dataset.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {acquireQueuedResourceLease,checkResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {openRepositoryCachePartitionV1,repositoryLeaseDirectoryV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
const writers=new AsyncLocalStorage(),datasets=new AsyncLocalStorage(),selections=new AsyncLocalStorage();

/** 🧮️ Reserves exact temporary peaks before optional retained cache bytes. */
export function createBuildGraphOptionalQuotaV1(state,check=()=>{}){
 if(validateJsonSchemaSubset(quotaSchema,state).length||state.usedBytes>state.maximumBytes||state.minimumAvailableBytes>state.availableBytes)throw Error("Closed optional graph cache quota required");
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
 if(validateJsonSchemaSubset(ledgerSchema.definitions.WriterDirectory,{scope,namespace},ledgerSchema).length)throw Error("Closed isolated graph writer directory required");return namespace!=="admission";
}

/** 🧷️ Refuses another writer's mutable optional cache namespace. */
export function buildGraphWriterOptionalPathAdmittedV1(scope,path){
 if(validateJsonSchemaSubset(ledgerSchema.definitions.WriterScope,scope,ledgerSchema).length||typeof path!=="string")throw Error("Closed isolated graph optional path required");return path.startsWith((scope.kind==="selection"?"selections":"contexts")+"/"+scope.identity+"/")&&isBuildGraphOptionalCachePathV1(path);
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
 if(validateJsonSchemaSubset(ledgerSchema.definitions.Maintenance,request,ledgerSchema).length)throw Error("Closed graph writer pressure required");return buildGraphOptionalRetirementDueV1(request.usedBytes,request.availableBytes);
}

/** 🧹️ Reclaims explicit optional leaves under actual exclusive custody and defers while readers remain. */
export async function prepareBuildGraphOptionalHeadroomV1(image,control,request){
 if(validateJsonSchemaSubset(quotaSchema.definitions.Headroom,request,quotaSchema).length)throw Error("Closed optional graph headroom request required");const minimumAvailableBytes=request.minimumAvailableBytes;
 await image.recheck();const initial=await optionalInventoryV1(image,control),policy=quotaSchema.policy,space=statfsSync(image.root),used=initial.reduce((sum,row)=>sum+row.bytes,0);
 if(used<=policy.lowWaterBytes&&space.bavail*space.bsize>=minimumAvailableBytes)return false;
 const result=await image.withExclusiveMaintenance({version:1,wait:false},async()=>{await image.recheck();const entries=await optionalInventoryV1(image,control);let remaining=entries.reduce((sum,row)=>sum+row.bytes,0);for(const row of entries.sort((a,b)=>a.modified-b.modified||Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)))){const free=statfsSync(image.root);if(remaining<=policy.lowWaterBytes&&free.bavail*free.bsize>=minimumAvailableBytes)break;await image.recheck();const actual=await lstat(requireNativeStoragePathV1(image.root,row.path));if(!actual.isFile()||actual.isSymbolicLink()||actual.nlink!==1||actual.dev!==row.device||actual.ino!==row.inode||actual.size!==row.bytes)throw Error("Quiescent optional graph cache identity changed");await unlink(row.path);remaining-=row.bytes;await control.publish({resource:"repository:graph-cache-retirement",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}const free=statfsSync(image.root);if(remaining>policy.maximumBytes||free.bavail*free.bsize<minimumAvailableBytes)throw Error("Graph cache cannot admit original required publication headroom: "+JSON.stringify({retainedOptionalBytes:remaining,availableBytes:free.bavail*free.bsize,requiredAvailableBytes:minimumAvailableBytes}));console.log("[DEBUG] Original exclusive graph image retired only optional cache leaves "+JSON.stringify({beforeBytes:entries.reduce((sum,row)=>sum+row.bytes,0),retainedBytes:remaining,availableBytes:free.bavail*free.bsize}));return true;});return result.acquired;
}


/** 🚰️ Preserves the ordinary producer low-water and mandatory publication floor. */
export async function maintainBuildGraphOptionalCachesV1(image,control){return prepareBuildGraphOptionalHeadroomV1(image,control,{minimumAvailableBytes:quotaSchema.policy.minimumAvailableBytes});}

/** 📦️ Admits optional cache bytes only within the current original serial writer. */
export function withBuildGraphOptionalPublicationV1(path,plan,write){
 const scope=writers.getStore();if(!scope){if(path.split(sep).join("/").includes("/.cache/b/g/m/"))throw Error("Original optional graph writer required");write();return true;}
 requireRepositoryBuildGraphWriterV1(scope.workspace);requireNativeStoragePathV1(scope.store.root,resolve(path));const name=relative(join(scope.store.root,"inference"),resolve(path)).split(sep).join("/");if(!isBuildGraphOptionalCachePathV1(name))throw Error("Explicit optional graph cache leaf required");const isolated=isolatedGraphWriterScopeV1(scope);if(isolated&&!buildGraphWriterOptionalPathAdmittedV1(isolated,name))throw Error("Original isolated graph optional namespace required");const free=statfsSync(scope.store.root);if(free.bavail*free.bsize-plan.peakBytes<quotaSchema.policy.minimumAvailableBytes)return false;const reservation=scope.store.optionalQuota.reserve(plan);if(!reservation)return false;try{if(!scope.ledger.publish(plan,write))return false;reservation.commit();return true;}finally{reservation.release();}
}

/** 🔌️ Closes retained cache descriptors before releasing original image custody. */
export function registerBuildGraphOptionalHandleV1(close){const scope=writers.getStore();if(scope)scope.handles.add(close);}

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
export function buildGraphWriterResourceV1(workspace,kind,identity){
 if(!["selection","dataset"].includes(kind)||!/^[a-f0-9]{64}$/u.test(identity))throw Error("Closed graph namespace writer required");
 const shard=createHash("sha256").update(kind+":"+identity).digest()[0]%ledgerSchema.protocol.writerShards;return "repository-graph-facts:"+resolve(workspace)+":shard:"+shard;
}

/** 🎟️ Holds enforcing storage before any synchronous graph fact producer runs. */
export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return withGraphFactWriterV1(workspace,invocation,phaseControl,body);}

/** 🔒️ Retains one physical inference namespace while shared quota commits stay short. */
async function withGraphFactWriterV1(workspace,invocation,phaseControl,body,namespace){
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
 const current=writers.getStore(),owner=graphDatasetWorkspaceV1(workspace);
 if(current?.workspace===owner&&current.store.invocation===invocation&&(!namespace||current.namespace===namespace)){requireRepositoryBuildGraphWriterV1(workspace);requireIssuedResourceControlRefinementV1(current.phaseControl,phaseControl);const nested={...current,live:true,phaseControl};try{return await writers.run(nested,async()=>{checkResourceLeaseControl(phaseControl);const value=await body(current.store,phaseControl);requireRepositoryBuildGraphWriterV1(workspace);return value;});}finally{nested.live=false;}}
 if(datasets.getStore()||current?.workspace===owner&&current.store.invocation!==invocation)throw Error("Original graph writer invocation identity changed");
 return withRepositoryCacheStoreV1(workspace,"leases",invocation,phaseControl,async(leases,leasePhase)=>{
  const check=()=>{checkResourceLeaseControl(phaseControl);checkResourceLeaseControl(leasePhase);};check();
  const held=await acquireQueuedResourceLease({directory:repositoryLeaseDirectoryV1(workspace),resource:namespace?buildGraphWriterResourceV1(workspace,...namespace.split(":")):"repository-graph-facts:"+resolve(workspace),mode:"exclusive",owner:randomBytes(16).toString("base64url"),control:leasePhase,signal:leasePhase.signal});
  let primary;
  try{
   check();
   let maintained=false;
   const maintain=async force=>{const image=await openRepositoryCachePartitionV1(workspace,"graph",leasePhase);let failed=false;try{await image.recheck();check();const space=statfsSync(image.root);if(force||space.bavail*space.bsize<quotaSchema.policy.minimumAvailableBytes){maintained=true;await maintainBuildGraphOptionalCachesV1(image,leasePhase);check();}}catch(error){failed=true;throw error;}finally{try{await image.close();}catch(error){if(!failed)throw error;}}};
   if(!current)await maintain(false);
   for(;;){
    let retry=false;
    const value=await withRepositoryCacheStoreV1(workspace,"graph",invocation,leasePhase,async(original,actualControl)=>{
     const checkActual=()=>{check();requireIssuedResourceControlRefinementV1(leasePhase,actualControl);checkResourceLeaseControl(actualControl);};checkActual();
     await original.recheck();checkActual();const status=statSync(original.root),ledger=await openOriginalGraphLedgerV1(workspace,original,actualControl);let failed=false;
     try{
      checkActual();const observed=ledger.observe(),space=statfsSync(original.root);
      if(!current&&!maintained&&buildGraphOptionalRetirementDueV1(observed.usedBytes,space.bavail*space.bsize)){retry=true;return;}
      const optionalQuota=current?.store.invocation===invocation?current.store.optionalQuota:createBuildGraphOptionalQuotaV1({maximumBytes:quotaSchema.policy.maximumBytes,usedBytes:observed.usedBytes,availableBytes:space.bavail*space.bsize,minimumAvailableBytes:quotaSchema.policy.minimumAvailableBytes},()=>{checkActual();requireRepositoryBuildGraphWriterV1(workspace);}),store=Object.freeze({...original,optionalQuota}),scope={workspace:resolve(workspace),store,device:status.dev,inode:status.ino,live:true,handles:new Set(),ledger,namespace,phaseControl:actualControl};
      try{return await writers.run(scope,async()=>{let failed=false;try{const value=await body(store,actualControl);checkActual();return value;}catch(error){failed=true;throw error;}finally{let closeFailed=false,failure;for(const close of scope.handles)try{close();}catch(error){if(!closeFailed){closeFailed=true;failure=error;}}if(closeFailed&&!failed)throw failure;}});}finally{scope.live=false;}
     }catch(error){failed=true;throw error;}finally{try{ledger.close();}catch(error){if(!failed)throw error;}}
    });
    if(!retry)return value;
    await maintain(true);
   }
  }catch(error){primary={error};throw error;}finally{try{held.release();}catch(error){if(!primary)throw error;if(primary.error instanceof Error&&Object.isExtensible(primary.error))try{Object.defineProperty(primary.error,"graphWriterLeaseReleaseFailure",{value:error});}catch{}}}
 });
}

/** 🔒️ Refuses graph fact publication outside live original fixed-volume custody. */
export function requireRepositoryBuildGraphWriterV1(workspace){
 const current=writers.getStore();if(!current?.live||current.workspace!==graphDatasetWorkspaceV1(workspace))throw Error("Original bounded graph writer custody required");checkScriptInvocation(current.store.invocation);requireIssuedResourceControlInvocationV1(current.phaseControl,current.store.invocation);requireIssuedResourceControlRefinementV1(current.store.control,current.phaseControl);checkResourceLeaseControl(current.phaseControl);const actual=statSync(current.store.root);if(actual.dev!==current.device||actual.ino!==current.inode)throw Error("Original bounded graph writer volume changed");
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
 if(validateJsonSchemaSubset(datasetSchema,value).length)throw Error("Closed original inference dataset required");return Object.freeze({version:1,identity:value.identity,sourceRoot:value.sourceRoot});
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
