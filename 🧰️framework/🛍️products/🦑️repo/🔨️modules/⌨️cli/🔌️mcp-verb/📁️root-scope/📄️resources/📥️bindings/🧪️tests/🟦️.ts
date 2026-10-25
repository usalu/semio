import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("repository process and event ports retain captured context",()=>{
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);const contexts=new Map(fixture.contexts.map(row=>[row.id,Object.freeze({...row})]));const db=new Database(":memory:");try{db.run("CREATE TABLE contexts(id TEXT PRIMARY KEY, root TEXT, endpoint TEXT)");for(const row of fixture.contexts)db.run("INSERT INTO contexts VALUES(?,?,?)",[row.id,row.root,row.endpoint]);for(const operation of fixture.operations){const context=contexts.get(operation.context)!;const actual=operation.kind==="process"?context.root:context.endpoint;const oracle=db.query("SELECT CASE ?2 WHEN 'process' THEN root ELSE endpoint END AS value FROM contexts WHERE id=?1").get(operation.context,operation.kind) as {value:string};expect(actual).toBe(operation.expected);expect(oracle.value).toBe(operation.expected);expect(actual).not.toBe(operation.ambient);}}finally{db.close();}
});
