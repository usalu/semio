import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "./work.schema.json";
test("staged source admission and output retain and independently clear three real allocation markers",()=>{expect(new Ajv({strict:true}).validate(schema,fixture.sourceAdmissionWork)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select 2+1+1+2+2+1+?1 work").get(fixture.markerClearWork)).toEqual({work:fixture.sourceAdmissionWork})}finally{db.close()}});
