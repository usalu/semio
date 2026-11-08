import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import data from "./accepted-proof.json";
import schema from "./accepted-proof.schema.json";
test("accepted construction evidence requires its actual issuer and finished Work",()=>{expect(new Ajv().validate(schema,data)).toBe(true);const db=new Database(":memory:");const q=db.query("SELECT ?1 AND ?2 AND ?3 AND ?4 AS valid");for(const row of data){const r=q.get(+row.sameIssuer,+row.published,+row.accepted,+row.finishedWork) as {valid:number};expect(Boolean(r.valid)).toBe(row.valid)}db.close()});
