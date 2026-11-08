import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./array-projection-owned.schema.json";
import fixture from "./array-projection-owned.json";
test("non-Copy array elements keep their original order",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE item(position INTEGER PRIMARY KEY,value TEXT NOT NULL)");for(const [position,value] of fixture.values.entries())db.run("INSERT INTO item VALUES(?,?)",[position,value]);expect(db.query<{value:string},[]>("SELECT value FROM item ORDER BY position").all().map(row=>row.value)).toEqual(fixture.values)}finally{db.close()}});
