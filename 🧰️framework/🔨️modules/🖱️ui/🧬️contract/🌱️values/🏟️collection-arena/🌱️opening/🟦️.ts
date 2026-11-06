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
test("phase-only transitions and typed pointer/control writes have separate exact demands",()=>{const db=new Database(":memory:");for(const row of corpus.metadata){const actual=db.query("select 2*?+1 allocation,1 rowEnd,22*?+1 seal,?+1 take,4*? closeVector,2*? releaseBacking,? scan,?+1 scanEnd,?+3 recovery").get(...Array(8).fill(row.word));const{word,...expected}=row;expect(actual).toEqual(expected)}db.close()});
