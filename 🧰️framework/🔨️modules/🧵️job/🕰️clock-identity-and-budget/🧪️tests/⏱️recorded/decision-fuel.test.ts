import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./decision-fuel.schema.json";
import cases from "./decision-fuel.json";
/** ⛽️ Recovery retains the Source action while observing its actual remaining fuel. */
test("clock decision uses decreasing same-turn fuel",()=>{expect(new Ajv({strict:true,allowUnionTypes:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT ? <= ? AS valid, CASE WHEN ? > ? THEN NULL ELSE (? = 0 OR ?) END AS decision");for(const c of cases){const row=q.get(c.currentFuel,c.initialFuel,c.currentFuel,c.initialFuel,c.currentFuel,+c.expired) as {valid:number,decision:number|null};expect(Boolean(row.valid)).toBe(c.valid);expect(row.decision===null?null:Boolean(row.decision)).toBe(c.shouldYield)}}finally{db.close()}});
