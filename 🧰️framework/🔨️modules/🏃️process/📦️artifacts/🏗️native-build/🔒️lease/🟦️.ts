import { spawn } from "node:child_process";
import { createHash, randomBytes } from "node:crypto";
import { closeSync, existsSync, mkdirSync, openSync, readFileSync, realpathSync, renameSync, rmSync, writeFileSync, writeSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { createInterface } from "node:readline";
import { setTimeout as delay } from "node:timers/promises";
import { acquireQueuedResourceLease, type LeaseWait, type ResourceLease } from "../../../🔒️leases/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🪓️termination/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🪪️ One canonical compiler directory and profile protected before any Cargo unit lock. */
export type CargoBuildLeaseIdentityV1 = { readonly buildDirectory: string; readonly profile: string; readonly mode: "exclusive" };

/** 🏷️ The one Cargo profile a build names before the compiler arguments, `undefined` when it names none. */
export function cargoRequestedProfileV1(args: readonly string[]): string | undefined {
  const argumentsBeforeCompiler = args.slice(0, args.includes("--") ? args.indexOf("--") : args.length);
  const profiles: string[] = [];
  for (let index = 0; index < argumentsBeforeCompiler.length; index++) {
    const arg = argumentsBeforeCompiler[index]!;
    if (arg === "--release") profiles.push("release");
    else if (arg === "--profile") { const value = argumentsBeforeCompiler[++index]; if (!value) throw Error("Cargo profile requires a value"); profiles.push(value); }
    else if (arg.startsWith("--profile=")) profiles.push(arg.slice(10));
  }
  if (profiles.length > 1) throw Error("Cargo build declares multiple profiles");
  return profiles[0];
}

/** 🧭️ Resolves actual Cargo profile aliases and physical cache identity from first-party schema rules. */
export function cargoBuildLeaseIdentityV1(buildDirectory: string, args: readonly string[]): CargoBuildLeaseIdentityV1 {
  const authored = cargoRequestedProfileV1(args) ?? schema.definitions.Protocol.const.defaultProfile;
  if (!new RegExp(schema.properties.profile.pattern, "u").test(authored)) throw Error("Invalid Cargo build profile");
  const aliases = schema.definitions.Protocol.const.profileDirectories as Record<string, string>;
  const profile = aliases[authored] ?? authored;
  let existing = resolve(buildDirectory);
  const suffix: string[] = [];
  while (!existsSync(existing)) { const parent = dirname(existing); if (parent === existing) throw Error("Cargo build directory has no physical ancestor"); suffix.unshift(basename(existing)); existing = parent; }
  const canonical = join(realpathSync(existing), ...suffix).replaceAll("\\", "/").normalize("NFC");
  if (canonical.length > schema.properties.buildDirectory.maxLength || canonical.includes("\0")) throw Error("Invalid Cargo build directory");
  const mode = schema.properties.mode.const;
  if (mode !== "exclusive") throw Error("Cargo build lease requires exclusive compiler ownership");
  return { buildDirectory: canonical, profile, mode };
}

/** 🔒️ Queues one compiler process while keeping different profile caches independent. */
export async function acquireCargoBuildLeaseV1(options: { readonly directory: string; readonly buildDirectory: string; readonly args: readonly string[]; readonly signal: AbortSignal; readonly onWait?: (progress: LeaseWait) => void }): Promise<ResourceLease> {
  const identity = cargoBuildLeaseIdentityV1(options.buildDirectory, options.args);
  const resource = JSON.stringify([schema.definitions.Protocol.const.namespace, identity.buildDirectory, identity.profile]);
  return acquireQueuedResourceLease({ directory: options.directory, resource, mode: identity.mode, owner: randomBytes(16).toString("base64url"), signal: options.signal, onWait: options.onWait });
}

const prebuildRule = schema.definitions.Protocol.const.prebuild;

/** 🧱️ One Cargo workspace and the component packages a prebuild compiles together. */
export type CargoPrebuildWorkspaceV1 = { readonly manifest: string; readonly packages: readonly string[] };

/** 🗺️ Every workspace whose component packages one build session compiles in a single Cargo pass per profile. */
export type CargoPrebuildPlanV1 = { readonly version: 1; readonly workspaces: readonly CargoPrebuildWorkspaceV1[] };

/** 🎯️ The pass a narrow component build triggers: its workspace's planned packages under its own profile and target. */
export type CargoPrebuildRequestV1 = { readonly workspace: CargoPrebuildWorkspaceV1; readonly profile: string; readonly target: string };

/** 🛂️ Admits only a plan that satisfies the first-party prebuild schema. */
export function admitCargoPrebuildPlanV1(value: unknown): CargoPrebuildPlanV1 {
  const errors = validateJsonSchemaSubset(schema.definitions.PrebuildPlan, value);
  if (errors.length) throw Error(`Invalid Cargo prebuild plan: ${errors.join("; ")}`);
  return value as CargoPrebuildPlanV1;
}

/** 📖️ Reads the plan a caller names in its environment, `null` when it names none. */
export function readCargoPrebuildPlanV1(environment: Readonly<Record<string, string | undefined>>): CargoPrebuildPlanV1 | null {
  const path = environment[prebuildRule.planEnvironment];
  return path ? admitCargoPrebuildPlanV1(JSON.parse(readFileSync(path, "utf8"))) : null;
}

/** 🧱️ How many profiles' passes may run at once: the largest compile unit of a profile (the stdio semio artifact, measured 14 to 20 GiB at
 * both dev and release) must fit beside the other profile's, so a 32 GiB machine runs one pass at a time. */
export function cargoPrebuildPassesV1(memoryBytes: number): 1 | 2 {
  return Math.floor((memoryBytes - prebuildRule.reserveBytes) / prebuildRule.heavyUnitBytes) >= 2 ? 2 : 1;
}

/** 🧮️ Concurrent compiler jobs ONE prebuild pass may run: every core, held back by the memory the largest units need
 * beside a heavy one (a linked hub component measured 5 to 7 GiB, the stdio semio artifact 14 to 20 GiB), shared between the passes that run at once. */
export function cargoPrebuildJobsV1(cpus: number, memoryBytes: number, environment: Readonly<Record<string, string | undefined>> = {}): number {
  const override = environment[prebuildRule.jobsEnvironment];
  if (override !== undefined) {
    const jobs = Number(override);
    if (!Number.isSafeInteger(jobs) || jobs < 1) throw Error(`Invalid ${prebuildRule.jobsEnvironment}`);
    return jobs;
  }
  const machine = Math.max(1, Math.min(cpus, Math.floor((memoryBytes - prebuildRule.reserveBytes) / prebuildRule.bytesPerJob)));
  return Math.max(1, Math.ceil(machine / cargoPrebuildPassesV1(memoryBytes)));
}

/** 🎯️ Selects the planned workspace that owns a narrow build's `-p` package, `null` for any other build. */
export function cargoPrebuildRequestV1(plan: CargoPrebuildPlanV1, args: readonly string[]): CargoPrebuildRequestV1 | null {
  const profile = cargoRequestedProfileV1(args);
  const targetIndex = args.indexOf("--target");
  const packageIndex = args.indexOf("-p");
  const target = targetIndex >= 0 ? args[targetIndex + 1] : undefined;
  const name = packageIndex >= 0 ? args[packageIndex + 1] : undefined;
  if (!profile || !target || !name) return null;
  const workspace = plan.workspaces.find((candidate) => candidate.packages.includes(name));
  return workspace ? { workspace, profile, target } : null;
}

/** 🔑️ Names one prebuild by its physical compiler directory, profile, target and exact package roster. */
export function cargoPrebuildKeyV1(buildDirectory: string, request: CargoPrebuildRequestV1): string {
  return createHash("sha256").update(JSON.stringify([prebuildRule.namespace, buildDirectory, request.profile, request.target, request.workspace.manifest, [...request.workspace.packages].sort()])).digest("hex").slice(0, 32);
}

/** 🏁️ The planned package whose linked component a Cargo JSON message announces, `null` for every other message. */
export function cargoPrebuildReadyPackageV1(line: string, packages: ReadonlySet<string>): string | null {
  if (!line.startsWith("{")) return null;
  try {
    const message = JSON.parse(line) as { reason?: string; package_id?: string; target?: { kind?: string[] }; filenames?: string[] };
    if (message.reason !== "compiler-artifact" || !message.target?.kind?.includes("cdylib") || !message.filenames?.some((file) => file.endsWith(".wasm"))) return null;
    const name = (message.package_id ?? "").split("#")[1]?.split("@")[0];
    return name !== undefined && packages.has(name) ? name : null;
  } catch { return null; }
}

/** 🫀️ Whether the process that claimed a pass still exists (EPERM means it exists under another user). */
function claimantAlive(pid: number): boolean {
  if (!Number.isSafeInteger(pid) || pid <= 0) return false;
  try { process.kill(pid, 0); return true; } catch (error) { return (error as NodeJS.ErrnoException).code === "EPERM"; }
}

/** 🔥️ Gets the planned components of a workspace compiled in ONE Cargo pass before the first narrow build of the profile.
 *
 * Cargo schedules the whole dependency graph across the job budget instead of one package at a time, and no second
 * Cargo process contends for the shared units while it runs, which is what made concurrent per-package builds deadlock on
 * `prebuild_lock_exclusive`. Feature unification stays per package, so every unit equals the unit a narrow
 * `cargo build -p <package>` selects and the narrow build afterwards finds the compiler store fresh.
 *
 * Where memory cannot hold two profiles' largest units the passes of different profiles queue on one lease per compiler
 * directory ({@link cargoPrebuildPassesV1}); the claim stays held meanwhile, so waiters of either profile keep polling their own package.
 *
 * The first caller of a key claims the pass (`<key>.run`, exclusive create) and runs it; every other caller waits for
 * its OWN package, not for the pass: the runner records `<key>.ready/<package>` the moment Cargo links that component,
 * so narrow builds, staging, provenance, describe and the TypeScript materialization of finished components overlap the
 * compile of the rest. A claim whose process died is taken over; the new pass finds every finished unit fresh. The
 * pass is only a cache warm: `--keep-going` and a non-zero status never fail it, each narrow build reports its own
 * errors. The caller holds no lease: a package is announced only after all of its units are complete, so a narrow build
 * of it finds nothing the pass still writes. Diagnostics go to a bounded log file; the terminal sees one throttled
 * progress row and, after a failure, the compiler errors.
 * @returns `ran` when this call ran the pass, `ready` when it waited for its package or the pass, `skipped` when the build is not planned. */
export async function ensureCargoPrebuildV1(options: { readonly plan: CargoPrebuildPlanV1; readonly args: readonly string[]; readonly cwd: string; readonly directory: string; readonly buildDirectory: string; readonly environment: Readonly<Record<string, string | undefined>>; readonly jobs: number; readonly passes: 1 | 2; readonly signal: AbortSignal; readonly commandPort?: Readonly<{ command: string; args: readonly string[] }> }): Promise<"ran" | "ready" | "skipped"> {
  const request = cargoPrebuildRequestV1(options.plan, options.args);
  if (!request) return "skipped";
  const identity = cargoBuildLeaseIdentityV1(options.buildDirectory, options.args);
  const key = cargoPrebuildKeyV1(identity.buildDirectory, request);
  const store = join(options.directory, prebuildRule.directory);
  const marker = join(store, `${key}.json`), claim = join(store, `${key}.run`), ready = join(store, `${key}.ready`);
  const own = options.args[options.args.indexOf("-p") + 1]!;
  mkdirSync(ready, { recursive: true });
  for (;;) {
    options.signal.throwIfAborted();
    if (existsSync(marker) || existsSync(join(ready, own))) return "ready";
    try { writeFileSync(claim, `${process.pid}\n`, { flag: "wx" }); break; }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
      let pid = Number.NaN;
      try { pid = Number(readFileSync(claim, "utf8")); } catch { continue; }
      if (!claimantAlive(pid)) rmSync(claim, { force: true });
      else await delay(prebuildRule.pollMs, undefined, { signal: options.signal });
    }
  }
  const pass = options.passes === 1 ? await acquireQueuedResourceLease({ directory: options.directory, resource: JSON.stringify([prebuildRule.namespace, "pass", identity.buildDirectory]), mode: "exclusive", owner: randomBytes(16).toString("base64url"), signal: options.signal }).catch((error) => { rmSync(claim, { force: true }); throw error; }) : null;
  const packages = new Set(request.workspace.packages);
  const target = join(store, `${key}.target`);
  const logPath = join(store, `${key}.log`);
  const args = ["build", "--locked", "--manifest-path", request.workspace.manifest, ...request.workspace.packages.flatMap((name) => ["-p", name]), "--lib", "--target", request.target, "--profile", request.profile, "--keep-going", "--jobs", String(options.jobs), "--message-format=json-render-diagnostics"];
  const environment = { ...options.environment, CARGO_TARGET_DIR: target, CARGO_BUILD_BUILD_DIR: options.buildDirectory, SEMIO_COMPILER_RESOURCE_ROOT: join(options.buildDirectory, "semio-compiler-resources"), CARGO_UNSTABLE_FEATURE_UNIFICATION: "true", CARGO_RESOLVER_FEATURE_UNIFICATION: "package", CARGO_TERM_COLOR: "never" };
  const started = Date.now();
  const log = openSync(logPath, "w");
  const failures: string[] = [];
  let logged = 0, compiled = 0, lines = 0, finished = 0, nextProgress = started;
  const commandPort = options.commandPort ?? { command: "cargo", args: [] };
  const child = spawn(commandPort.command, [...commandPort.args, ...args], { cwd: options.cwd, env: environment, detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] });
  const cancel = (): void => terminateOwnedChildTree(child);
  options.signal.addEventListener("abort", cancel, { once: true });
  if (options.signal.aborted) cancel();
  const status = new Promise<number>((accept) => { child.once("error", (error) => { console.error(error.message); accept(1); }); child.once("close", (code) => accept(code ?? 1)); });
  const progress = (): void => {
    const now = Date.now();
    if (now >= nextProgress) { console.log(`[cargo-prebuild] ${request.profile} ${finished}/${packages.size} component(s) linked, ${compiled} unit(s) compiled in ${Math.round((now - started) / 1000)} s`); nextProgress = now + prebuildRule.progressMs; }
  };
  try {
    await Promise.all([
      (async () => { for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) { const name = cargoPrebuildReadyPackageV1(line, packages); if (name !== null) { writeFileSync(join(ready, name), ""); finished++; } progress(); } })(),
      (async () => { for await (const line of createInterface({ input: child.stderr!, crlfDelay: Infinity })) {
        lines++;
        if (logged < prebuildRule.logLimitBytes) logged += writeSync(log, `${line}\n`);
        if (/^error/u.test(line) && failures.length < prebuildRule.tailLines) failures.push(line);
        if (/^\s+Compiling /u.test(line)) compiled++;
        progress();
      } })(),
    ]);
    const exitCode = await status;
    options.signal.throwIfAborted();
    const elapsedMs = Date.now() - started;
    console.log(`[cargo-prebuild] ${request.profile} ${finished}/${packages.size} component(s) linked, ${compiled} unit(s) compiled, ${lines} diagnostic line(s) in ${logPath}, exit ${exitCode}, ${Math.round(elapsedMs / 1000)} s`);
    if (exitCode !== 0) console.error(`[cargo-prebuild] pass reported errors; narrow builds report their own:\n${failures.join("\n")}`);
    const temporary = `${marker}.${process.pid}.tmp`;
    writeFileSync(temporary, `${JSON.stringify({ version: 1, key, profile: request.profile, target: request.target, manifest: request.workspace.manifest, packages: request.workspace.packages, jobs: options.jobs, exitCode, compiled, finished, elapsedMs })}\n`);
    renameSync(temporary, marker);
    return "ran";
  } finally {
    options.signal.removeEventListener("abort", cancel);
    closeSync(log);
    rmSync(claim, { force: true });
    rmSync(target, { recursive: true, force: true });
    pass?.release();
  }
}
