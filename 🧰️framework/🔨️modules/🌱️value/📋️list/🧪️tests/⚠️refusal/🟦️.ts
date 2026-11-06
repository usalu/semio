/** ⚠️ Checks the native refusal corpus against an independent schema and SQL classification. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {semioSchemaAjvV1} from "../../../../🧬️schema/📦️packages/🟦️typescript/🔮️oracles/✅️validator/🟦️.ts";
import {SqliteDatabaseV1} from "../../../../⚙️runtime/🗃️sqlite/🟦️.ts";

test("paged refusal corpus agrees with the independent SQLite oracle",()=>{
 const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/⚠️refusal/🔣️.json",import.meta.url),"utf8")) as {cases:{id:string;operation:string;expected:{kind:string;reason:string}}[]};
 const schema=JSON.parse(readFileSync(new URL("🧬️schema/🔣️.json",import.meta.url),"utf8")),validate=semioSchemaAjvV1().compile(schema);
 expect(validate(fixture)).toBe(true);
 expect(new Set(fixture.cases.map(row=>row.operation)).size).toBe(fixture.cases.length);
 const db=new SqliteDatabaseV1(":memory:");
 try{
  const query=db.query("SELECT CASE WHEN ? LIKE 'fail%' THEN 'allocationFailed' WHEN ? LIKE 'extra%' THEN 'invariantViolated' ELSE 'ownershipLimit' END AS kind");
  for(const row of fixture.cases)expect(query.get(row.operation,row.operation)).toEqual({kind:row.expected.kind});
 }finally{db.close();}
});
