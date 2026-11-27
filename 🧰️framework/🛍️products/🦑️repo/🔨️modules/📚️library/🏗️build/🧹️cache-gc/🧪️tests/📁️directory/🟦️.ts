import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {activateNativeDirectoryV1,allocateNativeDirectoryV1,deactivateNativeDirectoryV1,nativeDirectoryProtocolV1,nativeDirectoryUsageV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import {readCacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheRunnerPortsV1,mountedPartitionUsageAsyncV1,mountedPartitionUsageV1,reportCacheLedgerV1} from "../../🏃️runner/🟦️.ts";
import {nativeImageVolumeUsedBytesSyncV1,nativeImageVolumeUsedBytesV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/⚖️availability/🟦️.ts";
import {planMachineBudgetV1,type MachinePolicyV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts";
import usage from "./usage.json" with {type:"json"};

const protocol=nativeDirectoryProtocolV1(),purpose={en:"p",de:"q"},envelope={filesystemBytes:33554432,maximumPhysicalBytes:34603008};
const ledger=readCacheLedgerV1({version:1,roots:{repository:{directory:"r/cache",maximumBytes:1<<30,undeclaredUnusedAgeMs:1000,undeclaredQuietMs:0,ignored:[]},native:{directory:".cache",maximumBytes:1<<20,undeclaredUnusedAgeMs:1000,undeclaredQuietMs:0,ignored:[]},rust:{directory:".r",maximumBytes:0,undeclaredUnusedAgeMs:1000,undeclaredQuietMs:0,ignored:[]}},areas:[{id:"alpha",root:"native",kind:"partition",directory:"b/a",partition:"alpha",managed:{owner:"image"},writers:["w"],purpose}],retirements:[]},{alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30}});
const ports=(platform:NodeJS.Platform)=>cacheRunnerPortsV1({signal:new AbortController().signal,progress:async()=>undefined,exclusive:async()=>({release:()=>undefined}),platform});
const status=async(workspace:string,platform:NodeJS.Platform)=>(await reportCacheLedgerV1(workspace,ledger,ports(platform),{alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30}})).partitions.find(row=>row.id==="alpha")!;

test("a directory partition reports attached, detached or plain from its anchor, with the declared envelope as capacity and the measured tree as use",async()=>{
 const base=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"gcd-"));try{
  const directory=join(base,".cache/b/a"),root=join(directory,"m");mkdirSync(root,{recursive:true});
  expect((await status(base,"linux")).state,"no anchor: a plain directory").toBe("directory");
  allocateNativeDirectoryV1({directory,...envelope},()=>true);
  const detached=await status(base,"linux");expect(detached.state).toBe("detached");expect(detached.usedBytes).toBeNull();expect(detached.capacityBytes).toBeNull();
  activateNativeDirectoryV1(directory);writeFileSync(join(root,"a.bin"),Buffer.alloc(1048576,1));await nativeDirectoryUsageV1(root,envelope.maximumPhysicalBytes).fresh();
  const mounted=await status(base,"win32");expect(mounted.state).toBe("mounted");expect(mounted.capacityBytes).toBe(envelope.filesystemBytes);expect(mounted.usedBytes).toBeGreaterThanOrEqual(1048576);expect(mounted.usedBytes!).toBeLessThan(1048576+65536);
  deactivateNativeDirectoryV1(directory);expect((await status(base,"linux")).state).toBe("detached");
  expect(protocol.anchor).toBe("fixed.dir");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the same partition tree is read by the macOS rules as a plain directory because no image file sits beside it",async()=>{
 const base=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"gcd-"));try{
  const directory=join(base,".cache/b/a");mkdirSync(join(directory,"m"),{recursive:true});writeFileSync(join(directory,"fixed.dmg"),"");
  expect((await status(base,"darwin")).state).toBe("detached");
  rmSync(join(directory,"fixed.dmg"));expect((await status(base,"darwin")).state).toBe("directory");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("host-limited Darwin image availability does not become occupied cache bytes",()=>{
 const {mount,volume,plist,expected,quotaBytes}=usage;
 const read=(observedMount:string)=>`<plist version="1.0"><dict><key>MountPoint</key><string>${observedMount}</string><key>FilesystemType</key><string>${plist.FilesystemType}</string><key>CapacityInUse</key><integer>${plist.CapacityInUse}</integer></dict></plist>`;
 const facts=mountedPartitionUsageV1(mount,volume,"darwin",(path,row,platform)=>nativeImageVolumeUsedBytesSyncV1(path,row,platform,()=>read(plist.MountPoint)));
 expect(facts).toEqual({usedBytes:expected.usedBytes,capacityBytes:expected.capacityBytes,partial:false});
 expect(volume.bavail*volume.bsize).toBe(expected.availableBytes);
 const unknown=mountedPartitionUsageV1(mount,volume,"darwin",(path,row,platform)=>nativeImageVolumeUsedBytesSyncV1(path,row,platform,()=>read(mount+"-foreign")));
 expect(unknown).toEqual({usedBytes:null,capacityBytes:expected.capacityBytes,partial:true});

 const policy:MachinePolicyV1={version:1,defaultTotalBytes:quotaBytes,environment:"TEST_BUDGET",file:"budget.json",hostFreeFloorBytes:0,alignBytes:1024**2,toleranceBytes:0,stores:[{id:"rust",partition:"rust",weightBasisPoints:10000,floorBytes:0,headroomBasisPoints:0,purpose}],dedupe:[]};
 const plan=(usedBytes:number)=>planMachineBudgetV1({totalBytes:quotaBytes,hostFreeBytes:expected.availableBytes,uses:[{id:"rust",usedBytes,envelopeBytes:facts.capacityBytes,evictableBytes:null}]},policy);
 expect(plan(facts.usedBytes!).evictBytes).toBe(0);
 expect(plan(facts.capacityBytes-expected.availableBytes).evictBytes).toBeGreaterThan(0);
});

test("the volume tool runs without blocking the loop, agrees with the blocking reading and a hung tool is cut short as an unknown use",async()=>{
 const {mount,volume,plist,expected}=usage;
 const read=`<plist version="1.0"><dict><key>MountPoint</key><string>${plist.MountPoint}</string><key>FilesystemType</key><string>${plist.FilesystemType}</string><key>CapacityInUse</key><integer>${plist.CapacityInUse}</integer></dict></plist>`;
 const blocking=nativeImageVolumeUsedBytesSyncV1(mount,volume,"darwin",()=>read);
 const asynchronous=await nativeImageVolumeUsedBytesV1(mount,volume,"darwin",async()=>read);
 expect(asynchronous).toBe(blocking);expect(asynchronous).toBe(expected.usedBytes);
 const hung=async(_program:string,_args:readonly string[],signal?:AbortSignal)=>new Promise<string|undefined>(resolve=>signal?.addEventListener("abort",()=>resolve(undefined),{once:true}));
 const stop=new AbortController(),pending=nativeImageVolumeUsedBytesV1(mount,volume,"darwin",hung,stop.signal);
 let beat=0;const timer=setInterval(()=>beat++,5);await new Promise(done=>setTimeout(done,40));clearInterval(timer);
 expect(beat,"the loop kept running while the tool hung").toBeGreaterThan(2);
 stop.abort();expect(await pending).toBeUndefined();
 expect(await mountedPartitionUsageAsyncV1(mount,volume,"darwin",AbortSignal.abort())).toEqual({usedBytes:null,capacityBytes:expected.capacityBytes,partial:true});
});
