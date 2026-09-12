import { describe, expect, it } from "vitest";
import { retainedUiIntakeStepCeiling, RETAINED_UI_INTAKE_STEPS_PER_NODE, RETAINED_UI_INTAKE_SLICE_STEPS } from "../../🟦️.ts";

describe("paged scene-lane intake ceiling", () => {
  it("credits every node a document's first publication has to mint", () => {
    expect(retainedUiIntakeStepCeiling({ maxNodes: 20_000 })).toBe(20_000 * RETAINED_UI_INTAKE_STEPS_PER_NODE);
    expect(retainedUiIntakeStepCeiling({ maxNodes: 145 })).toBeGreaterThan(671_321);
    expect(retainedUiIntakeStepCeiling({ maxNodes: 1 })).toBe(RETAINED_UI_INTAKE_STEPS_PER_NODE);
  });
  it("carries the numbers the language-agnostic fixture declares, so the Rust twin cannot drift", async () => {
    const { default: fixture } = await import("../../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/📥️intake/🔣️.json");
    expect(RETAINED_UI_INTAKE_STEPS_PER_NODE).toBe(fixture.budget.stepsPerNode);
    expect(RETAINED_UI_INTAKE_SLICE_STEPS).toBe(fixture.budget.sliceSteps);
    for (const ceiling of fixture.budget.ceilings) expect(retainedUiIntakeStepCeiling({ maxNodes: ceiling.maxNodes })).toBe(ceiling.steps);
    expect(fixture.laws).toContain("budget-scales-with-node-quota");
    expect(fixture.laws).toContain("budget-slice-is-resumable");
  });
  it("prices a slice far below the document a surface may mint, so exhausting one is a yield and not a fault", () => {
    expect(RETAINED_UI_INTAKE_SLICE_STEPS).toBeLessThan(retainedUiIntakeStepCeiling({ maxNodes: 1 }));
  });
});
