/** 🌐️ Build-time closure and isolation laws for browser component factories. */
import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import ts from "typescript";
import { runExactCargoLawProcess } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

export type BrowserComponentCore = Readonly<{ name: string; bytes: Uint8Array }>;
export type BrowserComponentFactoryControl = Readonly<{ cancelled?: () => boolean; progress?: (completedBytes: number, totalBytes: number) => void }>;

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
  const closed = ts.createSourceFile("closed.js", `function instantiate(getCoreModule, imports, instantiateCore = WebAssembly.instantiate) ${body}`, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const requested = new Set<string>();
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) || ts.isImportEqualsDeclaration(node) || ts.isExportDeclaration(node)) throw new Error("browser component factory: executable import");
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
  return `function __semioInstantiate(getCoreModule, imports, instantiateCore = WebAssembly.instantiate) ${body}
const __semioEmbeddedCores = Object.freeze(Object.fromEntries(Object.entries(${JSON.stringify(table)}).map(([name, value]) => [name, Object.freeze({ length: value.length, chunks: Object.freeze(value.chunks) })])));
async function instantiateFreshComponent(imports, control = {}) {
  const check = () => { if (control.signal?.aborted) throw new Error("browser component factory: cancelled"); };
  check();
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
  const fixtureRoot = join(import.meta.dir, "🧪️fixtures/🧊️component-factory");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactBase?.includes("🗑️generated"), "browser factory law requires ticket-generated evidence root");
  mkdirSync(artifactBase, { recursive: true });
  const evidence = mkdtempSync(join(artifactBase, "browser-component-factory-"));
  const probe = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import { parse, transpile } from "@bytecodealliance/jco";
    import { readFileSync } from "node:fs";
    const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const result = await transpile(await parse(fixture.component), { name: fixture.name, instantiation: "async", nodejsCompat: false, base64Cutoff: 0, quiet: true });
    process.stdout.write(JSON.stringify({ source: new TextDecoder().decode(result.files[fixture.name + ".js"]), cores: Object.entries(result.files).filter(([name]) => name.endsWith(".wasm")).map(([name, bytes]) => ({ name, hex: Buffer.from(bytes).toString("hex") })) }));
  `, join(fixtureRoot, "🔣️.json")], { cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 2 * 1024 * 1024, stdoutPath: join(evidence, "jco.stdout.json"), stderrPath: join(evidence, "jco.stderr"), cancelled: () => false });
  assert.equal(probe.status, 0, probe.stderr);
  const input = JSON.parse(probe.stdout);
  const cores = input.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") }));
  const factory = await closedBrowserComponentFactory(input.source, cores);
  writeFileSync(join(evidence, "factory.json"), JSON.stringify({ factory, fixture, core: input.cores[0].hex }));
  const runtime = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import assert from "node:assert/strict";
    import { readFileSync } from "node:fs";
    const { factory, fixture, core } = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const module = await import("data:text/javascript;base64," + Buffer.from(factory + "\\nexport { instantiateFreshComponent };").toString("base64"));
    const a = await module.instantiateFreshComponent({});
    const b = await module.instantiateFreshComponent({});
    const actual = fixture.calls.map(name => ({ a, b })[name].next());
    const compiled = await WebAssembly.compile(Buffer.from(core, "hex"));
    const independent = { a: await WebAssembly.instantiate(compiled), b: await WebAssembly.instantiate(compiled) };
    const oracle = fixture.calls.map(name => independent[name].exports.next());
    assert.deepEqual(actual, fixture.expected);
    assert.deepEqual(actual, oracle);
    const nativeCompile = WebAssembly.compile;
    const nativeInstantiate = WebAssembly.instantiate;
    const abort = new AbortController(); abort.abort();
    let compilations = 0, instances = 0, staging;
    WebAssembly.compile = async (...args) => { compilations++; return nativeCompile(...args); };
    WebAssembly.instantiate = (...args) => { instances++; return nativeInstantiate(...args); };
    let cancelledBeforeCompile, cancelledDuringCompile;
    try {
      await assert.rejects(module.instantiateFreshComponent({}, { signal: abort.signal }), /cancelled/);
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
  ];
  assert.equal(hostile.length, fixture.forbidden.length);
  for (const reject of hostile) await assert.rejects(reject);
  let builderProgress = 0;
  await assert.rejects(closedBrowserComponentFactory(input.source, cores, { cancelled: () => true, progress: () => builderProgress++ }), /cancelled/);
  assert.equal(builderProgress, 0);
  console.log(`browser-component-factory: AJV=1 JCO=1 native-Wasm-oracle=1 actors=2 hostile=${hostile.length} cancellation=3 bytes=${factory.length} evidence=${evidence}`);
}
