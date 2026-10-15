import{withNxSdkDerivativeReaderV1}from"../📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {withRepositoryToolingStoreV1} from "../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {join,resolve,isAbsolute,basename,dirname} from "node:path";
import {fileURLToPath} from "node:url";
import {mkdirSync,readFileSync,writeFileSync,rmSync,lstatSync,realpathSync} from "node:fs";
import {createServer,type Socket} from "node:net";
import {randomUUID} from "node:crypto";
import {withResourceLeases,requireMaterializedPathV1,checkResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {receiveScriptProcessInvocation,scriptInvocationEnvironment,SCRIPT_PROCESS_INVOCATION_ENV,type ScriptInvocation,type ScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import witnessSchema from "./🧬️schema/📥️witness.json" with {type:"json"};

export type NxToolingUseStorageV1={readonly directory:string;readonly witnessDirectory:string};
export type NxToolingUseAuthorityV1={readonly identity:string;readonly toolkitRoot:string;readonly invocation:ScriptInvocation;readonly reader?:(token:string)=>Promise<object|undefined>};
export type NxToolingUseWitnessV1={readonly version:1;readonly identity:string;readonly toolkitRoot:string;readonly processId:number;readonly nonce:string;readonly startedAt:number;readonly original:ScriptProcessEnvelope;readonly endpoint:{readonly host:"127.0.0.1";readonly port:number}};

/** 🗂️ Names the shared physical toolkit lifetime namespace for one workspace. */
export function nxToolingUseStorageV1(root:string):NxToolingUseStorageV1{return {directory:join(root,".cache/b/l/m/data"),witnessDirectory:join(repositoryCachePartitionRootV1(root,"tools"),"use")};}

/** 🎟️ Keeps one exact immutable toolkit in use through its original owned operation. */
export async function withNxToolingUseV1<T>(storage:NxToolingUseStorageV1,authority:NxToolingUseAuthorityV1,run:(witness:NxToolingUseWitnessV1)=>Promise<T>):Promise<T>{
 const {identity,invocation}=authority,toolkitRoot=resolve(authority.toolkitRoot),control=scriptCompilerAdmissionControlV1(invocation,"nx:toolkit-use",0);
 if(!/^[a-f0-9]{64}$/u.test(identity)||!isAbsolute(authority.toolkitRoot)||basename(toolkitRoot)!==identity||realpathSync(toolkitRoot)!==toolkitRoot||JSON.parse(readFileSync(join(toolkitRoot,".semio-nx-tooling.json"),"utf8")).digest!==identity)throw Error("Exact loaded immutable toolkit receipt required");
 const directory=requireMaterializedPathV1(storage.witnessDirectory),nonce=randomUUID(),path=requireMaterializedPathV1(join(directory,nonce+".json"));
 return withResourceLeases({directory:storage.directory,signal:control.signal,control,resources:[{resource:"nx-tooling:"+identity,mode:"shared"}]},async()=>{const root=readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot;return withRepositoryToolingStoreV1(root,invocation,store=>withNxSdkDerivativeReaderV1(root,store,toolkitRoot,identity,async()=>{
  const sockets=new Set<Socket>();let witness:NxToolingUseWitnessV1|undefined,active=false;
  const server=createServer({allowHalfOpen:true},socket=>{
   sockets.add(socket);socket.once("close",()=>sockets.delete(socket));socket.on("error",()=>{});socket.setTimeout(10000,()=>socket.destroy());let input="";
   let answered=false;socket.on("data",async bytes=>{input+=bytes.toString("utf8");if(input.length>128){socket.destroy();return;}if(answered||!input.includes("\n"))return;answered=true;try{checkResourceLeaseControl(control);let response:object|undefined;if(active&&input===nonce+"\n")response=witness;else if(active&&input.startsWith(nonce+" ")&&/^[a-f0-9]{32}\n$/u.test(input.slice(nonce.length+1)))response=await authority.reader?.(input.slice(nonce.length+1,-1));checkResourceLeaseControl(control);socket.end(active&&response?JSON.stringify(response):"");}catch{socket.destroy();}});
  });
  try{
   await new Promise<void>((accept,reject)=>{server.once("error",reject);server.listen({host:"127.0.0.1",port:0},()=>accept());});
   checkResourceLeaseControl(control);const endpoint=server.address();if(!endpoint||typeof endpoint==="string")throw Error("Original finite toolkit endpoint required");
   witness={version:1,identity,toolkitRoot,processId:process.pid,nonce,startedAt:Date.now(),original:JSON.parse(scriptInvocationEnvironment(invocation,{})[SCRIPT_PROCESS_INVOCATION_ENV]!),endpoint:{host:"127.0.0.1",port:endpoint.port}};
   if(validateJsonSchemaSubset(witnessSchema,witness).length)throw Error("Original finite toolkit witness refused");
   mkdirSync(directory,{recursive:true});if(lstatSync(directory).isSymbolicLink())throw Error("Finite toolkit witness directory must be physical");
   writeFileSync(path,JSON.stringify(witness),{flag:"wx",mode:0o600});active=true;
   return await run(witness);
  }finally{
   active=false;try{rmSync(path,{force:true});}finally{for(const socket of sockets)socket.destroy();if(server.listening)await new Promise<void>((accept,reject)=>server.close(error=>error?reject(error):accept()));}
  }
 }));});
}

/** 🧷️ The actual immutable module owns its independent process use until the original task body closes. */
export async function withNxLoadedToolingUseV1<T>(environment:Readonly<Record<string,string|undefined>>,run:()=>Promise<T>):Promise<T>{
 const toolkitRoot=dirname(fileURLToPath(import.meta.url)),identity=basename(toolkitRoot);
 return receiveScriptProcessInvocation(environment,original=>{const root=readNativeCallerAuthorityV1(original.capabilities).repositoryRoot;return withRepositoryToolingStoreV1(root,original,()=>withNxToolingUseV1(nxToolingUseStorageV1(root),{identity,toolkitRoot,invocation:original},async()=>{
  const key="SEMIO_NX_TASK_USE_OWNER_PID",previous=process.env[key];process.env[key]=String(process.pid);
  try{return await run()}finally{if(previous===undefined)delete process.env[key];else process.env[key]=previous;}
 }));});
}
