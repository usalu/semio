import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./reply.schema.json";
import cases from "./reply.json";
/** 🧷️ A retained alias recovers only its entered original invocation. */
test("alias reply recovery",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND NOT ? AND ?) AS recover");for(const c of cases){const r=q.get(+c.entered,+c.recorded,+c.same) as {recover:number};expect(Boolean(r.recover)).toBe(c.recover)}}finally{db.close()}});
