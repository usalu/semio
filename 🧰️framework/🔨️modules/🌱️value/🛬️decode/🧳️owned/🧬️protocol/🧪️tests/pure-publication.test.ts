import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../📐️publication.json";
import schema from "../🔣️publication.json";

test("denied publication retains the original until a funded turn",()=>{const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,denied:"pending"})).toBe(false);const db=new Database(":memory:");try{expect(db.query("select ? as denied, ? as work, ? as published").get(fixture.denied,fixture.deniedWork,fixture.published)).toEqual({denied:"completedNoProgress",work:0,published:"complete"})}finally{db.close()}});
