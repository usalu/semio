import{requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createHash} from "node:crypto";
import {chmod,lstat,mkdir,mkdtemp,open,readFile,rename,rm} from "node:fs/promises";
import {dirname,join} from "node:path";
import {withResourceLeases,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import manifest from "./🔣️.json";
import type {RepositoryToolingStoreV1} from "../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";

export type BazelDistribution={readonly platform:string;readonly arch:string;readonly file:string;readonly bytes:number;readonly sha256:string};
export type BazelAcquisitionProgress={readonly phase:"waiting"|"verifying"|"downloading"|"ready";readonly bytes:number;readonly total:number};
export const BAZEL_VERSION=manifest.version;

/** 🏗️ Selects an authored Bazel binary with its embedded runtime. */
export function bazelDistribution(platform:string,arch:string):BazelDistribution{
 const row=manifest.distributions.find(row=>row.platform===platform&&row.arch===arch);
 if(!row)throw Error(`Unsupported pinned Bazel: ${manifest.version} ${platform}/${arch}`);
 return row;
}

/** 🪪️ Hashes executable bytes without buffering an entire distribution. */
async function executableHash(path:string,control:ResourceLeaseControl):Promise<string>{
 checkResourceLeaseControl(control);const file=await open(path,"r"),hash=createHash("sha256");let failed=false;
 try{for await(const bytes of file.createReadStream({autoClose:false})){checkResourceLeaseControl(control);hash.update(bytes);}checkResourceLeaseControl(control);return hash.digest("hex");}catch(error){failed=true;throw error;}
 finally{try{await file.close();}catch(error){if(!failed)throw error;}}
}

/** 🔐️ Acquires the pinned Bazel binary with bounded progress and atomic publication. */
export async function prepareBazel(workspace:string,store:RepositoryToolingStoreV1,phaseControl:ResourceLeaseControl,progress:(event:BazelAcquisitionProgress)=>void|Promise<void>=()=>{},offline=process.env.CARGO_NET_OFFLINE==="true"):Promise<string>{
 requireNativeStorageReservationV1(store.storage,store.control);requireIssuedResourceControlInvocationV1(phaseControl,store.invocation);requireIssuedResourceControlRefinementV1(store.control,phaseControl);checkResourceLeaseControl(phaseControl);await store.recheck();checkResourceLeaseControl(phaseControl);if(store.root!==repositoryCachePartitionRootV1(workspace,"tools"))throw Error("Original declared Bazel tools partition required");const signal=phaseControl.signal;checkResourceLeaseControl(phaseControl);
 const row=bazelDistribution(process.platform,process.arch),base=requireNativeStoragePathV1(store.root,join(store.root,"bazel/bin")),directory=join(base,row.sha256.slice(0,16)),name=process.platform==="win32"?"bazel.exe":"bazel",executable=join(directory,name);
 for(let current=base;;current=dirname(current)){
  try{const stat=await lstat(current);checkResourceLeaseControl(phaseControl);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error(`Invalid Bazel directory: ${current}`);}
  catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  if(dirname(current)===current)break;
 }
 await mkdir(base,{recursive:true});checkResourceLeaseControl(phaseControl);await progress({phase:"waiting",bytes:0,total:row.bytes});
 return withResourceLeases({directory:join(workspace,".cache/b/l/m/data"),signal,control:phaseControl,resources:[{resource:`bazel-${row.sha256}`,mode:"exclusive"}]},async()=>{
  checkResourceLeaseControl(phaseControl);await progress({phase:"verifying",bytes:0,total:row.bytes});
  try{const stat=await lstat(executable),parent=await lstat(directory);if(!stat.isFile()||stat.isSymbolicLink()||parent.isSymbolicLink()||stat.size!==row.bytes||await executableHash(executable,phaseControl)!==row.sha256)throw Error("Bazel executable checksum mismatch");checkResourceLeaseControl(phaseControl);await progress({phase:"ready",bytes:row.bytes,total:row.bytes});checkResourceLeaseControl(phaseControl);return executable;}
  catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  if(offline)throw Error("Pinned Bazel executable is not prepared for the original offline invocation");
  const staging=await mkdtemp(join(base,".prepare-")),target=join(staging,name);
  try{
   const response=await fetch(`${manifest.release}/${row.file}`,{signal});checkResourceLeaseControl(phaseControl);if(!response.ok||!response.body)throw Error(`Bazel download failed: ${response.status}`);
   const file=await open(target,"wx"),reader=response.body.getReader(),hash=createHash("sha256");let bytes=0;
   try{for(let read=await reader.read();!read.done;read=await reader.read()){checkResourceLeaseControl(phaseControl);bytes+=read.value.length;if(bytes>row.bytes)throw Error("Bazel executable exceeds its pinned size");hash.update(read.value);await file.writeFile(read.value);checkResourceLeaseControl(phaseControl);await progress({phase:"downloading",bytes,total:row.bytes});}}
   catch(error){await reader.cancel().catch(()=>undefined);throw error;}
   finally{reader.releaseLock();await file.close();}
   if(bytes!==row.bytes||hash.digest("hex")!==row.sha256)throw Error("Bazel executable checksum mismatch");
   await chmod(target,0o755);checkResourceLeaseControl(phaseControl);await rename(staging,directory);checkResourceLeaseControl(phaseControl);await progress({phase:"ready",bytes,total:row.bytes});checkResourceLeaseControl(phaseControl);return executable;
  }finally{await rm(staging,{recursive:true,force:true});}
 });
}
