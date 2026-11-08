import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import data from "./case.json";
import schema from "./case.schema.json";
test("birth remains hidden until its original Source accepts its actual action",()=>{expect(new Ajv().validate(schema,data)).toBe(true);const db=new Database(":memory:");const q=db.query("SELECT ?1 AND ?2 AND ?3 AND ?4 AND ?5 AS admitted");for(const row of data){const r=q.get(+row.sameAction,+row.sameSource,+row.sameWork,+row.accepted,+row.published) as {admitted:number};expect(Boolean(r.admitted)).toBe(row.canPublish)}db.close()});
