/** 🏗️ Builds the proctor under original compiler authority and retains immutable executable custody through every child.
 * The digest copy stays inside the bounded compiler image, so Windows never locks Cargo's mutable executable.
 * {@link buildProctor} returns an owner that callers close after all consumers, including restarts, have drained.
 * {@link launchProctor} stages an explicitly supplied external executable for diagnostic runs.
 *
 * @see ./🦀️.rs — the process entry of the binary
 * @see ../../🏛️architecture/📚️course/🧱️stack/🟦️.ts — the local stack of proctor and site
 * @see ../README.md — the environment and the commands
 * @see https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages */
import { spawn } from "node:child_process";
import { createHash, randomUUID } from "node:crypto";
import { open } from "node:fs/promises";
import { chmodSync, closeSync, copyFileSync, existsSync, lstatSync, mkdirSync, openSync, readdirSync, renameSync, rmSync } from "node:fs";
import { constants } from "node:os";
import { basename, dirname, join, resolve, sep } from "node:path";
import { terminateOwnedChildTree } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptInvocationEnvironment, requireIssuedScriptInvocationV1, type ScriptInvocation } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import { buildModeEnvironment, type BuildMode } from "../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { buildCargoArtifacts, withCapturedCargoArtifactsTaskV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { requireNativeStoragePathV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import { pinExecutableArtifact } from "../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts";
import { withResourceLeases, checkResourceLeaseControl } from "../../../🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { withRepositoryCompilerStoreV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import { repositoryCargoArtifactBuildPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { repositoryCargoPreparationStorageV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { cargoWorkspacePreparationInvocationV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import { repositoryCargoNetworkV1, repositoryCargoCompilerInputsV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import buildSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import { cargoPackageClosure, type ManifestClosureOptions } from "./🕸️manifest-closure/🟦️.ts";

/** 📦️ The crate that holds the binary. */
export const PROCTOR_PACKAGE = "teaching-proctor";
/** 🧱️ The library crates below the proctor, whose unit laws run with the proctor's own: the base (storage, catalog, config, crowd, presence) and the domain (deciders, projections, queries). */
export const PROCTOR_LIBRARY_PACKAGES = ["teaching-proctor-base", "teaching-proctor-domain"];

/** 🎓️ The Cargo workspace of the teaching products, relative to the repository root: the proctor is a member of it, not of the root workspace. */
export const TEACHING_WORKSPACE = "🎓️teaching";

/** 🔌️ The port a dev proctor listens on unless `PROCTOR_PORT` says otherwise (design §12). */
export const PROCTOR_DEV_PORT = "8791";

/** 🗃️ Where a launcher keeps the private copies it runs, relative to the repository root. */
export const PROCTOR_COPY_DIRECTORY = [".🧬semio", "🎓️teaching", "proctor-bin"] as const;

/** 📚️ The course the dev site serves, relative to the repository root. */
export const PROCTOR_DEV_COURSE = ["🎓️teaching", "🏛️architecture", "📚️course", "🔣️.json"] as const;

/** 🧺️ The data directory a dev proctor keeps unless `PROCTOR_DATA` names another, relative to the repository root
 * (git-ignored): the launcher's own, disposable development data. */
export const PROCTOR_DEV_DATA_DIRECTORY = [".🧬semio", "🎓️teaching", "proctor-dev"] as const;

/** 🗺️ The environment of a dev proctor: every `PROCTOR_*` the launcher set wins, the rest defaults to the dev port, the
 * git-ignored `.🧬semio/🎓️teaching/proctor-dev/` data directory and the architecture course. */
export function proctorDevelopmentEnvironment(repoRoot: string, env: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  return {
    ...env,
    PROCTOR_PORT: env.PROCTOR_PORT ?? PROCTOR_DEV_PORT,
    PROCTOR_DATA: resolve(env.PROCTOR_DATA ?? join(repoRoot, ...PROCTOR_DEV_DATA_DIRECTORY)),
    PROCTOR_COURSE: resolve(env.PROCTOR_COURSE ?? join(repoRoot, ...PROCTOR_DEV_COURSE)),
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

/** 📚️ The course a `check` validates: the named one, else `PROCTOR_COURSE`, else the dev course. */
export function proctorCourse(repoRoot: string, named?: string, env: NodeJS.ProcessEnv = process.env): string {
  return resolve(named ?? env.PROCTOR_COURSE ?? join(repoRoot, ...PROCTOR_DEV_COURSE));
}

/** 🎼️ Admits the closed portable selection of the proctor's native compiler target. */
export function proctorBuildPlan(value: unknown): { readonly mode: BuildMode; readonly arguments: readonly string[] } {
  if (validateJsonSchemaSubset(buildSchema, value).length) throw Error("Closed proctor build intent required");
  const { mode } = value as { mode: BuildMode };
  return Object.freeze({ mode, arguments: Object.freeze(["--package", PROCTOR_PACKAGE, "--bin", "proctor", "--profile", mode]) });
}

/** 🎚️ Preserves the original caller's mode and refuses an attempted cross-mode build. */
export function proctorBuildMode(value: unknown): BuildMode {
  if (validateJsonSchemaSubset(buildSchema.definitions.ModeSelection, value).length) throw Error("Original proctor build mode required");
  return (value as { readonly originalMode: BuildMode }).originalMode;
}

/** 📤️ A bounded executable byte reader whose lifetime belongs to its retained artifact callback. */
export interface ProctorExecutable {
  readonly platform: NodeJS.Platform;
  readonly architecture: string;
  readonly mode: BuildMode;
  readonly bytes: number;
  readonly signal: AbortSignal;
  recheck(): Promise<void>;
  chunks(): AsyncGenerator<Uint8Array>;
}

/** 🧊️ An immutable executable owner retained through every child and bounded export callback. */
export interface ProctorArtifact {
  readonly sources: readonly string[];
  launch(args: readonly string[], env?: NodeJS.ProcessEnv, log?: string): ProctorProcess;
  withExecutable<T>(consume: (executable: ProctorExecutable) => Promise<T>): Promise<T>;
  close(): Promise<void>;
}

/** 🏗️ Builds with the original caller, then retains the charged compiler image and immutable reader for all consumers. The course obtains the proctor through `./🧾️product/🟦️.ts` instead, which resolves the cached product of `@teaching/proctor:build`. */
export async function buildProctor(repoRoot: string, invocation: ScriptInvocation, env: NodeJS.ProcessEnv = process.env, signal?: AbortSignal, requestedMode?: BuildMode): Promise<ProctorArtifact> {
  requireIssuedScriptInvocationV1(invocation);
  const authority = readNativeCallerAuthorityV1(invocation.capabilities), mode = proctorBuildMode({ version: 1, originalMode: authority.original.buildMode, ...(requestedMode === undefined ? {} : { requestedMode }) });
  const plan = proctorBuildPlan({ version: 1, mode }), root = resolve(repoRoot), manifest = join(root, TEACHING_WORKSPACE, "🛂️proctor", "📦️packages", "🦀️rust", "Cargo.toml");
  const control = scriptCompilerAdmissionControlV1(invocation, "proctor:build", 0, signal), environment = scriptInvocationEnvironment(invocation, buildModeEnvironment(mode, env));
  const network = repositoryCargoNetworkV1(invocation, root), policy = repositoryCargoArtifactBuildPolicyV1(root, root, environment), leaseDirectory = policy.leaseDirectory;
  let publish!: (artifact: ProctorArtifact) => void, fail!: (error: unknown) => void, release!: () => void;
  const ready = new Promise<ProctorArtifact>((resolve, reject) => { publish = resolve; fail = reject; });
  const released = new Promise<void>(resolve => { release = resolve; });
  const lifetime = withRepositoryCompilerStoreV1(root, mode, control, environment, async (store, control) => {
    const output = requireNativeStoragePathV1(store.root, join(store.root, "proctor-build", randomUUID())), args = [...plan.arguments];
    const pinned = requireNativeStoragePathV1(store.root, join(store.root, "proctor-cli", mode, basename(output)));
    try {
      const preparedEnvironment = { ...store.environment, CARGO_NET_OFFLINE: String(network.offline) }, preparation = cargoWorkspacePreparationInvocationV1({ ...repositoryCargoPreparationStorageV1(root), directory: requireNativeStoragePathV1(store.root, join(store.buildDirectory, "preparation")) }, root, ["build", "--manifest-path", manifest, ...args], root, preparedEnvironment);
      if (preparation) {
        await store.storage.recheck();
        const diagnostics = await captureOwnedCommandV1(preparation.command, [...preparation.args], preparation.cwd, { control, env: preparation.environment, output: { maximumBytes: 16777216, maximumLines: 65536 } });
        if (diagnostics) process.stderr.write(diagnostics);
        await store.storage.recheck();
      }
      const sources = await proctorSourceDirectories(root, { signal: control.signal });
      let executable: string | undefined;
      try {
        await buildCargoArtifacts(manifest, args, { ...policy, buildDirectory: store.buildDirectory, captureDirectory: store.captureDirectory }, {
          control, storage: store.storage, sourceBoundary: store.sourceBoundary, environment: preparedEnvironment, output,
          maximumObservationBytes: Math.min(CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1, authority.transport.maximumBytes),
          withCargoInputs: repositoryCargoCompilerInputsV1({ invocation, consumption: { kind: "compiler", producer: "artifact", operation: "build" }, workspace: root, network, control, storage: store.storage }),
          consume: capture => withCapturedCargoArtifactsTaskV1(capture, async view => {
            const source = view.files.get(process.platform === "win32" ? "proctor.exe" : "proctor");
            if (!source) throw Error("Completed proctor executable required");
            await view.recheck();
            executable = await pinExecutableArtifact(source, pinned, "teaching:proctor:" + mode, { control: view.control, signal: view.control.signal, leaseDirectory });
          }),
        });
      } finally {
        rmSync(output, { recursive: true, force: true });
      }
      if (!executable) throw Error("Retained proctor executable required");
      const built = { executable, sources };
      await withResourceLeases({ directory: leaseDirectory, control, signal: control.signal, resources: [{ resource: "artifact:" + dirname(built.executable), mode: "shared" }] }, async () => {
        await store.storage.recheck();
        const children = new Set<ProctorProcess>();
        const consumers = new Set<Promise<unknown>>();
        let closing: Promise<void> | undefined;
        const artifact: ProctorArtifact = Object.freeze({ sources: Object.freeze(built.sources), launch: (args: readonly string[], childEnvironment = env, log?: string) => {
          if (closing) throw Error("Proctor artifact is closing");
          checkResourceLeaseControl(control);
          const child = launchProctorProcess(built.executable, root, args, scriptInvocationEnvironment(invocation, childEnvironment), log);
          children.add(child);
          void child.exited.then(() => children.delete(child), () => children.delete(child));
          return child;
        }, withExecutable: <T>(consume: (executable: ProctorExecutable) => Promise<T>): Promise<T> => {
          if (closing) return Promise.reject(Error("Proctor artifact is closing"));
          const operation = (async () => {
            checkResourceLeaseControl(control); await store.storage.recheck();
            const facts = lstatSync(built.executable), readers = new Set<AsyncGenerator<Uint8Array>>();
            let active = true;
            const recheck = async (): Promise<void> => { if (!active) throw Error("Proctor executable export retired"); checkResourceLeaseControl(control); await store.storage.recheck(); };
            const executable: ProctorExecutable = Object.freeze({ platform: process.platform, architecture: process.arch, mode, bytes: facts.size, signal: control.signal, recheck, chunks: () => {
              const reader = (async function* (): AsyncGenerator<Uint8Array> {
                await recheck();
                const file = await open(built.executable, "r");
                try {
                  const digest = createHash("sha256"), before = await file.stat();
                  if (before.dev !== facts.dev || before.ino !== facts.ino || before.size !== facts.size) throw Error("Proctor executable changed before export");
                  let bytes = 0, chunks = 0;
                  for (;;) { if (!active) throw Error("Proctor executable export retired"); checkResourceLeaseControl(control); const chunk = Buffer.allocUnsafe(65536), row = await file.read(chunk, 0, chunk.length, null); if (!row.bytesRead) break; bytes += row.bytesRead; if (bytes > facts.size) throw Error("Proctor executable exceeded its retained size"); const output = chunk.subarray(0, row.bytesRead); digest.update(output); yield output; if (++chunks % 64 === 0) { await control.publish({ resource: "proctor:executable-export", mode: "shared", elapsedMs: 0 }); await control.yieldContinuation(); } }
                  if (bytes !== facts.size || digest.digest("hex") !== basename(dirname(built.executable))) throw Error("Proctor executable bytes changed during export");
                  await recheck();
                } finally { await file.close(); }
              })();
              readers.add(reader); return reader;
            } });
            try { const result = await consume(executable); await recheck(); return result; }
            finally { active = false; const closed = await Promise.allSettled([...readers].map(reader => reader.return(undefined))); const failures = closed.filter(row => row.status === "rejected").map(row => row.reason); if (failures.length) throw new AggregateError(failures, "Proctor executable exports did not close cleanly"); }
          })();
          consumers.add(operation); void operation.then(() => consumers.delete(operation), () => consumers.delete(operation));
          return operation;
        }, close: () => closing ??= (async () => {
          const outcomes = await Promise.allSettled([...[...children].map(child => child.stop()), ...consumers]);
          release();
          await lifetime;
          const failures = outcomes.filter(row => row.status === "rejected").map(row => row.reason);
          if (failures.length) throw new AggregateError(failures, "Proctor processes did not settle cleanly");
        })() });
        const cancel = (): void => { void artifact.close().catch(() => undefined); };
        const remaining = checkResourceLeaseControl(control), deadline = remaining === null ? undefined : setTimeout(cancel, Math.min(remaining, 2147483647));
        control.signal.addEventListener("abort", cancel, { once: true });
        try {
          publish(artifact);
          if (control.signal.aborted) cancel();
          await released;
          await store.storage.recheck();
        } finally { clearTimeout(deadline); control.signal.removeEventListener("abort", cancel); }
      });
    } finally { rmSync(requireNativeStoragePathV1(store.root, pinned), { recursive: true, force: true }); }
  });
  void lifetime.catch(fail);
  return ready;
}

/** 🗂️ The directory a package's sources live in: the owner whose `📦️packages/<language>` folder holds the manifest
 * (the repository's taxonomy keeps sources beside the owner, not beside the manifest), else the manifest's own. */
function packageRoot(manifest: string): string {
  const directory = dirname(manifest);
  return basename(dirname(directory)) === "📦️packages" ? dirname(dirname(directory)) : directory;
}

/** 🧾️ The Cargo metadata fields needed to project the local normal and build dependency owners. */
interface CargoPackage {
  readonly id: string;
  readonly name: string;
  readonly source: string | null;
  readonly manifest_path: string;
}

export interface CargoMetadata {
  readonly packages: readonly CargoPackage[];
  readonly resolve: { readonly nodes: readonly { readonly id: string; readonly deps: readonly { readonly pkg: string; readonly dep_kinds: readonly { readonly kind: string | null }[] }[] }[] };
}

/** 🗂️ Projects local normal and build owners from an already prepared Cargo dependency graph. */
export function proctorSourceRoots(metadata: CargoMetadata): string[] {
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

/** 🗂️ Reads the proctor's local dependency owners from the prepared manifests with cancellation and progress.
 * @see ./🕸️manifest-closure/🟦️.ts */
export async function proctorSourceDirectories(repoRoot: string, options: ManifestClosureOptions = {}): Promise<string[]> {
  const crates = await cargoPackageClosure(join(repoRoot, TEACHING_WORKSPACE, "Cargo.toml"), PROCTOR_PACKAGE, options);
  const roots = crates.map((crate) => packageRoot(crate.manifest)).sort();
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
  return launchProctorProcess(copy, repoRoot, args, env, log, () => rmSync(copy, { force: true, maxRetries: 20, retryDelay: 100 }));
}

/** 🧵️ Drains a runtime process before releasing its executable or log descriptor. */
function launchProctorProcess(executable: string, repoRoot: string, args: readonly string[], env: NodeJS.ProcessEnv, log?: string, cleanup?: () => void): ProctorProcess {
  const sink = log === undefined ? undefined : openSync(log, "a");
  let child: ReturnType<typeof spawn>;
  try {
    child = spawn(executable, [...args], { cwd: repoRoot, env, stdio: sink === undefined ? "inherit" : ["ignore", sink, sink], windowsHide: true });
  } catch (error) {
    if (sink !== undefined) closeSync(sink);
    cleanup?.();
    throw error;
  }
  const exited = new Promise<number>((accept) => {
    child.once("error", (error) => {
      console.error(`[ERROR] proctor ${args.join(" ")}: ${error.message}`);
    });
    child.once("close", (code, signal) => accept(code ?? (signal ? 128 : 1)));
  }).finally(() => {
    if (sink !== undefined) closeSync(sink);
    cleanup?.();
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
