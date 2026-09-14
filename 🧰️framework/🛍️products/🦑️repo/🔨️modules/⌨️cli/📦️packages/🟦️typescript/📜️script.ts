#!/usr/bin/env bun
/** 🧭️ Repo CLI task router — builds and runs the CLI binary of the selected implementation. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, buildRepoCliBin, devToolingEnv, resolveCliBin, resolveRepoImplementation, runBundleScriptMain, runCmd, runTestBudgeted, REPO_GO_CLI_DIR, REPO_RUST_CLI_CRATE } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** ▶️Builds the selected implementation's binary — cargo/go incremental caches keep a no-op rebuild fast — then forwards argv. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    const bin = buildRepoCliBin(this.repoRoot);
    runCmd(bin, [...segments], {
      cwd: this.repoRoot,
      env: { ...devToolingEnv(), GOWORK: join(this.repoRoot, "go.work") },
    });
  }
}

/** 📦️Produces the release binary of the selected implementation at [[resolveCliBin]]. */
class BuildScript extends BundleScript {
  run(): void {
    if (resolveRepoImplementation() === "go") {
      runCmd("go", ["build", "-trimpath", "-ldflags=-s -w", "-o", resolveCliBin(this.repoRoot), `./${REPO_GO_CLI_DIR}/🚀️bin`], {
        cwd: this.repoRoot,
        env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
      });
      return;
    }
    buildRepoCliBin(this.repoRoot);
  }
}

/** ⏱️Delegates to the selected implementation's own package suite (`-short` keeps the Go default ≤30s). */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    if (resolveRepoImplementation() === "go") {
      runTestBudgeted("go", ["test", `./${REPO_GO_CLI_DIR}`, "-short", ...segments], {
        cwd: this.repoRoot,
        env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
      });
      return;
    }
    runTestBudgeted("cargo", ["test", "-p", REPO_RUST_CLI_CRATE, ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
