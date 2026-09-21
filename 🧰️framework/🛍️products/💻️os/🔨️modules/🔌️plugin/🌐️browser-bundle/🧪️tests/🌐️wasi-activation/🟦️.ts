import { fileURLToPath as testFileUrlToPath, pathToFileURL } from "node:url";
const testSourceDirectory = testFileUrlToPath(new URL("../../🧫️fixtures/🌐️wasi-activation/", import.meta.url));
/** 🧭️ Qualifies isolated browser WASI resources against neutral traces and Preview2. */
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { runExactCargoLawProcess } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🗣️ Exercises installed and vendored CLI streams against the neutral fragmented-line trace. */
export async function testPreview2GuestLogVendoring(repoRoot: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(join(testSourceDirectory, "🔣️.json"), "utf8"));
  const { ensurePreview2ShimVendorAt, patchPreview2ShimGuestLogClassification } = await import("../../🏗️materialization/🟦️.ts");
  const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactBase?.includes("🗑️generated"));
  mkdirSync(artifactBase, { recursive: true });
  const vendor = mkdtempSync(join(artifactBase, "preview2-log-vendor-"));
  const observe = async (path: string, isolated: boolean) => {
    const module = await import(pathToFileURL(path).href);
    const cli = isolated ? module.createCli() : module;
    const calls: { channel: string; text: string; level: string }[] = [];
    const original = { log: console.log, debug: console.debug, error: console.error };
    try {
      for (const level of ["log", "debug", "error"] as const) console[level] = (text: string) => calls.push({ channel: "stderr", text, level });
      const stream = cli.stderr.getStderr();
      for (const chunk of fixture.lineBuffer.chunks) {
        const bytes = new TextEncoder().encode(chunk);
        assert(stream.checkWrite() >= BigInt(bytes.byteLength));
        stream.write(bytes);
      }
    } finally { Object.assign(console, original); }
    assert.equal(typeof cli.stdin.getStdin, "function");
    assert.equal(typeof cli.stdout.getStdout, "function");
    assert.equal(typeof cli.exit.exit, "function");
    return calls;
  };
  for (const isolated of [false, true]) {
    const oracle = await observe(join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser/cli.js"), isolated);
    assert.deepEqual(oracle.map(({ text }) => text), fixture.lineBuffer.hostCalls.map(({ text }: { text: string }) => text));
  }
  ensurePreview2ShimVendorAt(vendor, repoRoot);
  const path = join(vendor, "cli.js"), first = readFileSync(path, "utf8");
  patchPreview2ShimGuestLogClassification(path);
  assert.equal(readFileSync(path, "utf8"), first);
  for (const isolated of [false, true]) assert.deepEqual(await observe(path, isolated), fixture.lineBuffer.hostCalls);
  const drift = join(vendor, "drift.js");
  writeFileSync(drift, "export const stderr = {};\n");
  assert.throws(() => patchPreview2ShimGuestLogClassification(drift), /patch did not match/);
  console.log("[DEBUG] Preview2 guest log vendoring: installed oracle, fragmented lines, severity, idempotence and drift refusal passed");
}

export async function testBrowserWasiActivation(repoRoot: string): Promise<void> {
  await testPreview2GuestLogVendoring(repoRoot);
  const fixture = JSON.parse(readFileSync(join(testSourceDirectory, "🔣️.json"), "utf8"));
  const schemaDocument = JSON.parse(readFileSync(resolve(testSourceDirectory, "../../🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schemaDocument);
  const validate = ajv.getSchema(`${schemaDocument.$id}#/$defs/WasiActivationV1`)!;
  assert(validate(fixture), JSON.stringify(validate.errors));
  const { browserWasiInterfaces, createBrowserWasiActivation, createGuestLogLineSink, classifyGuestLogLine } = await import("../../🌐️wasi/🟦️.ts");
  const { DOCUMENT_BROWSER_ACTOR_INTERFACES } = await import("../../../../📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts");
  const program = ts.createProgram([join(testSourceDirectory, "../../🌐️wasi/🟦️.ts")], { noEmit: true, strict: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, lib: ["lib.es2023.d.ts", "lib.esnext.disposable.d.ts", "lib.dom.d.ts"], types: [], skipLibCheck: true });
  const diagnostics = ts.getPreEmitDiagnostics(program);
  assert.equal(diagnostics.length, 0, ts.formatDiagnosticsWithColorAndContext(diagnostics, { getCanonicalFileName: path => path, getCurrentDirectory: () => testSourceDirectory, getNewLine: () => "\n" }));
  const writes: number[][] = [];
  const port = { nowNs: () => BigInt(Math.floor(performance.now() * 1e6)), wallNs: () => BigInt(Date.now()) * 1_000_000n, write(_stream: "stdout" | "stderr", bytes: Uint8Array) { writes.push([...bytes]); } };
  const a = createBrowserWasiActivation(port), b = createBrowserWasiActivation(port);
  const aImports = a.imports, bImports = b.imports;
  const key = <P extends string>(path: P): `wasi:${P}@0.2.0` => `wasi:${path}@0.2.0`;
  const environment = aImports[key("cli/environment")];
  assert.deepEqual({ environment: environment.getEnvironment(), arguments: environment.getArguments(), cwd: environment.initialCwd() ?? null, terminal: aImports[key("cli/terminal-stdin")].getTerminalStdin() ?? null }, fixture.emptyAmbient);
  assert.notEqual(aImports[key("io/streams")].OutputStream, bImports[key("io/streams")].OutputStream);
  const stdin = aImports[key("cli/stdin")].getStdin();
  assert.throws(() => stdin.read(1n), error => (error as { tag: string }).tag === "closed");
  const output = aImports[key("cli/stdout")].getStdout();
  assert.throws(() => output.write(new Uint8Array(fixture.output)), /permit/);
  assert.equal(output.checkWrite(), BigInt(fixture.limits.writeBytes));
  output.write(new Uint8Array(fixture.output));
  assert.deepEqual(writes, [fixture.output]);
  const admitted = [
    "wasi:cli/environment@0.2.0", "wasi:cli/exit@0.2.0", "wasi:cli/stderr@0.2.0", "wasi:cli/stdin@0.2.0", "wasi:cli/stdout@0.2.0",
    "wasi:cli/terminal-input@0.2.0", "wasi:cli/terminal-output@0.2.0", "wasi:cli/terminal-stderr@0.2.0", "wasi:cli/terminal-stdin@0.2.0", "wasi:cli/terminal-stdout@0.2.0",
    "wasi:clocks/monotonic-clock@0.2.0", "wasi:clocks/wall-clock@0.2.0", "wasi:io/error@0.2.0", "wasi:io/poll@0.2.0", "wasi:io/streams@0.2.0",
    "wasi:random/insecure-seed@0.2.9",
  ];
  assert.deepEqual([...browserWasiInterfaces].sort(), admitted);
  assert.deepEqual(Object.keys(aImports).sort(), admitted);
  assert.deepEqual([...DOCUMENT_BROWSER_ACTOR_INTERFACES].sort(), ["semio:framework/host-async@1.0.0", "semio:framework/pure@1.0.0", ...admitted]);
  const wallPort = { ...port, wallNs: () => 1_726_000_000_123_456_789n };
  const wall = createBrowserWasiActivation(wallPort);
  assert.deepEqual(wall.imports["wasi:clocks/wall-clock@0.2.0"].now(), { seconds: 1_726_000_000n, nanoseconds: 123_456_789 });
  assert.deepEqual(wall.imports["wasi:clocks/wall-clock@0.2.0"].resolution(), { seconds: 0n, nanoseconds: 1_000_000 });
  const seed = wall.imports["wasi:random/insecure-seed@0.2.9"].insecureSeed();
  assert.equal(seed.length, 2);
  assert(seed.every(word => typeof word === "bigint" && word >= 0n && word <= 0xffffffffffffffffn));
  assert.deepEqual(wall.imports["wasi:random/insecure-seed@0.2.9"].insecureSeed(), seed);
  const sibling = createBrowserWasiActivation(wallPort);
  assert.notDeepEqual(sibling.imports["wasi:random/insecure-seed@0.2.9"].insecureSeed(), seed);
  await sibling.close();
  await wall.close();
  assert.throws(() => wall.imports["wasi:clocks/wall-clock@0.2.0"].now(), /closed/);
  assert.throws(() => wall.imports["wasi:random/insecure-seed@0.2.9"].insecureSeed(), /closed/);
  const clock = aImports[key("clocks/monotonic-clock")], polling = aImports[key("io/poll")];
  const hostileDurations: readonly unknown[] = [-1n, 0x10000000000000000n, 1, NaN];
  for (const value of hostileDurations) assert.throws(() => clock.subscribeDuration(value), /u64/);
  for (const value of hostileDurations) {
    const hostile = createBrowserWasiActivation({ ...port, wallNs: () => value as bigint });
    assert.throws(() => hostile.imports["wasi:clocks/wall-clock@0.2.0"].now(), /u64/);
    await hostile.close();
  }
  const ready = clock.subscribeDuration(0n);
  assert.deepEqual([...(await polling.poll([ready]))], fixture.readyIndices);
  await assert.rejects(bImports[key("io/poll")].poll([ready]), /foreign/);
  await assert.rejects(polling.poll([]), /poll list/);
  const pending = clock.subscribeDuration(0xffffffffffffffffn);
  const waiters = Array.from({ length: fixture.limits.waiters }, () => polling.poll([pending]));
  const settling = Promise.allSettled(waiters);
  await assert.rejects(polling.poll([pending]), /waiter capacity/);
  const closing = a.close();
  assert.equal(a.close(), closing);
  await closing;
  assert.equal((await settling).filter(result => result.status === "rejected").length, fixture.limits.waiters);
  assert.deepEqual(a.progress(), fixture.terminal);
  assert.throws(() => output.checkWrite(), /closed/);
  await b.close();
  const capacity = createBrowserWasiActivation(port);
  for (let count = 0; count < fixture.limits.resources; count++) capacity.imports[key("clocks/monotonic-clock")].subscribeDuration(0n);
  assert.throws(() => capacity.imports[key("cli/stdout")].getStdout(), /resource capacity/);
  await capacity.close();
  const fault = createBrowserWasiActivation({ ...port, write() { throw new Error("sink-fault"); } });
  const faultOutput = fault.imports[key("cli/stdout")].getStdout();
  while (fault.progress().resources < fixture.limits.resources) fault.imports[key("clocks/monotonic-clock")].subscribeDuration(0n);
  faultOutput.checkWrite();
  assert.throws(() => faultOutput.write(new Uint8Array([1])), error => {
    const failure = error as { tag: string; val: { toDebugString(): string } };
    assert.equal(failure.tag, "last-operation-failed");
    assert.match(failure.val.toDebugString(), /sink-fault/);
    return true;
  });
  await fault.close();
  assert.deepEqual(fault.progress(), fixture.terminal);
  let closingSink: ReturnType<typeof createBrowserWasiActivation>;
  closingSink = createBrowserWasiActivation({ ...port, write() { void closingSink.close(); throw new Error("closed-sink"); } });
  const closingOutput = closingSink.imports[key("cli/stdout")].getStdout();
  closingOutput.checkWrite();
  assert.throws(() => closingOutput.write(new Uint8Array([1])), error => (error as { tag: string }).tag === "closed");
  assert.deepEqual(closingSink.progress(), fixture.terminal);
  let closingClock: ReturnType<typeof createBrowserWasiActivation>, clockReads = 0;
  closingClock = createBrowserWasiActivation({ ...port, nowNs() { if (++clockReads === 2) void closingClock.close(); return 0n; } });
  assert.throws(() => closingClock.imports[key("clocks/monotonic-clock")].subscribeDuration(1n), /closed/);
  assert.deepEqual(closingClock.progress(), fixture.terminal);
  const quota = createBrowserWasiActivation({ ...port, write() {} });
  const limited = quota.imports[key("cli/stdout")].getStdout();
  for (let offset = 0; offset < fixture.limits.outputBytes; offset += fixture.limits.writeBytes) {
    assert.equal(limited.checkWrite(), BigInt(fixture.limits.writeBytes));
    limited.write(new Uint8Array(fixture.limits.writeBytes));
  }
  assert.throws(() => limited.checkWrite(), error => (error as { tag: string }).tag === "closed");
  await quota.close();
  const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactBase !== undefined && artifactBase.includes("🗑️generated"));
  mkdirSync(artifactBase, { recursive: true });
  const evidence = mkdtempSync(join(artifactBase, "browser-wasi-activation-"));
  const oracle = await runExactCargoLawProcess("node", ["--input-type=module", "-e", `
    import assert from "node:assert/strict";
    import { readFileSync } from "node:fs";
    import { pathToFileURL } from "node:url";
    const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
    const io = await import(pathToFileURL(process.argv[2] + "/node_modules/@bytecodealliance/preview2-shim/dist/browser/io.js"));
    const writes = [];
    const output = io.outputStreamCreate({ write(bytes) { writes.push([...bytes]); } });
    assert(output.checkWrite() >= BigInt(fixture.output.length));
    output.write(new Uint8Array(fixture.output));
    const ready = await io.poll.poll([io.pollableCreate()]);
    assert.deepEqual(writes, [fixture.output]);
    assert.deepEqual([...ready], fixture.readyIndices);
    console.log(JSON.stringify({ writes, ready: [...ready], preview2: 1 }));
  `, join(testSourceDirectory, "🔣️.json"), repoRoot], { cwd: repoRoot, env: process.env, budgetMs: 60_000, maxOutputBytes: 64 * 1024, stdoutPath: join(evidence, "oracle.stdout.json"), stderrPath: join(evidence, "oracle.stderr"), cancelled: () => false });
  assert.equal(oracle.status, 0, oracle.stderr);
  const hostCalls: Parameters<Parameters<typeof createGuestLogLineSink>[0]>[0][] = [];
  const sink = createGuestLogLineSink((line) => hostCalls.push(line));
  for (const chunk of fixture.lineBuffer.chunks) sink.write("stderr", new TextEncoder().encode(chunk));
  assert.deepEqual(hostCalls, fixture.lineBuffer.hostCalls);
  assert.equal(hostCalls.length, 2);
  assert.equal(sink.pendingBytes("stderr"), 0);
  assert.equal(classifyGuestLogLine("stderr", hostCalls[0].text), "debug");
  assert.equal(classifyGuestLogLine("stderr", hostCalls[1].text), "error");
  const preview2Writes = [];
  const io = await import(pathToFileURL(join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser/io.js")).href);
  const preview2 = io.outputStreamCreate({ write(bytes: Uint8Array) { preview2Writes.push(new TextDecoder().decode(bytes)); } });
  for (const chunk of fixture.lineBuffer.chunks) {
    const bytes = new TextEncoder().encode(chunk);
    assert(preview2.checkWrite() >= BigInt(bytes.byteLength));
    preview2.write(bytes);
  }
  assert.equal(preview2Writes.length, fixture.lineBuffer.preview2HostCalls);

  console.log(`browser-wasi-activation: AJV=1 TypeScript=1 Preview2=1 actors=2 laws=${fixture.laws.length} resources=${fixture.limits.resources} waiters=${fixture.limits.waiters} evidence=${evidence}`);
}
