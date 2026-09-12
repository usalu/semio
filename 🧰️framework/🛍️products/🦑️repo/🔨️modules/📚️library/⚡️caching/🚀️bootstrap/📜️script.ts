#!/usr/bin/env bun
import { spawn as spawnNxProcess, spawnSync as stopNxProcessTree } from "node:child_process";
import { existsSync, readFileSync, rmSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { Script, ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { orchestratorBudgetOpts, semioShipEnv } from "../../🏃️process/🟦️.ts";
import { devToolingEnv } from "../../🏃️process/🌿️environment/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

const WORKSPACE_ROOT = getWorkspaceRoot();

/** 🧭️ Loads domain selection helpers only for commands that request them. */
function nxRoutingServices(): typeof import("../../📦️packages/🟦️typescript/🟦️.ts") {
  return createRequire(import.meta.url)("../../📦️packages/🟦️typescript/🟦️.ts");
}

/** 🛠️ Loads acquisition only when the selected Nx installation needs it. */
function nxBootstrapServices(): typeof import("./🛠️tools/📜️script.ts") {
  return createRequire(import.meta.url)("./🛠️tools/📜️script.ts");
}

/** 🧮️ Lets each watched build finish its graph while the daemon retains source watching. */
export function nxChildEnvironment(environment: NodeJS.ProcessEnv, args: readonly string[], watching: boolean): NodeJS.ProcessEnv {
  if (args[0] === "watch") return { ...environment, NX_DAEMON: "true" };
  return watching || environment.NX_FILE_CHANGES !== undefined ? { ...environment, NX_DAEMON: "false" } : environment;
}

/** ⏳️ Awaits Nx's readiness handshake while cold graph discovery reports progress and remains cancellable. */
export function waitForNxWatcher(watcher: ReturnType<typeof spawnNxProcess>): Promise<void> {
  return new Promise((accept, reject) => {
    let pending = "";
    const started = Date.now(), progress = setInterval(() => console.log(`[nx] Waiting for source watcher and project graph (${Math.floor((Date.now() - started) / 1000)}s); Ctrl+C cancels`), 15000);
    const finish = (error?: Error): void => {
      clearInterval(progress);
      watcher.removeListener("close", exited);
      watcher.removeListener("error", failed);
      watcher.stdout!.removeListener("data", received);
      if (error) reject(error); else accept();
    };
    const exited = (code: number | null, signal: string | null): void => finish(new Error(`Nx source watcher exited before readiness (${signal ?? code ?? "unknown status"})`));
    const failed = (error: Error): void => finish(error);
    const received = (chunk: Buffer): void => {
      pending = (pending + chunk.toString()).slice(-8192);
      if (pending.includes("watch process waiting...")) finish();
    };
    watcher.once("close", exited);
    watcher.once("error", failed);
    watcher.stdout!.on("data", received);
  });
}

//#region 🔖️NxScript
export class NxScript extends Script {
  /** 🧿️ `nx watch` refuses to run without the daemon, and Nx disables its daemon for every later
   * client by writing `<workspace-data>/d/disabled` whenever one start fails — a marker that outlives
   * the crash it records (seen: a 2026-09-08 start failure still blocked `bun dev:puzzle:3d` two days
   * later while `nx daemon --start` ran fine). A watching invocation therefore drops a stale marker
   * and starts the daemon itself, so a developer's first `dev` never depends on a manual reset. */
  static ensureDaemon(nxCli: string, root: string, env: NodeJS.ProcessEnv): void {
    const marker = join(env.NX_WORKSPACE_DATA_DIRECTORY ?? join(root, ".nx", "workspace-data"), "d", "disabled");
    if (existsSync(marker)) rmSync(marker, { force: true });
    const started = stopNxProcessTree("node", [nxCli, "daemon", "--start"], { cwd: root, env, stdio: "inherit" });
    if (started.status !== 0) throw new Error(`Nx daemon did not start (status ${started.status ?? started.signal})`);
  }

  static ownedDescendants(roots: readonly number[], rows: readonly { pid: number; parent: number; command: string }[], daemonScript: string): number[] {
    const children = new Map<number, typeof rows[number][]>(), normalize = (path: string): string => path.replaceAll("\\", "/").toLowerCase();
    for (const row of rows) { const siblings = children.get(row.parent) ?? []; siblings.push(row); children.set(row.parent, siblings); }
    const seen = new Set(roots), pending = [...roots], owned: number[] = [], daemon = normalize(daemonScript);
    while (pending.length) for (const row of children.get(pending.pop()!) ?? []) {
      if (seen.has(row.pid) || normalize(row.command).includes(daemon)) continue;
      seen.add(row.pid); owned.push(row.pid); pending.push(row.pid);
    }
    return owned.sort((a, b) => a - b);
  }
  async run(segments: string[]): Promise<void> {
    let tooling: { cli: string; modulePath: string } | undefined;
    if (!existsSync(join(this.root, "node_modules/nx/package.json")) || segments.some(argument => /(?:^|[:,=])(?:deps-javascript|setup)(?:$|[,:])/.test(argument))) {
      const controller = new AbortController();
      let stopped: NodeJS.Signals | undefined;
      const stop = (signal: NodeJS.Signals): void => { stopped ??= signal; controller.abort(); };
      const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
      process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
      try {
        const api = nxBootstrapServices();
        tooling = await api.provisionNxTools(this.root, controller.signal);
        await api.activateNxTools(this.root, tooling, controller.signal);
      } catch (error) { if (!stopped) throw error; process.exitCode = stopped === "SIGINT" ? 130 : 143; return; }
      finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
    }
    const nxCli = tooling?.cli ?? createRequire(join(this.root, existsSync(join(this.root, ".nx/installation/package.json")) ? ".nx/installation/package.json" : "package.json")).resolve("nx/bin/nx.js");
    const invocation = resolveNxInvocation(segments), children: ReturnType<typeof spawnNxProcess>[] = [];
    const env = devToolingEnv({ ...invocation.env, ...(tooling ? { NODE_PATH: tooling.modulePath } : {}), NX_WORKSPACE_DATA_DIRECTORY: invocation.env.NX_WORKSPACE_DATA_DIRECTORY || process.env.NX_WORKSPACE_DATA_DIRECTORY || join(this.root, ".nx", "workspace-data"), NX_SOCKET_DIR: undefined, NX_DAEMON_SOCKET_DIR: undefined, npm_lifecycle_event: undefined, npm_lifecycle_script: undefined });
    let cancelled: NodeJS.Signals | undefined, cancellationDeadline = 0, watchFailure = 0, finishing = false;
    let force: ReturnType<typeof setTimeout> | undefined, watcher: ReturnType<typeof spawnNxProcess> | undefined;
    const descendants = new Set<number>();
    const captureDescendants = (): void => {
      const windows = process.platform === "win32";
      const snapshot = stopNxProcessTree(windows ? "powershell.exe" : "ps", windows ? ["-NoLogo", "-NoProfile", "-NonInteractive", "-Command", "Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId,CommandLine | ConvertTo-Json -Compress"] : ["-axo", "pid=,ppid=,command="], { encoding: "utf8", windowsHide: true, timeout: 5000 });
      if (snapshot.status !== 0) { console.error("Could not capture Nx descendants for cancellation"); return; }
      const rows = windows ? [JSON.parse(snapshot.stdout)].flat().filter((row) => row && Number.isSafeInteger(row.ProcessId) && row.ProcessId > 0 && Number.isSafeInteger(row.ParentProcessId)).map((row) => ({ pid: row.ProcessId, parent: row.ParentProcessId, command: typeof row.CommandLine === "string" ? row.CommandLine : "" })) : snapshot.stdout.trim().split("\n").flatMap((row) => { const match = row.trim().match(/^(\d+)\s+(\d+)\s+(.*)$/); return match ? [{ pid: Number(match[1]), parent: Number(match[2]), command: match[3] }] : []; });
      const daemon = createRequire(nxCli).resolve("nx/src/daemon/server/start.js");
      for (const pid of NxScript.ownedDescendants(children.flatMap((child) => child.pid ? [child.pid] : []), rows, daemon)) descendants.add(pid);
    };
    const kill = (child: ReturnType<typeof spawnNxProcess>, signal: NodeJS.Signals): void => {
      if (!child.pid) return;
      try { process.kill(-child.pid, signal); } catch {}
    };
    const killAll = (signal: NodeJS.Signals): void => {
      if (process.platform === "win32") {
        const pids = [...descendants, ...children.flatMap((child) => child.pid ? [child.pid] : [])];
        if (pids.length) stopNxProcessTree("taskkill", [...pids.flatMap((pid) => ["/pid", String(pid)]), "/f"], { stdio: "ignore", windowsHide: true });
        return;
      }
      for (const pid of descendants) try { process.kill(pid, signal); } catch {}
      children.forEach((child) => kill(child, signal));
    };
    const stop = (signal: NodeJS.Signals): void => {
      if (cancelled) return;
      cancelled = signal;
      cancellationDeadline = Date.now() + 5000;
      try { captureDescendants(); }
      catch (error) { console.error(`Could not inspect Nx descendants; stopping owned launch processes: ${error instanceof Error ? error.message : String(error)}`); }
      killAll(signal);
      force = setTimeout(() => killAll("SIGKILL"), 5000);
      force.unref();
    };
    const launch = (args: string[], capture = false): ReturnType<typeof spawnNxProcess> => {
      const child = spawnNxProcess("node", [nxCli, ...args], { cwd: this.root, env: nxChildEnvironment(env, args, Boolean(invocation.watch)), stdio: capture ? ["inherit", "pipe", "inherit"] : "inherit", detached: process.platform !== "win32" });
      children.push(child);
      return child;
    };
    const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
    process.once("SIGINT", interrupt);
    process.once("SIGTERM", terminate);
    const budget = orchestratorBudgetOpts().budgetMs ?? 0;
    const timeout = budget > 0 ? setTimeout(() => { console.error(`[budget] Nx exceeded ${budget}ms`); stop("SIGTERM"); }, budget) : undefined;
    try {
      if (invocation.watch) {
        NxScript.ensureDaemon(nxCli, this.root, env);
        watcher = launch(["watch", "--all", "--includeGlobalWorkspaceFiles", "--verbose", "--", "bun", "nx", "run", invocation.watch, "--output-style=stream"], true);
        watcher.stdout!.on("data", (chunk) => process.stdout.write(chunk));
        await waitForNxWatcher(watcher);
        watcher.once("close", (code) => { if (!finishing && !cancelled) { watchFailure = code || 1; stop("SIGTERM"); } });
      }
      if (!cancelled) {
        const child = launch(invocation.args);
        const status = await new Promise<number>((accept, reject) => { child.once("error", reject); child.once("close", (code) => accept(code ?? 1)); });
        process.exitCode = watchFailure || (cancelled ? cancelled === "SIGINT" ? 130 : 143 : status);
      } else process.exitCode = cancelled === "SIGINT" ? 130 : 143;
    } catch (error) {
      if (cancelled) process.exitCode = cancelled === "SIGINT" ? 130 : 143;
      else throw error;
    } finally {
      finishing = true;
      if (watcher && !cancelled) stop("SIGTERM");
      if (cancelled && process.platform !== "win32") {
        const alive = (child: ReturnType<typeof spawnNxProcess>): boolean => { try { if (child.pid) { process.kill(-child.pid, 0); return true; } } catch {} return false; };
        const descendantAlive = (pid: number): boolean => { try { process.kill(pid, 0); return true; } catch { return false; } };
        while ((children.some(alive) || [...descendants].some(descendantAlive)) && Date.now() < cancellationDeadline) await new Promise((accept) => setTimeout(accept, 25));
        killAll("SIGKILL");
        const settle = Date.now() + 1000;
        while ([...descendants].some(descendantAlive) && Date.now() < settle) await new Promise((accept) => setTimeout(accept, 25));
      }
      if (force) clearTimeout(force);
      if (timeout) clearTimeout(timeout);
      process.removeListener("SIGINT", interrupt);
      process.removeListener("SIGTERM", terminate);
    }
  }
}

/** 🧭️ Resolves public selections before Nx creates the single task graph. */
export function resolveNxInvocation(segments: string[]): { args: string[]; env: NodeJS.ProcessEnv; watch?: string } {
  if (segments[0] !== "run") return { args: segments, env: {} };
  const delimiter = segments.indexOf("--");
  const selected = delimiter < 0 ? [] : segments.slice(delimiter + 1);
  const options = segments.slice(2, delimiter < 0 ? undefined : delimiter);
  const target = segments[1];
  const print = target?.match(/^@semio-tech\/print:(build|watch)(?:-(.*))?$/);
  if (print) {
    const catalog = JSON.parse(readFileSync(join(WORKSPACE_ROOT, "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json"), "utf8")) as { documents: { id: string }[] };
    const suffix = print[2], viz = suffix === "viz" || selected[0] === "viz";
    const names = selected[0] === "viz" ? selected.slice(1) : selected;
    const ids = suffix && suffix !== "viz" ? [suffix] : names.map(name => viz && !name.startsWith("viz-") ? `viz-${name}` : name);
    if (suffix && suffix !== "viz" && selected.length) throw new Error("A Print document target accepts no compiler arguments");
    for (const id of ids) if (!catalog.documents.some(document => document.id === id)) throw new Error(`Unknown Print document: ${id}`);
    if (print[1] === "watch" && ids.length > 1) throw new Error("Select one Print document or a complete collection to watch");
    const targets = ids.length ? [...new Set(ids)].map(id => `${print[1]}-${id}`) : [`${print[1]}${viz ? "-viz" : ""}`];
    const args = targets.length === 1 ? ["run", `@semio-tech/print:${targets[0]}`, ...options] : ["run-many", "--projects=@semio-tech/print", `--targets=${targets.join(",")}`, ...options];
    return { args, env: {}, ...(print[1] === "watch" && !options.some(argument => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: `@semio-tech/print:${targets[0]!.replace(/^watch/, "build")}` } : {}) };
  }
  const report = target?.match(/^@semio-tech\/mit-bestand-bericht:(build|watch)(?:-(.*))?$/);
  if (report) {
    const catalog = JSON.parse(readFileSync(join(WORKSPACE_ROOT, "♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🔣️.json"), "utf8")) as { documents: { id: string }[] };
    if (selected.length) throw new Error("Report targets accept no compiler arguments");
    if (report[2] && !catalog.documents.some(document => document.id === report[2])) throw new Error(`Unknown report document: ${report[2]}`);
    return { args: ["run", target, ...options], env: {}, ...(report[1] === "watch" && !options.some(argument => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: target.replace(":watch", ":build") } : {}) };
  }
  const demonstrator = target?.match(/^@semio-tech\/mit-bestand-demonstrator:(prepare-(dev|release)|build|activate-dev|serve|dev|prepare-e2e|serve-e2e|test-e2e)$/);
  if (demonstrator) {
    if (selected.length) throw new Error("Demonstrator targets accept no compiler arguments");
    const release = demonstrator[1] === "build" || demonstrator[2] === "release";
    return { args: segments, env: { SEMIO_BUILD_MODE: release ? "ship" : "dev", SEMIO_RENDERER: "react" }, ...(demonstrator[1] === "dev" && !options.some(argument => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: "@semio-tech/mit-bestand-demonstrator:activate-dev" } : {}) };
  }
  const preparation = target?.match(/^@semio-tech\/framework-os-dev:(prepare|activate|serve|dev|build)-(.+)-react-(dev|release)$/);
  if (preparation) return { args: segments, env: { SEMIO_BUILD_MODE: preparation[3] === "release" ? "ship" : "dev", SEMIO_PLUGIN: preparation[2], SEMIO_RENDERER: "react" }, ...(preparation[1] === "dev" && !options.some((argument) => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: `@semio-tech/framework-os-dev:activate-${preparation[2]}-react-${preparation[3]}` } : {}) };
  if (["@semio-tech/framework-renderer-wgpu:native", "@semio-tech/framework-renderer-wgpu:native-build"].includes(target) && selected.some((argument) => argument === "--release" || argument === "--dist")) {
    const args = selected.filter((argument) => argument !== "--release" && argument !== "--dist");
    return { args: ["run", `${target}-release`, ...options, ...(args.length ? ["--", ...args] : [])], env: {} };
  }
  if (target === "workspace:setup" && selected.length) {
    const command = selected[0] === "deps" ? `deps-${selected[1]}` : selected[0] === "prepare" ? "prepare" : `setup-${selected[0]}`;
    return { args: ["run", `workspace:${command}`, ...options], env: {} };
  }
  if (target === "workspace:bench" || target === "@semio-tech/framework-os-dev:bench") {
    if (selected[0] !== "plugins") throw new Error("Select bench plugins through Nx");
    let renderer = "native";
    const args: string[] = [];
    for (let index = 1; index < selected.length; index++) {
      const argument = selected[index];
      if (argument === "--renderer") renderer = selected[++index];
      else if (argument.startsWith("--renderer=")) renderer = argument.slice(11);
      else args.push(argument);
    }
    if (!["native", "react", "wgpu"].includes(renderer)) throw new Error(`Unknown benchmark renderer: ${renderer}`);
    return { args: ["run", `@semio-tech/framework-os-dev:bench-plugins-${renderer}`, ...options, ...(args.length ? ["--", ...args] : [])], env: {} };
  }
  if (target === "workspace:cpp" && selected.length) {
    const [command, ...args] = selected;
    if (!["setup", "configure", "build", "test", "all"].includes(command)) throw new Error(`Unknown CMake operation: ${command}`);
    return { args: ["run", command === "all" ? "workspace:cpp" : `workspace:cpp-${command}`, ...options, ...(args.length ? ["--", ...args] : [])], env: {} };
  }
  if (target === "workspace:test") {
    const { resolveTestLevel } = nxRoutingServices();
    const { level, rest } = resolveTestLevel(selected);
    if (!rest.length) return { args: ["run", `workspace:test-${level}`, ...options], env: { SEMIO_TEST_LEVEL: level } };
    if (rest[0] === "repo-client" || rest[0] === "repo-mcp") {
      const project = rest[0] === "repo-client" ? "@semio-tech/repo-client" : "@semio-tech/repo-mcp-go";
      return { args: ["run", `${project}:${level === "fundamental" ? "test" : `test-${level}`}`, ...options, ...(rest.length > 1 ? ["--", ...rest.slice(1)] : [])], env: { SEMIO_TEST_LEVEL: level } };
    }
    const taxonomy = JSON.parse(readFileSync(join(WORKSPACE_ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
    if (((taxonomy.testPhases ?? []) as string[]).includes(rest[0])) return { args: ["run", `@semio-tech/repo-test-domain:test-${rest[0]}`, ...options, "--", ...(((taxonomy.testLevellessPhases ?? []) as string[]).includes(rest[0]) ? [] : [level]), ...rest.slice(1)], env: { SEMIO_TEST_LEVEL: level } };
  }
  if (target === "workspace:lint" && selected[0] === "repo") return { args: ["run", "workspace:lint-repo", ...options, ...(selected.length > 1 ? ["--", ...selected.slice(1)] : [])], env: {} };
  if (target === "workspace:dev" || target === "@semio-tech/framework-os-dev:dev") {
    if (selected[0] === "mcp" && ["stdio", "http"].includes(selected[1]) && selected[2] === "os") {
      const args = selected.slice(3);
      if (selected[1] === "http" && !args.includes("--port")) args.push("--port", process.env.S_OS_MCP_PORT ?? "6300");
      return { args: ["run", "@semio-tech/framework-os-mcp-rs:dev", ...options, "--", selected[1], ...args], env: {} };
    }
    if (selected[0] === "mcp" || selected[0] === "storybook-static") return { args: segments, env: {} };
    if (selected[0] === "storybook") return { args: ["run", "workspace:dev-storybook", ...options, "--", ...selected.slice(1)], env: {} };
    if (selected[0] === "multi") return { args: ["run", "@semio-tech/framework-os-dev:dev-s-react-dev", ...options, ...(selected.length > 1 ? ["--", ...selected.slice(1)] : [])], env: { S_OS_PORT: process.env.S_OS_PORT ?? "6071", SEMIO_RENDERER: "react", SEMIO_PLUGIN: "s", SEMIO_BUILD_MODE: "dev" }, ...(!options.some((argument) => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: "@semio-tech/framework-os-dev:activate-s-react-dev" } : {}) };
    const { loadFrameworkOsPlaygroundSelections, resolveFrameworkOsPlaygroundPlugin, frameworkOsPlaygroundDevEnv } = nxRoutingServices();
    const catalog = loadFrameworkOsPlaygroundSelections();
    const app = resolveFrameworkOsPlaygroundPlugin(catalog, selected.length ? selected : ["s"]);
    if (!app) throw new Error(`Unknown development selection: ${selected.join(" ")}`);
    const served = app.rest.includes("served"), env = frameworkOsPlaygroundDevEnv(catalog, app.plugin, served ? { SEMIO_RENDERER: "react" } : {});
    if (env.SEMIO_RENDERER === "react") {
      const profile = process.env.SEMIO_BUILD_MODE === "ship" ? "release" : "dev", command = served ? "serve" : "dev";
      const remaining = app.rest.filter((segment) => segment !== "served");
      return { args: ["run", `@semio-tech/framework-os-dev:${command}-${app.plugin}-react-${profile}`, ...options, ...(remaining.length ? ["--", ...remaining] : [])], env: { ...env, SEMIO_BUILD_MODE: profile === "release" ? "ship" : "dev" }, ...(!served && !options.some((argument) => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: `@semio-tech/framework-os-dev:activate-${app.plugin}-react-${profile}` } : {}) };
    }
    return { args: ["run", "@semio-tech/framework-os-dev:dev", ...options, "--", app.plugin, ...app.rest], env };
  }
  if (target === "workspace:build" && selected.length) {
    const targets: Record<string, string> = { assets: "@semio-tech/assets:build", storybook: "workspace:build-storybook", "repo-cli": "@semio-tech/repo-client:build", "repo-server": "@semio-tech/repo-coordinator:build", "repo-vscode": "@semio-tech/repo-vscode:build-vsix" };
    const resolved = targets[selected[0]];
    if (!resolved) throw new Error(`Unknown build selection: ${selected[0]}`);
    return { args: ["run", resolved, ...options, ...(selected.length > 1 ? ["--", ...selected.slice(1)] : [])], env: semioShipEnv() };
  }
  return { args: segments, env: target === "workspace:build" ? semioShipEnv() : {} };
}
//#endregion 🔖️NxScript

if (import.meta.main) await new ScriptRouter(WORKSPACE_ROOT).register("nx", NxScript).run(process.argv.slice(2));
