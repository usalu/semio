import{test,expect}from"bun:test";
import{createHash}from"node:crypto";
const toolsSource=await Bun.file(new URL("../../../../🟦️.ts",import.meta.url)).text();
import{nxSdkDerivativeDispositionV1}from"../../../../📦️sdk/📥️source/🔗️derivative/🟦️.ts";
const plan=()=>import("../../🧾️sealed-plan/🟦️.ts");
const snapshot="a".repeat(64),digest=(value:string)=>createHash("sha256").update(value).digest("hex");
const entries=(count:number)=>[{path:"node_modules",kind:"directory"},...Array.from({length:count},(_,i)=>({path:"node_modules/nx/f"+String(i).padStart(6,"0")+".js",kind:"file",bytes:i*37+1,sha256:digest(String(i)),executable:false})),...Array.from({length:count},(_,i)=>({path:"node_modules/other/f"+String(i).padStart(6,"0")+".js",kind:"file",bytes:i*11+5,sha256:digest("o"+i),executable:false}))].sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)));
const bytesOf=(count:number,snap=snapshot)=>Buffer.from(JSON.stringify({version:1,snapshot:snap,entries:entries(count)})+"\n");
const reference=(index:{entries:readonly{path:string;kind:"file"|"directory"|"link";bytes?:number}[]},unit:number)=>{let copyAllocatedBytes=0,ownedBytes=0;for(const row of index.entries)if(row.kind==="file"&&nxSdkDerivativeDispositionV1({path:row.path,kind:row.kind})==="copy"){ownedBytes+=row.bytes!;copyAllocatedBytes+=Math.ceil(row.bytes!/unit)*unit;}return{copyAllocatedBytes,ownedBytes};};
test("the planned copy bytes equal the direct loop at two allocation units",async()=>{
 const{sealedSdkIndexV1,sdkIndexCopyBytesV1}=await plan(),index=sealedSdkIndexV1(bytesOf(600),snapshot);expect(reference(index,4096).ownedBytes).toBeGreaterThan(0);for(const unit of[4096,65536])expect(await sdkIndexCopyBytesV1(index,unit,()=>{},async()=>{})).toEqual(reference(index,unit));
});
test("the second planning of the same sealed index is O(1): the same object, no entry walk, no progress step",async()=>{
 const{sealedSdkIndexV1,sdkIndexCopyBytesV1}=await plan(),bytes=bytesOf(3000);let first=0,second=0;const a=sealedSdkIndexV1(bytes,snapshot),one=await sdkIndexCopyBytesV1(a,4096,()=>{},async()=>{first++;}),b=sealedSdkIndexV1(Buffer.from(bytes),snapshot),two=await sdkIndexCopyBytesV1(b,4096,()=>{},async()=>{second++;});
 expect(b).toBe(a);expect(two).toEqual(one);expect(first).toBeGreaterThan(40);expect(second).toBe(0);
});
test("another unit, other bytes or another snapshot never reuse a verdict",async()=>{
 const{sealedSdkIndexV1,sdkIndexCopyBytesV1}=await plan(),bytes=bytesOf(300),a=sealedSdkIndexV1(bytes,snapshot);let steps=0;await sdkIndexCopyBytesV1(a,4096,()=>{},async()=>{steps++;});const other=await sdkIndexCopyBytesV1(a,65536,()=>{},async()=>{steps++;});expect(other).toEqual(reference(a,65536));
 expect(()=>sealedSdkIndexV1(bytes,"b".repeat(64))).toThrow("Exact planned sealed SDK index required");expect(()=>sealedSdkIndexV1(Buffer.from(bytes.toString().replace("\n"," \n")),snapshot)).toThrow("Exact planned sealed SDK index required");
 const changed=bytesOf(301);expect(sealedSdkIndexV1(changed,snapshot)).not.toBe(a);
});
test("a cancelled check stops the walk and is not remembered",async()=>{
 const{sealedSdkIndexV1,sdkIndexCopyBytesV1}=await plan(),a=sealedSdkIndexV1(bytesOf(500),snapshot);let calls=0;await expect(sdkIndexCopyBytesV1(a,512,()=>{if(++calls>10)throw Error("cancelled");},async()=>{})).rejects.toThrow("cancelled");expect(await sdkIndexCopyBytesV1(a,512,()=>{},async()=>{})).toEqual(reference(a,512));
});
test("the storage planning of the Nx tool reads the sealed index once per content",()=>{
 expect(toolsSource).toContain("sealedSdkIndexV1(bytes,captured.sdkDigest)");expect(toolsSource).toContain("sdkIndexCopyBytesV1(index,unit");expect(toolsSource).not.toContain('JSON.stringify(index)+"\\n"!==bytes.toString()');
});
