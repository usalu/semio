import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️.json";
import foundation from "../../🧪️tests/🧫️fixtures/🧱️foundation/🔣️.json";
test("bounded native UI text agrees with SQLite UTF8 and JSON ownership corpus",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 for(const row of foundation.texts){const input=row.unit.repeat(row.repeat);const actual=db.query("select length(cast(? as blob)) bytes,json_valid(json_quote(?)) valid").get(input,input) as {bytes:number,valid:number};expect(actual.bytes).toBe(row.bytes);expect(actual.valid).toBe(1);expect(actual.bytes<=512).toBe(row.valid);}
 db.close();
});
