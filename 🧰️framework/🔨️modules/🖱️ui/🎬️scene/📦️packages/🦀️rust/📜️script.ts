#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/⏱️budget/🟦️.ts";
/** ⚙️ Runs the `semio-framework-ui-scene` test suite and the guest-target compile gates.
 *
 * This crate carries the pack-encoded `SurfaceDoc` payload every wasm32-wasip2 plugin component and
 * wasm32-unknown-unknown browser renderer moves across the `Component::Surface` boundary, so a
 * dependency that fails either guest target is a design error — see `ui_runtime`'s own `script.ts`
 * header for why native `cargo check` cannot see that on its own. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));

//#region 🔖️test
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: [], cwd: this.root, extraArgs: ["--all-features", ...rest] }, readCargoTestPolicyV1(process.env));
  }
}
//#endregion 🔖️test

//#region 🔖️check-wasm
/** 🌐️ Both guest flavours: wasip2 (plugin components) and unknown-unknown (browser renderers). */
class CheckWasmScript extends BundleScript {
  async run(): Promise<void> {
    const check = (args: string[]) => runOwnedCommand("cargo", ["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", "semio-framework-ui-scene", ...args], packageRoot, "tool:owner", buildBudgetMs(), {env: process.env});
    await check(["--target", "wasm32-wasip2"]);
    await check(["--target", "wasm32-unknown-unknown"]);
  }
}
//#endregion 🔖️check-wasm

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
