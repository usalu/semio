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
 * {@link runProctor} is the whole launcher of one command; {@link buildProctor}, {@link launchProctor} and
 * {@link proctorReady} are its parts for a caller that runs the proctor beside something else (the site's local stack).
 *
 * @see ./🦀️.rs — the process entry of the binary
 * @see ../../🏛️architecture/❓️quiz/🧱️stack/🟦️.ts — the local stack of proctor and site
 * @see ../README.md — the environment and the commands
 * @see https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages */
import { spawn } from "node:child_process";
import { randomUUID } from "node:crypto";
import { chmodSync, closeSync, copyFileSync, existsSync, mkdirSync, openSync, readdirSync, renameSync, rmSync } from "node:fs";
import { constants } from "node:os";
import { basename, dirname, join, resolve, sep } from "node:path";
import { terminateOwnedChildTree } from "../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { startNativeProgress } from "../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";

/** 📦️ The crate that holds the binary. */
export const PROCTOR_PACKAGE = "teaching-proctor";

/** 🎓️ The Cargo workspace of the teaching products, relative to the repository root: the proctor is a member of it, not of the root workspace. */
export const TEACHING_WORKSPACE = "🎓️teaching";

/** 🔌️ The port a dev proctor listens on unless `PROCTOR_PORT` says otherwise (design §12). */
export const PROCTOR_DEV_PORT = "8791";

/** 🗃️ Where a launcher keeps the private copies it runs, relative to the repository root. */
export const PROCTOR_COPY_DIRECTORY = [".🧬semio", "🎓️teaching", "proctor-bin"] as const;

/** 📚️ The catalog the dev site serves, relative to the repository root. */
export const PROCTOR_DEV_CATALOG = ["🎓️teaching", "🏛️architecture", "❓️quiz", "🔣️.json"] as const;

/** 🧺️ The data directory a dev proctor keeps unless `PROCTOR_DATA` names another, relative to the repository root
 * (git-ignored): the launcher's own, disposable development data. */
export const PROCTOR_DEV_DATA_DIRECTORY = [".🧬semio", "🎓️teaching", "proctor-dev"] as const;

/** 🗺️ The environment of a dev proctor: every `PROCTOR_*` the launcher set wins, the rest defaults to the dev port, the
 * git-ignored `.🧬semio/🎓️teaching/proctor-dev/` data directory and the architecture catalog. */
export function proctorDevelopmentEnvironment(repoRoot: string, env: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  return {
    ...env,
    PROCTOR_PORT: env.PROCTOR_PORT ?? PROCTOR_DEV_PORT,
    PROCTOR_DATA: resolve(env.PROCTOR_DATA ?? join(repoRoot, ...PROCTOR_DEV_DATA_DIRECTORY)),
    PROCTOR_CATALOG: resolve(env.PROCTOR_CATALOG ?? join(repoRoot, ...PROCTOR_DEV_CATALOG)),
  };
}

/** 🗄️ The one file of a data directory that holds all state.
 * @see ../🔨️modules/🗄️storage/🦀️.rs — `DATABASE_FILE` */
export const PROCTOR_DATABASE_FILE = "proctor.sqlite";

/** 🔖️ A storage format: what the format row of a database names. */
export interface ProctorStorageFormat {
  readonly schema: string;
  readonly version: number;
}

/** 🏷️ The storage format the proctor writes and accepts; a database of any other is refused at open, by design without
 * a migration.
 * @see ../🔨️modules/🗄️storage/🦀️.rs — `FORMAT_SCHEMA`, `FORMAT_VERSION` */
export const PROCTOR_STORAGE_FORMAT: ProctorStorageFormat = { schema: "semio.teaching.proctor.sqlite", version: 3 };

const FORMAT_ROW = "SELECT schema, version FROM proctor_format WHERE singleton = 1";

/** 📖️ The first row `query` answers from the SQLite file `file`, opened read-only with the SQLite the runtime ships —
 * `bun:sqlite` under Bun (every version the repository admits; `node:sqlite` only arrived in Bun 1.4), `node:sqlite`
 * under Node (the coverage runs). The statement is finalized and the connection closed before it returns, so no handle
 * outlives it: Windows moves no folder that holds an open file.
 * @see https://bun.com/docs/api/sqlite
 * @see https://nodejs.org/api/sqlite.html */
async function firstRow(file: string, query: string): Promise<unknown> {
  if (process.versions.bun !== undefined) {
    const { Database } = await import("bun:sqlite");
    const database = new Database(file, { readonly: true });
    try {
      const statement = database.query(query);
      try {
        return statement.get();
      } finally {
        statement.finalize();
      }
    } finally {
      database.close();
    }
  }
  const { DatabaseSync } = await import("node:sqlite");
  const database = new DatabaseSync(file, { readOnly: true });
  try {
    return database.prepare(query).get();
  } finally {
    database.close();
  }
}

/** 🧾️ The storage format the database in the data directory `data` is stamped with; none when there is no database, when
 * it has no format row, when it is no SQLite file, or when a proctor holds it. The database is opened read-only, through
 * its write-ahead log like any reader (a young database a proctor was killed over still keeps its format row there), so
 * nothing of the data changes; SQLite may leave the log's two files beside it. */
export async function storedProctorFormat(data: string): Promise<ProctorStorageFormat | undefined> {
  const file = join(data, PROCTOR_DATABASE_FILE);
  if (!existsSync(file)) return undefined;
  try {
    const row = (await firstRow(file, FORMAT_ROW)) as { readonly schema?: unknown; readonly version?: unknown } | null | undefined;
    return typeof row?.schema === "string" && typeof row.version === "number" ? { schema: row.schema, version: row.version } : undefined;
  } catch {
    return undefined;
  }
}

/** 🚧️ Development data the dev launcher does not start over; the message is the one line that says how to go on. */
export class DevelopmentDataRefused extends Error {
  constructor(message: string) {
    super(message);
    this.name = "DevelopmentDataRefused";
  }
}

/** 🧹️ Makes sure a dev proctor can open the data directory of `env` (as {@link proctorDevelopmentEnvironment} made it)
 * before anything is built: data stamped with another storage format than {@link PROCTOR_STORAGE_FORMAT} would be
 * refused at open. The launcher's own folder is disposable, so it is moved aside to a sibling named by the format and
 * the time `at`, the proctor starts with empty data, and the one line to tell the developer is returned. A folder the
 * developer named with `PROCTOR_DATA` is never touched: that fails with {@link DevelopmentDataRefused}. This is the
 * dev launcher's convenience only — the proctor itself refuses such a file in every mode. */
export async function settleDevelopmentData(repoRoot: string, env: NodeJS.ProcessEnv, at: Date = new Date()): Promise<string | undefined> {
  const data = resolve(env.PROCTOR_DATA ?? join(repoRoot, ...PROCTOR_DEV_DATA_DIRECTORY));
  const own = resolve(repoRoot, ...PROCTOR_DEV_DATA_DIRECTORY);
  const stored = await storedProctorFormat(data);
  if (stored === undefined || (stored.schema === PROCTOR_STORAGE_FORMAT.schema && stored.version === PROCTOR_STORAGE_FORMAT.version)) return undefined;
  const age = stored.schema !== PROCTOR_STORAGE_FORMAT.schema ? "another" : stored.version < PROCTOR_STORAGE_FORMAT.version ? "an older" : "a newer";
  const format = `in ${age} storage format (${stored.schema} v${stored.version}; this proctor reads v${PROCTOR_STORAGE_FORMAT.version}, and there is no migration)`;
  if (data !== own) throw new DevelopmentDataRefused(`${data} holds proctor data ${format}. PROCTOR_DATA names that folder, so it is left as it is: delete or move it, or unset PROCTOR_DATA to use the launcher's own disposable folder ${own}.`);
  const base = `${data}.v${stored.version}-${at.toISOString().replace(/[-:]|\.\d+Z$/gu, "")}`;
  let aside = base;
  for (let attempt = 1; existsSync(aside); attempt++) aside = `${base}-${attempt}`;
  try {
    renameSync(data, aside);
  } catch (error) {
    throw new DevelopmentDataRefused(`${data} holds disposable development data ${format}, and it could not be moved aside (${error instanceof Error ? error.message : String(error)}). Stop the proctor that still serves it, or delete the folder, and run the command again.`);
  }
  return `${data} held disposable development data ${format}. It was moved aside to ${aside} and the proctor starts with empty data. Delete ${aside} when you do not need it; to reset development data yourself at any time, stop the proctor and delete ${data}.`;
}

/** 🚦️ {@link settleDevelopmentData} for a dev command: says through `say` what was set aside and answers whether the
 * proctor may start. A refusal is said as its one line — no stack trace — and ends the command with status 1. */
export async function developmentDataSettled(repoRoot: string, env: NodeJS.ProcessEnv, say: (line: string) => void, at: Date = new Date()): Promise<boolean> {
  try {
    const settled = await settleDevelopmentData(repoRoot, env, at);
    if (settled !== undefined) say(settled);
    return true;
  } catch (error) {
    if (!(error instanceof DevelopmentDataRefused)) throw error;
    say(error.message);
    process.exitCode = 1;
    return false;
  }
}

/** 💾️ Where a dev `backup` writes unless it is told where, relative to the repository root (git-ignored). */
export const PROCTOR_BACKUP_DIRECTORY = [".🧬semio", "🎓️teaching", "proctor-backups"] as const;

/** 📸️ What a `backup` is given: the named target as typed (`-`, a file, or a directory ending in a separator), else the
 * dev backup directory — with the trailing separator that makes the proctor create it and name the file by its time. */
export function proctorBackupTarget(repoRoot: string, named?: string): string {
  return named ?? `${join(repoRoot, ...PROCTOR_BACKUP_DIRECTORY)}/`;
}

/** 🎯️ The arguments of an `erase` or a `prune` as the proctor takes them, from what a task runner forwards:
 * `--handle=Ada` is `--handle Ada` and `--older-than=7d` is `--older-than 7d`, a handle a shell split into words is one
 * argument again (a handle's inner whitespace is one space anyway), and `--dry-run` is the bare flag whatever value it
 * was given. */
export function proctorSelection(forwarded: readonly string[]): string[] {
  const selection: string[] = [];
  let value: string[] | undefined;
  const close = (): void => {
    if (value !== undefined && value.length > 0) selection.push(value.join(" "));
    value = undefined;
  };
  for (const segment of forwarded) {
    const flag = /^(--[a-z-]+)(?:=(.*))?$/su.exec(segment);
    if (!flag) {
      if (value === undefined) selection.push(segment);
      else value.push(segment);
      continue;
    }
    close();
    selection.push(flag[1]!);
    if (flag[1] === "--dry-run") continue;
    value = flag[2] === undefined ? [] : [flag[2]];
  }
  close();
  return selection;
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

/** 🏗️ Build the binary and return the path Cargo uplifted it to: diagnostics go to the terminal, a progress line follows
 * every ten seconds (a first build takes minutes), and an aborted `signal` ends the whole cargo tree. `profile` is
 * Cargo's `dev` build (what the dev launcher runs) or its `release` build (what is deployed and what capacity is
 * measured on). */
export async function buildProctor(repoRoot: string, env: NodeJS.ProcessEnv = process.env, signal?: AbortSignal, profile: "dev" | "release" = "dev"): Promise<string> {
  if (signal?.aborted) throw new Error(`cargo build of ${PROCTOR_PACKAGE} cancelled`);
  const child = spawn("cargo", ["build", "--package", PROCTOR_PACKAGE, "--bin", "proctor", ...(profile === "release" ? ["--release"] : []), "--message-format=json-render-diagnostics"], { cwd: join(repoRoot, TEACHING_WORKSPACE), env, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32", windowsHide: true });
  let messages = "";
  child.stdout!.setEncoding("utf8").on("data", (chunk: string) => (messages += chunk));
  child.stderr!.pipe(process.stderr, { end: false });
  const cancel = (): void => terminateOwnedChildTree(child);
  signal?.addEventListener("abort", cancel, { once: true });
  const stopProgress = startNativeProgress("proctor build");
  try {
    const status = await new Promise<number | string>((accept) => {
      child.once("error", (error) => accept(error.message));
      child.once("close", (code, ended) => accept(code ?? ended ?? 1));
    });
    if (signal?.aborted) throw new Error(`cargo build of ${PROCTOR_PACKAGE} cancelled`);
    if (status !== 0) throw new Error(`cargo build of ${PROCTOR_PACKAGE} failed (${status})`);
  } finally {
    stopProgress();
    signal?.removeEventListener("abort", cancel);
  }
  const executable = proctorExecutable(messages);
  if (!executable) throw new Error(`cargo build of ${PROCTOR_PACKAGE} reported no proctor executable`);
  return executable;
}

/** 📦️ One package as `cargo metadata` describes it. */
interface CargoPackage {
  readonly id: string;
  readonly name: string;
  readonly source: string | null;
  readonly manifest_path: string;
}

/** 🕸️ What `cargo metadata` says of a workspace: its packages and their resolved dependencies, by kind. */
interface CargoMetadata {
  readonly packages: readonly CargoPackage[];
  readonly resolve: { readonly nodes: readonly { readonly id: string; readonly deps: readonly { readonly pkg: string; readonly dep_kinds: readonly { readonly kind: string | null }[] }[] }[] };
}

/** 🗂️ The directory a package's sources live in: the owner whose `📦️packages/<language>` folder holds the manifest
 * (the repository's taxonomy keeps sources beside the owner, not beside the manifest), else the manifest's own. */
function packageRoot(manifest: string): string {
  const directory = dirname(manifest);
  return basename(dirname(directory)) === "📦️packages" ? dirname(dirname(directory)) : directory;
}

/** 🗂️ The directories the `proctor` binary is built from: the root of every package of this repository it depends on
 * to build (as `cargo metadata` resolves the teaching workspace offline), a directory inside another left out. */
export async function proctorSourceDirectories(repoRoot: string, env: NodeJS.ProcessEnv = process.env): Promise<string[]> {
  const child = spawn("cargo", ["metadata", "--format-version", "1", "--offline"], { cwd: join(repoRoot, TEACHING_WORKSPACE), env, stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
  let output = "";
  let failure = "";
  child.stdout!.setEncoding("utf8").on("data", (chunk: string) => (output += chunk));
  child.stderr!.setEncoding("utf8").on("data", (chunk: string) => (failure += chunk));
  const status = await new Promise<number | string>((accept) => {
    child.once("error", (error) => accept(error.message));
    child.once("close", (code, ended) => accept(code ?? ended ?? 1));
  });
  if (status !== 0) throw new Error(`cargo metadata of ${TEACHING_WORKSPACE} failed (${status}): ${failure.trim().split(/\r?\n/u).at(-1) ?? ""}`);
  const metadata = JSON.parse(output) as CargoMetadata;
  const packages = new Map(metadata.packages.map((cargoPackage) => [cargoPackage.id, cargoPackage]));
  const nodes = new Map(metadata.resolve.nodes.map((node) => [node.id, node]));
  const pending = metadata.packages.filter((cargoPackage) => cargoPackage.name === PROCTOR_PACKAGE).map((cargoPackage) => cargoPackage.id);
  const reached = new Set<string>();
  for (let id = pending.pop(); id !== undefined; id = pending.pop()) {
    if (reached.has(id)) continue;
    reached.add(id);
    for (const dep of nodes.get(id)?.deps ?? []) if (dep.dep_kinds.some((kind) => kind.kind !== "dev")) pending.push(dep.pkg);
  }
  const roots = [...reached].flatMap((id) => (packages.get(id)?.source === null ? [packageRoot(packages.get(id)!.manifest_path)] : [])).sort();
  return roots.filter((root, index) => !roots.slice(0, index).some((outer) => root.startsWith(`${outer}${sep}`) || root === outer));
}

/** 🗂️ Whether a changed file — `file` relative to a watched source directory — is one a build of the proctor reads: Rust
 * sources and manifests, outside build output and installed packages. */
export function proctorSourceChanged(file: string): boolean {
  const parts = file.split(/[\\/]/u);
  return (file.endsWith(".rs") || parts.at(-1) === "Cargo.toml") && !parts.some((part) => part === "target" || part === "node_modules" || part === "dist");
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

/** 🩺️ The route a listening proctor answers `200` on with its instance id (the image's health check asks the same). */
export const PROCTOR_READY_ROUTE = "/instance";

/** ⏳️ How long a proctor that was asked to stop may take before its process is ended. */
export const PROCTOR_STOP_GRACE_MS = 5_000;

/** 🌐️ The loopback origin the proctor of `env` listens on. */
export function proctorOrigin(env: NodeJS.ProcessEnv = process.env): string {
  return `http://127.0.0.1:${env.PROCTOR_PORT ?? PROCTOR_DEV_PORT}`;
}

/** 🩺️ Whether a proctor answers ready at `origin` within `timeoutMs`. */
export async function proctorReady(origin: string, timeoutMs = 2_000): Promise<boolean> {
  try {
    const answer = await fetch(`${origin}${PROCTOR_READY_ROUTE}`, { signal: AbortSignal.timeout(timeoutMs) });
    return answer.ok && (await answer.text()).includes(PROCTOR_PACKAGE);
  } catch {
    return false;
  }
}

/** 🧵️ A proctor this process launched: `exited` settles with its exit status once it ended and its private copy is
 * deleted; `interrupt` forwards an interrupt or termination where POSIX signals exist (on Windows the console delivers
 * Ctrl+C to the proctor itself); `stop` asks it to stop, waits up to `graceMs` and then ends its process. */
export interface ProctorProcess {
  readonly pid: number | undefined;
  readonly exited: Promise<number>;
  interrupt(signal: NodeJS.Signals): void;
  stop(graceMs?: number): Promise<number>;
}

/** 🚀️ Run `proctor <args…>` from a private copy of `executable` with `env`: its output shares the terminal, or is
 * appended to the file `log`. */
export function launchProctor(executable: string, repoRoot: string, args: readonly string[], env: NodeJS.ProcessEnv = process.env, log?: string): ProctorProcess {
  const copy = stagePrivateCopy(executable, join(repoRoot, ...PROCTOR_COPY_DIRECTORY), `${process.pid}-${randomUUID()}`);
  const sink = log === undefined ? undefined : openSync(log, "a");
  const child = spawn(copy, [...args], { cwd: repoRoot, env, stdio: sink === undefined ? "inherit" : ["ignore", sink, sink], windowsHide: true });
  const exited = new Promise<number>((accept) => {
    child.once("error", (error) => {
      console.error(`[ERROR] proctor ${args.join(" ")}: ${error.message}`);
      accept(1);
    });
    child.once("close", (code, signal) => accept(code ?? (signal ? 128 : 1)));
  }).finally(() => {
    if (sink !== undefined) closeSync(sink);
    rmSync(copy, { force: true, maxRetries: 20, retryDelay: 100 });
  });
  const interrupt = (signal: NodeJS.Signals): void => {
    if (process.platform !== "win32") child.kill(signal === "SIGINT" ? "SIGINT" : "SIGTERM");
  };
  return {
    pid: child.pid,
    exited,
    interrupt,
    stop: async (graceMs = process.platform === "win32" ? 0 : PROCTOR_STOP_GRACE_MS) => {
      interrupt("SIGTERM");
      let grace: ReturnType<typeof setTimeout> | undefined;
      await Promise.race([exited, new Promise<void>((accept) => (grace = setTimeout(accept, graceMs)))]);
      clearTimeout(grace);
      terminateOwnedChildTree(child);
      return exited;
    },
  };
}

/** ▶️ Build the binary, run `proctor <args…>` from a private copy with `env`, wait for it and delete the copy; a
 * non-zero exit is an error naming the command. An interrupt during the build cancels it. */
export async function runProctor(repoRoot: string, args: readonly string[], env: NodeJS.ProcessEnv = process.env): Promise<void> {
  const interruption = new AbortController();
  let proctor: ProctorProcess | undefined;
  const forward = (signal: NodeJS.Signals): void => {
    interruption.abort();
    proctor?.interrupt(signal);
  };
  for (const signal of LAUNCHER_SIGNALS) process.on(signal, forward);
  try {
    proctor = launchProctor(await buildProctor(repoRoot, env, interruption.signal), repoRoot, args, env);
    const status = await proctor.exited;
    if (status !== 0) throw new Error(`proctor ${args.join(" ")} exited with status ${status}`);
  } finally {
    for (const signal of LAUNCHER_SIGNALS) process.off(signal, forward);
  }
}

/** 🛠️ `dev` of the proctor alone: settle the development data ({@link developmentDataSettled}), saying what was set
 * aside or why it ends here, then build, serve with the dev defaults and wait like {@link runProctor}. */
export async function runDevelopmentProctor(repoRoot: string, env: NodeJS.ProcessEnv = proctorDevelopmentEnvironment(repoRoot)): Promise<void> {
  if (await developmentDataSettled(repoRoot, env, (line) => console.warn(`[proctor] ${line}`))) await runProctor(repoRoot, ["serve"], env);
}
