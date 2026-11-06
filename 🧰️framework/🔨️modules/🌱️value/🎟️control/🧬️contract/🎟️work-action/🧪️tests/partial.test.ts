import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./partial.schema.json";
import fixture from "./partial.json";
/** ⏱️ Partial actual work is invoiced once while the original ceiling remains unchanged. */
test("original work retains its reservation across pending turns",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE paid(delta INTEGER NOT NULL)");for(const row of fixture.cases){db.exec("DELETE FROM paid");for(const delta of row.partials)db.query("INSERT INTO paid VALUES(?)").run(delta);const paid=Number(db.query("SELECT COALESCE(SUM(delta),0) AS n FROM paid").get().n);const valid=row.final>=paid;if(valid)db.query("INSERT INTO paid VALUES(?)").run(row.final-paid);const total=Number(db.query("SELECT COALESCE(SUM(delta),0) AS n FROM paid").get().n);expect(valid).toBe(row.valid);expect(total).toBe(row.expectedActual);expect(valid&&row.final<=row.reserved).toBe(row.expectedAdmitted)}}finally{db.close()}});
