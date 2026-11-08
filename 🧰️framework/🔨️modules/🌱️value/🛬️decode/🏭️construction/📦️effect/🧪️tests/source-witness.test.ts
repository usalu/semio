import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import law from "./source-witness.json";
import schema from "./source-witness.schema.json";
test("only the actual Work Source lends its dispatch witness",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{const row=db.query("SELECT ?1 AS accepted, ?2 AS callbacks, ?3 AS work").get(Number(law.foreignAccepted),law.foreignCallbacks,law.workSpent);expect(row).toEqual({accepted:0,callbacks:0,work:0})}finally{db.close()}});
