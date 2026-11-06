import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import law from "../📐️.json";
import schema from "../🔣️.json";
test("ranked scan CPU quotes match independent recursive SQLite rank visits",()=>{expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{for(const row of law.scanCases){const result=db.query("WITH RECURSIVE ranks(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM ranks WHERE n<?1) SELECT count(*) AS work FROM ranks").get(row.remainingRanks) as {work:number};expect(result.work).toBe(row.selectedWork);}}finally{db.close();}});
