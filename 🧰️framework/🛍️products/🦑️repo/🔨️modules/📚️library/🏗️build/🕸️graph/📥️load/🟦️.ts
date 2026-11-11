import type {BuildGraphV1} from "../📋️model/🟦️.ts";
import {buildGraphRetryDiagnosticV1} from "../🔁retry/🟨️.mjs";
import {REVISION_POLICY_V1,captureBoundedSourceRevisionV1,clearPinnedAuthorityV1,physicalSourceOriginV1,withImplementationPinV1,type PhysicalSourceModuleMetaV1} from "../../📥️inputs/🔏️revision/🟨️.mjs";
import * as observationV1 from "../../👁️observation/🟨️.mjs";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withRepositoryBuildGraphConsumerV1} from "../../📥️inputs/💾️storage/🟨️.mjs";
import {reconcileBuildOwnerRegistry} from "../🧾️registry-consistency/🟦️.ts";
import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,type BuildGraphSelectionV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "../🚪️admission/🟦️.ts";
import {selectLiteralBuildProjectsV1} from "../../🧭️selection/📥️options/🟦️.ts";
import {readdir,readFile,open,mkdir,writeFile,rename,lstat,rm} from "node:fs/promises";
import {constants} from "node:fs";
import {withArtifactDescriptorV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import {join,dirname,isAbsolute} from "node:path";
import {randomUUID,randomBytes,createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {checkScriptInvocation,advanceScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireQueuedResourceLease} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {decodeGraphRecordsV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🟨️.mjs";
import {readGenerationExternalInputsV1,resolveGenerationExternalInputV1,canonicalJsonV1 as canonicalGenerationJsonV1} from "../🕰️generations/🟨️.mjs";
import {loadGenerationGraphV1,type GenerationLoadStatsV1} from "../🕰️generations/🔌️load/🟨️.mjs";
import {graphSnapshotPolicyV1,GRAPH_LIMITS_V1,graphDeclarationV1,graphPinSchemaV1,buildGraphProducerAuthorityV1,loadedGraphModuleAuthorityV1,graphSelfUrlsV1} from "../🔐️authority/🟦️.ts";
import {portableInputs,buildGraphMembersV1,workspaceBuildFilesV1,buildGraphMembershipV1,buildGraphSourceFactsHoldV1} from "../📁️workspace/🟦️.ts";
import {readBuildGraphV1,buildGraphPathV1} from "../📜️contract/🟦️.ts";
import {inventoryDirectoriesV1} from "../🕊️release/🟦️.ts";
import {discoverBuildGraphV1} from "../🧭️discovery/🟦️.ts";
import {generationHostV1,requireEqualBuildGraphsV1} from "../🕰️host/🟦️.ts";
import {boundGenerationPinV1} from "../📍️pin/🟦️.ts";
import {withBuildGraphStateV1} from "../🛑️custody/🟦️.ts";


type OwnerRegistryInvalidationV1={readonly kind:"invalidated";readonly reason:"owner-registry"};

const ownerRegistryInvalidationV1:OwnerRegistryInvalidationV1=Object.freeze({kind:"invalidated",reason:"owner-registry"});

const buildGraphErrorCodeV1=(error:unknown):unknown=>error!==null&&typeof error==="object"&&"code" in error?error.code:undefined;

const isOwnerRegistryInvalidationV1=(value:unknown):value is OwnerRegistryInvalidationV1=>value===ownerRegistryInvalidationV1;


/** 📌️ Reads immutable snapshot bytes while the existing original Graph store and exact content reader lease remain held. */
async function withRetainedGraphSnapshotV1<T>(workspace:string,store:RepositoryCacheStoreV1,path:string,control:ResourceLeaseControl,run:(source:string)=>Promise<T>):Promise<T>{
 const policy=graphSnapshotPolicyV1,lease=await acquireQueuedResourceLease({directory:join(workspace,".cache/b/l/m/data"),resource:"build-graph:"+path,mode:"shared",owner:randomBytes(16).toString("base64url"),control,signal:control.signal});let failed=false;
 try{await store.recheck();checkResourceLeaseControl(control);const directory=dirname(path),parent=await lstat(directory,{bigint:true}),before=await lstat(path,{bigint:true});checkResourceLeaseControl(control);if(!parent.isDirectory()||parent.isSymbolicLink()||!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<0n||before.size>BigInt(policy.maximumSnapshotBytes))throw Error("Original bounded Graph snapshot required");
 return await withArtifactDescriptorV1(()=>open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0)),async file=>{const verify=async()=>{await store.recheck();checkResourceLeaseControl(control);requireNativeStoragePathV1(store.root,path);const linked=await lstat(path,{bigint:true}),opened=await file.stat({bigint:true}),actualParent=await lstat(directory,{bigint:true});checkResourceLeaseControl(control);if(!actualParent.isDirectory()||actualParent.isSymbolicLink()||actualParent.dev!==parent.dev||actualParent.ino!==parent.ino||[linked,opened].some(row=>!row.isFile()||row.isSymbolicLink()||["dev","ino","size","nlink","mtimeNs"].some(key=>row[key as keyof typeof row]!==before[key as keyof typeof before])))throw Error("Retained Graph snapshot descriptor changed");};await verify();const bytes=Buffer.alloc(Number(before.size));try{let offset=0;while(offset<bytes.length){await verify();const row=await file.read(bytes,offset,Math.min(GRAPH_LIMITS_V1.snapshotReadChunkBytes,bytes.length-offset),offset);checkResourceLeaseControl(control);if(!row.bytesRead)throw Error("Retained Graph snapshot shortened");offset+=row.bytesRead;await control.yieldContinuation();}if((await file.read(Buffer.alloc(1),0,1,bytes.length)).bytesRead)throw Error("Retained Graph snapshot grew");await verify();const value=await run(new TextDecoder("utf-8",{fatal:true}).decode(bytes));await verify();return value;}finally{bytes.fill(0);}});
 }catch(error){failed=true;throw error;}finally{try{lease.release();}catch(error){if(!failed)throw error;}}
}


/** 🏪️ Delivers the pinned reusable graph only through its existing private Store consumer and retained exact content descriptor. */
export function withRepositoryBuildGraphPublicationV1<T>(root:string,control:ResourceLeaseControl,body:(graph:BuildGraphV1,digest:string,actual:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 const pin={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST};if(validateJsonSchemaSubset(graphPinSchemaV1,pin).length||!isAbsolute(pin.path!))throw Error("Original immutable build graph path and digest required");
 return withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{await store.recheck();checkResourceLeaseControl(actual);const expected=requireNativeStoragePathV1(store.root,join(store.root,"snapshots",pin.digest+".json"));if(pin.path!==expected)throw Error("Original retained Graph snapshot identity required");return withRetainedGraphSnapshotV1(root,store,expected,actual,async source=>{if(createHash("sha256").update(source).digest("hex")!==pin.digest)throw Error("Pinned build graph digest differs from original authority");let reads=0;const records=dirname(expected),graph=await decodeGraphRecordsV1(Buffer.from(source),async digest=>{if(++reads%64===0)await store.recheck();checkResourceLeaseControl(actual);const path=requireNativeStoragePathV1(store.root,join(records,digest+".gz")),status=await lstat(path);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>graphSnapshotPolicyV1.maximumSnapshotBytes)throw Error("Original bounded Graph record required");return readFile(path);},{policy:graphSnapshotPolicyV1.records as never}) as BuildGraphV1;checkResourceLeaseControl(actual);const value=await body(graph,pin.digest!,actual);await store.recheck();checkResourceLeaseControl(actual);return value;});});
}


/** 🎬️ Resolves one candidate while keeping every candidate's memory within its own finite attempt. */
async function loadBuildGraphAttemptV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean;sealedOnly?:boolean},registry?:BuildOwnerRegistryV1,scope?:string):Promise<BuildGraphV1|OwnerRegistryInvalidationV1|undefined> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
  const invalidated=(reason:string,details:Record<string,unknown>={})=>{if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true"){let diagnostic;try{diagnostic=buildGraphRetryDiagnosticV1(root,reason,details,path=>captureBoundedSourceRevisionV1(path,REVISION_POLICY_V1.maximumSourceBytes).sha256);}catch{diagnostic=buildGraphRetryDiagnosticV1(root,reason);}process.stderr.write("[graph-retry] "+JSON.stringify(diagnostic)+"\n");}return undefined;};
  const phaseRows:{phase:string;milliseconds:number}[]=[],timing=(phase:string,started:number,details:Record<string,unknown>={})=>{const row={phase,milliseconds:Math.round(performance.now()-started),...details};phaseRows.push(row);if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1"||process.env.SEMIO_GRAPH_PHASES==="1")process.stderr.write("[graph-phase] "+JSON.stringify(row)+"\n");};
  const image=async(source:string)=>{let started=performance.now();const decoded=JSON.parse(source);timing("graph-json-parse",started,{bytes:Buffer.byteLength(source)});started=performance.now();const expanded=await portableInputs(root,decoded,invocation,phaseControl);timing("graph-input-expand",started);started=performance.now();const graph=readBuildGraphV1(expanded);timing("graph-schema-validation",started,{projects:Object.keys(graph.projects).length,dependencies:graph.dependencies.length});return graph;};
  if(!registry)throw Error("Original canonical graph owner registry required");
  const selected=options.selection.kind!=="all",path=scope?join(dirname(buildGraphPathV1(root)),"selected",scope+".json"):buildGraphPathV1(root),authorityPath=scope?join(dirname(path),scope+".authority.json"):join(dirname(path),graphDeclarationV1("../🧬️schema/📣️publication.json").properties.authority.const);
  if(options.sealedOnly){if(selected)return undefined;try{if(!buildGraphProducerAuthorityV1(JSON.parse(await readFile(authorityPath,"utf8")),loadedGraphModuleAuthorityV1))return undefined;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||error instanceof SyntaxError)return undefined;throw error;}}
  const observation=observationV1,[producer]=await Promise.all([import("../📥️inference/🟨️.mjs"),import("../../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),import("../📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);
  const inventory=async()=>{let started=performance.now();const captured=selected?await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{const current=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(buildGraphScopeKeyV1(options.selection,current)!==scope)return ownerRegistryInvalidationV1;const nativeCache=new Map(),nativeClosures=new Map();return closeBuildOwnerAdmissionV1(options.selection,registry!,async(owner,targets,inputs)=>{if(invocation)checkScriptInvocation(invocation);return producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,invocation,phaseControl,selection:options.selection,nativeCache,nativeClosures},inputs);});},{signal:invocation?.control.signal})):undefined,candidate=captured?.value;if(isOwnerRegistryInvalidationV1(candidate))return candidate;const admission=candidate as BuildOwnerAdmissionV1|undefined,files=admission?.files??buildGraphMembersV1(await workspaceBuildFilesV1(root,invocation));timing("workspace-catalog-lookup",started,{files:files.length,directories:inventoryDirectoriesV1.get(files)?.length??0});started=performance.now();const digest=await buildGraphMembershipV1(root,files,invocation);timing("workspace-membership-hash",started);return {files,digest,admission,observations:captured?.observations??[]};};
  let initial:Awaited<ReturnType<typeof inventory>>;try{initial=await inventory();}catch(error){if(buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return invalidated("membership");throw error;}
  if(isOwnerRegistryInvalidationV1(initial)){invalidated("owner-registry");return initial;}
  const {files,digest:before}=initial;
  let phaseStarted=performance.now();
  await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();timing("source-parser-bootstrap",phaseStarted);
  let cachedReceipt:any;try{cachedReceipt=JSON.parse(await readFile(authorityPath,"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"&&!(error instanceof SyntaxError))throw error;}
  if(cachedReceipt)try{
    phaseStarted=performance.now();timing("source-authority-read-parse",phaseStarted,{observations:cachedReceipt.observations?.length??0});phaseStarted=performance.now();
    const cachedReceiptValid=buildGraphProducerAuthorityV1(cachedReceipt,loadedGraphModuleAuthorityV1)&&cachedReceipt.membership===before;timing("source-authority-schema",phaseStarted,{admitted:cachedReceiptValid});phaseStarted=performance.now();const factsHold=cachedReceiptValid&&await buildGraphSourceFactsHoldV1(cachedReceipt.observations,invocation);timing("source-authority-facts",phaseStarted,{holds:factsHold});
    if(factsHold){
      phaseStarted=performance.now();const source=await readFile(path,"utf8");timing("graph-file-read",phaseStarted,{bytes:Buffer.byteLength(source)});phaseStarted=performance.now();const digest=createHash("sha256").update(source).digest("hex");timing("graph-content-hash",phaseStarted);if(digest===cachedReceipt.graph){const graph=await image(source);if(selected&&JSON.stringify(graph.admission?.selection)!==JSON.stringify(options.selection))throw Error("Original selected publication scope differs");if(await buildGraphSourceFactsHoldV1([...initial.observations,...cachedReceipt.observations],invocation))return graph;return undefined;}
    }
  }catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  if(options.sealedOnly)return undefined;
  let captured:{value:BuildGraphV1|OwnerRegistryInvalidationV1;observations:readonly unknown[]},failedFacts:unknown[]=[],failedAuthority:unknown[]=[];
  phaseStarted=performance.now();
  try{captured=await observation.captureObservedSourceInputsV1(async()=>{
    for(const source of [...graphSelfUrlsV1(),new URL("../../🧬️schema/📣️publication.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🧬️schema/📥️authority.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../⚡️caching/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../🗂️workspaces/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../📤️arguments/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))])observation.observedFs.readFileSync(source);
    const admittedRegistry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(JSON.stringify(admittedRegistry)!==JSON.stringify(registry))return ownerRegistryInvalidationV1;
    for(const source of [new URL("../../🧬️schema/🚪️selection.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🧬️schema/📋️owners.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🧬️schema/🚪️admission.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🚪️admission/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))])observation.observedFs.readFileSync(source);
    return discoverBuildGraphV1(root,options.selection,registry!,invocation,phaseControl,files,initial.admission,true);
  },(rows:unknown[],authority:unknown[])=>{failedFacts=rows;failedAuthority=authority;});}
  catch(error){if(invocation)checkScriptInvocation(invocation);if(buildGraphErrorCodeV1(error)==="BUILD_AUTHORITY_CHANGED")throw error;let current;try{current=await inventory();}catch(recheck){if(buildGraphErrorCodeV1(recheck)==="BUILD_MEMBERSHIP_CHANGED")return invalidated("membership",{before});throw recheck;}if(isOwnerRegistryInvalidationV1(current))throw error;const stale:{kind:string;path:string;fact:unknown}[]=[],holds=await buildGraphSourceFactsHoldV1(failedFacts,invocation,row=>{if(stale.length<8)stale.push(row);});if(current.digest===before&&holds)throw error;return invalidated("producer-error",{before,after:current.digest,stale});}
  if(isOwnerRegistryInvalidationV1(captured.value)){invalidated("owner-registry",{before});return captured.value;}
  timing("graph-original-inference",phaseStarted,{observations:captured.observations.length});
  let final:Awaited<ReturnType<typeof inventory>>;try{final=await inventory();}catch(error){if(buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return invalidated("membership",{before});throw error;}
  if(isOwnerRegistryInvalidationV1(final)){invalidated("owner-registry",{before});return final;}
  const after=final.digest;
  captured.observations=[...initial.observations,...captured.observations];
  const stale:{kind:string;path:string;fact:unknown}[]=[],holds=await buildGraphSourceFactsHoldV1(captured.observations,invocation,row=>{if(stale.length<8)stale.push(row);});
  if(selected&&JSON.stringify(initial.admission)!==JSON.stringify(final.admission))return invalidated("admission",{before:createHash("sha256").update(JSON.stringify(initial.admission)).digest("hex"),after:createHash("sha256").update(JSON.stringify(final.admission)).digest("hex"),stale});
  if(before!==after||!holds)return invalidated(before!==after?"membership":"observed-facts",{before,after,stale});
  const publishStarted=performance.now(),compact=captured.value as BuildGraphV1,source=JSON.stringify(compact),digest=createHash("sha256").update(source).digest("hex"),authority=JSON.stringify({version:3,module:loadedGraphModuleAuthorityV1,membership:after,graph:digest,observations:captured.observations});timing("graph-serialize",publishStarted,{graphBytes:Buffer.byteLength(source),authorityBytes:Buffer.byteLength(authority)});if(Buffer.byteLength(source)>graphSnapshotPolicyV1.maximumSnapshotBytes||Buffer.byteLength(authority)>graphSnapshotPolicyV1.maximumSnapshotBytes)throw Error("Build graph cache exceeds its finite publication limit: "+Buffer.byteLength(source)+" graph bytes, "+Buffer.byteLength(authority)+" authority bytes, "+graphSnapshotPolicyV1.maximumSnapshotBytes+" allowed");await mkdir(dirname(path),{recursive:true});const expandStarted=performance.now(),graph=readBuildGraphV1(await portableInputs(root,compact,invocation,phaseControl));timing("graph-expansion",expandStarted);
  const temporary=path+"."+randomUUID(),receipt=authorityPath+"."+randomUUID();try{await writeFile(temporary,source,{flag:"wx"});await writeFile(receipt,authority,{flag:"wx"});if(invocation)checkScriptInvocation(invocation);if(!await buildGraphSourceFactsHoldV1(captured.observations,invocation,row=>{if(stale.length<8)stale.push(row);}))return invalidated("publication-facts",{before,after,stale});timing("graph-publication",publishStarted);process.stderr.write("[graph-phase] "+JSON.stringify({summary:phaseRows.map(row=>[row.phase,row.milliseconds])})+"\n");await rename(temporary,path);await rename(receipt,authorityPath);}finally{await Promise.all([rm(temporary,{force:true}),rm(receipt,{force:true})]);}
  return graph;
}

/** 🔓️ Serves a current sealed graph to any number of shared readers: only a cache-store reader is held, the graph writer and its exclusive lease are never requested, so readers never queue behind a builder that is still inferring. */
async function readSealedBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean},registry:BuildOwnerRegistryV1):Promise<BuildGraphV1|OwnerRegistryInvalidationV1|undefined> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  return withRepositoryCacheStoreV1(root,"graph",invocation,phaseControl,async(store,actualControl)=>{await store.recheck();checkResourceLeaseControl(actualControl);return loadBuildGraphAttemptV1(root,invocation,actualControl,{...options,sealedOnly:true},registry,undefined);});
}


/** 🔌️ Requires the pinned selected view to carry exactly the requested canonical external roots. */
export function requireBuildGraphExternalInputsV1(graph:BuildGraphV1,inputs:readonly string[]):readonly string[]{const requested=[...new Set(readGenerationExternalInputsV1(inputs).map(name=>{const key=resolveGenerationExternalInputV1(graph.externalNodes,name);if(key===undefined)throw Error("Pinned build graph lacks external input: "+name);return key;}))].sort();if(JSON.stringify(requested)!==JSON.stringify(graph.admission?.externalInputs??[]))throw Error("Original selected external input admission required");return requested;}


/** 💾️ Publishes only a current graph and releases each invalidated candidate before retrying. */
/** 🪪️ The native roots a refused inference names because no registry row owns them yet (the two refusals of the native closure), whatever wrapper carried the message across the worker farm. */
export function pendingOwnerRootsV1(message:string):string[]{
  return [...new Set([...message.matchAll(/Native dependency has no build project owner: (.+?) \(required by|Canonical native dependency owner required: (\S+)/gu)].map(match=>(match[1]??match[2])!))];
}


/** 🤝️ Adopts the crates a refused inference named into the registry before the graph is planned again: the registry consistency law's reconciler, scoped to exactly those roots, gives a crate that has a manifest but no row its row. Returns whether a row was added, so the caller plans once more only when something changed. */
export function adoptPendingBuildOwnersV1(root:string,message:string):boolean{
  const roots=pendingOwnerRootsV1(message);
  return roots.length>0&&reconcileBuildOwnerRegistry(root,{scope:roots,write:true}).added.length>0;
}


/** 🕸️ Loads the graph; a refusal that names crates without an owner row adopts them into the registry once and plans again, any other refusal (or a crate that cannot be adopted) is the caller's. */
export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  try{return await loadBuildGraphOnceV1(root,invocation,phaseControl,options);}
  catch(error){if(invocation?.control.signal.aborted||!adoptPendingBuildOwnersV1(root,String((error as Error)?.message??"")))throw error;}
  return loadBuildGraphOnceV1(root,invocation,phaseControl,options);
}


async function loadBuildGraphOnceV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  if(options.selection?.kind&&!options.baseline&&!options.reuse&&invocation){
    clearPinnedAuthorityV1();
    if(options.selection.kind==="all")return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));
    return withImplementationPinV1(async()=>{
      const sliced=await loadBuildGraphSliceV1(root,invocation,phaseControl,options);
      if(process.env.SEMIO_BUILD_GRAPH_VERIFY==="1")requireSelectedSubgraphV1(sliced,await loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));
      return sliced;
    });
  }
  return loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options);
}


/** 🍰️ Derives the graph of a selected scope (tasks or literal owners) from the newest sealed generation: the closure of the selected projects is read, verified and, under a fresh policy, refreshed incrementally. The generation is the only source of a graph: when none is usable it is built first, and a selected project the generation does not know refuses. */
async function loadBuildGraphSliceV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  const producer=await import("../📥️inference/🟨️.mjs"),registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),selection=readBuildGraphSelectionV1(options.selection);
  if(selection.kind==="all")throw Error("Selected build graph scope required");
  const roots=selection.kind==="tasks"?[...new Set(selection.tasks.map(task=>task.project))]:selectLiteralBuildProjectsV1(registry.owners,selection.projects,selection.exclude);
  const loaded=await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry,options.graph?.compact===true),{policy:options.graph?.policy??"fresh",selection,slice:{selection,roots,providers:registry.catalogProviders.map(({consumerRoots,sourcePath})=>({consumerRoots,sourcePath}))},externalInputs:options.graph?.externalInputs,pin:boundGenerationPinV1()});options.onStats?.(loaded.stats);return loaded.graph as BuildGraphV1;
}


/** 🔭️ Names the first difference between a generation slice and the selected graph a full selected inference produced, in both directions: every selected project is equal, the dependencies among the selected projects are the same set, and every selected external node is equal; the slice may hold more projects. The oracle of `SEMIO_BUILD_GRAPH_VERIFY=1` for selected scopes. */
function requireSelectedSubgraphV1(slice:BuildGraphV1,selected:BuildGraphV1):void{
  const same=(left:unknown,right:unknown)=>canonicalGenerationJsonV1(left)===canonicalGenerationJsonV1(right),chosen=new Set(Object.keys(selected.projects));
  for(const[name,project]of Object.entries(selected.projects)){if(!slice.projects[name])throw Error(`Graph slice lacks the selected project ${name}`);if(!same(slice.projects[name],project))throw Error(`Graph slice differs from the selected graph at project ${name}`);}
  const inside=(edge:{source:string;target:string})=>chosen.has(edge.source)&&(chosen.has(edge.target)||selected.externalNodes?.[edge.target]!==undefined||slice.externalNodes?.[edge.target]!==undefined);
  const mine=new Set(slice.dependencies.filter(inside).map(edge=>canonicalGenerationJsonV1(edge))),theirs=new Set(selected.dependencies.map(edge=>canonicalGenerationJsonV1(edge)));
  for(const edge of theirs)if(!mine.has(edge))throw Error("Graph slice lacks a selected dependency: "+edge);
  for(const edge of mine)if(!theirs.has(edge))throw Error("Graph slice adds a dependency the selected graph lacks: "+edge);
  for(const[key,node]of Object.entries(selected.externalNodes??{}))if(!same(slice.externalNodes?.[key],node))throw Error(`Graph slice differs from the selected graph at external node ${key}`);
}


type BuildGraphLoadOptionsV1={selection:BuildGraphSelectionV1;reuse?:boolean;baseline?:boolean;onStats?:(stats:GenerationLoadStatsV1)=>void;graph?:{policy?:"fresh"|"current";closure?:readonly string[];lean?:boolean;compact?:boolean;externalInputs?:readonly string[]}};


/** 🕸️ Loads the graph with the strict self-checks of the loaded implementation; the generation path runs pinned and completes with what it loaded. */
async function loadBuildGraphUnpinnedV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  options={...options,selection:readBuildGraphSelectionV1(options.selection)};
  if(options.reuse)return withRepositoryBuildGraphPublicationV1(root,phaseControl,async graph=>{checkScriptInvocation(invocation);if(!graph.admission||JSON.stringify(readBuildGraphSelectionV1(graph.admission.selection))!==JSON.stringify(readBuildGraphSelectionV1(options.selection)))throw Error("Original selected build graph admission required");if(options.graph?.externalInputs!==undefined)requireBuildGraphExternalInputsV1(graph,options.graph.externalInputs);return graph;});
  if(!invocation)throw Error("Original build graph producer invocation required");
  if(options.selection.kind==="all"&&!options.baseline){
    const producer=await import("../📥️inference/🟨️.mjs"),registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));
    const loadedAll=await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry),{policy:options.graph?.policy??"fresh",closure:options.graph?.closure,lean:options.graph?.lean,externalInputs:options.graph?.externalInputs,pin:boundGenerationPinV1()}) as BuildGraphV1;
    options.onStats?.(loadedAll.stats);const graph=loadedAll.graph as BuildGraphV1;
    if(process.env.SEMIO_BUILD_GRAPH_VERIFY==="1"&&!options.graph?.closure&&!options.graph?.lean)requireEqualBuildGraphsV1(graph,await loadBuildGraphV1(root,invocation,phaseControl,{selection:options.selection,baseline:true}));
    checkScriptInvocation(invocation);return graph;
  }
  for(let attempt=0;attempt<=8;attempt++){
    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);
    const producer=await import("../📥️inference/🟨️.mjs");checkResourceLeaseControl(phaseControl);const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),scope=options.selection.kind!=="all"?buildGraphScopeKeyV1(options.selection,registry):undefined;
    if(scope===undefined){const sealed=await readSealedBuildGraphV1(root,invocation,phaseControl,options,registry);if(sealed&&!isOwnerRegistryInvalidationV1(sealed)){checkScriptInvocation(invocation);return sealed;}}
    const graph=await withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl)=>{
      const staging=scope?join(store.root,"selected"):store.root;await store.recheck();checkResourceLeaseControl(actualControl);requireNativeStoragePathV1(store.root,staging);await mkdir(staging,{recursive:true});checkResourceLeaseControl(actualControl);const selectedStage=scope?new RegExp("^"+scope+"(?:\\.authority)?\\.json\\.[a-f0-9-]{36}$","u"):undefined;
      const staged=await readdir(staging);checkResourceLeaseControl(actualControl);for(const name of staged){if(!(selectedStage?.test(name)??/^(?:canonical|authority)\.json\.[a-f0-9-]{36}$/u.test(name)))continue;await store.recheck();checkResourceLeaseControl(actualControl);const path=join(staging,name),status=await lstat(path);checkResourceLeaseControl(actualControl);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>graphSnapshotPolicyV1.maximumSnapshotBytes)throw Error("Owned bounded graph staging file required");await rm(path);checkResourceLeaseControl(actualControl);}
      checkResourceLeaseControl(actualControl);await store.recheck();return loadBuildGraphAttemptV1(root,invocation,actualControl,options,registry,scope);
    },scope);
    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);
    if(graph&&!isOwnerRegistryInvalidationV1(graph))return graph;
    await advanceScriptInvocation(invocation,"build:refresh","running");
  }
  throw Error("Build source authority keeps changing during inference");
}
