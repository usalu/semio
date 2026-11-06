import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import cases from "./wide.json";
import schema from "./wide.schema.json";
test("wide clock deadlines compare without narrowing actual time",()=>{expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);const db=new Database(":memory:");try{for(const c of cases){const row=db.query("select (? > ? or (? = ? and (? > ? or (? = ? and ? >= ?)))) yielded").get(c.now[0],c.deadline[0],c.now[0],c.deadline[0],c.now[1],c.deadline[1],c.now[1],c.deadline[1],c.now[2],c.deadline[2]) as {yielded:number};expect(Boolean(row.yielded)).toBe(c.yield)}}finally{db.close()}});
