import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🧫️cleanup.json";
import schema from "./🧬️cleanup.json";

test("retirement authorizes only the unaliased cleanup export on its actual Source",()=>{
 const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 expect(validate(corpus)).toBe(true);
 db.exec("create table exports(name text primary key)");
 for(const row of corpus.cases){db.exec("delete from exports");for(const name of row.exports)db.query("insert into exports values(?)").run(name);const result=db.query("select (? and ? and exists(select 1 from exports where name in ('Deallocate','CloseActorOutput')) and not exists(select 1 from exports where name not in ('Deallocate','CloseActorOutput'))) allowed").get(row.loan?1:0,row.sameSource?1:0) as {allowed:number};expect(Boolean(result.allowed),row.name).toBe(row.allowed)}
 db.close();
});
