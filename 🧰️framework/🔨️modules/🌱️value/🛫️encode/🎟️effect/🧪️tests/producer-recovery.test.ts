import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import cases from "./producer-recovery.json";
import schema from "./producer-recovery.schema.json";
test("same original producer recovers its recorded reply and actual work",()=>{expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);const db=new Database(":memory:");try{for(const c of cases){const row=db.query("select case when not ? then 'reject' when ? <> 'none' then 'consume' when ? then 'recover' else 'receive' end next, ? > ? overrun").get(Number(c.sameAction),c.reply,Number(c.entered),c.work,c.ceiling) as {next:string,overrun:number};expect(row.next).toBe(c.next);expect(Boolean(row.overrun)).toBe(c.overrun)}}finally{db.close()}});
