import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import law from "./🔣️.json";
test("native intrinsic Record output agrees with independent JSON Schema and SQLite",()=>{const check=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["label","values"],properties:{label:{type:"string"},values:{type:"array",items:{type:"integer",minimum:0,maximum:4294967295}}}});expect(check(law.document)).toBe(true);expect(check({...law.document,values:[4294967296]})).toBe(false);expect(check({...law.document,values:[-1]})).toBe(false);expect(check({...law.document,values:[1.5]})).toBe(false);const db=new Database(":memory:");const source=JSON.stringify(law.document);const label=(db.query("SELECT json_extract(?, '$.label') AS label").get(source)as{label:string}).label;const values=(db.query("SELECT value FROM json_each(?, '$.values') ORDER BY key").all(source)as{value:number}[]).map(v=>v.value);expect({label,values}).toEqual(law.document);db.close();});

import fundingSchema from "./🧬️schema/🔣️.json";

test("selected intrinsic closure refuses each insufficient immutable currency",()=>{const axes=["items","copyBytes","capacityBytes","releaseBytes","depth"]as const;expect(new Ajv({strict:true}).compile(fundingSchema)(law)).toBe(true);for(const row of law.selectedClosure.cases)expect(axes.every(axis=>row.grant[axis]>=law.selectedClosure.demand[axis])).toBe(row.accepted);});
