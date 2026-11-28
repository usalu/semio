/** 🧩️ Contract: a Cargo lane gets an image of its own before its first byte is written when the rule, the platform and the host allow it, an existing image is mounted again after a reboot, a lane whose files live in the home image stays there, a refusal or a failure leaves the lane in the home image, and the image of a lane that is gone is retired whole.
 * @see ../🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{NATIVE_IMAGE_SHARD_V1,appendNativeImageShardEventV1,nativeImageShardFoldV1,nativeImageShardLayoutV1,nativeImageShardPathV1,readNativeImageShardLedgerV1,type NativeImageShardLifecycleV1,type NativeImageShardRuleV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
import{NativeImageWritesRefusedErrorV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪫️relief/🟦️.ts";
import{tryClaimV1}from"../../../../🔒️lock/🟦️.ts";
import{laneMessageV1}from"../../../../🚪️launch/🟦️.ts";
import{UNIT_CACHE_POLICY_V1}from"../../../../🔏️key/🟦️.ts";
import{crateKeyV1,enterLaneV1,lanePathsV1,readLanePeaksV1,recordLanePeakV1,recordLaneSizeV1,removeLaneV1}from"../../../🟦️.ts";
import{ensureLaneImageV1,laneFullVerdictV1,laneIdleSlackBytesV1,laneImageDirectoryV1,laneImagePathV1,laneMeasuredNeedBytesV1,laneSizesV1,repairLaneImagesV1,repairRefusingShardsV1,requireShardsWritableV1,retireClampedLaneImagesV1,retireIdleLaneImagesV1,type LaneImagePortsV1,type LaneImageWorldV1}from"../🟦️.ts";

const GiB=1073741824,scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const rule:NativeImageShardRuleV1={enabled:true,headroomPercent:100,hostFreeReservePercent:66,roots:[{prefix:"u/l",pattern:"^[a-z0-9]{1,12}$",needBytes:8*GiB}]};
function world(hostFreeGiB:number,options:Readonly<{enabled?:boolean}>={}){
 const base=mkdtempSync(join(tmpdir(),"lane-image-"));scratch.push(base);
 const directory=join(base,".cache","b","r"),root=join(directory,"m","u"),calls:string[]=[];
 mkdirSync(join(root,"l"),{recursive:true});mkdirSync(join(root,UNIT_CACHE_POLICY_V1.layout.locks),{recursive:true});writeFileSync(join(directory,"fixed.dmg"),"");
 const mountedPaths=new Set<string>([join(directory,"m")]);
 const lifecycle:NativeImageShardLifecycleV1={
  async create(layout){calls.push("create:"+layout.id);},
  async attach(layout){calls.push("attach:"+layout.id);mountedPaths.add(layout.mountPoint);},
  async detach(layout){calls.push("detach:"+layout.id);mountedPaths.delete(layout.mountPoint);},
  async retire(layout){calls.push("retire:"+layout.id);mountedPaths.delete(layout.mountPoint);},
  async resize(layout,bytes){calls.push("grow:"+layout.id+":"+bytes/GiB);}
 };
 let nowMs=1_000;
 const ports:LaneImagePortsV1={directory,rule:{...rule,enabled:options.enabled??true},platform:"darwin",hostFreeBytes:()=>hostFreeGiB*GiB,now:()=>++nowMs,lifecycle,lock:async body=>body()};
 const files=new Map<string,string[]>(),existing=new Set<string>();
 const fake:LaneImageWorldV1={exists:path=>existing.has(path),entries:path=>files.get(path)??[],mounted:path=>mountedPaths.has(path)};
 return{directory,root,calls,ports,fake,files,existing,mountedPaths};
}

test("a lane without files gets an image of its own, born once, logged at its path and aligned within the cap",async()=>{
 const w=world(100);
 const made=await ensureLaneImageV1(w.root,"q90be4b",w.ports,w.fake);
 expect(made.kind).toBe("birth");
 expect(w.calls).toEqual(["detach:q90be4b","create:q90be4b","attach:q90be4b"]);
 const live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)),record=live.get("q90be4b")!;
 expect(record.path).toBe(laneImagePathV1("q90be4b"));
 expect(record.path).toBe("u/l/q90be4b");
 expect(record.capacityBytes%GiB).toBe(0);
 expect(record.capacityBytes).toBeLessThanOrEqual(NATIVE_IMAGE_SHARD_V1.maximumImageBytes.darwin);
 expect(record.capacityBytes).toBeGreaterThanOrEqual(16*GiB);
 const script=`
import json,sys
rows=[json.loads(line) for line in open(sys.argv[1]) if line.strip()]
born=[row for row in rows if row["type"]=="born"]
print(json.dumps([[row["id"],row["path"],row["capacityBytes"]%(1<<30)==0,row["capacityBytes"]<=int(sys.argv[2])] for row in born]))`;
 const run=spawnSync("python3",["-I","-c",script,join(w.directory,NATIVE_IMAGE_SHARD_V1.ledgerFile),String(NATIVE_IMAGE_SHARD_V1.maximumImageBytes.darwin)],{encoding:"utf8"});
 expect(run.status).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual([["q90be4b","u/l/q90be4b",true,true]]);
 const again=await ensureLaneImageV1(w.root,"q90be4b",w.ports,w.fake);
 expect(again.kind).toBe("existing");
 expect(w.calls.filter(call=>call.startsWith("create"))).toHaveLength(1);
});

test("a lane whose files already live in the home image stays there, nothing is mounted over them",async()=>{
 const w=world(100),base=lanePathsV1(w.root,"p1a2b3c").base;
 w.existing.add(base);w.files.set(base,["target","live"]);
 expect((await ensureLaneImageV1(w.root,"p1a2b3c",w.ports,w.fake)).kind).toBe("home");
 expect(w.calls).toEqual([]);
 expect(readNativeImageShardLedgerV1(w.directory)).toEqual([]);
});

test("a host that cannot hold the image, a rule that is off, a home image that is not mounted and a failing lifecycle all leave the lane in the home image",async()=>{
 const tight=world(15);
 const refused=await ensureLaneImageV1(tight.root,"p000001",tight.ports,tight.fake);
 expect(refused).toEqual({kind:"refused",id:"p000001",detail:"host-free"});
 expect(tight.calls).toEqual([]);
 expect((await ensureLaneImageV1(tight.root,"p000001",undefined,tight.fake)).kind).toBe("off");
 const off=world(100,{enabled:false});
 expect((await ensureLaneImageV1(off.root,"p000001",off.ports,off.fake)).kind).toBe("off");
 const unmounted=world(100);unmounted.mountedPaths.clear();
 expect((await ensureLaneImageV1(unmounted.root,"p000001",unmounted.ports,unmounted.fake)).kind).toBe("home");
 const broken=world(100);
 const failing:LaneImagePortsV1={...broken.ports,lifecycle:{...broken.ports.lifecycle,async create(){throw Error("hdiutil: create failed");}}};
 const failed=await ensureLaneImageV1(broken.root,"p000002",failing,broken.fake);
 expect(failed.kind).toBe("failed");
 expect(failed.detail).toContain("create failed");
});

test("after a reboot every live image is mounted again and an image without files is retired",async()=>{
 const w=world(100);
 await ensureLaneImageV1(w.root,"p000001",w.ports,w.fake);await ensureLaneImageV1(w.root,"p000002",w.ports,w.fake);
 w.mountedPaths.clear();w.mountedPaths.add(join(w.directory,"m"));w.calls.length=0;
 const kept=nativeImageShardLayoutV1(w.directory,{id:"p000001",key:"p000001",path:"u/l/p000001"});
 mkdirSync(join(kept.image,".."),{recursive:true});writeFileSync(kept.image,"");
 const done=await repairLaneImagesV1(w.ports,w.fake);
 expect(done).toEqual({p000001:"mounted",p000002:"retired"});
 expect(w.calls).toEqual(["attach:p000001","retire:p000002"]);
 expect([...nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)).keys()]).toEqual(["p000001"]);
});

test("the image of a lane that is gone is retired whole, a claimed lane and a lane with a record keep theirs",async()=>{
 const w=world(100);
 for(const id of["p000001","p000002","p000003"])await ensureLaneImageV1(w.root,id,w.ports,w.fake);
 w.calls.length=0;
 const keep=lanePathsV1(w.root,"p000002"),claimed=lanePathsV1(w.root,"p000003");
 w.existing.add(keep.base);w.existing.add(keep.metadata);
 w.existing.add(claimed.base);
 const claim=tryClaimV1(join(w.root,UNIT_CACHE_POLICY_V1.layout.locks),"lane-p000003")!;
 try{
  const retired=await retireIdleLaneImagesV1(w.root,w.ports,5_000,w.fake);
  expect(retired).toEqual(["p000001"]);
 }finally{claim.release();}
 expect(w.calls).toEqual(["retire:p000001"]);
 expect([...nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)).keys()].sort()).toEqual(["p000002","p000003"]);
 const layout=nativeImageShardLayoutV1(w.directory,{id:"p000002",key:"p000002",path:nativeImageShardPathV1(rule,0,"p000002")});
 expect(layout.mountPoint).toBe(join(w.directory,"m","u","l","p000002"));
});

test("the reserved default lane is never retired as idle, its image stays while an idle ordinary lane goes",async()=>{
 const w=world(100);
 for(const id of[...UNIT_CACHE_POLICY_V1.pool.reserved,"p000001"])expect((await ensureLaneImageV1(w.root,id,w.ports,w.fake)).kind).toBe("birth");
 w.calls.length=0;
 expect(await retireIdleLaneImagesV1(w.root,w.ports,5_000,w.fake)).toEqual(["p000001"]);
 expect(w.calls).toEqual(["retire:p000001"]);
 expect([...nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)).keys()]).toEqual(["d"]);
});

test("the partition directory of a cache root is the one that holds the home image, nothing else qualifies",()=>{
 const w=world(100);
 expect(laneImageDirectoryV1(w.root)).toBe(w.directory);
 expect(laneImageDirectoryV1(join(w.directory,"m"))).toBeUndefined();
 expect(laneImageDirectoryV1(join(w.directory,"x","u"))).toBeUndefined();
 expect(laneImageDirectoryV1(join(tmpdir(),"nowhere","m","u"))).toBeUndefined();
});

test("a lane image is born as large as the measured need asks, within the floor of the root and the cap, and the measure is the largest lane recorded",async()=>{
 const w=world(400);
 expect(laneMeasuredNeedBytesV1(w.root)).toBeUndefined();
 mkdirSync(lanePathsV1(w.root,"p000009").base,{recursive:true});mkdirSync(lanePathsV1(w.root,"p00000a").base,{recursive:true});
 recordLaneSizeV1(w.root,"p000009",3*GiB,0,1);recordLaneSizeV1(w.root,"p00000a",20*GiB,0,1);
 expect(laneMeasuredNeedBytesV1(w.root)).toBe(20*GiB);
 const ids=["p000001","p000002","p000003","p000004"],measured=[undefined,3*GiB,10*GiB,60*GiB];
 for(const[index,id]of ids.entries())await ensureLaneImageV1(w.root,id,{...w.ports,measuredNeedBytes:()=>measured[index]},w.fake);
 const live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory));
 expect(ids.map(id=>live.get(id)!.capacityBytes/GiB)).toEqual([16,16,20,32]);
});

test("an existing lane image whose need outgrew it is stretched before its first byte, a failing stretch leaves the lane on its image",async()=>{
 const w=world(400);
 await ensureLaneImageV1(w.root,"p000001",w.ports,w.fake);w.calls.length=0;
 const full:LaneImagePortsV1={...w.ports,measure:()=>({usedBytes:15*GiB,hostFreeBytes:400*GiB})},idle:LaneImagePortsV1={...w.ports,measure:()=>({usedBytes:GiB,hostFreeBytes:400*GiB})};
 expect(await ensureLaneImageV1(w.root,"p000001",idle,w.fake)).toEqual({kind:"existing",id:"p000001"});expect(w.calls).toEqual(["attach:p000001"]);
 w.calls.length=0;
 expect(await ensureLaneImageV1(w.root,"p000001",full,w.fake)).toEqual({kind:"grown",id:"p000001",detail:String(30*GiB)});
 expect(w.calls).toEqual(["attach:p000001","grow:p000001:30"]);
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)).get("p000001")!.capacityBytes).toBe(30*GiB);
 const second=world(400);
 await ensureLaneImageV1(second.root,"p000001",second.ports,second.fake);
 const broken:LaneImagePortsV1={...second.ports,measure:full.measure!,lifecycle:{...second.ports.lifecycle,async resize(){throw Error("hdiutil: resize: busy");}}};
 const refused=await ensureLaneImageV1(second.root,"p000001",broken,second.fake);
 expect(refused.kind).toBe("existing");expect(refused.detail).toContain("busy");
});

test("a lane image is sized from the measured lanes: a host with less room than the declared need still births one, a roomy host keeps the preferred size, an existing image grows by its plan and a holder makes it wait",async()=>{
 const samples=[5*GiB,6*GiB,6*GiB,7*GiB];
 const poor=world(21);
 expect(await ensureLaneImageV1(poor.root,"p000001",poor.ports,poor.fake)).toEqual({kind:"refused",id:"p000001",detail:"host-free"});
 expect(await ensureLaneImageV1(poor.root,"p000002",{...poor.ports,laneSizes:()=>samples},poor.fake)).toMatchObject({kind:"birth",id:"p000002"});
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(poor.directory)).get("p000002")!.capacityBytes).toBe(13*GiB);
 const roomy=world(400);
 await ensureLaneImageV1(roomy.root,"p000001",{...roomy.ports,laneSizes:()=>samples},roomy.fake);
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(roomy.directory)).get("p000001")!.capacityBytes).toBe(16*GiB);
 expect(laneSizesV1(roomy.root)).toEqual([]);
 const full:LaneImagePortsV1={...roomy.ports,measure:()=>({usedBytes:15*GiB,hostFreeBytes:400*GiB})};
 roomy.calls.length=0;
 expect(await ensureLaneImageV1(roomy.root,"p000001",full,roomy.fake)).toEqual({kind:"grown",id:"p000001",detail:String(30*GiB)});
 const tight:LaneImagePortsV1={...full,measure:()=>({usedBytes:28*GiB,hostFreeBytes:4*GiB})};
 expect(await ensureLaneImageV1(roomy.root,"p000001",tight,roomy.fake)).toEqual({kind:"existing",id:"p000001"});
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(roomy.directory)).get("p000001")!.capacityBytes).toBe(30*GiB);
});

const evidenceOwner={session:"s.1",agent:"a"};
const withEvidence=(w:ReturnType<typeof world>,key:string):LaneImagePortsV1=>({...w.ports,laneSizes:()=>laneSizesV1(w.root,key),measuredNeedBytes:()=>laneMeasuredNeedBytesV1(w.root,key)});

test("the peak a crate set reached outlives the lane that reached it, only ever rises and is the evidence the next lane of that crate set is sized from",()=>{
 const w=world(100),key=crateKeyV1([]);
 enterLaneV1(w.root,"pold0001",evidenceOwner,[]).release();
 recordLaneSizeV1(w.root,"pold0001",20*GiB,0,1000);
 expect(readLanePeaksV1(w.root)).toEqual({[key]:20*GiB});
 recordLaneSizeV1(w.root,"pold0001",5*GiB,0,2000);recordLanePeakV1(w.root,key,7*GiB);
 expect(readLanePeaksV1(w.root)[key]).toBe(20*GiB);
 expect(removeLaneV1(w.root,"pold0001","lane",3000)).toBe(true);
 expect(laneSizesV1(w.root,key)).toEqual([20*GiB]);expect(laneMeasuredNeedBytesV1(w.root,key)).toBe(20*GiB);
 expect(laneSizesV1(w.root,crateKeyV1(["other"]))).toEqual([]);expect(laneMeasuredNeedBytesV1(w.root,crateKeyV1(["other"]))).toBeUndefined();
 recordLanePeakV1(w.root,key,-1);recordLanePeakV1(w.root,key,Number.NaN);expect(readLanePeaksV1(w.root)[key]).toBe(20*GiB);
 writeFileSync(join(w.root,UNIT_CACHE_POLICY_V1.layout.peaks),"{broken");expect(readLanePeaksV1(w.root)).toEqual({});
});

test("a lane image is born as large as its crate set has needed, not at the flat declared size, and a crate set without evidence keeps the declared size",async()=>{
 const flat=world(100);
 expect((await ensureLaneImageV1(flat.root,"pflat001",flat.ports,flat.fake)).kind).toBe("birth");
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(flat.directory)).get("pflat001")!.capacityBytes).toBe(16*GiB);
 const w=world(100),key=crateKeyV1([]);
 recordLanePeakV1(w.root,key,14*GiB);
 expect((await ensureLaneImageV1(w.root,"pnext001",withEvidence(w,key),w.fake)).kind).toBe("birth");
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)).get("pnext001")!.capacityBytes).toBe(28*GiB);
 const big=world(100);recordLanePeakV1(big.root,key,40*GiB);
 await ensureLaneImageV1(big.root,"pbig0001",withEvidence(big,key),big.fake);
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(big.directory)).get("pbig0001")!.capacityBytes).toBe(NATIVE_IMAGE_SHARD_V1.maximumImageBytes.darwin);
});

test("idle lane images count what they hold against the commitment budget and a lane a process holds counts in full, so dead sessions' sparse images do not refuse a live lane",async()=>{
 const w=world(100),held=enterLaneV1(w.root,"phold001",evidenceOwner,["a"]);
 try{
  enterLaneV1(w.root,"pidle001",evidenceOwner,["b"]).release();enterLaneV1(w.root,"pidle002",evidenceOwner,["c"]).release();
  for(const id of["phold001","pidle001","pidle002"])appendNativeImageShardEventV1(w.directory,{type:"born",id,key:id,root:0,path:"u/l/"+id,capacityBytes:16*GiB,atMs:1});
  const measure=()=>({usedBytes:3*GiB});
  expect(laneIdleSlackBytesV1(w.root,w.directory,Date.now(),measure)).toBe(26*GiB);
  recordLaneSizeV1(w.root,"pidle001",9*GiB,0,1);
  expect(laneIdleSlackBytesV1(w.root,w.directory,Date.now(),measure)).toBe(20*GiB);
  const budget=45*GiB;
  expect((await ensureLaneImageV1(w.root,"pnew0001",{...w.ports,budgetBytes:()=>budget},w.fake)).kind).toBe("refused");
  expect((await ensureLaneImageV1(w.root,"pnew0001",{...w.ports,budgetBytes:()=>budget+laneIdleSlackBytesV1(w.root,w.directory,Date.now(),measure)},w.fake)).kind).toBe("birth");
 }finally{held.release();}
});

test("a full lane image either grows past what its crate set reached or is refused for a named reason, never left to restart empty",()=>{
 const facts={capacityBytes:16*GiB,usedBytes:16*GiB,peakBytes:16*GiB,hostFreeBytes:60*GiB};
 expect(laneFullVerdictV1("darwin",rule,facts)).toEqual({kind:"grows",targetBytes:32*GiB});
 expect(laneFullVerdictV1("darwin",rule,{...facts,budgetRoomBytes:4*GiB})).toEqual({kind:"grows",targetBytes:20*GiB});
 expect(laneFullVerdictV1("darwin",rule,{...facts,budgetRoomBytes:0})).toEqual({kind:"capped",reason:"budget"});
 expect(laneFullVerdictV1("darwin",rule,{...facts,hostFreeBytes:2*GiB})).toEqual({kind:"capped",reason:"host-short"});
 expect(laneFullVerdictV1("darwin",rule,{...facts,capacityBytes:32*GiB,usedBytes:32*GiB,peakBytes:32*GiB})).toEqual({kind:"capped",reason:"at-cap"});
 expect(laneFullVerdictV1("freebsd",rule,facts)).toEqual({kind:"capped",reason:"unsupported"});
});

test("the typed refusal of a full lane is told in both languages and names the lane, the sizes and the way out",()=>{
 const parameters={lane:"p7c9473",usedGib:"16.0",capacityGib:"16.0",targetGib:"32.0",reason:"budget"};
 for(const language of["en","de"]as const)for(const kind of["laneFullGrows","laneFullCapped"]as const){const text=laneMessageV1(kind,parameters,language);expect(text).toContain("p7c9473");expect(text).toContain("16.0");expect(text).not.toContain("{");}
 expect(laneMessageV1("laneFullGrows",parameters,"en")).toContain("32.0");expect(laneMessageV1("laneFullCapped",parameters,"de")).toContain("budget");
});

test("a lane image the host clamps is re-created at its right size when no build holds it, a healthy or a busy one stays",async()=>{
 const w=world(100);
 await ensureLaneImageV1(w.root,"qclamp1",w.ports,w.fake);await ensureLaneImageV1(w.root,"qfine01",w.ports,w.fake);
 const record=(id:string)=>nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)).get(id)!;
 const big=record("qclamp1").capacityBytes,measured={...w.ports,rule:{...rule,roots:[{...rule.roots[0]!,needBytes:2*GiB}]},measure:()=>({usedBytes:GiB,hostFreeBytes:100*GiB})},available=new Map<string,number>();
 for(const id of["qclamp1","qfine01"])available.set(nativeImageShardLayoutV1(w.directory,record(id)).mountPoint,id==="qclamp1"?GiB/2:big-GiB);
 const clamped={...w.fake,available:(path:string)=>available.get(path)};
 expect(big).toBeGreaterThan(8*GiB);w.calls.length=0;
 const retired=await retireClampedLaneImagesV1(w.root,measured,1_000_000,clamped);
 expect(retired).toEqual(["qclamp1"]);expect(w.calls).toEqual(["retire:qclamp1"]);expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)).has("qfine01")).toBe(true);
 expect(await retireClampedLaneImagesV1(w.root,{...measured,measure:undefined},1_000_000,clamped)).toEqual([]);
});

test("a shard the host clamps is shrunk in place before a build and takes writes again, a held clamped one ends in the typed bilingual refusal, healthy and really full ones pass",async()=>{
 const w=world(100),roots=[...rule.roots,{prefix:"u",pattern:"^(sealed|w)$",needBytes:4*GiB}],layoutOf=(id:string,root:number,path:string,capacityBytes:number)=>{appendNativeImageShardEventV1(w.directory,{type:"born",id,key:id,root,path,capacityBytes,atMs:1});const mount=nativeImageShardLayoutV1(w.directory,{id,key:id,path}).mountPoint;w.mountedPaths.add(mount);return mount;};
 const mounts={w:layoutOf("w",1,"u/w",30*GiB),sealed:layoutOf("sealed",1,"u/sealed",26*GiB),lane:layoutOf("qfull001",0,"u/l/qfull001",4*GiB),fine:layoutOf("qfine001",0,"u/l/qfine001",16*GiB)};
 const free=new Map<string,number>([[mounts.w,0],[mounts.sealed,0],[mounts.lane,0],[mounts.fine,12*GiB]]),used=new Map<string,number>([["w",3*GiB],["sealed",GiB],["qfull001",4*GiB],["qfine001",4*GiB]]),held=new Set<string>(["sealed"]),log:string[]=[];
 const full=()=>Object.assign(Error("ENOSPC: no space left on device"),{code:"ENOSPC"});
 const io={available:(mount:string)=>free.get(mount)!,write:(file:string)=>{const mount=[...free.keys()].find(path=>file.startsWith(path+"/"))!;if(free.get(mount)===0)throw full();},remove:()=>{}};
 const ports:LaneImagePortsV1={...w.ports,rule:{...rule,roots},measure:layout=>({usedBytes:used.get(layout.id)!,hostFreeBytes:24*GiB}),holders:async layout=>held.has(layout.id)?1:0,lifecycle:{...w.ports.lifecycle,async resize(layout,bytes){log.push("resize:"+layout.id+":"+bytes/GiB);free.set(layout.mountPoint,bytes-used.get(layout.id)!);}}};
 const rows=await repairRefusingShardsV1(w.root,ports,{...w.fake,io});
 expect(rows.map(row=>[row.id,row.state])).toEqual([["w","shrunk"],["sealed","refuses"],["qfull001","writable"],["qfine001","writable"]]);
 expect(rows[0]!.shrunk).toEqual({fromBytes:30*GiB,toBytes:8*GiB,usedBytes:3*GiB});expect(log).toEqual(["resize:w:8"]);
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)).get("w")!.capacityBytes).toBe(8*GiB);expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(w.directory)).get("sealed")!.capacityBytes).toBe(26*GiB);
 expect(rows[1]!.facts).toMatchObject({image:"u/sealed",reason:"ENOSPC",capacityBytes:26*GiB,availableBytes:0,innerFreeBytes:25*GiB,hostFreeBytes:100*GiB,targetBytes:8*GiB});
 const lines:string[]=[],error=await requireShardsWritableV1(w.root,{language:"de",report:line=>lines.push(line),ports,world:{...w.fake,io}}).catch(thrown=>thrown);
 expect(error).toBeInstanceOf(NativeImageWritesRefusedErrorV1);expect(error.code).toBe("ENOSPC");expect(error.message).toContain("The cache image u/sealed refuses writes before the build");expect(error.message).toContain("lehnt Schreibzugriffe vor dem Build ab");expect(lines).toEqual([]);
 held.clear();
 const done=await requireShardsWritableV1(w.root,{language:"en",report:line=>lines.push(line),ports,world:{...w.fake,io}});
 expect(done.map(row=>row.state)).toEqual(["writable","shrunk","writable","writable"]);expect(lines).toHaveLength(1);expect(lines[0]).toContain("cache image sealed was shrunk in place from 26.0 GiB to 8.0 GiB");expect(log).toEqual(["resize:w:8","resize:sealed:8"]);
 expect(await repairRefusingShardsV1(w.root,undefined,{...w.fake,io})).toEqual([]);expect(await repairRefusingShardsV1(w.root,{...ports,rule:{...ports.rule,enabled:false}},{...w.fake,io})).toEqual([]);
});
