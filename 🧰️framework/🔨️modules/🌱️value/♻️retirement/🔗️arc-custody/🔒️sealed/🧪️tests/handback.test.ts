import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./handback.schema.json";
import cases from "./handback.json";
/** 📤️ A paid unique handback retains the same owner on every refusal. */
test("original handback selects only funded, unoccupied and physically unique owners",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const got=db.query("SELECT (? > 0 AND ? >= ? AND ? >= ? AND ? = 0) AS entered, (? > 0 AND ? >= ? AND ? >= ? AND ? = 0 AND ? = 0) AS transferred").get(row.items,row.copy,row.required,row.work,row.requiredWork,Number(row.occupied),row.items,row.copy,row.required,row.work,row.requiredWork,Number(row.occupied),row.aliases) as {entered:number,transferred:number};expect(Boolean(got.entered)).toBe(row.entered);expect(Boolean(got.transferred)).toBe(row.transferred)}}finally{db.close()}});
