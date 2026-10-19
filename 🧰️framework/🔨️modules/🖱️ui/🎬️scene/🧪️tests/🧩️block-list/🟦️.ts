import {describe,expect,test} from "bun:test";
import Ajv from "ajv";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {blockListPaletteTargetStepIdV1,type BlockListScene} from "../../🧬️schema/🧩️block-list/🟦️.ts";
import schema from "../../🧬️schema/🧩️block-list/🔣️.json" with {type:"json"};
import fixture from "../🧫️fixtures/🧩️block-list/🔣️.json" with {type:"json"};
const oracle=new Ajv({strict:false}).compile(schema);
describe("Typed block-list scene",()=>{
 test("owned schema admission agrees with independent Ajv",()=>{
  for(const value of [fixture.scene,{steps:[],palette:[]}]){expect(validateJsonSchemaSubset(schema,value)).toEqual([]);expect(oracle(value)).toBe(true);}
  for(const row of fixture.refusals){expect(validateJsonSchemaSubset(schema,row.value).length).toBeGreaterThan(0);expect(oracle(row.value)).toBe(false);}
 });
 test("typed targets preserve selected step, block parent, stale and empty cases",()=>{
  const scene:BlockListScene=fixture.scene;
  for(const row of fixture.targetCases){const steps=row.steps==="empty"?[]:scene.steps;expect(blockListPaletteTargetStepIdV1(steps,row.selectedId??undefined)).toBe(row.expectedStepId??undefined);}
  expect(scene.steps[0]!.target).toEqual({granularity:"section",id:"step:basics"});expect(scene.steps[0]!.blocks[0]!.target).toEqual({granularity:"field",id:"load"});
 });
});
