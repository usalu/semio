/** 🔐️ Checks the closed detached-recipient policy against independent SQLite and JSON oracles. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("Original local continuation keeps independent authority and exact semantic backing",()=>{
 const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const key of Object.keys(fixture)){const missing={...fixture} as Record<string,unknown>;delete missing[key];expect(validate(missing)).toBe(false);}
 expect(validate({...fixture,replacement:"accept"})).toBe(false);
 const bytes=Buffer.byteLength(JSON.parse(JSON.stringify(fixture.text)),"utf8");expect(bytes).toBe(fixture.textBytes);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE receipt(sequence INTEGER PRIMARY KEY,bytes INTEGER NOT NULL)");db.query("INSERT INTO receipt VALUES(?,?)").run(1,fixture.initialBytes);db.query("INSERT INTO receipt VALUES(?,?)").run(2,bytes);expect(db.query("SELECT SUM(bytes) FROM receipt").values()).toEqual([[fixture.initialBytes+fixture.textBytes]]);}finally{db.close();}
 expect(fixture.deniedGrant).toEqual({maximumItems:0,maximumCopyBytes:0,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:0});
 console.error("[DEBUG] Original local continuation closed policy and UTF8 cumulative receipt compared with independent Ajv, SQLite, Buffer and JSON; Native proof executes separately");
});
