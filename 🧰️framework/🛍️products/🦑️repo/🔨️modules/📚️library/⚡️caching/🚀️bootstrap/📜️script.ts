#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { spawn as spawnNxProcess, spawnSync as stopNxProcessTree } from "node:child_process";
import { existsSync, readFileSync, rmSync } from "node:fs";
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { join, resolve, win32, posix } from "node:path";
import { Script, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { orchestratorBudgetOpts, semioShipEnv } from "../../🏃️process/🟦️.ts";
import { devToolingEnv, semioNxParallelFlag } from "../../🏃️process/🌿️environment/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

const WORKSPACE_ROOT = getWorkspaceRoot();

/** 🥖️ Carries the acquired Bun identity into every Nx producer and source watcher. */
export function pinnedNxRuntimeEnvironment(source:NodeJS.ProcessEnv,bun:string,platform:NodeJS.Platform=process.platform):NodeJS.ProcessEnv {
  const paths=platform==="win32"?win32:posix;
  if(!paths.isAbsolute(bun))throw new Error("Nx Bun runtime must be absolute");
  const environment={...source},keys=Object.keys(source).filter(key=>platform==="win32"?key.toUpperCase()==="PATH":key==="PATH"),key=keys.find(value=>value==="PATH")??keys[0]??"PATH",previous=source[key]??"";
  for(const candidate of keys)delete environment[candidate];
  const directory=paths.dirname(bun),entries=previous.split(paths.delimiter).filter(entry=>entry && (platform==="win32"?entry.toLowerCase()!==directory.toLowerCase():entry!==directory));
  environment[key]=[directory,...entries].join(paths.delimiter);
  return environment;
}

/** 🧭️ Loads domain selection helpers only for commands that request them. */
function nxRoutingServices(): typeof import("../../📦️packages/🟦️typescript/🟦️.ts") {
  return createRequire(import.meta.url)("../../📦️packages/🟦️typescript/🟦️.ts");
}

/** 🛠️ Loads the owner of the current immutable Nx recipe and patch identity. */
function nxBootstrapServices(): typeof import("./🛠️tools/📜️script.ts") {
  return createRequire(import.meta.url)("./🛠️tools/📜️script.ts");
}

/** 🌱️ Publishes the declared bootstrap sources — generated files Bun or the repository tooling must find before Nx runs
 * and a fresh clone lacks — through their owners' dependency-free publishers, required lazily so the Nx bootstrap closure
 * stays system-only. @see ./🌱️sources/🔣️.json */
export function publishBootstrapSources(root: string): void {
  const declaration = join(import.meta.dirname, "🌱️sources", "🔣️.json");
  if (!existsSync(declaration)) return;
  const { sources } = JSON.parse(readFileSync(declaration, "utf8")) as { sources: readonly { module: string; export: string }[] };
  for (const source of sources) createRequire(import.meta.url)(join(root, source.module))[source.export](root);
}

/** 🧮️ Lets each watched build finish its graph while the daemon retains source watching. */
export function nxChildEnvironment(environment: NodeJS.ProcessEnv, args: readonly string[], watching: boolean): NodeJS.ProcessEnv {
  if (args[0] === "watch") return { ...environment, NX_DAEMON: "true" };
  return { ...environment, NX_DAEMON: watching || environment.NX_FILE_CHANGES !== undefined ? "false" : environment.NX_DAEMON ?? "false" };
}

/** 🧷️ Gives an activation binding its own watcher graph and short portable daemon socket path. */
export function nxWatcherEnvironment(root: string, environment: Record<string, string | undefined>, activation: string): Record<string, string | undefined> {
  const binding = Object.entries(environment).filter(([key]) => /^(?:S_|VITE_|PLAYGROUND_|SEMIO_(?:PLUGIN|RENDERER|BUILD_MODE|LOCALE|LOCKED_LOCALE|LOCKED_TERMINOLOGY)$)/.test(key)).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0);
  const identity = createHash("sha256").update(JSON.stringify([resolve(root).replaceAll("\\", "/"), activation, binding])).digest("hex").slice(0, 24);
  const temporary = tmpdir(), socketRoot = process.platform !== "win32" && Buffer.byteLength(temporary) > 50 ? "/tmp" : temporary;
  return { ...environment, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx", "workspace-data", "watchers", identity), NX_SOCKET_DIR: join(socketRoot, `semio-nx-${identity}`), NX_DAEMON_SOCKET_DIR: undefined };
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

/** 🔁️ Reconciles sources after both watcher readiness and the initial prerequisite graph. */
export function activateWhenNxWatcherReady(watcher: ReturnType<typeof spawnNxProcess>, prepared: Promise<void>, active: () => boolean, activate: () => void, failed: (error: Error) => void): void {
  void Promise.all([waitForNxWatcher(watcher), prepared]).then(() => { if (active()) activate(); }).catch(error => { if (active()) failed(error); });
}

/** 🛡️ Registers automatic source callbacks after the initial prerequisite graph has settled. */
export async function startNxWatcherAfterPreparation(prepared:Promise<void>,active:()=>boolean,start:()=>ReturnType<typeof spawnNxProcess>):Promise<ReturnType<typeof spawnNxProcess>|undefined> {
  await prepared;return active()?start():undefined;
}

/** ⏳️ Observes Nx starting the serving target after its prerequisite graph completes. */
export function waitForNxTargetStart(child: ReturnType<typeof spawnNxProcess>, target: string): Promise<void> {
  return new Promise((accept,reject) => {
    let pending="";
    const finish=(error?:Error):void => { child.stdout!.removeListener("data",received);child.removeListener("close",closed);child.removeListener("error",failed);if(error)reject(error);else accept(); };
    const received=(chunk:Buffer):void => { pending=(pending+chunk.toString()).replace(/\x1b\[[0-9;?]*[A-Za-z]/g,"").slice(-8192);const marker="> nx run "+target;if([" ","\r","\n"].some(separator=>pending.includes(marker+separator)))finish(); };
    const closed=(code:number|null):void => finish(new Error("Nx initial preparation ended before serving target "+target+" ("+code+")"));
    const failed=(error:Error):void => finish(error);
    child.stdout!.on("data",received);child.once("close",closed);child.once("error",failed);
  });
}

//#region 🔖️NxScript
export class NxScript extends Script {
  /** 📦️ The locked JavaScript environment every workspace script task imports from. On a fresh clone (no
   * `node_modules/nx`) it runs to completion before the requested graph: Nx schedules sibling setup tasks in parallel,
   * and without this every generator, `setup-git`, `cpp-setup` and `deps-browsers` started before `bun install` finished
   * (measured on a clean Linux container, ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP `📓️wp-z2.md`). */
  static readonly javascriptEnvironment = "workspace:deps-javascript";

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
    let tooling: { cli: string; modulePath: string; bun: string };
    {
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
    const nxCli = tooling.cli;
    const invocation = resolveNxInvocation(segments), children: ReturnType<typeof spawnNxProcess>[] = [];
    const env = pinnedNxRuntimeEnvironment(devToolingEnv({ ...invocation.env, NX_ISOLATE_PLUGINS: process.env.NX_ISOLATE_PLUGINS ?? "false", NODE_PATH: tooling.modulePath, NX_WORKSPACE_DATA_DIRECTORY: invocation.env.NX_WORKSPACE_DATA_DIRECTORY || process.env.NX_WORKSPACE_DATA_DIRECTORY || join(this.root, ".nx", "workspace-data"), NX_SOCKET_DIR: undefined, NX_DAEMON_SOCKET_DIR: undefined, npm_lifecycle_event: undefined, npm_lifecycle_script: undefined }), tooling.bun);
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
    const launch = (args: string[], capture = false, environment = env): ReturnType<typeof spawnNxProcess> => {
      const child = spawnNxProcess("node", [nxCli, ...args], { cwd: this.root, env: nxChildEnvironment(environment, args, Boolean(invocation.watch)), stdio: capture ? ["inherit", "pipe", "inherit"] : "inherit", detached: process.platform !== "win32" });
      children.push(child);
      return child;
    };
    const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
    process.once("SIGINT", interrupt);
    process.once("SIGTERM", terminate);
    const budget = orchestratorBudgetOpts().budgetMs ?? 0;
    const timeout = budget > 0 ? setTimeout(() => { console.error(`[budget] Nx exceeded ${budget}ms`); stop("SIGTERM"); }, budget) : undefined;
    try {
      if (!existsSync(join(this.root, "node_modules/nx/package.json")) && !invocation.args.includes(NxScript.javascriptEnvironment)) {
        const install = launch(["run", NxScript.javascriptEnvironment, "--output-style=stream"]);
        const installed = await new Promise<number>((accept, reject) => { install.once("error", reject); install.once("close", (code) => accept(code ?? 1)); });
        if (installed !== 0 || cancelled) { process.exitCode = cancelled ? cancelled === "SIGINT" ? 130 : 143 : installed; return; }
      }
      if (!cancelled) {
        const child=launch(invocation.args,Boolean(invocation.watch));
        const status=new Promise<number>((accept,reject) => { child.once("error",reject);child.once("close",code=>accept(code??1)); });void status.catch(()=>{});
        if(invocation.watch) {
          const prepared=invocation.watch.includes(":activate-") ? waitForNxTargetStart(child,invocation.args[1]!) : Promise.resolve();
          child.stdout!.on("data",chunk=>process.stdout.write(chunk));
          const watchEnvironment=nxWatcherEnvironment(this.root,env,invocation.watch);
          watcher=await startNxWatcherAfterPreparation(prepared,()=>!finishing&&!cancelled,()=>{
            NxScript.ensureDaemon(nxCli,this.root,watchEnvironment);
            const registered=launch(["watch","--all","--includeGlobalWorkspaceFiles","--verbose","--","bun","nx","run",invocation.watch,"--output-style=stream"],true,watchEnvironment);
            return registered;
          });
          if(!watcher){process.exitCode=cancelled?cancelled==="SIGINT"?130:143:await status;return;}
          watcher.stdout!.on("data",chunk=>process.stdout.write(chunk));
          watcher.once("close",code=>{if(!finishing&&!cancelled){watchFailure=code||1;stop("SIGTERM");}});
          activateWhenNxWatcherReady(watcher,Promise.resolve(),()=>!finishing&&!cancelled,()=>{
            const activation=launch(["run",invocation.watch!,"--output-style=stream"]);
            activation.once("error",error=>{console.error(error.message);watchFailure=1;stop("SIGTERM");});
            activation.once("close",code=>{if(code&&!finishing&&!cancelled){watchFailure=code;stop("SIGTERM");}});
          },error=>{console.error(error.message);watchFailure=1;stop("SIGTERM");});
        }
        const code=await status;
        process.exitCode=watchFailure||(cancelled?cancelled==="SIGINT"?130:143:code);
      } else process.exitCode=cancelled==="SIGINT"?130:143;
    } catch (error) {
      if (cancelled) process.exitCode = cancelled === "SIGINT" ? 130 : 143;
      else throw error;
    } finally {
      finishing = true;
      if (invocation.watch && children.length && !cancelled) stop("SIGTERM");
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
    const args = targets.length === 1 ? ["run", `@semio-tech/print:${targets[0]}`, ...options] : ["run-many", "--projects=@semio-tech/print", `--targets=${targets.join(",")}`, ...semioNxParallelFlag(), ...options];
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
  if (["@semio-tech/framework-renderer-wgpu:dev", "@semio-tech/framework-renderer-wgpu:serve"].includes(target)) {
    const command = target.endsWith(":dev") ? "dev" : "serve", variant = process.env.SEMIO_PLUGIN ?? "s", profile = process.env.SEMIO_BUILD_MODE === "ship" ? "release" : "dev";
    return { args: ["run", `@semio-tech/framework-os-dev:${command}-${variant}-wgpu-${profile}`, ...options, ...(selected.length ? ["--", ...selected] : [])], env: { SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "ship" : "dev" }, ...(command === "dev" && !options.some(argument => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: `@semio-tech/framework-os-dev:activate-${variant}-wgpu-${profile}` } : {}) };
  }
  const nativeRuntime = target?.match(/^@semio-tech\/framework-os-dev:(prepare|run|smoke)-(.+)-native-(dev|release)$/);
  if (nativeRuntime) return { args: segments, env: { SEMIO_PLUGIN: nativeRuntime[2], SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: nativeRuntime[3] === "release" ? "ship" : "dev" } };
  const preparation = target?.match(/^@semio-tech\/framework-os-dev:(prepare|activate|serve|dev|build)-(.+)-(react|wgpu)-(dev|release)$/);
  if (preparation) return { args: segments, env: { SEMIO_BUILD_MODE: preparation[4] === "release" ? "ship" : "dev", SEMIO_PLUGIN: preparation[2], SEMIO_RENDERER: preparation[3] }, ...(preparation[1] === "dev" && !options.some((argument) => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: `@semio-tech/framework-os-dev:activate-${preparation[2]}-${preparation[3]}-${preparation[4]}` } : {}) };
  if (["@semio-tech/framework-renderer-wgpu:native", "@semio-tech/framework-renderer-wgpu:native-release"].includes(target)) {
    const release = target.endsWith("-release") || selected.some(argument => ["--release", "--dist"].includes(argument));
    const args = selected.filter(argument => !["--release", "--dist"].includes(argument)), profile = release ? "release" : "dev";
    if (args.includes("--scale")) return { args: ["run", `@semio-tech/framework-renderer-wgpu:native-scale${release ? "-release" : ""}`, ...options, "--", ...args], env: {} };
    const variant = args[0] && !args[0].startsWith("-") ? args.shift()! : process.env.SEMIO_PLUGIN ?? "s";
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(variant) || args.some(argument => argument !== "--smoke") || args.length > 1) throw new Error("Select one native variant and optional --smoke");
    return { args: ["run", `@semio-tech/framework-os-dev:${args.includes("--smoke") ? "smoke" : "run"}-${variant}-native-${profile}`, ...options], env: { SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: release ? "ship" : "dev" } };
  }
  if (["@semio-tech/framework-renderer-wgpu:native-build"].includes(target) && selected.some((argument) => argument === "--release" || argument === "--dist")) {
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
    const { level, rest } = resolveTestLevel(selected);
    if (!rest.length) return { args: ["run", `workspace:test-${level}`, ...options], env: { SEMIO_TEST_LEVEL: level } };
    if (rest[0] === "repo-client" || rest[0] === "repo-mcp") {
      const project = rest[0] === "repo-client" ? "@semio-tech/repo-client" : "repo-mcp";
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
    const app = resolveFrameworkOsPlaygroundPlugin(catalog, selected.length ? selected : process.env.SEMIO_PLUGIN ? [process.env.SEMIO_PLUGIN] : ["s"]);
    if (!app) throw new Error(`Unknown development selection: ${selected.join(" ")}`);
    const served = app.rest.includes("served"), env = frameworkOsPlaygroundDevEnv(catalog, app.plugin, served ? { SEMIO_RENDERER: "react" } : {});
    if (["react", "wgpu"].includes(env.SEMIO_RENDERER!)) {
      const profile = process.env.SEMIO_BUILD_MODE === "ship" ? "release" : "dev", command = served ? "serve" : "dev";
      const remaining = app.rest.filter((segment) => segment !== "served");
      return { args: ["run", `@semio-tech/framework-os-dev:${command}-${app.plugin}-${env.SEMIO_RENDERER}-${profile}`, ...options, ...(remaining.length ? ["--", ...remaining] : [])], env: { ...env, SEMIO_BUILD_MODE: profile === "release" ? "ship" : "dev" }, ...(!served && !options.some((argument) => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: `@semio-tech/framework-os-dev:activate-${app.plugin}-${env.SEMIO_RENDERER}-${profile}` } : {}) };
    }
    throw new Error(`Unknown development renderer: ${env.SEMIO_RENDERER}`);
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

if (import.meta.main) {
  const args=process.argv.slice(2);
  const native=await import("../../../🎛️dashboard/📦️installation/🟦️.ts"),invocation=args[0]==="nx"&&native.dashboardInstalled(WORKSPACE_ROOT)?native.dashboardInvocation(args.slice(1)):undefined;
  if(invocation){try{process.exitCode=await native.launchDashboard(WORKSPACE_ROOT,invocation);}catch(error){console.error(error instanceof Error?error.message:String(error));process.exitCode=1;}}
  else{publishBootstrapSources(WORKSPACE_ROOT);await new ScriptRouter(WORKSPACE_ROOT).register("nx", NxScript).run(args);}
}
