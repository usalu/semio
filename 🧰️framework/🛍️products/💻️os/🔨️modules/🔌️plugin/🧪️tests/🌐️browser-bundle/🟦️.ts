export function createBrowserBundleTests(dependencies: Record<string, any>, source: { directory: string; url: string }) {
  const { assert, browserActorAsyncImports, browserActorInterfaces, browserBundleValidator, buildBrowserCodegenModule, buildClosedBrowserActorArtifactOwned, buildClosedBrowserActorArtifactV1, captureBrowserActorRuntime, captureBrowserCodegenSources, closeBrowserCodegenModule, closedBrowserActorBundle, closedBrowserActorBundleFromRuntime, closedBrowserComponentFactory, dirname, exactExecutableFingerprint, join, lstatSync, mkdirSync, mkdtempSync, parseBrowserActorCodegenManifest, readdirSync, readFileSync, realpathSync, renameSync, runExactCargoLawProcess, sealBrowserCodegenPolicy, ts, writeFileSync } = dependencies;
  type BrowserActorBuildControl = any;
  async function testClosedBrowserComponentFactory(repoRoot: string): Promise<void> {
    await testBrowserActorCodegenManifest();
    await testBrowserCodegenCapsule(repoRoot);
    await testBrowserCodegenSources(repoRoot);
    await testBrowserCodegenPolicy(repoRoot);
    await (await import("../../🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts")).testBrowserWasiActivation(repoRoot);
    await testBrowserHostActivation();
    await testClosedBrowserActorBundle(repoRoot);
    const fixtureRoot = join(source.directory, "🧫️fixtures/🧊️component-factory");
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
    const root = join(source.directory, "🧫️fixtures/🔒️compiler-capsule");
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
    const root = join(source.directory, "🧫️fixtures/📸️compiler-sources");
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
    const fixture = JSON.parse(readFileSync(join(source.directory, "🧫️fixtures/🔏️codegen-policy/🔣️.json"), "utf8"));
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
    const fixture = JSON.parse(readFileSync(join(source.directory, "🧫️fixtures/📦️codegen-manifest/🔣️.json"), "utf8"));
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
    const root = join(source.directory, "🧫️fixtures/🧊️actor-factory");
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
      writeFileSync(path, readFileSync(join(source.directory, name)), { mode: 0o600 });
    }
    const runtimeSnapshot = captureBrowserActorRuntime(runtimeRoot, () => {});
    for (const name of ["🌐️host/🟦️.ts", "🌐️wasi/🟦️.ts"]) {
      const path = join(runtimeRoot, name);
      renameSync(path, path + ".retained");
      writeFileSync(path, "throw new Error('runtime replacement executed');", { mode: 0o600 });
    }
    const capturedClosure = await closedBrowserActorBundleFromRuntime(input.closed.source, input.closed.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") })), { importInterfaces: input.closed.importInterfaces }, runtimeSnapshot);
    assert.equal(capturedClosure, nodeClosure);
    for (const name of ["🌐️host/🟦️.ts", "🌐️wasi/🟦️.ts"]) writeFileSync(join(runtimeRoot, name), "import './escape.ts';\n" + readFileSync(join(source.directory, name), "utf8"), { mode: 0o600 });
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
    const root = join(source.directory, "🧫️fixtures/🌐️host-activation");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = await browserBundleValidator("HostActivationV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const { createBrowserHostActivation } = await import("../../🌐️browser-bundle/🌐️host/🟦️.ts");
    const program = ts.createProgram([join(source.directory, "🌐️host/🟦️.ts")], { noEmit: true, strict: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, lib: ["lib.es2023.d.ts", "lib.dom.d.ts"], types: [], skipLibCheck: true });
    const diagnostics = ts.getPreEmitDiagnostics(program);
    assert.equal(diagnostics.length, 0, ts.formatDiagnosticsWithColorAndContext(diagnostics, { getCanonicalFileName: path => path, getCurrentDirectory: () => source.directory, getNewLine: () => "\n" }));
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
  return { testClosedBrowserComponentFactory, testBrowserCodegenCapsule, testBrowserCodegenSources, testBrowserCodegenPolicy, testBrowserActorCodegenManifest, testClosedBrowserActorBundle, testBrowserHostActivation };
}
