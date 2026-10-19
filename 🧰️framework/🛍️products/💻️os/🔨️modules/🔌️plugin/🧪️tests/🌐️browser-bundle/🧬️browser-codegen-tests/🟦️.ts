/** 🧬️ The browser codegen tests: compiler capsule, captured sources, policy snapshots, the async task return lift and the generated manifest.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | createBrowserBundleTests}: a factory over the same `dependencies` bag and `source` as the root, returning its test functions. */
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { captureOwnedProcess } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import assert from "node:assert/strict";

/** 🏗️ Builds the browser codegen test functions over the shared dependency bag. */
export function createBrowserCodegenTests(dependencies: import("../../../🌐️browser-bundle/📜️script.ts").BrowserBundleTestDependencies, source: { directory: string; url: string }) {
  const { browserActorAsyncImports, browserActorInterfaces, browserBundleValidator, buildBrowserCodegenModule, captureBrowserCodegenSources, closeBrowserCodegenModule, validateAsyncTaskReturnLift, exactExecutableFingerprint, join, lstatSync, mkdirSync, mkdtempSync, parseBrowserActorCodegenManifest, readFileSync, realpathSync, renameSync, sealBrowserCodegenPolicy, writeFileSync } = dependencies;
  /** 🔐️ Compares an immutable compiler capsule with native WebAssembly and rejects executable closure escapes. */
  async function testBrowserCodegenCapsule(repoRoot: string, invocation: ScriptInvocation): Promise<void> {
    const root = join(source.directory, "🧪️tests/🧫️fixtures/🔒️compiler-capsule");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    
    
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
    const runtime = await captureOwnedProcess("node", ["--input-type=module", "-e", `
      import assert from "node:assert/strict";
      import { readFileSync } from "node:fs";
      const { fixture, closed } = JSON.parse(readFileSync(process.argv[1], "utf8"));
      const module = await import("data:text/javascript;base64," + Buffer.from(closed).toString("base64"));
      const oracle = await Promise.all(fixture.cores.map(core => WebAssembly.compile(Buffer.from(core.hex, "hex"))));
      assert.equal(await module.generate(), fixture.expected);
      assert.equal(await module.generate(), oracle.length);
      process.stdout.write(JSON.stringify({ actual: await module.generate(), oracle: oracle.length }));
    `, join(evidence, "fixture.json")], { invocation, cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "runtime.stdout.json"), stderrPath: join(evidence, "runtime.stderr") });
    assert.equal(runtime.status, 0, runtime.stderr);
    assert.deepEqual(JSON.parse(runtime.stdout), { actual: fixture.expected, oracle: fixture.expected });
    console.log(`browser-compiler-capsule: AJV=1 native-Wasm-oracle=1 valid=1 denied=${hostile.length} evidence=${evidence}`);
  }

  /** 📸️ Compares captured compiler source semantics with Node and tests replacement after onLoad. */
  async function testBrowserCodegenSources(repoRoot: string, invocation: ScriptInvocation): Promise<void> {
    const root = join(source.directory, "🧪️tests/🧫️fixtures/📸️compiler-sources");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    
    
    const evidence = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "browser-compiler-sources-"));
    const directory = join(evidence, "source");
    mkdirSync(directory);
    const reset = () => { for (const [name, source] of Object.entries(fixture.files)) writeFileSync(join(directory, name), source as string, { mode: 0o600 }); };
    reset();
    const roots = [{ name: "fixture", path: directory }];
    const baseline = await captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => {}, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0));
    assert.deepEqual(baseline.inputs.map(row => row.logicalPath), fixture.inputs);
    for (const row of baseline.inputs) assert.equal(row.sha256, Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(fixture.files[row.logicalPath.slice(8)]))).toString("hex"));
    let replaced = 0;
    const changed = await captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => {}, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), path => {
      renameSync(path, path + ".retained");
      writeFileSync(path, "throw new Error('changed compiler source executed');", { mode: 0o600 });
      replaced++;
    });
    assert.equal(replaced, fixture.inputs.length);
    assert.deepEqual(changed, baseline);
    const oracle = await captureOwnedProcess("node", ["--input-type=module", "-e", `
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
    `, (() => { const path = join(evidence, "oracle.json"); writeFileSync(path, JSON.stringify({ source: changed.source, fixture }), { mode: 0o600 }); return path; })()], { invocation, cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "oracle.stdout.json"), stderrPath: join(evidence, "oracle.stderr") });
    assert.equal(oracle.status, 0, oracle.stderr);
    assert.deepEqual(JSON.parse(oracle.stdout), { answer: fixture.expected });
    reset();
    writeFileSync(join(evidence, "foreign.js"), "export const value = 900;", { mode: 0o600 });
    writeFileSync(join(directory, "entry.js"), "export { value } from '../foreign.js';", { mode: 0o600 });
    await assert.rejects(captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => {}, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0)), /compiler source/);
    writeFileSync(join(directory, "value.json"), "{}", { mode: 0o600 });
    writeFileSync(join(directory, "entry.js"), "export { default } from './value.json';", { mode: 0o600 });
    await assert.rejects(captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => {}, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0)), /compiler source/);
    await assert.rejects(captureBrowserCodegenSources(join(directory, "entry.js"), roots, () => { throw new Error("cancelled compiler source"); }, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0)), /cancelled compiler source/);
    console.log(`browser-compiler-sources: AJV=1 Node-oracle=1 WebCrypto=1 laws=${fixture.laws.length} inputs=${baseline.inputs.length} evidence=${evidence}`);
  }

  /** 🔏️ Compares policy snapshots and digests with independent canonical JSON and WebCrypto. */
  async function testBrowserCodegenPolicy(repoRoot: string, invocation: ScriptInvocation): Promise<void> {
    const fixture = JSON.parse(readFileSync(join(source.directory, "🧪️tests/🧫️fixtures/🔏️codegen-policy/🔣️.json"), "utf8"));
    const validate = await browserBundleValidator("CodegenPolicyV1");
    const { default: stableStringify } = await import("fast-json-stable-stringify");
    const digest = { sha256: "a".repeat(64), byteLength: 8 };
    const input = {
      schema: "semio.os.browser-codegen-policy.v1", revision: 1,
      runtime: { kind: "bun", version: "1.4.2", executable: { ...digest } },
      compiler: { module: { ...digest }, inputs: [{ logicalPath: "compiler/entry.js", ...digest }], cores: ["compiler/core.wasm", "compiler/core2.wasm"].map(logicalPath => ({ logicalPath, ...digest })) },
      packages: fixture.packages.map(([name, version]: string[]) => ({ name, version, lockSha256: digest.sha256, manifestSha256: digest.sha256 })),
      parser: { name: "typescript", version: "5.9.3", manifestSha256: digest.sha256, lockRowSha256: digest.sha256, entry: { logicalPath: "typescript/lib/typescript.js", ...digest } },
      firstParty: [{ logicalPath: "browser/script.ts", ...digest }],
      options: { jco: "1.34.0", entrypoint: "@bytecodealliance/jco/component", target: "browser", format: "esm", name: "browser-actor", instantiation: "async", asyncMode: "jspi", nodejsCompat: false, base64Cutoff: 0, importInterfaces: [...browserActorInterfaces], asyncImports: [...browserActorAsyncImports] },
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
    const inputCompilerEntry=input.compiler.inputs[0],sealedCompilerEntry=sealed.record.compiler.inputs[0];
    assert(inputCompilerEntry&&sealedCompilerEntry,"compiler input fixture required");
    inputCompilerEntry.sha256 = "c".repeat(64);
    assert.equal(sealedCompilerEntry.sha256, digest.sha256);
    assert(Object.isFrozen(sealedCompilerEntry));
    const executable = exactExecutableFingerprint(realpathSync(process.execPath));
    assert.equal(executable.path, realpathSync(process.execPath));
    assert.equal(executable.byteLength, lstatSync(executable.path).size);
    assert.match(executable.sha256, /^[a-f0-9]{64}$/);
    const evidence = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "browser-codegen-policy-"));
    const compiler = await buildBrowserCodegenModule(evidence, () => {}, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0));
    const capturedInputs = structuredClone(compiler.inputs);
    writeFileSync(join(evidence, "compiler-sources.json"), "[]", { mode: 0o600 });
    assert.deepEqual(compiler.inputs, capturedInputs);
    assert.equal(compiler.inputs.length, 11);
    assert(compiler.inputs.every(Object.isFrozen));
    assert.equal(compiler.cores.length, 2);
    console.log(`browser-codegen-policy: AJV=1 stable-stringify=1 WebCrypto=1 laws=${fixture.laws.length} inputs=${targets.length + input.packages.length * 2}`);
  }

  /** 🪝️ Pins the refusal that an unflattenable async result may never be bound to direct `task.return` params. */
  function testAsyncTaskReturnLift(): void {
    const fixture = JSON.parse(readFileSync(join(source.directory, "🧪️tests/🧫️fixtures/🪝️async-task-return/🔣️.json"), "utf8"));
    for (const admitted of fixture.admitted) assert.equal(validateAsyncTaskReturnLift(admitted), fixture.bindings);
    for (const denied of fixture.denied) assert.throws(() => validateAsyncTaskReturnLift(denied), /unflattenable async result requires an indirect task\.return/);
    assert.equal(validateAsyncTaskReturnLift(""), 0);
    console.log(`async-task-return-lift: TypeScript=1 admitted=${fixture.admitted.length} bindings=${fixture.bindings} denied=${fixture.denied.length}`);
  }

  /** 📦️ Compares generated-manifest admission with the strict schema and canonical array ordering. */
  async function testBrowserActorCodegenManifest(): Promise<void> {
    const fixture = JSON.parse(readFileSync(join(source.directory, "🧪️tests/🧫️fixtures/📦️codegen-manifest/🔣️.json"), "utf8"));
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
  return { testBrowserCodegenCapsule, testBrowserCodegenSources, testBrowserCodegenPolicy, testAsyncTaskReturnLift, testBrowserActorCodegenManifest };
}
