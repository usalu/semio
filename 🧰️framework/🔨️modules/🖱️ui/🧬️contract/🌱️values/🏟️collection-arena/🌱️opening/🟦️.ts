import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️.json";
test("native UI arena ownership permits detach only after every actual descendant and borrow returns",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");
 for(const row of corpus.registrationCases){const actual=db.query("select (?=0 and ?=0 and ?=0 and ?=0) permitted").get(row.borrowers,row.collections,row.pages,row.handbacks)as{permitted:number};expect(Boolean(actual.permitted)).toBe(row.detach);}
 db.exec("create table transitions(position integer primary key,name text unique not null)");for(const [index,name]of corpus.opening.entries())db.query("insert into transitions values (?,?)").run(index,name);expect(db.query("select name from transitions order by position").all()).toEqual(corpus.opening.map(name=>({name})));db.close();
});
