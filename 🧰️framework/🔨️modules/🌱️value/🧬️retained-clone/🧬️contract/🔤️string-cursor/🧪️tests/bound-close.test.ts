import {expect,test} from "bun:test";
import Ajv2020 from "ajv";
import {Database} from "bun:sqlite";
import schema from "./bound-close.schema.json";
import fixture from "./bound-close.json";
test("bound alias copy admission matches SQLite",()=>{expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{for(const row of fixture.cases){const result=db.query<{admitted:number},[number,number]>("SELECT ?1 >= ?2 AS admitted").get(row.copy,row.required)!;expect(Boolean(result.admitted)).toBe(row.accepted)}}finally{db.close()}});
