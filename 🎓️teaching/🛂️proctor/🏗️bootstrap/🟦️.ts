/** 🏗️ The development launcher of the proctor: build the `proctor` binary, run a private copy of it and remove the copy
 * once it exits.
 *
 * Cargo uplifts `proctor` as a hard link into the shared build directory, and Windows refuses to replace an executable
 * image while any link of it runs. A dev proctor started from Cargo's own file therefore made every later build of the
 * crate — `check`, `rebuild`, `test`, another agent's build — fail with `Access is denied`. The running process now only
 * ever holds its own copy under the git-ignored `.🧬semio/🎓️teaching/proctor-bin/`, one per run; the same code path runs
 * on Windows, macOS and Linux. The child shares the console, so Ctrl+C / Ctrl+Break / console close reach it directly
 * and it stops gracefully; the launcher outlives those signals, forwards them where POSIX signals exist, waits for the
 * child and deletes the copy (a copy left behind by a killed launcher is removed by the next run).
 *
 * @see ./🦀️.rs — the process entry of the binary
 * @see ../README.md — the environment and the commands
 * @see https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages */
import { spawn, spawnSync } from "node:child_process";
import { chmodSync, copyFileSync, mkdirSync, readdirSync, rmSync } from "node:fs";
import { constants } from "node:os";
import { join, resolve } from "node:path";

/** 📦️ The crate that holds the binary. */
export const PROCTOR_PACKAGE = "teaching-proctor";

/** 🔌️ The port a dev proctor listens on unless `PROCTOR_PORT` says otherwise (design §12). */
export const PROCTOR_DEV_PORT = "8791";

/** 🗃️ Where a launcher keeps the private copies it runs, relative to the repository root. */
export const PROCTOR_COPY_DIRECTORY = [".🧬semio", "🎓️teaching", "proctor-bin"] as const;

/** 📚️ The catalog the dev site serves, relative to the repository root. */
export const PROCTOR_DEV_CATALOG = ["🎓️teaching", "🏛️architecture", "❓️quiz", "🔣️.json"] as const;

/** 🗺️ The environment of a dev proctor: every `PROCTOR_*` the launcher set wins, the rest defaults to the dev port, the
 * git-ignored `.🧬semio/🎓️teaching/proctor-dev/` data directory and the architecture catalog. */
export function proctorDevelopmentEnvironment(repoRoot: string, env: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  return {
    ...env,
    PROCTOR_PORT: env.PROCTOR_PORT ?? PROCTOR_DEV_PORT,
    PROCTOR_DATA: resolve(env.PROCTOR_DATA ?? join(repoRoot, ".🧬semio", "🎓️teaching", "proctor-dev")),
    PROCTOR_CATALOG: resolve(env.PROCTOR_CATALOG ?? join(repoRoot, ...PROCTOR_DEV_CATALOG)),
  };
}

/** 📚️ The catalog a `check` validates: the named one, else `PROCTOR_CATALOG`, else the dev catalog. */
export function proctorCatalog(repoRoot: string, named?: string, env: NodeJS.ProcessEnv = process.env): string {
  return resolve(named ?? env.PROCTOR_CATALOG ?? join(repoRoot, ...PROCTOR_DEV_CATALOG));
}

/** 🔎️ The executable of the `proctor` binary target among Cargo's JSON messages. */
export function proctorExecutable(messages: string): string | undefined {
  for (const line of messages.split(/\r?\n/u)) {
    if (!line.startsWith("{")) continue;
    const message = JSON.parse(line) as { reason?: string; target?: { name?: string; kind?: string[] }; executable?: string | null };
    if (message.reason === "compiler-artifact" && message.target?.name === "proctor" && message.target.kind?.includes("bin") && message.executable) return message.executable;
  }
  return undefined;
}

/** 🏗️ Build the binary (diagnostics on the terminal) and return the path Cargo uplifted it to. */
export function buildProctor(repoRoot: string, env: NodeJS.ProcessEnv = process.env): string {
  const built = spawnSync("cargo", ["build", "--package", PROCTOR_PACKAGE, "--bin", "proctor", "--message-format=json-render-diagnostics"], { cwd: repoRoot, env, stdio: ["ignore", "pipe", "inherit"], encoding: "utf8", maxBuffer: 1 << 28 });
  if (built.status !== 0) throw new Error(`cargo build of ${PROCTOR_PACKAGE} failed (${built.status ?? built.signal ?? built.error?.message})`);
  const executable = proctorExecutable(built.stdout);
  if (!executable) throw new Error(`cargo build of ${PROCTOR_PACKAGE} reported no proctor executable`);
  return executable;
}

/** 📋️ Copy `executable` into `directory` under a name no other run uses, after removing every copy no process holds
 * any more (a copy still running refuses removal on Windows and is kept). */
export function stagePrivateCopy(executable: string, directory: string, runId = `${process.pid}-${Date.now()}`): string {
  mkdirSync(directory, { recursive: true });
  for (const stale of readdirSync(directory)) {
    try {
      rmSync(join(directory, stale), { force: true });
    } catch {}
  }
  const copy = join(directory, `proctor-${runId}${process.platform === "win32" ? ".exe" : ""}`);
  copyFileSync(executable, copy);
  if (process.platform !== "win32") chmodSync(copy, 0o755);
  return copy;
}

/** 🛑️ The signals a launcher outlives while its child stops — those this platform knows of interrupt, termination,
 * hang-up (console close) and Ctrl+Break. */
export const LAUNCHER_SIGNALS: readonly NodeJS.Signals[] = (["SIGINT", "SIGTERM", "SIGHUP", "SIGBREAK"] as const).filter((signal) => signal in constants.signals);

/** ▶️ Build the binary, run `proctor <args…>` from a private copy with `env`, wait for it and delete the copy; a
 * non-zero exit is an error naming the command. */
export async function runProctor(repoRoot: string, args: readonly string[], env: NodeJS.ProcessEnv = process.env): Promise<void> {
  const copy = stagePrivateCopy(buildProctor(repoRoot, env), join(repoRoot, ...PROCTOR_COPY_DIRECTORY));
  const child = spawn(copy, [...args], { cwd: repoRoot, env, stdio: "inherit" });
  const forward = (signal: NodeJS.Signals): void => {
    if (process.platform !== "win32") child.kill(signal === "SIGINT" ? "SIGINT" : "SIGTERM");
  };
  for (const signal of LAUNCHER_SIGNALS) process.on(signal, forward);
  try {
    const status = await new Promise<number>((accept) => {
      child.once("error", (error) => {
        console.error(`[ERROR] proctor ${args.join(" ")}: ${error.message}`);
        accept(1);
      });
      child.once("close", (code, signal) => accept(code ?? (signal ? 128 : 1)));
    });
    if (status !== 0) throw new Error(`proctor ${args.join(" ")} exited with status ${status}`);
  } finally {
    for (const signal of LAUNCHER_SIGNALS) process.off(signal, forward);
    rmSync(copy, { force: true, maxRetries: 20, retryDelay: 100 });
  }
}
