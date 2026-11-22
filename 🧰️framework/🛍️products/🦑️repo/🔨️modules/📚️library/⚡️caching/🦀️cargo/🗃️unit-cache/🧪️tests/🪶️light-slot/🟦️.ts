import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireLightSlotV1,acquireSlotV1,lightSlotsHeldV1} from "../../🔒️lock/🟦️.ts";
import {slotQueueMillisecondsV1} from "../../../../../🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts";
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"semio-light-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
test("light slots are not charged against the local compile slots: every local slot stays free while light slots are held, and the other way round",async()=>{
 const root=fresh(),light=await Promise.all([0,1,2].map(()=>acquireLightSlotV1(root,3)));
 expect(lightSlotsHeldV1(root,3)).toBe(3);expect(slotQueueMillisecondsV1(root,2),"local slots still free").toBe(0);
 const local=[await acquireSlotV1(root,2),await acquireSlotV1(root,2)];
 expect(slotQueueMillisecondsV1(root,2),"all local slots taken").toBeGreaterThan(0);expect(lightSlotsHeldV1(root,3)).toBe(3);
 for(const claim of local)claim.release();for(const claim of light)claim.release();
 expect(lightSlotsHeldV1(root,3)).toBe(0);expect(readdirSync(join(root,"s")).filter(name=>name.endsWith(".lock"))).toEqual([]);
});
test("a unit waits for a light slot instead of failing and gets it the moment one is released",async()=>{
 const root=fresh(),first=await acquireLightSlotV1(root,1);let waited=0;
 const second=acquireLightSlotV1(root,1,{onWait:milliseconds=>{waited=milliseconds;}});
 const pending=await Promise.race([second.then(()=>"granted"),new Promise<string>(done=>setTimeout(()=>done("waiting"),400))]);
 expect(pending).toBe("waiting");first.release();
 const claim=await second;expect(lightSlotsHeldV1(root,1)).toBe(1);expect(waited).toBeGreaterThan(0);claim.release();expect(lightSlotsHeldV1(root,1)).toBe(0);
});
test("a light slot count below one is refused and a cancelled wait leaves nothing behind",async()=>{
 const root=fresh();await expect(acquireLightSlotV1(root,0)).rejects.toThrow("Positive light slot count required");
 const held=await acquireLightSlotV1(root,1),controller=new AbortController(),waiting=acquireLightSlotV1(root,1,{signal:controller.signal});
 setTimeout(()=>controller.abort(Error("cancelled")),100);await expect(waiting).rejects.toThrow("cancelled");
 held.release();expect(lightSlotsHeldV1(root,1)).toBe(0);
});
