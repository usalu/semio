import { test, expect } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import { Database } from "bun:sqlite";
import schema from "./schema.json";
import law from "./fixture.json";
test("failed child and failed work settlement stay distinct",()=>{
 expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
 const db=new Database(":memory:");try {const row=db.query("select ? > ? exceeded, ? <> ? distinctCauses").get(law.actualWork,law.quotedWork,law.primary,law.workRefusal) as {exceeded:number,distinctCauses:number};expect(row).toEqual({exceeded:1,distinctCauses:1});expect(law.copiedItems).toBe(0)} finally {db.close()}
});
