#!/usr/bin/env bun
/**
 * 🛂️ `@semio-tech/os-plugin-describe-rs` task router: `bun ./📜️script.ts <build|test|describe>`.
 * `describe <component.wasm> --out <dir>` builds (if needed) and execs the
 * `semio-framework-plugin-describe` binary — the build-time-only descriptor emitter
 * (`📓️design-abi.md` §3). Called from the dev `📜️script.ts` right after the `wasm32-wasip2` build,
 * and from each plugin crate's own `📜️script.ts describe` (see that script's own doc for the exact
 * invocation convention every migrated plugin crate follows).
 */
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter, devToolingEnv, runBundleScriptMain, runCargoTestBudgeted, runCmd, runCmdStatus, resolveTestLevel } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

const CRATE_NAME = "semio-framework-plugin-describe";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", CRATE_NAME, "--release"], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([CRATE_NAME], this.repoRoot, rest);
  }
}

/** @emoji 🎯️ Resolves cargo's real target dir, honouring a `CARGO_TARGET_DIR` override (ticket-scoped
 * builds always set one — `important.md` binding rule 4) instead of assuming the repo-root `target/`. */
function cargoTargetRoot(repoRoot: string): string {
  return process.env.CARGO_TARGET_DIR ? resolve(repoRoot, process.env.CARGO_TARGET_DIR) : join(repoRoot, "target");
}

/** @emoji 🛠️ Resolves the debug-profile binary path for the current platform, after ensuring it is built (cargo's incremental cache makes a no-op rebuild fast — never exec a possibly-stale binary). */
function ensureBuiltBin(repoRoot: string): string {
  runCmd("cargo", ["build", "-p", CRATE_NAME], { cwd: repoRoot, env: devToolingEnv() });
  const binName = process.platform === "win32" ? `${CRATE_NAME}.exe` : CRATE_NAME;
  return join(cargoTargetRoot(repoRoot), "debug", binName);
}

/** @emoji 🛂️ `describe <component.wasm> --out <dir>` — builds then execs the emitter with forwarded argv and inherited stdio. */
class DescribeScript extends BundleScript {
  run(segments: string[]): void {
    const bin = ensureBuiltBin(this.repoRoot);
    const status = runCmdStatus(bin, ["describe", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

/** @emoji 🎯️ Debug-profile `wasm32-wasip2` artifact path cargo just built for `packageName`, honouring
 * the same `CARGO_TARGET_DIR` override as {@link ensureBuiltBin}. */
export function pluginWasmArtifactPath(repoRoot: string, packageName: string): string {
  return join(cargoTargetRoot(repoRoot), "wasm32-wasip2", "debug", `${packageName.replace(/-/g, "_")}.wasm`);
}

/** @emoji 🛂️ Shared implementation for a plugin/extension crate's own `📜️script.ts describe` command
 * (D0-descriptor-plumbing, `📌️important.md`): builds `packageName`'s `wasm32-wasip2` component — no
 * extra `--features component-guest` flag needed, every plugin crate's own `Cargo.toml` already
 * enables it unconditionally on its `semio-framework-plugin` dependency, confirmed empirically (no
 * plugin crate exposes a feature literally named `component-guest` of its own; passing that flag to
 * `cargo build -p <plugin>` fails with "does not contain this feature") — then runs the real emitter
 * (`describe_component`, `📇️describe/📦️packages/🦀️rust/📦️glue.rs`) against the built wasm, writing
 * `🛂️descriptor.semio` + `🔣️descriptor.json` straight into `ownerRoot` (the plugin/extension owner
 * root, sibling of the tracked `🛂️manifest.json` — NOT `🤖️generated/`, which is gitignored). One
 * shared function so every migrated plugin crate's own `describe` command stays a thin two-line
 * wrapper around it rather than duplicating the build+emit sequence 33 times. */
export function describePluginComponent(repoRoot: string, packageName: string, ownerRoot: string): number {
  const buildStatus = runCmdStatus("cargo", ["build", "-p", packageName, "--target", "wasm32-wasip2"], { cwd: repoRoot, env: devToolingEnv() });
  if (buildStatus !== 0) return buildStatus;
  const bin = ensureBuiltBin(repoRoot);
  return runCmdStatus(bin, ["describe", pluginWasmArtifactPath(repoRoot, packageName), "--out", ownerRoot], { cwd: repoRoot, env: devToolingEnv() });
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("describe", DescribeScript);
  await runBundleScriptMain(router, import.meta.url);
}
