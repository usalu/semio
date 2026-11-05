import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv/dist/2020";
import { Database } from "bun:sqlite";
test("VM callback scopes unwind and never authorize another thread", () => {
 const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/scope.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../🧬️schema/scope.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");db.exec("CREATE TABLE scope (depth INTEGER)");try{
  for(const row of fixture.cases){let depth=0;const own:boolean[]=[],oracle:boolean[]=[];db.exec("DELETE FROM scope");
   for(const step of row.steps){if(step==="enter"){depth++;db.run("INSERT INTO scope VALUES (?)",[depth]);}else if(step==="leave"){db.exec("DELETE FROM scope WHERE depth=(SELECT MAX(depth) FROM scope)");depth--;}else if(step==="unwind"){depth=0;db.exec("DELETE FROM scope");}else if(step==="foreignThread"){const other=new Database(":memory:");try{expect(other.query("SELECT 0 AS active").get()).toEqual({active:0});}finally{other.close();}}else{own.push(depth>0);oracle.push((db.query("SELECT COUNT(*) AS count FROM scope").get() as {count:number}).count>0);}}
   expect(own).toEqual(row.allowed);expect(oracle).toEqual(row.allowed);
  }
 }finally{db.close();}
});
