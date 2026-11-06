import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv from "ajv";

test("SQL work fixture preserves the same finite stock and serialized output",()=>{
 const fixture=JSON.parse(readFileSync(new URL("../../🧪️tests/work.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../../🧪️tests/work.schema.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE stage(rank INTEGER, units INTEGER, accepted INTEGER)");fixture.stages.forEach((units:number,index:number)=>db.run("INSERT INTO stage VALUES(?,?,?)",[index,units,Number(fixture.accepted[index])]));expect(db.query("SELECT SUM(units) AS actual FROM stage WHERE accepted=1").get()).toEqual({actual:fixture.actual});expect(fixture.actual).toBe(fixture.grant);expect(JSON.parse(JSON.stringify(fixture.output))).toEqual(fixture.output);}finally{db.close();}
});
