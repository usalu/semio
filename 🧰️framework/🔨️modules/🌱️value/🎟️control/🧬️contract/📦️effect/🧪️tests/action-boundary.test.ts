import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "./action-boundary.schema.json";
import law from "./action-boundary.json";
test("actual cumulative work survives receipt replacement without widening",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ? + ? as total, (? + ?) > ? as overrun").get(law.initial,law.later,law.initial,law.later,law.selected)).toEqual({total:law.total,overrun:1});expect(law.foreignRejected&&law.receiptRejected).toBe(true)}finally{db.close()}});
