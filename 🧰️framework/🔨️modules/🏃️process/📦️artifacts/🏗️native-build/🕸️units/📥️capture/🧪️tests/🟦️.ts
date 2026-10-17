import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Command} from "commander";
import {projectCargoCompilerPlanningArgumentsV1} from "../🟦️.ts";
import schema from "../🧬️schema/📋️projection.json" with {type:"json"};
import policy from "../🔣️policy.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
test("Cargo run compiler projection preserves exact selectors and excludes runtime arguments",()=>{
 expect(new Ajv({strict:true}).validate(schema,policy)).toBe(true);
 for(const row of fixture.cases){const original=[...row.arguments],observed=projectCargoCompilerPlanningArgumentsV1(original,fixture.facts);expect(observed,row.name).toEqual(row.expected);expect(original).toEqual(row.arguments);if(original[0]==="run"){const index=original.indexOf("--"),head=index<0?original:original.slice(0,index),oracle=new Command().exitOverride().allowUnknownOption().allowExcessArguments().option("--bin <name>").option("--example <name>").parse(head.slice(1),{from:"user"}).opts<{bin?:string;example?:string}>();if(oracle.bin)expect(observed).toContain(oracle.bin);if(oracle.example)expect(observed.some(arg=>arg.includes(oracle.example!))).toBe(true);expect(observed).not.toContain("--runtime");}}
 const ambiguous={...fixture.facts,packages:fixture.facts.packages.map(row=>({...row,default_run:null}))};expect(()=>projectCargoCompilerPlanningArgumentsV1(["run"],ambiguous)).toThrow();
});
