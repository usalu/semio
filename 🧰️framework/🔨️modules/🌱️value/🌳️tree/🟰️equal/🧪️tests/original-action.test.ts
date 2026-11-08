import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "./original-action.schema.json";
import law from "./original-action.json";
test("original equality child work stays within its selected action",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ? + ? + ? + ? as selected, ? > ? as overfunded").get(law.leftCheck,law.rightCheck,law.childFirst,law.childRecovery,law.parent,law.selected)).toEqual({selected:law.selected,overfunded:1});expect(law.childRecovery).toBe(law.remainingChild);expect([law.receives,law.recovers]).toEqual([1,1])}finally{db.close()}});
