import{createIssuedResourceControlV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {randomBytes,createHash} from "node:crypto";
import {join} from "node:path";
import {mkdir,open,rename,rm,lstat,readdir,readFile} from "node:fs/promises";
import type {ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireResourceLease} from "../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {withRepositoryCacheStoreV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "./🔣️policy.json" with {type:"json"};

/** 📊️ Admits a compact declared JSON report name before any filesystem operation. */
export function repositoryBuildReportNameV1(value:string):string{if(validateJsonSchemaSubset(schema,value).length)throw Error("Declared build report name required");return value;}
/** 📌️ Retains complete immutable report bytes and original fixed backing through consumption. */
export interface RepositoryBuildReportV1{readonly path:string;readonly digest:string;recheck():Promise<void>}

/** 📝️ Publishes finite JSON under exclusive custody, then retains its original reader callback. */
export async function withRepositoryBuildReportV1<T>(workspace:string,name:string,value:unknown,invocation:ScriptInvocation,body:(artifact:RepositoryBuildReportV1)=>Promise<T>):Promise<T>{
 name=repositoryBuildReportNameV1(name);const encoded=JSON.stringify(value,null,2);if(encoded===undefined)throw Error("JSON build report value required");const bytes=encoded+"\n";if(validateJsonSchemaSubset(schema.definitions.Policy,policy).length||Buffer.byteLength(bytes)>policy.maximumReportBytes)throw Error("Build report exceeds16MiB limit");
 return withRepositoryCacheStoreV1(workspace,"diagnostics",invocation,createIssuedResourceControlV1(invocation,"cache:store-entry",0),async store=>{
  const directory=requireNativeStoragePathV1(store.root,join(store.root,"reports")),path=requireNativeStoragePathV1(store.root,join(directory,name)),leases=join(workspace,".cache/b/l/m/data"),resource="repository-build-reports",writer=await acquireResourceLease({directory:leases,resource,mode:"exclusive",signal:store.control.signal,control:store.control}),temporary=requireNativeStoragePathV1(store.root,path+"~"+randomBytes(4).toString("hex"));
  try{
   await store.recheck();await mkdir(directory,{recursive:true});const rows:{path:string;bytes:number;stamp:number}[]=[];
   for(const name of await readdir(directory)){const file=requireNativeStoragePathV1(store.root,join(directory,name)),status=await lstat(file);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>policy.maximumReportBytes)throw Error("Original retained build report required");if(/~[a-f0-9]{8}$/u.test(name)){repositoryBuildReportNameV1(name.slice(0,-9));await rm(file);continue;}repositoryBuildReportNameV1(name);if(file!==path)rows.push({path:file,bytes:status.blocks*512,stamp:status.mtimeMs});}
   let retained=rows.reduce((sum,row)=>sum+row.bytes,0)+Math.ceil(Buffer.byteLength(bytes)/1048576)*1048576,reports=rows.length+1;
   for(const row of rows.sort((a,b)=>a.stamp-b.stamp||a.path.localeCompare(b.path))){if(retained<=policy.maximumRetainedBytes&&reports<=policy.maximumReports)break;await store.recheck();await rm(row.path);retained-=row.bytes;reports--;}
   const file=await open(temporary,"wx");try{await file.writeFile(bytes);await file.sync();await store.recheck();await rename(temporary,path);}finally{await file.close();}const parent=await open(directory,"r");try{await parent.sync();}finally{await parent.close();}

  let live=true;
  const digest=createHash("sha256").update(bytes).digest("hex"),identity=await lstat(path,{bigint:true}),recheck=async()=>{if(!live)throw Error("Original report callback released");await store.recheck();const current=await lstat(requireNativeStoragePathV1(store.root,path),{bigint:true});if(!current.isFile()||current.isSymbolicLink()||current.nlink!==1n||["dev","ino","size","mtimeNs","ctimeNs"].some(key=>identity[key as keyof typeof identity]!==current[key as keyof typeof current])||createHash("sha256").update(await readFile(path)).digest("hex")!==digest)throw Error("Original report bytes changed");};
  try{await recheck();const result=await body(Object.freeze({path,digest,recheck}));await recheck();return result;}finally{live=false;}
  }finally{try{await rm(temporary,{force:true});}finally{writer.release();}}
 });
}
