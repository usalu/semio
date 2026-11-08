import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import data from "./case.json";
import schema from "./case.schema.json";
test("witness requires the same granted frame action",()=>{expect(new Ajv().validate(schema,data)).toBe(true);const db=new Database(":memory:");const q=db.query("SELECT ?1 = 'started' AND ?2 AND ?3 AS exposed");for(const row of data){const result=q.get(row.phase,+row.same,+row.granted) as {exposed:number};expect(Boolean(result.exposed)).toBe(row.exposed)}db.close()});
