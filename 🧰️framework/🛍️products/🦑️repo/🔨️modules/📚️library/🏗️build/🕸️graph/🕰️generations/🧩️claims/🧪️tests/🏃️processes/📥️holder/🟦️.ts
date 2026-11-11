import{appendFileSync}from"node:fs";
import{exitWithParentV1}from"@semio-tech/process/test-orphans";
import{acquireFreshResourceLeaseV1,acquireQueuedResourceLease}from"@semio-tech/process/leases";

exitWithParentV1();
const env=process.env,log=env.HOLD_LOG!,name=env.HOLD_NAME!,write=(event:string)=>appendFileSync(log,JSON.stringify({name,event,at:Date.now()})+"\n");
const lease=env.HOLD_FRESH==="1"?await acquireFreshResourceLeaseV1({directory:env.HOLD_DIR!,resource:env.HOLD_RESOURCE!,mode:"exclusive",owner:name,signal:new AbortController().signal}):await acquireQueuedResourceLease({directory:env.HOLD_DIR!,resource:env.HOLD_RESOURCE!,mode:"shared",...(env.HOLD_COHORT==="1"?{sharedCohort:true}:{}),owner:name,signal:new AbortController().signal});
write("held");await new Promise(resolve=>setTimeout(resolve,Number(env.HOLD_MS??"1500")));lease.release();write("released");
