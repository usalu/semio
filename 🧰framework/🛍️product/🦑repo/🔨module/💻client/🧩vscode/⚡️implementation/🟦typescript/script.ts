#!/usr/bin/env bun
/** 🧭 `@semio-tech/repo-vscode` router: `bun ./script.ts <dev|test [level]|build|lint|build-vsix>`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBunx, runBundleScriptMain, TEST_LEVELS } from "../../../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🟦typescript/🕸️graph/🗣️dsl/🫀core/🟦typescript/📦index.ts";

class DevScript extends BundleScript {
  run(): void {
    runBunx(["vite", "build", "--watch"], this.root);
  }
}

/** ⏱️The extension-host Mocha suite (`js/🟦🟦extension.test.ts`) can only run inside the VSCode test harness — no fast in-repo unit split without a second test file (disallowed). Runs only at `long` and above. */
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
  run(): void {
    runBunx(["vite", "build"], this.root);
    runBunx(["vite", "build", "--config", "🟦🟦vite.test.config.ts"], this.root);
  }
}

class LintScript extends BundleScript {
  run(): void {
    runBunx(["eslint", "--max-warnings", "0", "--config", "🟦🟦eslint.config.ts", "."], this.root);
  }
}

/** 📦Packages the VSIX; nx's `dependsOn: ["build"]` runs the build target first. */
class BuildVsixScript extends BundleScript {
  run(): void {
    runBunx(["vsce", "package", "--no-dependencies", "--out", "🧩🧩repo.vsix"], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("test", TestScript)
  .register("build", BuildScript)
  .register("lint", LintScript)
  .register("build-vsix", BuildVsixScript);

await runBundleScriptMain(router, import.meta.url);
