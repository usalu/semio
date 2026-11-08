import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "./pair.schema.json";
import law from "./pair.json";
test("pair default phases retain the original children and close reply",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select value from json_each(?) order by key").all(JSON.stringify(law.phases))).toEqual(law.phases.map(value=>({value})));expect([law.closeReply,law.birth,law.failure,law.pending]).toEqual(["record-before-parent","original-before-effect","retain-original","same-action"]);}finally{db.close()}});
