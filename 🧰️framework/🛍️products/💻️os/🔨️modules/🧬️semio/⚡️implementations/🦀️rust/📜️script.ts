#!/usr/bin/env bun
/** @emoji 🧬 Runs `cargo test` / `semio` CLI for `semio-framework-os-kernel-semio`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel, getWorkspaceRoot, runCmd, buildBudgetMs } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-os-kernel-semio"], import.meta.dir, rest);
  }
}

function semioCargo(subcommand: string, segments: string[]): void {
  const root = getWorkspaceRoot();
  runCmd("cargo", ["run", "-p", "semio-framework-os-kernel-semio", "--bin", "semio", "--", subcommand, ...segments], {
    cwd: root,
    budgetMs: buildBudgetMs(),
  });
}

class InspectScript extends BundleScript {
  run(segments: string[]): void {
    semioCargo("inspect", segments);
  }
}

class VerifyFileScript extends BundleScript {
  run(segments: string[]): void {
    semioCargo("verify", segments);
  }
}

class OpenScript extends BundleScript {
  run(segments: string[]): void {
    semioCargo("open", segments);
  }
}

class ConvertScript extends BundleScript {
  run(segments: string[]): void {
    semioCargo("convert", segments);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir)
    .register("test", TestScript)
    .register("inspect", InspectScript)
    .register("verify", VerifyFileScript)
    .register("open", OpenScript)
    .register("convert", ConvertScript);
  await runBundleScriptMain(router, import.meta.url);
}
