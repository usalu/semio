import {expect,test} from "bun:test";
import {availableParallelism} from "node:os";
import {mkdirSync,mkdtempSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {withBuildTestInvocationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import {fixedCompilerCapacityRequestV1,scriptCompilerAdmissionControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import orderPolicy from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🧭️order/🔣️policy/🔣️json/🟦️.ts";
import {withCountedBunCapacity} from "../../🟦️.ts";

const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;
mkdirSync(output,{recursive:true});
const distinctClaimants=join(output,"lease-order-policy-bundle.json");writeFileSync(distinctClaimants,JSON.stringify({...orderPolicy,fairness:{...orderPolicy.fairness,reentrantShared:false}}));process.env.SEMIO_LEASE_ORDER_POLICY=distinctClaimants;process.env.SEMIO_LEASE_ORDER="off";
const storage=mkdtempSync(join(output,"bundle-slot-"));

test("two Bun bundles of different targets run inside the counted slot at the same time, each with every worker bound set to its own share of the host",()=>withBuildTestInvocationV1(async original=>{
 const cores=availableParallelism();if(cores<4)return;
 const slot=fixedCompilerCapacityRequestV1("bun-bundle") as {maximumParallelism:number;minimumParallelism:number},seen:Record<string,string|undefined>[]=[];let arrived=0;
 const run=(name:string)=>withCountedBunCapacity(storage,scriptCompilerAdmissionControlV1(original,"test:bun-slot:"+name,120_000),async environment=>{
  arrived++;seen.push(environment);
  for(let attempt=0;attempt<400&&arrived<2;attempt++)await Bun.sleep(25);
  return arrived;
 },{LANG:"en_US.UTF-8"});
 const together=await Promise.all([run("a"),run("b")]);
 expect(together,"both bundlers were inside their slot at once").toEqual([2,2]);
 for(const environment of seen){const bound=Number(environment.SEMIO_COMPILER_PARALLELISM);expect(bound).toBeGreaterThanOrEqual(slot.minimumParallelism);expect(bound).toBeLessThanOrEqual(slot.maximumParallelism);for(const name of["GOMAXPROCS","UV_THREADPOOL_SIZE"])expect(environment[name],name).toBe(String(bound));expect(environment.SEMIO_COMPILER_CAPACITY_HELD).toBeUndefined();}
 expect(seen.reduce((sum,environment)=>sum+Number(environment.SEMIO_COMPILER_PARALLELISM),0)).toBeLessThanOrEqual(cores);
}),60_000);
