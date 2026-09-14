import { describe, expect, it } from "vitest";
import { encodePackValue } from "@semio-tech/framework-os";
import { DEFAULT_UI_DOCUMENT_LIMITS } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { RETAINED_UI_INTAKE_SLICE_STEPS, RETAINED_UI_INTAKE_STEPS_PER_NODE, retainedUiIntakeStepCeiling } from "../../🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts";
import { WGPU_UI_INTAKE_STEP_CEILING, WgpuUiIntakeCursor } from "../../🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts";
import intakeFixture from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/📥️intake/🔣️.json";

/** @emoji 📏️ The fixed TOTAL budget the wgpu target applied before this suite existed. Kept as a literal
 * so the regression it caused stays legible: generation3d's first document faulted against it at
 * `shell-boot` with `wgpu-ui.intake-budget-exhausted` (`📓️wgpu-intake-budget-2026-09-10.md`). */
const RETIRED_FIXED_WGPU_BUDGET = 4_096;

/** @emoji 🧱️ One retained text node, the shape a window body's paged scene lanes are built from. */
function textNode(id: number, bytes: number): object {
  return { id, key: `n${id}`, component: { type: "text", value: "x".repeat(bytes), emphasize: null, dataAttributes: null }, children: [] };
}

/** @emoji 📦️ A retained surface patch carrying `totalBytes` of text across `leafBytes`-sized nodes,
 * measured through the SAME `pack` wire encoder the guest publishes with — the intake advances one
 * phase per wire element (a LEB128 byte, a text body, an attach), so the encoded size is the honest
 * lower bound on the phases the patch demands. */
function retainedPatch(totalBytes: number, leafBytes: number): { readonly nodes: number; readonly wireBytes: number } {
  let nodes = 0;
  let wireBytes = 0;
  for (let offset = 0; offset < totalBytes; offset += leafBytes) {
    nodes += 1;
    wireBytes += encodePackValue(textNode(nodes, Math.min(leafBytes, totalBytes - offset))).length;
  }
  wireBytes += encodePackValue({ tag: "set-root", val: nodes }).length;
  return { nodes, wireBytes };
}

describe("wgpu retained-UI intake budget", () => {
  it("prices a 300 KB retained patch under the proportional ceiling, where the retired fixed budget refused it", () => {
    const patch = retainedPatch(300 * 1024, 2_048);
    expect(patch, "pinned so a codec change that moves the demand shows up here").toEqual({ nodes: 150, wireBytes: 323_328 });
    expect(patch.wireBytes, "the patch alone demands more phases than the retired fixed budget ever credited").toBeGreaterThan(RETIRED_FIXED_WGPU_BUDGET);
    expect(retainedUiIntakeStepCeiling({ maxNodes: patch.nodes }), "every node of a first publication is credited").toBeGreaterThan(patch.wireBytes);
    expect(WGPU_UI_INTAKE_STEP_CEILING, "the wgpu ceiling is the contract's own node quota, so no legitimate document can exceed it").toBeGreaterThan(patch.wireBytes);
  });

  it("carries the same numbers as the React target and the language-agnostic fixture", () => {
    expect(WGPU_UI_INTAKE_STEP_CEILING).toBe(retainedUiIntakeStepCeiling(DEFAULT_UI_DOCUMENT_LIMITS));
    expect(RETAINED_UI_INTAKE_STEPS_PER_NODE).toBe(intakeFixture.budget.stepsPerNode);
    expect(RETAINED_UI_INTAKE_SLICE_STEPS).toBe(intakeFixture.budget.sliceSteps);
    for (const ceiling of intakeFixture.budget.ceilings) expect(retainedUiIntakeStepCeiling({ maxNodes: ceiling.maxNodes })).toBe(ceiling.steps);
  });

  it("resumes across the slice boundary with the retained cursor instead of faulting", async () => {
    const cursor = new WgpuUiIntakeCursor();
    for (let step = 0; step < RETAINED_UI_INTAKE_SLICE_STEPS * 3; step += 1) {
      const pending = cursor.next("intake");
      if (pending) await pending;
    }
    expect(cursor.steps, "exhausting a slice is a yield, so the same cursor keeps counting past it").toBe(RETAINED_UI_INTAKE_SLICE_STEPS * 3);
  });

  it("owes the isolate NOTHING for a step inside its hold budget, and a task only at the slice boundary", async () => {
    const cursor = new WgpuUiIntakeCursor();
    let awaited = 0;
    for (let step = 0; step < RETAINED_UI_INTAKE_SLICE_STEPS - 1; step += 1) {
      const pending = cursor.next("intake");
      if (pending) {
        awaited += 1;
        await pending;
      }
    }
    expect(awaited, "a whole slice of sub-microsecond phases fits inside one 8 ms hold, so it costs at most one hand-back").toBeLessThanOrEqual(2);
    const boundary = cursor.next("intake");
    expect(boundary, "the slice boundary itself is always a hand-back").toBeInstanceOf(Promise);
    await boundary;
  });

  it("crosses a slice boundary with a TASK and never with an animation frame", async () => {
    const host = globalThis as { requestAnimationFrame?: (callback: (timestampMs: number) => void) => number };
    const previous = host.requestAnimationFrame;
    let frames = 0;
    host.requestAnimationFrame = (callback) => {
      frames += 1;
      return Number(setTimeout(() => callback(0), 0));
    };
    try {
      const cursor = new WgpuUiIntakeCursor();
      for (let step = 0; step < RETAINED_UI_INTAKE_SLICE_STEPS * 3; step += 1) {
        const pending = cursor.next("intake");
        if (pending) await pending;
      }
      expect(cursor.steps, "the drive really crossed three slice boundaries").toBe(RETAINED_UI_INTAKE_SLICE_STEPS * 3);
      expect(frames, "an animation frame costs ~16 ms of pure sleep in the frame Worker, and a 525 k-step publication crosses 128 slices").toBe(0);
    } finally {
      if (previous) host.requestAnimationFrame = previous;
      else delete host.requestAnimationFrame;
    }
  });

  it("declares the slice-yield law language-agnostically", () => {
    expect(intakeFixture.laws).toContain("budget-slice-is-resumable");
    expect(intakeFixture.laws).toContain("slice-yield-is-a-task-not-a-frame");
  });

  it("faults only past the whole-document ceiling, and names the phase and the step that crossed it", async () => {
    const ceiling = RETAINED_UI_INTAKE_SLICE_STEPS * 2;
    const cursor = new WgpuUiIntakeCursor(ceiling);
    for (let step = 0; step < ceiling; step += 1) {
      const pending = cursor.next("intake");
      if (pending) await pending;
    }
    expect(cursor.steps, "two whole slices are spent without a fault").toBe(ceiling);
    expect(() => cursor.next("intake"), "the ceiling is terminal, and it names the phase and the step that crossed it").toThrow(`wgpu-ui.intake-budget-exhausted:intake:${ceiling + 1}`);
  });
});
