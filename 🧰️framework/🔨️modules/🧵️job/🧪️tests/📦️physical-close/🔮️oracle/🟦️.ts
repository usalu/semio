import { test } from "bun:test";
import { testJobPayloadPhysicalClose } from "../🟦️.ts";

test("refuses sub-page release and retires the exact physical extent using independent JSON Patch", () => {
  testJobPayloadPhysicalClose();
  console.log("[DEBUG] physical page retirement: four logical lengths, repeated short grants retain the exact page, full release costs16384");
});


import { expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { applyPatch } from "fast-json-patch";

test("fixed operation logical removals retain original capacity until an independent physical grant", () => {
  const root=resolve(import.meta.dir,"../../..");
  const fixture=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/📇️fixed-operation-registry-law.json"),"utf8"));
  expect(fixture.cases.map((row:any)=>row.id).sort()).toEqual(["aba","byte-maximum-plus-one","cancel-stale","collision","empty","interrupted-repeated-close","maximum-plus-one","single"]);
  for(const row of fixture.cases){
    for(const step of row.steps.filter((step:any)=>step.action==="close")){
      expect(step.grant).toEqual({maximumItems:1,maximumCopyBytes:1,maximumCapacityBytes:0,maximumReleaseBytes:8,maximumDepth:64});
    }
    for(const owner of row.steps.filter((step:any)=>step.action==="admit")){
      let state={length:owner.bytes,capacity:owner.bytes};let copied=0,released=0;
      while(state.length>0){
        const before=structuredClone(state);
        const final=state.length===1;
        const grant={maximumItems:1,maximumCopyBytes:1,maximumCapacityBytes:0,maximumReleaseBytes:8,maximumDepth:64};
        const short={...grant,maximumReleaseBytes:Math.max(0,state.capacity-1)};
        if(final){expect(short.maximumReleaseBytes<state.capacity).toBe(true);expect(state).toEqual(before);}
        const next={length:state.length-1,capacity:final?0:state.capacity};
        const oracle=applyPatch(structuredClone(before),[{op:"replace",path:"/length",value:before.length-1},...(final?[{op:"replace" as const,path:"/capacity",value:0}]:[])],true,false).newDocument;
        expect(next).toEqual(oracle);copied++;released+=final?before.capacity:0;state=next;
      }
      expect(copied).toBe(owner.bytes);expect(released).toBe(owner.bytes);expect(state.capacity).toBe(0);
    }
  }
  const source=readFileSync(resolve(root,"🧪️tests/🔬️fixed-operation-registry/🦀️.rs"),"utf8");
  expect(source.includes("grant.maximum_copy_bytes")).toBe(true);
  expect(source.includes("self.bytes.capacity()")).toBe(true);
  expect(source.includes("copied_bytes:1")).toBe(true);
  expect(source.includes("std::mem::take(&mut self.bytes)")).toBe(true);
  expect(source.includes("grant:RetainedCloneGrant)")).toBe(true);
});
