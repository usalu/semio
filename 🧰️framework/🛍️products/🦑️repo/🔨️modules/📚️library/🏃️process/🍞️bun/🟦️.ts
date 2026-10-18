import { dirname, join } from "node:path";
import { createRequire } from "node:module";
import { spawn, spawnSync } from "node:child_process";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveWorkspaceBin, runCmd } from "../🟦️.ts";
import { playPollingEnv } from "../🌿️environment/🟦️.ts";

//#region ⚙️ViteConfigLoader
/**
 * ⚙️ Pick Vite's config loader for the current runtime.
 * Node 24+ defaults Vite to `native` strip-only TypeScript, which rejects constructor parameter
 * properties used across monorepo configs — so Node must use `bundle`. Bun already runs full
 * TypeScript, and Vite's `bundle` loader fails on this config with an opaque `undefined` error,
 * so Bun keeps `native`. Callers that already pass `--configLoader` win.
 * @see https://vite.dev/config/#config-loader
 */
export function viteConfigLoader(): "native" | "bundle" {
  return typeof (globalThis as { Bun?: unknown }).Bun === "object" ? "native" : "bundle";
}

/** 🧷️ Prepends {@link viteConfigLoader}'s choice to a Vite CLI argument list. */
export function withViteConfigLoader(args: readonly string[]): string[] {
  if (args.includes("--configLoader")) return [...args];
  const loader = viteConfigLoader();
  const out = [...args];
  const viteIdx = out.indexOf("vite");
  if (viteIdx >= 0) {
    out.splice(viteIdx + 1, 0, "--configLoader", loader);
    return out;
  }
  return ["--configLoader", loader, ...out];
}

function bunArgsForVite(args: readonly string[]): string[] {
  return args.includes("vite") ? withViteConfigLoader(args) : [...args];
}

//#endregion ⚙️ViteConfigLoader

/** 🧭️ The workspace's own Vite CLI entry (`vite/bin/vite.js`), resolved through the package instead of `node_modules/.bin`,
 * whose extensionless `vite` is a POSIX shell shim on Windows whenever another package manager wrote it. */
function viteCliEntry(cwd: string): string {
  return join(dirname(createRequire(join(findWorkspaceRoot(cwd), "package.json")).resolve("vite/package.json")), "bin/vite.js");
}

function bunxCmdArgs(args: readonly string[], cwd: string): string[] {
  const formatted = bunArgsForVite(args);
  const viteIdx = formatted.indexOf("vite");
  if (viteIdx >= 0) {
    const copy = [...formatted];
    copy[viteIdx] = viteCliEntry(cwd);
    return copy;
  }
  const binName = formatted[0];
  if (binName && binName !== "x") {
    const resolved = resolveWorkspaceBin(binName, cwd);
    if (resolved) {
      return [resolved, ...formatted.slice(1)];
    }
  }
  return ["x", ...formatted];
}

/** 🥖️Runs `bun` with inherited stdio in `cwd`. */
export function runBun(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  runCmd(process.execPath, bunArgsForVite(args), { cwd, env });
}

/** 🥖️Runs `bunx` synchronously in `cwd`, returning status code. */
export function runBunxStatus(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): number {
  const result = spawnSync(process.execPath, bunxCmdArgs(args, cwd), { cwd, env, shell: false, stdio: "inherit" });
  if (result.error) {
    console.error(result.error);
    return 1;
  }
  return result.status ?? 1;
}

/** 🥖️Runs `bunx` synchronously in `cwd`. */
export function runBunx(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  const status = runBunxStatus(args, cwd, env);
  if (status !== 0) process.exit(status);
}

/** 🥖️Spawns `bunx` asynchronously; exits with child code. */
export function spawnBunx(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): Promise<void> {
  return new Promise((resolve, reject) => {
    const child = spawn(process.execPath, bunxCmdArgs(args, cwd), { cwd, env, shell: false, stdio: "inherit" });
    child.on("exit", (code) => {
      if ((code ?? 0) === 0) resolve();
      else process.exit(code ?? 0);
    });
    child.on("error", (error) => {
      console.error(error);
      reject(error);
      process.exit(1);
    });
  });
}

/** 🥖️Spawns `bun` asynchronously; exits with child code. */
export function spawnBun(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  const child = spawn(process.execPath, bunArgsForVite(args), { cwd, env, shell: false, stdio: "inherit", windowsHide: true });
  child.on("exit", (code) => process.exit(code ?? 0));
  child.on("error", (error) => {
    console.error(error);
    process.exit(1);
  });
}

/** ▶️Vite dev via `bunx` without a fixed config path (extra args only). */
export function runViteBunxDevPlain(bundleRoot: string, segments: string[]): void {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  spawnBunx(["vite", "--host", host, ...segments], bundleRoot, playPollingEnv());
}
