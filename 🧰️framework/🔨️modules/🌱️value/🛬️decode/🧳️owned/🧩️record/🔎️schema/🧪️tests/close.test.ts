import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../📐️close.json";
import schema from "../🔣️close.json";

test("matcher frame waits for a separately funded close",()=>{const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,denied:"pending"})).toBe(false);const db=new Database(":memory:");try{expect(db.query("select ? as denied, ? as work, ? as cleared").get(fixture.denied,fixture.deniedWork,fixture.cleared)).toEqual({denied:"completedNoProgress",work:0,cleared:"backingFrame"})}finally{db.close()}});
