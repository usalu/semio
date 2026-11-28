/** ♻️ Contract: the recycle decision needs a threshold in the environment, a live heap above it and a recycle left; its error names both languages and carries the counters the next worker reads back unchanged.
 * @see ../🟨️.mjs */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import {SLICE_RECYCLE_POLICY_V1 as policy,sliceRecycleCarriedV1,sliceRecycleDecisionV1,sliceRecycleEnvironmentV1,sliceRecycleErrorV1,sliceRecycleOfV1} from "../🟨️.mjs";

const MiB=1048576,environment=sliceRecycleEnvironmentV1(undefined);

test("the policy is the schema, with both languages in every description",()=>{
 expect<unknown>(policy).toEqual(schema.definitions.Policy.const);expect(policy.heapMegabytes).toBe(2048);
 for(const text of[schema.description,schema.definitions.Policy.description,schema.definitions.State.description])expect(text).toMatch(/^EN: .+ DE: .+/su);
});

test("a worker recycles only above the threshold, only with a threshold, and never after the maximum",()=>{
 expect(sliceRecycleDecisionV1({environment,kept:0,evicted:0,heapBytes:(policy.heapMegabytes-1)*MiB})).toBeUndefined();
 expect(sliceRecycleDecisionV1({environment,kept:0,evicted:0,heapBytes:(policy.heapMegabytes+1)*MiB})).toEqual({heapMegabytes:policy.heapMegabytes+1,limitMegabytes:policy.heapMegabytes,recycles:1});
 expect(sliceRecycleDecisionV1({environment:{},kept:0,evicted:0,heapBytes:1e15})).toBeUndefined();
 expect(sliceRecycleDecisionV1({environment:{[policy.heapEnvironment]:"0"},kept:0,evicted:0,heapBytes:1e15})).toBeUndefined();
 expect(sliceRecycleDecisionV1({environment,kept:0,evicted:0,heapBytes:Number.NaN})).toBeUndefined();
 const spent={...environment,...sliceRecycleEnvironmentV1({recycles:policy.maximumRecycles,round:2,passes:3,owners:5,heapMegabytes:3000})};
 expect(sliceRecycleDecisionV1({environment:spent,kept:0,evicted:0,heapBytes:1e15})).toBeUndefined();
});

test("the counters travel through the environment and the error carries them in both languages",()=>{
 const decision=sliceRecycleDecisionV1({environment,kept:0,evicted:0,heapBytes:3000*MiB})!,error=sliceRecycleErrorV1({decision,round:2,passes:2,owners:1310});
 expect(error.code).toBe("SEMIO_SLICE_RECYCLE");expect(error.message.split("\n")).toHaveLength(2);
 expect(error.message).toContain("3000 MB");expect(error.message).toContain("recycle 1 of "+policy.maximumRecycles);expect(error.message).toContain("Erneuerung 1 von "+policy.maximumRecycles);
 expect(error.recycle).toEqual({recycles:1,round:2,passes:2,owners:1310,heapMegabytes:3000});
 expect(sliceRecycleCarriedV1({...environment,...sliceRecycleEnvironmentV1(error.recycle)})).toEqual(error.recycle);
 const later=sliceRecycleErrorV1({decision:{...decision,recycles:2},round:3,passes:1,owners:1,carried:error.recycle});expect(later.recycle.passes).toBe(3);
 expect(sliceRecycleOfV1(Object.assign(new Error("wrapped"),{cause:error}))).toEqual(error.recycle);expect(sliceRecycleOfV1(new Error("plain"))).toBeUndefined();expect(sliceRecycleOfV1(undefined)).toBeUndefined();
 expect(sliceRecycleCarriedV1({[policy.stateEnvironment]:"{broken"})).toEqual({recycles:0,round:0,passes:0,owners:0,heapMegabytes:0});
});

test("below the ceiling a handover pays only when the next pass observes the declared share of the owners again (SQL oracle)",()=>{
 const db=new Database(":memory:");
 db.run("create table policy(limit_mb integer,share integer,ceiling integer)");db.run("insert into policy values(?,?,?)",[policy.heapMegabytes,policy.minimumEvictedBasisPoints,policy.ceilingBasisPoints]);
 const rows:{heap:number;kept:number;evicted:number}[]=[];
 for(const heap of[policy.heapMegabytes-1,policy.heapMegabytes,policy.heapMegabytes+1,2351,3071,3072,3073,9000])for(const [kept,evicted] of[[0,0],[1372,1],[1340,33],[1000,333],[1000,334],[999,334],[0,5],[5,0],[1,3]])rows.push({heap,kept,evicted});
 for(const row of rows){
  const oracle=db.query("select case when ?1<=limit_mb then 0 when ?2+?3=0 or ?3*10000>=(?2+?3)*share or ?1*10000>limit_mb*ceiling then 1 else 0 end as recycle from policy").get(row.heap,row.kept,row.evicted) as {recycle:number};
  const decision=sliceRecycleDecisionV1({environment,heapBytes:row.heap*MiB,kept:row.kept,evicted:row.evicted});
  expect(decision!==undefined).toBe(oracle.recycle===1);
 }
});

test("the live case: one path moved, no owner forgotten, a heap of 2351 MB stays in its worker; a heap past the ceiling does not",()=>{
 expect(sliceRecycleDecisionV1({environment,heapBytes:2351*MiB,kept:1373,evicted:0})).toBeUndefined();
 expect(sliceRecycleDecisionV1({environment,heapBytes:2195*MiB,kept:1340,evicted:33})).toBeUndefined();
 expect(sliceRecycleDecisionV1({environment,heapBytes:Math.ceil(policy.heapMegabytes*policy.ceilingBasisPoints/10000+1)*MiB,kept:1373,evicted:0})).toBeDefined();
});
