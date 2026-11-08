import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./source-payout.schema.json";
import cases from "./source-payout.json";
/** 🧾️ A Source payout stays hidden until its exact final physical reply is accepted. */
test("same Source receipt payout publishes only after acceptance",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases.cases){const result=db.query("select ? and ? and ? and (? = 'accepted') as published").get(Number(row.matching),Number(row.refused),Number(row.free),row.post) as {published:number};expect(Boolean(result.published)).toBe(row.published)}db.run("create table actions(name text primary key,work integer,posts integer)");for(const row of cases.actions)db.run("insert into actions values(?,?,?)",row.name,row.work,row.posts);expect(db.query("select count(*) bad from actions where work != case when name = ? then 5 else 1 end or posts != 1").get("semantic")).toEqual({bad:0});expect(db.query("select sum(work) totalWork,sum(posts) totalPosts from actions").get()).toEqual({totalWork:cases.totalWork,totalPosts:cases.totalPosts})}finally{db.close()}});
