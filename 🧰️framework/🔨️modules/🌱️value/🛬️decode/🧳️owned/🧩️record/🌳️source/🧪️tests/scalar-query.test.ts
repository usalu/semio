import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../📐️scalar-query.json";
import schema from "../🔣️scalar-query.json";

test("source scalar reply survives unwind without replay",()=>{const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,calls:2})).toBe(false);expect(validate({...fixture,operation:"replaced"})).toBe(false);expect(validate({...fixture,destination:"callerSwap"})).toBe(false);expect(validate({...fixture,binding:"replacementSource"})).toBe(false);const db=new Database(":memory:");try{db.run("create table events(i integer primary key, event text)");fixture.events.forEach((event,i)=>db.run("insert into events values(?,?)",[i,event]));expect(db.query("select sum(event='receive') as calls, min(case when event='recordError' then i end)<min(case when event='unwind' then i end) as recordedBeforeUnwind, min(case when event='recoverRecorded' then i end)<min(case when event='recordOuter' then i end) and min(case when event='recordOuter' then i end)<min(case when event='transfer' then i end) as recoveredBeforeTransfer from events").get()).toEqual({calls:fixture.calls,recordedBeforeUnwind:1,recoveredBeforeTransfer:1})}finally{db.close()}});
