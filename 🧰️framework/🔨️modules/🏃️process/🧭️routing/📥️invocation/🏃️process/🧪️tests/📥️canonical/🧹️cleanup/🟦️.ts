import assert from"node:assert/strict";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"./🔣️.json";
import schema from"../../../🧬️schema/🧹️cleanup.json";
import * as api from"../../../🟦️.ts";
import type{AcceptedPhysicalCleanupV1,PhysicalCleanupRequestV1}from"../../../🧹️cleanup/🟦️.ts";

/** 🛟️ Checks real canonical enrollment funding, exact-token custody and canceled-body cleanup drainage. */
export async function proveAcceptedPhysicalCleanupV1():Promise<void>{
 const result={...fixture.observations};for(const key of Object.keys(result)as(keyof typeof result)[])result[key]=false;
 await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:cleanup",maximumElapsedMilliseconds:5000},{fixture:"cleanup"},Date.now()),async original=>{
  const parent=api.createIssuedResourceControlV1(original,"fixture:cleanup",3000),validate=new Ajv({strict:true}).compile(schema);
  for(const row of fixture.requests){let effects=0;try{await api.withAcceptedPhysicalCleanupV1(parent,row.value as PhysicalCleanupRequestV1,async()=>{effects++;});}catch{}assert.equal(effects!==0,row.accepted);assert.equal(validate(row.value),row.accepted);}
  let effects=0;try{await api.withAcceptedPhysicalCleanupV1(api.createRequestedIssuedResourceControlV1(parent,50),{version:1,kind:"native-image",resource:"fixture:unfunded",reserveMilliseconds:100},async()=>{effects++;});}catch{}result.insufficientFundingBeforeEffects=effects===0;
  const canceled=new AbortController(),control=api.createRequestedIssuedResourceControlV1(parent,1000,canceled.signal);let escaped:AcceptedPhysicalCleanupV1|undefined;
  await api.withAcceptedPhysicalCleanupV1(control,{version:1,kind:"native-image",resource:"fixture:physical",reserveMilliseconds:200},async(token,body)=>{
   escaped=token;const facts=api.readAcceptedPhysicalCleanupV1(token);if(facts.deadlineEpochMilliseconds===null||facts.bodyDeadlineEpochMilliseconds===null)throw Error("Finite cleanup fixture deadlines required");assert.equal(facts.deadlineEpochMilliseconds-facts.bodyDeadlineEpochMilliseconds,200);assert.ok(body.remainingMilliseconds()!<=800);assert.throws(()=>api.readAcceptedPhysicalCleanupV1({...token}));result.privateToken=true;
   let entered!:()=>void,release!:()=>void,stopped=false,settled=false;const ready=new Promise<void>(done=>{entered=done;});void api.withIssuedResourceOperationV1(body,async()=>{entered();await new Promise<void>(done=>{release=done;});stopped=true;});await ready;
   const first=api.beginAcceptedPhysicalCleanupV1(token),second=api.beginAcceptedPhysicalCleanupV1(token);result.sameBeginPromise=first===second;void first.then(()=>{settled=true;});await new Promise<void>(done=>setImmediate(done));assert.equal(settled,false);try{api.withIssuedResourceOperationV1(body,async()=>{});}catch{result.lateBodyRefused=true;}
   canceled.abort(Error("fixture body canceled"));assert.throws(()=>body.remainingMilliseconds());release();await first;result.bodyDrained=stopped&&settled;result.independentCleanup=!facts.cleanupSignal.aborted&&api.checkAcceptedPhysicalCleanupV1(token)>0;result.fixedDeadline=api.readAcceptedPhysicalCleanupV1(token).deadlineEpochMilliseconds===facts.deadlineEpochMilliseconds;
  });try{api.readAcceptedPhysicalCleanupV1(escaped!);}catch{result.retiredTokenRefused=true;}
 });
 const database=new Database(":memory:");try{await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:cleanup-ordinary",maximumElapsedMilliseconds:0},{fixture:"cleanup"},Date.now()),async original=>{let effects=0;await api.withAcceptedPhysicalCleanupV1(api.createIssuedResourceControlV1(original,"fixture:ordinary",0),{version:1,kind:"native-image",resource:"fixture:ordinary",reserveMilliseconds:100},async(token,body)=>{effects++;const facts=api.readAcceptedPhysicalCleanupV1(token);assert.equal(body.remainingMilliseconds(),null);await api.beginAcceptedPhysicalCleanupV1(token);const remaining=api.checkAcceptedPhysicalCleanupV1(token),oracle=database.query("select ?1 is null and ?2 is null and ?3=1 and ?4>0 and ?4<=?5 accepted").get(facts.deadlineEpochMilliseconds,facts.bodyDeadlineEpochMilliseconds,effects,remaining,facts.reserveMilliseconds)as{accepted:number};result.ordinaryAcceptedWithFixedReserve=Boolean(oracle.accepted);assert.equal(facts.cleanupSignal.aborted,false);});assert.equal(original.control.remainingMilliseconds(),null);assert.equal(original.control.signal.aborted,false);});}finally{database.close();}
 const observation=new Ajv({strict:true}).addSchema(schema,"cleanup").compile({$ref:"cleanup#/definitions/EnrollmentObservations"});assert.equal(observation(result),true);
 assert.deepEqual(result,fixture.observations);console.log("[DEBUG] Actual canonical accepted physical cleanup funding and drain "+JSON.stringify({requests:fixture.requests.length,...result}));
}
