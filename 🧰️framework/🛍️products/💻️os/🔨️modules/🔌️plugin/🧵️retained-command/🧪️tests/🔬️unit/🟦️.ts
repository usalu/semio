import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import fixture from "../../🧫️fixtures/⛽️work-unit-authority.json";
import {pluginTestRunnerSelfTests} from "../../../📦️packages/🦀️rust/📜️script";

test("plugin runner forwards exact library and integration targets against Node's independent argument parser",async()=>{
 expect(await pluginTestRunnerSelfTests()).toBe(10);
 console.log("[DEBUG] Plugin exact-target invocation independent parser cases=10");
});

test("retained command's domain and fallback independently spend exactly one admitted unit",()=>{
 const validate=new Ajv().compile({type:"object",required:["grantUnits","cases"],properties:{grantUnits:{const:1},cases:{type:"array",minItems:2,items:{type:"object",required:["domainConsumes","domainObserves","remaining"],properties:{domainConsumes:{type:"integer",minimum:0,maximum:1},domainObserves:{const:1},remaining:{const:0}}}}}});
 expect(validate(fixture)).toBe(true);
 for(const row of fixture.cases){
  const state={fuel:fixture.grantUnits,observed:fixture.grantUnits};
  const domain=applyPatch(state,[{op:"replace",path:"/fuel",value:state.fuel-row.domainConsumes}],true).newDocument;
  const final=applyPatch(domain,[{op:"replace",path:"/fuel",value:domain.fuel===fixture.grantUnits?domain.fuel-1:domain.fuel}],true).newDocument;
  expect(final).toEqual({fuel:row.remaining,observed:row.domainObserves});
 }
 expect(fixture.phaseOrder).toEqual(["preflight","work","publish"]);
 console.log("[DEBUG] Retained command independent one-unit authority domain/fallback cases=2");
});
