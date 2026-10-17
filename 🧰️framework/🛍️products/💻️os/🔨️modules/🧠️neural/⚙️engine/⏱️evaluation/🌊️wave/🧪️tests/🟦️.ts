import {existsSync as testingSchemaExists} from "node:fs";
import{expect,test}from"bun:test";
import{applyPatch}from"fast-json-patch";
import fixture from"./🧫️fixtures/🔣️.json";
test("original extension wave input and metadata custody agree with independent JSON Patch",()=>{expect(testingSchemaExists(new URL("../🧬️schema/🔣️.json",import.meta.url))).toBe(false);for(const row of fixture.cases){const original=structuredClone(row.input);const oracle=applyPatch<{input?:unknown}>({},[{op:"add",path:"/input",value:row.input}],true,false).newDocument.input;expect(JSON.parse(row.expectedInputJson)).toEqual(oracle);expect(row.input).toEqual(original);expect(row.remaining).toEqual([row.neuronId]);}});
