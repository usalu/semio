/** @emoji 🏃️ Process execution with opt-in wall-clock budgets for repository commands and builds,
 * the `spawnSync` runners built on them, workspace-aware `.bin`
 * resolution and the dev/ship build-mode switch. Split out of `📦️packages/🟦️typescript/🟦️.ts` so a
 * consumer that only spawns a tool (the plugin package's jco/wasm-opt steps, and through them the
 * extension store and `⚙️vite.config.ts`) never drags the repository library's `🔍️discovery` taxonomy
 * walk into its module graph. */
import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../🗂️workspaces/🟦️.ts";

/** 🏗️Default build budget (ms): zero leaves compilation and Cargo lock waits unlimited. Opt in via `SEMIO_BUILD_BUDGET_MS`. */
export const BUILD_BUDGET_MS = 0;

/** ⏱️Resolves the active build-class budget: `SEMIO_BUILD_BUDGET_MS` env override, else [[BUILD_BUDGET_MS]]. */
export function buildBudgetMs(): number {
  return Number(process.env.SEMIO_BUILD_BUDGET_MS ?? BUILD_BUDGET_MS);
}

/** 🛠️Default command budget (ms): zero lets build tools and script wrappers finish. Opt in via `SEMIO_CMD_BUDGET_MS`. */
export const CMD_BUDGET_MS = 0;

/** ⏱️Resolves the active generic-command budget: `SEMIO_CMD_BUDGET_MS` env override, else [[CMD_BUDGET_MS]]. */
export function cmdBudgetMs(): number {
  return Number(process.env.SEMIO_CMD_BUDGET_MS ?? CMD_BUDGET_MS);
}

/** 🎛️Default Nx/script orchestrator budget (ms): zero avoids imposing a parent build deadline. Opt in via `SEMIO_ORCHESTRATOR_BUDGET_MS`. */
export const ORCHESTRATOR_BUDGET_MS = 0;

/** 🖥️Default dev-server and daemon budget (ms): zero permits long-lived sessions and their builds. Opt in via `SEMIO_DAEMON_BUDGET_MS`. */
export const DAEMON_BUDGET_MS = 0;

/** ⏱️Resolves the active orchestrator budget: `SEMIO_ORCHESTRATOR_BUDGET_MS` env override, else [[ORCHESTRATOR_BUDGET_MS]]. */
export function orchestratorBudgetMs(): number {
  return Number(process.env.SEMIO_ORCHESTRATOR_BUDGET_MS ?? ORCHESTRATOR_BUDGET_MS);
}

/** ⏱️Resolves the active daemon budget: `SEMIO_DAEMON_BUDGET_MS` env override, else [[DAEMON_BUDGET_MS]]. */
export function daemonBudgetMs(): number {
  return Number(process.env.SEMIO_DAEMON_BUDGET_MS ?? DAEMON_BUDGET_MS);
}

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
}

/** ⏱️[[RunCmdOpts]] preset for nx/script orchestrators — [[orchestratorBudgetMs]]. */
export function orchestratorBudgetOpts(): RunCmdOpts {
  return { budgetMs: orchestratorBudgetMs() };
}

/** ⏱️[[RunCmdOpts]] preset for dev servers and long-lived daemons — [[daemonBudgetMs]]. */
export function daemonBudgetOpts(): RunCmdOpts {
  return { budgetMs: daemonBudgetMs() };
}

/** ⏱️Shared `spawnSync` core for [[runCmd]]/[[runCmdStatus]]: throws on spawn error, budget timeout, or signal kill (printing `[budget]` first on timeout); otherwise returns the exit status. */
function runCmdInternal(cmd: string, args: string[], opts: RunCmdOpts): number {
  const budgetMs = opts.budgetMs ?? defaultBudgetMs(cmd);
  const formattedArgs = [...args];
  if (cmd === "bun" || cmd === process.execPath) {
    if (formattedArgs[0] && !formattedArgs[0].startsWith("-") && !formattedArgs[0].includes("/") && !formattedArgs[0].includes("\\")) {
      const resolved = resolveWorkspaceBin(formattedArgs[0], opts.cwd ?? process.cwd());
      if (resolved) {
        formattedArgs[0] = resolved;
      }
    }
  }
  const result = spawnSync(cmd, formattedArgs, {
    stdio: "inherit",
    cwd: opts.cwd,
    env: opts.env ?? process.env,
    timeout: budgetMs,
    killSignal: "SIGKILL",
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

/** 🏃️Like [[runCmd]] but ignores failures — including a budget kill, which is the desired never-hang behavior for optional commands. */
export function tryRun(cmd: string, args: string[], opts: RunCmdOpts = {}): void {
  try {
    runCmd(cmd, args, opts);
  } catch {
    /* optional */
  }
}

/** 🔍️ Resolves a CLI executable in `cwd` or workspace root's `node_modules/.bin` to avoid `bun x` cwd resolution bugs on emoji/ZWJ paths. */
export function resolveWorkspaceBin(binName: string, cwd: string = process.cwd()): string | null {
  const shortName = binName.includes("/") ? binName.split("/").pop()! : binName;
  const localBin = join(cwd, "node_modules", ".bin", shortName);
  if (existsSync(localBin)) return localBin;
  const rootBin = join(getWorkspaceRoot(), "node_modules", ".bin", shortName);
  if (existsSync(rootBin)) return rootBin;
  return null;
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

/** @emoji 🚦️ Whether child builds should use fast dev artifacts or ship optimization. */
export type SemioBuildMode = "dev" | "ship";

/** @emoji 🚦️ `ship` only when `SEMIO_BUILD_MODE=ship`; default is dev for local/agent loops. */
export function semioBuildMode(): SemioBuildMode {
  return process.env.SEMIO_BUILD_MODE === "ship" ? "ship" : "dev";
}

/** @emoji 🚀️ Env for nx/build orchestrators so spawned crate `wasm` scripts inherit ship mode. */
export function semioShipEnv(): NodeJS.ProcessEnv {
  return { ...process.env, SEMIO_BUILD_MODE: "ship" };
}

/** @emoji 📂 Cargo output directory name for a profile (`dev` → `debug`). */
export function cargoProfileDir(profile: string): string {
  return profile === "dev" ? "debug" : profile;
}
