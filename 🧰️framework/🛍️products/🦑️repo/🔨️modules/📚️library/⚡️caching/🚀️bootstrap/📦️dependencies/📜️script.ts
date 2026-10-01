#!/usr/bin/env bun
import { spawn, spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { Script, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { prepareCargoWorkspaceInvocation } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { discoverBunWorkspaces, bunWorkspaceNativePatterns } from "../../../🗂️workspaces/🟦️bun/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";

/** 🏃️ Runs one tool with progress, bounded optional output and cancellation of its process tree. */
export async function runTool(command: string, args: string[], cwd: string, signal: AbortSignal, capture: boolean | "ignore" = false, environment: NodeJS.ProcessEnv = process.env): Promise<string> {
    signal.throwIfAborted();
    if (command === "cargo") prepareCargoWorkspaceInvocation(getWorkspaceRoot(), args, cwd);
    const child = spawn(command, args, { cwd, env: environment, detached: process.platform !== "win32", stdio: ["ignore", capture === "ignore" ? "ignore" : capture ? "pipe" : "inherit", "inherit"], windowsHide: true });
    let output = "", overflow = false;
    let force: ReturnType<typeof setTimeout> | undefined;
    const kill = (signal: NodeJS.Signals): void => {
      if (!child.pid) return;
      if (process.platform === "win32") spawnSync("taskkill", ["/pid", String(child.pid), "/t", "/f"], { stdio: "ignore", windowsHide: true });
      else try { process.kill(-child.pid, signal); } catch {}
    };
    const stop = (): void => {
      kill("SIGTERM");
      force = setTimeout(() => kill("SIGKILL"), 5000);
    };
    child.stdout?.on("data", (chunk: Buffer) => { if (output.length + chunk.length > 1024 * 1024) { overflow = true; kill("SIGKILL"); } else output += chunk.toString(); });
    const progress = setInterval(() => (capture ? console.error : console.log)(`${command} ${args[0]} is still running…`), 10000);
    signal.addEventListener("abort", stop, { once: true });
    if (signal.aborted) stop();
    try {
      const status = await new Promise<number>((accept, reject) => { child.once("error", reject); child.once("close", code => accept(code ?? 1)); });
      signal.throwIfAborted();
      if (overflow) throw new Error(`${command} output exceeds its limit`);
      if (status !== 0) throw new Error(`${command} ${args[0]} failed (${status})`);
      return output;
    } finally {
      if (signal.aborted) kill("SIGKILL");
      if (force) clearTimeout(force);
      clearInterval(progress);
      signal.removeEventListener("abort", stop);
    }
}

/** 📦️ Runs the selected Bun runtime through the shared process owner. */
export async function runBun(args: string[], cwd: string, signal: AbortSignal): Promise<void> {
  await runTool(process.execPath, args, cwd, signal);
}

export type JavascriptDependencyRunner = (args: string[], cwd: string, signal: AbortSignal) => Promise<void>;

/** 🧾️ Selects the authored JavaScript dependency operation without caller installer overrides. */
export async function prepareJavascriptDependencies(mode: "sync" | "lock", workspace: string, signal: AbortSignal, runner: JavascriptDependencyRunner = runBun): Promise<void> {
  signal.throwIfAborted();
  await runner(["install", mode === "sync" ? "--frozen-lockfile" : "--lockfile-only"], workspace, signal);
  signal.throwIfAborted();
}

/** 📦️ Synchronizes the locked JavaScript environment without loading application modules. */
export class SyncScript extends Script {
  protected readonly mode: "sync" | "lock" = "sync";
  async run(args: string[]): Promise<void> {
    if (args.length && !(args.length===1 && args[0]==="--all") && !(args.length===2 && args[0]==="--scope")) throw new Error("Dependency synchronization accepts --all or --scope <owner>");
    const scopes=discoverBunWorkspaces(this.root), requested=args[0]==="--all" || !args.length && this.mode==="lock" ? scopes : scopes.filter(scope=>scope.directory===(args[0]==="--scope"?args[1]:"."));
    if(!requested.length)throw new Error("Unknown present Bun installation scope");
    const controller = new AbortController();
    let cancelled: NodeJS.Signals | undefined;
    const stop = (signal: NodeJS.Signals): void => { cancelled ??= signal; controller.abort(); };
    const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try { for(const scope of requested) { const path=join(this.root,scope.manifest), before=readFileSync(path,"utf8"), document=JSON.parse(before), patterns=bunWorkspaceNativePatterns(this.root,scope); if(JSON.stringify(document.workspaces)!==JSON.stringify(patterns)){if(readFileSync(path,"utf8")!==before)throw new Error("Bun workspace changed during source discovery");writeFileSync(path,JSON.stringify({...document,workspaces:patterns},null,2)+"\n");} console.log(`[bun-workspace] ${scope.manifest} ${this.mode}`); await prepareJavascriptDependencies(this.mode, join(this.root,scope.directory), controller.signal); } }
    catch (error) { if (!cancelled) throw error; process.exitCode = cancelled === "SIGINT" ? 130 : 143; }
    finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
  }
}

/** 🔒️ Refreshes only the declared workspace lock through the selected Bun runtime. */
export class RefreshLockScript extends SyncScript {
  protected readonly mode = "lock" as const;
}

/** ⚖️ Executes portable dependency command and independent Node oracle laws. */
export class ContractCheckScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Dependency contract check accepts no arguments");
    await runBun(["test", join(import.meta.dir, "🧪️tests/🟦️.ts")], this.root, new AbortController().signal);
  }
}

if (import.meta.main) await new ScriptRouter(getWorkspaceRoot()).register("sync", SyncScript).register("lock", RefreshLockScript).register("contract-check", ContractCheckScript).run(process.argv.slice(2));
