#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-actor` task router: `bun ./📜️script.ts <test|typegen|wasm>`. */
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runCargoTestBudgeted, runCmdStatus, runWasmPackWebBuild, resolveTestLevel } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-actor"], this.repoRoot, rest);
  }
}

//#region 🔖️Typegen
/** 🧬️ Name of the versioned owned-schema export test in `🦀️component.rs`. */
const TYPEGEN_TEST_FILTER = "exports_typescript_bindings";

/** 🎯️ The mirror lives at `<owner>/🤖️generated/🟦️actor.ts`, a sibling of `📦️packages`. */
function generatedBindingsPath(root: string): string {
  return join(root, "..", "..", "🤖️generated", "🟦️actor.ts");
}

function runTypegenExportTest(root: string, outPath: string): void {
  const env = { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  const status = runCmdStatus("cargo", ["test", "--features", "typegen", TYPEGEN_TEST_FILTER], { cwd: root, env, budgetMs: buildBudgetMs() });
  if (status !== 0) {
    console.error("framework-actor typegen: `cargo test --features typegen` failed — see output above.");
    process.exit(status);
  }
}

class TypegenScript extends BundleScript {
  run(): void {
    const outPath = generatedBindingsPath(this.root);
    mkdirSync(dirname(outPath), { recursive: true });
    runTypegenExportTest(this.root, outPath);
    console.log(`framework-actor typescript mirror refreshed -> ${outPath}`);
  }
}
//#endregion 🔖️Typegen

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_ACTOR_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/actor/rs",
      wasmBaseName: "framework_actor",
      shipProfile: "wasm-release",
      pkg: {
        name: "@semio-tech/framework-actor-rs",
        files: ["framework_actor_bg.wasm", "framework_actor.js", "framework_actor.d.ts", "framework_actor_bg.wasm.d.ts"],
        main: "framework_actor.js",
        module: "framework_actor.js",
        types: "framework_actor.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typegen", TypegenScript).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
