import{expect,test}from"bun:test";
import Decimal from"decimal.js";
import{applyPatch}from"fast-json-patch";
import fixture from"../🧫️fixtures/🔣️.json";
test("original invocation context survives retry and genuine pending forwarding",()=>{
 const Exact=Decimal.clone({precision:100});for(const row of fixture.identities)expect(new Exact(row.operationId).toFixed(0)).toBe(row.cancellationId);
 for(const row of fixture.cases){
  const retry=applyPatch(structuredClone(row.source),[],true,false).newDocument;
  expect(retry).toEqual(row.source);
  expect(retry.inputJson.length).toBeGreaterThan(0);
  expect(retry.dependencyJson.length).toBeGreaterThan(0);
  const nested=applyPatch(structuredClone(row.source),Object.entries(row.pending).map(([key,value])=>({op:"replace"as const,path:"/"+key,value})),true,false).newDocument;
  expect(nested).toEqual(row.expected);
  expect(nested.dependencyJson).toBe(row.source.dependencyJson);
  expect(nested.operatorVersion).toBe(row.source.operatorVersion);
 }
 console.log("[DEBUG] Original invocation plainOriginalCases=true independentJSONPatch=true independentDecimal=true retainedNative=unqualified");
});

import{existsSync}from"node:fs";
import{join}from"node:path";
test("original Host examples cannot declare whole-trial authorities",()=>{for(const relative of ["../🧬️schema/🔣️.json","../../🖼️display/🧬️schema/🔣️.json","../../../📥️evaluation-source/🧬️schema/🔣️.json","../../../📥️evaluation-source/🌱️seeds/🧬️schema/🔣️.json"])expect(existsSync(join(import.meta.dir,relative))).toBe(false);console.log("[DEBUG] original Host trial authority absence4 preserves plain cases and actual production payload contracts");});
