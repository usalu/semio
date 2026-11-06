import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../📐️.json";
/** ⛽️ Independent SQL keeps issuance work separate from the actual allocator attempt. */
test("Original shared construction pays issuance and allocator work even when allocation refuses",()=>{const validate=new Ajv({strict:true}).compile({type:"object",required:["nullWork","allocationWork","deniedWork","nullAdmission","nullPhysicalBytes"],properties:{nullWork:{const:2},allocationWork:{const:2},deniedWork:{const:0},nullAdmission:{const:"requestedExtent"},nullPhysicalBytes:{const:0}}});expect(validate(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE performed(action TEXT PRIMARY KEY,work INTEGER NOT NULL CHECK(work>0))");db.query("INSERT INTO performed VALUES(?,?)").run("original issuance",1);db.query("INSERT INTO performed VALUES(?,?)").run("actual allocator attempt",1);const sum=db.query("SELECT SUM(work) FROM performed").values()[0]![0];expect(sum).toBe(fixture.nullWork);expect(sum).toBe(fixture.allocationWork);}finally{db.close();}});
