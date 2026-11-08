import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "./projection.schema.json";
import fixture from "./projection.json";

test("a recorded projection survives unwind until its original settlement",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{
  expect(db.query("select value from json_each(?) order by key").all(JSON.stringify(fixture.steps))).toEqual(fixture.steps.map(value=>({value})));
  expect(db.query("select json_extract(?, '$.sourceCalls') as calls, json_extract(?, '$.recoveryCalls') as recoveries").get(JSON.stringify(fixture.unwind),JSON.stringify(fixture.unwind))).toEqual({calls:1,recoveries:0});
  expect(db.query("select value from json_each(?) order by key").all(JSON.stringify(fixture.cancelPhases))).toEqual(fixture.cancelPhases.map(value=>({value})));
  expect(db.query("select value from json_each(?) order by key").all(JSON.stringify(fixture.stagedCancelCuts))).toEqual(fixture.stagedCancelCuts.map(value=>({value})));
  expect(db.query("select json_extract(?, '$.input') as input, json_extract(?, '$.idleCancelCalls') as idle, json_extract(?, '$.successCalls') as success, json_extract(?, '$.resultAfterSettlement') as settled").get(...Array(4).fill(JSON.stringify(fixture.stagedProjection)))).toEqual({input:73,idle:0,success:1,settled:1});
  expect(db.query("select json_extract(?, '$.unboundWork') as work, json_extract(?, '$.deniedWork') as denied, json_extract(?, '$.unbound') as bound, json_extract(?, '$.boundWork') as boundWork, json_extract(?, '$.foreignWork') as foreignWork").get(...Array(5).fill(JSON.stringify(fixture.bindingCheck)))).toEqual({work:1,denied:0,bound:0,boundWork:5,foreignWork:2});
  expect(db.query("select json_extract(?, '$.value') as value, json_extract(?, '$.terminal') as terminal").get(...Array(2).fill(JSON.stringify(fixture.sourceClose)))).toEqual({value:73,terminal:1});
  expect(db.query("select json_extract(?, '$.ownerMoved') as moved, json_extract(?, '$.work') as work").get(JSON.stringify(fixture.denied),JSON.stringify(fixture.denied))).toEqual({moved:0,work:0});
  expect(db.query("select json_extract(?, '$.input') as input, json_extract(?, '$.deniedWork') as denied, json_extract(?, '$.ownerRetired') as retired, json_extract(?, '$.readyCut') as ready").get(...Array(4).fill(JSON.stringify(fixture.stagedRetirement)))).toEqual({input:73,denied:0,retired:1,ready:1});
 }finally{db.close();}
});
