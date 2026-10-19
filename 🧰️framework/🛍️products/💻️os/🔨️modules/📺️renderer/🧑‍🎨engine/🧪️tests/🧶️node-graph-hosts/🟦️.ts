/** 🧶 Node graph host, graph slider overlays, graph parameter dispatch and shared Flow runtime. */
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { dagSliderOverlayFixture as graphSliderFixture, artifactsDagVcsSchema as dagVcsSchema } from "../../../../♾️infinite/🧪️tests/🧫️fixtures/🟦️.ts";
import { describe, expect, it } from "vitest";
import { parseDagSliderOverlays } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { peerExport } from "./🧰️node-graph-host-fixtures/🟦️.ts";
import { registerHostRenderingAndGestureTests } from "./🖼️host-rendering-and-gestures/🟦️.ts";
import { registerInteractionAddressEncodingTests } from "./🏷️interaction-address-encoding/🟦️.ts";
import { registerGraphPickAndSliderAccessibilityTests } from "./🎚️graph-pick-channels-and-slider-accessibility/🟦️.ts";
import { registerGraphParameterDispatchTests } from "./🧪️graph-parameter-dispatch-and-shared-runtime/🟦️.ts";
import { registerSliderOverlayStateAndChromeTests } from "./🎛️slider-overlay-state-and-chrome/🟦️.ts";

describe("physical slider receiving contract",()=>{

  it("checks actual slider rows against the canonical contract and independent validator",()=>{
    const independent=peerExport(dagVcsSchema,"SliderOverlayRow");
    for(const item of graphSliderFixture.cases){expect(independent(item.row),JSON.stringify(independent.errors)).toBe(true);expect(validateJsonSchemaSubset(dagVcsSchema.$defs.SliderOverlayRow,item.row,dagVcsSchema)).toEqual([]);expect(parseDagSliderOverlays(JSON.stringify({sliders:[item.row]}))).toEqual([item.row]);}
  });
});

describe("framework renderer hosts", () => {
  registerHostRenderingAndGestureTests();
  registerInteractionAddressEncodingTests();
  registerGraphPickAndSliderAccessibilityTests();
  registerGraphParameterDispatchTests();
  registerSliderOverlayStateAndChromeTests();
});
