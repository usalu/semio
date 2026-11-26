/** ⏳️ Contract: a pass that the host killed a unit of restarts in the lane it was compiling in. The lane keeps the Cargo fingerprints, the incremental state and the identity stamps of every unit the pass finished, so it is held for a bounded time after the kill: neither the pool's trim nor the collector's idle age or lane share removes it, no other crate set takes its place as the least recently used lane, and the restart takes it again as its warm lane; the restart's entry clears the hold. The rule is written twice, in production and in an independent Python oracle over one corpus.
 * @see ../../🟦️.ts */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{requireTestToolV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{UNIT_CACHE_POLICY_V1}from"../../../../🔏️key/🟦️.ts";
import{holdLaneOfBuildDirectoryV1,holdLaneV1,laneRetainedV1,listLanesV1,readLaneV1,recordLaneSizeV1}from"../../../🟦️.ts";
import{acquirePooledLaneV1,boundedPoolV1,planPoolAcquireV1,planPoolTrimV1,poolBoundsV1,readPoolV1,type PoolLaneV1}from"../../🟦️.ts";
import corpus from"./🔣️.json";

const base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});
const scratch=()=>mkdtempSync(join(base,"lh-")),lanesOf=(rows:readonly Record<string,unknown>[]):PoolLaneV1[]=>rows.map(row=>({retained:row.held===true,...row}) as PoolLaneV1);

test("the corpus policy is the authored policy",()=>{
 expect(corpus.policy).toEqual({productPrefix:UNIT_CACHE_POLICY_V1.pool.product.prefix,retainMilliseconds:UNIT_CACHE_POLICY_V1.pool.product.retainMilliseconds});
 expect(UNIT_CACHE_POLICY_V1.pool.restartHoldMilliseconds).toBeGreaterThan(0);
 expect(UNIT_CACHE_POLICY_V1.pool.restartHoldMilliseconds).toBeLessThan(UNIT_CACHE_POLICY_V1.gc.laneIdleMilliseconds);
});

test("the production rules and the Python oracle reach the verdict of every corpus case",()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const verdicts=JSON.parse(run.stdout);
 for(const[index,row]of corpus.retained.entries()){expect(laneRetainedV1(row.id,row.lastUsedMs,row.nowMs,(row as {holdUntilMs?:number}).holdUntilMs),row.id+" production").toBe(row.expect);expect(verdicts.retained[index],row.id+" oracle").toBe(row.expect);}
 for(const[index,row]of corpus.trim.entries()){expect(planPoolTrimV1(boundedPoolV1(lanesOf(row.lanes)),row.bounds),row.id+" production").toEqual(row.expect);expect(verdicts.trim[index],row.id+" oracle").toEqual(row.expect);}
 for(const[index,row]of corpus.acquire.entries()){expect<unknown>(planPoolAcquireV1(lanesOf(row.lanes),row.request,row.bounds),row.id+" production").toEqual(row.expect);expect(verdicts.acquire[index],row.id+" oracle").toEqual(row.expect);}
});

test("a restart takes the lane of the killed pass again although another crate set arrived while it waited, and a lane without a hold is lost to that crate set",async()=>{
 const root=scratch(),bounds=poolBoundsV1(50);
 try{
  const lose=async(held:boolean)=>{
   const first=await acquirePooledLaneV1(root,{crates:[held?"hub-held":"hub-bare"],quotaBytes:bounds.quotaBytes});const id=first.paths.id;first.release();
   recordLaneSizeV1(root,id,1000,0,Date.now());if(held)holdLaneV1(root,id,Date.now());
   const peer=await acquirePooledLaneV1(root,{crates:["peer-"+(held?"held":"bare")],quotaBytes:bounds.quotaBytes});peer.release();
   const restart=await acquirePooledLaneV1(root,{crates:[held?"hub-held":"hub-bare"],quotaBytes:bounds.quotaBytes});restart.release();
   return{id,peer:peer.paths.id,restart:restart.paths.id,choice:restart.choice};
  };
  const bare=await lose(false),held=await lose(true);
  expect(bare.peer).toBe(bare.id);expect(bare.choice).not.toBe("warm");
  expect(held.restart).toBe(held.id);expect(held.choice).toBe("warm");expect(held.peer).not.toBe(held.id);
  expect(readLaneV1(root,held.id)?.holdUntilMs).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a hold marks the lane held for the pool until it expires or the lane is entered",async()=>{
 const root=scratch();
 try{
  const first=await acquirePooledLaneV1(root,{crates:["hub"],quotaBytes:Number.MAX_SAFE_INTEGER}),id=first.paths.id;first.release();
  const now=Date.now();holdLaneV1(root,id,now);
  expect(readPoolV1(root,now).find(lane=>lane.id===id)).toMatchObject({held:true,retained:true});
  expect(readPoolV1(root,now+UNIT_CACHE_POLICY_V1.pool.restartHoldMilliseconds).find(lane=>lane.id===id)).toMatchObject({held:false,retained:false});
  const again=await acquirePooledLaneV1(root,{crates:["hub"],quotaBytes:Number.MAX_SAFE_INTEGER});again.release();
  expect(again.paths.id).toBe(id);expect(readPoolV1(root,Date.now()).find(lane=>lane.id===id)).toMatchObject({held:false});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the collector sees a held lane as retained, and only the build directory of a pooled lane can be held by the driver",async()=>{
 const root=scratch();
 try{
  const first=await acquirePooledLaneV1(root,{crates:["hub"],quotaBytes:Number.MAX_SAFE_INTEGER});first.release();
  const now=Date.now();
  expect(listLanesV1(root,now).find(lane=>lane.id===first.paths.id)?.retained).toBe(false);
  expect(holdLaneOfBuildDirectoryV1(root,join(root,"x"),now)).toBe(false);
  expect(holdLaneOfBuildDirectoryV1(root,first.paths.target,now)).toBe(false);
  expect(holdLaneOfBuildDirectoryV1(root,first.paths.build,now)).toBe(true);
  expect(listLanesV1(root,now).find(lane=>lane.id===first.paths.id)?.retained).toBe(true);
  expect(listLanesV1(root,now+UNIT_CACHE_POLICY_V1.pool.restartHoldMilliseconds).find(lane=>lane.id===first.paths.id)?.retained).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
