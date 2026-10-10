import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type { ScriptControl } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { startNativeProgress } from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { relative, resolve } from "node:path";
import { BundleScript, ScriptRouter, scriptInvocationBudget, type ScriptCommand } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { artifactRustCargoArguments, validateNativeCargoArguments } from "../🎛️native-input/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../🏗️native-build/🟦️.ts";

/**
 * 🧪️ Runs an artifact crate through the repository's level filters, Nextest profile and assertion budget.
 *
 * 🎛️ `testFeatures` names the Cargo features whose `#[cfg(feature = …)]` trees hold tests that would
 * otherwise never compile, let alone run: a crate whose app assembly is feature-gated reports a
 * cheerful green over a fraction of its suite without them. They go in front of the caller's own
 * arguments and reach Nextest as build selectors (`--features` is a required build option in
 * `partitionNextestExecutionFilters`), so the warm build and the execution pass agree.
 */
export async function runArtifactRustTests(cargoName: string, repoRoot: string, segments: string[], control: ScriptControl, testFeatures: readonly string[] = []): Promise<void> {
  const { runRepositoryCargoTests } = await import("../../../🟦️.ts");
  const { rest } = resolveTestLevel(segments);
  validateNativeCargoArguments("test", rest);
  const featureArgs = testFeatures.flatMap((feature) => ["--features", feature]);
  const stopProgress = startNativeProgress(`artifact-rust:${cargoName}:test`);
  try { await runRepositoryCargoTests([cargoName], repoRoot, control, [...featureArgs, ...rest]); }
  finally { stopProgress(); }
}

/**
 * 🔬️ One artifact's TypeScript twin: the second, independent implementation of a law whose fixture
 * the Rust suite also answers. Returns how many assertions it carried, so a twin that silently
 * stopped asserting is visible as a collapsing count rather than a passing run.
 */
export type ArtifactTwinSelfTest = { readonly name: string; readonly run: () => number };

/** 📦️ Runs an independently owned Rust artifact package through the shared Nx-native contract. */
export async function runArtifactRustPackageMain(packageRoot: string, cargoName: string, options: { readonly testFeatures?: readonly string[]; readonly twins?: readonly ArtifactTwinSelfTest[]; readonly commands?: Readonly<Record<string, ScriptCommand>>; readonly testCommands?: Readonly<Record<string, ScriptCommand>>; readonly snapshotSqliteTests?: readonly string[]; readonly snapshotSqliteTestFeatures?: readonly string[]; readonly snapshotSqliteTestBudgetMs?: number; readonly snapshotSqliteTestGroups?: readonly (readonly string[])[] } = {}): Promise<void> {
  return receiveScriptProcessInvocation(process.env, async original => {
  if (options.snapshotSqliteTestBudgetMs !== undefined && (!Number.isSafeInteger(options.snapshotSqliteTestBudgetMs) || options.snapshotSqliteTestBudgetMs <= 0)) throw new Error("Owned SQLite snapshot test budget must be a positive safe integer");
  const snapshotSources = options.snapshotSqliteTests?.map(path => resolve(packageRoot, path)) ?? [];
  const snapshotGroups = options.snapshotSqliteTestGroups?.map(group => group.map(path => resolve(packageRoot, path))) ?? [snapshotSources];
  if (options.snapshotSqliteTestGroups !== undefined) {
    const selected = snapshotGroups.flat(), owned = new Set(snapshotSources);
    if (!snapshotGroups.length || snapshotGroups.some(group => !group.length) || owned.size !== snapshotSources.length || selected.length !== snapshotSources.length || new Set(selected).size !== selected.length || selected.some(path => !owned.has(path))) throw new Error("Owned SQLite snapshot source groups must include every owned source exactly once");
  }
  class BuildScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const { cargoArgs } = artifactRustCargoArguments("build", segments);
      await buildRepositoryCargoArtifacts(relative(this.repoRoot, resolve(this.root, "Cargo.toml")), cargoArgs, this.repoRoot);
    }
  }
  class CheckScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const { cargoArgs } = artifactRustCargoArguments("check", segments);
      await runRepositoryCommand("cargo", ["check", "--locked", "--manifest-path", resolve(this.root, "Cargo.toml"), ...cargoArgs], this.repoRoot, `artifact-rust:${cargoName}:check`, scriptInvocationBudget(this.invocation, 0), { signal: this.invocation.control.signal });
    }
  }
  class TestScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments[0] && options.testCommands && Object.hasOwn(options.testCommands, segments[0])) {
        const router = new ScriptRouter(this.root, this.repoRoot);
        for (const [name, Command] of Object.entries(options.testCommands)) router.register(name, Command);
        await router.run(segments, this.invocation);
        return;
      }
      for (const twin of options.twins ?? []) console.log(`[TRACE] ${twin.name}-twin checks=${twin.run()}`);
      await runArtifactRustTests(cargoName, this.repoRoot, segments, this.invocation.control, options.testFeatures);
    }
  }
  class CanonicalArchitectureScript extends BundleScript {
    run(): void {
      for (const twin of options.twins ?? []) console.log(`artifact-contribution: ${twin.name} checks=${twin.run()}`);
    }
  }
  class SnapshotSqliteTestScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const mode = segments[0];
      if (segments.length > 1 || (mode !== undefined && mode !== "source" && mode !== "native")) throw new Error("Unknown owned SQLite snapshot command");
      if (mode !== "source") await runArtifactRustTests(cargoName, this.repoRoot, ["--lib", "sqlite_snapshot_", "--no-fail-fast"], this.invocation.control, options.snapshotSqliteTestFeatures ?? options.testFeatures);
      if (mode !== "native") {
        const { runRepositoryTestCommand } = await import("../../../🟦️.ts");
        for (const group of snapshotGroups) await runRepositoryTestCommand(process.execPath, ["test", ...group], { cwd: this.repoRoot, budgetMs: scriptInvocationBudget(this.invocation, options.snapshotSqliteTestBudgetMs ?? 0), signal: this.invocation.control.signal });
      }
    }
  }
  const packageRouter = new ScriptRouter(packageRoot).register("build", BuildScript).register("check", CheckScript).register("test", TestScript);
  if (options.snapshotSqliteTests?.length) packageRouter.register("test-snapshot-sqlite", SnapshotSqliteTestScript);
  for (const [name, Command] of Object.entries(options.commands ?? {})) packageRouter.register(name, Command);
  if (options.twins?.length) packageRouter.register("canonical-architecture", CanonicalArchitectureScript);
  const segments = process.argv.slice(2);
  await packageRouter.run(segments.length ? segments : ["test"], original);
  });
}
