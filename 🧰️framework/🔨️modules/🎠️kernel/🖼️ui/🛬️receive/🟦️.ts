import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️.json";
test("original UI turn patch reservation advances one occupied slot per funded unit",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");
 for(const row of corpus.cases){const actual=db.query("select case when ? and not ? then min(case when value=0 then cast(key as integer) end) else null end reserved from json_each(?)").get(row.granted,row.contended,JSON.stringify(row.occupied))as{reserved:number|null};expect(actual.reserved).toBe(row.reserved);expect(!row.granted||row.contended?0:(actual.reserved??row.occupied.length-1)+1).toBe(row.scanned);}
 db.close();
});
