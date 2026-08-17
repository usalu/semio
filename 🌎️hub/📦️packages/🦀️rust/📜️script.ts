#!/usr/bin/env bun
/** 🌎️ `os-hub` router: `bun ./📜️script.ts <setup|build|test|dev>`. */
import { BundleScript, ScriptRouter, OS_HUB_PORT, OS_HUB_PORT_ENV, runBundleScriptMain, runCargo, runCargoTestBudgeted, runCmd, orchestratorBudgetOpts, resolveTestLevel } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

/** 🛡️ `os-hub-admin`'s build MUST land before cargo ever runs `main()` for real — `HubState.
 * admin_dir` (§C0 `OS_HUB_ADMIN_DIR`, else the compile-time default) is read at hub STARTUP, not
 * build time, so this is a runtime prerequisite, not a Cargo `build.rs` concern. Zero-touch/
 * cross-platform: `bun nx run os-hub-admin:build` is the same command every OS/devcontainer already
 * runs for every other nx target here. */
function buildAdminSpa(repoRoot: string): void {
  runCmd("bun", ["nx", "run", "os-hub-admin:build"], { cwd: repoRoot, ...orchestratorBudgetOpts() });
}

class SetupScript extends BundleScript {
  run(): void {
    runCargo(["fetch", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class BuildScript extends BundleScript {
  run(): void {
    buildAdminSpa(this.repoRoot);
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

/** 🔗️ `runCargo`'s `env` arg replaces `process.env` wholesale (see `runCmdInternal`'s
 * `opts.env ?? process.env`), so this inherits the full process env and only defaults the port —
 * otherwise the launcher's `OS_HUB_PORT`/`OS_HUB_DATA` (and `PATH`) would be silently dropped. */
class DevScript extends BundleScript {
  run(): void {
    buildAdminSpa(this.repoRoot);
    runCargo(["run", "--manifest-path", "Cargo.toml"], this.root, {
      ...process.env,
      [OS_HUB_PORT_ENV]: process.env[OS_HUB_PORT_ENV] ?? String(OS_HUB_PORT),
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("setup", SetupScript).register("build", BuildScript).register("test", TestScript).register("dev", DevScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
