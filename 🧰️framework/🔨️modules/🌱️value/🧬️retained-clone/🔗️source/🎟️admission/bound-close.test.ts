import {expect,test} from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import law from "./bound-close.json";
import schema from "./bound-close.schema.json";
test("borrowed alias return has one native work and no allocation on its strong parent",()=>{expect(new Ajv2020({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE event(work INTEGER,allocated INTEGER,released INTEGER,receive INTEGER,recover INTEGER)");db.run("INSERT INTO event VALUES(?,?,?,?,?)",[law.work,law.allocations,law.releases,law.receives,law.recovers]);expect(db.query("SELECT work,allocated,released,receive+recover AS calls FROM event").get()).toEqual({work:1,allocated:0,released:0,calls:1});expect(db.query("SELECT ?+? AS denied").get(law.deniedWork,law.foreignWork)).toEqual({denied:0});}finally{db.close()}});
