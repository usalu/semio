/** 🛡️ The closed browser actor bundle test: a real canonical pure import and exact actor shutdown.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | createBrowserBundleTests}: a factory over the same `dependencies` bag and `source` as the root, returning its test functions. */
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { captureOwnedProcess } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import assert from "node:assert/strict";

/** 🏗️ Builds the closed browser actor bundle test over the shared dependency bag. */
export function createBrowserActorBundleTests(dependencies: import("../../../🌐️browser-bundle/📜️script.ts").BrowserBundleTestDependencies, source: { directory: string; url: string }) {
  const { browserActorAsyncImports, browserActorImportAdmissionV1, browserActorInterfaces, browserBundleValidator, buildClosedBrowserActorArtifactOwned, buildClosedBrowserActorArtifactV1, captureBrowserActorRuntime, closedBrowserActorBundle, closedBrowserActorBundleFromRuntime, dirname, join, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, renameSync, writeFileSync } = dependencies;
  type BrowserActorBuildControl = import("../../../🌐️browser-bundle/📜️script.ts").BrowserActorBuildControl;
  /** 🎭️ Qualifies one closed ESM with a real canonical pure import and exact actor shutdown. */
  async function testClosedBrowserActorBundle(repoRoot: string, invocation: ScriptInvocation): Promise<void> {
    const root = join(source.directory, "🧪️tests/🧫️fixtures/🧊️actor-factory");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    
    
    const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
    assert(artifactBase !== undefined && artifactBase.includes("🗑️generated"));
    mkdirSync(artifactBase, { recursive: true });
    const evidence = mkdtempSync(join(artifactBase, "browser-actor-factory-"));
    const probe = await captureOwnedProcess("node", ["--input-type=module", "-e", `
      import { parse, transpile } from "@bytecodealliance/jco";
      import { readFileSync } from "node:fs";
      const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
      const component = await parse(fixture.component);
      const unsupported = await parse(fixture.component.replaceAll("semio:framework/pure@1.0.0", "untrusted:remote/pure@1.0.0"));
      const result = await transpile(component, { name: fixture.name, instantiation: "async", nodejsCompat: false, base64Cutoff: 0, quiet: true, map: Object.fromEntries((fixture.importInterfaces ?? []).map(name => [name, name])) });
      const closed = await transpile(component, { name: "browser-actor", instantiation: "async", nodejsCompat: false, base64Cutoff: 0, quiet: true, map: Object.fromEntries(JSON.parse(process.argv[2]).map(name => [name, name])), asyncMode: "jspi", asyncImports: JSON.parse(process.argv[3]) });
      process.stdout.write(JSON.stringify({ component: Buffer.from(component).toString("hex"), unsupported: Buffer.from(unsupported).toString("hex"), source: new TextDecoder().decode(result.files[fixture.name + ".js"]), cores: Object.entries(result.files).filter(([name]) => name.endsWith(".wasm")).map(([name, bytes]) => ({ name, hex: Buffer.from(bytes).toString("hex") })), closed: { source: new TextDecoder().decode(closed.files["browser-actor.js"]), importInterfaces: closed.imports, cores: Object.entries(closed.files).filter(([name]) => name.endsWith(".wasm")).map(([name, bytes]) => ({ name, hex: Buffer.from(bytes).toString("hex") })) } }));
    `, join(root, "🔣️.json"), JSON.stringify(browserActorInterfaces), JSON.stringify(browserActorAsyncImports)], { invocation, cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 2 * 1024 * 1024, stdoutPath: join(evidence, "jco.stdout.json"), stderrPath: join(evidence, "jco.stderr") });
    assert.equal(probe.status, 0, probe.stderr);
    const input = JSON.parse(probe.stdout);
    const componentBytes = Buffer.from(input.component, "hex");
    for (const key of ["repoRoot", "evidenceRoot"]) {
      const unread = new Proxy(componentBytes, { get() { throw new Error("unreserved input inspected"); } });
      await assert.rejects(buildClosedBrowserActorArtifactV1(unread, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), { [key]: evidence } as unknown as BrowserActorBuildControl), { message: "browser actor artifact: invalid build control" });
    }
    const scratchNames = () => readdirSync(artifactBase).filter(name => name.startsWith("browser-actor-codegen-")).sort();
    const scratchBefore = scratchNames();
    let scratchDuring: string[] = [];
    const artifact = await buildClosedBrowserActorArtifactV1(componentBytes, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), { progress(phase) { if (phase === "codegen") scratchDuring = scratchNames().filter(name => !scratchBefore.includes(name)); } });
    assert.equal(scratchDuring.length, 1);
    assert.deepEqual(scratchNames(), scratchBefore);
    let scratchCancelled = false;
    await assert.rejects(buildClosedBrowserActorArtifactV1(componentBytes, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), { cancelled: () => scratchCancelled, progress(phase) { if (phase === "codegen") { assert.equal(scratchNames().filter(name => !scratchBefore.includes(name)).length, 1); scratchCancelled = true; } } }), { message: "browser actor artifact: cancelled" });
    assert.equal(scratchCancelled, true);
    assert.deepEqual(scratchNames(), scratchBefore);
    const validatePolicy = await browserBundleValidator("CodegenPolicyV1");
    const policy = JSON.parse(artifact.policyCanonical);
    assert(validatePolicy(policy), JSON.stringify(validatePolicy.errors));
    assert.equal(artifact.policySha256, Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(artifact.policyCanonical))).toString("hex"));
    assert.equal(artifact.policyCanonical.includes(repoRoot), false);
    assert(artifact.producer.inputs.length > 20);
    assert.equal(artifact.producer.runtime.path, realpathSync(process.execPath));
    assert.equal(artifact.producer.compiler.byteLength, artifact.producer.compiler.bytes.byteLength);
    assert.equal(Buffer.from(await crypto.subtle.digest("SHA-256", new Uint8Array(artifact.producer.compiler.bytes).buffer)).toString("hex"), policy.compiler.module.sha256);
    for (const row of artifact.producer.inputs) {
      assert.equal(Buffer.from(await crypto.subtle.digest("SHA-256", readFileSync(row.path))).toString("hex"), row.sha256);
      assert.equal(readFileSync(row.path).byteLength, row.byteLength);
    }
    for (const row of [...policy.firstParty,...policy.compiler.inputs,...policy.compiler.cores,policy.parser.entry]) assert(artifact.producer.inputs.some((input:any)=>input.logicalPath===row.logicalPath&&input.sha256===row.sha256&&input.byteLength===row.byteLength));

    assert.equal(policy.compiler.inputs.length, 11);
    assert.equal(policy.firstParty.length, 6);
    assert.deepEqual(artifact.importInterfaces, input.closed.importInterfaces);
    assert.deepEqual(browserActorImportAdmissionV1(componentBytes), { admitted: input.closed.importInterfaces, refused: [] });
    assert.deepEqual(browserActorImportAdmissionV1(Buffer.from(input.unsupported, "hex")), { admitted: [], refused: ["untrusted:remote/pure@1.0.0"] });
    assert.throws(() => browserActorImportAdmissionV1(componentBytes.subarray(0, componentBytes.byteLength - 1)), /malformed component/);
    const nodeClosure = await closedBrowserActorBundle(input.closed.source, input.closed.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") })), invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), { importInterfaces: input.closed.importInterfaces });
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
    const capturedClosure = await closedBrowserActorBundleFromRuntime(input.closed.source, input.closed.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") })), { importInterfaces: input.closed.importInterfaces }, runtimeSnapshot, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0));
    assert.equal(capturedClosure, nodeClosure);
    for (const name of ["🌐️host/🟦️.ts", "🌐️wasi/🟦️.ts"]) writeFileSync(join(runtimeRoot, name), "import './escape.ts';\n" + readFileSync(join(source.directory, name), "utf8"), { mode: 0o600 });
    await assert.rejects(closedBrowserActorBundleFromRuntime(input.closed.source, input.closed.cores.map((core: { name: string; hex: string }) => ({ name: core.name, bytes: Buffer.from(core.hex, "hex") })), { importInterfaces: input.closed.importInterfaces }, captureBrowserActorRuntime(runtimeRoot, () => {}), invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0)), /Bundle failed|runtime import denied/);
    const ambient = Object.fromEntries(["NODE_OPTIONS", "NODE_PATH", "BUN_OPTIONS", "BUN_PRELOAD", "PATH"].map(name => [name, process.env[name]]));
    try {
      process.env.NODE_OPTIONS = "--require=/semio-denied-ambient-preload";
      process.env.NODE_PATH = process.env.PATH = "/semio-denied-ambient-path";
      process.env.BUN_OPTIONS = "--preload=/semio-denied-ambient-preload";
      process.env.BUN_PRELOAD = "/semio-denied-ambient-preload";
      assert.deepEqual(await buildClosedBrowserActorArtifactOwned(componentBytes, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), {}, evidence), artifact);
    } finally {
      for (const [name, value] of Object.entries(ambient)) if (value === undefined) delete process.env[name]; else process.env[name] = value;
    }
    const digest = async (bytes: Uint8Array<ArrayBuffer>) => Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex");
    assert.equal(artifact.schema, fixture.artifact.schema);
    assert.equal(artifact.codegenPolicy, fixture.artifact.codegenPolicy);
    assert.equal(artifact.componentSha256, await digest(componentBytes));
    assert.equal(artifact.sha256, await digest(artifact.bytes));
    assert.equal(artifact.byteLength, artifact.bytes.byteLength);
    assert(artifact.byteLength <= fixture.artifact.maximumBytes);
    assert.deepEqual(artifact.importInterfaces, fixture.importInterfaces);
    const mutable = Buffer.from(componentBytes), phases: string[] = [];
    const repeat = buildClosedBrowserActorArtifactOwned(mutable, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), { progress(phase) { phases.push(phase); if (phase === "snapshot") mutable.fill(0); } }, evidence);
    let rejectedProgress = 0;
    const unread = new Proxy(componentBytes, { get() { throw new Error("unreserved build touched input"); } });
    try {
      await assert.rejects(buildClosedBrowserActorArtifactV1(unread, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), { progress() { rejectedProgress++; } }), /build capacity/);
      assert.equal(rejectedProgress, 0);
    } finally { await repeat; }
    const repeated = await repeat;
    assert(mutable.every(value => value === 0));
    assert.deepEqual(repeated, artifact);
    assert.deepEqual([...new Set(phases)], ["snapshot", "policy", "codegen", "closure", "hash"]);
    await assert.rejects(buildClosedBrowserActorArtifactOwned(Buffer.from(input.unsupported, "hex"), invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), {}, evidence), /unsupported import interface/);
    let cancelled = false;
    await assert.rejects(buildClosedBrowserActorArtifactOwned(componentBytes, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), { cancelled: () => cancelled, progress(phase, completed) { if (phase === "codegen" && completed) cancelled = true; } }, evidence), /cancelled/);
    await assert.rejects(buildClosedBrowserActorArtifactV1(componentBytes, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), { cancelled: () => true }), /cancelled/);
    await assert.rejects(buildClosedBrowserActorArtifactV1(new Uint8Array(8), invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0)), /component header/);
    const replacementRoot = join(evidence, "opened-file-replacement");
    let codegenComplete = false, readChecks = 0, replaced = false;
    await assert.rejects(buildClosedBrowserActorArtifactOwned(componentBytes, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), {
      progress(phase, completed) { if (phase === "codegen" && completed) codegenComplete = true; },
      cancelled() {
        if (codegenComplete && ++readChecks === 2) {
          const rows = readdirSync(replacementRoot);
          assert.equal(rows.length, 1);
          const row=rows[0];assert(row,"generated actor fixture row required");
          const generated = join(replacementRoot, row, "browser-actor.js");
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
      await assert.rejects(buildClosedBrowserActorArtifactOwned(componentBytes, invocation, scriptCompilerAdmissionControlV1(invocation,"test:browser-phase",0), {
        cancelled() {
          const rows = readdirSync(replacementRoot);
          if (!replaced && rows.length === 1) {
            const row=rows[0];assert(row,"captured actor fixture row required");
            const directory = join(replacementRoot, row);
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
      const row=readdirSync(replacementRoot)[0];assert(row,"captured directory fixture required");
      const directory = join(replacementRoot, row);
      assert.equal(readdirSync(directory).some(name => name.startsWith("browser-actor.")), false);
      assert.equal(readFileSync(join(directory, "codegen.stdout.json"), "utf8"), "");
    }
    const bundle = new TextDecoder("utf-8", { fatal: true }).decode(artifact.bytes);
    writeFileSync(join(evidence, "bundle.json"), JSON.stringify({ bundle, fixture, core: input.cores[0].hex }));
    const runtime = await captureOwnedProcess("node", ["--input-type=module", "-e", `
      import assert from "node:assert/strict";
      import { readFileSync } from "node:fs";
      const { bundle, fixture, core } = JSON.parse(readFileSync(process.argv[1], "utf8"));
      const module = await import("data:text/javascript;base64," + Buffer.from(bundle).toString("base64"));
      assert.deepEqual(Object.keys(module), ["activate"]);
      const actors = {};
      for (const name of ["a", "b"]) {
        let clock = BigInt(fixture.starts[name]);
        actors[name] = await module.activate({ actorId: name, activationGeneration: 1n }, { nowMs: () => ++clock, operationClockUs: () => undefined, log() {}, traceSpan() {}, dispatch() { throw new Error("unexpected effect"); }, cancelEffect: () => "closed" });
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
      const port = { operationClockUs: () => undefined, nowMs() {
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
    `, join(evidence, "bundle.json")], { invocation, cwd: repoRoot, env: process.env, budgetMs: 120_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "runtime.stdout.json"), stderrPath: join(evidence, "runtime.stderr") });
    assert.equal(runtime.status, 0, runtime.stderr);
    const { bytes: artifactBytes, ...receipt } = artifact;
    writeFileSync(join(evidence, "closed-artifact.json"), JSON.stringify(receipt), { mode: 0o600 });
    console.log(`browser-actor-factory: AJV=1 JCO=1 native-Wasm-oracle=1 SHA256=node+webcrypto actors=2 laws=${fixture.laws.length} artifact-laws=${fixture.artifactLaws.length} bytes=${artifactBytes.byteLength} evidence=${evidence}`);
  }
  return { testClosedBrowserActorBundle };
}
