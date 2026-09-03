#!/usr/bin/env bun
/**
 * 🛂️ `@semio-tech/os-plugin-describe-rs` task router: `bun ./📜️script.ts <build|test|describe>`.
 * `describe <component.wasm> --core <core.wasm> --out <dir>` builds (if needed) and execs the
 * `semio-framework-plugin-describe` binary — the build-time-only descriptor emitter
 * (`📓️design-abi.md` §3). Called from the dev `📜️script.ts` right after the `wasm32-wasip2` build,
 * and from each plugin crate's own `📜️script.ts describe` (see that script's own doc for the exact
 * invocation convention every migrated plugin crate follows).
 */
import { existsSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, devToolingEnv, resolveWorkspaceBin, runBundleScriptMain, runCargoTestBudgeted, runCmd, runCmdStatus, resolveTestLevel } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

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
function ensureBuiltBin(repoRoot: string, budgetMs = buildBudgetMs()): string {
  runCmd("cargo", ["build", "-p", CRATE_NAME], { cwd: repoRoot, env: devToolingEnv(), budgetMs });
  const binName = process.platform === "win32" ? `${CRATE_NAME}.exe` : CRATE_NAME;
  return join(cargoTargetRoot(repoRoot), "debug", binName);
}

/** @emoji 🛂️ `describe <component.wasm> --core <core.wasm> --out <dir>` — builds then execs the emitter with forwarded argv and inherited stdio. */
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

/** @emoji 🧩 Builds one exact plugin component and returns cargo's fresh output path. */
export function buildPluginComponent(repoRoot: string, packageName: string, rootCdylib = false, budgetMs = buildBudgetMs()): string {
  const buildArgs = rootCdylib
    ? ["rustc", "-p", packageName, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2"]
    : ["build", "-p", packageName, "--target", "wasm32-wasip2"];
  runCmd("cargo", buildArgs, { cwd: repoRoot, env: devToolingEnv(), budgetMs });
  const component = pluginWasmArtifactPath(repoRoot, packageName);
  if (!existsSync(component)) throw new Error(`cargo did not produce ${component}`);
  return component;
}

/** @emoji 🧬 Extracts the first core module from the exact component with jco's independent parser. */
export function extractPluginCore(repoRoot: string, component: string, outDir: string, baseName: string, budgetMs = buildBudgetMs()): string {
  const jco = resolveWorkspaceBin("@bytecodealliance/jco", repoRoot);
  if (!jco) throw new Error("missing @bytecodealliance/jco workspace binary; run bun install");
  runCmd("node", [jco, "transpile", component, "-o", outDir, "--name", baseName, "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], {
    cwd: repoRoot,
    env: devToolingEnv(),
    budgetMs,
  });
  const core = join(outDir, `${baseName}.core.wasm`);
  if (!existsSync(core)) throw new Error(`jco did not extract ${core}`);
  return core;
}

/** @emoji 🛂️ Emits one canonical descriptor from independently supplied raw/core artifacts. */
export function emitPluginDescriptor(repoRoot: string, component: string, core: string, outDir: string, budgetMs = buildBudgetMs()): number {
  const bin = ensureBuiltBin(repoRoot, budgetMs);
  return runCmdStatus(bin, ["describe", component, "--core", core, "--out", outDir], { cwd: repoRoot, env: devToolingEnv(), budgetMs });
}

/** @emoji 🛂️ Shared implementation for a plugin/extension crate's own `📜️script.ts describe` command
 * (D0-descriptor-plumbing, `📌️important.md`): builds `packageName`'s `wasm32-wasip2` component — no
 * extra `--features component-guest` flag needed, every plugin crate's own `Cargo.toml` already
 * enables it unconditionally on its `semio-framework-plugin` dependency, confirmed empirically (no
 * plugin crate exposes a feature literally named `component-guest` of its own; passing that flag to
 * `cargo build -p <plugin>` fails with "does not contain this feature") — then runs the real emitter
 * (`describe_component`, `📇️describe/📦️packages/🦀️rust/🦀️.rs`) against the built wasm, writing
 * `🛂️.descriptor.semio` + `🔣️.json` straight into `ownerRoot` (the plugin/extension owner
 * root, sibling of the tracked `🛂️manifest.json` — NOT `🤖️generated/`, which is gitignored). One
 * shared function so every migrated plugin crate's own `describe` command stays a thin two-line
 * wrapper around it rather than duplicating the build+emit sequence 33 times. */
export function describePluginComponent(repoRoot: string, packageName: string, ownerRoot: string, rootCdylib = false): number {
  const component = buildPluginComponent(repoRoot, packageName, rootCdylib);
  const scratch = mkdtempSync(join(tmpdir(), "semio-plugin-core-"));
  try {
    const core = extractPluginCore(repoRoot, component, scratch, packageName.replace(/-/g, "_"));
    return emitPluginDescriptor(repoRoot, component, core, ownerRoot);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("describe", DescribeScript);
  await runBundleScriptMain(router, import.meta.url);
}
