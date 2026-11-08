import { test, expect } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("persistent original rows and replacement law agree with SQLite",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of fixture.cases){const result=db.query("SELECT ?1*2 AS original, CASE WHEN ?1=?2 THEN ?3 ELSE ?1*2 END AS changed").get(row.index,row.index,row.authored) as {original:number;changed:number};expect(result.original).toBe(row.before);expect(result.changed).toBe(row.after);expect(row.unchangedReader).toBe(row.before)}expect(fixture.projectionAtomicDenial&&fixture.projectionSameRoot).toBe(true)}finally{db.close()}
});
