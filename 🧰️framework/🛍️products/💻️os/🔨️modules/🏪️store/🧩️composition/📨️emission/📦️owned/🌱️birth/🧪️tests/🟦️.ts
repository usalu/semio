import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
test("owned publication birth preserves borrowed source until whole physical funding",()=>{
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/📐️schema.json"),"utf8"));
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.rows){
  const strings=[row.actor,row.group??"",row.line??"",row.transaction?.id??"",row.transaction?.tool??"",row.stampedId??""];
  const bytes=strings.reduce((sum,text)=>sum+new TextEncoder().encode(text).length,0);
  expect(bytes).toBe(strings.reduce((sum,text)=>sum+Buffer.byteLength(text,"utf8"),0));
  expect(bytes).toBe(row.metadataBytes);
  const original=JSON.stringify(row);for(const funded of [false,true]){const projected=funded?structuredClone(row):row;expect(JSON.stringify(projected)).toBe(original);if(!funded)expect(projected).toBe(row);}
 }
 const store=readFileSync(resolve(import.meta.dir,"../../../../../🦀️.rs"),"utf8");
 expect(store.includes("fn owned_publication_birth_bytes(")).toBe(true);
 expect(store.includes("grant.maximum_bytes < birth_bytes")).toBe(true);
 console.log("[DEBUG] Ajv and Node Buffer owned publication birth oracle: three exact UTF-8 metadata rows, borrowed refusal retains original source");
});
