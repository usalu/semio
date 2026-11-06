import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./schema.json";
import law from "./fixture.json";
test("registry selected page work agrees with SQLite ownership",()=>{expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select 0 denied, case when ? then 1 else 0 end release, case when ? then 1 else 0 end empty").get(1,0)).toEqual(law)}finally{db.close()}});
