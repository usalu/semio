
import{test,expect}from"bun:test";
import Ajv from"ajv";


import fixture from"./freshness.json";
import schema from"./freshness.schema.json";
import*as api from"../../🟦️.ts";
test("a shared short parent cannot authorize after real time elapsed in another branch",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const measured=[];
 for(const delayRead of fixture.delayReads){
  let result;
  try{
   await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:fixture.owner,maximumElapsedMilliseconds:0},{fixture:"control-dag-freshness"},Date.now()),async original=>{
    const p=api.createIssuedResourceControlV1(original,"short",fixture.shortMilliseconds);
    const q=api.createIssuedResourceControlV1(original,"long",fixture.longMilliseconds);
    await api.withRequestedIssuedResourceControlV1(p,0,async a=>{
     await api.withRequestedIssuedResourceControlV1(q,0,async b=>{
      await api.withRequestedIssuedResourceControlV1(p,0,async c=>{
       const clock=Date.now;let reads=0,delayed=false,authorized=false;
       try{
        Date.now=()=>{reads++;if(reads===delayRead){const started=performance.now();while(performance.now()-started<fixture.delayMilliseconds){}delayed=true;}return clock();};
        const left=c.remainingMilliseconds();authorized=left===null||left>0;
       }catch{}finally{Date.now=clock;}
       let parentExpired=false;try{p.remainingMilliseconds();}catch{parentExpired=true;}
       result={delayRead,reads,delayed,authorized,parentExpired};
      },undefined,b);
     },undefined,a);
    });
   });
  }catch{}
  expect(result).toBeDefined();measured.push(result);
 }
 expect(measured.filter(row=>row.delayed&&row.authorized&&row.parentExpired).length).toBe(fixture.expectedStaleAuthorizations);
});

test("fresh cutoff evaluation preserves an open authenticated admission wait",async()=>{
 const{beginAdmissionWaitV1,admissionWaitedMillisecondsV1}=await import("../../🐕️watchdog/🟦️.ts");
 let authorized=false,delayed=false,observed;
 await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:fixture.owner,maximumElapsedMilliseconds:0},{fixture:"control-dag-open-wait"},Date.now()),async original=>{
  const p=api.createIssuedResourceControlV1(original,"short-wait",fixture.shortMilliseconds),q=api.createIssuedResourceControlV1(original,"long-wait",fixture.longMilliseconds);
  await api.withRequestedIssuedResourceControlV1(p,0,async a=>{
   await api.withRequestedIssuedResourceControlV1(q,0,async b=>{
    await api.withRequestedIssuedResourceControlV1(p,0,async c=>{
     const before=admissionWaitedMillisecondsV1(),end=beginAdmissionWaitV1(),clock=Date.now;let reads=0;
     try{
      Date.now=()=>{reads++;if(reads===fixture.waitClockRead){const started=performance.now();while(performance.now()-started<fixture.delayMilliseconds){}delayed=true;}return clock();};
      const left=c.remainingMilliseconds();authorized=left===null||left>0;
     }catch(error){observed=error instanceof Error?error.message:String(error);}
     finally{Date.now=clock;end();}
     expect(admissionWaitedMillisecondsV1()-before).toBeGreaterThanOrEqual(fixture.delayMilliseconds);
    },undefined,b);
   },undefined,a);
  });
 });
 expect(delayed).toBe(true);expect(authorized).toBe(fixture.expectedWaitAuthorized);
});
