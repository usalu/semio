#!/usr/bin/env bun
/** 🌊️ Qualifies actor-isolated canonical async result and stream imports through JCO/Wasm. */
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import { buildClosedBrowserActorArtifactV1 } from "../../📜️script.ts";
import { runExactCargoLawProcess } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

export type ActorImportCore = Readonly<{ name: string; bytes: Uint8Array }>;
export type ActorImportFactoryPort = (source: string, cores: readonly ActorImportCore[], control: Readonly<{ importInterfaces: readonly string[] }>) => Promise<string>;
type ActorSpec = Readonly<{ actorId: string; link: Readonly<{ okRequest: readonly number[]; okValue: readonly number[]; errorRequest: readonly number[]; errorValue: readonly number[] }>; blob: Readonly<{ okHash: string; chunks: readonly (readonly number[])[]; errorHash: string; errorValue: readonly number[] }>; timer: Readonly<{ delayMs: number }> }>;
type JcoPolicy = Readonly<{ asyncMode: "jspi"; asyncImports: readonly string[] }>;
type EmittedAsyncBinding = Readonly<{ trampoline: string; fnName: string }>;
type Fixture = Readonly<{ identity: Readonly<{ component: string; canonicalInterface: string; expectedImports: readonly string[]; jco: JcoPolicy; emittedAsyncBindings: readonly EmittedAsyncBinding[]; unsupportedImport: string; productionHostAsyncAbiQualified: true; artifactPolicy: string }>; actors: readonly ActorSpec[]; limits: Readonly<{ buildBudgetMs: number; runtimeBudgetMs: number; maximumOutputBytes: number; actors: number }>; streamClose: Readonly<{ phase: "closed"; activeInvocations: 0; cancellations: 1; result: "rejected"; locked: false }>; guestStreamDrop: Readonly<{ phase: "closed"; activeInvocations: 0; cancellations: 1; result: "fulfilled"; locked: false }>; lateStreamFault: Readonly<{ code: string; message: string }>; failedStreamRetirement: Readonly<{ code: string; message: string; cancelMessage: string }> }>;

function repositoryRoot(start: string): string {
  let cursor = resolve(start);
  while (!existsSync(join(cursor, "nx.json"))) {
    const parent = dirname(cursor);
    if (parent === cursor) throw new Error("actor import fixture: repository root unavailable");
    cursor = parent;
  }
  return cursor;
}

function targetRoot(): string {
  const target = process.env.CARGO_TARGET_DIR;
  assert(target && resolve(target).split(/[\\/]/u).includes("🗑️generated"), "actor import fixture requires ticket-generated CARGO_TARGET_DIR");
  return resolve(target);
}

/** 🧪️ Builds the standalone guest and executes two independent canonical actors. */
export async function testCanonicalActorAsyncImport(repoRoot: string, closeFactory?: ActorImportFactoryPort, closeActorBundle?: ActorImportFactoryPort, pendingHostClose = true): Promise<void> {
  const fixtureRoot = import.meta.dir;
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as Fixture;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(new Set(fixture.actors.map(actor => actor.actorId)).size, fixture.limits.actors);
  const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactBase?.includes("🗑️generated"), "actor import fixture requires ticket-generated evidence root");
  mkdirSync(artifactBase, { recursive: true });
  const evidence = mkdtempSync(join(artifactBase, "actor-import-"));
  const manifest = join(fixtureRoot, "👽️guest", "📦️packages", "🦀️rust", "Cargo.toml");
  const build = await runExactCargoLawProcess("cargo", ["build", "--manifest-path", manifest, "--target", "wasm32-wasip2", "--release", "--offline"], {
    cwd: repoRoot,
    env: { ...process.env, CARGO_TARGET_DIR: targetRoot(), CARGO_BUILD_JOBS: "1" },
    budgetMs: fixture.limits.buildBudgetMs,
    maxOutputBytes: 8 * 1024 * 1024,
    stdoutPath: join(evidence, "cargo.stdout"),
    stderrPath: join(evidence, "cargo.stderr"),
    cancelled: () => false,
  });
  assert.equal(build.status, 0, `${build.reason}: ${build.stderr}`);
  const component = readFileSync(join(targetRoot(), "wasm32-wasip2", "release", "semio_browser_actor_import_guest.wasm"));
  const componentPath = join(evidence, "actor-import.component.wasm");
  const jcoRoot = join(evidence, "jco");
  mkdirSync(jcoRoot);
  writeFileSync(componentPath, component, { mode: 0o600 });
  const transpiled = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import { transpile } from "@bytecodealliance/jco";
    import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
    import { dirname, join } from "node:path";
    const [componentPath, outputRoot, importsJson, policyJson] = process.argv.slice(1);
    const expectedImports = JSON.parse(importsJson);
    const policy = JSON.parse(policyJson);
    const result = await transpile(readFileSync(componentPath), { name: "actor-import", instantiation: "async", nodejsCompat: false, base64Cutoff: 0, quiet: true, map: Object.fromEntries(expectedImports.map(name => [name, name])), asyncMode: policy.asyncMode, asyncImports: policy.asyncImports });
    for (const [name, bytes] of Object.entries(result.files)) {
      const path = join(outputRoot, name);
      mkdirSync(dirname(path), { recursive: true });
      writeFileSync(path, bytes);
    }
    process.stdout.write(JSON.stringify({ imports: result.imports, exports: result.exports, files: Object.keys(result.files), policy }));
  `, componentPath, jcoRoot, JSON.stringify(fixture.identity.expectedImports), JSON.stringify(fixture.identity.jco)], {
    cwd: repoRoot,
    env: process.env,
    budgetMs: fixture.limits.runtimeBudgetMs,
    maxOutputBytes: fixture.limits.maximumOutputBytes,
    stdoutPath: join(evidence, "jco.stdout.json"),
    stderrPath: join(evidence, "jco.stderr"),
    cancelled: () => false,
  });
  assert.equal(transpiled.status, 0, `${transpiled.reason}: ${transpiled.stderr}`);
  const generated = JSON.parse(transpiled.stdout) as { imports: string[]; exports: [string, string][]; files: string[]; policy: JcoPolicy };
  const source = readFileSync(join(jcoRoot, "actor-import.js"), "utf8");
  assert.equal(source.match(/_trampoline\d+\.manuallyAsync = true;/gu)?.length, fixture.identity.emittedAsyncBindings.length);
  for (const binding of fixture.identity.emittedAsyncBindings) {
    const implementation = `_trampoline${binding.trampoline}`;
    assert(source.includes(`${implementation}.fnName = '${binding.fnName}';`), `JCO omitted ${binding.fnName}`);
    assert(source.includes(`${implementation}.manuallyAsync = true;`), `JCO left ${binding.fnName} synchronous`);
    assert(source.includes(`let trampoline${binding.trampoline} = ${implementation}.manuallyAsync ? new WebAssembly.Suspending(`), `JCO omitted Suspended wrapper for ${binding.fnName}`);
  }
  const cores = generated.files.filter(name => name.endsWith(".wasm")).map(name => ({ name, bytes: readFileSync(join(jcoRoot, name)) }));
  const importInterfaces = [...generated.imports].sort();
  writeFileSync(join(evidence, "jco-imports.json"), JSON.stringify(importInterfaces, null, 2), { mode: 0o600 });
  assert.deepEqual(importInterfaces, [...fixture.identity.expectedImports].sort(), `JCO imports lost canonical versions: ${importInterfaces.join(",")}`);
  assert.deepEqual(generated.policy, fixture.identity.jco);
  writeFileSync(join(evidence, "jco-policy.json"), JSON.stringify(generated.policy, null, 2), { mode: 0o600 });
  assert(cores.length > 0);
  let actorModulePath: string | undefined;
  if (closeActorBundle) {
    const blocking = fixture.identity.emittedAsyncBindings.find(binding => binding.fnName === "wasi:io/poll@0.2.9#block");
    assert(blocking);
    const broken = source.replace(`_trampoline${blocking.trampoline}.manuallyAsync = true;`, `_trampoline${blocking.trampoline}.manuallyAsync = false;`);
    assert.notEqual(broken, source);
    await assert.rejects(closeActorBundle(broken, cores, { importInterfaces }), /blocking WASI import requires JSPI suspension/);
    const mapping = `'[method]pollable.block': trampoline${blocking.trampoline},`;
    const decoySuspension = source.replace(mapping, `'[method]pollable.block': _trampoline${blocking.trampoline},`);
    assert.notEqual(decoySuspension, source);
    await assert.rejects(closeActorBundle(decoySuspension, cores, { importInterfaces }), /blocking WASI import requires JSPI suspension/);
    const conditionalFalse = source.replace(`let trampoline${blocking.trampoline} = _trampoline${blocking.trampoline}.manuallyAsync ?`, `let trampoline${blocking.trampoline} = false ?`);
    assert.notEqual(conditionalFalse, source);
    await assert.rejects(closeActorBundle(conditionalFalse, cores, { importInterfaces }), /blocking WASI import requires JSPI suspension/);
    const indirectSuspension = source.replace(mapping, `'[method]pollable.block': (value => value)(trampoline${blocking.trampoline}),`);
    assert.notEqual(indirectSuspension, source);
    await assert.rejects(closeActorBundle(indirectSuspension, cores, { importInterfaces }), /blocking WASI import requires JSPI suspension/);
    await assert.rejects(closeActorBundle(source, cores, { importInterfaces: [...importInterfaces, fixture.identity.unsupportedImport] }), /unsupported import interface/);
    actorModulePath = join(evidence, "actor-import.closed.js");
    const { bytes, ...receipt } = await buildClosedBrowserActorArtifactV1(component, {});
    const digest = async (value: Uint8Array) => Buffer.from(await crypto.subtle.digest("SHA-256", value)).toString("hex");
    assert.equal(receipt.codegenPolicy, fixture.identity.artifactPolicy);
    assert.equal(receipt.componentSha256, await digest(component));
    assert.equal(receipt.sha256, await digest(bytes));
    assert.equal(receipt.byteLength, bytes.byteLength);
    assert.deepEqual(receipt.importInterfaces, importInterfaces);
    writeFileSync(join(evidence, "closed-artifact.json"), JSON.stringify(receipt), { mode: 0o600 });
    writeFileSync(actorModulePath, bytes, { mode: 0o600 });
  }
  const closed = closeFactory ? await closeFactory(source, cores, { importInterfaces }) : undefined;
  const modulePath = join(evidence, "actor-import.js");
  writeFileSync(modulePath, closed ? `${closed}\nexport { instantiateFreshComponent };\n` : source, { mode: 0o600 });
  for (const core of cores) writeFileSync(join(evidence, core.name), core.bytes, { mode: 0o600 });
  const inputPath = join(evidence, "input.json");
  writeFileSync(inputPath, JSON.stringify({ actors: fixture.actors, canonicalInterface: fixture.identity.canonicalInterface, component: fixture.identity.component, importInterfaces, moduleUrl: pathToFileURL(modulePath).href, mode: closed ? "closed" : "raw" }), { mode: 0o600 });
  const runtime = await runExactCargoLawProcess("node", ["--experimental-wasm-jspi", "--input-type=module", "-e", `
    import assert from "node:assert/strict";
    import { readFileSync } from "node:fs";
    import * as cli from "@bytecodealliance/preview2-shim/cli";
    import * as clocks from "@bytecodealliance/preview2-shim/clocks";
    import * as io from "@bytecodealliance/preview2-shim/io";
    const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const module = await import(fixture.moduleUrl);
    const preview = {
      "wasi:cli/environment@0.2.0": cli.environment, "wasi:cli/exit@0.2.0": cli.exit,
      "wasi:cli/stderr@0.2.0": cli.stderr, "wasi:cli/stdin@0.2.0": cli.stdin, "wasi:cli/stdout@0.2.0": cli.stdout,
      "wasi:cli/terminal-input@0.2.0": cli.terminalInput, "wasi:cli/terminal-output@0.2.0": cli.terminalOutput,
      "wasi:cli/terminal-stderr@0.2.0": cli.terminalStderr, "wasi:cli/terminal-stdin@0.2.0": cli.terminalStdin,
      "wasi:cli/terminal-stdout@0.2.0": cli.terminalStdout, "wasi:clocks/monotonic-clock@0.2.0": clocks.monotonicClock,
      "wasi:io/error@0.2.0": io.error, "wasi:io/poll@0.2.0": io.poll, "wasi:io/streams@0.2.0": io.streams,
    };
    const same = (left, right) => left.length === right.length && left.every((value, index) => value === right[index]);
    const normalize = value => ({ tag: value.tag, val: [...value.val] });
    const observe = async operation => {
      try { return { tag: "ok", val: [...await operation] }; }
      catch (error) {
        if (!(error instanceof Uint8Array)) throw error;
        return { tag: "err", val: [...error] };
      }
    };
    const unwrap = result => {
      if (result.tag === "ok") return result.val;
      throw result.val;
    };
    const instantiate = imports => fixture.mode === "closed" ? module.instantiateFreshComponent(imports) : module.instantiate(name => WebAssembly.compile(readFileSync(new URL(name, fixture.moduleUrl))), imports);
    const runtimes = new Map();
    for (const actor of fixture.actors) {
      const trace = [];
      const canonicalHost = {
        async linkResolve(request) {
          const bytes = [...request];
          const result = same(bytes, actor.link.okRequest) ? { tag: "ok", val: new Uint8Array(actor.link.okValue) } : { tag: "err", val: new Uint8Array(actor.link.errorValue) };
          trace.push({ actorId: actor.actorId, kind: "link", request: bytes, result: normalize(result) });
          return result;
        },
        async blobRead(hash) {
          if (hash === actor.blob.okHash) {
            const chunks = actor.blob.chunks.map(chunk => [...chunk]);
            trace.push({ actorId: actor.actorId, kind: "blob", hash, tag: "ok", bytes: chunks.flat() });
            return { tag: "ok", val: (async function* () { for (const chunk of chunks) for (const byte of chunk) yield byte; })() };
          }
          const result = { tag: "err", val: new Uint8Array(actor.blob.errorValue) };
          trace.push({ actorId: actor.actorId, kind: "blob", hash, tag: "err", bytes: [...result.val] });
          return result;
        },
      };
      const host = {
        async linkResolve(request) { return unwrap(await canonicalHost.linkResolve(request)); },
        async blobRead(hash) { return unwrap(await canonicalHost.blobRead(hash)); },
      };
      const imports = { ...preview, [fixture.canonicalInterface]: host };
      assert.deepEqual(Object.keys(imports).filter(name => fixture.importInterfaces.includes(name)).sort(), fixture.importInterfaces);
      const instance = await instantiate(imports);
      const probe = instance[fixture.component] ?? instance.actorImportProbe;
      assert(probe, "actor import probe export unavailable");
      runtimes.set(actor.actorId, { actor, probe, trace });
    }
    const observations = await Promise.all([...runtimes.values()].map(async ({ actor, probe, trace }) => {
      const linkOk = await observe(probe.linkRoundtrip(new Uint8Array(actor.link.okRequest)));
      const linkErr = await observe(probe.linkRoundtrip(new Uint8Array(actor.link.errorRequest)));
      const blobOk = await observe(probe.blobCollect(actor.blob.okHash));
      const blobErr = await observe(probe.blobCollect(actor.blob.errorHash));
      const timerStarted = performance.now();
      let timerSettled = false;
      const timerOperation = Promise.resolve(probe.timerRoundtrip(actor.timer.delayMs)).then(value => { timerSettled = true; return value; });
      await Promise.resolve();
      assert.equal(timerSettled, false);
      const timer = await timerOperation;
      assert.deepEqual(linkOk, { tag: "ok", val: actor.link.okValue }); assert.deepEqual(linkErr, { tag: "err", val: actor.link.errorValue });
      assert.deepEqual(blobOk, { tag: "ok", val: actor.blob.chunks.flat() }); assert.deepEqual(blobErr, { tag: "err", val: actor.blob.errorValue });
      assert.equal(timer, actor.timer.delayMs);
      assert(performance.now() - timerStarted >= actor.timer.delayMs - 2);
      assert(trace.every(entry => entry.actorId === actor.actorId));
      return { actorId: actor.actorId, linkOk, linkErr, blobOk, blobErr, timer, trace };
    }));
    assert.notEqual(runtimes.get("actor-a").trace, runtimes.get("actor-b").trace);
    console.log(JSON.stringify({ observations, component: fixture.component, canonicalInterface: fixture.canonicalInterface, imports: fixture.importInterfaces, mode: fixture.mode, jco: 1, wasm: 1 }));
  `, inputPath], {
    cwd: repoRoot,
    env: process.env,
    budgetMs: fixture.limits.runtimeBudgetMs,
    maxOutputBytes: fixture.limits.maximumOutputBytes,
    stdoutPath: join(evidence, "runtime.stdout.json"),
    stderrPath: join(evidence, "runtime.stderr"),
    cancelled: () => false,
  });
  assert.equal(runtime.status, 0, `${runtime.reason}: ${runtime.stderr}`);
  const observation = JSON.parse(runtime.stdout) as { observations: unknown[]; canonicalInterface: string; mode: string; jco: number; wasm: number };
  assert.equal(observation.observations.length, fixture.actors.length);
  assert.equal(observation.canonicalInterface, fixture.identity.canonicalInterface);
  assert.equal(observation.jco, 1);
  assert.equal(observation.wasm, 1);
  let closedLaws = 0;
  if (actorModulePath) {
    const actorInputPath = join(evidence, "closed-input.json");
    writeFileSync(actorInputPath, JSON.stringify({ actors: fixture.actors, component: fixture.identity.component, moduleUrl: pathToFileURL(actorModulePath).href, pendingHostClose, streamClose: fixture.streamClose, guestStreamDrop: fixture.guestStreamDrop, lateStreamFault: fixture.lateStreamFault, failedStreamRetirement: fixture.failedStreamRetirement }), { mode: 0o600 });
    const closedRuntime = await runExactCargoLawProcess("node", ["--unhandled-rejections=strict", "--experimental-wasm-jspi", "--input-type=module", "-e", `
      import assert from "node:assert/strict";
      import { readFileSync } from "node:fs";
      const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
      const module = await import(fixture.moduleUrl);
      assert.deepEqual(Object.keys(module), ["activate"]);
      const same = (left, right) => left.length === right.length && left.every((value, index) => value === right[index]);
      const observe = async operation => { try { return { tag: "ok", val: [...await operation] }; } catch (error) { if (!(error instanceof Uint8Array)) throw error; return { tag: "err", val: [...error] }; } };
      const hostFault = async (operation, code = "capability-revoked", message = "browser host: closed") => {
        const payload = await operation.then(() => assert.fail("host effect unexpectedly completed"), error => error);
        assert(payload instanceof Uint8Array, JSON.stringify({ type: typeof payload, message: payload && Object.getOwnPropertyDescriptor(payload, "message")?.value, ownPayload: payload && Object.hasOwn(payload, "payload") }));
        const fault = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(payload));
        assert.deepEqual(fault, { origin: "os", code, severity: "error", message, scope: {}, retryable: false });
        return fault;
      };
      const settle = async predicate => { for (let attempt = 0; attempt < 128; attempt++) { if (predicate()) return; await new Promise(resolve => setTimeout(resolve, 0)); } throw new Error("actor import fixture: settlement deadline"); };
      const basePort = (actor, replies = true, cancellation = "queued") => {
        let runtime;
        const frames = [], writes = [], cancellations = [];
        const port = {
          dispatch(frame) {
            frames.push(frame);
            if (!replies) return;
            const request = frame.frame.envelope.payload.payload;
            queueMicrotask(() => {
              if (request.effect === "link-resolve") {
                const bytes = [...request.params.link];
                const result = same(bytes, actor.link.okRequest) ? { tag: "ok", val: new Uint8Array(actor.link.okValue) } : { tag: "err", val: new Uint8Array(actor.link.errorValue) };
                assert.equal(runtime.resolveEffect(request.requestId, result), true);
              } else if (request.effect === "blob-read") {
                const result = request.params.hash === actor.blob.okHash
                  ? { tag: "ok", val: new ReadableStream({ start(controller) { for (const chunk of actor.blob.chunks) controller.enqueue(new Uint8Array(chunk)); controller.close(); } }) }
                  : { tag: "err", val: new Uint8Array(actor.blob.errorValue) };
                assert.equal(runtime.resolveEffect(request.requestId, result), true);
              } else throw new Error("unexpected actor import effect");
            });
          },
          cancelEffect(requestId) { cancellations.push(requestId); return cancellation; },
          log() {}, traceSpan() {}, nowMs: () => 1n,
          wasi: { nowNs: () => process.hrtime.bigint(), write(channel, bytes) { writes.push({ channel, bytes: [...bytes] }); } },
        };
        return { port, frames, writes, cancellations, bind(value) { runtime = value; } };
      };
      const runtimes = new Map();
      for (const actor of fixture.actors) {
        const connection = basePort(actor);
        const runtime = await module.activate({ actorId: actor.actorId, activationGeneration: 1n }, connection.port);
        connection.bind(runtime);
        runtimes.set(actor.actorId, { actor, runtime, connection });
      }
      const observations = await Promise.all([...runtimes.values()].map(async ({ actor, runtime, connection }) => {
        const path = method => [fixture.component, method];
        const linkOk = await observe(runtime.invoke(path("linkRoundtrip"), [new Uint8Array(actor.link.okRequest)]));
        const linkErr = await observe(runtime.invoke(path("linkRoundtrip"), [new Uint8Array(actor.link.errorRequest)]));
        const blobOk = await observe(runtime.invoke(path("blobCollect"), [actor.blob.okHash]));
        const blobErr = await observe(runtime.invoke(path("blobCollect"), [actor.blob.errorHash]));
        let timerSettled = false;
        const timerStarted = performance.now();
        const timerOperation = runtime.invoke(path("timerRoundtrip"), [actor.timer.delayMs]).then(value => { timerSettled = true; return value; });
        await Promise.resolve();
        assert.equal(timerSettled, false);
        assert.deepEqual(runtime.progress(), { phase: "open", activeInvocations: 1 });
        const sibling = await observe(runtime.invoke(path("linkRoundtrip"), [new Uint8Array(actor.link.okRequest)]));
        assert.deepEqual(sibling, { tag: "ok", val: actor.link.okValue });
        const timer = await timerOperation;
        assert.deepEqual(linkOk, { tag: "ok", val: actor.link.okValue }); assert.deepEqual(linkErr, { tag: "err", val: actor.link.errorValue });
        assert.deepEqual(blobOk, { tag: "ok", val: actor.blob.chunks.flat() }); assert.deepEqual(blobErr, { tag: "err", val: actor.blob.errorValue });
        assert.equal(timer, actor.timer.delayMs);
        assert(performance.now() - timerStarted >= actor.timer.delayMs - 2);
        assert(connection.frames.every(frame => frame.actorId === actor.actorId));
        assert.deepEqual(runtime.progress(), { phase: "open", activeInvocations: 0 });
        return { actorId: actor.actorId, linkOk, linkErr, blobOk, blobErr, timer, frames: connection.frames.length };
      }));
      await Promise.all([...runtimes.values()].map(({ runtime }) => runtime.close()));
      assert([...runtimes.values()].every(({ runtime }) => runtime.progress().phase === "closed"));
      let hostCloseLaws = 0;
      const pendingActor = fixture.actors[0];
      const pendingConnection = basePort(pendingActor);
      const pending = await module.activate({ actorId: "pending-close", activationGeneration: 1n }, pendingConnection.port);
      pendingConnection.bind(pending);
      const pendingCall = assert.rejects(pending.invoke([fixture.component, "timerRoundtrip"], [32]), /browser wasi: closed/);
      await settle(() => pending.progress().activeInvocations === 1);
      assert.deepEqual(pending.progress(), { phase: "open", activeInvocations: 1 });
      const pendingClose = pending.close();
      assert.equal(pending.close(), pendingClose);
      assert.deepEqual(pending.progress(), { phase: "closing", activeInvocations: 1 });
      await pendingCall;
      await pendingClose;
      assert.deepEqual(pending.progress(), { phase: "closed", activeInvocations: 0 });
      if (fixture.pendingHostClose) {
        const hostConnection = basePort(pendingActor, false, "queued");
        const hostPending = await module.activate({ actorId: "pending-host-close", activationGeneration: 1n }, hostConnection.port);
        hostConnection.bind(hostPending);
        const hostCall = hostFault(hostPending.invoke([fixture.component, "linkRoundtrip"], [new Uint8Array(pendingActor.link.okRequest)]));
        await settle(() => hostConnection.frames.length === 1);
        await hostPending.close();
        await hostCall;
        assert.deepEqual(hostPending.progress(), { phase: "closed", activeInvocations: 0 });
        assert.equal(hostConnection.cancellations.length, 1);
        const failedConnection = basePort(pendingActor, false, "denied");
        const failed = await module.activate({ actorId: "cancel-admission-failure", activationGeneration: 1n }, failedConnection.port);
        failedConnection.bind(failed);
        const failedCall = hostFault(failed.invoke([fixture.component, "linkRoundtrip"], [new Uint8Array(pendingActor.link.okRequest)]));
        await settle(() => failedConnection.frames.length === 1);
        const failedClose = failed.close();
        await failedCall;
        await assert.rejects(failedClose, /retirement failed/);
        assert.deepEqual(failed.progress(), { phase: "closed", activeInvocations: 0 });
        const hostAbortController = new AbortController();
        const hostAbortConnection = basePort(pendingActor, false, "queued");
        const hostAborted = await module.activate({ actorId: "pending-host-abort", activationGeneration: 1n }, hostAbortConnection.port, { signal: hostAbortController.signal });
        hostAbortConnection.bind(hostAborted);
        const hostAbortCall = hostFault(hostAborted.invoke([fixture.component, "linkRoundtrip"], [new Uint8Array(pendingActor.link.okRequest)]));
        await settle(() => hostAbortConnection.frames.length === 1);
        hostAbortController.abort();
        await hostAbortCall;
        await settle(() => hostAborted.progress().phase === "closed");
        assert.deepEqual(hostAborted.progress(), { phase: "closed", activeInvocations: 0 });
        const dispatchConnection = basePort(pendingActor);
        dispatchConnection.port.dispatch = () => { throw new Error("fixture dispatch rejected"); };
        const dispatchActor = await module.activate({ actorId: "dispatch-failure", activationGeneration: 1n }, dispatchConnection.port);
        dispatchConnection.bind(dispatchActor);
        await hostFault(dispatchActor.invoke([fixture.component, "linkRoundtrip"], [new Uint8Array(pendingActor.link.okRequest)]), "browser.host.request-failed", "fixture dispatch rejected");
        await dispatchActor.close();
        const malformedConnection = basePort(pendingActor, false);
        const malformedActor = await module.activate({ actorId: "malformed-response", activationGeneration: 1n }, malformedConnection.port);
        malformedConnection.bind(malformedActor);
        const malformedCall = hostFault(malformedActor.invoke([fixture.component, "linkRoundtrip"], [new Uint8Array(pendingActor.link.okRequest)]), "browser.host.request-failed", "browser host: invalid result");
        await settle(() => malformedConnection.frames.length === 1);
        const malformedRequest = malformedConnection.frames[0].frame.envelope.payload.payload;
        assert.equal(malformedActor.resolveEffect(malformedRequest.requestId, { tag: "ok" }), true);
        await malformedCall;
        await malformedActor.close();
        const streamConnection = basePort(pendingActor, false);
        const streamActor = await module.activate({ actorId: "pending-stream-close", activationGeneration: 1n }, streamConnection.port);
        streamConnection.bind(streamActor);
        const streamSettled = Promise.allSettled([streamActor.invoke([fixture.component, "blobCollect"], [pendingActor.blob.okHash])]);
        await settle(() => streamConnection.frames.length === 1);
        let streamReads = 0, streamCancellations = 0;
        const streamBody = new ReadableStream({ pull() { streamReads++; }, cancel() { streamCancellations++; } }, { highWaterMark: 0 });
        const streamRequest = streamConnection.frames[0].frame.envelope.payload.payload;
        assert.equal(streamActor.resolveEffect(streamRequest.requestId, { tag: "ok", val: streamBody }), true);
        await settle(() => streamReads === 1);
        await streamActor.close();
        const [streamResult] = await streamSettled;
        assert.deepEqual({ ...streamActor.progress(), cancellations: streamCancellations, result: streamResult.status, locked: streamBody.locked }, fixture.streamClose);
        const dropConnection = basePort(pendingActor, false);
        const dropActor = await module.activate({ actorId: "guest-stream-drop", activationGeneration: 1n }, dropConnection.port);
        dropConnection.bind(dropActor);
        const dropSettled = Promise.allSettled([dropActor.invoke([fixture.component, "blobDrop"], [pendingActor.blob.okHash])]);
        await settle(() => dropConnection.frames.length === 1);
        let dropCancellations = 0;
        const dropBody = new ReadableStream({ cancel() { dropCancellations++; } }, { highWaterMark: 0 });
        const dropRequest = dropConnection.frames[0].frame.envelope.payload.payload;
        assert.equal(dropActor.resolveEffect(dropRequest.requestId, { tag: "ok", val: dropBody }), true);
        const [dropResult] = await dropSettled;
        assert.equal(dropResult.status, "fulfilled");
        if (dropResult.status === "fulfilled") assert.equal(dropResult.value, undefined);
        await settle(() => dropCancellations === 1 && !dropBody.locked && dropActor.progress().activeInvocations === 0);
        assert.deepEqual(dropActor.progress(), { phase: "open", activeInvocations: 0 });
        await dropActor.close();
        assert.deepEqual({ ...dropActor.progress(), cancellations: dropCancellations, result: dropResult.status, locked: dropBody.locked }, fixture.guestStreamDrop);
        const lateConnection = basePort(pendingActor, false);
        const lateActor = await module.activate({ actorId: "late-stream-failure", activationGeneration: 1n }, lateConnection.port);
        lateConnection.bind(lateActor);
        const lateCall = hostFault(lateActor.invoke([fixture.component, "blobCollect"], [pendingActor.blob.okHash]), fixture.lateStreamFault.code, fixture.lateStreamFault.message);
        await settle(() => lateConnection.frames.length === 1);
        let lateController, lateReads = 0;
        const lateBody = new ReadableStream({ start(controller) { lateController = controller; }, pull() { lateReads++; } }, { highWaterMark: 0 });
        const lateRequest = lateConnection.frames[0].frame.envelope.payload.payload;
        assert.equal(lateActor.resolveEffect(lateRequest.requestId, { tag: "ok", val: lateBody }), true);
        await settle(() => lateReads === 1);
        lateController.error(new Error(fixture.lateStreamFault.message));
        await lateCall;
        await lateActor.close();
        assert.equal(lateBody.locked, false);
        assert.deepEqual(lateActor.progress(), { phase: "closed", activeInvocations: 0 });
        const retirementConnection = basePort(pendingActor, false);
        const retirementActor = await module.activate({ actorId: "failed-stream-retirement", activationGeneration: 1n }, retirementConnection.port);
        retirementConnection.bind(retirementActor);
        const retirementCall = hostFault(retirementActor.invoke([fixture.component, "blobCollect"], [pendingActor.blob.okHash]), fixture.failedStreamRetirement.code, fixture.failedStreamRetirement.message);
        await settle(() => retirementConnection.frames.length === 1);
        let retirementCancels = 0;
        const retirementBody = new ReadableStream({ pull(controller) { controller.enqueue("invalid-byte-chunk"); }, cancel() { retirementCancels++; throw new Error(fixture.failedStreamRetirement.cancelMessage); } }, { highWaterMark: 0 });
        const retirementRequest = retirementConnection.frames[0].frame.envelope.payload.payload;
        assert.equal(retirementActor.resolveEffect(retirementRequest.requestId, { tag: "ok", val: retirementBody }), true);
        await retirementCall;
        await assert.rejects(retirementActor.close(), /retirement failed/);
        assert.equal(retirementCancels, 1);
        assert.equal(retirementBody.locked, false);
        assert.deepEqual(retirementActor.progress(), { phase: "closed", activeInvocations: 0 });
        hostCloseLaws = 9;
      }
      const abortController = new AbortController();
      const abortConnection = basePort(pendingActor);
      const aborted = await module.activate({ actorId: "pending-abort", activationGeneration: 1n }, abortConnection.port, { signal: abortController.signal });
      abortConnection.bind(aborted);
      const abortCall = assert.rejects(aborted.invoke([fixture.component, "timerRoundtrip"], [32]), /browser wasi: closed/);
      await settle(() => aborted.progress().activeInvocations === 1);
      abortController.abort();
      await abortCall;
      await settle(() => aborted.progress().phase === "closed");
      assert.deepEqual(aborted.progress(), { phase: "closed", activeInvocations: 0 });
      await assert.rejects(module.activate({ actorId: "missing-wasi", activationGeneration: 1n }, { ...abortConnection.port, wasi: undefined }), /WASI port required/);
      console.log(JSON.stringify({ observations, pending: pending.progress(), aborted: aborted.progress(), laws: 15, hostCloseLaws }));
    `, actorInputPath], {
      cwd: repoRoot,
      env: process.env,
      budgetMs: fixture.limits.runtimeBudgetMs,
      maxOutputBytes: fixture.limits.maximumOutputBytes,
      stdoutPath: join(evidence, "closed-runtime.stdout.json"),
      stderrPath: join(evidence, "closed-runtime.stderr"),
      cancelled: () => false,
    });
    assert.equal(closedRuntime.status, 0, `${closedRuntime.reason}: ${closedRuntime.stderr}`);
    const closedObservation = JSON.parse(closedRuntime.stdout) as { laws: number; hostCloseLaws: number; observations: unknown[] };
    assert.equal(closedObservation.observations.length, fixture.actors.length);
    assert.equal(closedObservation.hostCloseLaws, pendingHostClose ? 9 : 0);
    closedLaws = closedObservation.laws;
  }
  console.log(`browser-actor-import: AJV=1 JCO=1 Wasm=1 JSPI=${fixture.identity.jco.asyncImports.length} emitted=${fixture.identity.emittedAsyncBindings.length} actors=${fixture.actors.length} pack-ok=2 pack-err=2 stream-ok=2 stream-err=2 closed-laws=${closedLaws} host-close-laws=${pendingHostClose ? 9 : 0} guest-stream-drop=1 interface=${fixture.identity.canonicalInterface} factory=${actorModulePath ? "raw+closed" : observation.mode} unsupported-wasi=${closeActorBundle ? fixture.identity.unsupportedImport : "unchecked"} evidence=${evidence}`);
}

if (import.meta.main) {
  const root = repositoryRoot(import.meta.dir);
  const bundle = await import(pathToFileURL(join(import.meta.dir, "..", "..", "📜️script.ts")).href) as { closedBrowserActorBundle: ActorImportFactoryPort };
  const command = process.argv[2] ?? "runtime-check";
  assert(["runtime-check", "pending-host-close-check"].includes(command), `actor import fixture: unknown command ${command}`);
  await testCanonicalActorAsyncImport(root, undefined, bundle.closedBrowserActorBundle, true);
}
