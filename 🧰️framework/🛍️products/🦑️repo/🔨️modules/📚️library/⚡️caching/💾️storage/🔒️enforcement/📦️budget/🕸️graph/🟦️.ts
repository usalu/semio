import {createHash,randomUUID} from "node:crypto";
import {lstat,mkdir,open,readFile,readdir,rename,rm} from "node:fs/promises";
import {join} from "node:path";
import {readBuildGraphV1,type BuildGraphV1} from "../../../../../🏗️build/🕸️graph/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,acquireResourceLease,acquireQueuedResourceLease,type ResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../📥️opening/🏪️store/🟦️.ts";
import policy from "./🔣️policy.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📌️ Names immutable graph content without exporting platform storage handles. */
export interface RepositoryBuildGraphArtifactV1{readonly path:string;readonly digest:string}
const hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");
const resource=(path:string)=>"build-graph:"+path;

/** 🧾️ Checks the actual original snapshot file before sharing it with a backend. */
async function inspectSnapshotV1(store:RepositoryCacheStoreV1,path:string,digest:string):Promise<{bytes:number;stamp:number}|undefined>{
 await store.recheck();requireNativeStoragePathV1(store.root,path);
 try{const before=await lstat(path,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size>BigInt(policy.maximumSnapshotBytes))throw Error("Original bounded graph snapshot required");const bytes=await readFile(path),after=await lstat(path,{bigint:true});if(["dev","ino","size","mtimeNs","ctimeNs"].some(key=>before[key as keyof typeof before]!==after[key as keyof typeof after])||hash(bytes)!==digest)throw Error("Original graph snapshot bytes changed");return{bytes:bytes.length,stamp:Number(before.mtimeMs)};}
 catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
}

/** 🧹️ Removes only unheld owned content under original exclusive file custody. */
async function retainSnapshotBudgetV1(store:RepositoryCacheStoreV1,directory:string,desired:string,additionalBytes:number,leaseDirectory:string):Promise<void>{
 const rows:{path:string;bytes:number;stamp:number}[]=[];for(const name of await readdir(directory)){checkResourceLeaseControl(store.control);if(!/^[a-f0-9]{64}(?:\.json|~[a-f0-9]{8})$/u.test(name))continue;const path=requireNativeStoragePathV1(store.root,join(directory,name)),status=await lstat(path);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>policy.maximumSnapshotBytes)throw Error("Original bounded graph snapshot inventory required");if(name.includes("~")){await store.recheck();await rm(path);continue;}rows.push({path,bytes:status.size,stamp:status.mtimeMs});}
 let bytes=rows.reduce((sum,row)=>sum+row.bytes,0)+additionalBytes;
 for(const row of rows.sort((left,right)=>left.stamp-right.stamp||left.path.localeCompare(right.path))){if(bytes<=policy.maximumRetainedBytes)break;if(row.path===desired)continue;let lease:ResourceLease|undefined;try{lease=await acquireResourceLease({directory:leaseDirectory,resource:resource(row.path),mode:"exclusive",signal:store.control.signal,control:store.control,timeoutMs:0});}catch(error){if(error instanceof Error&&error.message==="Resource lease timed out: "+resource(row.path))continue;throw error;}try{await store.recheck();await inspectSnapshotV1(store,row.path,row.path.split("/").at(-1)!.slice(0,64));await rm(row.path);bytes-=row.bytes;}finally{lease.release();}}
 if(bytes>policy.maximumRetainedBytes)throw Error("Pinned build graphs exhaust the finite snapshot budget");
}

/** 📌️ Retains immutable graph bytes and their enforcing storage through one backend consumer. */
export async function withRepositoryBuildGraphArtifactV1<T>(workspace:string,graph:BuildGraphV1,invocation:ScriptInvocation,body:(artifact:RepositoryBuildGraphArtifactV1)=>Promise<T>):Promise<T>{
 checkScriptInvocation(invocation);if(validateJsonSchemaSubset(schema,policy).length||policy.maximumSnapshotBytes>policy.maximumRetainedBytes)throw Error("Closed graph artifact budget required");const source=JSON.stringify(readBuildGraphV1(graph)),bytes=Buffer.byteLength(source);if(bytes>policy.maximumSnapshotBytes)throw Error("Build graph exceeds finite snapshot byte limit");const digest=hash(source),leaseDirectory=join(workspace,".cache/b/l/m/data");
 return withRepositoryCacheStoreV1(workspace,"graph",invocation,async store=>{
  const directory=requireNativeStoragePathV1(store.root,join(store.root,"snapshots")),path=requireNativeStoragePathV1(store.root,join(directory,digest+".json")),writer=await acquireQueuedResourceLease({directory:leaseDirectory,resource:"build-graph-store:"+store.root,mode:"exclusive",owner:randomUUID(),signal:store.control.signal,control:store.control});let reader:ResourceLease|undefined;
  try{await store.recheck();await mkdir(directory,{recursive:true});const existing=await inspectSnapshotV1(store,path,digest);await retainSnapshotBudgetV1(store,directory,path,existing?0:bytes,leaseDirectory);if(!existing){const temporary=requireNativeStoragePathV1(store.root,join(directory,digest+"~"+randomUUID().slice(0,8))),file=await open(temporary,"wx");try{await file.writeFile(source);await file.sync();await store.recheck();await rename(temporary,path);const parent=await open(directory,"r");try{await parent.sync();}finally{await parent.close();}}finally{await file.close();await rm(temporary,{force:true});}await inspectSnapshotV1(store,path,digest);}reader=await acquireResourceLease({directory:leaseDirectory,resource:resource(path),mode:"shared",signal:store.control.signal,control:store.control});}finally{writer.release();}
  try{await store.recheck();const result=await body(Object.freeze({path,digest}));await store.recheck();return result;}finally{reader?.release();}
 });
}
