/** 🧍️ Contract: a unit that falls back from the fleet compiles alone on the host: it holds every compile slot, so no other unit compiles beside it, units wait until it is done, and a cancelled wait leaves nothing behind.
 * @see ../../🔒️lock/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireAloneV1,acquireSlotV1,tryAcquireSlotV1} from "../../🔒️lock/🟦️.ts";
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"semio-alone-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const locks=(root:string)=>{try{return readdirSync(join(root,"s")).filter(name=>name.endsWith(".lock"));}catch{return[];}};

test("alone holds every slot: nothing else is admitted until it is released",async()=>{
 const root=fresh(),alone=await acquireAloneV1(root,3);
 expect(tryAcquireSlotV1(root,3)).toBeUndefined();
 const other=acquireSlotV1(root,3),state=await Promise.race([other.then(()=>"granted"),new Promise<string>(done=>setTimeout(()=>done("waiting"),300))]);
 expect(state).toBe("waiting");
 alone.release();
 (await other).release();
 expect(locks(root)).toEqual([]);
});

test("two lone units never overlap and a cancelled wait leaves no claim behind",async()=>{
 const root=fresh(),first=await acquireAloneV1(root,2),controller=new AbortController(),second=acquireAloneV1(root,2,{signal:controller.signal});
 setTimeout(()=>controller.abort(Error("cancelled")),100);
 await expect(second).rejects.toThrow("cancelled");
 first.release();
 const third=await acquireAloneV1(root,2);third.release();
 expect(locks(root)).toEqual([]);
});
