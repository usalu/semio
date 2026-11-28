/** ♻️ Contract: a pass boundary hands the read to a fresh worker only when the next pass observes a declared share of the owners again or the heap is past the ceiling; a boundary that forgets few or none of the owners continues in the same worker and keeps every verified owner.
 * @see ../🟦️.ts
 * @see ../../../../🧠️memory/♻️recycle/🧬️schema/🔣️.json */
import {expect,test} from "bun:test";
import {createPassReuseV1} from "../🟦️.ts";
import {SLICE_RECYCLE_POLICY_V1 as policy,sliceRecycleEnvironmentV1,sliceRecycleOfV1} from "../../../../🧠️memory/♻️recycle/🟨️.mjs";

const MiB=1048576;
const reuse=(size:number,forget:number,heapMegabytes:number)=>{
 let held=size;const lines:string[]=[];
 const memo={get size(){return held;},evictReferencing:()=>{held-=forget;return forget;}} as never;
 return {lines,reuse:createPassReuseV1({memo,environment:sliceRecycleEnvironmentV1(undefined),heapBytes:()=>heapMegabytes*MiB,say:line=>{lines.push(line);}})};
};
const boundary=(value:ReturnType<typeof reuse>)=>{value.reuse.boundary({round:1,moved:[]});try{value.reuse.boundary({round:2,moved:["/a"]});return undefined;}catch(error){return sliceRecycleOfV1(error);}};

test("the live read: one path moved, no owner forgotten, heap 2351 MB stays in the worker and says its boundary",()=>{
 const value=reuse(1373,0,2351);
 expect(boundary(value)).toBeUndefined();
 expect(value.lines).toHaveLength(1);expect(JSON.parse(value.lines[0]!.slice(14))).toMatchObject({evicted:0,entries:1373,heap:2351});
});

test("33 of 1373 owners forgotten keeps the worker; a third of them forgotten hands the read over",()=>{
 expect(boundary(reuse(1373,33,2195))).toBeUndefined();
 const state=boundary(reuse(1373,460,2195));
 expect(state).toMatchObject({recycles:1,round:2,heapMegabytes:2195});
});

test("a heap past the ceiling hands the read over whatever the pass forgets",()=>{
 const heap=Math.ceil(policy.heapMegabytes*policy.ceilingBasisPoints/10000)+1;
 expect(boundary(reuse(1373,0,heap))).toMatchObject({recycles:1,heapMegabytes:heap});
});
