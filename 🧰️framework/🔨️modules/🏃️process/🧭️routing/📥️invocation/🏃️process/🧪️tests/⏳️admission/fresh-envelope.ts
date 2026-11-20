import{test,expect,spyOn}from"bun:test";
import Ajv from"ajv";
import*as api from"../../🟦️.ts";
import{startTelemetrySamplerV1}from"../../../../../📦️artifacts/🏗️native-build/📈️telemetry/⏱️sampling/🟦️.ts";
import{refuseInternalV1}from"../../../../../🏗️build/🗣️language/📣️progress/🟦️.ts";
import{renderRepoErrorV1}from"../../../../../🏗️build/🗣️language/🚨️error/🟦️.ts";
import fixture from"./fresh-envelope.json";
import schema from"./fresh-envelope.schema.json";

const within=<T>(run:(invocation:any)=>Promise<T>)=>api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:fixture.owner,maximumElapsedMilliseconds:0},{},Date.now()),run);
const stalled=<T>(step:number,run:()=>T):T=>{const real=Date.now.bind(Date);let ticks=0;const clock=spyOn(Date,"now").mockImplementation(()=>real()+ticks++*step);try{return run();}finally{clock.mockRestore();}};
const projected=(control:Parameters<typeof api.scriptResourceProcessEnvironmentV1>[0],step:number):unknown=>{try{stalled(step,()=>api.scriptResourceProcessEnvironmentV1(control,{}));return undefined;}catch(error){return error;}};

test("the fresh envelope fixture is a closed schema-first contract",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);});

test("an envelope that expired by the time it is read is a typed loan expiry, an envelope beyond its own maximum stays a structural refusal",async()=>{
 await within(async invocation=>{
  const now=Date.now(),envelope=(deadline:number)=>({...api.createScriptProcessEnvelope({version:1,owner:fixture.owner,maximumElapsedMilliseconds:fixture.maximumMilliseconds},{},now),deadlineEpochMilliseconds:deadline});
  expect(()=>api.readScriptProcessEnvelope(envelope(now-fixture.expiredByMilliseconds),now)).toThrow(api.ResourceLoanDeadlineExhaustedV1);
  expect(()=>api.readScriptProcessEnvelope(envelope(now+fixture.maximumMilliseconds+1),now)).toThrow("Original Script process deadline refused");
  expect(()=>api.readScriptProcessEnvelope(envelope(now+fixture.maximumMilliseconds),now)).not.toThrow();
  expect(invocation.policy.owner).toBe(fixture.owner);
 });
});

for(const language of ["en","de"] as const)test(`a typed loan expiry is reported as the ${language} deadline notice and never as an internal error`,()=>{
 const text=renderRepoErrorV1(refuseInternalV1(new api.ResourceLoanDeadlineExhaustedV1()),language);
 expect(text).toContain(fixture.languages[language]);expect(text).not.toMatch(/Internal error|Interner Fehler/u);
});

test("a child envelope is never judged against a second clock reading: every stall ends as a projection or a typed loan expiry",async()=>{
 await within(async invocation=>{
  const outcomes=fixture.stallStepsMilliseconds.map(step=>projected(api.createRequestedIssuedResourceControlV1(api.createIssuedResourceControlV1(invocation,fixture.owner,0),fixture.sampleMilliseconds),step));
  for(const outcome of outcomes)if(outcome!==undefined)expect(outcome).toBeInstanceOf(api.ResourceLoanDeadlineExhaustedV1);
 });
});

test("a telemetry sample whose child projection stalls past its loan is skipped and the observed work stays successful",async()=>{
 let calls=0;
 await within(async invocation=>{
  const sampler=startTelemetrySamplerV1(api.createIssuedResourceControlV1(invocation,fixture.owner,0),async sample=>{const step=fixture.stallStepsMilliseconds[calls++%fixture.stallStepsMilliseconds.length]!;const outcome=projected(sample,step);if(outcome!==undefined)throw outcome;},fixture.intervalMilliseconds);
  await new Promise<void>(done=>setTimeout(done,fixture.observeMilliseconds));await sampler.stop();
 });
 expect(calls).toBeGreaterThan(fixture.stallStepsMilliseconds.length);
});
