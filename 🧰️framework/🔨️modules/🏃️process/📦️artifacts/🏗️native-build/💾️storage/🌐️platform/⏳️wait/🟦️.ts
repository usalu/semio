import {createRequestedIssuedResourceControlV1} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl,type ResourceLease,type LeaseWait} from "../../../../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import type {NativeImageMaintenanceWaitObserverV1} from "../📋️contract/🟦️.ts";

/** 🛑️ Withdraws only its own queued writer after exact DATA observation and joined acquisition cleanup. */
export async function acquireNativeImageMaintenanceWaitV1(control:ResourceLeaseControl,resource:string,acquire:(actual:ResourceLeaseControl,onWait?:((progress:LeaseWait)=>Promise<void>))=>Promise<ResourceLease>,observe:()=>Promise<unknown>,withdrawWhen?:NativeImageMaintenanceWaitObserverV1):Promise<Readonly<{acquired:true;lease:ResourceLease;control:ResourceLeaseControl}|{acquired:false;withdrawn:true}>>{
 if(withdrawWhen!==undefined&&typeof withdrawWhen!=="function")throw Error("Actual Native maintenance wait observer required");checkResourceLeaseControl(control);if(!withdrawWhen)return{acquired:true,lease:await acquire(control),control};
 const stop=new AbortController(),withdrawn=Object.freeze({}),child=createRequestedIssuedResourceControlV1(control,0,stop.signal);let observing:Promise<void>|undefined,lease:ResourceLease|undefined,failed=false,failure:unknown;
 try{lease=await acquire(child,progress=>{if(progress.resource!==resource)return Promise.resolve();const work=(async()=>{checkResourceLeaseControl(control);const value=await observe();checkResourceLeaseControl(control);if(validateJsonSchemaSubset(schema.definitions.ImageMaintenanceWaitObservation,value,schema).length)throw Error("Closed Native maintenance wait observation required");const row=value as {availableBytes:number},observation=Object.freeze({version:1 as const,availableBytes:row.availableBytes}),decision=withdrawWhen(observation);if(typeof decision!=="boolean")throw Error("Boolean Native maintenance withdrawal required");checkResourceLeaseControl(control);if(decision)stop.abort(withdrawn);})();observing=work;work.catch(()=>{});return work;});}catch(error){failed=true;failure=error;}
 if(observing)try{await observing;}catch(error){if(!failed){failed=true;failure=error;}}
 try{checkResourceLeaseControl(control);}catch(error){failed=true;failure=error;}
 if(!failed)try{checkResourceLeaseControl(child);}catch(error){failed=true;failure=error;}
 if(failed){if(lease)try{lease.release();}catch(error){if(failure===withdrawn)throw error;throw new AggregateError([failure,error],"Native maintenance wait and acquired release failed");}if(failure===withdrawn&&stop.signal.aborted&&stop.signal.reason===withdrawn)return Object.freeze({acquired:false,withdrawn:true});throw failure;}
 return{acquired:true,lease:lease!,control:child};
}
