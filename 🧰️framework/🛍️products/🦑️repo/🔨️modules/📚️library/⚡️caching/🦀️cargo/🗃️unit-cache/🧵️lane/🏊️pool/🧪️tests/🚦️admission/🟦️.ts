/** 🚦️ Contract: a second lane is never started for the same owner, crate set and flavour while a live process holds the first (the acquisition waits for it and reuses it afterwards), idle copies of another lane are shed before any bound is looked at, and a new lane is admitted only when the host volume keeps its free floor and the declared capacities of the lane images stay inside the commitment budget: the least recently used idle lanes are retired whole first, never a lane a live process holds, never more than can help. The rules are written twice, in production and in an independent Python oracle over one corpus, and once more over real lane directories and a recorded shard ledger.
 * @see ../../🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{appendNativeImageShardEventV1,nativeImageShardCommittedBytesV1,nativeImageShardFoldV1,readNativeImageShardLedgerV1,type NativeImageShardRuleV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
import{requireTestToolV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{UNIT_CACHE_POLICY_V1}from"../../../../🔏️key/🟦️.ts";
import{crateKeyV1,enterLaneV1,lanePathsV1,readLaneV1,recordLaneSizeV1}from"../../../🟦️.ts";
import{laneMessageV1}from"../../../../🚪️launch/🟦️.ts";
import type{LaneImagePortsV1}from"../../🧩️image/🟦️.ts";
import{retainRemovedLaneV1}from"../../../📤️retention/🟦️.ts";
import{acquirePooledLaneV1,admitNewLaneV1,removeIdleLaneV1,planLaneAdmissionV1,planPoolAcquireV1,planPoolDuplicatesV1,planPoolTrimV1,poolBoundsV1,poolOwnerKeyV1,readPoolV1,trimPoolV1,type PoolLaneV1}from"../../🟦️.ts";
import corpus from"./🔣️.json";

const base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});
const GIB=corpus.gib,scratch:string[]=[],area=()=>{const path=mkdtempSync(join(base,"ad-"));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const lanes=(rows:readonly Record<string,unknown>[]):PoolLaneV1[]=>rows.map(row=>({...row}) as PoolLaneV1);

test("the production rules and the Python oracle reach the verdict of every corpus case",()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const verdicts=JSON.parse(run.stdout);
 for(const[index,row]of corpus.acquire.entries()){const choice=planPoolAcquireV1(lanes(row.lanes),row.request,row.bounds);expect(choice,row.id+" production").toEqual(row.expect);expect(verdicts.acquire[index],row.id+" oracle").toEqual(row.expect);}
 for(const[index,row]of corpus.duplicates.entries()){expect(planPoolDuplicatesV1(lanes(row.lanes),row.requested),row.id+" production").toEqual(row.expect);expect(verdicts.duplicates[index],row.id+" oracle").toEqual(row.expect);}
 for(const[index,row]of corpus.trim.entries()){expect(planPoolTrimV1(lanes(row.lanes),row.bounds,{requested:row.requested,pressure:row.pressure}),row.id+" production").toEqual(row.expect);expect(verdicts.trim[index],row.id+" oracle").toEqual(row.expect);}
 for(const[index,row]of corpus.admission.entries()){expect(planLaneAdmissionV1(row.candidates,row.shortage),row.id+" production").toEqual(row.expect);expect(verdicts.admission[index],row.id+" oracle").toEqual(row.expect);}
});

const owner={session:"a3fba9d52db0.a2am278jy8",agent:"3efc810a-d82c-45b5-86f3-81ab1123fece"};

test("a second whole-workspace acquisition of the same owner waits for the lane a live process holds and takes it again when it is free, so no copy of the build outputs is ever started",async()=>{
 const root=area(),said:string[]=[];
 try{
  const first=await acquirePooledLaneV1(root,{crates:[],quotaBytes:Number.MAX_SAFE_INTEGER,owner});
  const waiting=new AbortController(),second=acquirePooledLaneV1(root,{crates:[],quotaBytes:Number.MAX_SAFE_INTEGER,owner},{signal:waiting.signal,report:kind=>{said.push(kind);}});
  const outcome=second.then(()=>"acquired",()=>"aborted");
  await new Promise(done=>setTimeout(done,400));waiting.abort();
  expect(await outcome).toBe("aborted");expect(said).toEqual(["poolSameCrates"]);
  expect(readPoolV1(root,Date.now()).map(lane=>lane.id)).toEqual([first.paths.id]);
  first.release();
  const again=await acquirePooledLaneV1(root,{crates:[],quotaBytes:Number.MAX_SAFE_INTEGER,owner});
  expect(again.paths.id).toBe(first.paths.id);expect(again.choice).toBe("warm");again.release();
  const other=await acquirePooledLaneV1(root,{crates:[],quotaBytes:Number.MAX_SAFE_INTEGER,owner:{...owner,agent:"other"}});
  expect(readPoolV1(root,Date.now())).toHaveLength(1);other.release();
  for(const language of["en","de"]as const)expect(laneMessageV1("poolSameCrates",{lane:"p1",holders:"pid 1"},language)).toContain("p1");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("two idle lanes of one owner and crate set never coexist after a trim: the newest stays, a lane of another owner is untouched",async()=>{
 const root=area();
 try{
  const make=(id:string,who:typeof owner,usedMs:number)=>{enterLaneV1(root,id,who,[]).release();const paths=lanePathsV1(root,id),record=readLaneV1(root,id)!;writeFileSync(paths.metadata,JSON.stringify({...record,usedMs}));utimesSync(paths.metadata,new Date(0),new Date(0));return id;};
  const old=make("pa000001",owner,1000),young=make("pa000002",owner,2000),foreign=make("pa000003",{...owner,session:"other.session"},500);
  expect(readPoolV1(root,5000).map(lane=>lane.owner)).toContain(poolOwnerKeyV1(owner));
  const result=trimPoolV1(root,poolBoundsV1(Number.MAX_SAFE_INTEGER),5000);
  expect(result.removed).toBe(1);
  expect(readPoolV1(root,5000).map(lane=>lane.id).sort()).toEqual([young,foreign].sort());
  expect(existsSync(lanePathsV1(root,old).base)).toBe(false);
  expect(trimPoolV1(root,poolBoundsV1(Number.MAX_SAFE_INTEGER),5000).removed).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

const rule:NativeImageShardRuleV1={enabled:true,headroomPercent:100,hostFreeReservePercent:66,roots:[{prefix:"u/l",pattern:"^[a-z0-9]{1,12}$",needBytes:8*GIB}]};
function world(input:Readonly<{hostFreeGiB:number;floorGiB:number;budgetGiB?:number;hostCeilingGiB?:number;foreignGiB?:number}>){
 const directory=area(),root=join(directory,"m","u"),retired:string[]=[],free={bytes:input.hostFreeGiB*GIB},sizes=new Map<string,number>();
 mkdirSync(join(root,"l"),{recursive:true});mkdirSync(join(root,UNIT_CACHE_POLICY_V1.layout.locks),{recursive:true});
 let tick=0;
 const ports:LaneImagePortsV1={directory,rule,platform:"darwin",hostFreeBytes:()=>free.bytes,hostFreeFloorBytes:()=>input.floorGiB*GIB,...input.budgetGiB===undefined?{}:{budgetBytes:()=>input.budgetGiB!*GIB},...input.hostCeilingGiB===undefined?{}:{hostCommitment:()=>({committedBytes:(input.foreignGiB??0)*GIB+nativeImageShardCommittedBytesV1(nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory))),ceilingBytes:input.hostCeilingGiB!*GIB})},now:()=>++tick,lock:async body=>body(),
  lifecycle:{async create(){},async attach(){},async detach(){},async resize(){},async retire(layout){retired.push(layout.id);free.bytes+=(sizes.get(layout.id)??0)*GIB;}}};
 const lane=(id:string,capacityGiB:number,usedMs:number,bytesGiB:number,options:Readonly<{held?:boolean}>={})=>{
  const entered=enterLaneV1(root,id,owner,[id]);if(options.held!==true)entered.release();
  const paths=lanePathsV1(root,id),record=readLaneV1(root,id)!;writeFileSync(paths.metadata,JSON.stringify({...record,usedMs}));utimesSync(paths.metadata,new Date(0),new Date(0));
  recordLaneSizeV1(root,id,bytesGiB*GIB,0,usedMs);sizes.set(id,bytesGiB);
  appendNativeImageShardEventV1(directory,{type:"born",id,key:id,root:0,path:"u/l/"+id,capacityBytes:capacityGiB*GIB,atMs:usedMs});
  return entered;
 };
 const live=()=>[...nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).keys()].sort();
 return{root,ports,lane,live,retired,free};
}

test("a new lane that the commitment budget has no room for retires the least recently used idle lane whole, never a lane a live process holds, and stops as soon as the image fits",async()=>{
 const w=world({hostFreeGiB:100,floorGiB:13,budgetGiB:50});
 const held=w.lane("pa000001",16,200,3,{held:true});
 try{
  w.lane("pa000002",30,100,3);w.lane("pa000003",16,300,3);
  const said:Record<string,unknown>[]=[],retired=await admitNewLaneV1(w.root,"pnew0001",w.ports,Date.now,{report:(kind,parameters)=>{if(kind==="poolAdmitted")said.push(parameters);}});
  expect(retired).toEqual(["pa000002"]);expect(w.retired).toEqual(["pa000002"]);expect(w.live()).toEqual(["pa000001","pa000003"]);
  expect(existsSync(lanePathsV1(w.root,"pa000002").base)).toBe(false);expect(existsSync(lanePathsV1(w.root,"pa000001").base)).toBe(true);
  expect(said).toHaveLength(1);expect(said[0]).toMatchObject({count:1,lanes:"pa000002"});
  for(const language of["en","de"]as const)expect(laneMessageV1("poolAdmitted",said[0] as Record<string,string|number>,language)).toContain("pa000002");
  expect(await admitNewLaneV1(w.root,"pnew0002",w.ports,Date.now)).toEqual([]);
 }finally{held.release();}
});

test("the admission of a new lane never retires the idle lane of the same owner and crate set that the requester takes again, it retires the next least recently used lane instead",async()=>{
 const w=world({hostFreeGiB:100,floorGiB:13,budgetGiB:50});
 w.lane("pa000001",16,100,3);w.lane("pa000002",16,200,3);w.lane("pa000003",16,300,3);
 const requester={owner:poolOwnerKeyV1(owner),crateKey:crateKeyV1(["pa000001"])};
 expect(await admitNewLaneV1(w.root,"pnew0001",w.ports,Date.now,{requester})).toEqual(["pa000002"]);
 expect(w.live()).toEqual(["pa000001","pa000003"]);expect(existsSync(lanePathsV1(w.root,"pa000001").base)).toBe(true);
 const other=world({hostFreeGiB:100,floorGiB:13,budgetGiB:50});
 other.lane("pa000001",16,100,3);other.lane("pa000002",16,200,3);other.lane("pa000003",16,300,3);
 expect(await admitNewLaneV1(other.root,"pnew0001",other.ports,Date.now,{requester:{...requester,owner:"other|agent"}})).toEqual(["pa000001"]);
});

test("a new lane that the host-wide commitment ceiling has no room for retires the least recently used idle lane whole before the birth is refused, however much the images of other roots declare",async()=>{
 const w=world({hostFreeGiB:100,floorGiB:13,budgetGiB:500,hostCeilingGiB:100,foreignGiB:50});
 w.lane("pa000001",16,100,3);w.lane("pa000002",16,200,3);w.lane("pa000003",16,300,3);
 expect(await admitNewLaneV1(w.root,"pnew0001",w.ports,Date.now)).toEqual(["pa000001"]);expect(w.live()).toEqual(["pa000002","pa000003"]);
 expect(await admitNewLaneV1(w.root,"pnew0002",w.ports,Date.now)).toEqual([]);
 const full=world({hostFreeGiB:100,floorGiB:13,budgetGiB:500,hostCeilingGiB:100,foreignGiB:98});
 full.lane("pa000001",16,100,3);full.lane("pa000002",16,200,3);
 expect(await admitNewLaneV1(full.root,"pnew0001",full.ports,Date.now)).toEqual([]);expect(full.live()).toEqual(["pa000001","pa000002"]);
});

test("a host with less free space than its floor plus one lane retires idle lanes one by one, measuring the host after each, and leaves a host that has room alone",async()=>{
 const w=world({hostFreeGiB:10,floorGiB:13,budgetGiB:500});
 w.lane("pa000001",16,100,6);w.lane("pa000002",16,200,6);w.lane("pa000003",16,300,6);
 expect(await admitNewLaneV1(w.root,"pnew0001",w.ports,Date.now)).toEqual(["pa000001","pa000002"]);
 expect(w.free.bytes).toBe(22*GIB);expect(w.live()).toEqual(["pa000003"]);
 expect(await admitNewLaneV1(w.root,"pnew0002",w.ports,Date.now)).toEqual([]);
});

test("a shortage that every idle lane together cannot cover removes nothing, and a disabled rule or a missing image admits without looking",async()=>{
 const w=world({hostFreeGiB:1,floorGiB:13,budgetGiB:500});
 w.lane("pa000001",16,100,2);w.lane("pa000002",16,200,2);
 expect(await admitNewLaneV1(w.root,"pnew0001",w.ports,Date.now)).toEqual([]);expect(w.live()).toEqual(["pa000001","pa000002"]);
 expect(await admitNewLaneV1(w.root,"pnew0001",{...w.ports,rule:{...rule,enabled:false}},Date.now)).toEqual([]);
 expect(await admitNewLaneV1(w.root,"pnew0001",undefined,Date.now)).toEqual([]);
});

test.skipIf(process.platform==="win32")("the custody check of a lane whose image root holds the unreadable system directories of the volume resolves: their names are declared, the evidence scan does not enter them",()=>{
 const root=area();
 try{
  enterLaneV1(root,"pa000001",owner,["x"]).release();
  const base=lanePathsV1(root,"pa000001").base,system=join(base,UNIT_CACHE_POLICY_V1.layout.volumeMetadata[0]!);
  expect(UNIT_CACHE_POLICY_V1.layout.volumeMetadata).toContain(".fseventsd");
  mkdirSync(system);chmodSync(system,0o000);
  try{expect(retainRemovedLaneV1(lanePathsV1(root,"pa000001"))).toEqual([]);}finally{chmodSync(system,0o700);}
  expect(removeIdleLaneV1(root,"pa000001",Date.now())).toBe(true);expect(existsSync(base)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
