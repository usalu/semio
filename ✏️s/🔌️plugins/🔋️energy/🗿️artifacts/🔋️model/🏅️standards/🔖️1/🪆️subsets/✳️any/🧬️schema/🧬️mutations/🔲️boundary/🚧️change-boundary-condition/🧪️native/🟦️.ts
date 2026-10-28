import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import recipeSchema from "../../../✨️derive/🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";

test("surface boundary schema and SQL foreign keys preserve the real union and selected inverse",()=>{
 expect(new Ajv2020({strict:true}).compile(recipeSchema)(schema["x-semio-energy-operation"])).toBe(true);
 const valid=new Ajv({strict:false}).compile(schema), db=new Database(":memory:");
 db.run("PRAGMA foreign_keys=ON");db.run("CREATE TABLE surfaces(id INTEGER PRIMARY KEY,tag TEXT,partner INTEGER REFERENCES surfaces(id),CHECK((tag='Interzone')=(partner IS NOT NULL)),CHECK(partner IS NULL OR partner<>id))");
 for(const row of [law.original,...law.siblings])db.run("INSERT INTO surfaces VALUES(?,?,NULL)",row.id,"OutdoorAir");
 for(const example of law.cases){const p=example.payload;const original=db.query<{tag:string,partner:number|null},[number]>("SELECT tag,partner FROM surfaces WHERE id=?").get(p.id);const outcome=!original?"missing":!valid({mutation:"changeSurfaceBoundaryCondition",...p})?"invalid":p.newInterzoneSurfaceId!==null&&(p.id===p.newInterzoneSurfaceId||!db.query("SELECT id FROM surfaces WHERE id=?").get(p.newInterzoneSurfaceId))?"invalid-reference":original.tag===p.newBoundary&&original.partner===p.newInterzoneSurfaceId?"no-op":"changed";expect(outcome).toBe(example.outcome);
 if(outcome==="changed"){db.run("BEGIN");db.run("UPDATE surfaces SET tag=?,partner=? WHERE id=?",p.newBoundary,p.newInterzoneSurfaceId,p.id);expect(db.query("SELECT tag,partner FROM surfaces WHERE id=?").get(p.id)).toEqual({tag:example.after![0],partner:example.after![1]});db.run("UPDATE surfaces SET tag=?,partner=? WHERE id=?",original!.tag,original!.partner,p.id);expect(db.query("SELECT tag,partner FROM surfaces WHERE id=?").get(p.id)).toEqual({tag:law.originalValue[0],partner:law.originalValue[1]});db.run("ROLLBACK");}}
 db.close();
});
