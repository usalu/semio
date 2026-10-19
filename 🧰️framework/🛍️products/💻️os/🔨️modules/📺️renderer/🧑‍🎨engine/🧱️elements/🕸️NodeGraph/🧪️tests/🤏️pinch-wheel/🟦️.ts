/**
 * 🤏️ The node-graph pinch lane: a shared-recognizer pinch step becomes the dag/flow session's OWN wheel
 * calls — whole zoom notches about the centroid, then one non-zoom pan — and the notch factors are the
 * schema tokens both Rust engines apply. No wasm: the plan is pure, and the parity rows read the Rust
 * sources off disk so a factor drift in either engine fails here.
 *
 * @see `🐍️u4-pinch-diagram-contrast-probe.mjs` (ticket 26/09/18) — the live twin on the `s` dag window.
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { GRAPH_WHEEL_ZOOM_NOTCH, graphPinchWheelPlan } from "../../🟦️.tsx";

/** 🗂️ The repo root, walked up from the runner's cwd (`import.meta.url` is a vite `/@fs` URL here). */
const repoRoot = findWorkspaceRoot(process.cwd());
const step = (scale: number, panX = 0, panY = 0) => ({ scale, panX, panY, rotation: 0, centroidX: 320, centroidY: 240 });

describe("🤏️ node-graph pinch → wheel plan", () => {
  it("replays a spread as whole zoom-in notches anchored at the centroid, carrying the remainder", () => {
    const plan = graphPinchWheelPlan(step(GRAPH_WHEEL_ZOOM_NOTCH.in ** 2), 0);
    expect(plan.calls).toEqual([
      { sx: 320, sy: 240, deltaX: 0, deltaY: -1, zoomGesture: true },
      { sx: 320, sy: 240, deltaX: 0, deltaY: -1, zoomGesture: true },
    ]);
    expect(Math.abs(plan.pendingLogScale)).toBeLessThan(1e-9);
  });

  it("a pinch-in becomes zoom-out notches; a sub-notch pinch waits for the next frame instead of dropping", () => {
    expect(graphPinchWheelPlan(step(GRAPH_WHEEL_ZOOM_NOTCH.out), 0).calls).toEqual([{ sx: 320, sy: 240, deltaX: 0, deltaY: 1, zoomGesture: true }]);
    const first = graphPinchWheelPlan(step(1.06), 0);
    expect(first.calls).toEqual([]);
    const second = graphPinchWheelPlan(step(1.06), first.pendingLogScale);
    expect(second.calls.filter((call) => call.zoomGesture)).toHaveLength(1);
  });

  it("the centroid travel becomes ONE non-zoom wheel whose delta keeps the graph under the fingers (camera -= delta / zoom)", () => {
    expect(graphPinchWheelPlan(step(1, 30, -12), 0).calls).toEqual([{ sx: 320, sy: 240, deltaX: 30, deltaY: -12, zoomGesture: false }]);
    expect(graphPinchWheelPlan(step(1), 0).calls).toEqual([]);
  });

  it("the notch factors are the schema camera tokens both Rust engines zoom by", () => {
    const tokens = readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs"), "utf8");
    expect(tokens).toContain(`pub const WHEEL_ZOOM_IN_FACTOR: f64 = ${GRAPH_WHEEL_ZOOM_NOTCH.in};`);
    expect(tokens).toContain(`pub const WHEEL_ZOOM_OUT_FACTOR: f64 = ${GRAPH_WHEEL_ZOOM_NOTCH.out};`);
    const flow = readModuleTreeSource(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/✏️host-authoring/🦀️.rs"));
    expect(flow).toMatch(/WHEEL_ZOOM_IN_FACTOR \} else \{ ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR/u);
    const camera = readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖼️canvas/📷️camera/🦀️.rs"), "utf8");
    expect(camera).toMatch(/if delta_y < 0\.0 \{ ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR \} else \{ ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR \}/u);
  });
});
