import type {ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import {ResourceLoanDeadlineExhaustedV1,requireIssuedResourceControlV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
const policy=Object.freeze({...schema.definitions.Policy.const});
/** ⏱️ Holds one finite, non-overlapping sampler until every accepted sample has drained; a sample that outlives its own loan on a loaded host is skipped because telemetry is advisory and never fails the observed work. Reaching the sample ceiling halves the rate (keeping half the budget) until the interval bound, then ends sampling quietly, so a long build keeps finite telemetry instead of failing. */
export function startTelemetrySamplerV1(control:ResourceLeaseControl,sample:(control:ResourceLeaseControl)=>Promise<void>,intervalMilliseconds:number=policy.sampleMs):Readonly<{stop():Promise<void>}>{
 requireIssuedResourceControlV1(control);
 if(typeof sample!=="function"||!Number.isSafeInteger(intervalMilliseconds)||intervalMilliseconds<1||intervalMilliseconds>policy.maximumIntervalMilliseconds)throw Error("Finite telemetry sample request required");
 let close:()=>void=()=>{},pending:Promise<void>|undefined,stopped=false,failed=false,failure:unknown,count=0,interval=intervalMilliseconds,timer:ReturnType<typeof setInterval>|undefined;
 const stop=()=>{stopped=true;if(timer!==undefined)clearInterval(timer);close();};
 const operation=withIssuedResourceOperationV1(control,async()=>{
  const closed=new Promise<void>(resolve=>{close=resolve;});
  const run=()=>{if(stopped||pending)return;if(count>=policy.maximumSamples){if(interval*2>policy.maximumIntervalMilliseconds){stop();return;}interval*=2;count=policy.maximumSamples/2;clearInterval(timer);timer=setInterval(run,interval);}count++;
   try{pending=withRequestedIssuedResourceControlV1(control,policy.maximumSampleMilliseconds,sample).then(()=>{pending=undefined;},error=>{pending=undefined;if(error instanceof ResourceLoanDeadlineExhaustedV1&&!control.signal.aborted)return;failed=true;failure=error;stop();});}catch(error){failed=true;failure=error;stop();}
  };
  control.signal.addEventListener("abort",stop,{once:true});
  try{if(stopped||control.signal.aborted)stop();else{timer=setInterval(run,interval);run();}await closed;await pending;if(failed)throw failure;}
  finally{stop();control.signal.removeEventListener("abort",stop);}
 });
 void operation.catch(()=>{});
 return Object.freeze({stop:async()=>{stop();await operation;}});
}
