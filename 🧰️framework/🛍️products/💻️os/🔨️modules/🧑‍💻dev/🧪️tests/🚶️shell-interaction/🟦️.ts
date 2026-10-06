import { describe, expect, it } from "vitest";

import authored from "../../⚖️parity/🧫️fixtures/🚶️shell-interaction/🔣️.json";
import { parseShellInteractionJourney, runShellInteractionJourney, shellInteractionDifferences } from "../../🧪️testing/🚶️journey/🟦️.ts";

describe("permanent physical shell journey", () => {
  it("admits all57 authored steps with independent schema authority", () => {
    expect(parseShellInteractionJourney(authored).steps).toHaveLength(57);
    for (const mutate of [(v: typeof authored) => { v.steps[0]!.operation = "fake-input"; }, (v: typeof authored) => { v.steps.pop(); }, (v: typeof authored) => { v.viewport.width = 0; }, (v: typeof authored) => { v.physicalInputTypes = ["synthetic-input"]; }, (v: typeof authored) => { Object.assign(v.steps[0]!, { unowned: true }); }]) {
      const value = structuredClone(authored); mutate(value); expect(() => parseShellInteractionJourney(value)).toThrow();
    }
  });
  it("refuses absent required live URLs and locale before browser launch", async () => { await expect(runShellInteractionJourney([])).rejects.toThrow(); });
  it("refuses forged physical owner/body/outcome and dispatched journal receipts", () => {
    const state = { ready: "puzzle", error: null, controls: [], surfaces: ["window:A"], fullscreen: false, generation: 1, selected: [], example: null, role: null };
    const side = { before: state, after: state, actions: [{ seq: 1, action: "focus" }], inputs: [{ seq: 1, type: "pointerdown", trusted: true, target: "dock.tab.A.focus" }], detail: { windowId: "A", physicalOutcome: true, afterBodies: { A: [0, 0, 100, 100] } }, error: null, screenshot: "receipt.png", observed: true };
    const receipt = { name: "window-cap-focus", renderers: { react: structuredClone(side), wgpu: structuredClone(side) }, differences: [] };
    const definition = parseShellInteractionJourney(authored);
    expect(shellInteractionDifferences(receipt, definition)).toEqual([]);
    for (const mutate of [(s: typeof side) => { s.detail.windowId = "foreign"; }, (s: typeof side) => { s.detail.afterBodies.A[2]! += 10; }, (s: typeof side) => { s.detail.physicalOutcome = false; }, (s: typeof side) => { s.actions = [{ seq: 2, action: "close" }]; }, (s: typeof side) => { s.observed = false; }, (s: typeof side) => { s.inputs[0]!.trusted = false; }, (s: typeof side) => { s.inputs = []; }]) {
      const wrong = structuredClone(receipt); mutate(wrong.renderers.wgpu); expect(shellInteractionDifferences(wrong, definition).length).toBeGreaterThan(0);
    }
  });
});
