#!/usr/bin/env bun
/** 🌎️ `os-hub` router: `bun ./📜️script.ts <setup|build|test|dev>`. */
import { BundleScript, ScriptRouter, OS_HUB_PORT, OS_HUB_PORT_ENV, runBundleScriptMain, runCargo, runCargoTestBudgeted, resolveTestLevel } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class SetupScript extends BundleScript {
  run(): void {
    runCargo(["fetch", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class BuildScript extends BundleScript {
  run(): void {
    runCargo(["build", "--release", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    // 🎛️ `--all-features` so a plain `bun ./📜️script.ts test` covers the full old 5-crate baseline
    // (directory core + sqlite/postgres/neo4j backends + the bin's own WS/REST suite) in one run —
    // `postgres`'s own tests still need a live Docker daemon regardless of this flag (pre-existing,
    // not a regression from the merge).
    runCargoTestBudgeted(["semio-hub"], this.repoRoot, ["--all-features", ...rest]);
  }
}

class DevScript extends BundleScript {
  run(): void {
    runCargo(["run", "--manifest-path", "Cargo.toml"], this.root, {
      [OS_HUB_PORT_ENV]: String(OS_HUB_PORT),
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("setup", SetupScript).register("build", BuildScript).register("test", TestScript).register("dev", DevScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
