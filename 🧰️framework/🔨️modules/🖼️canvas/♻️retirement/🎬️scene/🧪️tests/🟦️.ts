import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("portable fixed pages and first-party scene preserve independent original record order",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE owned (ordinal INTEGER PRIMARY KEY, value INTEGER)");const insert=db.prepare("INSERT INTO owned VALUES (?,?)");for(let n=0;n<law.pages.slots;n++)insert.run(n,n);expect(db.query<{value:number},[]>("SELECT value FROM owned ORDER BY ordinal").all().map(row=>row.value)).toEqual(Array.from({length:law.pages.slots},(_,n)=>n));}finally{db.close()}
 expect(Buffer.from(law.rgba).byteLength).toBe(4);expect(law.pages.pageSlots*law.pages.pageCount).toBe(law.pages.slots);
});
