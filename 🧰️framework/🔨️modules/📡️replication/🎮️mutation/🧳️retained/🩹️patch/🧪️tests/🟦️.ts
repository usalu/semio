import assert from "node:assert/strict";
import Ajv from "ajv";
import Ajv2020 from "ajv";
import {applyPatch,compare} from "fast-json-patch";
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "./🧫️fixtures/schema.json" with {type:"json"};
import closeSchema from "../🧬️schema/🎟️close/🔣️.json";
import closeFixture from "./🧫️fixtures/🎟️close/🔣️.json";
import {Database} from "bun:sqlite";

/** 🧪️ Portable sparse row outputs and original inverse fields match independent RFC6902 application. */
export function verifyRetainedRowPatchOracle():void{
 const closeValidate=new Ajv2020({strict:true}).compile(closeSchema);assert(closeValidate(closeFixture));assert(!closeValidate({...closeFixture,visits:4}));const db=new Database(":memory:");try{for(const body of closeFixture.bodyHints){const row=db.query("select count(*) as visits,? as body from (select 1 as original_child)").get(body)as{visits:number;body:number};assert.equal(row.visits,closeFixture.visits);assert.equal(row.body,body);}}finally{db.close();}
 const validate=new Ajv({strict:true}).compile(schema);assert(validate(corpus),JSON.stringify(validate.errors));
 for(const row of corpus){
  const operations=Object.entries(row.patch).map(([name,value])=>({op:"replace" as const,path:"/"+name,value}));
  const independent=applyPatch(structuredClone(row.before),operations,true,false).newDocument;assert.deepEqual(independent,row.after,row.id);
  const inverse=Object.fromEntries(Object.keys(row.patch).map(key=>[key,row.before[key as keyof typeof row.before]]));assert.deepEqual(inverse,row.inverse,row.id+" inverse source fields");
  const undo=applyPatch(structuredClone(row.after),compare(row.after,row.before),true,false).newDocument;assert.deepEqual(undo,row.before,row.id+" independent undo");
 }
}
