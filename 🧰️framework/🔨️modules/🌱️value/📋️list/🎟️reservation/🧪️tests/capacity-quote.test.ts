import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./capacity-quote.json";
import schema from "./capacity-quote.schema.json";
test("capacity Work quote follows the actual available frontier",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{const q=db.query("select case when ? > 4 then 'refused' when ? = 0 then 'zero' else 'same-selected' end as expected");for(const row of fixture)expect(q.get(row.capacity,row.capacity)).toEqual({expected:row.expected});}finally{db.close()}
});
