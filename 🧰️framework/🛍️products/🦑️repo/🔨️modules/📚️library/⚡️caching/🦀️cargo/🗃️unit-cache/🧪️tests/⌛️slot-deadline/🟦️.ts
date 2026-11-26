import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {SlotWaitTimeoutV1,acquireSlotV1,tryAcquireSlotV1} from "../../🔒️lock/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../🛰️fleet/📡️client/🟦️.ts";
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=():string=>{const root=mkdtempSync(join(tmpdir(),"semio-slot-deadline-"));roots.push(root);return root;};
test("a live holder whose compile never finishes cannot stall a waiter beyond its deadline, and the waiter leaves no ticket behind",async()=>{
 const root=scratch(),held=tryAcquireSlotV1(root,1)!,started=Date.now();
 const outcome=await acquireSlotV1(root,1,{maximumWaitMilliseconds:700}).then(()=>undefined,error=>error);
 expect(outcome).toBeInstanceOf(SlotWaitTimeoutV1);
 expect(Date.now()-started).toBeLessThan(3000);
 expect(readdirSync(join(root,"q"),{withFileTypes:true}).length).toBe(0);
 held.release();
 const next=await acquireSlotV1(root,1,{maximumWaitMilliseconds:700});
 next.release();
});
test("a waiter without a deadline keeps waiting for the slot as before",async()=>{
 const root=scratch(),held=tryAcquireSlotV1(root,1)!;
 const waiter=acquireSlotV1(root,1);
 await new Promise(done=>setTimeout(done,900));
 held.release();
 (await waiter).release();
 expect(existsSync(join(root,"s","0.lock"))).toBe(false);
});
test("a hung cache server cannot stall a fleet request beyond the client deadline",async()=>{
 const root=scratch(),server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:()=>new Promise<Response>(()=>undefined)});
 try{
  const settings=readFleetSettingsV1(root,{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:"+String(server.port),SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"1",SEMIO_REMOTE_CACHE_RETRIES:"0",SEMIO_REMOTE_CACHE_INSTANCE:"main"})!;
  const client=createFleetClientV1(settings),started=Date.now();
  const claim=await client.claim("a".repeat(64),"owner-1",30000);
  expect(claim.state).toBe("unavailable");
  expect(Date.now()-started).toBeLessThan(5000);
 }finally{server.stop(true);}
},20000);
