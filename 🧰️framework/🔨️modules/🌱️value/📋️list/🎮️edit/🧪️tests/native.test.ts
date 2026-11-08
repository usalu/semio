import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../📐️.json";
import schema from "../🔣️.json";
test("paid list insertion retains its item through a denied turn",()=>{const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,denied:"pending"})).toBe(false);const db=new Database(":memory:");try{db.run("create table rows(rank integer,value integer)");fixture.input.forEach((value,rank)=>db.run("insert into rows values(?,?)",[rank,value]));db.run("update rows set rank=rank+1 where rank>=?",[fixture.index]);db.run("insert into rows values(?,?)",[fixture.index,fixture.insert]);expect(db.query("select value from rows order by rank").all().map((row:any)=>row.value)).toEqual(fixture.expected)}finally{db.close()}});
