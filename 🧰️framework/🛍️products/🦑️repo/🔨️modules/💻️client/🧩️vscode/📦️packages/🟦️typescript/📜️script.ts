#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-vscode` router: `bun ./📜️script.ts <dev|test [level]|build|lint|build-vsix>`. */
import { build } from "vite";
import { extensionBuildConfig, extensionPackageEnvironment } from "../../🏗️builder/🟦️.ts";
import { TEST_LEVELS } from "../../../../../../../\uD83D\uDD28\uFE0Fmodules/\uD83C\uDFC3\uFE0Fprocess/\uD83E\uDDEA\uFE0Ftesting/\uD83C\uDF9A\uFE0Fbudget/\uD83D\uDFE6\uFE0F.ts";
import { runBunx } from "../../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

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
    runBunx(["vscode-test", "--config", "../../🧪️tests/🎚️config/🟨️.mjs"], this.root);
  }
}

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await buildExtension(this.root, false);
  }
}

class LintScript extends BundleScript {
  run(): void {
    runBunx(["eslint", "--max-warnings", "0", "--config", "../../../../🧹️lint/📐️source-policy/🟨️.mjs", "."], this.root);
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

await runScriptMain(router);
