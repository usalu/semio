#!/usr/bin/env bun
import { spawn, spawnSync } from "node:child_process";
import { Script, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";

/** 🏃️ Runs Bun with progress and cancellation of its owned process tree. */
export async function runBun(args: string[], cwd: string, signal: AbortSignal): Promise<void> {
    signal.throwIfAborted();
    const child = spawn(process.execPath, args, { cwd, env: process.env, detached: process.platform !== "win32", stdio: "inherit", windowsHide: true });
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
    const progress = setInterval(() => console.log(`Bun ${args[0]} is still running…`), 10000);
    signal.addEventListener("abort", stop, { once: true });
    if (signal.aborted) stop();
    try {
      const status = await new Promise<number>((accept, reject) => { child.once("error", reject); child.once("close", code => accept(code ?? 1)); });
      signal.throwIfAborted();
      if (status !== 0) throw new Error(`Bun ${args[0]} failed (${status})`);
    } finally {
      if (signal.aborted) kill("SIGKILL");
      if (force) clearTimeout(force);
      clearInterval(progress);
      signal.removeEventListener("abort", stop);
    }
}

/** 📦️ Synchronizes the locked JavaScript environment without loading application modules. */
export class SyncScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Dependency synchronization accepts no installer overrides");
    const controller = new AbortController();
    let cancelled: NodeJS.Signals | undefined;
    const stop = (signal: NodeJS.Signals): void => { cancelled ??= signal; controller.abort(); };
    const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try { await runBun(["install", "--frozen-lockfile"], this.root, controller.signal); }
    catch (error) { if (!cancelled) throw error; process.exitCode = cancelled === "SIGINT" ? 130 : 143; }
    finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
  }
}

if (import.meta.main) await new ScriptRouter(getWorkspaceRoot()).register("sync", SyncScript).run(process.argv.slice(2));
