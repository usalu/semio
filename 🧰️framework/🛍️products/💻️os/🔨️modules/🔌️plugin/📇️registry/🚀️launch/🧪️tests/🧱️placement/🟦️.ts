/** 🧱️ Admits executable and interactive input containers through the actual source producer. */
import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {parse,type ParseError} from "jsonc-parser";
import schema from "../../🧬️schema/🧱️placement/🔣️.json";
import fixture from "../../🧫️fixtures/🧱️placement/🔣️.json";

const marker=',\n\n  // 🎮️devLaunchers — per-playground-variant dev-launcher metadata (not part of the generated\n  // output); see 🚀️launch/🟦️.ts readSeed() for the exact split contract this marker line supports.\n  "devLaunchers": ';
const raw=(document:unknown):string=>JSON.stringify(document,null,2).trimEnd().slice(0,-1)+marker+"{}\n}\n";

test("every language-neutral placement row matches independent strict schema admission",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.cases)expect(validate(row.document),row.id).toBe(row.valid);
});

test("owned placement admission rejects every hostile row before accepting a source document",async()=>{
 const {assertLaunchSeedPlacement}=await import("../../🧱️placement/🟦️.ts");
 for(const row of fixture.cases){
  const admit=()=>assertLaunchSeedPlacement(row.document);
  if(row.valid)expect(admit,row.id).not.toThrow();else expect(admit,row.id).toThrow("invalid launch seed");
 }
});

test("the actual launch producer rejects misplaced rows before rendering and preserves admitted data",async()=>{
 const {generateLaunchJson}=await import("../../🟦️.ts");
 for(const row of fixture.cases){
  const seed=raw(row.document),produce=()=>generateLaunchJson(process.cwd(),[],[],()=>seed);
  if(!row.valid){expect(produce,row.id).toThrow("invalid launch seed");continue;}
  const rendered=produce(),errors:ParseError[]=[];
  const independent=parse(rendered,errors,{allowTrailingComma:true});
  expect(errors,row.id).toEqual([]);
  expect(independent,row.id).toEqual(row.document);
  expect(Bun.JSONC.parse(rendered),row.id).toEqual(independent);
 }
});

import "../🏭️generate/🟦️.ts";
