/** 🤲️ Tests for the leased generation file port: every operation of the five runs inside one lease hold and passes its arguments and results through; a refused lease rejects the operation without touching the files. */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readdirSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import{createNodeFilePortV1}from"../../🟨️.mjs";
import{createNativeImageSharedReaderV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts";
import{leasedGenerationFilePortV1}from"../🟦️.ts";
const none=new AbortController().signal;
const rig=(refuse=false)=>{
 const log={acquires:0,releases:0};
 const reader=createNativeImageSharedReaderV1({leaseDirectory:"/leases",imageDirectory:"/image",signal:none,port:{async acquire(){if(refuse)throw new Error("lease timed out");log.acquires++;return{release(){log.releases++;}};}}});
 return{log,reader};
};
test("every operation runs under the lease and equals the plain port on a real directory",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("leased-files"),"store-")),{log,reader}=rig();
 try{
  mkdirSync(join(directory,"s"));writeFileSync(join(directory,"s","a.json"),"{}");
  const plain=createNodeFilePortV1(),port=leasedGenerationFilePortV1(plain,reader);
  expect(await port.read(join(directory,"s","a.json"))).toEqual(await plain.read(join(directory,"s","a.json")));
  expect(await port.read(join(directory,"s","missing"))).toBeUndefined();
  expect(await port.names(join(directory,"s"))).toEqual(await plain.names(join(directory,"s")));
  expect((await port.list(join(directory,"s"))).map(row=>row.name)).toEqual(["a.json"]);
  expect(await port.write(join(directory,"s","b.json"),new TextEncoder().encode("[]"),{ifAbsent:true})).toBe(true);
  expect(await port.write(join(directory,"s","b.json"),new TextEncoder().encode("[1]"),{ifAbsent:true})).toBe(false);
  await port.remove(join(directory,"s","b.json"));expect(readdirSync(join(directory,"s"))).toEqual(["a.json"]);
  expect(log.acquires).toBe(1);await reader.close();expect(log.releases).toBe(1);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("a refused lease rejects the operation and no file is touched",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("leased-files"),"store-")),{reader}=rig(true);
 try{
  const port=leasedGenerationFilePortV1(createNodeFilePortV1(),reader);
  await expect(port.write(join(directory,"c.json"),new TextEncoder().encode("{}"))).rejects.toThrow("lease timed out");expect(readdirSync(directory)).toEqual([]);
  await expect(port.read(join(directory,"c.json"))).rejects.toThrow("lease timed out");
 }finally{rmSync(directory,{recursive:true,force:true});}
});
