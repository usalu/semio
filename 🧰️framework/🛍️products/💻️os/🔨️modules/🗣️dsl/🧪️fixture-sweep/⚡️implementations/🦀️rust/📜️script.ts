#!/usr/bin/env bun
/** 🧭️ `@semio-tech/dsl-fixture-sweep-rs` router: `bun ./📜️script.ts test`. Repo-wide fixture-law
 * sweep (parse→print→reparse fixpoint + canonicalize idempotence) over every real shipped
 * `📚️examples/**` DSL fixture — see `📦️lib.rs`'s module doc for the full design. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-os-kernel-dsl-fixture-sweep"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
