import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./work-narrow.schema.json";
import fixture from "./work-narrow.json";
/** ⏱️ Child work is limited by the actual remaining original payer. */
test("nested work cannot reuse a spent parent budget",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{for(const row of fixture.cases){const result=db.query("SELECT ? <= ? AND ? <= ? - ? AS admitted").get(row.spent,row.parent,row.child,row.parent,row.spent);expect(Boolean(result.admitted)).toBe(row.admitted)}}finally{db.close()}});

/** 🧮️ SQLite independently verifies remaining capacity after actual work. */
test("remaining work is the exact live ceiling",()=>{const db=new Database(":memory:");try{for(const row of fixture.cases){const result=db.query("SELECT CASE WHEN ? <= ? THEN ? - ? ELSE NULL END AS remaining").get(row.spent,row.parent,row.parent,row.spent) as {remaining:number|null};expect(result.remaining).toBe(row.spent<=row.parent?row.parent-row.spent:null)}}finally{db.close()}});

/** 📏️ The original receipt can narrow a larger local effect window. */
test("remaining work respects both original and local ceilings",()=>{const db=new Database(":memory:");try{for(const row of fixture.cases){const result=db.query("SELECT CASE WHEN ? <= ? AND ? + ? <= ? THEN MIN(? - ?, ? - ? - ?) ELSE NULL END AS remaining").get(row.spent,row.parent,row.base,row.spent,row.receipt,row.parent,row.spent,row.receipt,row.base,row.spent) as {remaining:number|null};expect(result.remaining).toBe(row.remaining)}}finally{db.close()}});
