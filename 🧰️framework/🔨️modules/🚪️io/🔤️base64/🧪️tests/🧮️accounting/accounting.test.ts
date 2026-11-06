import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import corpus from "./cases.json";
import schema from "./schema.json";

test("strict Base64 receipts preserve actual visited prefixes and retained output on cancellation",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of corpus.cases){
  const wire=row.prefix+row.word.repeat(row.repeat),decoded=Buffer.from(wire,"base64");
  expect(decoded.toString("base64")===wire).toBe(row.valid);
  const effects=db.query<{unvisited:number;metadata:number;retained:number},[number,number,number,number]>("SELECT ?1-?2 AS unvisited,?3-?2 AS metadata,?4 AS retained").get(wire.length*2,row.read,row.work,row.capacity)!;
  expect(effects.unvisited).toBeGreaterThanOrEqual(0);expect(effects.metadata).toBeGreaterThanOrEqual(0);expect(row.copy).toBeLessThanOrEqual(effects.retained);
  if(row.copy>0)expect(decoded.subarray(0,row.copy).toString("utf8")).toBe(row.output);
  if(row.cancel==="none"&&row.valid&&row.limit>=decoded.length){expect(row.read).toBe(wire.length*2);expect(row.work).toBe(row.read+4);expect(row.capacity).toBe(decoded.length);}
  if(row.id==="early-padding"){expect(wire.indexOf("=")).toBe(0);expect(row.read).toBe(1);expect(row.work).toBe(2);}
  if(row.cancel==="decode-end"){expect(row.read).toBe(wire.length*2);expect(row.work).toBe(row.read+3);expect(row.capacity).toBe(decoded.length);}
 }}
 finally{db.close();}
});
