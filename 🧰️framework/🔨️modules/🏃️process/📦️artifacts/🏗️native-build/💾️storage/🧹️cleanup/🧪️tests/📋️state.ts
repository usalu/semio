import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/📥️continuation.json" with {type:"json"};
import fundingSchema from "../📋️state/🧬️schema/🔣️.json" with {type:"json"};
import corpus from "./🧫️fixtures/📥️continuation.json" with {type:"json"};
import {nativeImageCleanupTraceV1,nativeImageCleanupFundingV1,nativeImageCleanupTerminalDeadlineV1,nativeImageCleanupStateV1,nativeImageCleanupTransitionV1,nativeImageCleanupResultV1,throwNativeImageCleanupFailuresV1} from "../📋️state/🟦️.ts";

function relationalTrace(schedule:readonly string[]):unknown{
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE events(position INTEGER PRIMARY KEY,event TEXT NOT NULL)");const insert=database.query("INSERT INTO events VALUES(?,?)");schedule.forEach((event,index)=>insert.run(index,event));
  const facts=database.query("SELECT COALESCE(MIN(CASE WHEN event='close-first' THEN position END),999) AS closing,COALESCE(MIN(CASE WHEN event='accept-persistent-handoff' THEN position END),999) AS persistent,MAX(event IN ('refuse-expired-original','present-copied-grant')) AS refused,MAX(event='create-owned-backing') AS owned,MAX(event='observe-exact-inactive') AS inactive,MAX(event='accept-unresolved-handoff') AS transferred,MAX(event='fail-guard-release') AS guard_failed,MAX(event='publish-body-progress')>MAX(event='join-body-progress') AS progress_pending,MAX(event IN ('replace-backing','fail-observation')) AS unknown,MAX(event='observe-exact-active') AS active FROM events").get() as {closing:number;persistent:number;refused:number;owned:number;inactive:number;transferred:number;guard_failed:number;progress_pending:number;unknown:number;active:number};
  const attempts=Number(!facts.refused&&facts.closing<999&&facts.persistent>facts.closing&&!facts.progress_pending&&!facts.unknown&&facts.active),settlement=facts.refused?"refused":facts.guard_failed?"unresolved":facts.persistent<facts.closing?"persistent":facts.inactive?"physical":"unresolved",guard=facts.refused?"never-acquired":facts.guard_failed?"held":settlement==="persistent"?"persistent-reader-release":settlement==="physical"?"after-physical-settlement":facts.transferred?"after-unresolved-handoff":"held";
  const failures=database.query("SELECT failure FROM (SELECT CASE WHEN event IN ('cancel-body','expire-body-cutoff','activation-child-fails') THEN 'body' WHEN event='reject-body-progress' OR (event='expire-cleanup-absolute-bound' AND ?=1) THEN 'progress' WHEN event IN ('expire-cleanup-absolute-bound','fail-deactivation','observe-still-active','replace-backing','fail-observation') THEN 'cleanup' WHEN event='fail-persistent-publication' THEN 'handoff' WHEN event='fail-guard-release' THEN 'guard' WHEN event IN ('refuse-expired-original','present-copied-grant') THEN 'admission' END AS failure,MIN(position) AS position FROM events GROUP BY failure) WHERE failure IS NOT NULL ORDER BY position").all(facts.progress_pending) as {failure:string}[];
  const ordered=failures.map(row=>row.failure);if(facts.progress_pending&&schedule.includes("expire-cleanup-absolute-bound")&&!ordered.includes("cleanup"))ordered.push("cleanup");
  return{deactivationAttempts:attempts,bodyAdmissionsAfterStop:0,guardRelease:guard,settlement,failures:ordered,absoluteDeadlineRenewed:false};
 }finally{database.close();}
}

test("cleanup continuation fixture admission agrees with independent Ajv",()=>{expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);});
for(const row of corpus.cases)test(row.id,()=>{
 const independent=relationalTrace(row.schedule);expect(independent).toEqual(row.expected);expect(independent).toEqual(nativeImageCleanupTraceV1(row.schedule));
});

test("cleanup funding subtracts its reserve from the same finite absolute operation",()=>{
 const database=new Database(":memory:"),value={version:1,acceptedAtMilliseconds:1000,wholeDeadlineMilliseconds:61000,reserveMilliseconds:10000};
 try{
  expect(new Ajv({strict:true}).compile(fundingSchema)(value)).toBe(true);
  const independent=database.query("SELECT ? AS version,? AS acceptedAtMilliseconds,? AS wholeDeadlineMilliseconds,?-? AS bodyDeadlineMilliseconds,? AS reserveMilliseconds").get(1,1000,61000,61000,10000,10000);
  expect(independent).toEqual(nativeImageCleanupFundingV1(value));
 }finally{database.close();}
});

test("cleanup funding refuses insufficient, unbounded and extra authority data",()=>{
 for(const value of[{version:1,acceptedAtMilliseconds:1000,wholeDeadlineMilliseconds:11000,reserveMilliseconds:10000},{version:1,acceptedAtMilliseconds:1000,wholeDeadlineMilliseconds:null,reserveMilliseconds:10000},{version:1,acceptedAtMilliseconds:1000,wholeDeadlineMilliseconds:61000,reserveMilliseconds:10000,control:{}}])expect(()=>nativeImageCleanupFundingV1(value)).toThrow();
});

test("body cancellation immediately closes creation and progress admissions",()=>{
 let state=nativeImageCleanupStateV1();for(const event of["receive-original","enroll-funded-cleanup","create-owned-backing","cancel-body"])state=nativeImageCleanupTransitionV1(state,event);
 expect(()=>nativeImageCleanupTransitionV1(state,"create-owned-backing")).toThrow();expect(()=>nativeImageCleanupTransitionV1(state,"publish-body-progress")).toThrow();
});

test("pending actual work refuses guard release and unresolved handoff",()=>{
 let state=nativeImageCleanupStateV1();for(const event of["receive-original","enroll-funded-cleanup","create-owned-backing","submit-activation","cancel-body","close-first"])state=nativeImageCleanupTransitionV1(state,event);
 expect(nativeImageCleanupResultV1(state).guardRelease).toBe("held");expect(()=>nativeImageCleanupTransitionV1(state,"accept-unresolved-handoff")).toThrow();
});

test("accepted body work drains before one physical cleanup attempt",()=>{
 let state=nativeImageCleanupStateV1();for(const event of["receive-original","enroll-funded-cleanup","create-owned-backing","submit-activation","join-activation-child","observe-exact-active","submit-body-work","cancel-body","close-first"])state=nativeImageCleanupTransitionV1(state,event);
 expect(nativeImageCleanupResultV1(state).deactivationAttempts).toBe(0);expect(nativeImageCleanupResultV1(state).guardRelease).toBe("held");expect(()=>nativeImageCleanupTransitionV1(state,"accept-unresolved-handoff")).toThrow();
 state=nativeImageCleanupTransitionV1(state,"join-body-work");expect(nativeImageCleanupResultV1(state).deactivationAttempts).toBe(1);
});

test("terminal cleanup deadline never exceeds its original whole bound",()=>{
 const funding=nativeImageCleanupFundingV1({version:1,acceptedAtMilliseconds:1000,wholeDeadlineMilliseconds:61000,reserveMilliseconds:10000}),database=new Database(":memory:");
 try{for(const closing of[2000,55000]){const expected=(database.query("SELECT min(?,?+?) AS value").get(61000,closing,10000) as {value:number}).value;expect(nativeImageCleanupTerminalDeadlineV1(funding,closing)).toBe(expected);}expect(()=>nativeImageCleanupTerminalDeadlineV1(funding,61000)).toThrow();expect(Object.isFrozen(funding)).toBe(true);}finally{database.close();}
});

test("funding facts cannot be rebound to a later accepted origin",()=>{
 const funding=nativeImageCleanupFundingV1({version:1,acceptedAtMilliseconds:1000,wholeDeadlineMilliseconds:61000,reserveMilliseconds:10000});expect(()=>nativeImageCleanupTerminalDeadlineV1({...funding,acceptedAtMilliseconds:60000},60000)).toThrow();
});

test("one enrollment cannot replace its accepted backing identity",()=>{
 let state=nativeImageCleanupStateV1();for(const event of["receive-original","enroll-funded-cleanup","create-owned-backing"])state=nativeImageCleanupTransitionV1(state,event);expect(()=>nativeImageCleanupTransitionV1(state,"create-owned-backing")).toThrow();
});

test("cleanup aggregation preserves every actual failure and thrown undefined",()=>{
 const failures=[{role:"body" as const,cause:undefined},{role:"cleanup" as const,cause:Error("deactivation failed")},{role:"guard" as const,cause:Error("release failed")}],database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE failures(position INTEGER PRIMARY KEY,role TEXT)");failures.forEach((row,index)=>database.query("INSERT INTO failures VALUES(?,?)").run(index,row.role));const independent=database.query("SELECT role FROM failures ORDER BY position").all();
  let actual:unknown;try{throwNativeImageCleanupFailuresV1(failures);}catch(error){actual=error;}expect(actual).toBeInstanceOf(AggregateError);expect((actual as AggregateError).errors).toEqual(failures.map(row=>row.cause));expect(independent).toEqual(failures.map(row=>({role:row.role})));
  let thrown=false,single:unknown;try{throwNativeImageCleanupFailuresV1([failures[0]!]);}catch(error){thrown=true;single=error;}expect(thrown).toBe(true);expect(single).toBeUndefined();
 }finally{database.close();}
});
