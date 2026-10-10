import {existsSync as testingSchemaExists} from "node:fs";
/** 🧪️ Shared no-op decisions agree with independent RFC 6902 application. */
import {test,expect} from "bun:test";
import {applyPatch,type Operation} from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔣️.json";
import {hasLayerFieldChange,selectFieldTargets} from "../../🟦️.ts";
import selections from "../../🧫️fixtures/🎯️selection/🔣️.json";
import {produce} from "immer";
import type {DrawingArtifact} from "../../../../../🧬️schema/🟦️.ts";
import type {DrawingMutation} from "../../../../../🧬️schema/🧬️mutations/🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

function lift(value:unknown):unknown {
 if(typeof value==="number")return binary64(value);
 if(Array.isArray(value))return value.map(lift);
 return value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,lift(item)])):value;
}
test("field history admits changes and suppresses unchanged values against JSON Patch",()=>{
expect(testingSchemaExists(new URL("../../🧬️schema/🔣️.json",import.meta.url))).toBe(false);
 const document={...fixture.document,layers:lift(fixture.document.layers)} as DrawingArtifact;
 const before=structuredClone(fixture.document);
 for(const row of fixture.cases) {
  const oracle=applyPatch(structuredClone(before),row.patch as Operation[]).newDocument;
  expect(JSON.stringify(oracle)!==JSON.stringify(before)).toBe(row.changed);
  expect(hasLayerFieldChange(document,row.mutation as DrawingMutation)).toBe(row.changed);
 }
 expect(fixture.document).toEqual(before);
 console.log("[DEBUG] Layer property history decisions matched independent JSON Patch across nested facets");
});
test("missing field targets and structural verbs are rejected without publication",()=>{
 const document={...fixture.document,layers:lift(fixture.document.layers)} as DrawingArtifact;
 expect(()=>hasLayerFieldChange(document,{mutation:"setLayerVisible",layerId:"missing",visible:true})).toThrow(/missing/);
 expect(()=>hasLayerFieldChange(document,{mutation:"deleteLayer",layerId:"rect"})).toThrow(/field mutation/);
});

test("bulk field targets reject missing identities atomically and retain document order",()=>{
 const document={...fixture.document,layers:lift(fixture.document.layers)} as DrawingArtifact;
 for(const row of selections) {
  if(row.expected===null){expect(()=>selectFieldTargets(document,row.ids)).toThrow(/missing/);continue;}
  const selected=selectFieldTargets(document,row.ids).map(layer=>layer.id);
  const oracle=produce([] as string[],draft=>{
   const visit=(layers:typeof fixture.document.layers):void=>{for(const layer of layers){if(row.ids.includes(layer.id)||row.ids.includes("drawing-play-layers."+layer.kind+"."+layer.id))draft.push(layer.id);if("children" in layer&&layer.kind==="group")visit(layer.children as typeof fixture.document.layers);}};
   visit(fixture.document.layers);
  });
  expect(selected).toEqual(row.expected);expect(selected).toEqual(oracle);
 }
 console.log("[DEBUG] Bulk field target admission matched independent Immer selection without partial edits");
});
