import {AsyncLocalStorage} from "node:async_hooks";
import {withRepositoryCacheStoreV1} from "../📦️budget/📥️opening/🏪️store/🟦️.ts";
import {withRetainedRepositoryCompilerStoreV1} from "../🏗️compiler/🟦️.ts";
import {createCustodyPoolV1,type CustodyPoolV1} from "./🏊️pool/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,scriptInvocationEnvironment,withRequestedIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {BuildMode} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import type {RepositoryCachePartitionIdV1} from "../📦️budget/🟦️.ts";

/** 🧵️ Names the stores one long-lived custody session keeps open: cache partitions and, optionally, the compiler workspace of one build configuration. */
export type RepositoryCustodyRequestV1=Readonly<{workspace:string;invocation:ScriptInvocation;control:ResourceLeaseControl;partitions:readonly RepositoryCachePartitionIdV1[];compiler?:Readonly<{mode:BuildMode;environment:Readonly<Record<string,string|undefined>>}>}>;
/** 🦑️ Runs many requests inside one retained custody: every nested store open inside `run` reuses the held stores instead of opening and closing the images again. */
export type RepositoryCustodySessionV1=Readonly<{
 run<T>(request:(control:ResourceLeaseControl)=>Promise<T>,options?:Readonly<{maximumMilliseconds?:number;signal?:AbortSignal}>):Promise<T>;
 close():Promise<void>;
 closed:Promise<void>;
}>;

/** 🔓️ Opens the requested stores once, in their canonical nesting order, and keeps them (with the same storage, lease and cancellation guarantees as the scoped opens) until `close`; each request runs in a cancellable refinement of the innermost store phase. */
export function openRepositoryCustodySessionV1(request:RepositoryCustodyRequestV1):Promise<RepositoryCustodySessionV1>{
 requireIssuedResourceControlInvocationV1(request.control,request.invocation);
 return new Promise<RepositoryCustodySessionV1>((resolve,reject)=>{
  let release!:()=>void,settled!:()=>void,live=true,lifetime:Promise<void>|undefined;
  const held=new Promise<void>(done=>{release=done;}),closed=new Promise<void>(done=>{settled=done;});
  const publish=async(control:ResourceLeaseControl):Promise<void>=>{
   const snapshot=AsyncLocalStorage.snapshot();
   resolve(Object.freeze({
    run:<T>(work:(control:ResourceLeaseControl)=>Promise<T>,options:Readonly<{maximumMilliseconds?:number;signal?:AbortSignal}>={})=>live?snapshot(()=>withRequestedIssuedResourceControlV1(control,options.maximumMilliseconds??0,work,options.signal)):Promise.reject(Error("Live custody session required")),
    close:async()=>{release();await lifetime?.catch(()=>undefined);},
    closed,
   }));
   await held;
  };
  const enter=(index:number,control:ResourceLeaseControl):Promise<void>=>{
   if(index<request.partitions.length)return withRepositoryCacheStoreV1(request.workspace,request.partitions[index]!,request.invocation,control,(_store,actual)=>enter(index+1,actual));
   if(request.compiler)return withRetainedRepositoryCompilerStoreV1(request.workspace,request.compiler.mode,control,scriptInvocationEnvironment(request.invocation,{...request.compiler.environment,SEMIO_BUILD_MODE:request.compiler.mode}),actual=>publish(actual));
   return publish(control);
  };
  lifetime=enter(0,request.control).catch(error=>{reject(error);throw error;}).finally(()=>{live=false;settled();});
  lifetime.catch(()=>undefined);
 });
}

/** 🏊️ Keeps one session open across requests and releases it when idle or old, so a warm request skips the store open while exclusive maintenance and dependency syncs are never starved. */
export function createRepositoryCustodyPoolV1(request:RepositoryCustodyRequestV1,lifetimes:Readonly<{idleMilliseconds:number;maximumLifetimeMilliseconds:number}>):CustodyPoolV1<RepositoryCustodySessionV1>{
 return createCustodyPoolV1<RepositoryCustodySessionV1>({
  ...lifetimes,
  timer:{now:()=>performance.now(),set:(milliseconds,run)=>{const handle=setTimeout(run,milliseconds);handle.unref?.();return handle;},clear:handle=>clearTimeout(handle as ReturnType<typeof setTimeout>)},
  open:async()=>{const session=await openRepositoryCustodySessionV1(request);return{session,close:()=>session.close()};},
 });
}
