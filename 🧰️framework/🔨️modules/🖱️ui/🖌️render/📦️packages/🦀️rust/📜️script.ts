#!/usr/bin/env bun
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { resolve } from "node:path";
import { mkdirSync, mkdtempSync } from "node:fs";
import { captureOwnedProcess } from "../../../../🏃️process/📥️capture/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/⏱️budget/🟦️.ts";
/** ⚙️ Runs the `semio-framework-ui-render` suite, the browser-target gate, and the dependency
 * boundary assertion that keeps this crate backend-neutral.
 *
 * `boundaries` is the load-bearing one: the whole point of this crate is that it describes frames
 * without owning a device, so `wgpu`, `winit` and every graphics binding must be absent from its
 * dependency tree. That is a property no type signature can express, so it is asserted here. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));

/** 🚫️ Crates that must never appear in this crate's dependency tree, and why. */
const FORBIDDEN_DEPENDENCIES: ReadonlyArray<readonly [string, string]> = [
  ["wgpu", "belongs to the browser backend target only — natively we hand-write D3D12/Metal/Vulkan"],
  ["winit", "windowing belongs to semio-framework-ui-host"],
  ["semio-framework-actor", "the renderer must not own or link the actor kernel"],
  ["memmap2", "fontique path/mmap font sources link std::fs, so every guest that shapes text would import wasi:filesystem, which no closed browser actor admits (fonts are embedded bytes)"],
];

//#region 🔖️test
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: [], cwd: this.root, extraArgs: ["--all-features", ...rest] }, readCargoTestPolicyV1(process.env));
    await runOwnedCommand("bun", ["../../🧪️tests/🖼️webgpu-surface/🟨️.js"], packageRoot, "tool:owner", buildBudgetMs(), {env: process.env});
  }
}
//#endregion 🔖️test

//#region 🔖️check-wasm
class CheckWasmScript extends BundleScript {
  async run(): Promise<void> {
    await runOwnedCommand("cargo", ["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", "semio-framework-ui-render", "--target", "wasm32-unknown-unknown"], packageRoot, "tool:owner", buildBudgetMs(), {env: process.env});
  }
}
//#endregion 🔖️check-wasm

//#region 🔖️boundaries
/** 🧭️ Fails when a forbidden crate reaches this crate's normal (non-dev, non-build) dep tree. */
class BoundariesScript extends BundleScript {
  async run(): Promise<void> {
    const { artifactDirectory } = readCargoTestPolicyV1(process.env);
    mkdirSync(artifactDirectory, { recursive: true });
    const evidence = mkdtempSync(resolve(artifactDirectory, "ui-render-boundaries-"));
    let cancelled = false;
    const cancel = (): void => { cancelled = true; };
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    let result;
    try {
      console.log("Inspecting ui-render normal dependencies (0/1).");
      result = await captureOwnedProcess("cargo", ["tree", "--manifest-path", resolve(packageRoot, "Cargo.toml"), "-p", "semio-framework-ui-render", "--edges", "normal", "--prefix", "none", "--format", "{p}"], { cwd: packageRoot, env: process.env, budgetMs: buildBudgetMs(), maxOutputBytes: 4 * 1024 * 1024, stdoutPath: resolve(evidence, "stdout.log"), stderrPath: resolve(evidence, "stderr.log"), cancelled: () => cancelled });
    } finally {
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
    if (result.reason !== "exit" || result.status !== 0) throw Error(`Cannot inspect ui-render dependencies: ${result.reason} (${result.status}). ${result.stderr}`);
    const dependencies = new Set(result.stdout.trim().split(/\r?\n/u).filter(Boolean).map(line => line.trim().split(/\s+/u)[0]));
    if (!dependencies.has("semio-framework-ui-render")) throw Error("Cargo dependency inventory does not contain its requested ui-render root");
    const violations: string[] = [];
    for (const [crate, reason] of FORBIDDEN_DEPENDENCIES) {
      if (dependencies.has(crate)) violations.push(`${crate}: ${reason}`);
    }
    if (violations.length > 0) {
      console.error("semio-framework-ui-render must stay backend-neutral, but its dependency tree contains:");
      for (const violation of violations) console.error(`  - ${violation}`);
      throw Error("ui-render dependency boundaries are violated");
    }
    console.log(`ui-render dependency boundaries hold (1/1; ${FORBIDDEN_DEPENDENCIES.length} forbidden crates absent).`);
  }
}
//#endregion 🔖️boundaries

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript).register("boundaries", BoundariesScript);
  await runScriptMain(router);
}
