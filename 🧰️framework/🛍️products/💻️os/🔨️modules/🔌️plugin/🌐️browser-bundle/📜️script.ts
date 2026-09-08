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

export async function testClosedBrowserComponentFactory(repoRoot: string): Promise<void> {
  await testBrowserActorCodegenManifest();
  await testBrowserCodegenCapsule(repoRoot);
  await testBrowserCodegenSources(repoRoot);
  await testBrowserCodegenPolicy(repoRoot);
  await (await import("./🧪️fixtures/🌐️wasi-activation/📜️script.ts")).testBrowserWasiActivation(repoRoot);
  await testBrowserHostActivation();
  await testClosedBrowserActorBundle(repoRoot);
  const fixtureRoot = join(import.meta.dir, "🧪️fixtures/🧊️component-factory");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validate = await browserBundleValidator("ComponentFactoryV1");
  assert(validate(fixture), JSON.stringify(validate.errors));
  const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactBase?.includes("🗑️generated"), "browser factory law requires ticket-generated evidence root");
  mkdirSync(artifactBase, { recursive: true });
  const evidence = mkdtempSync(join(artifactBase, "browser-component-factory-"));
  const probe = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import { parse, transpile } from "@bytecodealliance/jco";
    import { readFileSync } from "node:fs";
    const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const result = await transpile(await parse(fixture.component), { name: fixture.name, instantiation: "async", nodejsCompat: false, base64Cutoff: 0, quiet: true, map: Object.fromEntries((fixture.importInterfaces ?? []).map(name => [name, name])) });
    process.stdout.write(JSON.stringify({ source: new TextDecoder().decode(result.files[fixture.name + ".js"]), cores: Object.entries(result.files).filter(([name]) => name.endsWith(".wasm")).map(([name, bytes]) => ({ name, hex: Buffer.from(bytes).toString("hex") })) }));
  `, join(fixtureRoot, "🔣️.json")], { cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 2 * 1024 * 1024, stdoutPath: join(evidence, "jco.stdout.json"), stderrPath: join(evidence, "jco.stderr"), cancelled: () => false });
  assert.equal(probe.status, 0, probe.stderr);
  const input = JSON.parse(probe.stdout);
  const cores = input.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") }));
  const factory = await closedBrowserComponentFactory(input.source, cores);
  const explicitAsync = await closedBrowserComponentFactory(input.source.replace("export function instantiate(getCoreModule, imports, instantiateCore = WebAssembly.instantiate) {", "export async function instantiate(getCoreModule, imports, instantiateCore = WebAssembly.instantiate) { await Promise.resolve();"), cores);
  assert(explicitAsync.startsWith("async function __semioInstantiate"));
  writeFileSync(join(evidence, "factory.json"), JSON.stringify({ factory, explicitAsync, fixture, core: input.cores[0].hex }));
  const runtime = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import assert from "node:assert/strict";
    import { readFileSync } from "node:fs";
    const { factory, explicitAsync, fixture, core } = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const module = await import("data:text/javascript;base64," + Buffer.from(factory + "\\nexport { instantiateFreshComponent };").toString("base64"));
    const a = await module.instantiateFreshComponent({});
    const b = await module.instantiateFreshComponent({});
    const actual = fixture.calls.map(name => ({ a, b })[name].next());
    const compiled = await WebAssembly.compile(Buffer.from(core, "hex"));
    const independent = { a: await WebAssembly.instantiate(compiled), b: await WebAssembly.instantiate(compiled) };
    const oracle = fixture.calls.map(name => independent[name].exports.next());
    assert.deepEqual(actual, fixture.expected);
    assert.deepEqual(actual, oracle);
    const asyncModule = await import("data:text/javascript;base64," + Buffer.from(explicitAsync + "\\nexport { instantiateFreshComponent };").toString("base64"));
    const asyncActors = { a: await asyncModule.instantiateFreshComponent({}), b: await asyncModule.instantiateFreshComponent({}) };
    assert.deepEqual(fixture.calls.map(name => asyncActors[name].next()), oracle);
    const nativeCompile = WebAssembly.compile;
    const nativeInstantiate = WebAssembly.instantiate;
    const abort = new AbortController(); abort.abort();
    let compilations = 0, instances = 0, staging;
    WebAssembly.compile = async (...args) => { compilations++; return nativeCompile(...args); };
    WebAssembly.instantiate = (...args) => { instances++; return nativeInstantiate(...args); };
    let cancelledBeforeCompile, cancelledDuringCompile;
    try {
      await assert.rejects(module.instantiateFreshComponent({}, { signal: abort.signal }), /cancelled/);
      await assert.rejects(module.instantiateFreshComponent({ "semio:framework/pure@1.0.0": {} }), /runtime import manifest/);
      cancelledBeforeCompile = { compilations, instances };
      assert.deepEqual(cancelledBeforeCompile, fixture.cancelledBeforeCompile);
      const during = new AbortController();
      WebAssembly.compile = async bytes => { compilations++; staging = bytes; during.abort(); return compiled; };
      await assert.rejects(module.instantiateFreshComponent({}, { signal: during.signal }), /cancelled/);
      cancelledDuringCompile = { compilations, instances, stagingZeroed: staging.every(byte => byte === 0) };
      assert.deepEqual(cancelledDuringCompile, fixture.cancelledDuringCompile);
    } finally { WebAssembly.compile = nativeCompile; WebAssembly.instantiate = nativeInstantiate; }
    console.log(JSON.stringify({ actual, oracle, cancelledBeforeCompile, cancelledDuringCompile }));
  `, join(evidence, "factory.json")], { cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "runtime.stdout.json"), stderrPath: join(evidence, "runtime.stderr"), cancelled: () => false });
  assert.equal(runtime.status, 0, runtime.stderr);
  const observations = JSON.parse(runtime.stdout);
  assert.deepEqual(observations.actual, fixture.expected);
  const hostile = [
    () => closedBrowserComponentFactory(`import x from "external";\n${input.source}`, cores),
    () => closedBrowserComponentFactory(input.source.replace("const module0 =", "import('external'); const module0 ="), cores),
    () => closedBrowserComponentFactory(input.source.replace("const module0 =", "fetch('external'); const module0 ="), cores),
    () => closedBrowserComponentFactory(input.source.replace("const module0 =", "new URL('external'); const module0 ="), cores),
    () => closedBrowserComponentFactory(`${input.source}\nexport const extra = 1;`, cores),
    () => closedBrowserComponentFactory(input.source, []),
    () => closedBrowserComponentFactory(input.source, [...cores, cores[0]]),
    () => closedBrowserComponentFactory(input.source, [...cores, { name: "extra.core.wasm", bytes: cores[0].bytes }]),
    () => closedBrowserComponentFactory(input.source.replace("const module0 =", "void import.meta.url; const module0 ="), cores),
    () => closedBrowserComponentFactory(input.source.replace("const module0 =", "void imports['semio:framework/pure@1.0.0']; const module0 ="), cores),
    () => closedBrowserComponentFactory(input.source.replace("const module0 =", "void imports[String('semio:framework/pure@1.0.0')]; const module0 ="), cores),
  ];
  assert.equal(hostile.length, fixture.forbidden.length);
  for (const reject of hostile) await assert.rejects(reject);
  let builderProgress = 0;
  await assert.rejects(closedBrowserComponentFactory(input.source, cores, { cancelled: () => true, progress: () => builderProgress++ }), /cancelled/);
  assert.equal(builderProgress, 0);
  console.log(`browser-component-factory: AJV=1 JCO=1 native-Wasm-oracle=1 actors=2 hostile=${hostile.length} cancellation=3 bytes=${factory.length} evidence=${evidence}`);
}

/** 🔐️ Compares an immutable compiler capsule with native WebAssembly and rejects executable closure escapes. */
async function testBrowserCodegenCapsule(repoRoot: string): Promise<void> {
  const root = join(import.meta.dir, "🧪️fixtures/🔒️compiler-capsule");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = await browserBundleValidator("CompilerCapsuleV1");
  assert(validate(fixture), JSON.stringify(validate.errors));
  const cores = fixture.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") }));
  const closed = closeBrowserCodegenModule(fixture.source, cores);
  const hostile = [
    () => closeBrowserCodegenModule(fixture.source + " const =;", cores),
    () => closeBrowserCodegenModule(fixture.source + " const third = fetchCompile(new URL('./js-component-bindgen-component.core3.wasm', import.meta.url));", cores),
    () => closeBrowserCodegenModule(fixture.source.replaceAll("import.meta.url", "'https://untrusted.invalid/'"), cores),
    () => closeBrowserCodegenModule(fixture.source + " await import('node:net');", cores),
    () => closeBrowserCodegenModule("import 'node:net';" + fixture.source, cores),
    () => closeBrowserCodegenModule(fixture.source + " fetch('https://untrusted.invalid');", cores),
    () => closeBrowserCodegenModule(fixture.source.replace("node:fs/promises", "node:net"), cores),
    () => closeBrowserCodegenModule(fixture.source + "\n//# sourceURL=untrusted.js", cores),
    () => closeBrowserCodegenModule(fixture.source, cores.slice(0, 1)),
    () => closeBrowserCodegenModule(fixture.source, [...cores, { name: "extra.wasm", bytes: cores[0].bytes }]),
    () => closeBrowserCodegenModule(fixture.source + " fetchCompile = () => {};", cores),
  ];
  assert.equal(hostile.length, fixture.denied.length);
  for (const denied of hostile) assert.throws(denied, /compiler capsule/);
  const evidence = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "browser-compiler-capsule-"));
  writeFileSync(join(evidence, "fixture.json"), JSON.stringify({ fixture, closed }), { mode: 0o600 });
  const runtime = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import assert from "node:assert/strict";
    import { readFileSync } from "node:fs";
    const { fixture, closed } = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const module = await import("data:text/javascript;base64," + Buffer.from(closed).toString("base64"));
    const oracle = await Promise.all(fixture.cores.map(core => WebAssembly.compile(Buffer.from(core.hex, "hex"))));
    assert.equal(await module.generate(), fixture.expected);
    assert.equal(await module.generate(), oracle.length);
    process.stdout.write(JSON.stringify({ actual: await module.generate(), oracle: oracle.length }));
  `, join(evidence, "fixture.json")], { cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "runtime.stdout.json"), stderrPath: join(evidence, "runtime.stderr"), cancelled: () => false });
  assert.equal(runtime.status, 0, runtime.stderr);
  assert.deepEqual(JSON.parse(runtime.stdout), { actual: fixture.expected, oracle: fixture.expected });
  console.log(`browser-compiler-capsule: AJV=1 native-Wasm-oracle=1 valid=1 denied=${hostile.length} evidence=${evidence}`);
}

/** 📸️ Compares captured compiler source semantics with Node and tests replacement after onLoad. */
async function testBrowserCodegenSources(repoRoot: string): Promise<void> {
  const root = join(import.meta.dir, "🧪️fixtures/📸️compiler-sources");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = await browserBundleValidator("CompilerSourcesV1");
  assert(validate(fixture), JSON.stringify(validate.errors));
  const evidence = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "browser-compiler-sources-"));
  const directory = join(evidence, "source");
  mkdirSync(directory);
  const reset = () => { for (const [name, source] of Object.entries(fixture.files)) writeFileSync(join(directory, name), source as string, { mode: 0o600 }); };
  reset();
  const roots = [{ name: "fixture", path: directory }];
  const baseline = await captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => {});
  assert.deepEqual(baseline.inputs.map(row => row.logicalPath), fixture.inputs);
  for (const row of baseline.inputs) assert.equal(row.sha256, Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(fixture.files[row.logicalPath.slice(8)]))).toString("hex"));
  let replaced = 0;
  const changed = await captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => {}, path => {
    renameSync(path, path + ".retained");
    writeFileSync(path, "throw new Error('changed compiler source executed');", { mode: 0o600 });
    replaced++;
  });
  assert.equal(replaced, fixture.inputs.length);
  assert.deepEqual(changed, baseline);
  const oracle = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import assert from "node:assert/strict";
    import { readFileSync } from "node:fs";
    const { source, fixture } = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const module = await import("data:text/javascript;base64," + Buffer.from(source).toString("base64"));
    const dependency = "data:text/javascript;base64," + Buffer.from(fixture.files["value.js"]).toString("base64");
    const original = fixture.files["entry.js"].replace("./value.js", dependency);
    const independent = await import("data:text/javascript;base64," + Buffer.from(original).toString("base64"));
    assert.equal(module.answer, independent.answer);
    assert.equal(module.answer, fixture.expected);
    process.stdout.write(JSON.stringify({ answer: module.answer }));
  `, (() => { const path = join(evidence, "oracle.json"); writeFileSync(path, JSON.stringify({ source: changed.source, fixture }), { mode: 0o600 }); return path; })()], { cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "oracle.stdout.json"), stderrPath: join(evidence, "oracle.stderr"), cancelled: () => false });
  assert.equal(oracle.status, 0, oracle.stderr);
  assert.deepEqual(JSON.parse(oracle.stdout), { answer: fixture.expected });
  reset();
  writeFileSync(join(evidence, "foreign.js"), "export const value = 900;", { mode: 0o600 });
  writeFileSync(join(directory, "entry.js"), "export { value } from '../foreign.js';", { mode: 0o600 });
  await assert.rejects(captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => {}), /compiler source/);
  writeFileSync(join(directory, "value.json"), "{}", { mode: 0o600 });
  writeFileSync(join(directory, "entry.js"), "export { default } from './value.json';", { mode: 0o600 });
  await assert.rejects(captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => {}), /compiler source/);
  await assert.rejects(captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => { throw new Error("cancelled compiler source"); }), /cancelled compiler source/);
  console.log(`browser-compiler-sources: AJV=1 Node-oracle=1 WebCrypto=1 laws=${fixture.laws.length} inputs=${baseline.inputs.length} evidence=${evidence}`);
}

/** 🔏️ Compares policy snapshots and digests with independent canonical JSON and WebCrypto. */
async function testBrowserCodegenPolicy(repoRoot: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧪️fixtures/🔏️codegen-policy/🔣️.json"), "utf8"));
  const validate = await browserBundleValidator("CodegenPolicyV1");
  const { default: stableStringify } = await import("fast-json-stable-stringify");
  const digest = { sha256: "a".repeat(64), byteLength: 8 };
  const input = {
    schema: "semio.os.browser-codegen-policy.v1", revision: 1,
    runtime: { kind: "bun", version: "1.3.14", executable: { ...digest } },
    compiler: { module: { ...digest }, inputs: [{ logicalPath: "compiler/entry.js", ...digest }], cores: ["compiler/core.wasm", "compiler/core2.wasm"].map(logicalPath => ({ logicalPath, ...digest })) },
    packages: fixture.packages.map(([name, version]: string[]) => ({ name, version, lockSha256: digest.sha256, manifestSha256: digest.sha256 })),
    parser: { name: "typescript", version: "5.9.3", manifestSha256: digest.sha256, lockRowSha256: digest.sha256, entry: { logicalPath: "typescript/lib/typescript.js", ...digest } },
    firstParty: [{ logicalPath: "browser/script.ts", ...digest }],
    options: { jco: "1.27.0", entrypoint: "@bytecodealliance/jco/component", target: "browser", format: "esm", name: "browser-actor", instantiation: "async", asyncMode: "jspi", nodejsCompat: false, base64Cutoff: 0, importInterfaces: [...browserActorInterfaces], asyncImports: [...browserActorAsyncImports] },
  };
  assert(validate(input), JSON.stringify(validate.errors));
  const sealed = sealBrowserCodegenPolicy(input);
  const canonical = stableStringify(input);
  assert.equal(sealed.canonical, canonical);
  assert.equal(sealed.sha256, Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(canonical))).toString("hex"));
  assert.deepEqual(sealBrowserCodegenPolicy(Object.fromEntries(Object.entries(input).reverse())), sealed);
  const targets = [input.runtime.executable, input.compiler.module, ...input.compiler.inputs, ...input.compiler.cores, ...input.firstParty, input.parser.entry];
  for (const row of targets) {
    row.sha256 = "b".repeat(64);
    assert.notEqual(sealBrowserCodegenPolicy(input).sha256, sealed.sha256);
    row.sha256 = digest.sha256;
  }
  for (const row of input.packages) for (const key of ["lockSha256", "manifestSha256"]) {
    row[key] = "b".repeat(64);
    assert.notEqual(sealBrowserCodegenPolicy(input).sha256, sealed.sha256);
    row[key] = digest.sha256;
  }
  input.compiler.inputs[0].sha256 = "c".repeat(64);
  assert.equal(sealed.record.compiler.inputs[0].sha256, digest.sha256);
  assert(Object.isFrozen(sealed.record.compiler.inputs[0]));
  const executable = exactExecutableFingerprint(realpathSync(process.execPath));
  assert.equal(executable.path, realpathSync(process.execPath));
  assert.equal(executable.byteLength, lstatSync(executable.path).size);
  assert.match(executable.sha256, /^[a-f0-9]{64}$/);
  const evidence = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "browser-codegen-policy-"));
  const compiler = await buildBrowserCodegenModule(evidence, () => {});
  const capturedInputs = structuredClone(compiler.inputs);
  writeFileSync(join(evidence, "compiler-sources.json"), "[]", { mode: 0o600 });
  assert.deepEqual(compiler.inputs, capturedInputs);
  assert.equal(compiler.inputs.length, 8);
  assert(compiler.inputs.every(Object.isFrozen));
  assert.equal(compiler.cores.length, 2);
  console.log(`browser-codegen-policy: AJV=1 stable-stringify=1 WebCrypto=1 laws=${fixture.laws.length} inputs=${targets.length + input.packages.length * 2}`);
}

/** 📦️ Compares generated-manifest admission with the strict schema and canonical array ordering. */
async function testBrowserActorCodegenManifest(): Promise<void> {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧪️fixtures/📦️codegen-manifest/🔣️.json"), "utf8"));
  const validate = await browserBundleValidator("CodegenManifestV1");
  const oracle = (value: typeof fixture.valid) => Boolean(validate(value)) && [value.files, value.importInterfaces].every(items => JSON.stringify(items) === JSON.stringify([...items].sort()));
  assert.equal(oracle(fixture.valid), true);
  assert.deepEqual(parseBrowserActorCodegenManifest(fixture.valid), fixture.valid);
  for (const patch of fixture.denied) {
    const value = { ...fixture.valid, ...patch };
    assert.equal(oracle(value), false);
    assert.throws(() => parseBrowserActorCodegenManifest(value), /generated manifest/);
  }
  console.log(`browser-actor-codegen-manifest: AJV=1 canonical-arrays=1 valid=1 denied=${fixture.denied.length}`);
}

/** 🎭️ Qualifies one closed ESM with a real canonical pure import and exact actor shutdown. */
async function testClosedBrowserActorBundle(repoRoot: string): Promise<void> {
  const root = join(import.meta.dir, "🧪️fixtures/🧊️actor-factory");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = await browserBundleValidator("ActorFactoryV1");
  assert(validate(fixture), JSON.stringify(validate.errors));
  const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactBase?.includes("🗑️generated"));
  mkdirSync(artifactBase, { recursive: true });
  const evidence = mkdtempSync(join(artifactBase, "browser-actor-factory-"));
  const probe = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import { parse, transpile } from "@bytecodealliance/jco";
    import { readFileSync } from "node:fs";
    const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const component = await parse(fixture.component);
    const unsupported = await parse(fixture.component.replaceAll("semio:framework/pure@1.0.0", "untrusted:remote/pure@1.0.0"));
    const result = await transpile(component, { name: fixture.name, instantiation: "async", nodejsCompat: false, base64Cutoff: 0, quiet: true, map: Object.fromEntries((fixture.importInterfaces ?? []).map(name => [name, name])) });
    const closed = await transpile(component, { name: "browser-actor", instantiation: "async", nodejsCompat: false, base64Cutoff: 0, quiet: true, map: Object.fromEntries(JSON.parse(process.argv[2]).map(name => [name, name])), asyncMode: "jspi", asyncImports: JSON.parse(process.argv[3]) });
    process.stdout.write(JSON.stringify({ component: Buffer.from(component).toString("hex"), unsupported: Buffer.from(unsupported).toString("hex"), source: new TextDecoder().decode(result.files[fixture.name + ".js"]), cores: Object.entries(result.files).filter(([name]) => name.endsWith(".wasm")).map(([name, bytes]) => ({ name, hex: Buffer.from(bytes).toString("hex") })), closed: { source: new TextDecoder().decode(closed.files["browser-actor.js"]), importInterfaces: closed.imports, cores: Object.entries(closed.files).filter(([name]) => name.endsWith(".wasm")).map(([name, bytes]) => ({ name, hex: Buffer.from(bytes).toString("hex") })) } }));
  `, join(root, "🔣️.json"), JSON.stringify(browserActorInterfaces), JSON.stringify(browserActorAsyncImports)], { cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 2 * 1024 * 1024, stdoutPath: join(evidence, "jco.stdout.json"), stderrPath: join(evidence, "jco.stderr"), cancelled: () => false });
  assert.equal(probe.status, 0, probe.stderr);
  const input = JSON.parse(probe.stdout);
  const componentBytes = Buffer.from(input.component, "hex");
  for (const key of ["repoRoot", "evidenceRoot"]) {
    const unread = new Proxy(componentBytes, { get() { throw new Error("unreserved input inspected"); } });
    await assert.rejects(buildClosedBrowserActorArtifactV1(unread, { [key]: evidence } as unknown as BrowserActorBuildControl), { message: "browser actor artifact: invalid build control" });
  }
  const scratchNames = () => readdirSync(artifactBase).filter(name => name.startsWith("browser-actor-codegen-")).sort();
  const scratchBefore = scratchNames();
  let scratchDuring: string[] = [];
  const artifact = await buildClosedBrowserActorArtifactV1(componentBytes, { progress(phase) { if (phase === "codegen") scratchDuring = scratchNames().filter(name => !scratchBefore.includes(name)); } });
  assert.equal(scratchDuring.length, 1);
  assert.deepEqual(scratchNames(), scratchBefore);
  let scratchCancelled = false;
  await assert.rejects(buildClosedBrowserActorArtifactV1(componentBytes, { cancelled: () => scratchCancelled, progress(phase) { if (phase === "codegen") { assert.equal(scratchNames().filter(name => !scratchBefore.includes(name)).length, 1); scratchCancelled = true; } } }), { message: "browser actor artifact: cancelled" });
  assert.equal(scratchCancelled, true);
  assert.deepEqual(scratchNames(), scratchBefore);
  const validatePolicy = await browserBundleValidator("CodegenPolicyV1");
  const policy = JSON.parse(artifact.policyCanonical);
  assert(validatePolicy(policy), JSON.stringify(validatePolicy.errors));
  assert.equal(artifact.policySha256, Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(artifact.policyCanonical))).toString("hex"));
  assert.equal(artifact.policyCanonical.includes(repoRoot), false);
  assert.equal(policy.compiler.inputs.length, 8);
  assert.equal(policy.firstParty.length, 6);
  assert.deepEqual(artifact.importInterfaces, input.closed.importInterfaces);
  const nodeClosure = await closedBrowserActorBundle(input.closed.source, input.closed.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") })), { importInterfaces: input.closed.importInterfaces });
  assert.deepEqual(artifact.bytes, new TextEncoder().encode(nodeClosure));
  const runtimeRoot = join(evidence, "captured-runtime");
  for (const name of ["🌐️host/🟦️.ts", "🌐️wasi/🟦️.ts"]) {
    const path = join(runtimeRoot, name);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, readFileSync(join(import.meta.dir, name)), { mode: 0o600 });
  }
  const runtimeSnapshot = captureBrowserActorRuntime(runtimeRoot, () => {});
  for (const name of ["🌐️host/🟦️.ts", "🌐️wasi/🟦️.ts"]) {
    const path = join(runtimeRoot, name);
    renameSync(path, path + ".retained");
    writeFileSync(path, "throw new Error('runtime replacement executed');", { mode: 0o600 });
  }
  const capturedClosure = await closedBrowserActorBundleFromRuntime(input.closed.source, input.closed.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") })), { importInterfaces: input.closed.importInterfaces }, runtimeSnapshot);
  assert.equal(capturedClosure, nodeClosure);
  for (const name of ["🌐️host/🟦️.ts", "🌐️wasi/🟦️.ts"]) writeFileSync(join(runtimeRoot, name), "import './escape.ts';\n" + readFileSync(join(import.meta.dir, name), "utf8"), { mode: 0o600 });
  await assert.rejects(closedBrowserActorBundleFromRuntime(input.closed.source, input.closed.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") })), { importInterfaces: input.closed.importInterfaces }, captureBrowserActorRuntime(runtimeRoot, () => {})), /Bundle failed|runtime import denied/);
  const ambient = Object.fromEntries(["NODE_OPTIONS", "NODE_PATH", "BUN_OPTIONS", "BUN_PRELOAD", "PATH"].map(name => [name, process.env[name]]));
  try {
    process.env.NODE_OPTIONS = "--require=/semio-denied-ambient-preload";
    process.env.NODE_PATH = process.env.PATH = "/semio-denied-ambient-path";
    process.env.BUN_OPTIONS = "--preload=/semio-denied-ambient-preload";
    process.env.BUN_PRELOAD = "/semio-denied-ambient-preload";
    assert.deepEqual(await buildClosedBrowserActorArtifactOwned(componentBytes, {}, evidence), artifact);
  } finally {
    for (const [name, value] of Object.entries(ambient)) if (value === undefined) delete process.env[name]; else process.env[name] = value;
  }
  const digest = async (bytes: Uint8Array) => Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex");
  assert.equal(artifact.schema, fixture.artifact.schema);
  assert.equal(artifact.codegenPolicy, fixture.artifact.codegenPolicy);
  assert.equal(artifact.componentSha256, await digest(componentBytes));
  assert.equal(artifact.sha256, await digest(artifact.bytes));
  assert.equal(artifact.byteLength, artifact.bytes.byteLength);
  assert(artifact.byteLength <= fixture.artifact.maximumBytes);
  assert.deepEqual(artifact.importInterfaces, fixture.importInterfaces);
  const mutable = Buffer.from(componentBytes), phases: string[] = [];
  const repeat = buildClosedBrowserActorArtifactOwned(mutable, { progress(phase) { phases.push(phase); if (phase === "snapshot") mutable.fill(0); } }, evidence);
  let rejectedProgress = 0;
  const unread = new Proxy(componentBytes, { get() { throw new Error("unreserved build touched input"); } });
  try {
    await assert.rejects(buildClosedBrowserActorArtifactV1(unread, { progress() { rejectedProgress++; } }), /build capacity/);
    assert.equal(rejectedProgress, 0);
  } finally { await repeat; }
  const repeated = await repeat;
  assert(mutable.every(value => value === 0));
  assert.deepEqual(repeated, artifact);
  assert.deepEqual([...new Set(phases)], ["snapshot", "policy", "codegen", "closure", "hash"]);
  await assert.rejects(buildClosedBrowserActorArtifactOwned(Buffer.from(input.unsupported, "hex"), {}, evidence), /unsupported import interface/);
  let cancelled = false;
  await assert.rejects(buildClosedBrowserActorArtifactOwned(componentBytes, { cancelled: () => cancelled, progress(phase, completed) { if (phase === "codegen" && completed) cancelled = true; } }, evidence), /cancelled/);
  await assert.rejects(buildClosedBrowserActorArtifactV1(componentBytes, { cancelled: () => true }), /cancelled/);
  await assert.rejects(buildClosedBrowserActorArtifactV1(new Uint8Array(8)), /component header/);
  const replacementRoot = join(evidence, "opened-file-replacement");
  let codegenComplete = false, readChecks = 0, replaced = false;
  await assert.rejects(buildClosedBrowserActorArtifactOwned(componentBytes, {
    progress(phase, completed) { if (phase === "codegen" && completed) codegenComplete = true; },
    cancelled() {
      if (codegenComplete && ++readChecks === 2) {
        const rows = readdirSync(replacementRoot);
        assert.equal(rows.length, 1);
        const generated = join(replacementRoot, rows[0], "browser-actor.js");
        renameSync(generated, generated + ".retained");
        writeFileSync(generated, "throw new Error('untrusted replacement');", { mode: 0o600 });
        replaced = true;
      }
      return false;
    },
  }, replacementRoot), /build input: file changed while reading/);
  assert.equal(replaced, true);
  for (const name of ["compiler.mjs", "component.wasm"]) {
    const replacementRoot = join(evidence, name + "-replacement");
    mkdirSync(replacementRoot, { recursive: true });
    let replaced = false;
    await assert.rejects(buildClosedBrowserActorArtifactOwned(componentBytes, {
      cancelled() {
        const rows = readdirSync(replacementRoot);
        if (!replaced && rows.length === 1) {
          const directory = join(replacementRoot, rows[0]);
          if (readdirSync(directory).includes("compiler.mjs")) {
            const path = join(directory, name);
            renameSync(path, path + ".retained");
            writeFileSync(path, name.endsWith(".mjs") ? "throw new Error('untrusted compiler evaluated');" : Buffer.from(input.unsupported, "hex"), { mode: 0o600 });
            replaced = true;
          }
        }
        return false;
      },
    }, replacementRoot), /captured input identity/);
    assert.equal(replaced, true);
    const directory = join(replacementRoot, readdirSync(replacementRoot)[0]);
    assert.equal(readdirSync(directory).some(name => name.startsWith("browser-actor.")), false);
    assert.equal(readFileSync(join(directory, "codegen.stdout.json"), "utf8"), "");
  }
  const bundle = new TextDecoder("utf-8", { fatal: true }).decode(artifact.bytes);
  writeFileSync(join(evidence, "bundle.json"), JSON.stringify({ bundle, fixture, core: input.cores[0].hex }));
  const runtime = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import assert from "node:assert/strict";
    import { readFileSync } from "node:fs";
    const { bundle, fixture, core } = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const module = await import("data:text/javascript;base64," + Buffer.from(bundle).toString("base64"));
    assert.deepEqual(Object.keys(module), ["activate"]);
    const actors = {};
    for (const name of ["a", "b"]) {
      let clock = BigInt(fixture.starts[name]);
      actors[name] = await module.activate({ actorId: name, activationGeneration: 1n }, { nowMs: () => ++clock, log() {}, traceSpan() {}, dispatch() { throw new Error("unexpected effect"); }, cancelEffect: () => "closed" });
    }
    const actual = [];
    for (const name of fixture.calls) actual.push(await actors[name].invoke(["next"], []));
    const compiled = await WebAssembly.compile(Buffer.from(core, "hex"));
    const independent = {};
    for (const name of ["a", "b"]) {
      let clock = BigInt(fixture.starts[name]);
      independent[name] = await WebAssembly.instantiate(compiled, { host: { now: () => ++clock } });
    }
    const oracle = fixture.calls.map(name => independent[name].exports.next());
    assert.deepEqual(actual, fixture.expected);
    assert.deepEqual(actual, oracle);
    const close = actors.a.close();
    assert.equal(actors.a.close(), close);
    await close;
    await assert.rejects(actors.a.invoke(["next"], []), /closed/);
    const afterCloseB = await actors.b.invoke(["next"], []);
    assert.equal(afterCloseB, fixture.afterCloseB);
    await actors.b.close();
    assert.deepEqual(actors.a.progress(), fixture.lifecycle.closed);
    let reentrant, reentrantClose;
    const snapshots = [];
    const port = { nowMs() {
      snapshots.push(reentrant.progress());
      reentrantClose = reentrant.close();
      snapshots.push(reentrant.progress());
      return 7n;
    }, log() {}, traceSpan() {}, dispatch() { throw new Error("unexpected effect"); }, cancelEffect: () => "closed" };
    reentrant = await module.activate({ actorId: "reentrant", activationGeneration: 1n }, port);
    assert.deepEqual(reentrant.progress(), fixture.lifecycle.open);
    assert.equal(await reentrant.invoke(["next"], []), 7);
    await reentrantClose;
    assert.deepEqual(snapshots, [fixture.lifecycle.invoking, fixture.lifecycle.closing]);
    assert.deepEqual(reentrant.progress(), fixture.lifecycle.closed);
    const abort = new AbortController();
    const aborted = await module.activate({ actorId: "aborted", activationGeneration: 1n }, { ...port, nowMs: () => 8n }, { signal: abort.signal });
    abort.abort();
    for (let attempt = 0; attempt < 8 && aborted.progress().phase !== "closed"; attempt++) await new Promise(resolve => setTimeout(resolve, 0));
    assert.deepEqual(aborted.progress(), fixture.lifecycle.closed);
    await assert.rejects(aborted.invoke(["next"], []), /closed/);
    assert.equal(aborted.close(), aborted.close());
    let queuedGuestCalls = 0;
    const queued = await module.activate({ actorId: "queued", activationGeneration: 1n }, { ...port, nowMs() { queuedGuestCalls++; return 9n; } });
    const pending = Array.from({ length: 32 }, () => queued.invoke(["next"], []));
    const overflow = assert.rejects(queued.invoke(["next"], []), /capacity/);
    const settled = Promise.allSettled(pending);
    await queued.close();
    await overflow;
    assert.equal((await settled).filter(result => result.status === "rejected").length, 32);
    assert.equal(queuedGuestCalls, 0);
    assert.deepEqual(queued.progress(), fixture.lifecycle.closed);
    console.log(JSON.stringify({ actual, oracle, afterCloseB, snapshots, aborted: aborted.progress(), queuedGuestCalls, exports: Object.keys(module) }));
  `, join(evidence, "bundle.json")], { cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "runtime.stdout.json"), stderrPath: join(evidence, "runtime.stderr"), cancelled: () => false });
  assert.equal(runtime.status, 0, runtime.stderr);
  const { bytes: artifactBytes, ...receipt } = artifact;
  writeFileSync(join(evidence, "closed-artifact.json"), JSON.stringify(receipt), { mode: 0o600 });
  console.log(`browser-actor-factory: AJV=1 JCO=1 native-Wasm-oracle=1 SHA256=node+webcrypto actors=2 laws=${fixture.laws.length} artifact-laws=${fixture.artifactLaws.length} bytes=${artifactBytes.byteLength} evidence=${evidence}`);
}

/** 🧪️ Exercises actor-local host request and stream retirement against schema-owned traces. */
async function testBrowserHostActivation(): Promise<void> {
  const root = join(import.meta.dir, "🧪️fixtures/🌐️host-activation");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = await browserBundleValidator("HostActivationV1");
  assert(validate(fixture), JSON.stringify(validate.errors));
  const { createBrowserHostActivation } = await import("./🌐️host/🟦️.ts");
  const program = ts.createProgram([join(import.meta.dir, "🌐️host/🟦️.ts")], { noEmit: true, strict: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, lib: ["lib.es2023.d.ts", "lib.dom.d.ts"], types: [], skipLibCheck: true });
  const diagnostics = ts.getPreEmitDiagnostics(program);
  assert.equal(diagnostics.length, 0, ts.formatDiagnosticsWithColorAndContext(diagnostics, { getCanonicalFileName: path => path, getCurrentDirectory: () => import.meta.dir, getNewLine: () => "\n" }));
  const frames: import("./🌐️host/🟦️.ts").BrowserHostFrame[] = [];
  const cancellations: string[] = [];
  const port = { dispatch: (frame: import("./🌐️host/🟦️.ts").BrowserHostFrame) => { frames.push(frame); }, cancelEffect: (id: string) => { cancellations.push(id); return "queued" as const; }, log() {}, nowMs: () => 12n, traceSpan() {} };
  const ok = (val: unknown) => ({ tag: "ok", val });
  const make = (index: number, signal?: AbortSignal) => createBrowserHostActivation({ actorId: fixture.actors[index].id, activationGeneration: BigInt(fixture.actors[index].generation) }, port, signal);
  const requestId = (index = frames.length - 1) => (frames[index].frame.envelope.payload.payload as { requestId: string }).requestId;
  const a = make(0), b = make(1);
  const first = a.hostAsync.documentRead({ document: 1n });
  const aId = requestId();
  const second = b.hostAsync.documentRead({ document: 2n });
  const bId = requestId();
  assert.notEqual(aId, bId);
  assert.equal(a.resolveEffect(bId, new Uint8Array([99])), false);
  assert.equal(b.resolveEffect(aId, new Uint8Array([99])), false);
  assert.equal(a.resolveEffect(aId, ok(new Uint8Array(fixture.responses[0]))), true);
  assert.deepEqual(await first, new Uint8Array(fixture.responses[0]));
  const pending = a.hostAsync.storageRead({ key: "pending" });
  const pendingId = requestId();
  const pendingRejected = assert.rejects(pending, error => {
    assert(error instanceof Error);
    assert.match(error.message, /closed/);
    const payload = Object.getOwnPropertyDescriptor(error, "payload")?.value;
    assert(payload instanceof Uint8Array);
    assert.deepEqual(JSON.parse(new TextDecoder().decode(payload)), fixture.closedFault);
    return true;
  });
  const beforeClose = frames.length;
  const closeA = a.close();
  assert.equal(a.close(), closeA);
  await closeA;
  await pendingRejected;
  assert.deepEqual(cancellations, [pendingId]);
  assert.equal(a.resolveEffect(pendingId, new Uint8Array()), false);
  assert.equal(a.rejectEffect(pendingId, "late"), false);
  await assert.rejects(a.hostAsync.documentRead({}), /closed/);
  assert.throws(() => a.hostAsync.emit({}), /closed/);
  assert.equal(frames.length, beforeClose);
  b.resolveEffect(bId, ok(new Uint8Array(fixture.responses[1])));
  assert.deepEqual(await second, new Uint8Array(fixture.responses[1]));
  const waiting = Array.from({ length: fixture.limits.pendingEffects }, () => b.hostAsync.storageRead({}));
  const firstWaitingId = requestId(frames.length - fixture.limits.pendingEffects);
  const atCapacity = frames.length;
  await assert.rejects(b.hostAsync.storageRead({}), /capacity/);
  assert.equal(frames.length, atCapacity);
  b.resolveEffect(firstWaitingId, ok(null));
  await waiting[0];
  const recovered = b.hostAsync.storageRead({});
  assert.equal(frames.length, atCapacity + 1);
  b.resolveEffect(requestId(), ok(null));
  await recovered;
  const waitingSettled = Promise.allSettled(waiting);
  await b.close();
  assert.equal((await waitingSettled).filter(result => result.status === "rejected").length, fixture.limits.pendingEffects - 1);
  const faulty = createBrowserHostActivation({ actorId: "fault", activationGeneration: 1n }, { ...port, dispatch() { throw new Error("dispatch-fault"); } });
  for (let index = 0; index < fixture.limits.pendingEffects + 1; index++) await assert.rejects(faulty.hostAsync.storageRead({}), /dispatch-fault/);
  await faulty.close();
  const streamActor = make(0);
  let streamCancelled = 0;
  const streamResult = streamActor.hostAsync.blobRead("test");
  streamActor.resolveEffect(requestId(), ok(new ReadableStream<Uint8Array>({ start(controller) { controller.enqueue(new Uint8Array(fixture.stream)); }, cancel() { streamCancelled++; } })));
  const bytes = await streamResult;
  for (const byte of fixture.stream) assert.deepEqual(await bytes.next(), { done: false, value: byte });
  const parked = bytes.next();
  const parkedRejected = assert.rejects(parked, /closed/);
  await streamActor.close();
  await parkedRejected;
  assert.equal(streamCancelled, 1);
  const unused = make(0);
  let unusedCancelled = 0;
  const unusedResult = unused.hostAsync.blobRead("unused");
  unused.resolveEffect(requestId(), ok(new ReadableStream<Uint8Array>({ cancel() { unusedCancelled++; } })));
  await unusedResult;
  await unused.close();
  assert.equal(unusedCancelled, 1);
  const abort = new AbortController();
  const aborted = make(0, abort.signal);
  const abortedResult = aborted.hostAsync.documentRead({});
  const abortedRejected = assert.rejects(abortedResult, /closed/);
  abort.abort();
  await aborted.close();
  await abortedRejected;
  const errors = make(0);
  const errorResult = errors.hostAsync.blobRead("denied");
  const errorPack = { tag: "err", val: new Uint8Array([4, 0, 3]) };
  const errorRejected = assert.rejects(errorResult, error => { assert.deepEqual(error, errorPack.val); return true; });
  errors.resolveEffect(requestId(), errorPack);
  await errorRejected;
  const malformed = errors.hostAsync.documentRead({});
  errors.resolveEffect(requestId(), new Uint8Array([4, 0, 3]));
  await assert.rejects(malformed, /invalid result/);
  await errors.close();
  const reserved = make(0);
  let reservedCancelled = 0;
  const materialized = reserved.hostAsync.blobRead("materialized");
  reserved.resolveEffect(requestId(), ok(new ReadableStream<Uint8Array>({ cancel() { reservedCancelled++; } })));
  await materialized;
  const reservations = Array.from({ length: fixture.limits.streams - 1 }, () => reserved.hostAsync.blobRead("reserved"));
  const reservationCount = frames.length;
  const streamOverflow = reserved.hostAsync.blobRead("overflow");
  assert.equal(frames.length, reservationCount);
  await assert.rejects(streamOverflow, /stream capacity/);
  const reservationResults = Promise.allSettled(reservations);
  await reserved.close();
  assert.equal(reservedCancelled, 1);
  assert.equal((await reservationResults).filter(result => result.status === "rejected").length, fixture.limits.streams - 1);
  const bounded = make(0);
  const filled = bounded.hostAsync.blobRead("filled");
  bounded.resolveEffect(requestId(), ok(new Uint8Array(fixture.limits.bufferedStreamBytes)));
  const filledStream = await filled;
  const overflowBytes = bounded.hostAsync.blobRead("over-budget");
  bounded.resolveEffect(requestId(), ok(new Uint8Array([1])));
  await assert.rejects(overflowBytes, /buffered byte capacity/);
  await filledStream.return();
  const recoveredBytes = bounded.hostAsync.blobRead("recovered");
  bounded.resolveEffect(requestId(), ok(new Uint8Array(fixture.stream)));
  assert.deepEqual(await Array.fromAsync(await recoveredBytes), fixture.stream);
  let chunkCancelled = 0;
  const oversized = bounded.hostAsync.blobRead("large-backing");
  bounded.resolveEffect(requestId(), ok(new ReadableStream<Uint8Array>({ start(controller) { controller.enqueue(new Uint8Array(fixture.limits.streamChunkBytes + 1).subarray(0, 1)); }, cancel() { chunkCancelled++; } })));
  await assert.rejects((await oversized).next(), /chunk byte capacity/);
  assert.equal(chunkCancelled, 1);
  await bounded.close();
  for (const kind of ["return", "throw"] as const) {
    const consumer = make(0);
    let cancelled = 0;
    const response = consumer.hostAsync.blobRead(kind);
    consumer.resolveEffect(requestId(), ok(new ReadableStream<Uint8Array>({ cancel() { cancelled++; } })));
    const iterator = await response;
    const parked = iterator.next().then(value => ({ value }), error => ({ error }));
    const reason = new Error("consumer-throw");
    if (kind === "return") assert.deepEqual(await iterator.return(), { done: true, value: undefined });
    else await assert.rejects(iterator.throw(reason), error => error === reason);
    assert.deepEqual(await parked, { value: { done: true, value: undefined } });
    assert.equal(cancelled, 1);
    await consumer.close();
  }
  const cancelFault = make(0);
  let failedCancellations = 0;
  const faultyBody = new ReadableStream<Uint8Array>({ cancel() { failedCancellations++; throw new Error("cancel-fault"); } });
  const faultyResponse = cancelFault.hostAsync.blobRead("cancel-fault");
  cancelFault.resolveEffect(requestId(), ok(faultyBody));
  const faultyIterator = await faultyResponse;
  const faultyNext = faultyIterator.next();
  const faultNextRejected = assert.rejects(faultyNext, error => {
    const bytes = cancelFault.invocationFailure(error);
    assert(bytes instanceof Uint8Array);
    assert.deepEqual(JSON.parse(new TextDecoder().decode(bytes)), fixture.closedFault);
    return true;
  });
  const faultReturnRejected = assert.rejects(faultyIterator.return(), /cancel-fault/);
  const faultCloseRejected = assert.rejects(cancelFault.close(), /retirement failed/);
  await Promise.all([faultNextRejected, faultReturnRejected, faultCloseRejected]);
  assert.equal(failedCancellations, 1);
  assert.equal(faultyBody.locked, false);
  const late = make(0), foreign = make(1);
  let lateController: ReadableStreamDefaultController<Uint8Array> | undefined;
  const lateBody = new ReadableStream<Uint8Array>({ start(controller) { lateController = controller; } }, { highWaterMark: 0 });
  const lateResponse = late.hostAsync.blobRead("late-error");
  late.resolveEffect(requestId(), ok(lateBody));
  const lateRead = (await lateResponse).next().catch(error => error);
  lateController!.error(new Error(fixture.lateStreamFault.message));
  const lateError = await lateRead;
  const canonical = late.invocationFailure(lateError);
  assert(canonical instanceof Uint8Array);
  assert.deepEqual(JSON.parse(new TextDecoder().decode(canonical)), fixture.lateStreamFault);
  assert.equal(foreign.invocationFailure(lateError), lateError);
  const forged = Object.assign(new Error("forged-fault"), { payload: canonical });
  assert.equal(late.invocationFailure(forged), forged);
  await Promise.all([late.close(), foreign.close()]);
  assert.equal(lateBody.locked, false);
  for (const identity of [{ actorId: "", activationGeneration: 1n }, { actorId: "a", activationGeneration: 0n }, { actorId: "a", activationGeneration: 0x10000000000000000n }]) assert.throws(() => createBrowserHostActivation(identity, port), /identity/);
  console.log(`browser-host-activation: AJV=1 TypeScript=1 laws=${fixture.laws.length} actors=2 pending-bound=${fixture.limits.pendingEffects} close-streams=2`);
}
