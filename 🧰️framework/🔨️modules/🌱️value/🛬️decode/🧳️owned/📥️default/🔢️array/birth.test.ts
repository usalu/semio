import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "./birth.schema.json";
import law from "./birth.json";
test("original array Birth retains its real cumulative work window",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select max(0, ? - (? - ?)) as remaining").get(law.maximum,law.afterFirst,law.baseline)).toEqual({remaining:law.remaining});expect(db.query("select max(0, ? - (? - ?)) as remaining").get(law.maximum,law.afterLimit,law.baseline)).toEqual({remaining:law.atLimit});expect([law.changedGrant,law.foreignWork,law.phaseAfterCompletion]).toEqual(["pending","refused","separatePaidAction"]);}finally{db.close()}});
