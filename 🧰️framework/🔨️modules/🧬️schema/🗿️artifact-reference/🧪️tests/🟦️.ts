/** 🪪️ Neutral strict value admission checked independently by Ajv. */
import { test, expect } from "bun:test";
import Ajv from "ajv";
import schema from "../🔣️.json";
import fixture from "../🧫️fixtures/🪪️identity/🔣️.json";
import { parseArtifactRef } from "../🟦️.ts";
const oracle = new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/ArtifactRef"});
for (const [index,value] of fixture.valid.entries()) test(`owned artifact identity ${index}`,()=>{
  expect(oracle(value)).toBe(true);
  expect(parseArtifactRef(value)).toEqual(value);
});
for (const [index,value] of fixture.invalid.entries()) test(`refuse malformed artifact identity ${index}`,()=>{
  expect(oracle(value)).toBe(false);
  expect(()=>parseArtifactRef(value)).toThrow();
});
