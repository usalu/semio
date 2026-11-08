import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./close-stages.schema.json";
import cases from "./close-stages.json";
/** ♻️ Source close advances only after its genuine stage and Work admission. */
test("Source close stage and Work settlement",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT CASE WHEN ?=0 THEN ? WHEN ?='lending' THEN 'original' WHEN ?='original' THEN 'owner' WHEN ?='owner' THEN 'emptyOwner' WHEN ?='emptyOwner' THEN 'roots' WHEN ?='roots' THEN 'retiring' ELSE ? END AS next, CASE WHEN ?=0 THEN 0 ELSE ? END AS spent");for(const c of cases){const row=q.get(+c.admitted,c.stage,c.stage,c.stage,c.stage,c.stage,c.stage,c.stage,+c.admitted,c.work) as {next:string,spent:number};expect(row).toEqual({next:c.next,spent:c.spent})}}finally{db.close()}});
