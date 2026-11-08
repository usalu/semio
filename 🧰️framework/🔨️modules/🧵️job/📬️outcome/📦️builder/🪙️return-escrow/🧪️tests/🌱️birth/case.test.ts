import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./case.schema.json";
import rows from "./case.json";
/** 🗃️ The same Source funds each original escrow birth phase before any mutation. */
test("resident escrow birth requires the original funding and empty destination",()=>{expect(new Ajv({strict:true}).compile(schema)(rows)).toBe(true);const db=new Database(":memory:");try{for(const row of rows){const v=db.query("select (? and ? and (? != ? or not ?)) as performed").get(+row.same,+row.funded,row.phase,"publish",+row.slot) as {performed:number};expect(Boolean(v.performed)).toBe(row.performed)}}finally{db.close()}});
