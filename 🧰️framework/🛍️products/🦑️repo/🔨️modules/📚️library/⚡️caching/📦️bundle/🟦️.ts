import { scriptInvocationEnvironment, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { mkdir, mkdtemp, readFile, rm, lstat, realpath, copyFile } from "node:fs/promises";
import type { BunBundleRequest, BunBundleResult } from "./🟦️bun/📜️script.ts";
import { availableParallelism } from "node:os";
import { acquireCompilerCapacityLeaseV1, compilerCapacityWorkerCountV1, scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1, type CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { createRequire } from "node:module";
import { join, dirname, relative, resolve, isAbsolute } from "node:path";
import { createReadStream } from "node:fs";
import { createHash } from "node:crypto";
import workerSchema from "./🟦️bun/🧬️schema/🔣️.json" with { type: "json" };
import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { buildModeEnvironment, buildModePolicy, type BuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

/** 📦️ Describes a cancellable JavaScript artifact without exposing a compiler SDK. */
export interface JavaScriptBundle { readonly workspace: string; readonly source: string; readonly output: string; readonly mode: BuildMode; readonly platform: "node" | "browser"; readonly signal: AbortSignal; readonly environment: Readonly<Record<string, string | undefined>>; readonly external?: readonly string[]; readonly runtime?: "javascript" | "bun"; readonly invocation: ScriptInvocation; }

/** 🏗️ Starts esbuild after admission so its Go workers honor the inherited compiler budget. */
export async function bundleJavaScript(options: JavaScriptBundle): Promise<void> {
  const failures = validateJsonSchemaSubset(schema, Object.fromEntries(Object.entries(options).filter(([key, value]) => !["signal", "environment", "invocation"].includes(key) && value !== undefined)));
  if (failures.length) throw Error("Invalid JavaScript bundle: " + failures.join("; "));
  if (options.runtime === "bun") {
    if (!options.invocation) throw Error("Bun package compiler requires original invocation");
    const root = options.environment.SEMIO_TICKET_DIR ? join(options.environment.SEMIO_TICKET_DIR, "🗑️generated") : join(options.workspace, ".🧬semio/🦑️repo/⚡️cache/bun-workers");
    await withBunBundleFiles({ version: 1, kind: "bun-package", mode: options.mode, entrypoint: options.source }, { workspace: options.workspace, temporaryRoot: root, invocation: options.invocation, signal: options.signal, environment: options.environment }, async files => {
      const transferSignal = AbortSignal.any([options.signal, options.invocation.control.signal]);
      if (resolve(options.output).length > 256 || (files.sourceMap && resolve(options.output + ".map").length > 256)) throw Error("Bun package output path exceeds 256 characters"); await mkdir(dirname(options.output), { recursive: true }); await copyFile(files.source, options.output); await verifyBunFile(options.output, files.sourceFile, transferSignal);
      if (files.sourceMap) { await copyFile(files.sourceMap, options.output + ".map"); await verifyBunFile(options.output + ".map", files.sourceMapFile!, transferSignal); } transferSignal.throwIfAborted();
    });
    return;
  }
  const require = createRequire(join(options.workspace, "package.json")), policy = buildModePolicy(options.mode);
  const args = [require.resolve("esbuild/bin/esbuild"), options.source, "--bundle", `--tree-shaking=${policy.optimize}`, "--format=esm", "--target=es2022", `--platform=${options.platform}`, `--outfile=${options.output}`, ...(policy.optimize ? ["--minify"] : []), ...(policy.sourceMap ? ["--sourcemap=external"] : []), ...["bun", ...(options.external ?? [])].map(value => `--external:${value}`)];
  const control = scriptCompilerAdmissionControlV1(options.invocation, "javascript:esbuild", 120_000, options.signal), lease = await acquireCompilerCapacityLeaseV1({ directory: join(options.workspace, ".cache/framework/leases"), control, environment: options.environment });
  try {
    const environment = { ...options.environment, SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: String(lease.parallelism), GOMAXPROCS: String(lease.parallelism) };
    console.log(`[javascript-build] mode=${options.mode} workers=${lease.parallelism}`);
    await runOwnedCommand(process.execPath, args, options.workspace, "javascript:esbuild", compilerAdmissionBudgetV1(control), { env: environment, signal: control.signal });
  } finally { lease.release(); }
}

/** 🎟️ Admits compilers with an unbounded worker pool only while all host workers are reserved. */
export async function withWholeHostCompilerCapacity<T>(directory: string, control: CompilerAdmissionControl, operation: (environment: Readonly<Record<string, string | undefined>>) => Promise<T>, environment: Readonly<Record<string, string | undefined>> = process.env): Promise<T> {
  if (environment.SEMIO_COMPILER_CAPACITY_HELD === "true" && compilerCapacityWorkerCountV1(environment) !== availableParallelism()) throw Error("Unbounded compiler requires the complete host worker reservation");
  const lease = await acquireCompilerCapacityLeaseV1({ directory, control, environment });
  try { control.signal.throwIfAborted(); return await operation({ ...environment, SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: String(lease.parallelism), GOMAXPROCS: String(lease.parallelism) }); } finally { lease.release(); }
}

type BunCompileOptions = Readonly<{ workspace: string; temporaryRoot: string; invocation: ScriptInvocation; signal?: AbortSignal; environment?: Readonly<Record<string, string | undefined>>; onLine?: (line: string) => void }>;
type BunBundleFiles = Readonly<{ source: string; sourceMap?: string; metadata: Omit<BunBundleResult, "source" | "sourceMap">; sourceFile: BunFile; sourceMapFile?: BunFile }>;
type BunFile = Readonly<{ path: string; sha256: string; byteLength: number }>;

/** 🔏️ Checks real file bytes and stable identity without creating a bulk IPC representation. */
async function verifyBunFile(path: string, expected: Readonly<{ sha256: string; byteLength: number }>, signal: AbortSignal): Promise<void> {
  signal.throwIfAborted(); const before = await lstat(path);
  if (!before.isFile() || before.size !== expected.byteLength) throw Error("Bun compiler file custody refused");
  const hash = createHash("sha256"); let size = 0;
  for await (const chunk of createReadStream(path, { signal })) { hash.update(chunk); size += chunk.byteLength; }
  const after = await lstat(path); signal.throwIfAborted();
  if (size !== expected.byteLength || hash.digest("hex") !== expected.sha256 || ["dev", "ino", "size", "mtimeMs", "ctimeMs"].some(key => before[key as keyof typeof before] !== after[key as keyof typeof after])) throw Error("Bun compiler file bytes changed or digest refused");
}

/** 🗂️ Admits one bounded worker manifest and proves both captured sources and produced files before promotion. */
export async function readBunBundleFiles(request: BunBundleRequest, response: string, signal: AbortSignal): Promise<BunBundleFiles> {
  const stat = await lstat(response); if (!stat.isFile() || stat.size > 4096 || resolve(response).length > 256) throw Error("Bun compiler manifest custody refused");
  const manifest = JSON.parse(await readFile(response, { encoding: "utf8", signal }));
  if (validateJsonSchemaSubset(workerSchema.definitions.Response, manifest).length || manifest.mode !== request.mode || manifest.kind !== request.kind || manifest.requestSha256 !== createHash("sha256").update(JSON.stringify(request)).digest("hex")) throw Error("Bun compiler manifest request binding refused");
  const root = await realpath(dirname(response)), owner = dirname(manifest.source.path);
  const admit = async (record: BunFile, name: string): Promise<string> => {
    const path = resolve(root, record.path), child = relative(root, path);
    if (isAbsolute(record.path) || child.startsWith("..") || path.length > 256 || record.path !== `${owner}/${name}` || !owner.startsWith(`bundle-${request.mode}-`) || await realpath(path) !== path) throw Error("Bun compiler output ownership refused");
    await verifyBunFile(path, record, signal); return path;
  };
  const source = await admit(manifest.source, "source.js"), sourceMap = manifest.sourceMap ? await admit(manifest.sourceMap, "source.js.map") : undefined, metadataPath = await admit(manifest.metadata, "metadata.json"), metadata = JSON.parse(await readFile(metadataPath, { encoding: "utf8", signal }));
  if (validateJsonSchemaSubset(workerSchema.definitions.Metadata, metadata).length || metadata.inputs.length !== metadata.physicalInputs.length) throw Error("Bun compiler source metadata refused");
  const paths = new Set<string>();
  for (let index = 0; index < metadata.physicalInputs.length; index++) {
    const input = metadata.physicalInputs[index], { path, ...logical } = input;
    if (paths.has(path) || await realpath(path) !== path || JSON.stringify(logical) !== JSON.stringify(metadata.inputs[index])) throw Error("Bun compiler source coverage refused");
    paths.add(path); await verifyBunFile(path, input, signal);
  }
  signal.throwIfAborted(); return { source, sourceFile: manifest.source, ...(sourceMap ? { sourceMap, sourceMapFile: manifest.sourceMap } : {}), metadata };
}

async function withBunBundleFiles<T>(request: BunBundleRequest, options: BunCompileOptions, consume: (files: BunBundleFiles) => Promise<T>): Promise<T> {
  const control = scriptCompilerAdmissionControlV1(options.invocation, "javascript:bun-worker", 120_000, options.signal);
  const input = JSON.stringify(request); if (Buffer.byteLength(input) > 16 * 1024 * 1024) throw Error("Bun compiler request bytes refused");
  await mkdir(options.temporaryRoot, { recursive: true });
  const temporary = await mkdtemp(join(options.temporaryRoot, "bun-worker-")), output = join(temporary, "result.json");
  try {
    return await withWholeHostCompilerCapacity(join(options.workspace, ".cache/framework/leases"), control, async compilerEnvironment => {
      await runOwnedCommand(process.execPath, [join(import.meta.dir, "🟦️bun/📜️script.ts"), "compile", output], options.workspace, "javascript:bun-worker", compilerAdmissionBudgetV1(control), { input, signal: control.signal, env: scriptInvocationEnvironment(options.invocation, buildModeEnvironment(request.mode, compilerEnvironment)), onLine: options.onLine });
      const files = await readBunBundleFiles(request, output, control.signal); compilerAdmissionBudgetV1(control); const result = await consume(files); control.signal.throwIfAborted(); compilerAdmissionBudgetV1(control); return result;
    }, options.environment);
  } finally { await rm(temporary, { recursive: true, force: true }); }
}

/** 🧵️ Decodes admitted bytes only for callers whose owned API consumes generated source in memory. */
export async function bundleVirtualJavaScript(request: BunBundleRequest, options: BunCompileOptions): Promise<BunBundleResult> {
  return withBunBundleFiles(request, options, async files => ({ ...files.metadata, source: await readFile(files.source, { encoding: "utf8", signal: options.signal ? AbortSignal.any([options.signal, options.invocation.control.signal]) : options.invocation.control.signal }), ...(files.sourceMap ? { sourceMap: await readFile(files.sourceMap, { encoding: "utf8", signal: options.signal ? AbortSignal.any([options.signal, options.invocation.control.signal]) : options.invocation.control.signal }) } : {}) }));
}
