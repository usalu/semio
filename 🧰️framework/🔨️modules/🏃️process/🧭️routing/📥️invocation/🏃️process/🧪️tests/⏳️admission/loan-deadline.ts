import{test,expect}from"bun:test";
import Ajv from"ajv";
import*as api from"../../🟦️.ts";
import{noteScriptProcessActivityV1}from"../../🐕️watchdog/🟦️.ts";
import{startTelemetrySamplerV1}from"../../../../../📦️artifacts/🏗️native-build/📈️telemetry/⏱️sampling/🟦️.ts";
import{runToolSyncV1}from"../../../../../🧰️tools/🏃️run/🟦️.ts";
import fixture from"./loan-deadline.json";
import schema from"./loan-deadline.schema.json";

const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const within=<T>(run:(invocation:any)=>Promise<T>)=>api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:loan-deadline",maximumElapsedMilliseconds:0},{},Date.now()),run);

test("the loan fixture is a closed schema-first contract",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);});

test("an independent Python model of the renewable loan law agrees with every holder of the fixture",()=>{
 const model=`import json,sys
rows=json.loads(sys.argv[1])
def lives(r):
    beat=r["beatEveryMilliseconds"]
    gap=beat if beat>0 else r["workMilliseconds"]
    return "kept" if r["workMilliseconds"]<=r["maximumMilliseconds"] and gap<r["idleMilliseconds"] else "reclaimed"
print(json.dumps([lives(r) for r in rows]))`,run=runToolSyncV1("python",["-I","-c",model,JSON.stringify(fixture.holders)],{encoding:"utf8",caller:import.meta.url,timeout:10000});
 expect(run.status,run.stderr.toString()).toBe(0);
 expect(fixture.holders.map(row=>row.expected)).toEqual(JSON.parse(run.stdout.toString()));
});

for(const row of fixture.holders)test(`holder ${row.id} is ${row.expected} by its renewable loan`,async()=>{
 await within(async invocation=>{
  const control=api.createRenewableIssuedResourceControlV1(api.createIssuedResourceControlV1(invocation,"fixture:loan-deadline",0),{idleMilliseconds:row.idleMilliseconds,maximumMilliseconds:row.maximumMilliseconds},1),started=performance.now();
  while(!control.signal.aborted&&performance.now()-started<row.workMilliseconds){if(row.beatEveryMilliseconds>0)noteScriptProcessActivityV1();await sleep(row.beatEveryMilliseconds>0?row.beatEveryMilliseconds:20);}
  expect(control.signal.aborted?"reclaimed":"kept").toBe(row.expected);
 });
});

for(const row of fixture.samplers)test(`sampler ${row.id} ends the observed work as ${row.expected}`,async()=>{
 let calls=0;
 const outcome=await within(async invocation=>{
  const sampler=startTelemetrySamplerV1(api.createIssuedResourceControlV1(invocation,"fixture:loan-deadline",0),async()=>{if(row.fails)throw Error("fixture sample fault");if(calls++===0)await sleep(row.firstSampleMilliseconds);},row.intervalMilliseconds);
  await sleep(row.observeMilliseconds);await sampler.stop();
 }).then(()=>"success",error=>{expect(error).not.toBeInstanceOf(api.ResourceLoanDeadlineExhaustedV1);return"failure";});
 expect(outcome).toBe(row.expected);if(!row.fails)expect(calls).toBeGreaterThan(1);
});
