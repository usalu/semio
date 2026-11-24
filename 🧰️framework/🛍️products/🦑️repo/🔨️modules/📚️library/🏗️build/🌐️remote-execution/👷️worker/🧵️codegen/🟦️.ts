import {mkdirSync} from "node:fs";
import {join} from "node:path";
import {createJobserverV1,defaultJobserverStyleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts";
import {readRemoteExecutionPolicyV1} from "../../🟦️.ts";

/** 🧵️ The knobs of the grant, from the schema's `worker.codegen`. */
export type RemoteExecutionCodegenPolicyV1=Readonly<{maximumThreads:number;reserveCores:number;extraMegabytesPerThread:number}>;

/** 🧵️ What the grant of one unit reads: the machine's cores and load, the cores its other running actions hold (one each, the grant of a linked unit for itself), the spare memory of the machine before this unit and the peak this unit was admitted with. */
export type RemoteExecutionCodegenAskV1=Readonly<{cores:number;load:number;others:number;spareMegabytes:number;peakMegabytes:number}>;

/** 🧵️ How many code generation threads one linked unit gets on this worker: the cores that neither its other actions nor the machine's own load use, less the reserve, at most the maximum, and at most one thread plus as many as the spare memory above the unit's own peak holds, never less than one. Pure.
 * @see ./🧪️tests/🔣️.json */
export function remoteExecutionCodegenThreadsV1(ask:RemoteExecutionCodegenAskV1,policy:RemoteExecutionCodegenPolicyV1=readRemoteExecutionPolicyV1().worker.codegen):number{
 const busy=Math.max(Math.max(0,Math.floor(ask.others)),Math.ceil(Math.max(0,ask.load))),byCores=Math.floor(ask.cores)-policy.reserveCores-busy;
 const byMemory=1+Math.floor(Math.max(0,ask.spareMegabytes-Math.max(0,ask.peakMegabytes))/policy.extraMegabytesPerThread);
 return Math.max(1,Math.min(policy.maximumThreads,byCores,byMemory));
}

/** 🧵️ Whether the requester itself asked for a serial back end (`-Z no-parallel-backend`): the worker then grants one thread whatever its cores. */
export function remoteExecutionCodegenSerialRequestedV1(args:readonly string[]):boolean{
 return args.some((argument,index)=>argument==="no-parallel-backend"&&args[index-1]==="-Z"||argument==="-Zno-parallel-backend");
}

/** 🧵️ The rustc arguments after the grant: the serial back end (`-Z no-parallel-backend`) when one thread is granted and the requester did not already ask for it, otherwise unchanged. The job-token pool is {@link withRemoteExecutionCodegenPoolV1}. */
export function remoteExecutionCodegenArgumentsV1(args:readonly string[],threads:number):readonly string[]{
 return threads<=1&&!remoteExecutionCodegenSerialRequestedV1(args)?[...args,"-Z","no-parallel-backend"]:args;
}

/** 🧵️ Runs `run` with the environment of a job-token pool of `threads` slots (rustc keeps its own slot, the pool holds the others) made in `directory`; a pool that cannot be made (no fifo on this host) lets `run` get nothing and the caller falls back to one thread. */
export async function withRemoteExecutionCodegenPoolV1<T>(threads:number,directory:string,run:(environment:Readonly<Record<string,string>>|undefined)=>Promise<T>):Promise<T>{
 if(threads<=1)return run(undefined);
 let pool:Awaited<ReturnType<typeof createJobserverV1>>;
 try{mkdirSync(directory,{recursive:true});pool=await createJobserverV1({version:1,tokens:threads,style:defaultJobserverStyleV1()},join(directory,"jobs"));}
 catch{return run(undefined);}
 try{return await pool.withClient(environment=>run(environment));}finally{pool.close();}
}
