/** 📈️ Contract: a finished scheduler job that carries an observed peak teaches the host estimate store of its class, so the next estimate of that class is measured (and never learned from a job without a peak). */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {randomUUID} from "node:crypto";
import {runScheduledV1} from "../../🎟️ticket/🟦️.ts";
import {HOST_SERVICE_CLASS_V1,openHostSchedulerStoreV1,recordHostServicePeakV1,scheduledEstimateV1} from "../🟦️.ts";

const base=mkdtempSync(join(tmpdir(),"semio-learn-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const environment={...process.env,SEMIO_SCHEDULER_DIRECTORY:join(base,"scheduler"),SEMIO_HOST_CAPACITY_DIRECTORY:join(base,"capacity")} as Record<string,string>;
const request=(key:string,className:string)=>({id:randomUUID(),agent:"agent-1",workspace:"w",class:"compile" as const,priority:"agent" as const,depth:0,key,estimate:{class:className,floorMegabytes:0,limitMegabytes:256}});
const settle=async(until:()=>Promise<boolean>)=>{for(let attempt=0;attempt<100&&!await until();attempt++)await new Promise(resolve=>setTimeout(resolve,50));};

test("jobs with an observed peak teach their class, jobs without do not",async()=>{
 const store=openHostSchedulerStoreV1(environment);
 expect((await scheduledEstimateV1("phase:learn-test",64,environment)).learnedPeakMegabytes).toBeUndefined();
 await runScheduledV1(store,request("without","phase:learn-test"),async()=>({exit:"ok"}));
 await new Promise(resolve=>setTimeout(resolve,200));
 expect((await scheduledEstimateV1("phase:learn-test",64,environment)).learnedPeakMegabytes).toBeUndefined();
 for(let index=0;index<12;index++)await runScheduledV1(store,request("with-"+index,"phase:learn-test"),async()=>({exit:"ok",observedPeakMegabytes:900}));
 await settle(async()=>(await scheduledEstimateV1("phase:learn-test",64,environment)).learnedPeakMegabytes!==undefined);
 const estimate=await scheduledEstimateV1("phase:learn-test",64,environment);
 expect(estimate.learnedPeakMegabytes).toBeGreaterThanOrEqual(900);
 expect(estimate.limitMegabytes).toBeGreaterThanOrEqual(Math.round(900*1.25));
},60000);

test("the host service learns its own bounded peak in a class no client shares",async()=>{
 const own={...environment,SEMIO_HOST_CAPACITY_DIRECTORY:join(base,"service-capacity")};
 for(let index=0;index<12;index++)await recordHostServicePeakV1(own,index===0?90000:2000,2560);
 const service=await scheduledEstimateV1(HOST_SERVICE_CLASS_V1,64,own),client=await scheduledEstimateV1("phase:compile",64,own);
 expect(service.learnedPeakMegabytes).toBeLessThanOrEqual(2560);expect(service.learnedPeakMegabytes).toBeGreaterThanOrEqual(2000);
 expect(client.learnedPeakMegabytes).toBeUndefined();
});
