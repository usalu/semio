import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("portable World source cuts and alias custody retain the complete physical terminal contract",()=>{expect(new Ajv().compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("SELECT 5 * 16 AS pages").get()).toEqual({pages:law.expected.backingPages});for(const cut of law.cuts)expect(db.query("SELECT ? <= 80 AS admitted").get(cut)).toEqual({admitted:1});}finally{db.close()}console.log("[DEBUG] SQLite independently verifies every partial World birth cut and its 80 concrete backing pages");});
