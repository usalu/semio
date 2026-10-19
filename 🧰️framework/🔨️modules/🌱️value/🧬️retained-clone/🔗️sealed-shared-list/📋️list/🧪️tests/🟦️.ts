import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {applyPatch} from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("immutable original list root clone preserves portable samples through independent close orders",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);const database=new Database(":memory:");
 try{for(const row of fixture.cases){
  const original=database.query<{samples:string},[number,number,number]>("WITH RECURSIVE rows(n) AS (SELECT 0 WHERE ?>0 UNION ALL SELECT n+1 FROM rows WHERE n+1<?) SELECT json_group_array(2*n) AS samples FROM rows WHERE n<3 OR n=?-1").get(row.length,row.length,row.length)!;
  const samples=JSON.parse(original.samples)as number[];expect(samples).toEqual(row.expectedSamples);
  for(const order of fixture.closeOrders){const before={source:{samples},output:{samples}};const closed=applyPatch(before,[{op:"remove",path:order==="sourceFirst"?"/source":"/output"}],true,false).newDocument;expect((closed.source??closed.output).samples).toEqual(row.expectedSamples);expect(before.source.samples).toEqual(row.expectedSamples);}
  for(const cut of fixture.cuts){const before={source:{samples},candidate:cut===2?{samples}:undefined};const closed=applyPatch(before,[{op:"remove",path:"/source"}],true,false).newDocument;expect(before.source.samples).toEqual(row.expectedSamples);if(cut===2)expect(closed.candidate!.samples).toEqual(row.expectedSamples);}
 }
 console.log("[DEBUG] four portable immutable list root outputs and independent close orders match SQLite ranked rows and JSON Patch; physical zero-copy/allocation obligations belong to actual native proof");
 }finally{database.close();}
});
