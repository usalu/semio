/** 🛟️ Contract: the grant a slice worker holds at the capacity gate stays claimed for the whole life of the worker, not only at its start: the pool writes the claim again before the warden's freshness window (`claimMaximumMilliseconds`, 10 minutes) ends, so the warden keeps sheltering a planner inside its grant and never reads a long running one as unclaimed (a worker that ran 27 minutes was cancelled `overdue` at the guard level with the host at 3.5 GB available because its claim had expired).
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs */
import {expect,test} from "bun:test";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {HOST_WARDEN_POLICY_V1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as Record<string,number>,guard=HOST_POLICY_V1.slicePlanGuard as Record<string,number>,MiB=1048576;
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));

test("the claim of a running worker is written again within the freshness window of the warden",async()=>{
 const claims:{pid:number;grantMegabytes?:number;at:number}[]=[];let clock=0;
 let emit:(event:string,value?:unknown)=>void=()=>undefined;
 const refresh=Math.floor(HOST_WARDEN_POLICY_V1.claimMaximumMilliseconds/4);
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,sampleMilliseconds:10,heartbeatMilliseconds:20,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,planGuard:{...guard,settleWaitMilliseconds:20},hostPressure:()=>"normal",report:()=>undefined,now:()=>clock,
  claimRefreshMilliseconds:refresh,pressure:()=>({sample:()=>({tripped:false}),stop:()=>undefined}),headroom:()=>({limitBytes:Infinity,currentBytes:100*MiB,availableBytes:8000*MiB}),
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),observe:async()=>1000,
  claims:{claim:(pid:number,claim:{grantMegabytes?:number})=>{claims.push({pid,grantMegabytes:claim.grantMegabytes,at:clock});},release:()=>undefined},
  createWorker:()=>{const listeners:Record<string,(value?:unknown)=>void>={};emit=(event,value)=>listeners[event]?.(value);return{pid:4242,on:(event:string,listener:(value?:unknown)=>void)=>{listeners[event]=listener;},send:()=>true,terminate:async()=>0};}} as never);
 const run=pool.run({root:"/tmp/claim"});
 for(let tries=0;claims.length===0&&tries<200;tries++)await wait(10);
 expect(claims.length).toBe(1);
 clock+=refresh-1;await wait(60);expect(claims.length).toBe(1);
 clock+=2;for(let tries=0;claims.length<2&&tries<200;tries++)await wait(10);
 expect(claims.length).toBe(2);expect(claims[1]!.grantMegabytes).toBe(claims[0]!.grantMegabytes);
 expect(claims[1]!.at-claims[0]!.at).toBeLessThan(HOST_WARDEN_POLICY_V1.claimMaximumMilliseconds);
 emit("message",{type:"done",graph:{ok:1}});
 expect(await run).toEqual({graph:{ok:1}});await pool.close();
});
