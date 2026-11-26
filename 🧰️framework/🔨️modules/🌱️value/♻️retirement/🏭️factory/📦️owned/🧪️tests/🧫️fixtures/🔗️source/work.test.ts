import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "./work.schema.json";
test("staged source admission records the measured work of its reply-shaped turns",()=>{expect(new Ajv({strict:true}).validate(schema,fixture.sourceAdmissionWork)).toBe(true);expect((fixture as Record<string,unknown>).markerClearWork).toBeUndefined();const db=new Database(":memory:");try{expect(db.query("select ? work").get(fixture.sourceAdmissionWork)).toEqual({work:37})}finally{db.close()}});
