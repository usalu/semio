#!/usr/bin/env repo
/** 🏘️ Runs every cross-plugin composition law; plugins may not meet anywhere else. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runVitest } from "@semio-tech/process/vitest";
import { runRepositoryCommand } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🧪️ Executes the composition laws with the repository's owned command runner. */
class CompositionTestScript extends BundleScript {
  async run(): Promise<void> {
    const tests = ["🔱️trinity-graph-child/🟦️.ts", "🌀️procedural-formats/🧪️laws/🟦️.ts", "🖨️raster-decoded-image/🟦️.ts", "🚢️stdio-shipped-fleet/🟦️.ts", "🚀️procedural-renderer-boot/🟦️.ts", "🎯️raster-pixel-selection/🟦️.ts", "🎨️draw-canvas-svg-agreement/🟦️.ts", "📷️camera-presence/🟦️.ts", "🗺️gismap-shell-history-route/🟦️.ts"].map((file) => join(import.meta.dir, file));
    await runRepositoryCommand(this.invocation, process.execPath, ["test", ...tests], this.repoRoot, "plugin-composition", { version: 1, maximumElapsedMilliseconds: 0, output: null });
    await runVitest(this.root, [], "./🎚️renderer-contract/🟦️.ts");
  }
}

/** 🛍️ Runs the mounted Layout transport contracts through the renderer owner. */
class LayoutCatalogueTestScript extends BundleScript {
  async run(): Promise<void> {
    await runVitest(this.root, ["🛍️layout-canvas-catalogue/🟦️.ts"], "./🎚️renderer-contract/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", CompositionTestScript).register("test-layout-catalogue", LayoutCatalogueTestScript);
await receiveScriptProcessInvocation(process.env, (original) => runScriptMain(router, { invocation: original }));
