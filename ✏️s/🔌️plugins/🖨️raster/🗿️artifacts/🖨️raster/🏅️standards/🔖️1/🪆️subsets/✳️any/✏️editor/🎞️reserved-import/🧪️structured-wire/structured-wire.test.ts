import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import corpus from "./cases.json";
import schema from "./schema.json";

test("Raster declares one exact Structured workflow wire",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of corpus.cases){
  const result=db.query<{accepted:number},[string,string,string,string]>("SELECT ?1='image:in' AND ?2='2d.image' AND ?3='twoD' AND ?4='raster' AS accepted").get(row.port,row.schema,row.mediaClass,row.form)!;
  expect(Boolean(result.accepted)).toBe(row.accepted);
 }}finally{db.close();}
});
