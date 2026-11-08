import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./work-exact.schema.json";
import cases from "./work-exact.json";
/** 🧮️ Source work keeps its full aggregate while bounded fuel reaches zero. */
test("exact Source Work and overrun",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT ? + ? + ((? + ?) / 1000000000) AS hi, (? + ?) % 1000000000 AS lo, CASE WHEN ? > 0 OR ? >= ? THEN 0 ELSE ? - ? END AS remaining");for(const c of cases){const r=q.get(c.priorHi,c.deltaHi,c.priorLo,c.deltaLo,c.priorLo,c.deltaLo,c.deltaHi,c.deltaLo,c.fuel,c.fuel,c.deltaLo) as {hi:number,lo:number,remaining:number};expect(r).toEqual({hi:c.nextHi,lo:c.nextLo,remaining:c.remaining})}}finally{db.close()}});
