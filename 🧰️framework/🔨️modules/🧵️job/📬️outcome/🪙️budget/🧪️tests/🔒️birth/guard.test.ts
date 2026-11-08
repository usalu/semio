import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import data from "./guard.json";
import schema from "./guard.schema.json";
test("only the original gate owns its held guard",()=>{expect(new Ajv().validate(schema,data)).toBe(true);const db=new Database(":memory:");const q=db.query("SELECT ?1 AS owned");for(const row of data){const r=q.get(+row.sameGate) as {owned:number};expect(Boolean(r.owned)).toBe(row.owned)}db.close()});
