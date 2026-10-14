import {createHash} from "node:crypto";
import {chmod,lstat,mkdir,mkdtemp,open,readFile,rename,rm} from "node:fs/promises";
import {dirname,join} from "node:path";
import {withResourceLeases} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import manifest from "./🔣️.json";

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
async function executableHash(path:string,signal:AbortSignal):Promise<string>{
 const file=await open(path,"r"),hash=createHash("sha256");
 try{for await(const bytes of file.createReadStream({autoClose:false})){signal.throwIfAborted();hash.update(bytes);}return hash.digest("hex");}
 finally{await file.close();}
}

/** 🔐️ Acquires the pinned Bazel binary with bounded progress and atomic publication. */
export async function prepareBazel(workspace:string,signal:AbortSignal,progress:(event:BazelAcquisitionProgress)=>void=()=>{}):Promise<string>{
 signal.throwIfAborted();
 const row=bazelDistribution(process.platform,process.arch),base=join(workspace,".🧬semio/🦑️repo/⚡️cache/tools/bazel"),directory=join(base,row.sha256.slice(0,16)),name=process.platform==="win32"?"bazel.exe":"bazel",executable=join(directory,name);
 for(let current=base;;current=dirname(current)){
  try{const stat=await lstat(current);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error(`Invalid Bazel directory: ${current}`);}
  catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  if(dirname(current)===current)break;
 }
 await mkdir(base,{recursive:true});progress({phase:"waiting",bytes:0,total:row.bytes});
 return withResourceLeases({directory:join(workspace,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),signal,resources:[{resource:`bazel-${row.sha256}`,mode:"exclusive"}]},async()=>{
  signal.throwIfAborted();progress({phase:"verifying",bytes:0,total:row.bytes});
  try{const stat=await lstat(executable),parent=await lstat(directory);if(!stat.isFile()||stat.isSymbolicLink()||parent.isSymbolicLink()||stat.size!==row.bytes||await executableHash(executable,signal)!==row.sha256)throw Error("Bazel executable checksum mismatch");progress({phase:"ready",bytes:row.bytes,total:row.bytes});return executable;}
  catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  const staging=await mkdtemp(join(base,".prepare-")),target=join(staging,name);
  try{
   const response=await fetch(`${manifest.release}/${row.file}`,{signal});if(!response.ok||!response.body)throw Error(`Bazel download failed: ${response.status}`);
   const file=await open(target,"wx"),reader=response.body.getReader(),hash=createHash("sha256");let bytes=0;
   try{for(let read=await reader.read();!read.done;read=await reader.read()){signal.throwIfAborted();bytes+=read.value.length;if(bytes>row.bytes)throw Error("Bazel executable exceeds its pinned size");hash.update(read.value);await file.writeFile(read.value);progress({phase:"downloading",bytes,total:row.bytes});}}
   catch(error){await reader.cancel().catch(()=>undefined);throw error;}
   finally{reader.releaseLock();await file.close();}
   if(bytes!==row.bytes||hash.digest("hex")!==row.sha256)throw Error("Bazel executable checksum mismatch");
   await chmod(target,0o755);signal.throwIfAborted();await rename(staging,directory);progress({phase:"ready",bytes,total:row.bytes});return executable;
  }finally{await rm(staging,{recursive:true,force:true});}
 });
}
