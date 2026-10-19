/** 🏗️ Scalar native bindings and the final header retain distinct cancellation prefixes. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"../🧫️fixtures/🔣️.json";
import schema from"../../🧬️schema/🏗️initialization.json";
test("native header initialization preserves scalar256 and separate final header prefixes",()=>{const row=fixture.initialization,validate=new Ajv({strict:true}).compile(schema);expect(validate(row)).toBe(true);expect(validate({...row,borrowMutableLedger:true})).toBe(false);const db=new Database(":memory:");try{for(let cut=0;cut<=row.turns;cut++){const original={initialized:Math.min(cut*row.bindingsPerTurn,row.bindingCount),ready:cut===row.turns};const oracle=db.query("select min(?*?,?) as initialized,?=? as ready").get(cut,row.bindingsPerTurn,row.bindingCount,cut,row.turns)as{initialized:number;ready:number};expect(original).toEqual({initialized:oracle.initialized,ready:oracle.ready===1});if(cut===row.bindingTurns)expect(original.ready).toBe(false);}expect(row.headerTurns).toBe(1);expect(row.policy.copy).toBe(4096);}finally{db.close();}console.log("[DEBUG] Ajv/SQLite258 prefixes preserve256 scalar slots, final header separation and finite4096 metadata stock; native runtime evidence remains separate");});
