import {AsyncLocalStorage} from "node:async_hooks";
import {afterAll} from "bun:test";
import {receiveScriptProcessInvocation,requireIssuedScriptInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
let ready:Promise<void>|undefined,lifetime:Promise<void>|undefined,finish:(()=>void)|undefined,run:(<T>(body:(original:ScriptInvocation)=>Promise<T>)=>Promise<T>)|undefined;
/** 🧷️ Shares one actual process birth through all imported test owners without renewing its original deadline. */
export async function withBuildTestInvocationV1<T>(body:(original:ScriptInvocation)=>Promise<T>):Promise<T>{
 if(!ready){let accept!:()=>void,refuse!:(error:unknown)=>void;ready=new Promise<void>((resolve,reject)=>{accept=resolve;refuse=reject;});lifetime=receiveScriptProcessInvocation(process.env,async original=>{const context=AsyncLocalStorage.snapshot(),closed=new Promise<void>(resolve=>{finish=resolve;});run=callback=>context(async()=>{requireIssuedScriptInvocationV1(original);const value=await callback(original);requireIssuedScriptInvocationV1(original);return value;});accept();await closed;run=undefined;});void lifetime.catch(refuse);}
 await ready;if(!run)throw Error("Actual build test Original lifetime ended");return run(body);
}
const cleanups:(()=>Promise<void>)[]=[];
/** 🧹️ Drains owned fixture images before the one received Original lifetime settles. */
export function registerBuildTestCleanupV1(cleanup:()=>Promise<void>):void{cleanups.push(cleanup);}
afterAll(async()=>{const failures:unknown[]=[];for(const cleanup of cleanups.reverse())try{await cleanup();}catch(error){failures.push(error);}finish?.();try{await lifetime;}catch(error){failures.push(error);}if(failures.length)throw new AggregateError(failures,"Actual build fixture cleanup failed");},100000);
