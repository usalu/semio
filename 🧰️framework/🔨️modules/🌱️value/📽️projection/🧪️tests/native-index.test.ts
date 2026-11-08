import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./native-index.schema.json";
import fixture from "./native-index.json";
test("native index selects the exact retained array element",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE item(position INTEGER PRIMARY KEY,value INTEGER)");db.run("INSERT INTO item VALUES(0,?),(1,?)",[fixture.left,fixture.right]);expect(db.query<{value:number},[number]>("SELECT value FROM item WHERE position=?").get(fixture.index)!.value).toBe(fixture.selected);expect(db.query("SELECT value FROM item WHERE position=?").get(fixture.invalid)).toBeNull()}finally{db.close()}});
