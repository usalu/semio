import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./array-projection.schema.json";
import fixture from "./array-projection.json";
test("the retained array spans bounded original turns",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE source(value INTEGER,n INTEGER)");db.run("INSERT INTO source VALUES(?,?)",[fixture.value,fixture.length]);const row=db.query<{value:number;n:number},[]>("SELECT value,n FROM source").get()!;expect(row.value).toBe(fixture.value);expect(row.n).toBe(8192);expect(Math.ceil(row.n/fixture.maximumCopy)).toBe(2);expect(fixture.maximumWork).toBe(4096)}finally{db.close()}});
