import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import recipeSchema from "../../../✨️derive/🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("setpoint kind schema and SQL retain the exact original typed tag and four limits",()=>{
 expect(new Ajv2020({strict:true}).compile(recipeSchema)(schema["x-semio-energy-operation"])).toBe(true);
 const valid=new Ajv({strict:false,strictNumbers:true}).compile(schema),db=new Database(":memory:");
 db.run("CREATE TABLE controls(id INTEGER PRIMARY KEY,tag TEXT,lo REAL,hi REAL,low_setpoint REAL,high_setpoint REAL,CHECK((tag='OutdoorAirReset' AND lo<hi) OR (tag<>'OutdoorAirReset' AND lo=0 AND hi=0 AND low_setpoint=0 AND high_setpoint=0)))");
 db.run("INSERT INTO controls VALUES(?,?,?,?,?,?)",law.original.id,law.originalValue[0],...law.originalValue[1]);
 for(const c of law.cases){const p=c.payload,old=db.query<{tag:string,lo:number,hi:number,low_setpoint:number,high_setpoint:number},[number]>("SELECT tag,lo,hi,low_setpoint,high_setpoint FROM controls WHERE id=?").get(p.id);const limits=[p.newLowOutdoorC,p.newHighOutdoorC,p.newLowSetpointC,p.newHighSetpointC];const accepted=valid({mutation:"replaceSetpointManagerKind",...p})&&(p.newKind==="OutdoorAirReset"?limits[0]<limits[1]:limits.every(v=>v===0));const unchanged=old&&old.tag===p.newKind&&[old.lo,old.hi,old.low_setpoint,old.high_setpoint].every((v,i)=>v===limits[i]);expect(!old?"missing":!accepted?"invalid":unchanged?"no-op":"changed").toBe(c.outcome);if(c.outcome==="changed"){db.run("BEGIN");db.run("UPDATE controls SET tag=?,lo=?,hi=?,low_setpoint=?,high_setpoint=? WHERE id=?",p.newKind,...limits,p.id);const row=db.query<{tag:string,lo:number,hi:number,low_setpoint:number,high_setpoint:number},[number]>("SELECT tag,lo,hi,low_setpoint,high_setpoint FROM controls WHERE id=?").get(p.id)!;expect([row.tag,[row.lo,row.hi,row.low_setpoint,row.high_setpoint]]).toEqual(c.after);db.run("UPDATE controls SET tag=?,lo=?,hi=?,low_setpoint=?,high_setpoint=? WHERE id=?",old!.tag,old!.lo,old!.hi,old!.low_setpoint,old!.high_setpoint,p.id);expect(db.query("SELECT tag,lo,hi,low_setpoint,high_setpoint FROM controls WHERE id=?").get(p.id)).toEqual(old);db.run("ROLLBACK");}}
 expect(valid({mutation:"replaceSetpointManagerKind",...law.cases[0].payload,newKind:"Sunny"})).toBe(false);db.close();
});
