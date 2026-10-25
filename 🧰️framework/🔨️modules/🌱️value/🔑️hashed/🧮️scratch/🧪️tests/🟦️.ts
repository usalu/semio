import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import {ScratchHashMap} from "../🟦️.ts";
test("fixed scratch hash ownership matches independent Map through collisions, tombstones and full capacity",()=>{
 const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:true}).validate({type:"object",additionalProperties:false,required:["schema","maximum","operations"],properties:{schema:{const:"semio.value.scratch-hash.v1"},maximum:{type:"integer",minimum:0},operations:{type:"array",items:{type:"object",required:["kind"],properties:{kind:{enum:["insert","get","remove","entries"]},key:{type:"string"},value:{type:"integer"}}}}}},fixture)).toBe(true);
 for(const collide of [false,true]){
  const actual=new ScratchHashMap<string,number>(fixture.maximum,collide?()=>0:undefined),oracle=new Map<string,number>();let patched:Record<string,number>={};
  for(const row of fixture.operations){
   if(row.kind==="insert"){
    const accepted=oracle.has(row.key)||oracle.size<fixture.maximum,previous=oracle.get(row.key);
    expect(actual.tryInsert(row.key,row.value)).toEqual({accepted,previous:accepted?previous:undefined});
    if(accepted){patched=applyPatch(patched,[{op:oracle.has(row.key)?"replace":"add",path:"/"+row.key,value:row.value}],true,false).newDocument;oracle.set(row.key,row.value);}
   }else if(row.kind==="get")expect(actual.get(row.key)).toBe(oracle.get(row.key));
   else if(row.kind==="remove"){const old=oracle.get(row.key);if(oracle.has(row.key))patched=applyPatch(patched,[{op:"remove",path:"/"+row.key}],true,false).newDocument;oracle.delete(row.key);expect(actual.remove(row.key)).toBe(old);}
   expect([...actual.entries()].sort()).toEqual([...oracle.entries()].sort());
   expect(actual.size).toBe(oracle.size);
   expect([...actual.entries()].sort()).toEqual(Object.entries(patched).sort());
  }
 }
});
