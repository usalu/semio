import{appendFileSync}from"node:fs";
import{acquireQueuedResourceLease}from"@semio-tech/process/leases";

const env=process.env,log=env.HOLD_LOG!,name=env.HOLD_NAME!,write=(event:string)=>appendFileSync(log,JSON.stringify({name,event,at:Date.now()})+"\n");
const lease=await acquireQueuedResourceLease({directory:env.HOLD_DIR!,resource:env.HOLD_RESOURCE!,mode:"shared",...(env.HOLD_COHORT==="1"?{sharedCohort:true}:{}),owner:name,signal:new AbortController().signal});
write("held");await new Promise(resolve=>setTimeout(resolve,Number(env.HOLD_MS??"1500")));lease.release();write("released");
