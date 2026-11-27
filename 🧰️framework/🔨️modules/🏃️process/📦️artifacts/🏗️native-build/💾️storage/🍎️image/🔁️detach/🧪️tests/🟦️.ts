/** 🔁️ Tests for the busy-detach retry of a mounted image. */
import{expect,test}from"bun:test";
import{spawn,spawnSync}from"node:child_process";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{NATIVE_IMAGE_DETACH_POLICY_V1,createNativeImageDetachPortV1,detachNativeImageWithRetryV1,nativeImageDetachBackoffMillisecondsV1,nativeImageDetachBusyV1,nativeImageDetachSleepV1,parseNativeImageHoldersV1,type NativeImageDetachPortV1,type NativeImageHolderV1}from"../🟦️.ts";
const busy=()=>new Error("/usr/bin/hdiutil detach /x/m exited with status 16 signal null\nhdiutil: couldn't unmount \"disk11\" - Resource busy");
const holder:NativeImageHolderV1=Object.freeze({pid:4242,startMs:Date.UTC(2026,10,26,12,0,0),command:"node"});
const fake=(busyTimes:number,failure:()=>Error=busy)=>{
 const log={detaches:0,sleeps:[] as number[],said:[] as string[],census:0};
 const port:NativeImageDetachPortV1={detach:async()=>{log.detaches++;if(log.detaches<=busyTimes)throw failure();},holders:async()=>{log.census++;return[holder];},sleep:async ms=>{log.sleeps.push(ms);}};
 return{log,port};
};
test("the policy and texts are closed, bilingual and bounded",()=>{
 const policy=NATIVE_IMAGE_DETACH_POLICY_V1,texts=schema.definitions.Texts.const;
 expect(policy.maximumAttempts).toBeGreaterThan(1);
 expect(policy.backoffMilliseconds.every((value,index,all)=>value>0&&(index===0||value>=all[index-1]!))).toBe(true);
 expect(policy.holderArguments).toContain("<mountPoint>");
 for(const row of Object.values(texts))expect(Object.keys(row).sort()).toEqual(["de","en"]);
 expect(validateJsonSchemaSubset({type:"object",required:["Policy","Texts"]},schema.definitions)).toEqual([]);
});
test("only a busy refusal is retryable, on every platform's wording",()=>{
 expect(nativeImageDetachBusyV1(busy())).toBe(true);
 expect(nativeImageDetachBusyV1(new Error("/usr/bin/umount /x exited with status 32 signal null\numount: /x: target is busy"))).toBe(true);
 expect(nativeImageDetachBusyV1(new Error("The process cannot access the file because it is being used by another process"))).toBe(true);
 expect(nativeImageDetachBusyV1(new Error("/usr/bin/hdiutil detach /x exited with status 1 signal null\nno such disk"))).toBe(false);
 expect(nativeImageDetachBusyV1("Resource busy")).toBe(true);
});
test("the back-off follows the declared steps and repeats the last",()=>{
 const steps=NATIVE_IMAGE_DETACH_POLICY_V1.backoffMilliseconds;
 expect(steps.map((_,index)=>nativeImageDetachBackoffMillisecondsV1(index))).toEqual([...steps]);
 expect(nativeImageDetachBackoffMillisecondsV1(999)).toBe(steps[steps.length-1]!);
 expect(nativeImageDetachBackoffMillisecondsV1(-3)).toBe(steps[0]!);
});
test("busy N times then success: N retries, N bilingual progress lines naming the holder, never forced",async()=>{
 const{log,port}=fake(3),said:string[]=[];
 await detachNativeImageWithRetryV1({port,mountPoint:"/x/m",say:line=>said.push(line),check:()=>{}});
 expect(log.detaches).toBe(4);
 expect(log.sleeps).toEqual([500,1000,2000]);
 expect(said.length).toBe(3);
 said.forEach((line,index)=>{expect(line).toContain("is busy (attempt "+(index+1)+" of "+NATIVE_IMAGE_DETACH_POLICY_V1.maximumAttempts+")");expect(line).toContain("ist belegt (Versuch "+(index+1)+" von ");expect(line).toContain("node pid 4242 start 2026-11-26T12:00:00.000Z");expect(line).toContain("/x/m");});
});
test("an image that never frees ends in a bilingual refusal naming the holders after the declared attempts",async()=>{
 const{log,port}=fake(Number.MAX_SAFE_INTEGER),said:string[]=[];
 const error=await detachNativeImageWithRetryV1({port,mountPoint:"/x/m",say:line=>said.push(line),check:()=>{}}).then(()=>undefined,reason=>reason as Error);
 const attempts=NATIVE_IMAGE_DETACH_POLICY_V1.maximumAttempts;
 expect(log.detaches).toBe(attempts);expect(log.sleeps.length).toBe(attempts-1);expect(said.length).toBe(attempts-1);
 expect(error?.message).toContain("stayed busy after "+attempts+" attempts");expect(error?.message).toContain("blieb nach "+attempts+" Versuchen belegt");expect(error?.message).toContain("node pid 4242");
 expect((error as Error&{cause?:unknown}).cause).toBeInstanceOf(Error);
});
test("a refusal that is not busy propagates at once",async()=>{
 const{log,port}=fake(5,()=>new Error("/usr/bin/hdiutil detach /x/m exited with status 1 signal null"));
 await expect(detachNativeImageWithRetryV1({port,mountPoint:"/x/m",say:()=>{},check:()=>{}})).rejects.toThrow("status 1");
 expect(log.detaches).toBe(1);expect(log.sleeps).toEqual([]);expect(log.census).toBe(0);
});
test("cancellation ends the wait and no further detach starts",async()=>{
 const{log,port}=fake(Number.MAX_SAFE_INTEGER);let cancelled=false;
 const sleeping:NativeImageDetachPortV1={...port,sleep:async ms=>{log.sleeps.push(ms);if(log.sleeps.length===2)cancelled=true;}};
 await expect(detachNativeImageWithRetryV1({port:sleeping,mountPoint:"/x/m",say:()=>{},check:()=>{if(cancelled)throw new Error("Cancelled\nAbgebrochen");}})).rejects.toThrow("Abgebrochen");
 expect(log.detaches).toBe(2);expect(log.sleeps.length).toBe(2);
});
test("an image that a concurrent detach already removed ends the loop quietly",async()=>{
 const{log,port}=fake(Number.MAX_SAFE_INTEGER);
 await detachNativeImageWithRetryV1({port,mountPoint:"/x/m",say:()=>{},check:()=>{},ready:()=>true});
 expect(log.detaches).toBe(1);expect(log.sleeps).toEqual([]);
});
test("the sleep stops as soon as the check throws",async()=>{
 let polls=0;const started=Date.now();
 await expect(nativeImageDetachSleepV1(()=>{if(++polls>2)throw new Error("Cancelled\nAbgebrochen");},5)(60000)).rejects.toThrow("Cancelled");
 expect(Date.now()-started).toBeLessThan(5000);
});
test("the macOS port detaches without force and reads the holders with lsof, enriched by the process identity",async()=>{
 const calls:string[][]=[];
 const port=createNativeImageDetachPortV1({cwd:"/x",run:async(program,args)=>{calls.push([program,...args]);return program==="lsof"?"p10\ncnode\np11\ncbun\n":"";},sleep:async()=>{},startOf:pid=>pid*1000});
 await port.detach("/x/m");
 expect(calls[0]).toEqual(["/usr/bin/hdiutil","detach","/x/m"]);
 expect(await port.holders("/x/m")).toEqual([{pid:10,command:"node",startMs:10000},{pid:11,command:"bun",startMs:11000}]);
 expect(calls[1]).toEqual(["lsof","-F","pc","+f","--","/x/m"]);
 const silent=createNativeImageDetachPortV1({cwd:"/x",run:async()=>{throw new Error("lsof exited with status 1 signal null");},sleep:async()=>{}});
 expect(await silent.holders("/x/m")).toEqual([]);
});
test("the holder parser reads the lsof field output",()=>{
 expect(parseNativeImageHoldersV1("p30\ncnode\nf1\np7\ncsh\np30\ncdup\nzzz\np0\ncbad\n")).toEqual([{pid:7,command:"sh"},{pid:30,command:"node"}]);
 expect(parseNativeImageHoldersV1("")).toEqual([]);
});
test("the parser equals the real lsof on a real holder (third-party oracle)",async()=>{
 if(process.platform==="win32"||spawnSync("lsof",["-v"]).error)return;
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"detach-holder-")));
 const child=spawn("sleep",["30"],{cwd:directory,stdio:"ignore"});
 try{
  await new Promise(resolve=>setTimeout(resolve,300));
  const real=spawnSync("lsof",["-F","pc","--",directory],{encoding:"utf8"});
  const found=parseNativeImageHoldersV1(String(real.stdout)).find(row=>row.pid===child.pid);
  expect(found?.command).toBe("sleep");
 }finally{child.kill();rmSync(directory,{recursive:true,force:true});}
});
