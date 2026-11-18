import {describe,expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {graphSliceReadSelectedV1} from "../../📥️load/🟦️.ts";

/** 🔪️ A declared selected read: a selection other than `all` is inferred over its owner closure alone instead of being cut from the sealed generation of the whole workspace (which an empty machine must infer first, owner by owner, however small the slice). */
const source=readFileSync(resolve(import.meta.dir,"..","..","📥️load","🟦️.ts"),"utf8");
describe("selected slice read",()=>{
 test("a selected read is the default, only generation switches it off",()=>{
  for(const value of [undefined,"","selected","1"])expect(graphSliceReadSelectedV1({SEMIO_BUILD_GRAPH_SLICE_READ:value})).toBe(true);
  expect(graphSliceReadSelectedV1({SEMIO_BUILD_GRAPH_SLICE_READ:"generation"})).toBe(false);
 });
 test("the switch is consulted after the all selection and before the generation slice",()=>{
  const all=source.indexOf('if(options.selection.kind==="all")return withImplementationPinV1'),switched=source.indexOf("if(graphSliceReadSelectedV1())"),slice=source.indexOf("loadBuildGraphSliceV1(root,invocation,phaseControl,options)");
  expect(all).toBeGreaterThan(0);
  expect(switched).toBeGreaterThan(all);
  expect(slice).toBeGreaterThan(switched);
  expect(source).toContain("baseline:true}));");
 });
});
