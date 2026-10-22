/// <reference types="@webgpu/types" />
/** 🎨️ Actual Three/WebGPU pixel comparison for the shared World3d shading fixture. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runReferenceVisualOracle } from "./🔭️reference-visual-oracle/🟦️.ts";
import { runGridVisualOracle } from "./🕸️grid-visual-oracle/🟦️.ts";
import { runSceneShadingOracle } from "./📐️scene-shading-oracle/🟦️.ts";
import { runAuthoredSurfaceOracle } from "./🪟️authored-surface-oracle/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

declare const THREE: any;
declare const worldSurfaceMaterial: (name: string, material: any, vertexColors: boolean) => any;

/** 🌐️ The names the browser bundle's own prelude supplies to the oracle bodies below. Each
 * `run*Oracle` stringifies one of these functions (`Function.prototype.toString`) and prepends the
 * matching `import` lines before Bun bundles the pair for the page, so these bindings exist in the
 * BROWSER and are never resolved in this driver process — exactly like `THREE` above. */
declare const React: typeof import("react");
declare const createRoot: typeof import("react-dom/client").createRoot;
declare const Canvas: React.ComponentType<Record<string, unknown>>;
declare const Grid: React.ComponentType<Record<string, unknown>>;

/** 🧪️ Validates the recorded reference and production WGSL on an actual browser GPU. */
export class SceneShadingPixelCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (segments.length === 1 && segments[0] === "surface") {
      if (!artifacts) throw new Error("surface pixel check requires SEMIO_TEST_ARTIFACT_DIR");
      await runAuthoredSurfaceOracle(this.repoRoot, join(artifacts, "authored-surface-pixels"));
      return;
    }
    if (segments.length === 1 && segments[0] === "reference-visual") {
      await runReferenceVisualOracle(this.repoRoot, artifacts ? join(artifacts, "world3d-reference-visual", "three-reference") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "reference-visual-wgpu") {
      await runReferenceVisualOracle(this.repoRoot, artifacts ? join(artifacts, "world3d-reference-visual", "production-wgpu") : undefined, true);
      return;
    }
    if (segments.length === 1 && segments[0] === "grid-visual") {
      await runGridVisualOracle(this.repoRoot, artifacts ? join(artifacts, "world3d-grid-visual", "installed-drei") : undefined, false);
      return;
    }
    if (segments.length === 1 && segments[0] === "grid-visual-wgpu") {
      await runGridVisualOracle(this.repoRoot, artifacts ? join(artifacts, "world3d-grid-visual", "production-wgpu") : undefined, true);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-reference") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-oracle", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-oracle") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-material-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-wgpu", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-wgpu") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-current-mesh-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-current-mesh", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-current-mesh") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-ordered-mesh-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-ordered-mesh", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-ordered-mesh") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-painted-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-painted", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-painted") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-celebration-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-celebration", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-celebration") : undefined);
      return;
    }
    if (segments.length) throw new Error("scene-shading-pixel-check accepts only reference-visual, reference-visual-wgpu, grid-visual, grid-visual-wgpu, s2-reference, s2-material-wgpu, s2-current-mesh-wgpu, s2-ordered-mesh-wgpu, s2-painted-wgpu, or s2-celebration-wgpu");
    for (const command of ["shading-oracle", "shading-wgpu"] as const) await runSceneShadingOracle(this.repoRoot, command, artifacts ? join(artifacts, "world3d-scene-shading", command) : undefined);
  }
}

const router = new ScriptRouter(import.meta.dirname).register("scene-shading-pixel-check", SceneShadingPixelCheckScript);

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
