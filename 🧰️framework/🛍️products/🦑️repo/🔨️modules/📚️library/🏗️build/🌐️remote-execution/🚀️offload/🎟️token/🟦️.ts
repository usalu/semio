import {closeSync,constants,openSync,readSync,writeSync} from "node:fs";
import {jobserverEndpointV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🎰️jobserver/🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../📐️policy/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
/** 🎟️ The job token a wrapper gave back to its build while it waits for a remote result. */
export type OffloadTokenLeaseV1=Readonly<{reacquire(maximumMilliseconds?:number):Promise<boolean>;close():void}>;

/** 🎟️ Gives the implicit job token of this compiler call back to the pool of the build (one byte into the named pool, the protocol of GNU make and the `jobserver` crate), so a machine whose units wait on other machines does not stay saturated by waiting wrappers. Only a named pool (`fifo:PATH`, which the repository launcher creates) can be served without blocking; pipe, semaphore and absent pools keep the token (`undefined`). `reacquire` takes one byte back before the call returns its result; it resolves to `false` when none came in time, and then the call goes on without it (a unit's cheap restore, never a second compile). */
export function releaseImplicitTokenV1(environment:Environment,platform:NodeJS.Platform=process.platform):OffloadTokenLeaseV1|undefined{
 if(platform==="win32")return undefined;
 const endpoint=jobserverEndpointV1(environment,platform);
 if(endpoint.kind!=="fifo")return undefined;
 let descriptor:number;
 try{descriptor=openSync(endpoint.path,constants.O_RDWR|constants.O_NONBLOCK);writeSync(descriptor,"+");}catch{return undefined;}
 let open=true,held=false;
 return Object.freeze({
  async reacquire(maximumMilliseconds=OFFLOAD_POLICY_V1.limits.tokenWaitMilliseconds){
   if(!open||held)return held;
   const deadline=Date.now()+maximumMilliseconds,byte=Buffer.alloc(1);
   for(let pause=2;;pause=Math.min(pause*2,50)){
    try{if(readSync(descriptor,byte,0,1,null)===1){held=true;return true;}}catch(error){if((error as NodeJS.ErrnoException).code!=="EAGAIN")return false;}
    if(Date.now()>=deadline)return false;
    await new Promise(done=>setTimeout(done,pause));
   }
  },
  close(){if(!open)return;open=false;try{closeSync(descriptor);}catch{}},
 });
}
