#!/usr/bin/env bun
import { relative, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand, startNativeProgress } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { artifactRustCargoArguments, validateNativeCargoArguments } from "../🎛️native-input/🟦️.ts";
import { buildCargoArtifacts } from "../🏗️native-build/🟦️.ts";

/**
 * 🧪️ Runs an artifact crate through the repository's level filters, Nextest profile and assertion budget.
 *
 * 🎛️ `testFeatures` names the Cargo features whose `#[cfg(feature = …)]` trees hold tests that would
 * otherwise never compile, let alone run: a crate whose app assembly is feature-gated reports a
 * cheerful green over a fraction of its suite without them. They go in front of the caller's own
 * arguments and reach Nextest as build selectors (`--features` is a required build option in
 * `partitionNextestExecutionFilters`), so the warm build and the execution pass agree.
 */
export async function runArtifactRustTests(cargoName: string, repoRoot: string, segments: string[], testFeatures: readonly string[] = []): Promise<void> {
  const { resolveTestLevel, runCargoTestBudgeted } = await import("../../../📦️packages/🟦️typescript/🟦️.ts");
  const { rest } = resolveTestLevel(segments);
  validateNativeCargoArguments("test", rest);
  const featureArgs = testFeatures.flatMap((feature) => ["--features", feature]);
  const stopProgress = startNativeProgress(`artifact-rust:${cargoName}:test`);
  try { await runCargoTestBudgeted([cargoName], repoRoot, [...featureArgs, ...rest]); }
  finally { stopProgress(); }
}

/** 📦️ Runs an independently owned Rust artifact package through the shared Nx-native contract. */
export async function runArtifactRustPackageMain(packageRoot: string, cargoName: string, options: { readonly testFeatures?: readonly string[] } = {}): Promise<void> {
  class BuildScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const { cargoArgs } = artifactRustCargoArguments("build", segments);
      await buildCargoArtifacts(relative(this.repoRoot, resolve(this.root, "Cargo.toml")), cargoArgs, this.repoRoot);
    }
  }
  class CheckScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const { cargoArgs } = artifactRustCargoArguments("check", segments);
      await runOwnedCommand("cargo", ["check", "--locked", "--manifest-path", resolve(this.root, "Cargo.toml"), ...cargoArgs], this.repoRoot, `artifact-rust:${cargoName}:check`);
    }
  }
  class TestScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      await runArtifactRustTests(cargoName, this.repoRoot, segments, options.testFeatures);
    }
  }
  const packageRouter = new ScriptRouter(packageRoot).register("build", BuildScript).register("check", CheckScript).register("test", TestScript);
  const segments = process.argv.slice(2);
  await packageRouter.run(segments.length ? segments : ["test"]);
}
