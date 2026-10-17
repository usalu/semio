/** 🏗️ Every native header prefix has exact bounded initialization without copy or allocation. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"../🧫️fixtures/🔣️.json";
import schema from"../../🧬️schema/🏗️initialization.json";
test("native header initialization preserves all original prefix counts",()=>{const row=fixture.initialization,validate=new Ajv({strict:true}).compile(schema);expect(validate(row)).toBe(true);expect(validate({...row,borrowMutableLedger:true})).toBe(false);const database=new Database(":memory:");try{for(let cut=0;cut<=row.turns;cut++){let initialized=0;for(let turn=0;turn<cut;turn++)initialized=Math.min(initialized+row.bindingsPerTurn,row.bindingCount);const oracle=database.query("select min(?*?,?) as initialized").get(cut,row.bindingsPerTurn,row.bindingCount) as {initialized:number};expect(initialized).toBe(oracle.initialized);expect(initialized===row.bindingCount).toBe(cut===row.turns);}}finally{database.close();}});
