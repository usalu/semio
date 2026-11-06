import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import cases from "./producer-outcome.json";
import schema from "./producer-outcome.schema.json";
test("actual producer receipt preserves the original physical marker and refusal",()=>{expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);const db=new Database(":memory:");try{for(const c of cases){const row=db.query("select coalesce(?,?) submitted, case when ? is null then ? <> 0 else ? <> ? end mismatch").get(c.actual,c.capacity,c.actual,c.capacity,c.actual,c.capacity) as {submitted:number,mismatch:number};expect(row.submitted).toBe(c.submitted);expect(Boolean(row.mismatch)).toBe(c.mismatch);if(c.actual!==null)expect(c.performed).toBe(true);expect(c.primary).not.toBeNull()}}finally{db.close()}});
