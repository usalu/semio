/** 🔒️ Concurrent put and restore across two real processes over the real lease library and the real output resource of the receipt of `@semio-tech/norm-din4108:build`: a put (output read, shared lease) waits for a restoring writer (exclusive lease) of another process and is admitted when it releases; two puts hold the output together; a restore waits for a put. */
import {expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {classifyNxOutputV1} from "../../../📦️outputs/🟦️.ts";
const workspace="/Users/ueli/Documents/semio",receipt=workspace+"/🧰️framework/🧮️norm/🏛️din4108/.🧾️store/build.json",resource=classifyNxOutputV1({rust:workspace+"/.cache/b/t/m",products:workspace+"/.cache/b/p/m",receipts:workspace},receipt).resource;
const withEnvelope=<T>(body:(original:Parameters<Parameters<typeof withScriptProcessEnvelope<T>>[1]>[0])=>Promise<T>):Promise<T>=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"nx-nested-custody-test",maximumElapsedMilliseconds:60000},{},Date.now()),body);
const holder=async(directory:string,mode:"shared"|"exclusive")=>{
 const child=Bun.spawn([process.execPath,resolve(import.meta.dir,"./🧱️holder/🟦️.ts"),directory,resource,mode],{stdin:"pipe",stdout:"pipe",stderr:"inherit"}),reader=child.stdout.getReader();let text="";
 while(!text.includes("held")){const row=await reader.read();if(row.done)throw Error("holder exited before it held the lease");text+=new TextDecoder().decode(row.value);}
 return{release:async()=>{child.stdin.end();await child.exited;}};
};
const acquire=(original:Parameters<Parameters<typeof withScriptProcessEnvelope>[1]>[0],directory:string,mode:"shared"|"exclusive",timeoutMs:number)=>{const control=createIssuedResourceControlV1(original,"nx:nested-custody-test",0);return acquireResourceLease({directory,resource,mode,signal:control.signal,control,timeoutMs});};
test("the receipt output resource is the artifact resource of its path",()=>{expect(resource).toBe("artifact:"+receipt);});
test("a put waits for a restoring writer of another process and is admitted when it releases",()=>withEnvelope(async original=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir(),"nx-nested-custody-"));
 try{
  const restoring=await holder(directory,"exclusive");
  await expect(acquire(original,directory,"shared",300)).rejects.toThrow("timed out");
  const waiting=acquire(original,directory,"shared",20000);await restoring.release();const lease=await waiting;lease.release();
 }finally{rmSync(directory,{recursive:true,force:true});}
}),60000);
test("two puts of two processes hold the output together and a restore waits for them",()=>withEnvelope(async original=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir(),"nx-nested-custody-"));
 try{
  const other=await holder(directory,"shared"),mine=await acquire(original,directory,"shared",300);
  await expect(acquire(original,directory,"exclusive",300)).rejects.toThrow("timed out");
  mine.release();await other.release();const writer=await acquire(original,directory,"exclusive",5000);writer.release();
 }finally{rmSync(directory,{recursive:true,force:true});}
}),60000);
