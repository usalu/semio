/** 🛡️ Contract: a product lane used within the retention window of an open release generation is exempt from the pool's lane and byte bounds and from the collector's idle age and lane share; only free-space pressure removes it, after every other idle lane. The pool rule is written twice, in production (`laneRetainedV1`, `boundedPoolV1`, `planPoolTrimV1`) and in the independent Python oracle `./🐍️.py`; the collector rule is checked against the hand-authored vectors of `./🔣️.json`, which are language-agnostic.
 * @see ../../🟦️.ts */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{fileURLToPath}from"node:url";
import{requireTestToolV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{UNIT_CACHE_POLICY_V1}from"../../../../🔏️key/🟦️.ts";
import{laneRetainedV1}from"../../../🟦️.ts";
import{planGcV1,type GcInputV1}from"../../../../🧹️gc/🟦️.ts";
import{boundedPoolV1,planPoolTrimV1,type PoolLaneV1}from"../../🟦️.ts";
import corpus from"./🔣️.json";

const lanesOf=(rows:typeof corpus.trim[number]["lanes"],nowMs:number):PoolLaneV1[]=>rows.map(row=>({...row,product:row.id.startsWith(UNIT_CACHE_POLICY_V1.pool.product.prefix),retained:laneRetainedV1(row.id,row.lastUsedMs,nowMs)}));

test("the corpus policy is the authored policy",()=>{
 expect(corpus.policy).toEqual({productPrefix:UNIT_CACHE_POLICY_V1.pool.product.prefix,retainMilliseconds:UNIT_CACHE_POLICY_V1.pool.product.retainMilliseconds});
 expect(corpus.policy.retainMilliseconds).toBeLessThan(UNIT_CACHE_POLICY_V1.gc.laneIdleMilliseconds);
});

test("the production retention and trim and the Python oracle reach the verdict of every corpus case",()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const verdicts=JSON.parse(run.stdout);
 for(const[index,row]of corpus.retained.entries()){expect(laneRetainedV1(row.id,row.lastUsedMs,row.nowMs),row.id+" production").toBe(row.expect);expect(verdicts.retained[index],row.id+" oracle").toBe(row.expect);}
 for(const[index,row]of corpus.trim.entries()){expect(planPoolTrimV1(boundedPoolV1(lanesOf(row.lanes,row.nowMs)),row.bounds),row.id+" production").toEqual(row.expect);expect(verdicts.trim[index],row.id+" oracle").toEqual(row.expect);}
});

test("the collector plan exempts retained lanes from the idle age and the share and removes them last under pressure",()=>{
 for(const row of corpus.gc)expect(planGcV1(row.input as GcInputV1),row.id).toEqual(row.expect);
});

test("a lane without the retained flag is planned as before",()=>{
 const input:GcInputV1={capacityBytes:1000,volumeBytes:1000,freeBytes:1000,nowMs:1_000_000_000,lanes:[{id:"a",bytes:100,incrementalBytes:0,lastUsedMs:1,live:false},{id:"b",bytes:100,incrementalBytes:0,lastUsedMs:999_999_000,live:false}]};
 expect(planGcV1(input).removeLanes).toEqual(["a"]);
});
