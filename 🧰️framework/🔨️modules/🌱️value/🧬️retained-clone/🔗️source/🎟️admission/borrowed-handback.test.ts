import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./borrowed-handback.schema.json";
import fixture from "./borrowed-handback.json";
test("borrowed handback keeps the original through denial",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE owner(value INTEGER, source INTEGER, output INTEGER)");db.run("INSERT INTO owner VALUES(?,1,0)",[fixture.value]);db.run("UPDATE owner SET source=1,output=0 WHERE ?=0",[fixture.deniedWork]);expect(db.query<{value:number,source:number,output:number},[]>("SELECT * FROM owner").get()).toEqual({value:fixture.value,source:1,output:0});db.run("UPDATE owner SET source=0,output=1");expect(db.query<{value:number,source:number,output:number},[]>("SELECT * FROM owner").get()).toEqual({value:fixture.value,source:0,output:1})}finally{db.close()}});
