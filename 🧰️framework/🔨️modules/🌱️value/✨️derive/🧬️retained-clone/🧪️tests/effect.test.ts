import {expect,test} from "bun:test";import {Database} from "bun:sqlite";import Ajv from "ajv";import schema from "./effect.schema.json";import law from "./effect.json";
/** ♻️ An independent record oracle preserves the native generated fields. */
test("derived retirement original record",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ? + ? as total").get(law.first,law.second)).toEqual({total:law.sum})}finally{db.close()}});
