import {setPriority} from "node:os";
import {spawn} from "node:child_process";
import {fileURLToPath} from "node:url";
import {CACHE_GC_LIMITS_V1} from "../../🧬️schema/🟦️.ts";

/** ⏲️ What the maintenance loop needs from the world, replaceable by a fake clock: a one-shot timer, the pass itself (it receives the cancellation of the loop) and what to do with its failure. */
export type IdleCompactionLoopPortsV1=Readonly<{setTimer(callback:()=>void,milliseconds:number):Readonly<{cancel():void}>;run(signal:AbortSignal):Promise<void>;failed?(error:unknown):void}>;

/** ⏲️ Runs the compaction pass in the background of a long-lived supervised process: the first pass after `firstDelayMilliseconds` (so a starting worker is not loaded by it), then one pass `intervalMilliseconds` after the previous one ended; passes never overlap, a failing pass only reports and the loop goes on, and `stop` (or the signal) cancels the timer and the running pass. @see ../🟦️.ts */
export function startIdleCompactionLoopV1(input:Readonly<{ports:IdleCompactionLoopPortsV1;firstDelayMilliseconds:number;intervalMilliseconds:number;signal?:AbortSignal}>):Readonly<{stop():void;settled():Promise<void>}>{
 const control=new AbortController(),{ports}=input;let timer:Readonly<{cancel():void}>|undefined,running:Promise<void>=Promise.resolve();
 const schedule=(milliseconds:number):void=>{
  if(control.signal.aborted)return;
  timer=ports.setTimer(()=>{
   running=(async()=>{try{await ports.run(control.signal);}catch(error){if(!control.signal.aborted)ports.failed?.(error);}finally{schedule(input.intervalMilliseconds);}})();
  },milliseconds);
 };
 const stop=():void=>{control.abort();timer?.cancel();};
 input.signal?.addEventListener("abort",stop,{once:true});
 if(input.signal?.aborted)stop();else schedule(input.firstDelayMilliseconds);
 return Object.freeze({stop,settled:()=>running});
}

/** 🛌️ The real pass of one machine: `repo cache gc --idle` of this workspace as a child at the lowest scheduling priority and in the background marker environment, bounded by the signal (a killed child leaves its image whole: the reclaim cycle cleans up on SIGTERM). Never on the critical path of a command: the loop owns it, no command waits for it. */
export async function runIdleCompactionChildV1(workspace:string,environment:Readonly<Record<string,string|undefined>>,signal:AbortSignal):Promise<void>{
 const{graphCollectorEnvironmentV1}=await import("../../🚀️launch/🟦️.ts"),script=fileURLToPath(new URL("../../../📜️script.ts",import.meta.url));
 await new Promise<void>((accept,reject)=>{
  const child=spawn(process.execPath,[script,"repo","cache","gc","--idle"],{cwd:workspace,env:{...graphCollectorEnvironmentV1(environment)},stdio:"ignore",windowsHide:true,signal});
  if(child.pid!==undefined)try{setPriority(child.pid,19);}catch{}
  child.on("error",reject);child.on("exit",code=>code===0?accept():reject(Error("repo cache gc --idle exited with "+code)));
 });
}

/** 🛰️ The sidecar roles that run the maintenance loop: the agent (on every worker) and the cache server (on the coordinator). */
export const IDLE_COMPACTION_ROLES_V1:readonly string[]=Object.freeze(["agent","cache"]);

/** 🛰️ Starts the loop of compaction on idle inside a sidecar of the given role, at the limits of the cache collector; the other roles start nothing. The ports are replaceable by tests (fake clock, fake pass). */
export function startSidecarIdleCompactionV1(input:Readonly<{role:string;root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;write?(line:string):void;ports?:Partial<IdleCompactionLoopPortsV1>}>):ReturnType<typeof startIdleCompactionLoopV1>|undefined{
 if(!IDLE_COMPACTION_ROLES_V1.includes(input.role))return undefined;
 const ports:IdleCompactionLoopPortsV1={
  setTimer:(callback,milliseconds)=>{const timer=setTimeout(callback,milliseconds);timer.unref();return{cancel:()=>clearTimeout(timer)};},
  run:signal=>runIdleCompactionChildV1(input.root,input.environment,signal),
  failed:error=>input.write?.("idle compaction pass failed: "+String((error as Error)?.message??error)),
  ...input.ports};
 return startIdleCompactionLoopV1({ports,firstDelayMilliseconds:CACHE_GC_LIMITS_V1.idleCompactionFirstDelayMilliseconds,intervalMilliseconds:CACHE_GC_LIMITS_V1.idleCompactionIntervalMilliseconds,signal:input.signal});
}
