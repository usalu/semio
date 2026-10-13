#!/usr/bin/env bun
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { withCompilerState } from "../../../../📚️library/⚡️caching/🏗️state/🟦️.ts";
import { repoCacheDirectory } from "../../../../📚️library/⚡️caching/🟦️.ts";
import { collectArtifactFiles } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🗂️files/🟦️.ts";
import { stageRepositoryArtifacts } from "../../../../📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { mkdir, mkdtemp, rm, cp, writeFile } from "node:fs/promises";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-vscode` router: `bun ./📜️script.ts <dev|test [level]|build|lint|build-vsix>`. */
import { build } from "vite";
import { extensionBuildConfig, extensionPackageEnvironment } from "../../🏗️builder/🟦️.ts";
import { TEST_LEVELS } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runBunx } from "../../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

//#region Build
/** 🧩️Builds the extension host entry and its extension-host test bundle. */
async function buildExtension(root: string, watch: boolean, directory = "out"): Promise<void> {
  await build(extensionBuildConfig(root, "🟦️.ts", directory, "extension.js", watch));
  if (!watch) await build(extensionBuildConfig(root, "../../🧪️tests/🧩️extension/🟦️.ts", join(directory, "test"), "extension.test.js", false));
}
//#endregion

class DevScript extends BundleScript {
  async run(): Promise<void> {
    await buildExtension(this.root, true);
  }
}

/** ⏱️The extension-host Mocha case (`🧪️tests/🧩️extension/🟦️.ts`) is this package's only test and can
 * only run inside the VSCode test harness, so it is added at `long` and above. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level } = resolveTestLevel(segments);
    if (TEST_LEVELS.indexOf(level) < TEST_LEVELS.indexOf("long")) return;
    runBunx(["vscode-test", "--config", "../../🧪️tests/🎚️config/🟨️.mjs"], this.root);
  }
}

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), temporaryRoot = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated") : repoCacheDirectory(this.repoRoot, "vscode-staging");
    await mkdir(temporaryRoot, { recursive: true });
    const temporary = await mkdtemp(join(temporaryRoot, "vscode-"));
    try {
      await withCompilerState({ root: repoCacheDirectory(this.repoRoot), owner: "repo-vscode:bundle", compiler: `vite:${createRequire(join(this.root, "package.json"))("vite/package.json").version}`, configuration: this.root, mode, signal: this.invocation.control.signal }, async () => {
        await buildExtension(this.root, false, temporary);
        await stageRepositoryArtifacts(join(this.root, "dist", mode, "extension"), `repo-vscode:${mode}`, await collectArtifactFiles(temporary, this.invocation.control.signal), { signal: this.invocation.control.signal });
      });
    } finally { await rm(temporary, { recursive: true, force: true }); }
  }
}

class LintScript extends BundleScript {
  run(): void {
    runBunx(["eslint", "--max-warnings", "0", "--config", "../../../../🧹️lint/📐️source-policy/🟨️.mjs", "."], this.root);
  }
}

/** 📦️Packages the VSIX; nx's `dependsOn: ["build"]` runs the build target first. */
class BuildVsixScript extends BundleScript {
  async run(): Promise<void> {
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), temporaryRoot = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated") : repoCacheDirectory(this.repoRoot, "vscode-staging");
    await mkdir(temporaryRoot, { recursive: true });
    const temporary = await mkdtemp(join(temporaryRoot, "vsix-"));
    try {
      await cp(join(this.root, "dist", mode, "extension"), join(temporary, "out"), { recursive: true });
      await cp(join(this.root, "LICENSE.md"), join(temporary, "LICENSE.md"));
      await cp(join(this.root, ".vscodeignore"), join(temporary, ".vscodeignore"));
      const manifest = JSON.parse(readFileSync(join(this.root, "package.json"), "utf8"));
      delete manifest.scripts;
      await writeFile(join(temporary, "package.json"), JSON.stringify(manifest));
      const require = createRequire(join(this.root, "package.json"));
      await runOwnedCommand(process.execPath, [require.resolve("@vscode/vsce/vsce"), "package", "--no-dependencies", "--out", join(temporary, "🧩️repo.vsix")], temporary, "repo-vscode:vsix", 120_000, { env: extensionPackageEnvironment(process.env), signal: this.invocation.control.signal });
      await stageRepositoryArtifacts(join(this.root, "dist", mode, "vsix"), `repo-vscode:vsix:${mode}`, new Map([["🧩️repo.vsix", join(temporary, "🧩️repo.vsix")]]), { signal: this.invocation.control.signal });
    } finally { await rm(temporary, { recursive: true, force: true }); }
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("test", TestScript)
  .register("build", BuildScript)
  .register("lint", LintScript)
  .register("build-vsix", BuildVsixScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
