import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
test("owned publication birth preserves borrowed source until whole physical funding",()=>{
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 for(const row of fixture.rows){
  const strings=[row.actor,row.group??"",row.line??"",row.transaction?.id??"",row.transaction?.tool??"",row.stampedId??""];
  const bytes=strings.reduce((sum,text)=>sum+new TextEncoder().encode(text).length,0);
  expect(bytes).toBe(strings.reduce((sum,text)=>sum+Buffer.byteLength(text,"utf8"),0));
  expect(bytes).toBe(row.metadataBytes);
  const original=JSON.stringify(row);for(const funded of [false,true]){const projected=funded?structuredClone(row):row;expect(JSON.stringify(projected)).toBe(original);if(!funded)expect(projected).toBe(row);}
 }
 const store=readModuleTreeSource(resolve(import.meta.dir,"../../../../👥️space-member/📥️lifted/🏪️store/🦀️.rs")),owned=readFileSync(resolve(import.meta.dir,"../../🦀️.rs"),"utf8");
 expect(store.includes("fn owned_publication_birth_bytes(")).toBe(true);
 expect(owned.includes("self.birth_bytes > grant.maximum_capacity_bytes")).toBe(true);
});
