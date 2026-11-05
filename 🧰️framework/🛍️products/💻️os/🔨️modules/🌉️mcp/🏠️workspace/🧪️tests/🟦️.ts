import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/folder-custody.json" with {type:"json"};
import fixture from "./🧫️fixtures/folder-custody.json" with {type:"json"};

test("folder custody vectors match independent SQLite guarded append and ownership",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 const db=new Database(":memory:");try{db.exec("create table events(seq integer primary key,pack text,spr text);create table owners(schema text,plugin text,app text)");
 for(const row of fixture.commits){
 const expected=row.expected;
 const result=db.query("insert into events(pack,spr) select ?,? where (? is null and not exists(select 1 from events)) or (? is not null and exists(select 1 from (select pack,spr from events order by seq desc limit 1) where pack=? and spr=?))").run(row.pack,row.spr,expected===null?null:1,expected===null?null:1,expected?.[0]??null,expected?.[1]??null);
 expect(result.changes===1,row.id).toBe(row.accepted);const latest=db.query<{pack:string;spr:string},[]>("select pack,spr from events order by seq desc limit 1").get()!;expect([latest.pack,latest.spr],row.id).toEqual(row.current);
 }
 for(const row of fixture.owners){db.exec("delete from owners");for(const declaration of row.declarations)db.query("insert into owners values(?,?,?)").run(...declaration as [string,string,string]);
 const result=db.query<{n:number},[string]>("select count(*) n from (select distinct plugin,app from owners where schema=?)").get(row.schema)!;expect(result.n===1,row.id).toBe(row.accepted);}
 }finally{db.close();}
});

import {readFileSync} from "node:fs";
import durabilitySchema from "../../../🏪️store/🔄️sync/🖥️native/🧪️tests/💾️durability/schema.json" with {type:"json"};

test("folder durability and latest-format vectors match independent SQLite transactions",()=>{
 const read=(name:string)=>{const [header,...lines]=readFileSync(new URL("../../../🏪️store/🔄️sync/🖥️native/🧪️tests/💾️durability/"+name+".csv",import.meta.url),"utf8").trimEnd().split("\n");return lines.map(line=>Object.fromEntries(header!.split(",").map((key,index)=>[key,line.split(",")[index]!])));};
 const fixture={flush:read("flush"),archive:read("archive")},validate=new Ajv({strict:true}).compile(durabilitySchema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 const db=new Database(":memory:");try{db.exec("create table snapshots(seq integer primary key,root text,format text);create table failures(phase text)");
 for(const row of fixture.flush){db.exec("delete from snapshots;delete from failures;insert into snapshots(root,format) values('old','pair');begin;insert into snapshots(root,format) values('new','pair');rollback");db.query("insert into failures values(?)").run("original");if(row.persistent==="1")db.query("insert into failures values(?)").run("rollback");
 const current=db.query<{root:string},[]>("select root from snapshots order by seq desc limit 1").get()!,uncertain=db.query<{uncertain:number},[]>("select exists(select 1 from failures where phase='rollback') uncertain").get()!;expect(current.root,row.id).toBe(row.current);expect(String(uncertain.uncertain),row.id).toBe(row.uncertain);}
 db.exec("delete from snapshots");for(const row of fixture.archive){db.query("insert into snapshots(root,format) values(?,?)").run(row.root,row.format);const current=db.query<{root:string;recursive:number},[]>("select root,case format when 'archive' then 1 else 0 end recursive from snapshots order by seq desc limit 1").get()!;expect(current.root,row.id).toBe(row.current);expect(String(current.recursive),row.id).toBe(row.recursive);}
 }finally{db.close();}
});
