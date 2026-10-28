import{requireIssuedResourceControlInvocationV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {repositoryCacheBudgetV1} from "../🟦️.ts";
import {createHash,randomUUID,randomBytes} from "node:crypto";
import {lstat,mkdir,open,readFile,readdir,rename,rm,writeFile} from "node:fs/promises";
import {join} from "node:path";
import { readBuildGraphV1 } from "../../../../../🏗️build/🕸️graph/🟦️.ts"; import type { BuildGraphV1 } from "../../../../../🏗️build/🕸️graph/📋️model/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,acquireResourceLease,acquireQueuedResourceLease,type ResourceLease,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../📥️opening/🏪️store/🟦️.ts";
import {encodeGraphRecordsV1,graphRecordDigestsV1} from "./📇️records/🟨️.mjs";
import policy from "./🔣️policy.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📌️ Names immutable graph content without exporting platform storage handles. */
export interface RepositoryBuildGraphArtifactV1{readonly path:string;readonly digest:string}
const hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");
const resource=(path:string)=>"build-graph:"+path;

/** 🧾️ Checks the actual original snapshot file before sharing it with a backend. */
async function inspectSnapshotV1(store:RepositoryCacheStoreV1,path:string,digest:string,control:ResourceLeaseControl):Promise<{bytes:number;stamp:number}|undefined>{
 checkResourceLeaseControl(control);await store.recheck();checkResourceLeaseControl(control);requireNativeStoragePathV1(store.root,path);
 try{const before=await lstat(path,{bigint:true});checkResourceLeaseControl(control);if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size>BigInt(policy.maximumSnapshotBytes))throw Error("Original bounded graph snapshot required");const bytes=await readFile(path);checkResourceLeaseControl(control);const after=await lstat(path,{bigint:true});checkResourceLeaseControl(control);if(["dev","ino","size","mtimeNs","ctimeNs"].some(key=>before[key as keyof typeof before]!==after[key as keyof typeof after])||hash(bytes)!==digest)throw Error("Original graph snapshot bytes changed");return{bytes:bytes.length,stamp:Number(before.mtimeMs)};}
 catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
}

/** 🧹️ Removes only unheld owned content under original exclusive file custody. */
async function retainSnapshotBudgetV1(store:RepositoryCacheStoreV1,directory:string,desired:string,additionalBytes:number,leaseDirectory:string,control:ResourceLeaseControl,keep:ReadonlySet<string>,target:number=policy.maximumRetainedBytes):Promise<number>{
 checkResourceLeaseControl(control);const names=await readdir(directory);checkResourceLeaseControl(control);const rows:{path:string;bytes:number;stamp:number;records:ReadonlySet<string>}[]=[],records=new Map<string,{path:string;bytes:number}>();
 for(const name of names){checkResourceLeaseControl(control);if(!/^[a-f0-9]{64}(?:\.json|\.gz|~[a-f0-9]{8})$/u.test(name))continue;const path=requireNativeStoragePathV1(store.root,join(directory,name)),status=await lstat(path);checkResourceLeaseControl(control);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>policy.maximumSnapshotBytes)throw Error("Original bounded graph snapshot inventory required");if(name.includes("~")){await store.recheck();checkResourceLeaseControl(control);await rm(path);checkResourceLeaseControl(control);continue;}if(name.endsWith(".gz")){records.set(name.slice(0,64),{path,bytes:status.size});continue;}rows.push({path,bytes:status.size,stamp:status.mtimeMs,records:await manifestRecordsV1(path,status.size)});}
 const references=new Map<string,number>(),reference=(digest:string,step:number)=>references.set(digest,(references.get(digest)??0)+step);for(const row of rows)for(const digest of row.records)reference(digest,1);for(const digest of keep)reference(digest,1);
 const live=()=>{let total=rows.reduce((sum,row)=>sum+row.bytes,0);for(const[digest,row]of records)if((references.get(digest)??0)>0)total+=row.bytes;return total;};
 const sweep=async()=>{for(const[digest,row]of records){if((references.get(digest)??0)>0)continue;checkResourceLeaseControl(control);await store.recheck();checkResourceLeaseControl(control);await rm(row.path,{force:true});records.delete(digest);}};
 await sweep();let bytes=live()+additionalBytes;
 for(const row of [...rows].sort((left,right)=>left.stamp-right.stamp||left.path.localeCompare(right.path))){checkResourceLeaseControl(control);if(bytes<=target)break;if(row.path===desired)continue;let lease:ResourceLease|undefined;try{lease=await acquireResourceLease({directory:leaseDirectory,resource:resource(row.path),mode:"exclusive",signal:control.signal,control,timeoutMs:0});}catch(error){checkResourceLeaseControl(control);if(error instanceof Error&&error.message==="Resource lease timed out: "+resource(row.path))continue;throw error;}let primary:{error:unknown}|undefined;try{checkResourceLeaseControl(control);await store.recheck();checkResourceLeaseControl(control);await inspectSnapshotV1(store,row.path,row.path.split("/").at(-1)!.slice(0,64),control);checkResourceLeaseControl(control);await rm(row.path);checkResourceLeaseControl(control);rows.splice(rows.indexOf(row),1);for(const digest of row.records)reference(digest,-1);await sweep();bytes=live()+additionalBytes;}catch(error){primary={error};throw error;}finally{try{lease.release();}catch(error){if(!primary)throw error;if(primary.error instanceof Error&&Object.isExtensible(primary.error))try{Object.defineProperty(primary.error,"graphArtifactCleanupErrors",{value:[error],configurable:true});}catch{}}}}
 if(bytes>policy.maximumRetainedBytes)throw Error("Pinned build graphs exhaust the finite snapshot budget");
 return bytes;
}

/** 🧮️ The retained bytes the last full inventory found, plus every pin since: the retention sweep reads every snapshot file (50,000 `lstat` calls and 700 manifests on the live tree, 2.5-15 s), so a pin that stays within the budget and follows a recent inventory only adds its bytes, and a sweep evicts down to the low-water mark so that the next ones need none. */
async function readRetainedLedgerV1(directory:string):Promise<{bytes:number;at:number}|undefined>{
 try{const value=JSON.parse(await readFile(join(directory,"retained.ledger"),"utf8")) as {bytes?:unknown;at?:unknown};return Number.isSafeInteger(value.bytes)&&Number.isSafeInteger(value.at)&&(value.bytes as number)>=0?{bytes:value.bytes as number,at:value.at as number}:undefined;}catch{return undefined;}
}

/** 📒️ Replaces the retained-bytes ledger atomically under the graph store writer. */
async function writeRetainedLedgerV1(store:RepositoryCacheStoreV1,directory:string,bytes:number,at:number):Promise<void>{
 const target=requireNativeStoragePathV1(store.root,join(directory,"retained.ledger")),temporary=requireNativeStoragePathV1(store.root,join(directory,"retained.ledger~"+randomUUID().slice(0,8)));
 await writeFile(temporary,JSON.stringify({bytes,at}));await rename(temporary,target);
}

/** 🔗️ Names the record digests of one retained snapshot: a record manifest names its records, a whole-graph snapshot of an earlier format names none. */
async function manifestRecordsV1(path:string,size:number):Promise<ReadonlySet<string>>{
 if(size>16777216)return new Set();const bytes=await readFile(path);if(!bytes.subarray(0,13).equals(Buffer.from("{\"version\":2,")))return new Set();
 try{return graphRecordDigestsV1(bytes);}catch{return new Set();}
}

/** 🕸️ Retains immutable graph bytes and their enforcing storage through one backend consumer. */
export async function withRepositoryBuildGraphArtifactV1<T>(workspace:string,graph:BuildGraphV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(artifact:RepositoryBuildGraphArtifactV1,actualControl:ResourceLeaseControl)=>Promise<T>,prepare?: (additionalBytes:number)=>Promise<void>):Promise<T>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkScriptInvocation(invocation);if(validateJsonSchemaSubset(schema,policy).length||policy.maximumSnapshotBytes>policy.maximumRetainedBytes)throw Error("Closed graph artifact budget required");const encoded=await encodeGraphRecordsV1(readBuildGraphV1(graph),{policy:policy.records as never,maximumBytes:policy.maximumSnapshotBytes}),digest=encoded.digest,leaseDirectory=join(workspace,".cache/b/l/m/data");
 const maximumAdmissionAttempts=policy.maximumAdmissionAttempts;let fundedBytes:number|undefined;for(let attempt=0;attempt<maximumAdmissionAttempts;attempt++){
 const result=await withRepositoryCacheStoreV1(workspace,"graph",invocation,phaseControl,async(store,actualControl)=>{
  const check=()=>checkResourceLeaseControl(actualControl);check();const directory=requireNativeStoragePathV1(store.root,join(store.root,"snapshots")),path=requireNativeStoragePathV1(store.root,join(directory,digest+".json")),writer=await acquireQueuedResourceLease({directory:leaseDirectory,resource:"build-graph-store:"+store.root,mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:actualControl.signal,control:actualControl});let reader:ResourceLease|undefined,primary:{error:unknown}|undefined;
  const cleanup=async(failed:{error:unknown}|undefined,operations:readonly(()=>unknown|Promise<unknown>)[])=>{const errors:unknown[]=[];for(const operation of operations)try{await operation();}catch(error){errors.push(error);}if(errors.length){if(!failed)throw errors.length===1?errors[0]:new AggregateError(errors,"Graph artifact cleanup failed");if(failed.error instanceof Error&&Object.isExtensible(failed.error))try{Object.defineProperty(failed.error,"graphArtifactCleanupErrors",{value:errors,configurable:true});}catch{}}};
  try{let writerPrimary:{error:unknown}|undefined;try{check();await store.recheck();check();await mkdir(directory,{recursive:true});check();const existing=await inspectSnapshotV1(store,path,digest,actualControl);check();const present=new Set(await readdir(directory)),missing=[...encoded.records].filter(([name])=>!present.has(name+".gz")),additionalBytes=(existing?0:encoded.manifestBytes.length)+missing.reduce((sum,[,bytes])=>sum+bytes.length,0);if(prepare&&fundedBytes!==additionalBytes)return{ready:false as const,additionalBytes};if(additionalBytes>0){const known=await readRetainedLedgerV1(directory),now=Date.now(),incremental=known!==undefined&&now-known.at<policy.retention.recheckMilliseconds&&known.bytes+additionalBytes<=policy.maximumRetainedBytes,total=incremental?known.bytes+additionalBytes:await retainSnapshotBudgetV1(store,directory,path,additionalBytes,leaseDirectory,actualControl,new Set(encoded.records.keys()),Math.floor(policy.maximumRetainedBytes*policy.retention.lowWaterBasisPoints/10000));await writeRetainedLedgerV1(store,directory,total,incremental?known.at:now);}check();const publish=async(target:string,bytes:Uint8Array)=>{const temporary=requireNativeStoragePathV1(store.root,join(directory,target.split("/").at(-1)!.slice(0,64)+"~"+randomUUID().slice(0,8))),file=await open(temporary,"wx");let filePrimary:{error:unknown}|undefined;try{check();await file.writeFile(bytes);check();await file.sync();check();await store.recheck();check();await rename(temporary,target);check();}catch(error){filePrimary={error};throw error;}finally{await cleanup(filePrimary,[()=>file.close(),()=>rm(temporary,{force:true})]);}};for(const[name,bytes]of missing)await publish(requireNativeStoragePathV1(store.root,join(directory,name+".gz")),bytes);if(!existing)await publish(path,encoded.manifestBytes);if(missing.length||!existing){const parent=await open(directory,"r");let parentPrimary:{error:unknown}|undefined;try{check();await parent.sync();check();}catch(error){parentPrimary={error};throw error;}finally{await cleanup(parentPrimary,[()=>parent.close()]);}check();await inspectSnapshotV1(store,path,digest,actualControl);check();}reader=await acquireResourceLease({directory:leaseDirectory,resource:resource(path),mode:"shared",signal:actualControl.signal,control:actualControl});missing.length=0;encoded.records.clear();}catch(error){writerPrimary={error};throw error;}finally{await cleanup(writerPrimary,[()=>writer.release()]);}
  check();await store.recheck();check();const value=await body(Object.freeze({path,digest}),actualControl);check();await store.recheck();check();return{ready:true as const,value};}catch(error){primary={error};throw error;}finally{await cleanup(primary,[()=>reader?.release()]);}
 });
 if(result.ready)return result.value;checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);await prepare!(result.additionalBytes);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);fundedBytes=result.additionalBytes;
 }throw Error("Graph snapshot changed during bounded storage admission");
}
