import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";

test("Mounted command retains operation before phased original lease admission",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE original (operation INTEGER PRIMARY KEY, source INTEGER NOT NULL, phases INTEGER NOT NULL)");
  for(const row of fixture.cases){database.exec("DELETE FROM original");database.query("INSERT INTO original VALUES (?,?,0)").run(row.operation,row.source);for(const turn of row.turns)database.query("UPDATE original SET phases = phases + 1 WHERE operation = ? AND source = ? AND ? AND phases < 7").run(row.operation,turn.source,Number(turn.admitted));const result=database.query("SELECT operation, phases, phases = 7 AS published FROM original").get() as {operation:number,phases:number,published:number};expect(result.operation).toBe(row.operation);expect(result.phases).toBe(row.expectedPhases);expect(Boolean(result.published)).toBe(row.expectedPublished);}
  const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
  expect(source).toContain("fn admit_original_lease(");
  const start=source.indexOf("fn admit_original_lease(");const body=source.slice(start,source.indexOf("fn key_demands(",start));
  expect(body).toContain("self.operation.operation");expect(body).toContain("self.source_identity");expect(body).toContain("issuer.begin_keyed(&key,cx)");expect(body).not.toContain("allocate_operation_id");
 }finally{database.close();}
});
