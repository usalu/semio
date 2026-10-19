import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
test("Puzzle producers retain exact original routing and use same-source direct admission",()=>{
 const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:true}).validate({type:"object",required:["schema","owners"],additionalProperties:false,properties:{schema:{const:"semio.puzzle.original-producers.v1"},owners:{type:"array",minItems:3,maxItems:3,items:{type:"object",required:["path","command","raw","work","resume"],additionalProperties:false,properties:{path:{type:"string"},command:{type:"string"},raw:{type:"string"},work:{type:"string"},resume:{const:true}}}}}},fixture)).toBe(true);
 for(const row of fixture.owners){
  const rust=readFileSync(new URL(row.path,import.meta.url),"utf8");
  const start=rust.indexOf("fn build_tool_job("),end=rust.indexOf("\n    fn ",start+1),body=rust.slice(start,end<0?undefined:end);
  expect(body.includes("&mut Option<")).toBe(true);
  expect(body.includes("admit_retained_tool_producer")).toBe(true);
  expect(body.includes(row.command+"::action_id")).toBe(true);
  expect(body.includes(row.raw)).toBe(true);
  expect(body.includes(row.work)).toBe(true);
  expect(body.includes("ToolOperationSpec")).toBe(false);
  expect(body.includes("context.tool_run().cloned()")).toBe(false);
 }
 console.log("[DEBUG] Three Puzzle producers preserve exact owner policy, direct original admission and borrowed tool-run identity");
});
