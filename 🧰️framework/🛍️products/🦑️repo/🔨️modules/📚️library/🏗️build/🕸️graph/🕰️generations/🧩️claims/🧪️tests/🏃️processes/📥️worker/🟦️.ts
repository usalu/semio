import{appendFileSync}from"node:fs";
import{join}from"node:path";
import{acquireQueuedResourceLease,acquireResourceLease,acquireFreshResourceLeaseV1}from"@semio-tech/process/leases";
import{createClaimRegistryV1,createFileClaimLeasePortV1,createQueuedClaimLeasePortV1}from"../../../🟨️.mjs";
import{createNodeFilePortV1}from"../../../../🟨️.mjs";

const env=process.env,dir=env.CLAIMS_DIR!,log=env.CLAIMS_LOG!,name=env.CLAIMS_NAME!,wanted=JSON.parse(env.CLAIMS_WANTED!) as string[],mode=env.CLAIMS_MODE??"normal",milliseconds=Number(env.CLAIMS_RUN_MS??"30");
const write=(row:Record<string,unknown>)=>appendFileSync(log,JSON.stringify({name,pid:process.pid,...row})+"\n");
const registry=createClaimRegistryV1({root:join(dir,"generations"),fs:createNodeFilePortV1(),lease:env.CLAIMS_LEASE==="queued"?createQueuedClaimLeasePortV1({acquire:acquireQueuedResourceLease as never,tryAcquire:acquireResourceLease as never,acquireFresh:acquireFreshResourceLeaseV1 as never,directory:join(dir,"queued")}):createFileClaimLeasePortV1(join(dir,"leases"))});
const result=await registry.prefetch({base:4,implementation:"a".repeat(64),wanted,slots:Number(env.CLAIMS_SLOTS??"4"),run:async owners=>{
 write({event:"run",owners});
 if(mode==="hang"){write({event:"hang"});await new Promise(()=>{});}
 await new Promise(resolve=>setTimeout(resolve,milliseconds));
}});
write({event:"end",...result});
