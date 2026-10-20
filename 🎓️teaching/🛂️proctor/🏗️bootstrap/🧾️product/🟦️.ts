/** 🧾️ The proctor as a cached build product. `@teaching/proctor:build` compiles the executable through the repository's
 * issued Cargo launcher (`--store=shared`, the same code `bun repo cargo` runs) and leaves it at a content-addressed place,
 * `dist/build/<mode>/<sha256>/proctor`, beside `dist/build/<mode>/receipt.json`, which names that exact path, the digest,
 * the platform and every source file Cargo's dep-info says it read. Every consumer — the dev stack, `check`, the
 * end-to-end gate, the image build, the operator verbs — resolves that receipt with {@link acquireProctor}: a missing or
 * stale product is one error in English and German and never a rebuild in disguise.
 *
 * The inputs of the build are the Cargo closure of `teaching-proctor`: {@link proctorSourceGlobs} derives them from the
 * manifests, and the build fails closed when Cargo's own dep-info names a source they do not declare, so that no edit
 * can replay a stale cache entry.
 *
 * @see ../🟦️.ts — the in-process build this replaces for the course, the launch and the artifact types
 * @see ./🧪️tests/🧫️fixtures/🔣️.json — the language-agnostic fixture of every rule below
 * @see ../../../🏛️architecture/📚️course/🧪️tests/🧾️proctor-product/🟦️.ts — the laws, with Ajv, @noble/hashes and picomatch as oracles
 * @see https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages
 * @see https://doc.rust-lang.org/cargo/reference/build-cache.html#dep-info-files */
import { createHash } from "node:crypto";
import { createReadStream, readFileSync } from "node:fs";
import { chmod, copyFile, mkdir, open, readFile, readdir, rename, rm, stat, writeFile } from "node:fs/promises";
import { basename, dirname, extname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { createIssuedResourceControlV1, requireIssuedScriptInvocationV1, scriptInvocationEnvironment, type ScriptInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { writeOwnedCommandOutputV1 } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import type { BuildMode } from "../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { launchRepositoryCargoV1, readRepositoryCargoLaunchV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { PROCTOR_PACKAGE, TEACHING_WORKSPACE, buildProctor, developmentDataSettled, launchProctor, LAUNCHER_SIGNALS, proctorBuildMode, proctorDevelopmentEnvironment, proctorSourceDirectories, type ProctorArtifact, type ProctorExecutable, type ProctorProcess } from "../🟦️.ts";
import messages from "./💬️messages/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import declaredData from "./📥️inputs/🔣️.json" with { type: "json" };

/** 🗃️ Where the products of the modes live, relative to the repository root: the output directory `@teaching/proctor:build` declares. */
export const PROCTOR_PRODUCT_DIRECTORY = ["🎓️teaching", "🛂️proctor", "📦️packages", "🦀️rust", "dist", "build"] as const;

/** 📋️ The manifest of `@teaching/proctor`, which declares the inputs and outputs of the build. */
export const PROCTOR_MANIFEST = ["🎓️teaching", "🛂️proctor", "📦️packages", "🦀️rust", "📋️project.json"] as const;

/** 🧾️ The receipt inside the directory of a mode. */
export const PROCTOR_PRODUCT_RECEIPT = "receipt.json";

/** 🗂️ The workspace manifests whose edits change what Cargo builds although dep-info never lists them. */
export const PROCTOR_WORKSPACE_FILES = ["🎓️teaching/Cargo.toml", "🎓️teaching/Cargo.lock"] as const;

/** 🚫️ Directories whose files are build output or installed packages, never sources of the proctor. */
export const PROCTOR_EXCLUDED_DIRECTORIES = ["target", "dist", "node_modules"] as const;

/** 📥️ Which way the course obtains the proctor: from the built product, or (until the manifests declare the product as a
 * dependency of every consumer) by the in-process build of {@link buildProctor}. */
export const PROCTOR_ACQUISITION_DEFAULT: "product" | "build" = "product";

/** 🏷️ The environment variable that selects the acquisition for one run. */
export const PROCTOR_ACQUISITION_ENVIRONMENT = "SEMIO_PROCTOR_ACQUISITION";

/** 🧾️ What `receipt.json` says; the schema is `./🧬️schema/🔣️.json`. */
export interface ProctorProductReceipt {
  readonly version: 1;
  readonly package: "teaching-proctor";
  readonly binary: "proctor";
  readonly mode: BuildMode;
  readonly platform: NodeJS.Platform;
  readonly architecture: string;
  readonly executable: string;
  readonly sha256: string;
  readonly bytes: number;
  readonly sources: readonly string[];
}

/** 🖥️ The host a product runs on. */
export interface ProctorProductHost {
  readonly platform: NodeJS.Platform;
  readonly architecture: string;
}

/** 📦️ A product that passed {@link checkProctorProduct}: its directory, the receipt and the absolute path of the executable. */
export interface ProctorProduct {
  readonly directory: string;
  readonly receipt: ProctorProductReceipt;
  readonly executable: string;
}

/** 🚦️ Why a product cannot be used. */
export type ProctorProductFailure = "missing" | "stale" | "uncovered";

/** 🔎️ Which property of a stale product failed. */
export type ProctorProductReason = "unreadable" | "invalid" | "mode" | "platform" | "executable" | "size" | "hash";

type MessageValue = string | number | { readonly reason: ProctorProductReason; readonly values: Readonly<Record<string, string | number>> };

const receiptSchema = Object.freeze({ ...schema, $ref: "#/definitions/Receipt" });
const messageSchema = Object.freeze({ ...schema, $ref: "#/definitions/Messages" });
const inputsSchema = Object.freeze({ ...schema, $ref: "#/definitions/Inputs" });

/** 🌐️ The data files outside the owner roots (and not Rust) that Cargo reads: `include_str!`, schemas. */
export const PROCTOR_DATA_FILES: readonly string[] = (() => {
  if (validateJsonSchemaSubset(inputsSchema, declaredData).length) throw Error("Closed proctor input declaration required");
  return Object.freeze([...declaredData.files]);
})();

/** 🛠️ The command that produces the product of `mode`, as the tool of the run spells it. */
export function proctorBuildCommand(mode: BuildMode, env: Readonly<Record<string, string | undefined>> = process.env): string {
  return `bun repo${env.SEMIO_BUILD_TOOL === "bazel" ? " --tool=bazel" : ""} run @teaching/proctor:build${mode === "release" ? " --configuration=release" : ""}`;
}

function render(text: string, values: Readonly<Record<string, MessageValue>>, language: "en" | "de"): string {
  return text.replace(/\{(\w+)\}/gu, (_, name: string) => {
    const value = values[name];
    if (value === undefined) throw Error(`Proctor product message value ${name} required`);
    if (typeof value === "object") return render(messages.reasons[value.reason][language], value.values, language);
    return String(value);
  });
}

/** 🗣️ A product message in English, then German on the next line. `values` fill the `{placeholders}`; the build command is added. */
export function proctorProductMessage(code: ProctorProductFailure, values: Readonly<Record<string, MessageValue>>, env: Readonly<Record<string, string | undefined>> = process.env): string {
  if (validateJsonSchemaSubset(messageSchema, messages).length) throw Error("Closed proctor product messages required");
  const mode = values.mode === "release" ? "release" : "prototype", all = { command: proctorBuildCommand(mode, env), ...values };
  return (["en", "de"] as const).map(language => render(messages.errors[code][language], all, language)).join("\n");
}

/** 🚨️ The one error of an unusable product: `message` says it in English and German. */
export class ProctorProductError extends Error {
  constructor(readonly code: ProctorProductFailure, message: string, readonly reason?: ProctorProductReason) {
    super(message);
    this.name = "ProctorProductError";
  }
}

/** 📂️ The directory of the product of `mode`. */
export function proctorProductDirectory(repoRoot: string, mode: BuildMode): string {
  return join(resolve(repoRoot), ...PROCTOR_PRODUCT_DIRECTORY, mode);
}

/** 🖥️ This host. */
export function currentProctorHost(): ProctorProductHost {
  return { platform: process.platform, architecture: process.arch };
}

/** 🏷️ The file name of the executable on `platform`. */
export function proctorExecutableName(platform: NodeJS.Platform): string {
  return platform === "win32" ? "proctor.exe" : "proctor";
}

/** 🔐️ Admits a receipt: the closed schema, and an executable path that is the digest and the platform's name. */
export function readProctorProductReceipt(value: unknown): ProctorProductReceipt {
  if (validateJsonSchemaSubset(receiptSchema, value).length) throw Error("Closed proctor product receipt required");
  const receipt = value as ProctorProductReceipt;
  if (receipt.executable !== `${receipt.sha256}/${proctorExecutableName(receipt.platform)}`) throw Error("Proctor product receipt executable must be its digest and the platform's name");
  return Object.freeze({ ...receipt, sources: Object.freeze([...receipt.sources]) });
}

/** 🧮️ The SHA-256 of a file, read once in a stream. */
export async function proctorFileDigest(path: string): Promise<{ readonly sha256: string; readonly bytes: number }> {
  const hash = createHash("sha256");
  let bytes = 0;
  for await (const chunk of createReadStream(path)) {
    hash.update(chunk as Buffer);
    bytes += (chunk as Buffer).length;
  }
  return { sha256: hash.digest("hex"), bytes };
}

const missing = (error: unknown): boolean => typeof error === "object" && error !== null && "code" in error && (error.code === "ENOENT" || error.code === "ENOTDIR");

/** 🔎️ Resolves the product of `mode`: the receipt, its platform, the executable it names, that executable's size and digest.
 * Reads only; it starts nothing and builds nothing. Throws {@link ProctorProductError}. */
export async function checkProctorProduct(repoRoot: string, mode: BuildMode, host: ProctorProductHost = currentProctorHost(), env: Readonly<Record<string, string | undefined>> = process.env): Promise<ProctorProduct> {
  const directory = proctorProductDirectory(repoRoot, mode), path = join(directory, PROCTOR_PRODUCT_RECEIPT);
  const stale = (reason: ProctorProductReason, values: Record<string, string | number> = {}): ProctorProductError =>
    new ProctorProductError("stale", proctorProductMessage("stale", { mode, reason: { reason, values: { mode, path, ...values } } }, env), reason);
  let text: string;
  try {
    text = await readFile(path, "utf8");
  } catch (error) {
    if (missing(error)) throw new ProctorProductError("missing", proctorProductMessage("missing", { mode, path }, env));
    throw stale("unreadable", { detail: error instanceof Error ? error.message : String(error) });
  }
  let receipt: ProctorProductReceipt;
  try {
    receipt = readProctorProductReceipt(JSON.parse(text));
  } catch {
    throw stale("invalid");
  }
  if (receipt.mode !== mode) throw stale("mode", { actual: receipt.mode });
  if (receipt.platform !== host.platform || receipt.architecture !== host.architecture) throw stale("platform", { actual: `${receipt.platform}/${receipt.architecture}`, host: `${host.platform}/${host.architecture}` });
  const executable = join(directory, receipt.executable);
  let facts;
  try {
    facts = await stat(executable);
  } catch (error) {
    if (missing(error)) throw stale("executable", { path: executable });
    throw stale("unreadable", { detail: error instanceof Error ? error.message : String(error) });
  }
  if (!facts.isFile()) throw stale("executable", { path: executable });
  if (facts.size !== receipt.bytes) throw stale("size", { path: executable, actual: facts.size, expected: receipt.bytes });
  if ((await proctorFileDigest(executable)).sha256 !== receipt.sha256) throw stale("hash", { path: executable });
  return Object.freeze({ directory, receipt, executable });
}

/** 🧭️ The JSON messages of a Cargo run, one object per line; lines that are no JSON object are Cargo's plain text and skipped. */
function cargoMessages(stdout: string): Record<string, any>[] {
  const rows: Record<string, any>[] = [];
  for (const line of stdout.split(/\r?\n/u)) {
    if (!line.startsWith("{")) continue;
    try {
      const row = JSON.parse(line);
      if (row !== null && typeof row === "object") rows.push(row);
    } catch {}
  }
  return rows;
}

/** 🎯️ The one executable of the `proctor` binary of `teaching-proctor` that Cargo reports. */
export function selectProctorExecutable(stdout: string): string {
  const found = new Set<string>();
  for (const row of cargoMessages(stdout)) {
    if (row.reason !== "compiler-artifact" || row.target?.name !== "proctor" || !row.target?.kind?.includes("bin")) continue;
    if (typeof row.package_id === "string" && !row.package_id.includes(PROCTOR_PACKAGE)) continue;
    if (typeof row.executable === "string") found.add(row.executable);
  }
  if (found.size !== 1) throw Error(`Exactly one proctor executable required from Cargo's output, found ${found.size}`);
  return [...found][0]!;
}

const LIBRARY_EXTENSIONS = new Set([".rlib", ".rmeta", ".a", ".so", ".dylib"]);

/** 📄️ The dep-info file Cargo wrote beside an artifact: `libfoo-hash.rlib` has `foo-hash.d`, `proctor-hash` has `proctor-hash.d`. */
function depInfoOf(file: string): string {
  const extension = extname(file), name = basename(file, extension);
  return join(dirname(file), `${LIBRARY_EXTENSIONS.has(extension) && name.startsWith("lib") ? name.slice(3) : name}.d`);
}

/** 🗺️ The dep-info files of every unit of this repository (path packages, build scripts and procedural macros included) that
 * Cargo's output reports, in the order it reports them; registry crates are locked by `Cargo.lock` and have none to read. */
export function proctorDepInfoFiles(stdout: string): string[] {
  const files = new Set<string>();
  for (const row of cargoMessages(stdout)) {
    if (row.reason !== "compiler-artifact" || typeof row.package_id !== "string" || !row.package_id.includes("path+file://")) continue;
    for (const file of Array.isArray(row.filenames) ? row.filenames : []) if (typeof file === "string") files.add(depInfoOf(file));
  }
  return [...files];
}

/** 📖️ The files a dep-info text lists as the dependencies of its first rule: a Makefile line `target: dependency …` in which
 * only a space is escaped (`\ `); comments (`# env-dep`, `# checksum`) and the empty phony rules after it are skipped. */
export function parseDepInfo(text: string): string[] {
  const line = text.split(/\r?\n/u).find(row => row.length > 0 && !row.startsWith("#"));
  if (line === undefined) return [];
  const colon = /:(?: |$)/u.exec(line);
  if (!colon) return [];
  const files: string[] = [];
  let current = "";
  const rest = line.slice(colon.index + 1);
  for (let index = 0; index < rest.length; index++) {
    const character = rest[index]!;
    if (character === "\\" && rest[index + 1] === " ") {
      current += " ";
      index++;
    } else if (character === " ") {
      if (current) files.push(current);
      current = "";
    } else current += character;
  }
  if (current) files.push(current);
  return files;
}

const SEPARATORS = new RegExp(`[${sep === "\\" ? "\\\\" : sep}/]`, "u");

/** 🧹️ The workspace-relative sources among dep-info paths: inside the repository, outside hidden folders (the Cargo stores)
 * and outside {@link PROCTOR_EXCLUDED_DIRECTORIES}; a relative path is relative to the teaching workspace Cargo ran in. */
export function proctorProductSources(repoRoot: string, paths: readonly string[]): string[] {
  const root = resolve(repoRoot), workspace = join(root, TEACHING_WORKSPACE), sources = new Set<string>();
  for (const path of paths) {
    const absolute = isAbsolute(path) ? resolve(path) : resolve(workspace, path), local = relative(root, absolute);
    if (local === "" || local.startsWith("..") || isAbsolute(local)) continue;
    const parts = local.split(SEPARATORS);
    if (parts[0]!.startsWith(".") || parts.some(part => (PROCTOR_EXCLUDED_DIRECTORIES as readonly string[]).includes(part))) continue;
    sources.add(parts.join("/"));
  }
  return [...sources].sort();
}

/** 🧬️ The inputs `@teaching/proctor:build` declares, from the owner roots of the Cargo closure (workspace-relative, `/`-separated):
 * their Rust sources and manifests, the teaching workspace's manifest and lock, the `data` files Cargo reads, and the
 * exclusion of build output and installed packages. */
export function proctorSourceGlobs(roots: readonly string[], data: readonly string[] = PROCTOR_DATA_FILES): string[] {
  const workspace = (path: string): string => `{workspaceRoot}/${path}`;
  return [
    ...roots.flatMap(root => [workspace(`${root}/**/*.rs`), workspace(`${root}/**/Cargo.toml`)]),
    ...PROCTOR_WORKSPACE_FILES.map(workspace),
    ...data.map(workspace),
    ...PROCTOR_EXCLUDED_DIRECTORIES.map(directory => `!${workspace(`**/${directory}/**`)}`),
  ];
}

/** 📋️ The `proctorRust` named input a manifest declares, or `undefined` before the manifest declares one. */
export function declaredProctorInputs(manifest: { readonly namedInputs?: Record<string, unknown> }): string[] | undefined {
  const rows = manifest.namedInputs?.proctorRust;
  return Array.isArray(rows) && rows.every(row => typeof row === "string") ? (rows as string[]) : undefined;
}

/** 🧭️ The sources (workspace-relative) that `inputs` — the shapes {@link proctorSourceGlobs} writes — do not declare. */
export function uncoveredProctorSources(sources: readonly string[], inputs: readonly string[]): string[] {
  const include: ((file: string) => boolean)[] = [], exclude: ((file: string) => boolean)[] = [];
  for (const row of inputs) {
    const negated = row.startsWith("!"), pattern = (negated ? row.slice(1) : row).replace(/^\{workspaceRoot\}\//u, "");
    const directory = /^\*\*\/([^*/]+)\/\*\*$/u.exec(pattern), recursive = /^(.+)\/\*\*\/([^/]+)$/u.exec(pattern);
    if (negated && directory) exclude.push(file => file.split("/").includes(directory[1]!));
    else if (!negated && recursive && recursive[2]!.startsWith("*.")) include.push(file => file.startsWith(`${recursive[1]}/`) && file.endsWith(recursive[2]!.slice(1)));
    else if (!negated && recursive && !recursive[2]!.includes("*")) include.push(file => file.startsWith(`${recursive[1]}/`) && file.split("/").at(-1) === recursive[2]);
    else if (!negated && !pattern.includes("*")) include.push(file => file === pattern);
    else throw Error(`Unsupported proctor input ${JSON.stringify(row)}`);
  }
  return sources.filter(file => !include.some(match => match(file)) || exclude.some(match => match(file)));
}

/** 📥️ What the manifest of `@teaching/proctor` declares for the build, or `undefined` before it declares anything. */
function manifestInputs(repoRoot: string): string[] | undefined {
  try {
    return declaredProctorInputs(JSON.parse(readFileSync(join(resolve(repoRoot), ...PROCTOR_MANIFEST), "utf8")));
  } catch {
    return undefined;
  }
}

/** 🏗️ Options of {@link captureProctorProduct}. */
export interface CaptureProctorProductOptions {
  readonly host?: ProctorProductHost;
  readonly prune?: boolean;
  readonly inputs?: readonly string[];
  readonly env?: Readonly<Record<string, string | undefined>>;
}

/** 📸️ Turns the JSON output of a completed Cargo build into the product of `mode`: copies the executable to
 * `<mode>/<sha256>/proctor` (atomically; identical bytes keep the one place), reads the dep-info of every local unit, fails
 * closed when `options.inputs` does not declare one of its sources, writes the receipt last, and with `prune` removes every
 * other generation. Returns the receipt. */
export async function captureProctorProduct(repoRoot: string, mode: BuildMode, stdout: string, options: CaptureProctorProductOptions = {}): Promise<ProctorProductReceipt> {
  const host = options.host ?? currentProctorHost(), source = selectProctorExecutable(stdout);
  const paths: string[] = [];
  for (const file of proctorDepInfoFiles(stdout)) {
    try {
      paths.push(...parseDepInfo(await readFile(file, "utf8")));
    } catch (error) {
      if (!missing(error)) throw error;
    }
  }
  const sources = proctorProductSources(repoRoot, paths);
  if (sources.length === 0) throw Error("Cargo's dep-info lists no source of the proctor");
  const uncovered = options.inputs === undefined ? [] : uncoveredProctorSources(sources, options.inputs);
  if (uncovered.length) throw new ProctorProductError("uncovered", proctorProductMessage("uncovered", { count: uncovered.length, example: uncovered[0]! }, options.env ?? process.env));
  const directory = proctorProductDirectory(repoRoot, mode), { sha256, bytes } = await proctorFileDigest(source);
  if (bytes === 0) throw Error("The proctor executable is empty");
  const name = proctorExecutableName(host.platform), place = join(directory, sha256), target = join(place, name);
  await mkdir(directory, { recursive: true });
  const existing = await stat(target).then(facts => facts.size === bytes, () => false);
  if (!existing || (await proctorFileDigest(target)).sha256 !== sha256) {
    const staging = join(directory, `.staging-${process.pid}-${Date.now()}`);
    try {
      await mkdir(staging, { recursive: true });
      await copyFile(source, join(staging, name));
      if (process.platform !== "win32") await chmod(join(staging, name), 0o755);
      if ((await proctorFileDigest(join(staging, name))).sha256 !== sha256) throw Error("The proctor executable changed while it was copied");
      await rm(place, { recursive: true, force: true });
      await rename(staging, place);
    } finally {
      await rm(staging, { recursive: true, force: true });
    }
  }
  const receipt = readProctorProductReceipt({ version: 1, package: PROCTOR_PACKAGE, binary: "proctor", mode, platform: host.platform, architecture: host.architecture, executable: `${sha256}/${name}`, sha256, bytes, sources });
  const next = `${JSON.stringify(receipt, null, 2)}\n`, receiptPath = join(directory, PROCTOR_PRODUCT_RECEIPT);
  if ((await readFile(receiptPath, "utf8").catch(() => undefined)) !== next) {
    const staged = `${receiptPath}.${process.pid}.tmp`;
    await writeFile(staged, next);
    await rename(staged, receiptPath);
  }
  if (options.prune) {
    for (const entry of await readdir(directory)) if (entry !== PROCTOR_PRODUCT_RECEIPT && entry !== sha256) await rm(join(directory, entry), { recursive: true, force: true }).catch(() => undefined);
  }
  return receipt;
}

/** 🚪️ The argument vector of the proctor build for the issued Cargo launcher — the words after `bun repo cargo`. */
export function proctorCargoArguments(mode: BuildMode): string[] {
  return ["--store=shared", `--configuration=${mode}`, "--", "build", "--locked", "--package", PROCTOR_PACKAGE, "--bin", "proctor", "--message-format=json-render-diagnostics"];
}

/** 🏗️ Options of {@link buildProctorProduct}. */
export interface BuildProctorProductOptions {
  readonly mode?: BuildMode;
  readonly signal?: AbortSignal;
  readonly prune?: boolean;
  readonly env?: NodeJS.ProcessEnv;
}

/** 🏗️ `@teaching/proctor:build`: compiles the proctor through the issued Cargo launcher of the shared store (the code of
 * `bun repo cargo --store=shared --configuration=<mode> -- build …`, run in the teaching workspace under this invocation's
 * control, so progress and cancellation are the invocation's) and captures the result with {@link captureProctorProduct}
 * while the store is still retained. */
export async function buildProctorProduct(repoRoot: string, invocation: ScriptInvocation, options: BuildProctorProductOptions = {}): Promise<ProctorProductReceipt> {
  requireIssuedScriptInvocationV1(invocation);
  const root = resolve(repoRoot), env = options.env ?? process.env;
  const mode = proctorBuildMode({ version: 1, originalMode: readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode, ...(options.mode === undefined ? {} : { requestedMode: options.mode }) });
  const environment = scriptInvocationEnvironment(invocation, { ...env, SEMIO_BUILD_MODE: mode });
  const control = createIssuedResourceControlV1(invocation, "proctor:build", 0);
  options.signal?.throwIfAborted();
  const request = readRepositoryCargoLaunchV1(proctorCargoArguments(mode), environment);
  return launchRepositoryCargoV1(invocation, control, root, request, join(root, TEACHING_WORKSPACE), environment, {
    maximumMilliseconds: 0,
    output: "capture",
    limits: { maximumBytes: 16_777_216, maximumLines: 65_536 },
    consume: async (result, context) => {
      if (result.stderr) await writeOwnedCommandOutputV1(process.stderr, result.stderr, context.control);
      if (result.status !== 0 || result.signal !== null) throw Error(`The proctor build ended with ${JSON.stringify({ status: result.status, signal: result.signal })}`);
      options.signal?.throwIfAborted();
      return captureProctorProduct(root, mode, result.stdout, { prune: options.prune ?? true, inputs: manifestInputs(root), env });
    },
  });
}

/** 🔓️ An artifact over a resolved product: it launches private copies of the executable (so a running proctor never locks
 * the product), exports its verified bytes, and joins its children on close. Starts no build. */
export async function openProctorProduct(repoRoot: string, invocation: ScriptInvocation, env: NodeJS.ProcessEnv, signal: AbortSignal | undefined, mode: BuildMode, host: ProctorProductHost = currentProctorHost()): Promise<ProctorArtifact> {
  requireIssuedScriptInvocationV1(invocation);
  const root = resolve(repoRoot), product = await checkProctorProduct(root, mode, host, env);
  signal?.throwIfAborted();
  const sources = await proctorSourceDirectories(root, signal === undefined ? {} : { signal });
  const children = new Set<ProctorProcess>(), consumers = new Set<Promise<unknown>>(), idle = new AbortController().signal;
  let closing: Promise<void> | undefined;
  return Object.freeze({
    sources: Object.freeze(sources),
    launch: (args: readonly string[], childEnvironment: NodeJS.ProcessEnv = env, log?: string): ProctorProcess => {
      if (closing) throw Error("Proctor artifact is closing");
      signal?.throwIfAborted();
      const child = launchProctor(product.executable, root, args, scriptInvocationEnvironment(invocation, childEnvironment), log);
      children.add(child);
      void child.exited.then(() => children.delete(child), () => children.delete(child));
      return child;
    },
    withExecutable: <T>(consume: (executable: ProctorExecutable) => Promise<T>): Promise<T> => {
      if (closing) return Promise.reject(Error("Proctor artifact is closing"));
      const operation = (async () => {
        const readers = new Set<AsyncGenerator<Uint8Array>>(), { receipt } = product;
        let active = true;
        const recheck = async (): Promise<void> => {
          if (!active) throw Error("Proctor executable export retired");
          signal?.throwIfAborted();
        };
        const executable: ProctorExecutable = Object.freeze({
          platform: receipt.platform,
          architecture: receipt.architecture,
          mode,
          bytes: receipt.bytes,
          signal: signal ?? idle,
          recheck,
          chunks: () => {
            const reader = (async function* (): AsyncGenerator<Uint8Array> {
              await recheck();
              const file = await open(product.executable, "r"), hash = createHash("sha256");
              try {
                let total = 0;
                for (;;) {
                  await recheck();
                  const chunk = Buffer.allocUnsafe(65_536), row = await file.read(chunk, 0, chunk.length, null);
                  if (!row.bytesRead) break;
                  total += row.bytesRead;
                  if (total > receipt.bytes) throw Error("Proctor executable exceeded its receipt size");
                  const output = chunk.subarray(0, row.bytesRead);
                  hash.update(output);
                  yield output;
                }
                if (total !== receipt.bytes || hash.digest("hex") !== receipt.sha256) throw Error("Proctor executable bytes changed during export");
                await recheck();
              } finally {
                await file.close();
              }
            })();
            readers.add(reader);
            return reader;
          },
        });
        try {
          const result = await consume(executable);
          await recheck();
          return result;
        } finally {
          active = false;
          const closed = await Promise.allSettled([...readers].map(reader => reader.return(undefined)));
          const failures = closed.filter(row => row.status === "rejected").map(row => (row as PromiseRejectedResult).reason);
          if (failures.length) throw new AggregateError(failures, "Proctor executable exports did not close cleanly");
        }
      })();
      consumers.add(operation);
      void operation.then(() => consumers.delete(operation), () => consumers.delete(operation));
      return operation;
    },
    close: () => closing ??= (async () => {
      const outcomes = await Promise.allSettled([...[...children].map(child => child.stop()), ...consumers]);
      const failures = outcomes.filter(row => row.status === "rejected").map(row => (row as PromiseRejectedResult).reason);
      if (failures.length) throw new AggregateError(failures, "Proctor processes did not settle cleanly");
    })(),
  });
}

/** 🎛️ The acquisition of this run: {@link PROCTOR_ACQUISITION_ENVIRONMENT}, else {@link PROCTOR_ACQUISITION_DEFAULT}. */
export function proctorAcquisition(env: Readonly<Record<string, string | undefined>> = process.env): "product" | "build" {
  const chosen = env[PROCTOR_ACQUISITION_ENVIRONMENT];
  return chosen === "product" || chosen === "build" ? chosen : PROCTOR_ACQUISITION_DEFAULT;
}

/** 🧭️ The one way the course gets the proctor. With the product acquisition it resolves `@teaching/proctor:build`'s receipt for
 * the original build mode (a missing or stale one is a {@link ProctorProductError}); with `rebuild` — the dev stack after a
 * Rust edit — it first runs {@link buildProctorProduct}, which is an explicit rebuild the developer asked for by editing.
 * `requestedMode` can only repeat the original mode. */
export async function acquireProctor(repoRoot: string, invocation: ScriptInvocation, env: NodeJS.ProcessEnv = process.env, signal?: AbortSignal, requestedMode?: BuildMode, options: { readonly rebuild?: boolean } = {}): Promise<ProctorArtifact> {
  if (proctorAcquisition(env) === "build") return buildProctor(repoRoot, invocation, env, signal, requestedMode);
  requireIssuedScriptInvocationV1(invocation);
  const mode = proctorBuildMode({ version: 1, originalMode: readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode, ...(requestedMode === undefined ? {} : { requestedMode }) });
  if (options.rebuild) await buildProctorProduct(repoRoot, invocation, { mode, ...(signal === undefined ? {} : { signal }), prune: false, env });
  return openProctorProduct(repoRoot, invocation, env, signal, mode);
}

/** ▶️ Obtains the proctor with {@link acquireProctor}, runs `proctor <args…>` from a private copy with `env`, waits for it
 * and closes the artifact; a non-zero exit is an error naming the command. An interrupt during the wait stops it. */
export async function runProctor(repoRoot: string, invocation: ScriptInvocation, args: readonly string[], env: NodeJS.ProcessEnv = process.env): Promise<void> {
  const interruption = new AbortController();
  let artifact: ProctorArtifact | undefined;
  let proctor: ProctorProcess | undefined;
  const forward = (signal: NodeJS.Signals): void => {
    interruption.abort();
    proctor?.interrupt(signal);
  };
  for (const signal of LAUNCHER_SIGNALS) process.on(signal, forward);
  try {
    artifact = await acquireProctor(repoRoot, invocation, env, interruption.signal);
    proctor = artifact.launch(args, env);
    const status = await proctor.exited;
    if (status !== 0) throw new Error(`proctor ${args.join(" ")} exited with status ${status}`);
  } finally {
    try { await artifact?.close(); } finally { for (const signal of LAUNCHER_SIGNALS) process.off(signal, forward); }
  }
}

/** 🛠️ `dev` of the proctor alone: settle the development data, saying what was set aside or why it ends here, then obtain the
 * proctor, serve with the dev defaults and wait like {@link runProctor}. */
export async function runDevelopmentProctor(repoRoot: string, invocation: ScriptInvocation, env: NodeJS.ProcessEnv = proctorDevelopmentEnvironment(repoRoot)): Promise<void> {
  if (await developmentDataSettled(repoRoot, env, line => console.warn(`[proctor] ${line}`))) await runProctor(repoRoot, invocation, ["serve"], env);
}
