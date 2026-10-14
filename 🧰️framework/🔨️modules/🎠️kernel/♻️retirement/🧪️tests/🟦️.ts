import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("portable kernel owners retain all original strings and fixed command-page bytes",()=>{expect(new Ajv().compile(schema)(law)).toBe(true);expect(Buffer.from(law.bytes).toString("base64")).toBe(law.expected.pageBase64);const db=new Database(":memory:");try{for(const text of [law.appId,law.documentId,law.editId])expect(db.query("SELECT ? AS owner").get(text)).toEqual({owner:text});}finally{db.close()}console.log("[DEBUG] SQLite original UTF8 identities and Buffer fixed-page oracle agree");});
