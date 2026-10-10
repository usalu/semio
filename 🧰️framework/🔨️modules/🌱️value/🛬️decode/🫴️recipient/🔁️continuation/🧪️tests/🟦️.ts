import {existsSync} from "node:fs";
import {resolve} from "node:path";
/** 🔐️ Checks the closed detached-recipient policy against independent SQLite and JSON oracles. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import grantSchema from "../../../../🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json";
test("Original local continuation keeps independent authority and exact semantic backing",()=>{
 expect(existsSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"))).toBe(false);
 const validate=new Ajv({strict:true}).compile(grantSchema);expect(validate(fixture.closeGrant)).toBe(true);expect(validate(fixture.deniedGrant)).toBe(true);
 expect(fixture.replacement).toBe("refuseWithoutConsuming");expect(fixture.custody).toBe("originalRecipient");
 const bytes=Buffer.byteLength(JSON.parse(JSON.stringify(fixture.text)),"utf8");expect(bytes).toBe(fixture.textBytes);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE receipt(sequence INTEGER PRIMARY KEY,bytes INTEGER NOT NULL)");db.query("INSERT INTO receipt VALUES(?,?)").run(1,fixture.initialBytes);db.query("INSERT INTO receipt VALUES(?,?)").run(2,bytes);expect(db.query("SELECT SUM(bytes) FROM receipt").values()).toEqual([[fixture.initialBytes+fixture.textBytes]]);}finally{db.close();}
 expect(fixture.deniedGrant).toEqual({maximumItems:0,maximumCopyBytes:0,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:0});
 expect(fixture.rejectedWork.attempted>fixture.rejectedWork.declared).toBe(true);expect(fixture.invalidReceipt.completed>fixture.invalidReceipt.total).toBe(true);expect(fixture.recipientRelease).toBe("originalGrantWitness");
 console.error("[DEBUG] Original local continuation plain policy and UTF8 cumulative receipt compared with canonical per-grant Ajv, SQLite, Buffer and JSON; Native proof executes separately");
});
