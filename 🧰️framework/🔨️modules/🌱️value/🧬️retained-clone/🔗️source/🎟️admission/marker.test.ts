import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import law from "./case.json";
import schema from "./case.schema.json";
import work from "../🎟️work/🧪️tests/🧫️fixtures/🔣️.json";

test("source allocation markers distinguish real extent, allocator null and no effect",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  const rows=db.query("SELECT p.value AS phase, CASE WHEN EXISTS(SELECT 1 FROM json_each(?) a WHERE a.value=p.value) THEN json_extract(?, '$.allocated') ELSE json_extract(?, '$.nonallocating') END AS marker FROM json_each(?) p ORDER BY p.key").all(JSON.stringify(law.allocationPhases),JSON.stringify(law.allocationMarker),JSON.stringify(law.allocationMarker),JSON.stringify(law.phases));
  expect(rows).toEqual(law.phases.map(phase=>({phase,marker:law.allocationPhases.includes(phase)?law.allocationMarker.allocated:law.allocationMarker.nonallocating})));
  expect(db.query("SELECT json_extract(?, '$.failed') AS failed, json_extract(?, '$.denied') AS denied").get(JSON.stringify(law.allocationMarker),JSON.stringify(law.allocationMarker))).toEqual({failed:0,denied:null});
 }finally{db.close();}
});

test("source work includes each actual issuance, allocator attempt and handoff",()=>{
 const db=new Database(":memory:");
 try{
  expect(db.query("SELECT sum(value) AS work FROM json_each(?)").get(JSON.stringify(work.birthActions))).toEqual({work:work.expected.birthCpu});
  expect(db.query("SELECT key AS phase, value AS work FROM json_each(?) ORDER BY key").all(JSON.stringify(law.nullWork))).toEqual([{phase:"lease",work:2},{phase:"owner",work:2},{phase:"payload",work:1}]);
 }finally{db.close();}
});
