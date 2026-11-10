/** 🧭 Contract: the planner port sizes a loaded graph from a bounded sample of its project records, never by serialising the whole graph.
 * @see ../../🔌️ports/🧭️plan/🟨️.mjs */
import{expect,spyOn,test}from"bun:test";
import{estimatedGraphBytesV1}from"../../🔌️ports/🧭️plan/🟨️.mjs";

const graph=(count:number)=>({projects:Object.fromEntries(Array.from({length:count},(_,at)=>["p"+at,{root:"r/"+at,targets:{build:{inputs:["x".repeat(200)+at]}}}]))});

test("the estimate is twice the serialised projects within a tenth and serialises at most a bounded sample",()=>{
 const value=graph(4000),exact=Buffer.byteLength(JSON.stringify(value.projects))*2,spy=spyOn(JSON,"stringify");
 const estimate=estimatedGraphBytesV1(value),calls=spy.mock.calls.length;spy.mockRestore();
 expect(Math.abs(estimate-exact)/exact).toBeLessThan(0.1);
 expect(calls).toBeLessThanOrEqual(65);
});

test("an empty graph is zero and a graph smaller than the sample is sized within a tenth",()=>{
 expect(estimatedGraphBytesV1({projects:{}})).toBe(0);
 const small=graph(10),exact=Buffer.byteLength(JSON.stringify(small.projects))*2;expect(Math.abs(estimatedGraphBytesV1(small)-exact)/exact).toBeLessThan(0.1);
});
