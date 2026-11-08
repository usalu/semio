import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./metadata.schema.json";
import fixture from "./metadata.json";
/** 📏️ The original finite copy stock remains fixed across text metadata turns. */
test("original paged metadata stock",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ? = 4096 as admitted").get(fixture.perTurnCopy)).toEqual({admitted:1})}finally{db.close()}});
