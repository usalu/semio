import { repositoryCachePartitionRootDataV1, withRepositoryCompilerStoreV1, withRepositoryCacheStoreV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import {requireRepositoryCompilerStoreV1,type RepositoryCompilerStoreV1} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import {readCompletedCargoRetainedIndexV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📥️reading/🟦️.ts";
import type {RetainedCompilationFileV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📁️copy/🟦️.ts";
import type {CompletedCargoCompilationViewV1} from "@semio-tech/process/artifacts/native-build/receipt/retention/snapshot";
import {CurrentPhysicalOwnerV1} from "../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts";
import {cargoProvenancePhysicalControlV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {acquireResourceLease,checkResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import compilerInventory from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/📥️sources/🧬️schema/🔣️json/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,scriptInvocationEnvironment,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {type CompilerAdmissionControl} from "@semio-tech/process/compiler-capacity";
import {requestedCompilerAdmissionControlV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {readNativeStorageReservationPolicyV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {buildModePolicy} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { readNativeCallerAuthorityV1, withRepositoryArtifactV1, withRepositoryArtifactTaskV1, type RepositoryArtifactReaderV1, withArtifactScratchV1, withArtifactDestinationTaskV1, withRepositoryArtifactDestinationV1, stageRepositoryArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import { repositoryCargoNetworkV1, buildRepositoryCargoArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import {withCapturedCargoArtifactsTaskV1,withCapturedCargoCompilationTaskV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {decodeCargoProvenanceV1,CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {readArtifactBytesV1,fileDigest} from "../../🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import {descriptorEmitterCargoArgumentsV1,descriptorEmitterBinaryNameV1,descriptorEmitterPublicationFilesV1,descriptorEmitterPinnedRequestV1,descriptorEmitterRequestV1,readDescriptorEmitterIntentV1,readDescriptorEmitterPinV1,requireDescriptorEmitterCapabilitiesV1,DESCRIPTOR_EMITTER_MANIFEST_V1,type DescriptorEmitterIntentV1,type DescriptorEmitterPinV1} from "./📋️contract/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {join,basename} from "node:path";
import {lstat,writeFile} from "node:fs/promises";
import {descriptorEmitterReleaseArtifactRequestV1,descriptorEmitterReleaseHandoffPolicyV1,readDescriptorEmitterReleaseReplyV1,requireDescriptorEmitterReleaseRequestV1,type DescriptorEmitterReleaseEvidenceV1,type DescriptorEmitterReleaseRequestV1} from "./🤝️release-handoff/🟦️.ts";
import {assertSealedFilesV1} from "./🔏️sealed/🟦️.ts";
import {descriptorEmitterRecordedPathV1,staleDescriptorEmitterCargoInputsV1,staleDescriptorEmitterInputsV1} from "./🌡️freshness/🟦️.ts";
import {ensureFreshDescriptorEmitterWithV1,descriptorEmitterPrerequisiteResourceV1} from "./🥾️prerequisite/🟦️.ts";

declare const emitterBrand:unique symbol;
export type DescriptorEmitterV1=Readonly<{readonly[emitterBrand]:true}>;
type Scope={live:boolean;reader:RepositoryArtifactReaderV1;workspace:string;invocation:ScriptInvocation;intent:DescriptorEmitterIntentV1;cargoHome:string;pin?:DescriptorEmitterPinV1;release?:DescriptorEmitterReleaseRequestV1};
const emitters=new WeakMap<DescriptorEmitterV1,Scope>();
/** 🔧️ Publishes the actual captured executable and actual completed compiler observation before producer cleanup; the registry fetch and compilation run first, and only the final publication runs inside `publishing`, so a caller that serializes publications never holds its singleton while it waits for the registry fetch lease.
 * @see ./🥾️prerequisite/🟦️.ts */
export function buildDescriptorEmitterV1(workspace:string,invocation:ScriptInvocation,control:CompilerAdmissionControl,publishing:(body:()=>Promise<void>)=>Promise<void>=body=>body(),release?:DescriptorEmitterReleaseRequestV1,sourceRoot:string=workspace):Promise<void>{
 requireIssuedResourceControlInvocationV1(control,invocation);const authority=readNativeCallerAuthorityV1(invocation.capabilities),intent=readDescriptorEmitterIntentV1({version:1,mode:authority.original.buildMode,platform:process.platform}),name=descriptorEmitterBinaryNameV1(intent);if(workspace!==authority.repositoryRoot)throw Error("Actual emitter workspace required");
 return buildRepositoryCargoArtifacts(join(sourceRoot,DESCRIPTOR_EMITTER_MANIFEST_V1),[...descriptorEmitterCargoArgumentsV1(intent.mode)],workspace,{producer:"descriptor-emitter",sourceRoot,invocation,network:repositoryCargoNetworkV1(invocation,workspace),control,retainCompilationBytes:compilerInventory.properties.maximumRetainedCompilationBytes.const,retainConsumedSources:true,consume:capture=>withCapturedCargoArtifactsTaskV1(capture,async native=>withCapturedCargoCompilationTaskV1(capture,async compilation=>{
  const binary=native.files.get(name);if(!binary||[...native.files.keys()].some(file=>file!==name&&!/\.(?:pdb|dwp)$/u.test(file)))throw Error("One self-contained captured descriptor executable required");
  await publishing(async()=>{
   await withRepositoryCacheStoreV1(workspace,"products",invocation,native.control,(store,storePhase)=>withArtifactScratchV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:storePhase,request:{version:1,mode:intent.mode,owner:"browser/tools/descriptor-emitter"}},scratch=>withArtifactDestinationTaskV1(scratch,invocation,async sink=>{
    await native.recheck();await compilation.recheck();
    const files=descriptorEmitterCompilationFilesV1(binary,compilation),request=descriptorEmitterRequestV1(store.root,intent);
    await withRepositoryArtifactDestinationV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:sink.control,directory:request.path},destination=>stageRepositoryArtifacts(destination,request.owner,files));
    await compilation.recheck();
   })));
   if(release)await publishDescriptorEmitterReleaseReplyV1(workspace,invocation,native.control,release,binary,await fileDigest(binary,native.control.signal),compilation);
  });
 }))});
}

function descriptorEmitterCompilationFilesV1(binary:string,compilation:CompletedCargoCompilationViewV1):Map<string,string>{
 const prefix=schema.definitions.Protocol.const.compilation;
 return new Map([[basename(binary),binary],...[...compilation.files].map(([logical,file])=>[prefix+"/"+logical,file.path] as const)]);
}

/** 📬️ Stages the captured executable, exact compiler witnesses and typed request echo as one committed managed artifact. */
function publishDescriptorEmitterReleaseReplyV1(workspace:string,invocation:ScriptInvocation,control:CompilerAdmissionControl,value:DescriptorEmitterReleaseRequestV1,binary:string,binarySha256:string,compilation:CompletedCargoCompilationViewV1):Promise<void>{
 const request=requireDescriptorEmitterReleaseRequestV1(value,invocation);
 const evidence=readDescriptorEmitterReleaseReplyV1({request,evidence:{version:1,mode:"release",platform:request.platform,binarySha256,receiptSha256:compilation.observationSha256,indexSha256:compilation.files.get("index.json")?.sha256}},request);
 requireDescriptorEmitterToolPinV1(evidence,binarySha256,compilation);
 return withRepositoryCacheStoreV1(workspace,"products",invocation,control,(store,phase)=>
  withArtifactScratchV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:phase,request:{version:1,mode:"release",owner:"browser/tools/descriptor-emitter"}},scratch=>
   withArtifactDestinationTaskV1(scratch,invocation,async sink=>{
    const stamp=join(sink.directory,descriptorEmitterReleaseHandoffPolicyV1.file),files=descriptorEmitterCompilationFilesV1(binary,compilation);
    await writeFile(stamp,JSON.stringify({request,evidence}),{flag:"wx",mode:0o444});
    files.set(descriptorEmitterReleaseHandoffPolicyV1.file,stamp);
    const selection=descriptorEmitterReleaseArtifactRequestV1(store.root,request);
    await compilation.recheck();
    await withRepositoryArtifactDestinationV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:sink.control,directory:selection.path},destination=>stageRepositoryArtifacts(destination,selection.owner,files));
    await compilation.recheck();
   })));
}

/** 🫴️ Seals an exact release reply from a live producer already admitted under the same Original. */
export function retainDescriptorEmitterReleaseReplyV1(token:DescriptorEmitterV1,invocation:ScriptInvocation,value:DescriptorEmitterReleaseRequestV1):Promise<void>{
 const scope=emitters.get(token);
 if(!scope?.live||scope.invocation!==invocation||scope.pin||scope.release||scope.intent.mode!=="release")throw Error("Live release descriptor emitter producer required");
 return withDescriptorEmitterTaskV1(token,invocation,tool=>publishDescriptorEmitterReleaseReplyV1(scope.workspace,invocation,tool.control,value,tool.path,tool.sha256,tool.compilation));
}
/** 🕳️ Tells whether any file of the selected emitter publication is absent, so a cold or partial products partition is rebuilt rather than read. */
export async function descriptorEmitterAbsentV1(directory:string,intent:DescriptorEmitterIntentV1,control:CompilerAdmissionControl):Promise<boolean>{
 for(const name of descriptorEmitterPublicationFilesV1(intent)){
  checkResourceLeaseControl(control);
  try{await lstat(join(directory,name));}
  catch(error){
   if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;
   return true;
  }
 }
 return false;
}

/** 🥾️ Makes the published emitter current before a component build: a fresh one is kept, a missing, stale or unusable one is rebuilt and republished by the same `buildDescriptorEmitterV1` the `:build` target runs.
 * Inspection takes its reader inside its own scope and releases it before the publisher takes the writer; the build, including its registry fetch, runs without any lease of ours, and one exclusive tool singleton serializes only the final publication, because a tool singleton must never wait for the registry fetch lease of another build.
 * @see ./🥾️prerequisite/🟦️.ts */
export function ensureFreshDescriptorEmitterV1(workspace:string,invocation:ScriptInvocation,control:CompilerAdmissionControl,report:(line:string)=>void=console.log,release?:DescriptorEmitterReleaseRequestV1,sourceRoot:string=workspace):Promise<"fresh"|"built">{
 requireIssuedResourceControlInvocationV1(control,invocation);
 const authority=readNativeCallerAuthorityV1(invocation.capabilities);
 if(workspace!==authority.repositoryRoot)throw Error("Actual emitter workspace required");
 const intent=readDescriptorEmitterIntentV1({version:1,mode:authority.original.buildMode,platform:process.platform}),request=descriptorEmitterRequestV1(repositoryCachePartitionRootDataV1(workspace,"products"),intent);
 return withRepositoryCacheStoreV1(workspace,"dependencies",invocation,control,(store,actual)=>{
  const directory=readNativeStorageReservationPolicyV1(store.storage,store.control).leaseDirectory,phase=()=>requestedCompilerAdmissionControlV1(actual,0);
  return ensureFreshDescriptorEmitterWithV1({
   signal:actual.signal,report,
   inspect:async()=>{
    if(await descriptorEmitterAbsentV1(request.path,intent,actual))return {kind:"missing"};
    try{
     const stale=await withDescriptorEmitterV1(workspace,invocation,phase(),emitter=>withDescriptorEmitterTaskV1(emitter,invocation,async tool=>{
      const sources=await staleDescriptorEmitterInputsV1(tool.observation,sourceRoot,tool.control.signal,workspace);
      const cargo=await staleDescriptorEmitterCargoInputsV1(decodeCargoProvenanceV1(tool.observation),sourceRoot,tool.compilation,tool.control.signal,workspace);
      const stale=[...sources.stale,...cargo.stale].sort();
      if(!stale.length&&release)await publishDescriptorEmitterReleaseReplyV1(workspace,invocation,tool.control,release,tool.path,tool.sha256,tool.compilation);
      return stale;
     }));
     return stale.length?{kind:"stale",changed:stale.length,first:stale[0]!}:{kind:"fresh"};
    }catch(error){
     actual.signal.throwIfAborted();
     return {kind:"unreadable",reason:error instanceof Error?error.message:String(error)};
    }
   },
   publish:publishing=>buildDescriptorEmitterV1(workspace,invocation,phase(),publishing,release,sourceRoot),
   serialize:async body=>{
    const lease=await acquireResourceLease({directory,resource:descriptorEmitterPrerequisiteResourceV1(intent),mode:"exclusive",signal:actual.signal,control:actual});
    try{return await body();}finally{lease.release();}
   }
  });
 });
}

/** 📦️ Retains one explicitly named prototype tool generation before workspace compilation. */
export function ensurePinnedDescriptorEmitterV1(workspace:string,invocation:ScriptInvocation,control:CompilerAdmissionControl,value:DescriptorEmitterPinV1):Promise<void>{
 requireIssuedResourceControlInvocationV1(control,invocation);
 const authority=readNativeCallerAuthorityV1(invocation.capabilities),pin=readDescriptorEmitterPinV1(value);
 if(workspace!==authority.repositoryRoot||authority.original.buildMode!==pin.mode||process.platform!==pin.platform)throw Error("Published emitter must match the actual prototype invocation");
 const request=descriptorEmitterPinnedRequestV1(repositoryCachePartitionRootDataV1(workspace,"products"),pin);
 return withRepositoryCacheStoreV1(workspace,"dependencies",invocation,control,(_store,phase)=>
  withRepositoryCompilerStoreV1(workspace,pin.mode,phase,scriptInvocationEnvironment(invocation,process.env),async(compiler,actual)=>{
   let present=true;
   try{await lstat(request.path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;present=false;}
   if(present)return;
   await withCompilerStoreDescriptorEmitterV1(workspace,invocation,compiler,actual,emitter=>withDescriptorEmitterTaskV1(emitter,invocation,async tool=>{
    requireDescriptorEmitterToolPinV1(pin,tool.sha256,tool.compilation);
    const prefix=schema.definitions.Protocol.const.compilation;
    const files=new Map([[basename(tool.path),tool.path],...[...tool.compilation.files].map(([name,file])=>[prefix+"/"+name,file.path] as const)]);
    await withRepositoryCacheStoreV1(workspace,"products",invocation,tool.control,(store,phase)=>
     withRepositoryArtifactDestinationV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:phase,directory:request.path},destination=>
      stageRepositoryArtifacts(destination,request.owner,files)));
   }));
  }));
}

/** 🔗️ Ties the managed executable bytes to the exact completed Cargo receipt and index. */
export function requireDescriptorEmitterToolPinV1(pin:DescriptorEmitterPinV1|DescriptorEmitterReleaseEvidenceV1,binarySha256:string,compilation:CompletedCargoCompilationViewV1):void{
 if(binarySha256!==pin.binarySha256||
    compilation.files.get("blobs/"+binarySha256)?.sha256!==binarySha256||
    compilation.observationSha256!==pin.receiptSha256||
    compilation.files.get("index.json")?.sha256!==pin.indexSha256)throw Error("Managed descriptor emitter differs from the explicit binary, receipt or index digest");
 const {version,mode,platform}=pin,name=descriptorEmitterBinaryNameV1({version,mode,platform}),record=decodeCargoProvenanceV1(compilation.observation);
 const matches=record.units.filter(({message})=>{
  const target=message.target,profile=message.profile;
  return message.manifest_path===record.manifest&&target!==null&&typeof target==="object"&&profile!==null&&typeof profile==="object"&&Reflect.get(profile,"test")===false&&
   Reflect.get(target,"name")===schema.definitions.Protocol.const.binary&&Array.isArray(Reflect.get(target,"kind"))&&Reflect.get(target,"kind").includes("bin");
 }).flatMap(unit=>unit.artifacts.filter(output=>basename(output.path)===name&&output.sha256===binarySha256));
 if(matches.length!==1)throw Error("Pinned descriptor emitter requires its exact original production binary output");
}

/** 🪪️ Issues a private tool token from the published executable and its retained compiler evidence. */
export function withCompilerStoreDescriptorEmitterV1<T>(workspace:string,invocation:ScriptInvocation,compiler:RepositoryCompilerStoreV1,control:CompilerAdmissionControl,body:(emitter:DescriptorEmitterV1,control:CompilerAdmissionControl)=>Promise<T>,selection?:DescriptorEmitterPinV1,releaseRequest?:DescriptorEmitterReleaseRequestV1):Promise<T>{
 const mode=requireRepositoryCompilerStoreV1(compiler,workspace,invocation,control),intent=readDescriptorEmitterIntentV1({version:1,mode,platform:process.platform});
 const cargoHome=compiler.environment.CARGO_HOME;
 if(typeof body!=="function"||!cargoHome)throw Error("Actual descriptor emitter callback and compiler home required");
 const pin=selection===undefined?undefined:readDescriptorEmitterPinV1(selection);
 if(pin&&(pin.mode!==intent.mode||pin.platform!==intent.platform))throw Error("Published emitter must match the actual prototype invocation");
 const release=releaseRequest===undefined?undefined:requireDescriptorEmitterReleaseRequestV1(releaseRequest,invocation);
 if(release&&(pin||intent.mode!=="release"))throw Error("Original release descriptor emitter required");
 return withRepositoryCacheStoreV1(workspace,"products",invocation,control,(store,storePhase)=>
  withRepositoryArtifactV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:storePhase,request:release?descriptorEmitterReleaseArtifactRequestV1(store.root,release):pin?descriptorEmitterPinnedRequestV1(store.root,pin):descriptorEmitterRequestV1(store.root,intent)},reader=>
   withRepositoryArtifactTaskV1(reader,invocation,async artifact=>{
    const token=Object.freeze({})as DescriptorEmitterV1,scope:Scope={live:true,reader,workspace,invocation,intent,cargoHome,pin,release};
    emitters.set(token,scope);
    try{return await body(token,artifact.control);}finally{scope.live=false;}
   })));
}
/** 🥾️ Standalone describes admit their compiler store before reading any Cargo or tool artifacts. */
export function withDescriptorEmitterV1<T>(workspace:string,invocation:ScriptInvocation,control:CompilerAdmissionControl,body:(emitter:DescriptorEmitterV1,control:CompilerAdmissionControl)=>Promise<T>,release?:DescriptorEmitterReleaseRequestV1):Promise<T>{
 requireIssuedResourceControlInvocationV1(control,invocation);
 const authority=readNativeCallerAuthorityV1(invocation.capabilities);
 if(workspace!==authority.repositoryRoot||typeof body!=="function")throw Error("Actual descriptor emitter owner required");
 return withRepositoryCacheStoreV1(workspace,"dependencies",invocation,control,(_dependency,phase)=>
  withRepositoryCompilerStoreV1(workspace,authority.original.buildMode,phase,scriptInvocationEnvironment(invocation,process.env),(compiler,actual)=>
   withCompilerStoreDescriptorEmitterV1(workspace,invocation,compiler,actual,body,undefined,release)));
}

/** 🕳️ Distinguishes a producer that never published from an existing generation requiring full validation. */
export async function descriptorEmitterReleaseReplyExistsV1(workspace:string,invocation:ScriptInvocation,control:CompilerAdmissionControl,value:DescriptorEmitterReleaseRequestV1):Promise<boolean>{
 requireIssuedResourceControlInvocationV1(control,invocation);
 if(workspace!==readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot)throw Error("Actual descriptor emitter workspace required");
 const request=requireDescriptorEmitterReleaseRequestV1(value,invocation),directory=descriptorEmitterReleaseArtifactRequestV1(repositoryCachePartitionRootDataV1(workspace,"products"),request).path;
 checkResourceLeaseControl(control);
 try{await lstat(directory);}
 catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return false;throw error;}
 checkResourceLeaseControl(control);return true;
}
/** ▶️ Retains actual reader custody through executable use and verifies its real compiler witness. */
export function withDescriptorEmitterTaskV1<T>(token:DescriptorEmitterV1,invocation:ScriptInvocation,body:(tool:Readonly<{path:string;sha256:string;observation:string;compilation:CompletedCargoCompilationViewV1;pin?:DescriptorEmitterPinV1;control:CompilerAdmissionControl;recheck():Promise<void>}>)=>Promise<T>):Promise<T>{
 const scope=emitters.get(token);if(!scope?.live||scope.invocation!==invocation||typeof body!=="function")throw Error("Live actual descriptor emitter task required");return withRepositoryArtifactTaskV1(scope.reader,invocation,async artifact=>{
 const name=descriptorEmitterBinaryNameV1(scope.intent),prefix=schema.definitions.Protocol.const.compilation,path=join(artifact.path,name),sha256=await fileDigest(path,artifact.control.signal),limits=compilerInventory.properties,receiptBytes=await readArtifactBytesV1(join(artifact.path,prefix,"receipt.json"),Math.min(CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1,readNativeCallerAuthorityV1(invocation.capabilities).transport.maximumBytes),artifact.control.signal),indexBytes=await readArtifactBytesV1(join(artifact.path,prefix,"index.json"),limits.maximumRetainedIndexBytes.const,artifact.control.signal),index=readCompletedCargoRetainedIndexV1(indexBytes,receiptBytes,scope.workspace,scope.intent.mode),observation=new TextDecoder("utf-8",{fatal:true}).decode(receiptBytes),record=decodeCargoProvenanceV1(observation),matches=record.units.flatMap(unit=>unit.artifacts.filter(output=>basename(output.path)===name&&output.sha256===sha256));if(index.origin.cargoHome!==scope.cargoHome||record.status!==0||record.cancelled||descriptorEmitterRecordedPathV1(scope.workspace,record.manifest)!==join(scope.workspace,DESCRIPTOR_EMITTER_MANIFEST_V1)||matches.length!==1)throw Error("Actual descriptor executable compiler observation required");const expected=new Set([name,prefix+"/receipt.json",prefix+"/index.json",...index.files.map(row=>prefix+"/"+row.path)]);if(scope.release)expected.add(descriptorEmitterReleaseHandoffPolicyV1.file);if(artifact.files.length!==expected.size||[...artifact.files].some(file=>!expected.has(file)))throw Error("Exact full descriptor compiler inventory required");const physical=new CurrentPhysicalOwnerV1(scope.workspace,cargoProvenancePhysicalControlV1({cancelled:()=>artifact.control.signal.aborted,remainingMs:()=>checkResourceLeaseControl(artifact.control),onProgress:()=>checkResourceLeaseControl(artifact.control)})),files=new Map<string,RetainedCompilationFileV1>();for(const logical of["receipt.json","index.json",...new Set(index.files.map(row=>row.path))]){const actual=join(artifact.path,prefix,logical),measured=await physical.measure(actual);if(!measured)throw Error("Actual retained descriptor compiler file required");const claimed=index.files.find(row=>row.path===logical);if(claimed&&(claimed.sha256!==measured.sha256||claimed.bytes!==measured.byteLength))throw Error("Retained descriptor compiler bytes differ");const mode=(await lstat(actual)).mode&0o777;checkResourceLeaseControl(artifact.control);files.set(logical,Object.freeze({path:actual,...measured,mode}));}const recheck=async()=>{checkResourceLeaseControl(artifact.control);await assertSealedFilesV1([{path,sha256},...[...files.values()].map(file=>({path:file.path,sha256:file.sha256}))],candidate=>physical.digest(candidate));await physical.recheck();await artifact.recheck();};await recheck();const compilation:CompletedCargoCompilationViewV1=Object.freeze({origin:index.origin,observation,observationSha256:index.receipt.sha256,files,control:artifact.control,recheck});if(scope.pin)requireDescriptorEmitterToolPinV1(scope.pin,sha256,compilation);
 if(scope.release){
  const policy=descriptorEmitterReleaseHandoffPolicyV1,bytes=await readArtifactBytesV1(join(artifact.path,policy.file),policy.maximumBytes,artifact.control.signal);
  const evidence=readDescriptorEmitterReleaseReplyV1(JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes)),scope.release);
  requireDescriptorEmitterToolPinV1(evidence,sha256,compilation);
 }
 const value=await body(Object.freeze({path,sha256,observation,compilation,pin:scope.pin,control:artifact.control,recheck}));await recheck();return value;});
}

/** 🩺️ Confirms the installed emitter answers `capabilities` before a component lane is admitted; currency of its sources is the prerequisite stage's concern. */
export function checkDescriptorEmitterCapabilitiesV1(workspace:string,invocation:ScriptInvocation,compiler:RepositoryCompilerStoreV1,control:CompilerAdmissionControl,pin?:DescriptorEmitterPinV1):Promise<void>{
 const output=readNativeCallerAuthorityV1(invocation.capabilities).transport;
 return withCompilerStoreDescriptorEmitterV1(workspace,invocation,compiler,control,emitter=>
  withDescriptorEmitterTaskV1(emitter,invocation,async tool=>{
   const stdout=await captureOwnedCommandV1(tool.path,["capabilities"],workspace,{control:tool.control,env:compiler.environment,output});
   requireDescriptorEmitterCapabilitiesV1(stdout);
  }),pin);
}
