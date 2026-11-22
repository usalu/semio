/** ✂️ Contract: a cut (or hand-off, or stall restart) starts the next worker only after the stopped one is confirmed gone, so two workers of one root never hold memory or write owner records at the same time; owner records are written to a temporary file and renamed over the record, so a stopped writer leaves no torn record.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs
 * @see ../../../👁️observation/🟨️.mjs */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number},guard=HOST_POLICY_V1.slicePlanGuard as {reserveMegabytes:number;ceilingBasisPoints:number;settleWaitMilliseconds:number},MiB=1048576;
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));

test("the next worker of a cut starts only after the stopped worker's termination has completed",async()=>{
 const created:number[]=[],listeners:Record<string,(value:unknown)=>void>[]=[];let release!:()=>void,terminated=0;const gate=new Promise<void>(resolve=>{release=resolve;});
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,sampleMilliseconds:10,heartbeatMilliseconds:30,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,planGuard:{...guard,settleWaitMilliseconds:20},hostPressure:()=>"normal",report:()=>undefined,
  headroom:()=>({limitBytes:Infinity,currentBytes:0,availableBytes:8000*MiB}),reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),observe:async()=>created.length===1?20000:1000,
  createWorker:()=>{const listener:Record<string,(value:unknown)=>void>={};listeners.push(listener);created.push(Date.now());return{pid:4242,on:(event:string,fn:(value:unknown)=>void)=>{listener[event]=fn;},send:()=>true,terminate:async()=>{terminated++;if(created.length===1)await gate;return 0;}};}});
 const run=pool.run({root:"/tmp/order"});
 for(let tries=0;created.length<1&&tries<300;tries++)await wait(10);
 await wait(300);
 expect(created).toHaveLength(1);expect(terminated).toBe(1);
 release();
 for(let tries=0;created.length<2&&tries<300;tries++)await wait(10);
 expect(created).toHaveLength(2);listeners[1]!.message?.({type:"done",graph:{ok:true}});expect(await run).toEqual({graph:{ok:true}});await pool.close();
});

test("owner records and the receipt are published crash-safely through the durable publication helper, never by an unsynced write and rename",()=>{
 const source=readFileSync(new URL("../../../👁️observation/🟨️.mjs",import.meta.url),"utf8");
 expect(source).toContain("publishFileDurablySyncV1(temporary,cachePath,packed)");expect(source).toContain("publishFileDurablySyncV1(temporary, receiptPath,");
 expect(source).toMatch(/\$\{cachePath\}\.\$\{process\.pid\}\.\$\{started\}\.tmp/u);
 expect(source).not.toMatch(/fs\.writeFileSync\(temporary/u);expect(source).not.toContain("renameOverBusyFileSyncV1");
});
