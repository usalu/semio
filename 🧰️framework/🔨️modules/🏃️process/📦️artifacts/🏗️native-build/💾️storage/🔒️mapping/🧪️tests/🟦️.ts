import {test,expect} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {setTimeout as pause} from "node:timers/promises";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {withNativeStorageMappingReadV1} from "../🟦️.ts";
declareBuildTestEnvelopeV1({owner:"native-mapping-admission",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});
const fixture=async(body:(binding:Parameters<typeof withNativeStorageMappingReadV1>[0])=>Promise<void>)=>withBuildTestInvocationV1(async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(base,{recursive:true});const directory=mkdtempSync(join(base,"mapping-")),root=join(directory,"root"),leases=join(directory,"leases");mkdirSync(root);mkdirSync(leases);
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
