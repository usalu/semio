/** ♻️ Contract: the recycle decision needs a threshold in the environment, a live heap above it and a recycle left; its error names both languages and carries the counters the next worker reads back unchanged.
 * @see ../🟨️.mjs */
import {expect,test} from "bun:test";
import schema from "../🧬️schema/🔣️.json";
import {SLICE_RECYCLE_POLICY_V1 as policy,sliceRecycleCarriedV1,sliceRecycleDecisionV1,sliceRecycleEnvironmentV1,sliceRecycleErrorV1,sliceRecycleOfV1} from "../🟨️.mjs";

const MiB=1048576,environment=sliceRecycleEnvironmentV1(undefined);

test("the policy is the schema, with both languages in every description",()=>{
 expect<unknown>(policy).toEqual(schema.definitions.Policy.const);expect(policy.heapMegabytes).toBe(2048);
 for(const text of[schema.description,schema.definitions.Policy.description,schema.definitions.State.description])expect(text).toMatch(/^EN: .+ DE: .+/su);
});

test("a worker recycles only above the threshold, only with a threshold, and never after the maximum",()=>{
 expect(sliceRecycleDecisionV1({environment,heapBytes:(policy.heapMegabytes-1)*MiB})).toBeUndefined();
 expect(sliceRecycleDecisionV1({environment,heapBytes:(policy.heapMegabytes+1)*MiB})).toEqual({heapMegabytes:policy.heapMegabytes+1,limitMegabytes:policy.heapMegabytes,recycles:1});
 expect(sliceRecycleDecisionV1({environment:{},heapBytes:1e15})).toBeUndefined();
 expect(sliceRecycleDecisionV1({environment:{[policy.heapEnvironment]:"0"},heapBytes:1e15})).toBeUndefined();
 expect(sliceRecycleDecisionV1({environment,heapBytes:Number.NaN})).toBeUndefined();
 const spent={...environment,...sliceRecycleEnvironmentV1({recycles:policy.maximumRecycles,round:2,passes:3,owners:5,heapMegabytes:3000})};
 expect(sliceRecycleDecisionV1({environment:spent,heapBytes:1e15})).toBeUndefined();
});

test("the counters travel through the environment and the error carries them in both languages",()=>{
 const decision=sliceRecycleDecisionV1({environment,heapBytes:3000*MiB})!,error=sliceRecycleErrorV1({decision,round:2,passes:2,owners:1310});
 expect(error.code).toBe("SEMIO_SLICE_RECYCLE");expect(error.message.split("\n")).toHaveLength(2);
 expect(error.message).toContain("3000 MB");expect(error.message).toContain("recycle 1 of "+policy.maximumRecycles);expect(error.message).toContain("Erneuerung 1 von "+policy.maximumRecycles);
 expect(error.recycle).toEqual({recycles:1,round:2,passes:2,owners:1310,heapMegabytes:3000});
 expect(sliceRecycleCarriedV1({...environment,...sliceRecycleEnvironmentV1(error.recycle)})).toEqual(error.recycle);
 const later=sliceRecycleErrorV1({decision:{...decision,recycles:2},round:3,passes:1,owners:1,carried:error.recycle});expect(later.recycle.passes).toBe(3);
 expect(sliceRecycleOfV1(Object.assign(new Error("wrapped"),{cause:error}))).toEqual(error.recycle);expect(sliceRecycleOfV1(new Error("plain"))).toBeUndefined();expect(sliceRecycleOfV1(undefined)).toBeUndefined();
 expect(sliceRecycleCarriedV1({[policy.stateEnvironment]:"{broken"})).toEqual({recycles:0,round:0,passes:0,owners:0,heapMegabytes:0});
});
