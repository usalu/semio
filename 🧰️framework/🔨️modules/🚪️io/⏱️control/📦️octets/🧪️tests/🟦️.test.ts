import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import fixture from "./📦️allocation.json";
import schema from "./📦️allocation.schema.json";
test("original backing distinguishes denial, allocator failure and physical birth",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{const query=db.query("SELECT CASE ? WHEN ? THEN ? WHEN ? THEN ? ELSE ? END AS actual, CASE WHEN ? = ? THEN 1 ELSE 0 END AS output, CASE WHEN ? = ? THEN 0 ELSE 1 END AS work");for(const row of fixture.cases){const result=query.get(row.phase,"denied","none","allocator-failed","zero","positive",row.phase,"allocated",row.phase,"denied") as {actual:string;output:number;work:number};expect(result).toEqual({actual:row.actual,output:Number(row.output),work:row.work})}}finally{db.close()}});
