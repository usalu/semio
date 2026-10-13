#!/usr/bin/env bun
import { createHash, randomUUID } from "node:crypto";
import { existsSync, readFileSync, rmSync } from "node:fs";
import { availableParallelism, homedir } from "node:os";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { Script, ScriptRouter, findWorkspaceRoot, scriptInvocationBudget } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { readProcessOwnerContextV1 } from "../../../../../../🔨️modules/🏃️process/📋️context/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { acquireNativeCapacityLeaseV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { devToolingEnv } from "../../🏃️process/🌿️environment/🟦️.ts";

const runProbe = (command: string, args: string[], options: {env?: NodeJS.ProcessEnv} = {}) => spawnSync(command,args,{encoding:"utf8",...options});
const WINDOWS_CMAKE_GENERATOR = "Visual Studio 18 2026";

function queryVisualStudio2026InstallPath(): string | undefined {
 if (process.platform !== "win32") return undefined;
 const vswhere = join(process.env["ProgramFiles(x86)"] ?? "C:\\Program Files (x86)", "Microsoft Visual Studio", "Installer", "vswhere.exe");
 if (!existsSync(vswhere)) return undefined;
 const result = runProbe(vswhere, ["-latest", "-version", "[18.0,19.0)", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"]);
 return result.status === 0 ? result.stdout.trim() || undefined : undefined;
}
//#region 🔖️CppScript
export class CppScript extends Script {
  async run(segments: string[]): Promise<void> {
    const [command = "all", ...arguments_] = segments;
    if (arguments_.length) throw Error("CMake uses the host preset and explicit SEMIO_BUILD_MODE");
    if (command === "setup") { await this.runSetup(); return; }
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), host = process.platform === "win32" ? "windows" : process.platform === "darwin" ? "macos" : "linux", preset = `${host}-${mode}`;
    if (command === "configure") await this.runConfigure(preset);
    else if (command === "build") await this.runBuild(preset);
    else if (command === "test") await this.runTest(preset);
    else if (command === "all") console.log(`[cpp] Nx completed the ${mode} native graph`);
    else throw Error("CMake command must be setup, configure, build, test or all");
  }

  private async runSetup(): Promise<void> {
    await this.ensureTool("cmake", "cmake");
    if (process.platform !== "win32") await this.ensureTool("ninja", "ninja");
    if (process.platform === "win32") this.ensureWindowsMsvc();
  }

  private async withCompilerState(preset: string, action: (environment: NodeJS.ProcessEnv, signal: AbortSignal) => Promise<void>): Promise<void> {
    const compiler = process.env.CXX ?? (process.platform === "win32" ? "cl" : "c++");
    const compilerIdentity = process.platform === "win32" ? queryVisualStudio2026InstallPath() : runProbe(compiler, ["--version"]).stdout;
    const toolchain = createHash("sha256").update(JSON.stringify([runProbe(this.resolveTool("cmake"), ["--version"]).stdout, compiler, compilerIdentity, process.env.CC, process.env.CMAKE_TOOLCHAIN_FILE, process.env.CMAKE_TOOLCHAIN_FILE ? readFileSync(resolve(this.root, process.env.CMAKE_TOOLCHAIN_FILE)) : null])).digest("hex").slice(0, 12);
    const directory = join(readProcessOwnerContextV1(process.env,this.root).cacheRoot, "cmake", preset, `${process.arch}-${toolchain}`);
    const leases = readProcessOwnerContextV1(process.env,this.root).leaseDirectory;
    const controller = new AbortController(), cancel = () => controller.abort();
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    let capacity: Awaited<ReturnType<typeof acquireNativeCapacityLeaseV1>> | undefined;
    try {
      capacity = await acquireNativeCapacityLeaseV1({ directory: leases, signal: controller.signal, onWait: ({ elapsedMs }) => console.log(`[cpp] Waiting for native capacity: ${elapsedMs}ms`) });
      const lease = await acquireQueuedResourceLease({directory: leases, resource: `cmake:${directory}`, mode: "exclusive", owner: randomUUID(), signal: controller.signal});
      try {
        this.purgeStaleCmakeCache(preset, directory);
        await action({...this.cppEnv(), SEMIO_CMAKE_BINARY_DIR: directory, SEMIO_NATIVE_CAPACITY_HELD: "true"}, controller.signal);
      } finally { lease.release(); }
    } finally { capacity?.release(); process.off("SIGINT", cancel); process.off("SIGTERM", cancel); }
  }

  private async runConfigure(preset: string): Promise<void> {
    await this.withCompilerState(preset, (env, signal) => runOwnedCommand(this.resolveTool("cmake"), ["--preset", preset], this.root, "cpp:configure", scriptInvocationBudget(this.invocation,buildBudgetMs()), { env, signal }));
  }

  private async runBuild(preset: string): Promise<void> {
    await this.withCompilerState(preset, async (env, signal) => {
      await runOwnedCommand(this.resolveTool("cmake"), ["--preset", preset], this.root, "cpp:configure", scriptInvocationBudget(this.invocation,buildBudgetMs()), { env, signal });
      await runOwnedCommand(this.resolveTool("cmake"), ["--build", "--preset", preset], this.root, "cpp:build", scriptInvocationBudget(this.invocation,buildBudgetMs()), { env, signal });
    });
  }

  private async runTest(preset: string): Promise<void> {
    await this.withCompilerState(preset, (env, signal) => runOwnedCommand(this.resolveTool("ctest"), ["--preset", preset], this.root, "cpp:test", scriptInvocationBudget(this.invocation,buildBudgetMs()), { env, signal }));
  }

  private async ensureTool(command: string, uvTool: string): Promise<void> {
    if (this.hasTool(command)) return;
    if (this.hasTool("uv")) {
      await runOwnedCommand(this.resolveTool("uv"), ["tool", "install", "--upgrade", uvTool], this.root, "cpp:tool", scriptInvocationBudget(this.invocation,buildBudgetMs()), { signal:this.invocation.control.signal });
      if (this.hasTool(command)) return;
    }
    console.error(`[cpp] ${command} is required. Run the native bootstrap or rebuild the devcontainer.`);
    process.exit(1);
  }

  private hasTool(command: string): boolean {
    try {
      return runProbe(this.resolveTool(command), ["--version"], { env: this.cppEnv() }).status === 0;
    } catch {
      return false;
    }
  }

  private resolveTool(command: string): string {
    const extension = process.platform === "win32" ? ".exe" : "";
    const candidates = [command, join(homedir(), ".local", "bin", `${command}${extension}`), join(homedir(), ".local", "bin", command)];
    return candidates.find((candidate) => existsSync(candidate)) ?? command;
  }

  private cppEnv(): NodeJS.ProcessEnv {
    return {
      ...devToolingEnv(),
      CMAKE_BUILD_PARALLEL_LEVEL: String(availableParallelism()),
      CTEST_PARALLEL_LEVEL: String(availableParallelism()),
    };
  }

  private ensureWindowsMsvc(): void {
    if (process.platform !== "win32") return;
    if (queryVisualStudio2026InstallPath()) return;
    console.error("[cpp] Visual Studio 2026 with the Desktop development with C++ workload is required.");
    console.error("[cpp] On native Windows run: bun ./📜️script.ts setup native");
    process.exit(1);
  }

  private purgeStaleCmakeCache(preset: string, cacheDir: string): void {
    const cacheFile = join(cacheDir, "CMakeCache.txt");
    if (!existsSync(cacheFile)) return;
    const content = readFileSync(cacheFile, "utf8");
    const generatorMatch = content.match(/^CMAKE_GENERATOR:INTERNAL=(.+)$/m);
    const cachedGenerator = generatorMatch?.[1]?.trim();
    if (process.platform === "win32" && cachedGenerator && cachedGenerator !== WINDOWS_CMAKE_GENERATOR) {
      console.log(`[cpp] Removing stale CMake cache for preset "${preset}" (generator ${cachedGenerator}).`);
      rmSync(cacheDir, { recursive: true, force: true });
    }
  }
}
//#endregion 🔖️CppScript

if (import.meta.main) { const root = findWorkspaceRoot(import.meta.dir); await receiveScriptProcessInvocation(process.env,original=>runScriptMain(new ScriptRouter(root,root).register("cpp",CppScript),{invocation:original})); }
