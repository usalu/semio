import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./host-admission.schema.json";
import cases from "./host-admission.json";
/** 🎟️ A host frame is admitted only when its original work and physical grant fit. */
test("host clock admission respects the real frame grant",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const result=db.query("SELECT ? > 0 AND ? >= ? AND ? > 0 AS admitted").get(row.items,row.capacity,row.required,row.work) as {admitted:number};expect(Boolean(result.admitted)).toBe(row.admitted)}}finally{db.close()}});
