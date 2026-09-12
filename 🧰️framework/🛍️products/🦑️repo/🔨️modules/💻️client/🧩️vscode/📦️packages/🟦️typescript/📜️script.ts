#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-vscode` router: `bun ./📜️script.ts <dev|test [level]|build|lint|build-vsix>`. */
import { build } from "vite";
import { extensionBuildConfig, extensionPackageEnvironment } from "../../🏗️builder/🟦️.ts";
import { BundleScript, ScriptRouter, resolveTestLevel, runBunx, runBundleScriptMain, TEST_LEVELS } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

//#region Build
/** 🧩️Builds the extension host entry and its extension-host test bundle. */
async function buildExtension(root: string, watch: boolean): Promise<void> {
  await build(extensionBuildConfig(root, "🟦️.ts", "out", "extension.js", watch));
  if (!watch) await build(extensionBuildConfig(root, "../../🧪️tests/🧩️extension/🟦️.ts", "out/test", "extension.test.js", false));
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
    runBunx(["vscode-test"], this.root);
  }
}

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await buildExtension(this.root, false);
  }
}

class LintScript extends BundleScript {
  run(): void {
    runBunx(["eslint", "--max-warnings", "0", "--config", "🟦️eslint.config.ts", "."], this.root);
  }
}

/** 📦️Packages the VSIX; nx's `dependsOn: ["build"]` runs the build target first. */
class BuildVsixScript extends BundleScript {
  run(): void {
    runBunx(["vsce", "package", "--no-dependencies", "--out", "🧩️repo.vsix"], this.root, extensionPackageEnvironment(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("test", TestScript)
  .register("build", BuildScript)
  .register("lint", LintScript)
  .register("build-vsix", BuildVsixScript);

await runBundleScriptMain(router, import.meta.url);
