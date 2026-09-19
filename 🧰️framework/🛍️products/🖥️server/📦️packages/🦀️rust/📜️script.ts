#!/usr/bin/env bun
/** 🖥️ `semio-framework-server` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 *
 * `test` runs the crate's own suite **and** a `--features conformance` check, because the
 * conformance module is `cfg(any(test, feature = "conformance"))`: `cargo test` compiles it through
 * the `test` half, so a break in the feature half — the half every instance crate consumes as a
 * dev-dependency — would otherwise stay invisible until hub's own build hit it. Nothing else depends
 * on this crate, which is exactly how 28 errors once survived a month here
 * (`📓️h2-server-crate-and-wave3-memo.md` §B.7 step 14). */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { buildCargoArtifacts } from "../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargo(["test", "-p", "semio-framework-server", ...rest], this.repoRoot);
    runCargo(["check", "-p", "semio-framework-server", "--features", "conformance"], this.repoRoot);
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
