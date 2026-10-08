/** 📷️ Viewer navigation is local to one window and agrees with an independent JSON Patch reducer. */
import { expect,test } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json" with { type: "json" };
import { applyPatch } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import { applyDrawingViewerCanvasWindowConfigMutation,parseDrawingViewerCanvasWindowConfig,type DrawingViewerCanvasWindowConfig } from "../../🧬️schema/🟦️.ts";

test("viewer cameras are independent and restore from the same neutral trace",async () => {
  const viewportSchema = await Bun.file(new URL("../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🔣️.json",import.meta.url)).json();
  const ajv = new Ajv().addMetaSchema(draft7);
  ajv.addSchema(viewportSchema);
  const validate = ajv.compile(schema);
  const own: Record<string,DrawingViewerCanvasWindowConfig> = {},oracle: typeof own = {};
  for (const step of fixture.steps) {
    const config = { viewport: step.camera,framed: true };
    expect(validate(config)).toBe(true);
    own[step.window] = applyDrawingViewerCanvasWindowConfigMutation(own[step.window] ?? fixture.base,{ kind: "set",...config });
    oracle[step.window] = applyPatch(structuredClone(oracle[step.window] ?? fixture.base),[{ op: "replace",path: "",value: config }]).newDocument;
  }
  expect(own).toEqual(fixture.expected);
  expect(own).toEqual(oracle);
  for (const [window,value] of Object.entries(JSON.parse(JSON.stringify(own)))) expect(parseDrawingViewerCanvasWindowConfig(value)).toEqual(own[window]);
});

test("viewer camera guards refuse malformed navigation",() => {
  for (const viewport of [{ x: 0,y: 0,zoom: 0 },{ x: Infinity,y: 0,zoom: 1 },{ x: 0,y: NaN,zoom: 1 }]) expect(() => parseDrawingViewerCanvasWindowConfig({ viewport,framed: true })).toThrow();
  expect(() => parseDrawingViewerCanvasWindowConfig({ ...fixture.base,shared: true })).toThrow();
});
