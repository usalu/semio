import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"./🔣️.json";
import schema from"./🧬️schema/🔣️.json";
import{createNxSdkVerificationObservationV1,withNxSdkVerificationObservationV1,measureNxSdkVerificationV1}from"../🟦️.ts";
/** ⏱️ Bounded aggregate observations preserve the exact result and primary failure; SQLite independently sums the neutral trace. */
test("SDK verification aggregate matches the neutral trace and SQL oracle",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);let now=0;const metrics=createNxSdkVerificationObservationV1(()=>now),db=new Database(":memory:"),failure={};db.exec("create table timing(name text,duration integer)");try{for(const row of fixture.steps){const result={};let actual:unknown,thrown:unknown;try{actual=await metrics.measure(row.name,()=>{now+=row.duration;if(row.failure)throw failure;return result;});}catch(error){thrown=error;}if(row.failure)expect(thrown).toBe(failure);else expect(actual).toBe(result);db.query("insert into timing values(?,?)").run(row.name,row.duration);}const expected=Object.fromEntries(db.query("select name,count(*) calls,sum(duration) duration from timing group by name").all().map((row:any)=>[row.name,[row.calls,row.duration]]));expect(expected).toEqual(fixture.expected);expect(Object.fromEntries(Object.keys(fixture.expected).map(key=>[key,metrics.snapshot()[key]]))).toEqual(fixture.expected);const same={};expect(await metrics.measureAsync("metadata",async()=>same)).toBe(same);}finally{db.close();}
});

/** 🪢️ Concurrent timing scopes retain original receivers and cannot mix their bounded observations. */
test("SDK timing scopes preserve receiver identity and remain independent",async()=>{
 const first=createNxSdkVerificationObservationV1(()=>1),second=createNxSdkVerificationObservationV1(()=>1),receiver={check(this:unknown){expect(this).toBe(receiver);}};
 await Promise.all([withNxSdkVerificationObservationV1(first,async()=>{await Promise.resolve();measureNxSdkVerificationV1("context",()=>receiver.check());}),withNxSdkVerificationObservationV1(second,async()=>{measureNxSdkVerificationV1("context",()=>receiver.check());await Promise.resolve();measureNxSdkVerificationV1("context",()=>receiver.check());})]);expect(first.snapshot().context).toEqual([1,0]);expect(second.snapshot().context).toEqual([2,0]);
});

/** ⏳️ The asynchronous path records completion while preserving the exact rejection and resolved object. */
test("SDK async observations preserve the neutral success and failure identities",async()=>{
 let now=0;const metrics=createNxSdkVerificationObservationV1(()=>now),failure={};
 for(const row of fixture.steps){const value={};let actual:unknown,thrown:unknown;try{actual=await metrics.measureAsync(row.name,async()=>{now+=row.duration;await Promise.resolve();if(row.failure)throw failure;return value;});}catch(error){thrown=error;}if(row.failure)expect(thrown).toBe(failure);else expect(actual).toBe(value);}
 expect(Object.fromEntries(Object.keys(fixture.expected).map(key=>[key,metrics.snapshot()[key]]))).toEqual(fixture.expected);
});
