#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework` task router: `bun ./📜️script.ts test|generate|check|lint`. */
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runCargoLint, runCargoTestBudgeted, runCmdStatus, runVitest, resolveTestLevel } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { mkdirSync } from "node:fs";
import { join } from "node:path";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework"], this.repoRoot, rest);
    await runVitest(this.root, rest, "../🟦️typescript/🧪️vitest.config.ts");
  }
}

/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["semio-framework"], this.root, segments);
  }
}

//#region 🔖️Typegen
const TYPEGEN_TEST_FILTER = "exports_typescript_bindings";

function generatedManifestPath(root: string): string {
  return join(root, "..", "..", "🔨️modules", "🛂️manifest", "🤖️generated", "🟦️manifest.ts");
}

/** 🧬️ Runs the owned framework schema export test, optionally writing its stable projection. */
function runTypegenExportTest(root: string, outPath?: string): void {
  const env = outPath === undefined ? process.env : { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  const status = runCmdStatus("cargo", ["test", "--features", "typegen", TYPEGEN_TEST_FILTER], {
    cwd: root,
    env,
    budgetMs: buildBudgetMs(),
  });
  if (status !== 0) {
    console.error("framework typegen: owned schema export failed — see output above.");
    process.exit(status);
  }
}

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const outPath = generatedManifestPath(this.root);
    mkdirSync(join(this.root, "..", "..", "🔨️modules", "🛂️manifest", "🤖️generated"), { recursive: true });
    runTypegenExportTest(this.root, outPath);
    console.log(`framework typescript mirror refreshed -> ${outPath}`);
  }
}

/** 🔎️ Validates metadata and byte-compares the owned projection with the committed mirror. */
class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    runTypegenExportTest(this.root);
    console.log("framework typescript mirror is fresh.");
  }
}
//#endregion 🔖️Typegen

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("generate", GenerateScript).register("check", CheckScript).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
