#!/usr/bin/env bun
/** 🦀️ `semio-framework-3d` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoLint, runCargoTestBudgeted, runExactCargoLaws, runCmd } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-3d"], this.repoRoot, rest);
  }
}

class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("canonical-architecture accepts no arguments");
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      groups: [{ package: "semio-framework-3d", target: { kind: "lib" }, laws: ["retained_modeling_jobs_slice_work_and_match_synchronous_geometry"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: buildBudgetMs(),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`canonical-mesh-modeling ${event.stage}: ${event.law ?? ""}`); },
    });
    console.log(`canonical-mesh-modeling receipts=${receipts.length}`);
  }
}

/** 📈️ Runs Cargo's benchmark mode for `semio-framework-3d`; this package declares no standalone benchmark target.
 * The dedicated Brep Criterion benchmark belongs to `semio-s-plugin-stdio` and its own `bench` command. */
class BenchScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["bench", "-p", "semio-framework-3d"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}


/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework-3d --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["semio-framework-3d"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalArchitectureScript).register("test", TestScript).register("bench", BenchScript).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
