import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./🧬️failure-retirement.json";
import corpus from "./🧫️failure-retirement.json";
test("failure owners prohibit false terminal retirement",()=>{expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");db.exec("create table held(name text primary key)");for(const row of corpus.cases){db.exec("delete from held");for(const name of row.held)db.query("insert into held values(?)").run(name);expect((db.query("select count(*)=0 empty from held").get() as {empty:number}).empty===1).toBe(row.empty)}for(const row of corpus.scalar){expect((db.query("select ? in ('cancelled','deadline') failed").get(row.status) as {failed:number}).failed===1).toBe(row.failed)}db.close()});
