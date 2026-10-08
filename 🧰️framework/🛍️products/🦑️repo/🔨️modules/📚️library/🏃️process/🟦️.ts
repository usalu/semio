import { buildBudgetMs, cmdBudgetMs, orchestratorBudgetMs, daemonBudgetMs } from "../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { terminateOwnedProcessTree } from "../../../../../🔨️modules/🏃️process/🪓️termination/🟦️.ts";

/** 🏃️ Process execution with opt-in wall-clock budgets for repository commands and builds,
 * the `spawnSync` runners built on them, workspace-aware `.bin`
 * resolution and the dev/ship build-mode switch. Split out of `📦️packages/🟦️typescript/🟦️.ts` so a
 * consumer that only spawns a tool (the plugin package's jco/wasm-opt steps, and through them the
 * extension store and `⚙️vite.config.ts`) never drags the repository library's `🔍️discovery` taxonomy
 * walk into its module graph. */
import { spawn, spawnSync } from "node:child_process";
import { createInterface } from "node:readline";
import { randomUUID } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve, isAbsolute } from "node:path";
import { fileURLToPath } from "node:url";
import { cargoInvocationManifestV1, prepareCargoWorkspaceInvocation } from "../🗂️workspaces/🦀️cargo/🟦️.ts";
import { getWorkspaceRoot } from "../🗂️workspaces/🟦️.ts";
import { cargoDirectories } from "../⚡️caching/🦀️cargo/🟦️.ts";
import { devToolingEnv } from "./🌿️environment/🟦️.ts";

/** 🧭️Selects the opt-in build budget for Cargo and command budget for other executables; both default to unlimited. */
export function defaultBudgetMs(cmd: string): number {
  return cmd === "cargo" ? buildBudgetMs() : cmdBudgetMs();
}

/** ⏱️Timeout hint for a budget-exceeded message; `cargo` commands default to the shared target-dir lock-contention hint (by far the most common real cause), everything else to a generic budget-tuning hint. An explicit `override` always wins. */
export function budgetTimeoutHint(cmd: string, override?: string): string {
  if (override) return override;
  return cmd === "cargo"
    ? "Likely shared cargo target-dir lock contention from another concurrent session — investigate before retrying."
    : "Trim it, or raise its budget (`budgetMs`, `SEMIO_CMD_BUDGET_MS`, `SEMIO_BUILD_BUDGET_MS`).";
}

export interface RunCmdOpts {
  cwd?: string;
  env?: NodeJS.ProcessEnv;
  /** ⏱️Wall-clock budget (ms); zero disables the timeout. Defaults to [[defaultBudgetMs]]. Named wrappers use [[orchestratorBudgetOpts]] / [[daemonBudgetOpts]]. */
  budgetMs?: number;
  onTimeoutHint?: string;
  /** 🐚️ Resolve the command through the platform shell — required on native Windows for `.cmd`/`.bat` launchers such as `bunx`. */
  shell?: boolean;
}

/** ⏱️[[RunCmdOpts]] preset for nx/script orchestrators — [[orchestratorBudgetMs]] and full CPU [[devToolingEnv]]. */
export function orchestratorBudgetOpts(extra: Partial<NodeJS.ProcessEnv> = {}): RunCmdOpts {
  return { budgetMs: orchestratorBudgetMs(), env: devToolingEnv(extra) };
}

/** ⏱️[[RunCmdOpts]] preset for dev servers and long-lived daemons — [[daemonBudgetMs]] and [[devToolingEnv]]. */
export function daemonBudgetOpts(extra: Partial<NodeJS.ProcessEnv> = {}): RunCmdOpts {
  return { budgetMs: daemonBudgetMs(), env: devToolingEnv(extra) };
}

/** 🌊️ The library's cargo relay, `⚡️caching/🦀️cargo/📜️script.ts relay <cargo args…>` ([[cargoStreamingStatus]]). */
export const CARGO_RELAY_SCRIPT = join(dirname(fileURLToPath(import.meta.url)), "..", "⚡️caching", "🦀️cargo", "📜️script.ts");

/** 🌊️ Carries a relayed cargo's wall-clock budget (ms, `0` = unlimited) from [[runCmd]] into the relay. */
export const CARGO_RELAY_BUDGET_ENV = "SEMIO_CARGO_RELAY_BUDGET_MS";

const observesCargoInvocation = (args: readonly string[], env: NodeJS.ProcessEnv): boolean => Boolean(env.SEMIO_TEST_ARTIFACT_DIR) || args.some((arg, index) => /^--message-format=json/u.test(arg) || arg === "--message-format" && args[index + 1]?.startsWith("json"));

/** 🌊️ Runs `cargo` with its stdout/stderr piped and forwarded, never inherited: Bun marks its own stdio `O_NONBLOCK` once
 * written and an inherited pipe shares that flag, so a cargo burst (the replayed warnings of fresh units) fails with
 * `EAGAIN` — output cut mid-line, exit 101 — as soon as a slow reader lets the 64 KiB pipe fill (ticket 26/09/23 W4: flaky
 * component builds, the trusted-catalog publish's `os-hub` build). Bun's own writer waits the pipe out. Returns cargo's exit
 * status; `budgetMs` (> 0) elapsing or a SIGINT/SIGTERM of this process ends the whole cargo tree and throws. */
export async function cargoStreamingStatus(args: readonly string[], cwd: string, env: NodeJS.ProcessEnv, budgetMs: number): Promise<number> {
  prepareCargoWorkspaceInvocation(getWorkspaceRoot(env), args, cwd,env);
  const observes = observesCargoInvocation(args, env), buildDirectory = observes ? cargoDirectories(getWorkspaceRoot(), env).build : undefined, provenanceRoot = env.SEMIO_TEST_ARTIFACT_DIR ?? (buildDirectory ? join(buildDirectory, "semio-cargo-provenance") : undefined), builtAtMs = Date.now(), units: any[] = [], buildScripts: any[] = [];
  const child = spawn("cargo", [...args], { cwd, env: buildDirectory ? { ...env, SEMIO_COMPILER_RESOURCE_ROOT: join(buildDirectory, "semio-compiler-resources") } : env, stdio: ["inherit", "pipe", "pipe"], detached: process.platform !== "win32" });
  const observation = provenanceRoot ? (async () => { for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) { let message; try { message = JSON.parse(line); } catch { continue; } if (message.reason === "compiler-artifact") units.push({ message }); else if (message.reason === "build-script-executed") buildScripts.push(message); } })() : Promise.resolve();
  child.stdout!.pipe(process.stdout, { end: false });
  child.stderr!.pipe(process.stderr, { end: false });
  let stopped: string | undefined, status = 1;
  const stop = (reason: string): void => {
    stopped ??= reason;
    if (child.exitCode === null && child.signalCode === null && child.pid) terminateOwnedProcessTree(child.pid);
  };
  const onSignal = (signal: NodeJS.Signals): void => stop(`stopped: the relay received ${signal}`);
  const budget = budgetMs > 0 ? setTimeout(() => {
    console.error(`[budget] cargo ${args.join(" ")} exceeded ${budgetMs}ms — killed. ${budgetTimeoutHint("cargo")}`);
    stop(`exceeded ${budgetMs}ms`);
  }, budgetMs) : undefined;
  process.once("SIGINT", onSignal);
  process.once("SIGTERM", onSignal);
  try {
    const [code, signal] = await new Promise<[number | null, NodeJS.Signals | null]>((accept, reject) => {
      child.once("error", reject);
      child.once("close", (exitCode, exitSignal) => accept([exitCode, exitSignal]));
    });
    status = code ?? 1;
    if (stopped || signal) throw new Error(`cargo ${args.join(" ")} ${stopped ?? `killed by signal ${signal}`}`);
    return status;
  } finally {
    if (budget) clearTimeout(budget);
    process.removeListener("SIGINT", onSignal);
    process.removeListener("SIGTERM", onSignal);
    await observation;
    if (provenanceRoot) {
      const { writeCompletedCargoInvocationProvenanceV1 } = await import("../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts");
      const manifest = cargoInvocationManifestV1(args, cwd);
      writeCompletedCargoInvocationProvenanceV1(join(provenanceRoot, `cargo-unit-provenance-cargo-relay-${randomUUID()}.json`), { manifest, cwd, command: "cargo", args: [...args], buildDirectory: buildDirectory!, builtAtMs, status, cancelled: Boolean(stopped), units, buildScripts }, undefined, env.CARGO_HOME);
    }
  }
}

/** ⏱️Shared `spawnSync` core for [[runCmd]]/[[runCmdStatus]]: throws on spawn error, budget timeout, or signal kill (printing `[budget]` first on timeout); otherwise returns the exit status. `cargo` runs through
 * [[CARGO_RELAY_SCRIPT]] (POSIX), so it never writes to an inherited, possibly non-blocking descriptor ([[cargoStreamingStatus]]). */
function runCmdInternal(cmd: string, args: string[], opts: RunCmdOpts): number {
  const budgetMs = opts.budgetMs ?? defaultBudgetMs(cmd);
  const formattedArgs = [...args];
  if (cmd === "bun" || cmd === process.execPath) {
    if (formattedArgs[0] && !formattedArgs[0].startsWith("-") && !formattedArgs[0].includes("/") && !formattedArgs[0].includes("\\") && !workspaceScriptExists(formattedArgs[0])) {
      const resolved = resolveWorkspaceBin(formattedArgs[0], opts.cwd ?? process.cwd());
      if (resolved) {
        formattedArgs[0] = resolved;
      }
    }
  }
  const relayed = cmd === "cargo" && (process.platform !== "win32" || observesCargoInvocation(formattedArgs, opts.env ?? process.env));
  if (cmd === "cargo" && !relayed) prepareCargoWorkspaceInvocation(getWorkspaceRoot(), args, opts.cwd ?? process.cwd(), opts.env ?? process.env);
  const result = relayed
    ? spawnSync(process.versions.bun ? process.execPath : "bun", [CARGO_RELAY_SCRIPT, "relay", ...formattedArgs], {
        stdio: "inherit",
        cwd: opts.cwd,
        env: { ...(opts.env ?? process.env), [CARGO_RELAY_BUDGET_ENV]: String(budgetMs) },
        timeout: budgetMs > 0 ? budgetMs + 60_000 : 0,
        killSignal: "SIGTERM",
        shell: false,
      })
    : spawnSync(cmd, formattedArgs, {
        stdio: "inherit",
        cwd: opts.cwd,
        env: opts.env ?? process.env,
        timeout: budgetMs,
        killSignal: "SIGKILL",
        shell: opts.shell ?? false,
      });
  if (result.error) {
    if ((result.error as NodeJS.ErrnoException).code === "ETIMEDOUT") {
      console.error(`[budget] ${cmd} ${args.join(" ")} exceeded ${budgetMs}ms — killed. ${budgetTimeoutHint(cmd, opts.onTimeoutHint)}`);
    }
    throw result.error;
  }
  if (result.signal) throw new Error(`${cmd} ${args.join(" ")} killed by signal ${result.signal}`);
  return result.status ?? 1;
}

/**
 * 🏃️Runs a subprocess with inherited stdio and an opt-in wall-clock budget (default [[defaultBudgetMs]]);
 * throws on non-zero exit, signal, or budget exceed (the `[budget]` line is printed
 * to stderr first so it survives a caller's try/catch, e.g. [[tryRun]]).
 */
export function runCmd(cmd: string, args: string[], opts: RunCmdOpts = {}): void {
  const status = runCmdInternal(cmd, args, opts);
  if (status !== 0) throw new Error(`${cmd} ${args.join(" ")} exited with status ${status}`);
}

/** 🏃️Like [[runCmd]] but returns the exit status instead of throwing on non-zero exit — for call sites
 *  that branch on it. Budget exceed still prints `[budget]` and throws (never silently returns a status). */
export function runCmdStatus(cmd: string, args: string[], opts: RunCmdOpts = {}): number {
  return runCmdInternal(cmd, args, opts);
}

/** 🏃️Like [[runCmd]] but ignores failures, including timeouts from an explicitly configured budget. */
export function tryRun(cmd: string, args: string[], opts: RunCmdOpts = {}): void {
  try {
    runCmd(cmd, args, opts);
  } catch {
    /* optional */
  }
}

/** 🔍️ Resolves a Node tool from installation launchers or its package bin declaration in the caller and workspace scopes. */
export function resolveWorkspaceBin(binName: string, cwd: string = process.cwd()): string | null {
  const shortName = binName.includes("/") ? binName.split("/").pop()! : binName;
  for (const root of new Set([cwd, getWorkspaceRoot()])) {
    const launcher = join(root, "node_modules", ".bin", shortName);
    if (existsSync(launcher)) return launcher;
    const owner = join(root, "node_modules", binName), manifest = join(owner, "package.json");
    if (!existsSync(manifest)) continue;
    const { bin } = JSON.parse(readFileSync(manifest, "utf8")) as { bin?: string | Record<string, string> };
    const entry = typeof bin === "string" ? bin : bin?.[shortName];
    if (typeof entry !== "string") continue;
    const executable = resolve(owner, entry), path = relative(owner, executable).replaceAll("\\", "/");
    if (!isAbsolute(path) && path !== ".." && !path.startsWith("../") && existsSync(executable)) return executable;
  }
  return null;
}

let workspaceScriptNames: ReadonlySet<string> | null = null;

/** 📜️Whether the workspace `package.json` declares a script under `name`. A declared script is the
 * workspace's deliberate wrapper for that tool (`nx` routes through the caching bootstrap, which
 * owns the daemon-served project graph the async ES-module inference plugin needs), so `bun <name>`
 * must reach the script and never the bare `node_modules/.bin` entry of the same name. */
export function workspaceScriptExists(name: string): boolean {
  if (workspaceScriptNames === null) {
    const manifest = join(getWorkspaceRoot(), "package.json");
    const scripts = existsSync(manifest) ? (JSON.parse(readFileSync(manifest, "utf8")) as { scripts?: Record<string, string> }).scripts ?? {} : {};
    workspaceScriptNames = new Set(Object.keys(scripts));
  }
  return workspaceScriptNames.has(name);
}

/** 🟢️Runs a CLI tool using `node` synchronously in `cwd`, returning status code. */
export function runNodeBinStatus(args: string[], cwd: string = process.cwd(), env: NodeJS.ProcessEnv = process.env): number {
  const binName = args[0]!;
  const resolved = resolveWorkspaceBin(binName, cwd);
  const executable = resolved ?? binName;
  const result = spawnSync("node", [executable, ...args.slice(1)], { cwd, env, shell: false, stdio: "inherit" });
  if (result.error) {
    console.error(result.error);
    return 1;
  }
  return result.status ?? 1;
}

/** 🟢️Runs a CLI tool using `node` synchronously in `cwd`. */
export function runNodeBin(args: string[], cwd: string = process.cwd(), env: NodeJS.ProcessEnv = process.env): void {
  const status = runNodeBinStatus(args, cwd, env);
  if (status !== 0) process.exit(status);
}

/** 🚦️ Whether child builds should use fast dev artifacts or ship optimization. */
export type SemioBuildMode = "dev" | "ship";

/** 🚦️ `ship` only when `SEMIO_BUILD_MODE=ship`; default is dev for local/agent loops. */
export function semioBuildMode(): SemioBuildMode {
  return process.env.SEMIO_BUILD_MODE === "ship" ? "ship" : "dev";
}

/** 🚀️ Env for nx/build orchestrators so spawned crate `wasm` scripts inherit ship mode. */
export function semioShipEnv(): NodeJS.ProcessEnv {
  return { ...process.env, SEMIO_BUILD_MODE: "ship" };
}

/** 📂 Cargo output directory name for a profile (`dev` → `debug`). */
export function cargoProfileDir(profile: string): string {
  return profile === "dev" ? "debug" : profile;
}
