import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {activateNativeDirectoryV1,allocateNativeDirectoryV1,deactivateNativeDirectoryV1,nativeDirectoryProtocolV1,nativeDirectoryUsageV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import {readCacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheRunnerPortsV1,reportCacheLedgerV1} from "../../🏃️runner/🟦️.ts";

const protocol=nativeDirectoryProtocolV1(),purpose={en:"p",de:"q"},envelope={filesystemBytes:33554432,maximumPhysicalBytes:34603008};
const ledger=readCacheLedgerV1({version:1,roots:{repository:{directory:"r/cache",maximumBytes:1<<30,undeclaredUnusedAgeMs:1000,undeclaredQuietMs:0,ignored:[]},native:{directory:".cache",maximumBytes:1<<20,undeclaredUnusedAgeMs:1000,undeclaredQuietMs:0,ignored:[]}},areas:[{id:"alpha",root:"native",kind:"partition",directory:"b/a",partition:"alpha",managed:{owner:"image"},writers:["w"],purpose}],retirements:[]},{alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30}});
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
