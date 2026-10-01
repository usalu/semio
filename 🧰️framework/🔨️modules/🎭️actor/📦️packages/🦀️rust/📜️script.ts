#!/usr/bin/env bun
/** 🦀️ Registers the actor package tasks and semantic typegen commands. */
import { resolveTestLevel, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { PreviewGeneratedScript, TypegenScript } from "../../🧬️typegen/🏃️execution/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-actor"], this.repoRoot, rest);
  }
}

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      logPrefix: "framework/actor/rs",
      wasmBaseName: "framework_actor",
      shipProfile: "wasm-release",
      pkg: { name: "@semio-tech/framework-actor-rs", files: ["framework_actor_bg.wasm", "framework_actor.js", "framework_actor.d.ts", "framework_actor_bg.wasm.d.ts"], main: "framework_actor.js", module: "framework_actor.js", types: "framework_actor.d.ts" },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typegen", TypegenScript).register("preview-generated", PreviewGeneratedScript).register("wasm", WasmScript);
await runScriptMain(router, { defaultCommand: "test" });
