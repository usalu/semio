import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdirSync,mkdtempSync,rmSync,statfsSync,writeFileSync,readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import fixture from "./🧫️fixtures/🔣️.json";
import {activateNativeDirectoryV1,allocateNativeDirectoryV1,deactivateNativeDirectoryV1,measureNativeDirectoryUsageV1,nativeDirectoryUsageV1,nativeDirectoryProtocolV1} from "../../../../📁️directory/🟦️.ts";
import {nativeDirectoryUsedBytesSyncV1,nativePartitionOfV1,nativeStatfsSyncV1,nativeStatfsV1} from "../🟦️.ts";

const protocol=nativeDirectoryProtocolV1();
const scratch=()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();mkdirSync(base,{recursive:true});return mkdtempSync(join(base,"sfs-"));};
const sh=(...args:string[])=>Bun.spawnSync(args,{stdout:"pipe",stderr:"pipe"});
const partition=(base:string,name:string,attach:boolean)=>{const directory=join(base,name);mkdirSync(directory,{recursive:true});mkdirSync(join(directory,"m"));allocateNativeDirectoryV1({directory,filesystemBytes:fixture.envelope.filesystemBytes,maximumPhysicalBytes:fixture.envelope.maximumPhysicalBytes},()=>true);if(attach)activateNativeDirectoryV1(directory);return directory;};
const walk=`import os,stat,sys
seen=set();t=0
def w(p):
    global t
    r=os.lstat(p)
    if stat.S_ISREG(r.st_mode) and r.st_nlink>1:
        k=(r.st_dev,r.st_ino)
        if k in seen: return
        seen.add(k)
    t+=r.st_blocks*512
    if stat.S_ISDIR(r.st_mode):
        for n in os.listdir(p): w(os.path.join(p,n))
w(sys.argv[1]);print(t)`;

test("a path below the root of a directory partition resolves to the partition and nothing else does",()=>{
 const base=scratch();try{
  const directory=partition(base,"p",true),inside=join(directory,"m","deep","er");mkdirSync(inside,{recursive:true});
  const found=nativePartitionOfV1(inside,"linux")!;
  expect(found.directory).toBe(directory);expect(found.root).toBe(join(directory,"m"));expect(found.attached).toBe(true);expect(found.filesystemBytes).toBe(fixture.envelope.filesystemBytes);expect(found.maximumPhysicalBytes).toBe(fixture.envelope.maximumPhysicalBytes);
  for(const row of fixture.hostPlatforms)expect(nativePartitionOfV1(inside,row.platform),row.platform).toBeUndefined();
  expect(nativePartitionOfV1(join(base,"elsewhere"),"linux")).toBeUndefined();expect(nativePartitionOfV1(directory,"linux")).toBeUndefined();
  deactivateNativeDirectoryV1(directory);expect(nativePartitionOfV1(join(directory,"m"),"linux")!.root).toBe(join(directory,protocol.volume));expect(nativePartitionOfV1(join(directory,"m"),"linux")!.attached).toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("statfs of a partition is the declared envelope minus the measured tree, in both number and bigint form, and agrees with an independent python walk",async()=>{
 const base=scratch();try{
  const directory=partition(base,"p",true),root=join(directory,"m");
  for(const[name,size]of fixture.files)writeFileSync(join(root,String(name)),Buffer.alloc(Number(size),3));
  const python=testPythonV1(),used=python?Number(sh(python,"-c",walk,root).stdout.toString()):measureNativeDirectoryUsageV1(root).allocatedBytes,unit=protocol.usage.blockBytes;
  await nativeDirectoryUsageV1(root,fixture.envelope.maximumPhysicalBytes).fresh();
  const wide=await nativeStatfsV1(root,{bigint:true,platform:"linux"}),narrow=await nativeStatfsV1(join(root,"."),{platform:"linux"}),sync=nativeStatfsSyncV1(root,{bigint:true,platform:"linux"});
  expect(Number(wide.bsize)).toBe(unit);expect(Number(wide.blocks)*unit).toBe(fixture.envelope.filesystemBytes);
  expect((Number(wide.blocks)-Number(wide.bfree))*unit).toBe(Math.ceil(used/unit)*unit);expect(wide.bavail).toBe(wide.bfree);
  expect(typeof narrow.bsize).toBe("number");expect(narrow.bavail).toBe(Number(wide.bavail));expect(sync).toEqual(wide);
  expect(typeof wide.bsize).toBe("bigint");
  writeFileSync(join(root,"more"),Buffer.alloc(1048576,4));await nativeDirectoryUsageV1(root,fixture.envelope.maximumPhysicalBytes).fresh();
  const after=await nativeStatfsV1(root,{bigint:true,platform:"linux"});expect(Number(wide.bavail)-Number(after.bavail)).toBe(1048576/unit);
  const reading=nativeStatfsSyncV1(root,{platform:"linux"});expect(reading.bavail).toBe(Number(after.bavail));
  writeFileSync(join(root,"again"),Buffer.alloc(2097152,5));
  const cached=await nativeStatfsV1(root,{bigint:true,platform:"linux"}),exact=await nativeStatfsV1(root,{bigint:true,platform:"linux",fresh:true});
  expect(cached.bavail,"a young measurement is served as is").toBe(after.bavail);expect(Number(after.bavail)-Number(exact.bavail),"a fresh request sees the new bytes").toBe(2097152/unit);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a young measurement is served as is, an older one is served while one shared walk refreshes it, and only a missing or ancient one blocks on a walk",async()=>{
 const base=scratch();try{
  const directory=partition(base,"p",true),root=join(directory,"m"),max=fixture.envelope.maximumPhysicalBytes;writeFileSync(join(root,"a"),Buffer.alloc(8192,1));
  const start=Date.now(),first=nativeDirectoryUsedBytesSyncV1(root,max,()=>start);
  writeFileSync(join(root,"b"),Buffer.alloc(65536,1));
  expect(nativeDirectoryUsedBytesSyncV1(root,max,()=>start+protocol.usage.recentMilliseconds-1),"young").toBe(first);
  expect(nativeDirectoryUsedBytesSyncV1(root,max,()=>start+protocol.usage.recentMilliseconds+1),"old but servable").toBe(first);
  await nativeDirectoryUsageV1(root,max).fresh();
  const refreshed=nativeDirectoryUsedBytesSyncV1(root,max,()=>Date.now());expect(refreshed,"the background walk replaced it").toBeGreaterThan(first);
  writeFileSync(join(root,"c"),Buffer.alloc(131072,1));
  expect(nativeDirectoryUsedBytesSyncV1(root,max,()=>Date.now()+protocol.usage.synchronousMaximumAgeMilliseconds+1000),"ancient: walked again").toBeGreaterThan(refreshed);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("host paths keep the host answer and Windows reports free space without statfs",async()=>{
 const base=scratch();try{
  const host=statfsSync(base),viaShim=await nativeStatfsV1(base,{platform:"linux"});
  expect(viaShim.bsize).toBe(host.bsize);expect(viaShim.blocks).toBe(host.blocks);
  const windows=await nativeStatfsV1(base,{bigint:true,platform:"win32"});expect(Number(windows.bsize)).toBe(protocol.usage.blockBytes);expect(Number(windows.bavail)).toBeGreaterThan(0);expect(windows.bavail).toBe(windows.bfree);
  expect(nativeStatfsSyncV1(base,{platform:"win32"}).bsize).toBe(protocol.usage.blockBytes);
 }finally{rmSync(base,{recursive:true,force:true});}
});
