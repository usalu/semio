#!/usr/bin/env bun
import { buildWasmWebV1, readWasmBuildPolicyV1 } from "../../../🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🦀️ Registers the actor package tasks and semantic typegen commands. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { PreviewGeneratedScript, TypegenScript } from "../../🧬️typegen/🏃️execution/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-actor"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

class WasmScript extends BundleScript {
  async run(): Promise<void> {
    await buildWasmWebV1({
      rsDir: this.root,
      logPrefix: "framework/actor/rs",
      wasmBaseName: "framework_actor",
      shipProfile: "wasm-release",
      pkg: { name: "@semio-tech/framework-actor-rs", files: ["framework_actor_bg.wasm", "framework_actor.js", "framework_actor.d.ts", "framework_actor_bg.wasm.d.ts"], main: "framework_actor.js", module: "framework_actor.js", types: "framework_actor.d.ts" },
    }, readWasmBuildPolicyV1(process.env,this.root));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typegen", TypegenScript).register("preview-generated", PreviewGeneratedScript).register("wasm", WasmScript);
await runScriptMain(router, { defaultCommand: "test" });
