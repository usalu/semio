#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-async` task router: `bun ./📜️script.ts <test|typegen>`. */
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runCargoTestBudgeted, runCmdStatus, resolveTestLevel } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-async"], this.repoRoot, rest);
  }
}

//#region 🔖️Typegen
/** 🧬️ Name of the versioned owned-schema export test in `🦀️component.rs`. */
const TYPEGEN_TEST_FILTER = "exports_typescript_bindings";

/** 🎯️ The mirror lives at `<owner>/🤖️generated/🟦️async.ts`, a sibling of `📦️packages`. */
function generatedBindingsPath(root: string): string {
  return join(root, "..", "..", "🤖️generated", "🟦️async.ts");
}

function runTypegenExportTest(root: string, outPath: string): void {
  const env = { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  const status = runCmdStatus("cargo", ["test", "--features", "typegen", TYPEGEN_TEST_FILTER], { cwd: root, env, budgetMs: buildBudgetMs() });
  if (status !== 0) {
    console.error("framework-async typegen: `cargo test --features typegen` failed — see output above.");
    process.exit(status);
  }
}

class TypegenScript extends BundleScript {
  run(): void {
    const outPath = generatedBindingsPath(this.root);
    mkdirSync(dirname(outPath), { recursive: true });
    runTypegenExportTest(this.root, outPath);
    console.log(`framework-async typescript mirror refreshed -> ${outPath}`);
  }
}
//#endregion 🔖️Typegen

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typegen", TypegenScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
