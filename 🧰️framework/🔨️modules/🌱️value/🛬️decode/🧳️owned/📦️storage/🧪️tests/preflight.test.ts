import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../📐️.json";
import schema from "../🔣️.json";

test("unstarted storage denial settles before a larger funded turn",()=>{const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,preflight:"pending"})).toBe(false);expect(validate({...fixture,releaseFields:[]})).toBe(false);expect(validate({...fixture,moveFields:[]})).toBe(false);const db=new Database(":memory:");try{expect(db.query("select ? as outcome, ? as work").get(fixture.preflight,fixture.deniedWork)).toEqual({outcome:"completedNoProgress",work:0})}finally{db.close()}});
