import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import {classifyHistoryObservation} from "../🟦️.ts";

test("history lifecycle decisions match closed neutral cuts and independent SQLite",()=>{
 const validate=new Ajv({strict:false}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){const oracle=db.query("SELECT CASE WHEN json_extract(?1,'$.cancel')='before' THEN 'cancelled' WHEN json_extract(?1,'$.namespace')='foreign' OR json_extract(?1,'$.intent')<>'valid' OR json_extract(?1,'$.selection')='fork' THEN 'quarantined' WHEN json_extract(?1,'$.request')='rollover' THEN CASE WHEN json_extract(?1,'$.segments')>=4096 THEN 'history-full' WHEN json_extract(?1,'$.selection')='missing' THEN 'selection-required' ELSE 'segment-selected' END WHEN json_extract(?1,'$.request')='inspect' THEN CASE WHEN json_extract(?1,'$.names')>=4080 OR json_extract(?1,'$.bytes')>=66060288 THEN 'rollover-required' ELSE 'inspection' END WHEN json_extract(?1,'$.object') IN ('foreign','unknown') THEN 'quarantined' WHEN json_extract(?1,'$.outcome')='complete' THEN 'settled' WHEN json_extract(?1,'$.object')='expected' AND json_extract(?1,'$.source') IN ('absent','after') THEN 'committed-observed' WHEN json_extract(?1,'$.object')='expected' AND json_extract(?1,'$.source')='foreign' THEN 'conflict-retained' WHEN json_extract(?1,'$.object')='absent' AND json_extract(?1,'$.source')='before' THEN 'not-applied' ELSE 'quarantined' END AS outcome").get(JSON.stringify(row.input))as{outcome:string};expect(classifyHistoryObservation(row.input),row.name).toBe(row.expected);expect(oracle.outcome,row.name).toBe(row.expected);}}
 finally{db.close();}
});

import pagingSchema from "../🧬️schema/📈️paging.json" with {type:"json"};
import pagingFixture from "./📈️paging/🔣️.json" with {type:"json"};

test("history paging work matches neutral bounds and independent SQLite",()=>{
 const validate=new Ajv({strict:false}).compile(pagingSchema);expect(validate(pagingFixture),JSON.stringify(validate.errors)).toBe(true);const db=new Database(":memory:");
 try{for(const row of pagingFixture.cases){const remaining=row.segments*row.perSegment-row.offset;const model={entries:Math.min(64,remaining),proofs:Math.min(65,remaining),chainWalks:2,selectionReads:2*row.segments,next:remaining>64};const oracle=db.query("SELECT min(64,?1*?2-?3) AS entries,min(65,?1*?2-?3) AS proofs,2 AS chainWalks,2*?1 AS selectionReads,(?1*?2-?3)>64 AS next").get(row.segments,row.perSegment,row.offset)as typeof model;expect(model,row.name).toEqual(row.expected);expect({...oracle,next:Boolean(oracle.next)},row.name).toEqual(row.expected);}for(const row of pagingFixture.interference){const oracle=db.query("SELECT CASE WHEN ?1 IN ('Checking','Listing','Complete') AND ?2 IN ('object','outcome','reconciliation','selection','seal','ancestor') THEN 'refused' ELSE 'invalid' END AS outcome").get(row.phase,row.change)as{outcome:string};expect(oracle.outcome).toBe(row.expected);}}
 finally{db.close();}
});
