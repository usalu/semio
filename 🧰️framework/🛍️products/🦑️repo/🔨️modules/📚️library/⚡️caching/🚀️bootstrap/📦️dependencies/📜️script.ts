#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {parseCargoPreparationStorageV1,type CargoPreparationStorageV1} from "../../../🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🟦️.ts";
import { spawn, spawnSync } from "node:child_process";
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { Script, ScriptRouter, checkScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {repositoryCargoPreparationStorageV1, prepareCargoWorkspaceInvocation } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { publishBunWorkspaceOwnership } from "../../../🗂️workspaces/🟦️bun/🟦️.ts";
import {withPreparedCargoDependencyPairV1} from "../../../🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts";
import {fileURLToPath} from "node:url";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";

/** 🏃️ Runs one tool with progress, bounded optional output and cancellation of its process tree. */
export async function runTool(command: string, args: string[], cwd: string, signal: AbortSignal, capture: boolean | "ignore", environment: NodeJS.ProcessEnv, storage:CargoPreparationStorageV1): Promise<string> {
    signal.throwIfAborted();parseCargoPreparationStorageV1(storage);
    if (command === "cargo") prepareCargoWorkspaceInvocation(storage,getWorkspaceRoot(environment),args,cwd,environment);
    return executeTool(command,args,cwd,signal,capture,environment);
}

async function executeTool(command:string,args:string[],cwd:string,signal:AbortSignal,capture:boolean|"ignore",environment:NodeJS.ProcessEnv):Promise<string> {
    signal.throwIfAborted();
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
      if (status !== 0) throw new Error(`${command} ${args[0]} failed (${status})${output ? "\n" + output : ""}`);
      return output;
    } finally {
      if (signal.aborted) kill("SIGKILL");
      if (force) clearTimeout(force);
      clearInterval(progress);
      signal.removeEventListener("abort", stop);
    }
}

/** 🦀️ Executes only the owned prepared dependency pair without a public preparation bypass. */
export async function runPreparedCargoDependencyPairV1(root:string,manifest:string,signal:AbortSignal,storage:CargoPreparationStorageV1,runner?:(command:string,args:string[],cwd:string,signal:AbortSignal,capture:boolean|"ignore",environment:NodeJS.ProcessEnv,storage:CargoPreparationStorageV1)=>Promise<string>):Promise<void> {
 await withPreparedCargoDependencyPairV1(storage,root,manifest,signal,async(args,cwd,signal)=>{if(runner)await runner("cargo",args,cwd,signal,false,process.env,storage);else await executeTool("cargo",args,cwd,signal,false,process.env);},[fileURLToPath(import.meta.url),fileURLToPath(new URL("./🏗️native/📜️script.ts",import.meta.url))]);
}

/** 📦️ Runs the selected Bun runtime through the shared process owner. */
export async function runBun(args: string[], cwd: string, signal: AbortSignal): Promise<void> {
  await runTool(process.execPath,args,cwd,signal,false,process.env,repositoryCargoPreparationStorageV1(process.cwd()));
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
    checkScriptInvocation(this.invocation);
    const report = await publishBunWorkspaceOwnership(this.root, {
      signal: this.invocation.control.signal,
      advance: async progress => {
        checkScriptInvocation(this.invocation);
        if (progress.completedOperations % 128 === 0) {
          await this.invocation.control.publish({ owner: this.invocation.policy.owner, command: "Bun ownership " + progress.operation + " " + progress.completedOperations, stage: "running" });
          await this.invocation.control.yieldContinuation();
        }
        checkScriptInvocation(this.invocation);
      },
    });
    const scopes = report.scopes.map(scope => ({ ...scope, directory: dirname(scope.manifest).replaceAll("\\", "/") })), requested = args[0] === "--all" || !args.length && this.mode === "lock" ? scopes : scopes.filter(scope => scope.directory === (args[0] === "--scope" ? args[1] : "."));
    if (!requested.length) throw new Error("Unknown present Bun installation scope");
    for (const scope of requested) {
      checkScriptInvocation(this.invocation);
      console.log("[bun-workspace] " + scope.manifest + " " + this.mode);
      await prepareJavascriptDependencies(this.mode, join(this.root, scope.directory), this.invocation.control.signal);
      checkScriptInvocation(this.invocation);
    }
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
    checkScriptInvocation(this.invocation);
    const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(this.root, ".🧬semio/🦑️repo/⚡️cache/tests/bootstrap-dependencies");
    mkdirSync(artifacts, { recursive: true });
    await runTool(process.execPath,["test", join(import.meta.dir, "🧪️tests/🟦️.ts")],this.root,this.invocation.control.signal,false,{ ...process.env, SEMIO_TEST_ARTIFACT_DIR: artifacts },repositoryCargoPreparationStorageV1(process.cwd()));
    checkScriptInvocation(this.invocation);
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(getWorkspaceRoot()).register("sync", SyncScript).register("lock", RefreshLockScript).register("contract-check", ContractCheckScript)).run(process.argv.slice(2), original));

export {repositoryCargoPreparationStorageV1} from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
