/** 🧪️ Typed sparse patch slots preserve clear independently of transport. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../../../../../🧫️fixtures/🔺️diff/🩹️typed-clear/🔣️.json";
import {parseDrawingLayerPatch} from "../../../../../🧬️schema/🔺️diff/🟦️.ts";
import {decodeDrawingDiffJson} from "../../🟦️.ts";
import documentSchema from "../../../../../🧬️schema/🔣️.json";
import diffSchema from "../../../../../🧬️schema/🔺️diff/🔣️.json";

test("typed fill and stroke clear differ from untouched",()=>{
  const clear=parseDrawingLayerPatch(fixture),untouched=parseDrawingLayerPatch({});
  expect(clear.fill).toEqual({value:null});
  expect(clear.stroke).toEqual({value:null});
  expect(untouched.fill).toBeUndefined();
  expect(untouched.stroke).toBeUndefined();
  const wrapper={type:"object",properties:{value:{type:"null"}},required:["value"],additionalProperties:false};
  const oracle=new Ajv().compile({type:"object",properties:{fill:wrapper,stroke:wrapper},required:["fill","stroke"],additionalProperties:false});
  expect(oracle(fixture)).toBe(true);
  expect(oracle({})).toBe(false);
  console.error("[DEBUG] typed clear and untouched patches remain distinct; Ajv independent shape oracle agrees");
});

test("JSON numeric transform is admitted before semantic validation",()=>{
  const diff=decodeDrawingDiffJson('{"layers":{"modified":[{"id":"layer","patch":{"transform":{"x":24,"y":-8,"scaleX":2,"scaleY":1.5,"shear":0,"rotation":0}}}]}}');
  const transform=diff.layers!.modified[0]!.patch.transform!;
  const oracle=new DataView(new ArrayBuffer(8));oracle.setFloat64(0,24);
  expect(transform.x.bits).toBe(oracle.getBigUint64(0));
  expect(diff.layers!.modified[0]!.patch.fill).toBeUndefined();
  console.error("[DEBUG] IO admits numeric JSON as typed transform words before semantic guards");
});

test("complete diff schema resolves typed patch definitions",()=>{
  const oracle=new Ajv({strict:false}).addSchema(documentSchema).compile(diffSchema);
  expect(oracle({layers:{removed:[],inserted:[],moved:[],modified:[{id:"layer",patch:fixture}]}})).toBe(true);
  expect(oracle({layers:{removed:[],inserted:[],moved:[],modified:[{id:"layer",patch:{transform:"encoded"}}]}})).toBe(false);
  console.error("[DEBUG] Complete diff schema resolves typed clear and rejects encoded transform strings");
});
