import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import corpus from "./failure-storage.json";
import schema from "./failure-storage.schema.json";

test("prepaid scalar work settlement keeps each original role and actual work",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const db=new Database(":memory:");
 try{
  expect(db.query("SELECT ? > ? AS refused, ? - ? AS overrun").get(corpus.overrunWork,corpus.actualWork,corpus.overrunWork,corpus.actualWork)).toEqual({refused:1,overrun:1});
  const row=db.query("SELECT CASE WHEN ?=0 THEN 0 ELSE 1 END AS childCalls, CASE WHEN ?=0 THEN 0 ELSE ? END AS work, CASE WHEN ?=0 THEN 0 ELSE ? END AS copiedItems").get(corpus.childCallsOnDenial,corpus.childCallsOnDenial,corpus.actualWork,Number(corpus.genericResult),corpus.genericItems);
  expect(row).toEqual({childCalls:0,work:0,copiedItems:7});
 }finally{db.close();}
});
