import {test,expect} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";
import {setTimeout as pause} from "node:timers/promises";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {withNativeStorageMappingReadV1,nativeStorageMappingQueueV1} from "../🟦️.ts";
declareBuildTestEnvelopeV1({owner:"native-mapping-admission",maximumElapsedMilliseconds:60000,capabilities:{}});
const fixture=async(body:(binding:Parameters<typeof withNativeStorageMappingReadV1>[0])=>Promise<void>)=>withBuildTestInvocationV1(async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(base,{recursive:true});const directory=mkdtempSync(join(base,"mapping-")),root=join(directory,"root"),leases=join(directory,"leases");mkdirSync(root);mkdirSync(leases);
 try{await body({root,identity:"mapping-test",leaseDirectory:leases,control:createIssuedResourceControlV1(original,"native:mapping-admission",0)});}finally{rmSync(directory,{recursive:true,force:true});}
});
test("100 concurrent outermost mapping reads all complete instead of exhausting the owner table",()=>fixture(async binding=>{
 const rows=await Promise.all(Array.from({length:100},(_,index)=>withNativeStorageMappingReadV1(binding,async()=>{await pause(2);return index;})));
 expect(rows).toEqual(Array.from({length:100},(_,index)=>index));
}),30000);
test("100 nested mapping reads in one transaction complete in arrival order and a fire-and-forget tail finishes before the frame dies",()=>fixture(async binding=>{
 const order:number[]=[];let tail:Promise<number[]>|undefined;
 const rows=await withNativeStorageMappingReadV1(binding,async()=>{
  tail=Promise.all(Array.from({length:70},(_,index)=>withNativeStorageMappingReadV1(binding,async()=>{await pause(1);return index;})));
  return Promise.all(Array.from({length:100},(_,index)=>withNativeStorageMappingReadV1(binding,async()=>{order.push(index);await pause(1);return index;})));
 });
 expect(rows).toEqual(Array.from({length:100},(_,index)=>index));expect(order).toEqual(rows);expect(await tail).toEqual(Array.from({length:70},(_,index)=>index));
}),30000);
test("a nested mapping read after its transaction released is still refused",()=>fixture(async binding=>{
 const late=await new Promise<unknown>(resolve=>{void withNativeStorageMappingReadV1(binding,async()=>{setTimeout(()=>{try{resolve(withNativeStorageMappingReadV1(binding,async()=>0));}catch(error){resolve(error);}},50);});});
 expect(late).toBeInstanceOf(Error);expect(String((late as Error).message)).toContain("Released parent mapping operation refused");
}),30000);

test("the mapping queue names who waits for what and for how long: owner slots are told apart from the lease grant, finished waits are tallied",()=>fixture(async binding=>{
 const before=nativeStorageMappingQueueV1(),seen:{slot:number;grant:number;slots:number}[]=[];
 const sampler=setInterval(()=>{const now=nativeStorageMappingQueueV1();seen.push({slot:now.waiting.filter(row=>row.step==="slot").length,grant:now.waiting.filter(row=>row.step==="grant").length,slots:now.ownerSlots.retained});},3);
 const rows=await Promise.all(Array.from({length:48},(_,index)=>withNativeStorageMappingReadV1(binding,async()=>{await pause(25);return index;})));
 clearInterval(sampler);
 const after=nativeStorageMappingQueueV1();
 expect(rows.length).toBe(48);
 expect(Math.max(...seen.map(row=>row.slot)),"the 33rd read waits for an owner slot, not for the lease").toBeGreaterThan(0);
 expect(Math.max(...seen.map(row=>row.slots))).toBeLessThanOrEqual(after.ownerSlots.maximum);
 expect(after.waiting).toEqual([]);
 expect(after.granted.shared.count-before.granted.shared.count).toBe(48);
 expect(after.granted.shared.maximumMilliseconds).toBeGreaterThanOrEqual(20);
 expect(after.holders).toEqual({shared:0,exclusive:0,exclusiveHeldMilliseconds:0});
}),30000);
test("shared mapping readers never wait behind each other: 32 concurrent readers are inside together",()=>fixture(async binding=>{
 const before=nativeStorageMappingQueueV1().granted.shared.count,waited:number[]=[];let inside=0,peak=0;
 await Promise.all(Array.from({length:32},async()=>{const asked=performance.now();await withNativeStorageMappingReadV1(binding,async()=>{waited.push(performance.now()-asked);inside++;peak=Math.max(peak,inside);await pause(1500);inside--;});}));
 expect(nativeStorageMappingQueueV1().granted.shared.count-before).toBe(32);
 expect(peak,"no reader waited for another to leave").toBe(32);
 expect(Math.max(...waited),"the grants cost one lease acquisition each, no more").toBeLessThan(1400);
}),30000);
