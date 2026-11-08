import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./overrun.schema.json";
import fixture from "./overrun.json";
/** 🧾️ A reported source overrun leaves its original error and override independently owned. */
test("paged source overrun retains its original cause",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ? > ? as overrun, ? as items, ? as bytes, ? <= 4096 as funded").get(fixture.reportedWork,fixture.bodyQuote,fixture.originalItems,fixture.originalBytes,fixture.maximumCopy)).toEqual({overrun:1,items:7,bytes:13,funded:1})}finally{db.close()}});
