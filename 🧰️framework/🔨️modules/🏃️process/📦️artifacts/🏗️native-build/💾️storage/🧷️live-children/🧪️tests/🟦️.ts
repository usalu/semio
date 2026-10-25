import{test,expect}from"bun:test";
import{existsSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{holdNativeLiveChildV1,nativeLiveChildV1}from"../🟦️.ts";
const leaseModule=join(import.meta.dir,"../../../../../🔒️leases/🟦️.ts");
test("a child is live while this process registered it (nested holds count), and a leaked one is not",()=>{
 const root=mkdtempSync(join(testScratchRootV1("live-children"),"lc-")),leases=join(root,"leases"),child=join(root,"tmp","cc-1");
 try{
  expect(nativeLiveChildV1(child,leases)).toBe(false);
  const first=holdNativeLiveChildV1(child),second=holdNativeLiveChildV1(child);
  expect(nativeLiveChildV1(child,leases)).toBe(true);
  first();first();expect(nativeLiveChildV1(child,leases)).toBe(true);
  second();expect(nativeLiveChildV1(child,leases)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("a child whose image lease another live process holds is live; once that process lets go it is a leak again",async()=>{
 const root=mkdtempSync(join(testScratchRootV1("live-children"),"lc-")),leases=join(root,"leases"),child=join(root,"tmp","cc-2"),held=join(root,"held"),stop=join(root,"stop");
 const script=`const fs=await import("node:fs"),{acquireResourceLease}=await import(process.argv[1]);const lease=await acquireResourceLease({directory:process.argv[2],resource:"native-fixed-image:"+process.argv[3],mode:"shared",signal:new AbortController().signal});fs.writeFileSync(process.argv[4],"1");while(!fs.existsSync(process.argv[5]))await new Promise(done=>setTimeout(done,50));lease.release();`;
 const proc=Bun.spawn([process.execPath,"-e",script,leaseModule,leases,child,held,stop],{stdin:"ignore",stdout:"inherit",stderr:"inherit"});
 const until=async(check:()=>boolean)=>{for(let at=0;at<100&&!check();at++)await Bun.sleep(100);return check();};
 try{
  expect(await until(()=>existsSync(held))).toBe(true);
  expect(await until(()=>nativeLiveChildV1(child,leases))).toBe(true);
  writeFileSync(stop,"1");await proc.exited;
  expect(await until(()=>!nativeLiveChildV1(child,leases))).toBe(true);
 }finally{try{proc.kill();}catch{}rmSync(root,{recursive:true,force:true});}
},30000);
