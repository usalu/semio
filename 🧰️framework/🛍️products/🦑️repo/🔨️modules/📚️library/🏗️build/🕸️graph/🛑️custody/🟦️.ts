import type {BuildGraphInvalidationInventoryV1} from "../📋️model/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withRepositoryBuildGraphFactStoreV1,withRepositoryBuildGraphSelectionV1} from "../../📥️inputs/💾️storage/🟨️.mjs";
import {buildGraphPublicationWriterResourceV1} from "../../📥️inputs/💾️storage/🔒️writer/🟨️.mjs";
import {readdir,lstat,rm} from "node:fs/promises";
import {join,dirname,resolve} from "node:path";
import {randomBytes,createHash} from "node:crypto";
import {checkScriptInvocation,advanceScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireQueuedResourceLease,acquireResourceLease,acquireFreshResourceLeaseV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {type RepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {createGenerationStoreV1,createNodeFilePortV1} from "../🕰️generations/🟨️.mjs";
import {createQueuedClaimLeasePortV1} from "../🕰️generations/🧩️claims/🟨️.mjs";
import {createSliceCustodyV1,renderCustodyMessageV1,type SliceClaimV1,type SliceCustodyV1} from "../🕰️generations/🔐️custody/🟨️.mjs";
import {createGenerationFilesV1} from "../🗂️generation-files/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {buildGraphPathV1,planBuildGraphInvalidationV1} from "../📜️contract/🟦️.ts";


/** 🛑️ Serializes graph publication and invalidation inside the original enforcing store. */
export async function withBuildGraphStateV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(store:RepositoryCacheStoreV1,actualControl:ResourceLeaseControl)=>Promise<T>,scope?:string):Promise<T> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  const run=async(store:RepositoryCacheStoreV1,actualControl:ResourceLeaseControl)=>{
    const writer=await acquireQueuedResourceLease({directory:join(root,".cache/b/l/m/data"),resource:scope?buildGraphPublicationWriterResourceV1(store.root,scope):"build-graph-state:"+store.root,mode:"exclusive",owner:randomBytes(16).toString("base64url"),control:actualControl,signal:actualControl.signal});
    try{checkResourceLeaseControl(actualControl);return await body(store,actualControl);}finally{writer.release();}
  };
  return scope?withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,scope,run):withRepositoryBuildGraphFactStoreV1(root,invocation,phaseControl,run);
}


/** 🛡️ The slice custody of the generation builders of one graph image (see `./🕰️generations/🔐️custody`): claim documents beside the generations, leases in the repository lease directory. */
export function generationCustodyV1(root:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl):SliceCustodyV1{
  return createSliceCustodyV1({root:join(store.root,"generations"),fs:generationFilesV1(store) as never,lease:createQueuedClaimLeasePortV1({acquire:acquireQueuedResourceLease as never,tryAcquire:acquireResourceLease as never,acquireFresh:acquireFreshResourceLeaseV1 as never,directory:join(root,".cache/b/l/m/data"),control})});
}


/** 🧑‍🤝‍🧑️ Builds inside the shared cohort of the generation namespace (sweeps stay out) under a slice claim on the owners `request` writes (every owner when it is undefined or structural): refreshes of disjoint projects build at the same time, overlapping ones one after the other in ticket order. */
export async function withGenerationCustodyV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,request:{readonly dirty:readonly string[];readonly structural:boolean}|undefined,body:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl,claim:SliceClaimV1)=>Promise<T>):Promise<T> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  return withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{
    const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,claim=await generationCustodyV1(root,store,control).hold({owners:request?.dirty??[],structural:request===undefined||request.structural,signal:control.signal,say:(key,fields)=>{process.stderr.write(renderCustodyMessageV1(key,fields,language)+"\n");}});
    try{checkResourceLeaseControl(control);return await body(store,control,claim);}finally{await claim.release();}
  },{optionalCaches:false,shared:true});
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
    await createGenerationStoreV1({root:join(store.root,"generations"),fs:generationFilesV1(store),holds:async()=>true,resources:{facts:"repository-graph-facts:"+resolve(root)}}).invalidate({reason:"reset",signal:invocation.control.signal});
  });
}


/** 🔌️ The generation namespace of the graph image: its writer lease is taken as a shared cohort by every builder, prefetch and idle probe (sweeps take it exclusively), the slice custody decides who builds. */
export const generationScopeV1=createHash("sha256").update("semio-graph-generations-v1").digest("hex");


/** 📈️ Opens the generation documents of the graph partition: every path stays inside the original store and the store is rechecked while documents move. */
export function generationFilesV1(store:RepositoryCacheStoreV1){
  return createGenerationFilesV1({store,base:createNodeFilePortV1(),requirePath:requireNativeStoragePathV1});
}
