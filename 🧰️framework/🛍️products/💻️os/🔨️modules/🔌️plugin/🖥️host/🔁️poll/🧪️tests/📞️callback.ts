import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import corpus from"./🧫️callback.json";
import schema from"./🧬️callback.json";
test("funded callback failure remains owned and cannot invoke a second time",()=>{
 const db=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema);expect(validate(corpus)).toBe(true);db.exec("create table events(name text,position integer)");
 for(const row of corpus.cases){db.exec("delete from events");for(const[index,name]of row.events.entries())db.query("insert into events values(?,?)").run(name,index);const result=db.query("select ((select count(*) from events where name='callback')=1 and (select position from events where name='residentClosed')<(select position from events where name='callback') and (select position from events where name='captureRetired')<(select position from events where name='sourceClosed') and (not exists(select 1 from events where name='callbackFailed') or (select position from events where name='callbackFailed')<(select position from events where name='failureRetired') and (select position from events where name='failureRetired')<(select position from events where name='sourceClosed'))) valid").get()as{valid:number|null};expect(Boolean(result.valid),row.name).toBe(row.valid)}db.close();
});
