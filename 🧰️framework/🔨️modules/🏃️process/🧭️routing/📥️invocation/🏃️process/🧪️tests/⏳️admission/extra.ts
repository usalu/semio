import{test,expect}from"bun:test";
import Ajv from"ajv";
import graphlib from"graphlib";
import*as api from"../../🟦️.ts";
import fixture from"./extra.json";
import schema from"./extra.schema.json";
const envelope=(owner:string,maximum:number)=>api.createScriptProcessEnvelope({version:1,owner,maximumElapsedMilliseconds:maximum},{fixture:owner},Date.now());
const wire=(environment:Readonly<Record<string,string|undefined>>)=>JSON.parse(environment[api.SCRIPT_PROCESS_INVOCATION_ENV]!);

test("distinct issued invocation roots retain the followed bound and cancellation",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const graph=new graphlib.Graph({directed:true});graph.setEdge("child","parent");graph.setEdge("child","following");expect(graphlib.alg.preorder(graph,"child").length).toBe(fixture.expected.reachableControls);
 let ready!:()=>void,release!:()=>void;const readyPromise=new Promise<void>(done=>ready=done),gate=new Promise<void>(done=>release=done),abort=new AbortController(),reason=Error("distinct invocation following cancelled");let followed:ReturnType<typeof api.createIssuedResourceControlV1>|undefined,firstInvocation:unknown;
 const first=api.withScriptProcessEnvelope(envelope(fixture.owners.first,fixture.followingMaximumMilliseconds),async invocation=>{
  firstInvocation=invocation;followed=api.createIssuedResourceControlV1(invocation,fixture.owners.first,0,abort.signal);ready();await gate;
 });
 const firstSettlement=first.then(()=>({ok:true}),error=>({ok:false,error}));
 try{
  await readyPromise;
  await expect(api.withScriptProcessEnvelope(envelope(fixture.owners.second,0),async invocation=>{
   expect(invocation).not.toBe(firstInvocation);expect(invocation.policy.owner).toBe(fixture.owners.second);
   const parent=api.createIssuedResourceControlV1(invocation,fixture.owners.second,fixture.parentMaximumMilliseconds);
   await api.withRequestedIssuedResourceControlV1(parent,0,async child=>{
    const remaining=child.remainingMilliseconds();expect(remaining).toBeGreaterThan(0);expect(remaining!).toBeLessThanOrEqual(fixture.followingMaximumMilliseconds);
    const projected=wire(api.scriptResourceProcessEnvironmentV1(child,{}));expect(projected.deadlineEpochMilliseconds-Date.now()).toBeLessThanOrEqual(fixture.followingMaximumMilliseconds);
    abort.abort(reason);expect(()=>child.remainingMilliseconds()).toThrow(reason);expect(()=>api.scriptResourceProcessEnvironmentV1(child,{})).toThrow(reason);
   },undefined,followed!);
  })).rejects.toThrow();
 }finally{release();}
 expect(await firstSettlement).toEqual({ok:true});
});

test("maximum-only wire projection excludes a healthy renewable idle window",async()=>api.withScriptProcessEnvelope(envelope(fixture.owners.renewable,0),async invocation=>{
 const parent=api.createIssuedResourceControlV1(invocation,fixture.owners.renewable,0),control=api.createRenewableIssuedResourceControlV1(parent,fixture.renewable,0),normal=control.remainingMilliseconds();
 expect(normal).toBeGreaterThan(0);expect(normal!).toBeLessThanOrEqual(fixture.renewable.idleMilliseconds);
 const projected=wire(api.scriptResourceProcessEnvironmentV1(control,{})),left=projected.deadlineEpochMilliseconds-Date.now();
 expect(projected.policy.maximumElapsedMilliseconds).toBe(fixture.renewable.maximumMilliseconds);expect(left).toBeGreaterThan(fixture.renewable.idleMilliseconds*2);expect(left).toBeLessThanOrEqual(fixture.renewable.maximumMilliseconds);
}));

test("maximum-only projection refuses an actually expired renewable idle restriction",async()=>api.withScriptProcessEnvelope(envelope(fixture.owners.renewable,0),async invocation=>{
 const parent=api.createIssuedResourceControlV1(invocation,fixture.owners.renewable,0),control=api.createRenewableIssuedResourceControlV1(parent,fixture.renewable,0);
 expect(wire(api.scriptResourceProcessEnvironmentV1(control,{})).deadlineEpochMilliseconds).toBeGreaterThan(Date.now());
 const started=performance.now();while(performance.now()-started<fixture.expiryWaitMilliseconds){}
 let error:unknown;try{api.scriptResourceProcessEnvironmentV1(control,{});}catch(failure){error=failure;}
 expect(error).toBeInstanceOf(Error);expect((error as Error).name).toBe(fixture.expected.expiryName);expect((error as Error&{reason:string}).reason).toBe(fixture.expected.expiryReason);
}));

test("maximum-only projection rereads renewable cancellation after a healthy projection",async()=>api.withScriptProcessEnvelope(envelope(fixture.owners.renewable,0),async invocation=>{
 const parent=api.createIssuedResourceControlV1(invocation,fixture.owners.renewable,0),abort=new AbortController(),reason=Error("renewable projection cancelled"),control=api.createRenewableIssuedResourceControlV1(parent,fixture.renewable,0,abort.signal);
 expect(wire(api.scriptResourceProcessEnvironmentV1(control,{})).deadlineEpochMilliseconds).toBeGreaterThan(Date.now());abort.abort(reason);expect(()=>api.scriptResourceProcessEnvironmentV1(control,{})).toThrow(reason);
}));
