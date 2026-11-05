import{test,expect}from"bun:test";import Ajv from"ajv";import graphlib from"graphlib";
import fixture from"./dag.json";import schema from"./dag.schema.json";
import*as api from"../../🟦️.ts";
test("shared-parent resource DAG checks each reachable restriction within a linear public clock bound",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const measured=[];
 for(const depth of fixture.depths){
  const graph=new graphlib.Graph({directed:true,multigraph:true});for(let i=0;i<=depth;i++){graph.setNode(String(i));if(i){graph.setEdge(String(i),String(i-1),{},"parent");graph.setEdge(String(i),String(i-1),{},"following");}}
  const reachable=graphlib.alg.preorder(graph,String(depth));expect(reachable.length).toBe(depth+1);
  await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:fixture.owner,maximumElapsedMilliseconds:0},{fixture:"control-dag"},Date.now()),async original=>{
   const root=api.createIssuedResourceControlV1(original,fixture.owner,fixture.maximumMilliseconds);
   const visit=async(parent:ReturnType<typeof api.createIssuedResourceControlV1>,remaining:number):Promise<void>=>{if(remaining)return api.withRequestedIssuedResourceControlV1(parent,fixture.maximumMilliseconds,child=>visit(child,remaining-1),undefined,parent);
    const clock=Date.now;let reads=0;let available;try{Date.now=()=>{reads++;return clock();};available=parent.remainingMilliseconds();}finally{Date.now=clock;}
    expect(available).toBeGreaterThan(0);expect(available).toBeLessThanOrEqual(fixture.maximumMilliseconds);measured.push({depth,uniqueNodes:reachable.length,clockReads:reads,maximum:fixture.maximumClockReadsPerUniqueNode*reachable.length});
   };await visit(root,depth);
  });
 }
 console.log("[DEBUG] public control DAG real-clock counts "+JSON.stringify(measured));for(const row of measured)expect(row.clockReads).toBeLessThanOrEqual(row.maximum);
});

test("a followed deadline remains the earliest maximum without renewal",async()=>{
 await expect(api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:fixture.owner,maximumElapsedMilliseconds:0},{fixture:"control-dag-deadline"},Date.now()),async original=>{
  const parent=api.createIssuedResourceControlV1(original,fixture.owner,fixture.maximumMilliseconds),following=api.createRequestedIssuedResourceControlV1(parent,120);
  await api.withRequestedIssuedResourceControlV1(parent,fixture.maximumMilliseconds,async child=>{
   const first=child.remainingMilliseconds();expect(first).toBeGreaterThan(0);expect(first).toBeLessThanOrEqual(120);
   await new Promise(done=>setTimeout(done,170));expect(()=>child.remainingMilliseconds()).toThrow();
  },undefined,following);
 })).rejects.toThrow();
});
test("a followed cancellation is rechecked after a previous healthy traversal",async()=>{
 const reason=Error("fixture followed cancellation"),abort=new AbortController();
 await expect(api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:fixture.owner,maximumElapsedMilliseconds:0},{fixture:"control-dag-following"},Date.now()),async original=>{
  const parent=api.createIssuedResourceControlV1(original,fixture.owner,fixture.maximumMilliseconds),following=api.createIssuedResourceControlV1(original,fixture.owner,fixture.maximumMilliseconds,abort.signal);
  await api.withRequestedIssuedResourceControlV1(parent,fixture.maximumMilliseconds,async child=>{
   expect(child.remainingMilliseconds()).toBeGreaterThan(0);abort.abort(reason);expect(()=>child.remainingMilliseconds()).toThrow(reason);
  },undefined,following);
 })).rejects.toThrow();
});
test("duplicate shared ancestry preserves parent cancellation",async()=>{
 const reason=Error("fixture parent cancellation"),abort=new AbortController();
 await expect(api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:fixture.owner,maximumElapsedMilliseconds:0},{fixture:"control-dag-parent"},Date.now()),async original=>{
  const parent=api.createIssuedResourceControlV1(original,fixture.owner,fixture.maximumMilliseconds,abort.signal);
  await api.withRequestedIssuedResourceControlV1(parent,fixture.maximumMilliseconds,async child=>{
   expect(child.remainingMilliseconds()).toBeGreaterThan(0);abort.abort(reason);expect(()=>child.remainingMilliseconds()).toThrow(reason);
  },undefined,parent);
 })).rejects.toThrow();
});
