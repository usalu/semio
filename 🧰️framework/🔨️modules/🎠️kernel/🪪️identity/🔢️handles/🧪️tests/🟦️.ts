import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "../🔣️.json";
import {canonicalDecimalHandle} from "../🟦️.ts";
import schema from "../🧬️.json";
test("native handle identity remains canonical decimal string through the full u128 boundary",()=>{
 const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 for(const row of corpus.cases){const value=JSON.parse(row.wire);expect(Boolean(validate(value))).toBe(row.accepted);expect(canonicalDecimalHandle(value)).toBe(row.accepted?value:null);const result=db.query("select case when json_type(?)='text' and length(json_extract(?,'$')) between 1 and 39 and json_extract(?,'$') not glob '*[^0-9]*' and (json_extract(?,'$')='0' or substr(json_extract(?,'$'),1,1)!='0') and (length(json_extract(?,'$'))<39 or json_extract(?,'$')<='340282366920938463463374607431768211455' collate binary) then 1 else 0 end accepted").get(row.wire,row.wire,row.wire,row.wire,row.wire,row.wire,row.wire)as{accepted:number};expect(Boolean(result.accepted)).toBe(row.accepted);}
 db.close();
});
