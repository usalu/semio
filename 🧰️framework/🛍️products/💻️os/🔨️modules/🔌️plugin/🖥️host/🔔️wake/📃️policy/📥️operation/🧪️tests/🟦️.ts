import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};
test("per-operation source bounds preserve declared cleanup and match independent SQLite projection",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 const db=new Database(":memory:");try{
 const minimum=(a:number,b:number)=>db.query<{n:number},[number,number]>("select min(?,?) n").get(a,b)!.n;
 for(const row of fixture.cases){if(!row.valid){expect(row.request.workUnits===0||row.request.maximumElapsedMicroseconds===0).toBe(true);continue;}
 const actual=structuredClone(fixture.caps);
 actual.maximumLifetimeMicroseconds=minimum(actual.maximumLifetimeMicroseconds,row.request.maximumElapsedMicroseconds);
 actual.maximumTurnMicroseconds=minimum(actual.maximumTurnMicroseconds,actual.maximumLifetimeMicroseconds);
 expect(actual,row.id).toEqual(row.expected!);expect(Object.keys(actual)).not.toContain("operationId");expect(Object.keys(actual)).not.toContain("generation");
 }
 }finally{db.close();}
});

test("infrastructure is required and cannot reuse cleanup or transport stock",()=>{const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");try{for(const row of fixture.refusals){if(row.field==="missing"){const caps={...fixture.caps}as Partial<typeof fixture.caps>;delete caps.infrastructure;expect(validate({...fixture,caps}),row.id).toBe(false);}else{const work=row.field==="workFuel",total=work?fixture.caps.operationFuel:fixture.caps.operation.maximumCapacityBytes,cleanup=work?fixture.caps.cleanup.workFuel:fixture.caps.cleanup.retained.maximumCapacityBytes,transit=work?fixture.caps.transit.workFuel:fixture.caps.transit.retained.maximumCapacityBytes;expect((db.query("SELECT ?-?-?-? AS available").get(total,cleanup,transit,total)as{available:number}).available,row.id).toBeLessThan(0);}}}finally{db.close();}});

test("one finite lifetime spans several physical turns without replacing its original deadline",()=>{const db=new Database(":memory:"),trace=fixture.turnTrace;try{expect(trace.now[1]-trace.now[0]).toBeGreaterThan(trace.maximumTurnMicroseconds);expect(trace.absoluteDeadlineMicroseconds).toBe(trace.now[0]+trace.requestedLifetimeMicroseconds);for(const[index,now]of trace.now.entries()){const actual=(db.query("SELECT min(?+?,?)-? AS deadline").get(now,trace.maximumTurnMicroseconds,trace.absoluteDeadlineMicroseconds,fixture.caps.cleanup.maximumMicroseconds)as{deadline:number}).deadline;expect(actual).toBe(trace.callbackDeadlinesMicroseconds[index]);expect(actual).toBeLessThan(trace.absoluteDeadlineMicroseconds);}}finally{db.close();}});
