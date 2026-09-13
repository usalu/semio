#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { Script, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { orchestratorBudgetMs } from "../../../../🏃️process/🟦️.ts";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { repoCacheDirectory } from "../../../🟦️.ts";
import { wasmBindgenVersion } from "../../../🦀️cargo/🟦️.ts";
import { runTool } from "../📜️script.ts";

export type DependencyToolRunner = (command: string, args: string[], cwd: string, signal: AbortSignal, capture?: boolean | "ignore", environment?: NodeJS.ProcessEnv) => Promise<string>;

/** 🛠️ Synchronizes one native environment; Nx schedules every cross-environment prerequisite. */
export async function prepareDependencies(kind: string, workspace: string, signal: AbortSignal, runner: DependencyToolRunner = runTool): Promise<void> {
  signal.throwIfAborted();
  const run = async (command: string, args: string[], capture = false, environment?: NodeJS.ProcessEnv): Promise<string> => {
    signal.throwIfAborted();
    const result = await runner(command, args, workspace, signal, capture, environment);
    signal.throwIfAborted();
    return result;
  };
  const probe = async (command: string, args: string[]): Promise<string> => {
    try { return await run(command, args, true); }
    catch (error) { signal.throwIfAborted(); return ""; }
  };
  const tool = async (crate: string, command: string[], version: string): Promise<void> => {
    if ((await probe(command[0]!, [...command.slice(1), "--version"])).split(/\s+/).includes(version)) return;
    await run("cargo", ["install", crate, "--version", version, "--locked"]);
  };
  const target = async (name: string): Promise<void> => {
    if (!(await probe("rustup", ["target", "list", "--installed"])).trim().split(/\r?\n/).includes(name))
      await run("rustup", ["target", "add", name]);
  };
  if (kind === "python") await run("uv", ["sync", "--locked", "--all-packages", "--all-groups"]);
  else if (kind === "cargo") await run("cargo", ["fetch", "--locked", "--manifest-path", "Cargo.toml"]);
  else if (kind === "go") await run("go", ["mod", "download"], false, { ...process.env, GOWORK: join(workspace, "go.work") });
  else if (kind === "dotnet") console.log("[deps-dotnet] Nx project restores completed");
  else if (kind === "cpp") console.log("[deps-cpp] Nx native tooling prerequisite completed");
  else if (kind === "browsers") await run("bun", [join(workspace, "node_modules/playwright/cli.js"), "install", "chromium"], false, { ...process.env, PLAYWRIGHT_BROWSERS_PATH: repoCacheDirectory(workspace, "tools", "ms-playwright") });
  else if (kind === "wasm" || kind === "trunk") {
    const bindgen = wasmBindgenVersion(readFileSync(join(workspace, "Cargo.lock"), "utf8"));
    if (kind === "wasm") await tool("wasm-pack", ["wasm-pack"], "0.15.0");
    else await tool("trunk", ["trunk"], "0.21.14");
    await tool("wasm-bindgen-cli", ["wasm-bindgen"], bindgen);
    await target(kind === "wasm" ? "wasm32-wasip2" : "wasm32-unknown-unknown");
    console.log(`[deps-${kind}] Pinned tools and Rust WebAssembly target are ready`);
  } else if (kind === "tools") {
    await tool("cargo-nextest", ["cargo", "nextest"], "0.9.140");
    await tool("cargo-llvm-cov", ["cargo", "llvm-cov"], "0.8.7");
  } else throw new Error(`Unknown dependency environment: ${kind}`);
}

/** 🔌️ Owns cancellation and an optional budget without importing the application router. */
export class NativeDependenciesScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length !== 1) throw new Error("Native dependency synchronization requires one environment and accepts no installer overrides");
    const controller = new AbortController(), budget = orchestratorBudgetMs();
    if (!Number.isSafeInteger(budget) || budget < 0) throw new Error("SEMIO_ORCHESTRATOR_BUDGET_MS must be a nonnegative integer");
    let cancelled: NodeJS.Signals | undefined;
    const stop = (signal: NodeJS.Signals): void => { cancelled ??= signal; controller.abort(); };
    const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    const timer = budget ? setTimeout(() => controller.abort(new Error(`Native dependency synchronization exceeded ${budget}ms`)), budget) : undefined;
    try { await prepareDependencies(args[0]!, this.root, controller.signal); }
    catch (error) { if (!cancelled) throw error; process.exitCode = cancelled === "SIGINT" ? 130 : 143; }
    finally { clearTimeout(timer); process.off("SIGINT", interrupt); process.off("SIGTERM", terminate); }
  }
}

if (import.meta.main) await new ScriptRouter(getWorkspaceRoot()).register("sync", NativeDependenciesScript).run(process.argv.slice(2));

