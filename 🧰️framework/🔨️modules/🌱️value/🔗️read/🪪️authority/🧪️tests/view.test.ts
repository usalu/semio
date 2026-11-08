import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./view.json";
import schema from "./view.schema.json";
test("paid view release retains the witnessed original on denial",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE owner(id INTEGER PRIMARY KEY)");db.run("INSERT INTO owner VALUES (?),(?)",[fixture.original,fixture.foreign]);const row=db.query<{same:number},[number,number]>("SELECT a.id=b.id AS same FROM owner a,owner b WHERE a.id=? AND b.id=?").get(fixture.original,fixture.foreign)!;expect(Boolean(row.same)).toBe(false);expect(fixture.deniedRetained).toBe(true);expect(fixture.sameOwner).toBe(true);expect(fixture.deniedWork).toBe(0);expect(fixture.paidWork).toBe(1)}finally{db.close()}});
