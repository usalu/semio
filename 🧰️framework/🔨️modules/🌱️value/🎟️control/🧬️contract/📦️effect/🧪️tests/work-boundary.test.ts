import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import cases from "./work-boundary.json";
import schema from "./work-boundary.schema.json";
test("a work boundary measures only the same receipt",()=>{expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);const db=new Database(":memory:");try{for(const c of cases){const row=db.query("select ? and ? >= ? allowed, case when ? and ? >= ? then ? - ? else null end delta").get(Number(c.same),c.end,c.start,Number(c.same),c.end,c.start,c.end,c.start) as {allowed:number,delta:number|null};expect(Boolean(row.allowed)).toBe(c.allowed);expect(row.delta).toBe(c.delta)}}finally{db.close()}});
