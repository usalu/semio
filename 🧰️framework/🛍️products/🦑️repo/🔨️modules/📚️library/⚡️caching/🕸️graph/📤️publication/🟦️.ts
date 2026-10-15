import {withGraphPublicationLeaseV1,publishGraphTransactionV1,readGraphPublicationSnapshotV1,recoverGraphPublicationV1,requireGraphPublicationLeaseV1,graphPublicationFileAllocationV1} from "./📦️transaction/🟦️.ts";
import {existsSync,lstatSync,mkdirSync,realpathSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {receiveScriptProcessInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {withRepositoryCacheStoreV1} from "../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {withNxGraphCacheStoreV1,acquireNxGraphNamespaceV1,nxGraphNamespaceDirectoryV1,withNxGraphPublicationCapacityV1,withNxGraphPoolAdmissionV1,requireNxGraphCacheStoreV1,requireNxGraphNativeDirectoryV1,type NxGraphCacheStoreV1} from "../../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";

export interface GraphPublicationFileV1 {readonly name:"project-graph.json"|"source-maps.json"|"file-map.json";readonly value:unknown}
export interface GraphPublicationLeaseV1 {readonly directory:string;readonly toolingIdentity:string;readonly control:ResourceLeaseControl}
export interface GraphPublicationV1 {readonly directory:string;readonly toolingIdentity:string;readonly files:readonly GraphPublicationFileV1[];readonly environment:Readonly<Record<string,string|undefined>>;readonly generationHolds:()=>boolean;readonly published?:(lease:GraphPublicationLeaseV1)=>Promise<void>}
function generationImage(file:string,image:string):string{
 const value=JSON.parse(image);if(file.endsWith("project-graph.json")&&value&&typeof value==="object"&&!Array.isArray(value)&&Object.hasOwn(value,"computedAt")){if(!Number.isSafeInteger(value.computedAt)||value.computedAt<1)throw Error("Original graph publication time required");const {computedAt,...graph}=value;return JSON.stringify(graph);}return JSON.stringify(value);
}
function generationUnchanged(rows:readonly {name:string;file:string;image:string}[],snapshot:Readonly<Record<string,unknown>>|null,check:()=>void):boolean{if(!snapshot)return false;for(const row of rows){check();if(generationImage(row.file,JSON.stringify(snapshot[row.name]))!==generationImage(row.file,row.image))return false;}check();return true;}
function requireGraphPublicationV1(value:GraphPublicationV1):void{if(!/^[a-f0-9]{64}$/u.test(value.toolingIdentity)||!value.files.length||new Set(value.files.map(file=>file.name)).size!==value.files.length||value.files.some(file=>!["project-graph.json","source-maps.json","file-map.json"].includes(file.name))||!value.files.some(file=>file.name==="project-graph.json")||!value.files.some(file=>file.name==="source-maps.json"))throw Error("Closed original graph generation required");}
/** 🧯️ Publishes one original graph generation under SQL writer ownership; faults restore immutable prior inodes and retain their cause. */
export async function publishGraphFilesV1(value:GraphPublicationV1):Promise<void>{
 requireGraphPublicationV1(value);
 const directory=resolve(value.directory);
 for(let path=directory;;path=dirname(path)){try{const stat=lstatSync(path);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error("Graph publication no-follow directory required");}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}if(dirname(path)===path)break;}
 await receiveScriptProcessInvocation(value.environment,async original=>{
  const control=scriptCompilerAdmissionControlV1(original,"nx:graph-publication",0),check=()=>{checkResourceLeaseControl(control);if(!value.generationHolds())throw Error("Original graph publication generation changed");};
  const workspace=readNativeCallerAuthorityV1(original.capabilities).repositoryRoot;
  await withRepositoryCacheStoreV1(workspace,"graph",original,parent=>withNxGraphCacheStoreV1(parent,async graph=>{const identity={version:1 as const,toolingIdentity:value.toolingIdentity,workspaceRoot:value.environment.NX_WORKSPACE_ROOT_PATH??workspace,graphIdentity:value.environment.SEMIO_BUILD_GRAPH_DIGEST??null,implementationIdentity:value.environment.SEMIO_NX_IMPLEMENTATION_IDENTITY!},namespace=await acquireNxGraphNamespaceV1(graph,identity);try{const base=nxGraphNamespaceDirectoryV1(graph.root,identity),watcher=value.environment.SEMIO_NX_WATCHER_IDENTITY,expected=watcher?join(base,"watchers",watcher):base;if(watcher&&!/^[a-f0-9]{24}$/u.test(watcher)||value.environment.SEMIO_NX_GRAPH_STORAGE!==graph.root||directory!==expected||value.environment.NX_WORKSPACE_DATA_DIRECTORY!==directory)throw Error("Original funded graph publication namespace required");mkdirSync(directory,{recursive:true});if(realpathSync(directory)!==directory)throw Error("Original graph directory changed");
  await withGraphPublicationLeaseV1(directory,value.toolingIdentity,control,async lease=>{
   await publishGraphGenerationV1(graph,lease,value);
  });
  }finally{await namespace.release();}}));
 });
}

/** 📦️ Shares the same admitted generation step between genuine private fixtures and the public publisher. */
export async function publishGraphGenerationV1(graph:NxGraphCacheStoreV1,lease:GraphPublicationLeaseV1,value:GraphPublicationV1):Promise<void>{
 requireGraphPublicationV1(value);requireNxGraphCacheStoreV1(graph);requireGraphPublicationLeaseV1(lease);if(lease.directory!==resolve(value.directory)||lease.toolingIdentity!==value.toolingIdentity)throw Error("Exact graph publication owner required");requireNxGraphNativeDirectoryV1(value.environment.NX_WORKSPACE_ROOT_PATH!,value.environment);if(value.environment.SEMIO_NX_GRAPH_STORAGE!==graph.root||value.environment.NX_WORKSPACE_DATA_DIRECTORY!==lease.directory)throw Error("Original graph publication namespace required");
 const directory=lease.directory,rows=value.files.map(file=>({name:file.name,file:requireMaterializedPathV1(join(directory,file.name)),image:JSON.stringify(file.value)})),check=()=>{checkResourceLeaseControl(lease.control);if(!value.generationHolds())throw Error("Original graph publication generation changed");};await withNxGraphPoolAdmissionV1(graph.invocation,graph.storage,graph.control,graph.recheck,async()=>recoverGraphPublicationV1(lease));
 if(!value.published&&generationUnchanged(rows,readGraphPublicationSnapshotV1(directory,value.files.map(file=>file.name),lease),check))return;
 const rounded=(bytes:number)=>Math.ceil(bytes/4096)*4096,nextBytes=rows.reduce((sum,row)=>sum+rounded(Buffer.byteLength(row.image)),0)+32768,priorBytes=rows.reduce((sum,row)=>{if(!existsSync(row.file))return sum;const stat=lstatSync(row.file);if(!stat.isFile()||stat.isSymbolicLink()||stat.nlink!==1)throw Error("Graph publication regular single-owner file required");return sum+Math.max(stat.blocks*512,rounded(stat.size));},0);
 await withNxGraphPublicationCapacityV1(graph,directory,nextBytes,0,()=>publishGraphTransactionV1(lease,value.files,value.generationHolds,async()=>value.published?.(lease)),priorBytes);
}

/** ⏳️ Forwards one authenticated publication caller until its owned journal capture settles. */
export async function withGraphPublisherControlV1<T>(environment:Readonly<Record<string,string|undefined>>,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 return receiveScriptProcessInvocation(environment,original=>body(scriptCompilerAdmissionControlV1(original,"nx:graph-publisher",0)));
}

export {withGraphPublicationLeaseV1,publishGraphTransactionV1,readGraphPublicationSnapshotV1,recoverGraphPublicationV1,graphPublicationFileAllocationV1};
