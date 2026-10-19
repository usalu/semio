import type {ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import {requireIssuedResourceControlV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
const policy=Object.freeze({...schema.definitions.Policy.const});
/** ⏱️ Holds one finite, non-overlapping sampler until every accepted sample has drained. */
export function startTelemetrySamplerV1(control:ResourceLeaseControl,sample:(control:ResourceLeaseControl)=>Promise<void>,intervalMilliseconds:number=policy.sampleMs):Readonly<{stop():Promise<void>}>{
 requireIssuedResourceControlV1(control);
 if(typeof sample!=="function"||!Number.isSafeInteger(intervalMilliseconds)||intervalMilliseconds<1||intervalMilliseconds>policy.maximumIntervalMilliseconds)throw Error("Finite telemetry sample request required");
 let close:()=>void=()=>{},pending:Promise<void>|undefined,stopped=false,failed=false,failure:unknown,count=0,timer:ReturnType<typeof setInterval>|undefined;
 const stop=()=>{stopped=true;if(timer!==undefined)clearInterval(timer);close();};
 const operation=withIssuedResourceOperationV1(control,async()=>{
  const closed=new Promise<void>(resolve=>{close=resolve;});
  const run=()=>{if(stopped||pending)return;if(count>=policy.maximumSamples){failed=true;failure=Error("Finite telemetry sample ceiling exceeded");stop();return;}count++;
   try{pending=withRequestedIssuedResourceControlV1(control,policy.maximumSampleMilliseconds,sample).then(()=>{pending=undefined;},error=>{failed=true;failure=error;pending=undefined;stop();});}catch(error){failed=true;failure=error;stop();}
  };
  control.signal.addEventListener("abort",stop,{once:true});
  try{if(stopped||control.signal.aborted)stop();else{timer=setInterval(run,intervalMilliseconds);run();}await closed;await pending;if(failed)throw failure;}
  finally{stop();control.signal.removeEventListener("abort",stop);}
 });
 void operation.catch(()=>{});
 return Object.freeze({stop:async()=>{stop();await operation;}});
}
