import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";

test("original identity mutations agree with independent SQLite sparse updates",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE identity_value(id INTEGER PRIMARY KEY,value TEXT NOT NULL)");
  for(const [id,row]of law.cases.entries()){
   database.query("INSERT INTO identity_value VALUES (?,?)").run(id,row.before);
   const whitespace=/^[\u0009-\u000d\u0020\u0085\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000]*$/u;
   const outcome=whitespace.test(row.value)?"refused":row.value===row.before?"no-op":"changed";
   expect(outcome).toBe(row.outcome);
   if(outcome==="changed")database.query("UPDATE identity_value SET value=? WHERE id=?").run(row.value,id);
   const after=database.query("SELECT value FROM identity_value WHERE id=?").get(id)as {value:string};
   expect(after.value).toBe(row.after);
   if(outcome==="changed")database.query("UPDATE identity_value SET value=? WHERE id=?").run(row.before,id);
   expect((database.query("SELECT value FROM identity_value WHERE id=?").get(id)as {value:string}).value).toBe(row.before);
  }
 }finally{database.close();}
});
