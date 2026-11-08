import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./work-demand.schema.json";
import cases from "./work-demand.json";
/** ⏱️ The original host quote must fit real Work and physical capacity before its leaf runs. */
test("recorded clock demand is admitted before producer entry",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const found=db.query("SELECT ? >= CASE WHEN ? THEN ? ELSE ? END AND ? >= ? AND ? <= ? AS admitted").get(row.fuel,Number(row.opened),row.nextWork,row.work,row.capacity,row.requested,row.nextWork,row.work) as {admitted:number};expect(Boolean(found.admitted)).toBe(row.admitted)}}finally{db.close()}});
