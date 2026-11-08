import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./native-field.schema.json";
import fixture from "./native-field.json";
test("native field witness selects the original second field",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE source(left_value INTEGER,right_value INTEGER)");db.run("INSERT INTO source VALUES(?,?)",[fixture.left,fixture.right]);expect(db.query<{right_value:number},[]>("SELECT right_value FROM source").get()!.right_value).toBe(fixture.selected)}finally{db.close()}});
