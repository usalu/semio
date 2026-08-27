#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-vscode` router: `bun ./📜️script.ts <dev|test [level]|build|lint|build-vsix>`. */
import { builtinModules } from "node:module";
import { resolve } from "node:path";
import { build, type InlineConfig } from "vite";
import { BundleScript, ScriptRouter, resolveTestLevel, runBunx, runBundleScriptMain, TEST_LEVELS } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

//#region Build
const extensionExternals = new Set(["vscode", ...builtinModules, ...builtinModules.map((name) => `node:${name}`)]);

/** 📦️Defines a dependency-bundled CommonJS entry while retaining VS Code and Node host modules. */
function extensionBuildConfig(root: string, entry: string, outputDirectory: string, outputFile: string, watch: boolean): InlineConfig {
  return {
    configFile: false,
    root,
    build: {
      emptyOutDir: true,
      lib: { entry: resolve(root, entry), formats: ["cjs"], fileName: () => outputFile },
      minify: false,
      outDir: resolve(root, outputDirectory),
      rollupOptions: { external: (id) => extensionExternals.has(id) },
      sourcemap: false,
      target: "node22",
      watch: watch ? {} : undefined,
    },
  };
}

/** 🧩️Builds the extension host entry and its extension-host test bundle. */
async function buildExtension(root: string, watch: boolean): Promise<void> {
  await build(extensionBuildConfig(root, "🟦️extension.ts", "out", "extension.js", watch));
  if (!watch) await build(extensionBuildConfig(root, "🧪️extension.test.ts", "out/test", "extension.test.js", false));
}
//#endregion

class DevScript extends BundleScript {
  async run(): Promise<void> {
    await buildExtension(this.root, true);
  }
}

/** ⏱️The extension-host Mocha suite (`js/🧪️extension.test.ts`) can only run inside the VSCode test harness — no fast in-repo unit split without a second test file (disallowed). Runs only at `long` and above. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level } = resolveTestLevel(segments);
    if (TEST_LEVELS.indexOf(level) < TEST_LEVELS.indexOf("long")) {
      console.log(`[test] @semio-tech/repo-vscode has no ${level}-level suite — run at "long" or above for the extension-host suite.`);
      return;
    }
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
    runBunx(["vsce", "package", "--no-dependencies", "--out", "🧩️repo.vsix"], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("test", TestScript)
  .register("build", BuildScript)
  .register("lint", LintScript)
  .register("build-vsix", BuildVsixScript);

await runBundleScriptMain(router, import.meta.url);
