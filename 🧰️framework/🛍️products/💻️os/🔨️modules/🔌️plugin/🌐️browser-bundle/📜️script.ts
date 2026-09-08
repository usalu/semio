import { createBrowserBundleTests } from "../🧪️tests/🌐️browser-bundle/🟦️.ts";
/** 🌐️ Build-time closure and isolation laws for browser component factories. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { lstatSync, mkdtempSync, mkdirSync, readdirSync, readFileSync, realpathSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";
import { browserWasiInterfaces } from "./🌐️wasi/🟦️.ts";
import { exactExecutableFingerprint, readStableBuildFile, runExactCargoLawProcess } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { canonicalJson } from "../../../../🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

export type BrowserComponentCore = Readonly<{ name: string; bytes: Uint8Array }>;
export type BrowserComponentFactoryControl = Readonly<{ importInterfaces?: readonly string[]; cancelled?: () => boolean; progress?: (completedBytes: number, totalBytes: number) => void }>;
export type BrowserActorPort = import("./🌐️host/🟦️.ts").BrowserHostPort & Readonly<{ wasi?: import("./🌐️wasi/🟦️.ts").BrowserWasiPort }>;
export type ClosedBrowserActorArtifactV1 = Readonly<{ schema: "semio.os.closed-browser-actor.v1"; codegenPolicy: "semio.os.browser-jco-1.27.0-jspi.v1"; policySha256: string; policyCanonical: string; componentSha256: string; sha256: string; byteLength: number; importInterfaces: readonly string[]; bytes: Uint8Array }>;
export type BrowserActorBuildControl = Readonly<{ cancelled?: () => boolean; progress?: (phase: "snapshot" | "policy" | "codegen" | "closure" | "hash", completedBytes: number, totalBytes: number) => void }>;

const browserActorMaximumBytes = 64 * 1024 * 1024;
const browserActorRepoRoot = resolve(import.meta.dir, "../../../../../..");

type SchemaValidator = ((value: unknown) => boolean) & { readonly errors?: unknown };

/** 🧬️ Compiles one named export of the browser-bundle schema module against the repository draft-07 dialect. */
async function browserBundleValidator(exportId: string): Promise<SchemaValidator> {
  const { default: Ajv } = await import("ajv");
  const document = JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`browser bundle schema: unknown export ${exportId}`);
  return validate as SchemaValidator;
}
let browserActorBuildOccupied = false;
const browserActorInterfaces = Object.freeze(["semio:framework/pure@1.0.0", "semio:framework/host-async@1.0.0", ...browserWasiInterfaces].sort());
const browserActorAsyncImports = Object.freeze([
  "wasi:io/poll#poll", "wasi:io/poll#[method]pollable.block",
  "wasi:io/streams#[method]input-stream.blocking-read", "wasi:io/streams#[method]input-stream.blocking-skip",
  "wasi:io/streams#[method]output-stream.blocking-flush", "wasi:io/streams#[method]output-stream.blocking-write-and-flush",
  "wasi:io/streams#[method]output-stream.blocking-write-zeroes-and-flush", "wasi:io/streams#[method]output-stream.blocking-splice",
]);

type BrowserActorCodegenManifest = Readonly<{ version: "1.27.0"; runtime: "bun@1.3.14"; importInterfaces: readonly string[]; files: readonly string[] }>;

/** 🛂️ Admits exact canonical generated-file and interface names independently of the codegen subprocess. */
function parseBrowserActorCodegenManifest(value: unknown): BrowserActorCodegenManifest {
  const denied = (): never => { throw new Error("browser actor artifact: invalid generated manifest"); };
  if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).sort().join(",") !== "files,importInterfaces,runtime,version") return denied();
  const record = value as Record<string, unknown>;
  if (record.version !== "1.27.0" || record.runtime !== "bun@1.3.14") return denied();
  const strings = (value: unknown, maximum: number, valid: (entry: string) => boolean): readonly string[] => {
    if (!Array.isArray(value) || value.length > maximum || !value.every((entry, index) => typeof entry === "string" && valid(entry) && (index === 0 || value[index - 1] < entry))) return denied();
    return Object.freeze([...value]);
  };
  const importInterfaces = strings(record.importInterfaces, browserActorInterfaces.length, name => browserActorInterfaces.includes(name));
  const files = strings(record.files, 65, name => name === "browser-actor.js" || /^browser-actor\.core\d*\.wasm$/.test(name));
  if (files.length < 2 || files.filter(name => name === "browser-actor.js").length !== 1) return denied();
  return Object.freeze({ version: "1.27.0", runtime: "bun@1.3.14", importInterfaces, files });
}

/** 🧊️ Replaces the compiler's exact two file loaders with captured core bytes and rejects residual module IO. */
function closeBrowserCodegenModule(source: string, cores: readonly BrowserComponentCore[]): string {
  const denied = (): never => { throw new Error("browser actor artifact: compiler capsule"); };
  if (Buffer.byteLength(source) > 8 * 1024 * 1024 || /source(?:Mapping)?URL\s*=/.test(source) || cores.length !== 2) return denied();
  const inputs = new Map<string, Uint8Array>();
  let total = 0;
  for (const core of cores) {
    if (!/^js-component-bindgen-component\.core(2)?\.wasm$/.test(core.name) || inputs.has(core.name) || !(core.bytes instanceof Uint8Array) || core.bytes.byteLength < 8 || core.bytes.byteLength > browserActorMaximumBytes - total) return denied();
    total += core.bytes.byteLength;
    inputs.set(core.name, core.bytes);
  }
  if (Buffer.byteLength(source) + Math.ceil(total / 3) * 4 + 1024 > browserActorMaximumBytes) return denied();
  const parse = (text: string): ts.SourceFile => {
    const parsed = ts.createSourceFile("compiler.js", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
    if ((parsed as ts.SourceFile & { parseDiagnostics: readonly unknown[] }).parseDiagnostics.length) return denied();
    return parsed;
  };
  const parsed = parse(source);
  const loaders = parsed.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "fetchCompile");
  if (loaders.length !== 1) return denied();
  const loader = loaders[0], edits: { start: number; end: number; text: string }[] = [];
  const used = new Set<string>();
  let references = 0, imports = 0;
  const visit = (node: ts.Node): void => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier) return denied();
    if (ts.isIdentifier(node) && node.text === "fetchCompile") references++;
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword) {
      if (node.pos < loader.pos || node.end > loader.end || node.arguments.length !== 1 || !ts.isStringLiteral(node.arguments[0]) || node.arguments[0].text !== "node:fs/promises") return denied();
      imports++;
    }
    if (ts.isNewExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "URL") {
      const [name, base] = node.arguments ?? [];
      if (node.arguments?.length !== 2 || !name || !ts.isStringLiteral(name) || !name.text.startsWith("./") || !base || !ts.isPropertyAccessExpression(base) || base.name.text !== "url" || !ts.isMetaProperty(base.expression) || base.expression.keywordToken !== ts.SyntaxKind.ImportKeyword || !ts.isCallExpression(node.parent) || !ts.isIdentifier(node.parent.expression) || node.parent.expression.text !== "fetchCompile" || node.parent.arguments.length !== 1) return denied();
      const key = name.text.slice(2), bytes = inputs.get(key);
      if (!bytes || used.has(key)) return denied();
      used.add(key);
      edits.push({ start: node.getStart(parsed), end: node.end, text: JSON.stringify(Buffer.from(bytes).toString("base64")) });
    }
    ts.forEachChild(node, visit);
  };
  visit(parsed);
  if (used.size !== 2 || references !== 3 || imports !== 1) return denied();
  edits.push({ start: loader.getStart(parsed), end: loader.end, text: "async function fetchCompile(encoded) { const bytes = Uint8Array.from(atob(encoded), value => value.charCodeAt(0)); try { return await WebAssembly.compile(bytes); } finally { bytes.fill(0); } }" });
  let output = source;
  for (const edit of edits.sort((left, right) => right.start - left.start)) output = output.slice(0, edit.start) + edit.text + output.slice(edit.end);
  const closed = parse(output);
  const validate = (node: ts.Node): void => {
    if (ts.isMetaProperty(node) || (ts.isIdentifier(node) && ["fetch", "URL"].includes(node.text)) || (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword) || ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier)) return denied();
    ts.forEachChild(node, validate);
  };
  validate(closed);
  return output;
}

type BrowserCodegenSourceDigest = Readonly<{ logicalPath: string; sha256: string; byteLength: number }>;
type BrowserActorRuntimeSnapshot = ReadonlyMap<string, Readonly<{ bytes: Uint8Array; row: BrowserCodegenSourceDigest }>>;

/** 🫙️ Captures the only two first-party actor runtime modules before asynchronous code generation. */
function captureBrowserActorRuntime(root: string, check: () => void): BrowserActorRuntimeSnapshot {
  const admission = { remaining: 1024 * 1024 };
  return new Map(["host", "wasi"].map(name => {
    const logicalPath = `🌐️${name}/🟦️.ts`;
    const bytes = readStableBuildFile(join(root, logicalPath), 512 * 1024, admission, check);
    return [name, Object.freeze({ bytes, row: Object.freeze({ logicalPath: "browser/" + logicalPath, sha256: createHash("sha256").update(bytes).digest("hex"), byteLength: bytes.byteLength }) })];
  }));
}

/** 🔏️ Owns a canonical immutable policy value and its content digest independently of evidence files. */
function sealBrowserCodegenPolicy<T>(input: T): Readonly<{ record: T; canonical: string; sha256: string }> {
  const canonical = canonicalJson(input);
  if (Buffer.byteLength(canonical) > 256 * 1024) throw new Error("browser actor artifact: policy bound");
  const record = JSON.parse(canonical) as T;
  const freeze = (value: unknown): void => {
    if (!value || typeof value !== "object") return;
    for (const child of Object.values(value)) freeze(child);
    Object.freeze(value);
  };
  freeze(record);
  return Object.freeze({ record, canonical, sha256: createHash("sha256").update(canonical).digest("hex") });
}

/** 🧭️ Captures the fixed build-host policy inputs independently of caller roots and scratch evidence. */
function captureBrowserCodegenPolicyInputs(runtime: BrowserActorRuntimeSnapshot, check: () => void) {
  const admission = { remaining: browserActorMaximumBytes };
  const read = (path: string) => readStableBuildFile(path, 16 * 1024 * 1024, admission, check);
  const hash = (bytes: Uint8Array | string) => createHash("sha256").update(bytes).digest("hex");
  const lockBytes = read(join(browserActorRepoRoot, "bun.lock"));
  const lock = ts.parseConfigFileTextToJson("bun.lock", new TextDecoder("utf-8", { fatal: true }).decode(lockBytes));
  if (lock.error || !lock.config?.packages) throw new Error("browser actor artifact: invalid toolchain lock");
  const locked = (name: string, version: string) => {
    const row = lock.config.packages[name];
    if (!Array.isArray(row) || row[0] !== `${name}@${version}`) throw new Error("browser actor artifact: unqualified locked tool");
    return hash(canonicalJson(row));
  };
  const packageRoots = [
    { name: "@bytecodealliance/jco", version: "1.27.0", path: dirname(dirname(fileURLToPath(import.meta.resolve("@bytecodealliance/jco/component")))) },
    { name: "@bytecodealliance/jco-transpile", version: "0.6.1", path: dirname(dirname(fileURLToPath(import.meta.resolve("@bytecodealliance/jco-transpile/component")))) },
    { name: "@bytecodealliance/preview2-shim", version: "0.20.1", path: dirname(dirname(dirname(fileURLToPath(import.meta.resolve("@bytecodealliance/preview2-shim/io"))))) },
  ];
  const packages = packageRoots.map(({ name, version, path }) => {
    const bytes = read(join(path, "package.json")), manifest = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
    if (manifest.name !== name || manifest.version !== version) throw new Error("browser actor artifact: unqualified tool manifest");
    return Object.freeze({ name, version, manifestSha256: hash(bytes), lockSha256: locked(name, version) });
  });
  const parserPath = fileURLToPath(import.meta.resolve("typescript"));
  const parserBytes = read(parserPath), parserManifestBytes = read(join(dirname(parserPath), "../package.json"));
  const parserManifest = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(parserManifestBytes));
  if (parserManifest.name !== "typescript" || parserManifest.version !== "5.9.3" || ts.version !== "5.9.3") throw new Error("browser actor artifact: unqualified parser");
  const parser = { name: "typescript", version: "5.9.3", manifestSha256: hash(parserManifestBytes), lockRowSha256: locked("typescript", "5.9.3"), entry: { logicalPath: "typescript/lib/typescript.js", sha256: hash(parserBytes), byteLength: parserBytes.byteLength } };
  const paths = [
    { logicalPath: "browser/📜️script.ts", path: fileURLToPath(import.meta.url) },
    { logicalPath: "browser/🧬️schema/🔣️.json", path: join(import.meta.dir, "🧬️schema/🔣️.json") },
    { logicalPath: "repo/execution/🟦️.ts", path: resolve(import.meta.dir, "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts") },
    { logicalPath: "repo/normalization/🟦️.ts", path: resolve(import.meta.dir, "../../../../🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts") },
  ];
  const firstParty = [...runtime.values()].map(value => value.row);
  for (const { logicalPath, path } of paths) {
    const bytes = read(path);
    firstParty.push(Object.freeze({ logicalPath, sha256: hash(bytes), byteLength: bytes.byteLength }));
  }
  firstParty.sort((left, right) => left.logicalPath < right.logicalPath ? -1 : left.logicalPath > right.logicalPath ? 1 : 0);
  return Object.freeze({ packages: Object.freeze(packages), parser: Object.freeze(parser), firstParty: Object.freeze(firstParty) });
}

/** 📚️ Supplies bounded captured JS to Bun and proves every metafile input was captured exactly once. */
async function captureBrowserCodegenSources(entrypoint: string, roots: readonly Readonly<{ name: string; path: string }>[], check: () => void, captured?: (path: string) => void): Promise<Readonly<{ source: string; inputs: readonly BrowserCodegenSourceDigest[] }>> {
  check();
  const admittedRoots = roots.map(root => ({ ...root, path: realpathSync(root.path) }));
  const buildCwd = process.cwd();
  const snapshots = new Map<string, Readonly<{ bytes: Uint8Array; row: BrowserCodegenSourceDigest }>>();
  const admission = { remaining: 8 * 1024 * 1024 };
  const result = await Bun.build({ entrypoints: [entrypoint], root: admittedRoots[0].path, target: "browser", format: "esm", splitting: false, minify: false, write: false, metafile: true, external: ["node:fs/promises"], plugins: [{ name: "semio-browser-codegen-snapshot", setup(build) {
    build.onLoad({ filter: /.*/, namespace: "file" }, args => {
      check();
      if (!args.path.endsWith(".js")) throw new Error("browser actor artifact: compiler source type");
      const path = realpathSync(args.path);
      const root = admittedRoots.find(root => path.startsWith(root.path + sep));
      if (!root) throw new Error("browser actor artifact: compiler source root");
      let snapshot = snapshots.get(path);
      if (!snapshot) {
        if (snapshots.size >= 128) throw new Error("browser actor artifact: compiler source count");
        const bytes = readStableBuildFile(path, 8 * 1024 * 1024, admission, check);
        const row = Object.freeze({ logicalPath: root.name + "/" + relative(root.path, path).split(sep).join("/"), sha256: createHash("sha256").update(bytes).digest("hex"), byteLength: bytes.byteLength });
        snapshot = Object.freeze({ bytes, row });
        snapshots.set(path, snapshot);
        captured?.(path);
      }
      return { contents: snapshot.bytes, loader: "js" };
    });
  } }] }).catch(cause => { check(); throw new Error("browser actor artifact: compiler source closure failed", { cause }); });
  check();
  if (!result.success || result.outputs.length !== 1 || !result.metafile) throw new Error("browser actor artifact: compiler source closure failed");
  const paths = Object.keys(result.metafile.inputs).map(path => isAbsolute(path) ? path : resolve(buildCwd, path)).sort();
  if (JSON.stringify(paths) !== JSON.stringify([...snapshots.keys()].sort())) throw new Error("browser actor artifact: compiler source coverage");
  if (result.outputs[0].size > 8 * 1024 * 1024) throw new Error("browser actor artifact: compiler source output bound");
  return Object.freeze({ source: await result.outputs[0].text(), inputs: Object.freeze([...snapshots.values()].map(snapshot => snapshot.row).sort((left, right) => left.logicalPath < right.logicalPath ? -1 : left.logicalPath > right.logicalPath ? 1 : 0)) });
}

/** 🧬️ Closes the public browser compiler entry with its two component cores in one private build directory. */
async function buildBrowserCodegenModule(evidence: string, check: () => void): Promise<Readonly<{ path: string; sha256: string; byteLength: number; inputs: readonly BrowserCodegenSourceDigest[]; cores: readonly BrowserCodegenSourceDigest[] }>> {
  check();
  const entrypoint = fileURLToPath(import.meta.resolve("@bytecodealliance/jco/component"));
  const vendor = fileURLToPath(import.meta.resolve("@bytecodealliance/jco-transpile/component"));
  const shim = dirname(dirname(fileURLToPath(import.meta.resolve("@bytecodealliance/preview2-shim/io"))));
  const packageBytes = readStableBuildFile(join(dirname(entrypoint), "../package.json"), 64 * 1024, { remaining: 64 * 1024 }, check);
  if (JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(packageBytes)).version !== "1.27.0") throw new Error("browser actor artifact: unqualified JCO version");
  const { source, inputs } = await captureBrowserCodegenSources(entrypoint, [
    { name: "@bytecodealliance/jco/dist", path: dirname(entrypoint) },
    { name: "@bytecodealliance/jco-transpile/vendor", path: dirname(vendor) },
    { name: "@bytecodealliance/preview2-shim/dist/browser", path: join(shim, "browser") },
  ], check);
  writeFileSync(join(evidence, "compiler-sources.json"), JSON.stringify(inputs), { mode: 0o600 });
  if (Buffer.byteLength(source) > 8 * 1024 * 1024) throw new Error("browser actor artifact: compiler closure bound");
  const admission = { remaining: browserActorMaximumBytes - Buffer.byteLength(source) };
  const cores = ["js-component-bindgen-component.core.wasm", "js-component-bindgen-component.core2.wasm"].map(name => ({ name, bytes: readStableBuildFile(join(dirname(vendor), name), browserActorMaximumBytes, admission, check) }));
  const closed = closeBrowserCodegenModule(source, cores);
  const path = join(evidence, "compiler.mjs");
  writeFileSync(path, closed, { mode: 0o600 });
  return Object.freeze({ path, sha256: createHash("sha256").update(closed).digest("hex"), byteLength: Buffer.byteLength(closed), inputs, cores: Object.freeze(cores.map(core => Object.freeze({ logicalPath: "@bytecodealliance/jco-transpile/vendor/" + core.name, sha256: createHash("sha256").update(core.bytes).digest("hex"), byteLength: core.bytes.byteLength }))) });
}

/** 🏗️ Derives closed actor bytes without caller-selected paths or compiler authority. */
export async function buildClosedBrowserActorArtifactV1(component: Uint8Array, control: BrowserActorBuildControl = {}): Promise<ClosedBrowserActorArtifactV1> {
  if (!control || typeof control !== "object" || Array.isArray(control) || ![Object.prototype, null].includes(Object.getPrototypeOf(control)) || !Reflect.ownKeys(control).every(key => {
    const field = Object.getOwnPropertyDescriptor(control, key)!;
    return (key === "cancelled" || key === "progress") && "value" in field && (field.value === undefined || typeof field.value === "function");
  })) throw new Error("browser actor artifact: invalid build control");
  return buildClosedBrowserActorArtifactOwned(component, Object.freeze({ cancelled: control.cancelled, progress: control.progress }));
}

/** 🗝️ Retains the one build owner; only in-module laws may retain diagnostic scratch. */
async function buildClosedBrowserActorArtifactOwned(component: Uint8Array, control: BrowserActorBuildControl, evidenceRoot?: string): Promise<ClosedBrowserActorArtifactV1> {
  const check = () => { if (control.cancelled?.()) throw new Error("browser actor artifact: cancelled"); };
  check();
  if (browserActorBuildOccupied) throw new Error("browser actor artifact: build capacity");
  browserActorBuildOccupied = true;
  let snapshot: Uint8Array | undefined, evidence: string | undefined;
  try {
    if (typeof Bun === "undefined" || Bun.version !== "1.3.14" || JSON.parse(readFileSync(join(browserActorRepoRoot, "package.json"), "utf8")).packageManager !== "bun@1.3.14") throw new Error("browser actor artifact: unqualified build runtime");
    if (!(component instanceof Uint8Array) || component.byteLength < 8 || component.byteLength > browserActorMaximumBytes || ![0, 97, 115, 109, 13, 0, 1, 0].every((value, index) => component[index] === value)) throw new Error("browser actor artifact: component header or byte bound");
    snapshot = Uint8Array.from(component);
    const actorRuntime = captureBrowserActorRuntime(import.meta.dir, check);
    const componentSha256 = createHash("sha256").update(snapshot).digest("hex");
    control.progress?.("snapshot", snapshot.byteLength, snapshot.byteLength);
    check();
    const executable = exactExecutableFingerprint(realpathSync(process.execPath), { cancelled: control.cancelled, progress: (completed, total) => control.progress?.("policy", completed, total) });
    const policyInputs = captureBrowserCodegenPolicyInputs(actorRuntime, check);
    const scratchRoot = evidenceRoot ?? process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
    mkdirSync(scratchRoot, { recursive: true });
    evidence = mkdtempSync(join(scratchRoot, "browser-actor-codegen-"));
    const componentPath = join(evidence, "component.wasm");
    writeFileSync(componentPath, snapshot, { mode: 0o600 });
    const configPath = join(evidence, "codegen.bunfig.toml");
    writeFileSync(configPath, "", { mode: 0o600 });
    const environment: Record<string, string> = { LANG: "C", LC_ALL: "C", TZ: "UTC", NO_COLOR: "1", TMPDIR: evidence, TMP: evidence, TEMP: evidence };
    if (process.platform === "win32" && process.env.SystemRoot) environment.SystemRoot = process.env.SystemRoot;
    control.progress?.("codegen", 0, snapshot.byteLength);
    check();
    const compiler = await buildBrowserCodegenModule(evidence, check);
    const policy = sealBrowserCodegenPolicy({
      schema: "semio.os.browser-codegen-policy.v1", revision: 1,
      runtime: { kind: "bun", version: "1.3.14", executable: { sha256: executable.sha256, byteLength: executable.byteLength } },
      compiler: { module: { sha256: compiler.sha256, byteLength: compiler.byteLength }, inputs: compiler.inputs, cores: compiler.cores },
      ...policyInputs,
      options: { jco: "1.27.0", entrypoint: "@bytecodealliance/jco/component", target: "browser", format: "esm", name: "browser-actor", instantiation: "async", asyncMode: "jspi", nodejsCompat: false, base64Cutoff: 0, importInterfaces: browserActorInterfaces, asyncImports: browserActorAsyncImports },
    });
    writeFileSync(join(evidence, "codegen-policy.json"), policy.canonical, { mode: 0o600 });
    check();
    const generated = await runExactCargoLawProcess(executable.path, ["--no-install", "--no-env-file", "--conditions=browser", "--config=" + configPath, "--eval", `
      import assert from "node:assert/strict";
      import { closeSync, fstatSync, lstatSync, openSync, readSync, writeFileSync } from "node:fs";
      import { createHash } from "node:crypto";
      import { join } from "node:path";
      assert.equal(globalThis.Bun?.version, "1.3.14", "browser actor artifact: unqualified codegen runtime");
      assert(["NODE_OPTIONS", "NODE_PATH", "BUN_OPTIONS", "BUN_PRELOAD", "PATH"].every(name => process.env[name] === undefined), "browser actor artifact: ambient codegen environment");
      const [componentPath, outputRoot, importsJson, asyncJson, maximum, compilerPath, compilerHash, compilerLength, componentHash, componentLength] = process.argv.slice(1);
      const readCaptured = ${readStableBuildFile.toString()};
      const capture = (path, hash, length) => {
        const bytes = readCaptured(path, Number(maximum), { remaining: Number(maximum) }, () => {});
        assert.equal(bytes.byteLength, Number(length), "browser actor artifact: captured input identity");
        assert.equal(createHash("sha256").update(bytes).digest("hex"), hash, "browser actor artifact: captured input identity");
        return bytes;
      };
      const compilerBytes = capture(compilerPath, compilerHash, compilerLength);
      const componentBytes = capture(componentPath, componentHash, componentLength);
      const compilerUrl = URL.createObjectURL(new Blob([compilerBytes], { type: "text/javascript" }));
      let generate;
      try { ({ generate } = await import(compilerUrl)); }
      finally { URL.revokeObjectURL(compilerUrl); compilerBytes.fill(0); }
      const version = "1.27.0";
      const admitted = JSON.parse(importsJson), asyncImports = JSON.parse(asyncJson);
      const output = await generate(componentBytes, { name: "browser-actor", instantiation: { tag: "async" }, noNodejsCompat: true, base64Cutoff: 0, map: admitted.map(name => [name, name]), asyncMode: { tag: "jspi", val: { imports: asyncImports, exports: [] } }, noTypescript: false, tlaCompat: false, validLiftingOptimization: false, tracing: false, noNamespacedExports: false, multiMemory: false, bindgenEnableWasmExnref: false, strict: false, asmjs: false });
      componentBytes.fill(0);
      const result = { imports: output.imports, files: Object.fromEntries(output.files) };
      assert(Array.isArray(result.imports) && new Set(result.imports).size === result.imports.length && result.imports.every(name => admitted.includes(name)), "browser actor artifact: unsupported import interface");
      const files = Object.entries(result.files).filter(([name]) => !name.endsWith(".d.ts"));
      assert(files.length >= 2 && files.length <= 65 && files.filter(([name]) => name === "browser-actor.js").length === 1, "browser actor artifact: generated file count");
      let total = 0;
      for (const [name, bytes] of files) {
        assert(name === "browser-actor.js" || /^browser-actor\\.core\\d*\\.wasm$/.test(name), "browser actor artifact: generated file path");
        assert(bytes instanceof Uint8Array && bytes.byteLength <= (name.endsWith(".js") ? 8 * 1024 * 1024 : Number(maximum)), "browser actor artifact: generated file bound");
        total += bytes.byteLength;
        assert(total <= Number(maximum), "browser actor artifact: generated byte bound");
        writeFileSync(join(outputRoot, name), bytes, { mode: 0o600 });
      }
      process.stdout.write(JSON.stringify({ version, runtime: "bun@" + Bun.version, importInterfaces: result.imports.sort(), files: files.map(([name]) => name).sort() }));
    `, componentPath, evidence, JSON.stringify(browserActorInterfaces), JSON.stringify(browserActorAsyncImports), String(browserActorMaximumBytes), compiler.path, compiler.sha256, String(compiler.byteLength), componentSha256, String(snapshot.byteLength)], { cwd: browserActorRepoRoot, env: environment, budgetMs: 600_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "codegen.stdout.json"), stderrPath: join(evidence, "codegen.stderr"), cancelled: () => control.cancelled?.() ?? false });
    check();
    if (canonicalJson(exactExecutableFingerprint(executable.path, { cancelled: control.cancelled })) !== canonicalJson(executable)) throw new Error("browser actor artifact: build runtime changed");
    if (generated.status !== 0) throw new Error(`browser actor artifact: codegen ${generated.reason}: ${generated.stderr.slice(0, 4096)}`);
    const manifest = parseBrowserActorCodegenManifest(JSON.parse(generated.stdout));
    control.progress?.("codegen", snapshot.byteLength, snapshot.byteLength);
    const admission = { remaining: browserActorMaximumBytes };
    const read = (name: string, maximum: number): Uint8Array => readStableBuildFile(join(evidence, name), maximum, admission, check);
    const source = new TextDecoder("utf-8", { fatal: true }).decode(read("browser-actor.js", 8 * 1024 * 1024));
    const cores = manifest.files.filter(name => name.endsWith(".wasm")).map(name => ({ name, bytes: read(name, browserActorMaximumBytes) }));
    const output = await closedBrowserActorBundleFromRuntime(source, cores, { importInterfaces: manifest.importInterfaces, cancelled: control.cancelled, progress: (completed, total) => control.progress?.("closure", completed, total) }, actorRuntime);
    check();
    const byteLength = Buffer.byteLength(output);
    if (byteLength > browserActorMaximumBytes) throw new Error("browser actor artifact: closed byte bound");
    const bytes = new TextEncoder().encode(output);
    const sha256 = createHash("sha256").update(bytes).digest("hex");
    control.progress?.("hash", byteLength, byteLength);
    check();
    return Object.freeze({ schema: "semio.os.closed-browser-actor.v1", codegenPolicy: "semio.os.browser-jco-1.27.0-jspi.v1", policySha256: policy.sha256, policyCanonical: policy.canonical, componentSha256, sha256, byteLength, importInterfaces: Object.freeze([...manifest.importInterfaces]), bytes });
  } finally {
    snapshot?.fill(0);
    try { if (evidenceRoot === undefined && evidence !== undefined) rmSync(evidence, { recursive: true, force: true, maxRetries: 3 }); }
    finally { browserActorBuildOccupied = false; }
  }
}

/** 🪂️ Requires actual generated suspension wrappers for every used blocking Preview2 operation. */
function validateWasiSuspension(source: string): void {
  const parsed = ts.createSourceFile("actor-input.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const functions = new Map<string, string>(), asynchronous = new Map<string, boolean>();
  const declarations = new Map<string, ts.VariableDeclaration[]>(), rebound = new Set<string>(), coreBindings = new Set<string>();
  const denied = (): never => { throw new Error("browser actor bundle: blocking WASI import requires JSPI suspension"); };
  const lowerBinding = (node: ts.Expression): string | undefined => {
    if (!ts.isCallExpression(node) || !ts.isPropertyAccessExpression(node.expression) || !ts.isIdentifier(node.expression.expression) || !["_lowerImport", "_lowerImportBackwardsCompat"].includes(node.expression.expression.text) || node.expression.name.text !== "bind" || node.arguments.length !== 2 || node.arguments[0].kind !== ts.SyntaxKind.NullKeyword || !ts.isObjectLiteralExpression(node.arguments[1])) return;
    const bindings = node.arguments[1].properties.filter(property => ts.isPropertyAssignment(property) && (ts.isIdentifier(property.name) || ts.isStringLiteral(property.name)) && property.name.text === "importFn");
    if (bindings.length !== 1 || !ts.isPropertyAssignment(bindings[0]) || !ts.isIdentifier(bindings[0].initializer)) return;
    return bindings[0].initializer.text;
  };
  const coreImports = (node: ts.ObjectLiteralExpression): void => {
    for (const property of node.properties) {
      if (!ts.isPropertyAssignment(property)) continue;
      if (ts.isIdentifier(property.initializer)) coreBindings.add(property.initializer.text);
      else if (ts.isObjectLiteralExpression(property.initializer)) coreImports(property.initializer);
    }
  };
  const visit = (node: ts.Node): void => {
    if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name)) {
      const entries = declarations.get(node.name.text) ?? [];
      entries.push(node);
      declarations.set(node.name.text, entries);
    }
    if (ts.isBinaryExpression(node) && node.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && node.operatorToken.kind <= ts.SyntaxKind.LastAssignment) {
      if (ts.isIdentifier(node.left)) rebound.add(node.left.text);
      if (ts.isPropertyAccessExpression(node.left) && ts.isIdentifier(node.left.expression)) {
        const name = node.left.expression.text;
        if (node.left.name.text === "fnName" && ts.isStringLiteral(node.right)) {
          if (functions.has(name)) denied();
          functions.set(name, node.right.text);
        }
        if (node.left.name.text === "manuallyAsync") {
          if (asynchronous.has(name) || node.operatorToken.kind !== ts.SyntaxKind.EqualsToken) denied();
          asynchronous.set(name, node.right.kind === ts.SyntaxKind.TrueKeyword);
        }
      }
    }
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "instantiateCore" && node.arguments[1] && ts.isObjectLiteralExpression(node.arguments[1])) coreImports(node.arguments[1]);
    ts.forEachChild(node, visit);
  };
  visit(parsed);
  for (const [name, operation] of functions) {
    if (!/^wasi:io\/(?:poll|streams)@[0-9]+\.[0-9]+\.[0-9]+#(?:poll|block|blockingRead|blockingSkip|blockingFlush|blockingWriteAndFlush|blockingWriteZeroesAndFlush|blockingSplice)$/.test(operation)) continue;
    if (!asynchronous.get(name) || rebound.has(name) || declarations.get(name)?.length !== 1) denied();
    const wrappers: string[] = [];
    for (const [binding, entries] of declarations) for (const entry of entries) {
      const value = entry.initializer;
      if (!value || !ts.isConditionalExpression(value) || !ts.isPropertyAccessExpression(value.condition) || !ts.isIdentifier(value.condition.expression) || value.condition.expression.text !== name || value.condition.name.text !== "manuallyAsync") continue;
      if (!ts.isNewExpression(value.whenTrue) || value.whenTrue.expression.getText(parsed) !== "WebAssembly.Suspending" || value.whenTrue.arguments?.length !== 1 || lowerBinding(value.whenTrue.arguments[0]) !== name || lowerBinding(value.whenFalse) !== name || entries.length !== 1 || rebound.has(binding)) denied();
      wrappers.push(binding);
    }
    if (wrappers.length !== 1 || !coreBindings.has(wrappers[0])) denied();
  }
}

/** 🎭️ Builds one closed actor module with fresh first-party host state and an explicit owner. */
export async function closedBrowserActorBundle(source: string, cores: readonly BrowserComponentCore[], control: BrowserComponentFactoryControl = {}): Promise<string> {
  const check = () => { if (control.cancelled?.()) throw new Error("browser actor bundle: cancelled"); };
  return closedBrowserActorBundleFromRuntime(source, cores, control, captureBrowserActorRuntime(import.meta.dir, check));
}

/** 🎬️ Closes the generated actor over exact retained runtime modules without reopening source paths. */
async function closedBrowserActorBundleFromRuntime(source: string, cores: readonly BrowserComponentCore[], control: BrowserComponentFactoryControl, runtime: BrowserActorRuntimeSnapshot): Promise<string> {
  const names = control.importInterfaces ?? [];
  const known = { "semio:framework/pure@1.0.0": "pure", "semio:framework/host-async@1.0.0": "hostAsync" };
  if (names.some(name => !Object.hasOwn(known, name) && !browserWasiInterfaces.includes(name))) throw new Error("browser actor bundle: unsupported import interface");
  const wasiRequired = names.some(name => browserWasiInterfaces.includes(name));
  if (wasiRequired) validateWasiSuspension(source);
  const component = await closedBrowserComponentFactory(source, cores, control);
  const imports = names.map(name => `${JSON.stringify(name)}: ${browserWasiInterfaces.includes(name) ? `wasi.imports[${JSON.stringify(name)}]` : `host.${known[name as keyof typeof known]}`}`).join(",");
  const contents = `import { createBrowserHostActivation } from "semio:actor-host";
import { createBrowserWasiActivation } from "semio:actor-wasi";
${component}
export async function activate(identity, port, control = {}) {
  const host = createBrowserHostActivation(identity, port, control.signal);
  let component, wasi, closeOnExit = () => {};
  try {
    if (${wasiRequired}) {
      if (typeof WebAssembly.Suspending !== "function" || typeof WebAssembly.promising !== "function") throw new Error("browser actor bundle: JSPI unavailable");
      if (!port.wasi || typeof port.wasi.nowNs !== "function" || typeof port.wasi.write !== "function") throw new Error("browser actor bundle: WASI port required");
      wasi = createBrowserWasiActivation({ nowNs: () => port.wasi.nowNs(), write: (channel, bytes) => port.wasi.write(channel, bytes), exit(status) { closeOnExit(); port.wasi.exit?.(status); } }, control.signal);
    }
    component = await instantiateFreshComponent({${imports}}, control);
  }
  catch (error) {
    const results = await Promise.allSettled([Promise.resolve().then(() => host.close()), Promise.resolve().then(() => wasi?.close())]);
    const errors = results.flatMap(result => result.status === "rejected" ? [result.reason] : []);
    if (errors.length) throw new AggregateError([error, ...errors], "browser actor bundle: activation and retirement failed");
    throw error;
  }
  const active = new Set();
  let phase = "open", closing;
  const close = () => {
    if (closing) return closing;
    phase = "closing";
    control.signal?.removeEventListener("abort", onAbort);
    closing = Promise.resolve().then(async () => {
      try {
        const results = await Promise.allSettled([Promise.resolve().then(() => host.close()), Promise.resolve().then(() => wasi?.close()), ...active]);
        const errors = results.slice(0, 2).flatMap(result => result.status === "rejected" ? [result.reason] : []);
        if (errors.length) throw new AggregateError(errors, "browser actor bundle: retirement failed");
      } finally { component = null; phase = "closed"; }
    });
    closing.catch(() => {});
    return closing;
  };
  closeOnExit = () => { void close(); };
  const onAbort = () => { void close(); };
  control.signal?.addEventListener("abort", onAbort, { once: true });
  if (control.signal?.aborted) { await close(); throw new Error("browser actor bundle: cancelled"); }
  const invoke = async (path, args) => {
    if (phase !== "open") throw new Error("browser actor bundle: closed");
    if (!Array.isArray(path) || path.length < 1 || path.length > 2 || path.some(name => typeof name !== "string" || name.length > 256) || !Array.isArray(args) || args.length > 64) throw new Error("browser actor bundle: invalid invocation");
    if (active.size >= 32) throw new Error("browser actor bundle: invocation capacity");
    let owner = component;
    if (path.length === 2) {
      if (!Object.hasOwn(owner, path[0])) throw new Error("browser actor bundle: unknown interface");
      owner = owner[path[0]];
    }
    const name = path[path.length - 1];
    if (!owner || !Object.hasOwn(owner, name) || typeof owner[name] !== "function") throw new Error("browser actor bundle: unknown operation");
    const operation = Promise.resolve().then(() => {
      if (phase !== "open") throw new Error("browser actor bundle: closed");
      return owner[name](...args);
    });
    active.add(operation);
    try { return await operation; }
    catch (error) { throw host.invocationFailure(error); }
    finally { active.delete(operation); }
  };
  return Object.freeze({ invoke, close, progress: () => Object.freeze({ phase, activeInvocations: active.size }), resolveEffect: host.resolveEffect, rejectEffect: host.rejectEffect });
}
`;
  if (control.cancelled?.()) throw new Error("browser actor bundle: cancelled");
  const loadedRuntime = new Set<string>();
  const build = await Bun.build({ entrypoints: ["semio:closed-browser-actor"], target: "browser", format: "esm", splitting: false, minify: false, write: false, plugins: [{ name: "semio-closed-browser-actor", setup(builder) {
    builder.onResolve({ filter: /^semio:closed-browser-actor$/ }, () => ({ path: "actor.ts", namespace: "semio-actor" }));
    builder.onResolve({ filter: /^semio:actor-(host|wasi)$/ }, args => ({ path: args.path.slice("semio:actor-".length), namespace: "semio-runtime" }));
    builder.onResolve({ filter: /.*/, namespace: "semio-runtime" }, () => { throw new Error("browser actor bundle: runtime import denied"); });
    builder.onLoad({ filter: /.*/, namespace: "file" }, () => { throw new Error("browser actor bundle: runtime file denied"); });
    builder.onLoad({ filter: /.*/, namespace: "semio-actor" }, () => ({ contents, loader: "ts" }));
    builder.onLoad({ filter: /.*/, namespace: "semio-runtime" }, args => {
      const captured = runtime.get(args.path);
      if (!captured || loadedRuntime.has(args.path)) throw new Error("browser actor bundle: runtime snapshot denied");
      loadedRuntime.add(args.path);
      return { contents: captured.bytes, loader: "ts" };
    });
  } }] });
  if (!build.success || build.outputs.length !== 1 || loadedRuntime.size !== 2) throw new Error(`browser actor bundle: build failed ${build.logs.map(log => log.message).join("; ")}`);
  if (control.cancelled?.()) throw new Error("browser actor bundle: cancelled");
  const output = await build.outputs[0].text();
  const parsed = ts.createSourceFile("actor.js", output, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  if ((parsed as unknown as { parseDiagnostics: unknown[] }).parseDiagnostics.length) throw new Error("browser actor bundle: invalid output");
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) || (ts.isExportDeclaration(node) && node.moduleSpecifier) || (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword) || (ts.isMetaProperty(node) && node.keywordToken === ts.SyntaxKind.ImportKeyword)) throw new Error("browser actor bundle: open module graph");
    ts.forEachChild(node, visit);
  };
  visit(parsed);
  return output;
}

/** 🧊️ Closes a JCO instantiation function over exact immutable core bytes, preserving per-call guest state. */
export async function closedBrowserComponentFactory(source: string, cores: readonly BrowserComponentCore[], control: BrowserComponentFactoryControl = {}): Promise<string> {
  const check = () => { if (control.cancelled?.()) throw new Error("browser component factory: cancelled"); };
  check();
  if (Buffer.byteLength(source) > 8 * 1024 * 1024 || cores.length < 1 || cores.length > 64) throw new Error("browser component factory: source/core bound");
  const parsed = ts.createSourceFile("component.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  if ((parsed as unknown as { parseDiagnostics: unknown[] }).parseDiagnostics.length) throw new Error("browser component factory: invalid source");
  let factory: ts.FunctionDeclaration | undefined;
  for (const statement of parsed.statements) {
    if (ts.isExpressionStatement(statement) && ts.isStringLiteral(statement.expression) && statement.expression.text === "use components") continue;
    if (ts.isEmptyStatement(statement)) continue;
    if (ts.isFunctionDeclaration(statement) && statement.name?.text === "instantiate" && !factory) { factory = statement; continue; }
    if (ts.isVariableStatement(statement) && statement.declarationList.declarations.length === 1) {
      const declaration = statement.declarationList.declarations[0];
      if (ts.isIdentifier(declaration.name) && declaration.name.text === "_util" && declaration.initializer && ts.isObjectLiteralExpression(declaration.initializer) && declaration.initializer.properties.length === 0) continue;
    }
    throw new Error("browser component factory: unexpected top-level declaration");
  }
  if (!factory?.body || factory.parameters.length !== 3 || factory.parameters[0].name.getText(parsed) !== "getCoreModule" || factory.parameters[1].name.getText(parsed) !== "imports" || factory.parameters[2].name.getText(parsed) !== "instantiateCore") throw new Error("browser component factory: instantiation signature");
  if (factory.asteriskToken || factory.modifiers?.some(modifier => ![ts.SyntaxKind.ExportKeyword, ts.SyntaxKind.AsyncKeyword].includes(modifier.kind))) throw new Error("browser component factory: instantiation modifier");
  const modifier = factory.modifiers?.some(modifier => modifier.kind === ts.SyntaxKind.AsyncKeyword) ? "async " : "";
  const edits: { start: number; end: number }[] = [];
  for (const statement of factory.body.statements) {
    if (ts.isVariableStatement(statement) && statement.declarationList.declarations.length === 1 && statement.declarationList.declarations[0].name.getText(parsed) === "fetchCompile") {
      const initializer = statement.declarationList.declarations[0].initializer;
      if (!initializer || initializer.getText(parsed).replaceAll(/\s+/g, "") !== "url=>fetch(url).then(WebAssembly.compileStreaming)") throw new Error("browser component factory: unrecognized fetch fallback");
      edits.push({ start: statement.getStart(parsed), end: statement.end });
    }
    if (ts.isIfStatement(statement) && statement.expression.getText(parsed) === "!getCoreModule") {
      const body = statement.thenStatement;
      if (statement.elseStatement || !ts.isExpressionStatement(body) || !ts.isBinaryExpression(body.expression) || body.expression.left.getText(parsed) !== "getCoreModule" || body.expression.operatorToken.kind !== ts.SyntaxKind.EqualsToken || !ts.isArrowFunction(body.expression.right) || !body.expression.right.body.getText(parsed).startsWith("fetchCompile(new URL(")) throw new Error("browser component factory: unrecognized module fallback");
      edits.push({ start: statement.getStart(parsed), end: statement.end });
    }
  }
  if (edits.length !== 2) throw new Error("browser component factory: missing explicit resolver boundary");
  let body = source.slice(factory.body.getStart(parsed), factory.body.end);
  for (const edit of edits.sort((a, b) => b.start - a.start)) body = body.slice(0, edit.start - factory.body.getStart(parsed)) + body.slice(edit.end - factory.body.getStart(parsed));
  const closed = ts.createSourceFile("closed.js", `${modifier}function instantiate(getCoreModule, imports, instantiateCore = WebAssembly.instantiate) ${body}`, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  if ((closed as unknown as { parseDiagnostics: unknown[] }).parseDiagnostics.length) throw new Error("browser component factory: invalid closed source");
  const requested = new Set<string>();
  const requiredInterfaces = new Set<string>();
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) || ts.isImportEqualsDeclaration(node) || ts.isExportDeclaration(node)) throw new Error("browser component factory: executable import");
    if (ts.isMetaProperty(node) && node.keywordToken === ts.SyntaxKind.ImportKeyword) throw new Error("browser component factory: residual module identity");
    if (ts.isIdentifier(node) && node.text === "imports" && !ts.isParameter(node.parent)) {
      if (!ts.isElementAccessExpression(node.parent) || node.parent.expression !== node || !ts.isStringLiteral(node.parent.argumentExpression)) throw new Error("browser component factory: dynamic interface access");
      requiredInterfaces.add(node.parent.argumentExpression.text);
    }
    if (ts.isIdentifier(node) && ["fetch", "eval", "importScripts"].includes(node.text)) throw new Error("browser component factory: external loader reference");
    if (ts.isElementAccessExpression(node) && ts.isStringLiteral(node.argumentExpression) && ["fetch", "eval", "importScripts"].includes(node.argumentExpression.text)) throw new Error("browser component factory: external loader reference");
    if (ts.isCallExpression(node)) {
      if (node.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(node.expression) && ["fetch", "eval", "Function"].includes(node.expression.text)) || (ts.isPropertyAccessExpression(node.expression) && ["fetch", "eval"].includes(node.expression.name.text))) throw new Error("browser component factory: dynamic code or network loader");
      if (ts.isIdentifier(node.expression) && node.expression.text === "getCoreModule") {
        if (node.arguments.length !== 1 || !ts.isStringLiteral(node.arguments[0])) throw new Error("browser component factory: dynamic core selector");
        requested.add(node.arguments[0].text);
      }
    }
    if (ts.isNewExpression(node) && ts.isIdentifier(node.expression) && ["URL", "Function", "Worker", "SharedWorker", "WebSocket", "XMLHttpRequest"].includes(node.expression.text)) throw new Error("browser component factory: external executable resource");
    ts.forEachChild(node, visit);
  };
  visit(closed);
  const admittedInterfaces = new Set(control.importInterfaces ?? []);
  if (admittedInterfaces.size > 64 || admittedInterfaces.size !== (control.importInterfaces?.length ?? 0) || admittedInterfaces.size !== requiredInterfaces.size || [...admittedInterfaces].some(name => !/^[a-z0-9][a-z0-9-]*:[a-z0-9][a-z0-9-]*\/[a-z0-9][a-z0-9-]*@[0-9]+\.[0-9]+\.[0-9]+$/.test(name) || !requiredInterfaces.has(name))) throw new Error("browser component factory: import interface manifest mismatch");
  const names = new Set<string>();
  let total = 0;
  for (const core of cores) {
    if (!/^[a-zA-Z0-9_-]+\.core\d*\.wasm$/.test(core.name) || names.has(core.name) || !requested.has(core.name) || core.bytes.byteLength < 8) throw new Error("browser component factory: invalid or duplicate core");
    names.add(core.name);
    total += core.bytes.byteLength;
    if (total > 128 * 1024 * 1024) throw new Error("browser component factory: core byte bound");
  }
  if (names.size !== requested.size) throw new Error("browser component factory: missing core");
  const table: Record<string, { length: number; chunks: string[] }> = Object.create(null);
  let completed = 0;
  for (const core of cores) {
    const chunks: string[] = [];
    for (let offset = 0; offset < core.bytes.byteLength; offset += 48 * 1024) {
      check();
      const chunk = core.bytes.subarray(offset, Math.min(offset + 48 * 1024, core.bytes.byteLength));
      chunks.push(Buffer.from(chunk).toString("base64"));
      completed += chunk.byteLength;
      control.progress?.(completed, total);
      await new Promise<void>(resolve => setTimeout(resolve, 0));
    }
    table[core.name] = { length: core.bytes.byteLength, chunks };
  }
  check();
  return `${modifier}function __semioInstantiate(getCoreModule, imports, instantiateCore = WebAssembly.instantiate) ${body}
const __semioRequiredInterfaces = Object.freeze(${JSON.stringify([...requiredInterfaces].sort())});
const __semioEmbeddedCores = Object.freeze(Object.fromEntries(Object.entries(${JSON.stringify(table)}).map(([name, value]) => [name, Object.freeze({ length: value.length, chunks: Object.freeze(value.chunks) })])));
async function instantiateFreshComponent(imports, control = {}) {
  const check = () => { if (control.signal?.aborted) throw new Error("browser component factory: cancelled"); };
  check();
  if (!imports || typeof imports !== "object" || Reflect.ownKeys(imports).length !== __semioRequiredInterfaces.length) throw new Error("browser component factory: runtime import manifest mismatch");
  for (const name of __semioRequiredInterfaces) {
    const descriptor = Object.getOwnPropertyDescriptor(imports, name);
    if (!descriptor || !("value" in descriptor) || !descriptor.value || typeof descriptor.value !== "object" || Array.isArray(descriptor.value)) throw new Error("browser component factory: runtime import manifest mismatch");
  }
  const pending = [];
  const compileCore = async (name) => {
    check();
    if (!Object.hasOwn(__semioEmbeddedCores, name)) throw new Error("browser component factory: unknown core");
    const core = __semioEmbeddedCores[name];
    const bytes = new Uint8Array(core.length);
    try {
      let offset = 0;
      for (const chunk of core.chunks) {
        check();
        const decoded = atob(chunk);
        for (let index = 0; index < decoded.length; index++) bytes[offset++] = decoded.charCodeAt(index);
        control.onProgress?.({ phase: "decode", core: name, completed: offset, total: core.length });
        await new Promise(resolve => setTimeout(resolve, 0));
      }
      check();
      if (offset !== core.length) throw new Error("browser component factory: core length mismatch");
      control.onProgress?.({ phase: "compile", core: name, completed: 0, total: core.length });
      const module = await WebAssembly.compile(bytes);
      check();
      return module;
    } finally { bytes.fill(0); }
  };
  const getCoreModule = (name) => {
    const operation = compileCore(name);
    operation.catch(() => {});
    pending.push(operation);
    return operation;
  };
  control.onProgress?.({ phase: "instantiate", completed: 0, total: ${cores.length} });
  try {
    const instance = await __semioInstantiate(getCoreModule, imports);
    check();
    return instance;
  } finally { await Promise.allSettled(pending); }
}
`;
}

const createBrowserBundleTestsInstance = createBrowserBundleTests({ assert, browserActorAsyncImports, browserActorInterfaces, browserBundleValidator, buildBrowserCodegenModule, buildClosedBrowserActorArtifactOwned, buildClosedBrowserActorArtifactV1, captureBrowserActorRuntime, captureBrowserCodegenSources, closeBrowserCodegenModule, closedBrowserActorBundle, closedBrowserActorBundleFromRuntime, closedBrowserComponentFactory, dirname, exactExecutableFingerprint, join, lstatSync, mkdirSync, mkdtempSync, parseBrowserActorCodegenManifest, readdirSync, readFileSync, realpathSync, renameSync, runExactCargoLawProcess, sealBrowserCodegenPolicy, ts, writeFileSync }, { directory: import.meta.dir, url: import.meta.url });
export const testClosedBrowserComponentFactory = createBrowserBundleTestsInstance.testClosedBrowserComponentFactory;
const testBrowserCodegenCapsule = createBrowserBundleTestsInstance.testBrowserCodegenCapsule;
const testBrowserCodegenSources = createBrowserBundleTestsInstance.testBrowserCodegenSources;
const testBrowserCodegenPolicy = createBrowserBundleTestsInstance.testBrowserCodegenPolicy;
const testBrowserActorCodegenManifest = createBrowserBundleTestsInstance.testBrowserActorCodegenManifest;
const testClosedBrowserActorBundle = createBrowserBundleTestsInstance.testClosedBrowserActorBundle;
const testBrowserHostActivation = createBrowserBundleTestsInstance.testBrowserHostActivation;

