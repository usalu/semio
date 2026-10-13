#!/usr/bin/env bun
import { join, resolve } from "node:path";
import { Script, ScriptRouter, findWorkspaceRoot, scriptInvocationBudget } from "../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { readBuildMode } from "../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { buildBudgetMs } from "../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";

/** 🏗️ Executes workspace build leaves without importing repository audits. */
export class BuildScript extends Script {
  async run(segments: string[]): Promise<void> {
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), slice = segments[0];
    if (!slice) {
      console.log(`[build] Nx completed the ${mode} workspace build graph.`);
      return;
    }
    if (slice === "canvas") {
      if (segments.length !== 2 || segments[1] !== "fonts") throw Error("Expected build canvas fonts");
      const cwd = join(this.repoRoot, "🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust");
      const { repositoryProcessOwnerContextV1 } = await import("../🟦️.ts");
      const { repositoryCargoArtifactBuildPolicyV1 } = await import("../⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts");
      const { repositoryCargoPreparationStorageV1, prepareCargoWorkspaceInvocation } = await import("../🗂️workspaces/🦀️cargo/🟦️.ts");
      const { devToolingEnv } = await import("../🏃️process/🌿️environment/🟦️.ts");
      prepareCargoWorkspaceInvocation(repositoryCargoPreparationStorageV1(this.repoRoot), this.repoRoot, ["build", "--manifest-path", join(cwd, "Cargo.toml")], this.repoRoot, process.env);
      const env = devToolingEnv({ SEMIO_PROCESS_OWNER_CONTEXT: JSON.stringify(repositoryProcessOwnerContextV1(cwd)), SEMIO_CARGO_ARTIFACT_POLICY: JSON.stringify(repositoryCargoArtifactBuildPolicyV1(cwd, this.repoRoot)) });
      await runOwnedCommand(process.execPath, [join(cwd, "📜️script.ts"), "build"], cwd, "canvas-font-assets:build", scriptInvocationBudget(this.invocation, buildBudgetMs()), { env, signal: this.invocation.control.signal });
      return;
    }
    if (slice === "storybook") {
      const { assertNoOwnedStorybookMdx, assertUiStorybookDiscovery } = await import("./📖️storybook/🟦️.ts");
      assertNoOwnedStorybookMdx(this.repoRoot);
      const [{ mkdir, mkdtemp, rm }, { collectArtifactFiles }, { stageRepositoryArtifacts }, { withCompilerState }, { repoCacheDirectory }, { createRequire }] = await Promise.all([
        import("node:fs/promises"), import("../../../../../🔨️modules/🏃️process/📦️artifacts/🗂️files/🟦️.ts"), import("../⚡️caching/📦️artifacts/🟦️.ts"), import("../⚡️caching/🏗️state/🟦️.ts"), import("../⚡️caching/🟦️.ts"), import("node:module"),
      ]);
      const require = createRequire(join(this.repoRoot, "package.json")), output = resolve(this.repoRoot, process.env.STORYBOOK_OUTPUT_DIR ?? `dist/${mode}/storybook`);
      const temporaryRoot = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated") : repoCacheDirectory(this.repoRoot, "storybook-staging");
      await mkdir(temporaryRoot, { recursive: true });
      const temporary = await mkdtemp(join(temporaryRoot, "storybook-"));
      try {
        await withCompilerState({ root: repoCacheDirectory(this.repoRoot), owner: "storybook", compiler: `storybook:${require("storybook/package.json").version}:bun:${process.versions.bun}`, configuration: process.env.STORYBOOK_SCOPE ?? "", mode, signal: this.invocation.control.signal }, async directory => {
          await runOwnedCommand(process.execPath, ["x", "storybook", "build", "-c", ".storybook", "--output-dir", temporary], this.repoRoot, "storybook:build", scriptInvocationBudget(this.invocation, buildBudgetMs()), { signal: this.invocation.control.signal, env: { ...process.env, SEMIO_VITE_CACHE_DIRECTORY: directory } });
          await stageRepositoryArtifacts(output, `storybook:${mode}`, await collectArtifactFiles(temporary, this.invocation.control.signal), { signal: this.invocation.control.signal });
        });
      } finally { await rm(temporary, { recursive: true, force: true }); }
      assertUiStorybookDiscovery(this.repoRoot);
      return;
    }
    const targets: Readonly<Record<string, string>> = { assets: "@semio-tech/assets:build", "repo-cli": "@semio-tech/repo-client:build", "repo-server": "@semio-tech/repo-coordinator:build", "repo-vscode": "@semio-tech/repo-vscode:build-vsix" };
    const target = targets[slice];
    if (!target) throw Error(`Unknown workspace build selection: ${slice}`);
    const { runOwnedNxInvocationV1 } = await import("../⚡️caching/🚀️bootstrap/📜️script.ts");
    await runOwnedNxInvocationV1(this.repoRoot, ["run", target, `--configuration=${mode}`, ...(segments.length > 1 ? ["--", ...segments.slice(1)] : [])], this.invocation);
  }
}

/** 🧪️ Runs the independently bundled workspace build ownership contract. */
class TestScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Workspace build tests accept no arguments");
    await runOwnedCommand(process.execPath, ["test", resolve(import.meta.dir, "🧪️tests/🟦️.ts")], this.repoRoot, "workspace-build:contract", scriptInvocationBudget(this.invocation, 300_000), { signal: this.invocation.control.signal });
  }
}

if (import.meta.main) {
  const root = findWorkspaceRoot(import.meta.dir);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(root, root).register("build", BuildScript).register("test", TestScript), { invocation: original }));
}
