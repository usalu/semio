/** 🏭️ The closed browser component factory test: AJV, jco, the native WebAssembly oracle, hostile sources and cancellation.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | createBrowserBundleTests}: a factory over the same `dependencies` bag and `source` as the root, returning its test functions. */
import { captureOwnedProcess } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import assert from "node:assert/strict";
import type { createBrowserCodegenTests } from "../🧬️browser-codegen-tests/🟦️.ts";
import type { createBrowserHostActivationTests } from "../🪢️host-activation-tests/🟦️.ts";
import type { createBrowserActorBundleTests } from "../🛡️actor-bundle-tests/🟦️.ts";

/** 🔧️ Builds the closed browser component factory test, which first runs every other browser bundle test. */
export function createClosedBrowserComponentFactoryTest(dependencies: import("../../../🌐️browser-bundle/📜️script.ts").BrowserBundleTestDependencies, source: { directory: string; url: string }, { testBrowserCodegenCapsule, testBrowserCodegenSources, testBrowserCodegenPolicy, testAsyncTaskReturnLift, testBrowserActorCodegenManifest }: Pick<ReturnType<typeof createBrowserCodegenTests>, "testBrowserCodegenCapsule" | "testBrowserCodegenSources" | "testBrowserCodegenPolicy" | "testAsyncTaskReturnLift" | "testBrowserActorCodegenManifest">, { testBrowserHostActivation }: Pick<ReturnType<typeof createBrowserHostActivationTests>, "testBrowserHostActivation">, { testClosedBrowserActorBundle }: Pick<ReturnType<typeof createBrowserActorBundleTests>, "testClosedBrowserActorBundle">) {
  const { closedBrowserComponentFactory, join, mkdirSync, mkdtempSync, readFileSync, writeFileSync } = dependencies;
  async function testClosedBrowserComponentFactory(repoRoot: string, invocation: ScriptInvocation): Promise<void> {
    await testBrowserActorCodegenManifest();
    testAsyncTaskReturnLift();
    await testBrowserCodegenCapsule(repoRoot, invocation);
    await testBrowserCodegenSources(repoRoot, invocation);
    await testBrowserCodegenPolicy(repoRoot, invocation);
    await (await import("../../../🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts")).testBrowserWasiActivation(repoRoot, invocation);
    await testBrowserHostActivation();
    await testClosedBrowserActorBundle(repoRoot, invocation);
    const fixtureRoot = join(source.directory, "🧪️tests/🧫️fixtures/🧊️component-factory");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    
    
    const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
    assert(artifactBase !== undefined && artifactBase.includes("🗑️generated"), "browser factory law requires ticket-generated evidence root");
    mkdirSync(artifactBase, { recursive: true });
    const evidence = mkdtempSync(join(artifactBase, "browser-component-factory-"));
    const probe = await captureOwnedProcess("node", ["--input-type=module", "-e", `
      import { parse, transpile } from "@bytecodealliance/jco";
      import { readFileSync } from "node:fs";
      const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
      const result = await transpile(await parse(fixture.component), { name: fixture.name, instantiation: "async", nodejsCompat: false, base64Cutoff: 0, quiet: true, map: Object.fromEntries((fixture.importInterfaces ?? []).map(name => [name, name])) });
      process.stdout.write(JSON.stringify({ source: new TextDecoder().decode(result.files[fixture.name + ".js"]), cores: Object.entries(result.files).filter(([name]) => name.endsWith(".wasm")).map(([name, bytes]) => ({ name, hex: Buffer.from(bytes).toString("hex") })) }));
    `, join(fixtureRoot, "🔣️.json")], { invocation, cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 2 * 1024 * 1024, stdoutPath: join(evidence, "jco.stdout.json"), stderrPath: join(evidence, "jco.stderr") });
    assert.equal(probe.status, 0, probe.stderr);
    const input = JSON.parse(probe.stdout);
    const cores = input.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") }));
    const factory = await closedBrowserComponentFactory(input.source, cores);
    const explicitAsync = await closedBrowserComponentFactory(input.source.replace("export function instantiate(getCoreModule, imports, instantiateCore = WebAssembly.instantiate) {", "export async function instantiate(getCoreModule, imports, instantiateCore = WebAssembly.instantiate) { await Promise.resolve();"), cores);
    assert(explicitAsync.startsWith("async function __semioInstantiate"));
    writeFileSync(join(evidence, "factory.json"), JSON.stringify({ factory, explicitAsync, fixture, core: input.cores[0].hex }));
    const runtime = await captureOwnedProcess("node", ["--input-type=module", "-e", `
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
    `, join(evidence, "factory.json")], { invocation, cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "runtime.stdout.json"), stderrPath: join(evidence, "runtime.stderr") });
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
  return { testClosedBrowserComponentFactory };
}
