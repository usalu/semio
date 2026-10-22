import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("local dashboard failures preserve primary cleanup and bounded history under an independent SQLite oracle",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.terminalCases){const result=db.query("select coalesce(?1,case when ?3 then 'detach' when ?2 then 'leave' end) primaryCause,json_array(case when ?1 is not null and ?3 then 'detach' end,case when (?1 is not null or ?3) and ?2 then 'leave' end) cleanupCause,case when ?1 is not null or ?2 or ?3 then 1 else 0 end exitCode").get(row.fail,Number(row.leaveFails),Number(row.detachFails))as{primaryCause:string|null;cleanupCause:string;exitCode:number};expect({primary:result.primaryCause,cleanup:(JSON.parse(result.cleanupCause)as(string|null)[]).filter((cause)=>cause!==null),exit:result.exitCode}).toEqual(row.expected);}const history=db.query("select min(?1,?2) retained,max(0,?1-?2) first").get(fixture.history.count,fixture.maximumEvents)as{retained:number;first:number};expect(history).toEqual({retained:fixture.history.retained,first:fixture.history.first});const languages=db.query("select count(distinct json_extract(value,'$.language')) n from json_each(?)").get(JSON.stringify(fixture.locales))as{n:number};expect(languages.n).toBe(2);expect(new Ajv({strict:true}).validate(schema,{...fixture,unexpected:true})).toBe(false);}finally{db.close();}
});
