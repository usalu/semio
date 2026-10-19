import assert from "node:assert/strict";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
/** 🔒️ Ordered immutable row wire agrees with strict schema and independent SQLite JSON. */
export function immutableCanonicalOracle():void{
 assert(new Ajv({strict:true}).compile(schema)(corpus));const db=new Database(":memory:");
 try{db.run("CREATE TABLE rows(id INTEGER PRIMARY KEY,label TEXT NOT NULL)");for(const law of corpus.cases){db.run("DELETE FROM rows");for(let i=0;i<law.rows;i++)db.run("INSERT INTO rows VALUES(?,?)",i,corpus.label);const value=db.query("SELECT json_group_array(json_object('id',id,'label',label)) AS wire FROM (SELECT id,label FROM rows ORDER BY id)").get()as{wire:string};assert.equal(value.wire,law.output);assert.deepEqual(JSON.parse(value.wire),Array.from({length:law.rows},(_,id)=>({id,label:corpus.label})));}}finally{db.close();}
 console.log("[DEBUG] immutable canonical strict Ajv/SQLite oracle shapes="+corpus.cases.length);
}
