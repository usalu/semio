import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import schema from "./🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("original unsigned diagnostic targets agree with independent decimal SQL casts",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{for(const row of law.cases){expect((db.query("SELECT CAST(? AS TEXT) AS target").get(row.value)as{target:string}).target).toBe(row.target);expect(JSON.stringify(row.value)).toBe(row.target);}}finally{db.close();}});
