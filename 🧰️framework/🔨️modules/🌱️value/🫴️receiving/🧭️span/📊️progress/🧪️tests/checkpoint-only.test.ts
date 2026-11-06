import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import cases from "./checkpoint-only.json";
import schema from "./checkpoint-only.schema.json";
test("the original checkpoint settles without a span",()=>{expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);const db=new Database(":memory:");try{for(const c of cases){const row=db.query("select case when not ? then 'reject' when ? <> 'none' then ? when ? then 'recover' else 'receive' end next, 0 spans").get(Number(c.same),c.reply,c.reply,Number(c.entered)) as {next:string,spans:number};expect(row).toEqual({next:c.next,spans:c.spans})}}finally{db.close()}});
