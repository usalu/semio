/** ♻️ Independent SQLite preserves each original error string and target backing. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../../../🧬️schema/🔣️.json";
test("native MutationApplyError retirement conserves every original message and target allocation",()=>{
 const validate=new Ajv({strict:true}).compile({...schema,$ref:"#/$defs/MutationApplyError"}),db=new Database(":memory:");db.exec("CREATE TABLE owners(ordinal INTEGER PRIMARY KEY,capacity INTEGER,live INTEGER)");
 try{for(const row of fixture.cases){expect(validate(row.wire)).toBe(true);const projected=db.query("SELECT json_object('code',?,'message',?,'target',json(?)) AS wire").get(row.wire.code,row.wire.message,JSON.stringify(row.wire.target)) as {wire:string};expect(JSON.parse(projected.wire)).toEqual(row.wire);
  db.exec("DELETE FROM owners");row.capacities.forEach((capacity,index)=>db.query("INSERT INTO owners VALUES(?,?,1)").run(index,capacity));const original=row.capacities.reduce((a,b)=>a+b,0);expect((db.query("SELECT SUM(capacity) AS bytes FROM owners").get() as {bytes:number}).bytes).toBe(original);
  let released=0;for(let index=0;index<row.capacities.length;index++){const capacity=row.capacities[index];for(const grant of [0,capacity-1]){expect(grant<capacity).toBe(true);expect((db.query("SELECT SUM(capacity) AS bytes FROM owners WHERE live=1").get() as {bytes:number}).bytes).toBe(original-released);}db.query("UPDATE owners SET live=0 WHERE ordinal=?").run(index);released+=capacity;}expect(released).toBe(original);
 }}finally{db.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("artifact_retire_struct!(super::MutationApplyError");expect(source).toContain("code, message, target");console.log("[DEBUG] MutationApplyError SQLite per-field original capacity and semantic JSON output agree with independent Ajv");
});
