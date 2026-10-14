import { test, expect } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { mkdir, mkdtemp, writeFile, rm } from "node:fs/promises";
import { dirname, join, resolve, relative } from "node:path";
import { createRequire } from "node:module";
import { scriptCompilerAdmissionControlV1 } from "../../🎟️capacity/🟦️.ts";
import { spawnSync } from "node:child_process";
import ts from "typescript";
import Ajv from "ajv";
import cases from "./🧫️fixtures/🔣️.json";
import { readScriptPolicy, type ScriptInvocation } from "../../../🧭️routing/📥️invocation/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../.."), output = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!output) throw Error("Compiler tests require the original artifact directory");

test("general TypeScript receivers have no specific compiler import", () => {
  for (const entry of cases.entrypoints) {
    const path = resolve(root, entry), source = readFileSync(path, "utf8"), file = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
    const imports = file.statements.flatMap(node => ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) ? [node.moduleSpecifier.text] : []);
    const oracle = ts.preProcessFile(source, true, true).importedFiles.map(row => row.fileName);
    expect(imports).toEqual(oracle);
    for (const imported of imports.filter(value => value.startsWith("."))) {
      const target = resolve(dirname(path), imported);
      expect(target.includes("/🛍️products/"), entry + " -> " + imported).toBe(false);
      expect(target.includes("/✏️s/")).toBe(false);
      expect(target.includes("/🌎️hub/")).toBe(false);
    }
  }
});

test("neutral compiler state preserves every schema partition and rejects foreign owners", async () => {
  const { compilerStateDirectory } = await import("../../🗃️state/🟦️.ts");
  const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🗃️state/🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema), state = { root: output!, owner: "compiler-test", compiler: "typescript:1", configuration: "one", mode: "prototype" as const, platform: "test", architecture: "test" };
  expect(validate(state)).toBe(true);
  const directories = new Set([compilerStateDirectory(state)]);
  for (const change of cases.partitions) {
    const candidate = { ...state, ...change };
    expect(validate(candidate)).toBe(true);
    directories.add(compilerStateDirectory(candidate as typeof state));
  }
  expect(directories.size).toBe(cases.partitions.length + 1);
  for (const change of cases.invalidOwners) {
    const candidate = { ...state, ...change };
    expect(validate(candidate)).toBe(false);
    expect(() => compilerStateDirectory(candidate as typeof state)).toThrow();
  }
});

test("neutral compiler state serializes the same owner and preserves original cancellation", async () => {
  const { withCompilerState } = await import("../../🗃️state/🟦️.ts");
  const controller = new AbortController(), control = { signal: controller.signal, remainingMilliseconds: () => null, publish: async () => {}, yieldContinuation: async () => {} }, state = { root: output!, owner: "compiler-test", compiler: "typescript:1", configuration: "two", mode: "prototype" as const, control, leaseDirectory: join(output!, "leases"), files: ["probe"], capacity: false };
  let active = 0, maximum = 0;
  await Promise.all([0, 1].map(() => withCompilerState(state, async directory => {
    maximum = Math.max(maximum, ++active);
    await writeFile(join(directory, "probe"), "owned");
    await new Promise(accept => setTimeout(accept, 10));
    active--;
  })));
  expect(maximum).toBe(1);
  const reason = Error("original cancellation");
  controller.abort(reason);
  await expect(withCompilerState(state, async () => { throw Error("Cancelled compiler admitted"); })).rejects.toBe(reason);
});

test("neutral TypeScript preserves cold and warm diagnostics and independently refuses invalid source", async () => {
  const { typecheckProject } = await import("../🟦️.ts");
  await mkdir(output!, { recursive: true });
  const directory = await mkdtemp(join(output!, "a-"));
  await writeFile(join(directory, "package.json"), '{"type":"module"}');
  const project = join(directory, "tsconfig.json"), source = join(directory, "value.ts");
  await writeFile(project, JSON.stringify({ compilerOptions: { target: "ES2022", module: "ESNext", types: [], strict: true, skipLibCheck: true }, files: ["value.ts"] }));
  await writeFile(source, cases.source);
  const controller = new AbortController(), started = performance.now(), progress: string[] = [];
  const invocation: ScriptInvocation = { policy: readScriptPolicy({ version: 1, owner: "compiler-test", maximumElapsedMilliseconds: 30000 }), control: { signal: controller.signal, remainingMilliseconds: () => Math.max(0, 30000 - (performance.now() - started)), publish: async event => { progress.push(event.stage); await Promise.resolve(); }, yieldContinuation: () => new Promise(accept => setImmediate(accept)) }, capabilities: {} };
  const options = { workspace: root, stateRoot: output!, leaseDirectory: join(output!, "leases"), cwd: directory, project, owner: "compiler-test", mode: "prototype" as const, timeoutMs: 30000, invocation };
  try {
    const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧬️schema/🔣️.json"), "utf8")), validate = new Ajv({ strict: true }).compile(schema);
    const { invocation: original, ...input } = options;
    expect(validate(input)).toBe(true);
    for (const change of cases.invalidChecks) {
      const candidate = { ...input, ...change };
      expect(validate(candidate)).toBe(false);
      await expect(typecheckProject({ ...candidate, invocation: original } as typeof options)).rejects.toThrow();
    }
    await expect(typecheckProject({ ...options, execution: { signal: original.control.signal } } as typeof options)).rejects.toThrow("cannot replace original control");
    const oracle = ts.getParsedCommandLineOfConfigFile(project, {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: diagnostic => { throw Error(ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")); } });
    expect(oracle?.fileNames).toEqual([source]);
    const oracleProgram = ts.createProgram(oracle!.fileNames, oracle!.options);
    expect(ts.getPreEmitDiagnostics(oracleProgram)).toEqual([]);
    const diagnostics: string[][] = [];
    for (const label of ["cold", "warm"]) {
      const lines: string[] = [];
      await typecheckProject({ ...options, args: ["--extendedDiagnostics"], execution: { onLine: (line: string) => { lines.push(line); } } });
      diagnostics.push(lines);
      console.log("[DEBUG] Neutral TypeScript " + label + " " + lines.filter(line => /Check time|Total time/.test(line)).join("; "));
    }
    expect(diagnostics[0]!.some(line => /^Check time:/.test(line))).toBe(true);
    expect(diagnostics[1]!.some(line => /^Check time:/.test(line))).toBe(false);
    await writeFile(source, cases.invalidSource);
    expect(ts.getPreEmitDiagnostics(ts.createProgram({ rootNames: oracle!.fileNames, options: oracle!.options, oldProgram: oracleProgram })).some(diagnostic => diagnostic.code === 2322)).toBe(true);
    await expect(typecheckProject(options)).rejects.toThrow();
    expect(progress).toContain("running");
    expect(progress).toContain("complete");
    expect(invocation.control.signal).toBe(controller.signal);
    const require = createRequire(join(root, "package.json"));
    expect(require.resolve("typescript/bin/tsc").includes("typescript")).toBe(true);
    console.log("[DEBUG] Neutral TypeScript original input, warm state and independent TypeScript diagnostics verified");
  } finally { await rm(directory, { recursive: true, force: true }); }
});


test("finite original authority ends real SQLite compiler admission without aborting its signal", async () => {
  const { withCompilerState } = await import("../../🗃️state/🟦️.ts"), { acquireResourceLease } = await import("../../../🔒️leases/🟦️.ts"), { acquireCompilerCapacityLeaseV1 } = await import("../../🎟️capacity/🟦️.ts");
  const root = await mkdtemp(join(output!, "d-")), state = { root, owner: "finite-admission", compiler: "typescript:test", configuration: "deadline", mode: "prototype" as const };
  const { compilerStateDirectory } = await import("../../🗃️state/🟦️.ts");
  const directory = compilerStateDirectory(state);
  for (const stage of cases.admission.stages) {
    const controller = new AbortController(), originalSignal = controller.signal;
    let published = 0, yielded = 0, active = false;
    const leaseDirectory = join(output!, "leases"), resource = stage === "state" ? `compiler-state:${directory}` : stage === "host" ? "semio.compiler.host-capacity/v1" : "semio.compiler.cpu/v1/0";
    const held = stage === "core" ? await Promise.all(Array.from({ length: (await import("node:os")).availableParallelism() }, (_, index) => acquireResourceLease({ directory: leaseDirectory, resource: `semio.compiler.cpu/v1/${index}`, mode: "exclusive", signal: originalSignal }))) : [await acquireResourceLease({ directory: leaseDirectory, resource, mode: "exclusive", signal: originalSignal })];
    try {
      const { resourceLeasePathV1 } = await import("../../../🔒️leases/🟦️.ts"), path = resourceLeasePathV1(leaseDirectory, resource);
      const oracle = spawnSync("python3", ["-c", "import sqlite3,sys;d=sqlite3.connect(sys.argv[1],timeout=0)\ntry:d.execute('begin exclusive');raise Exception('unexpected admission')\nexcept sqlite3.OperationalError:print('busy')", path], { encoding: "utf8" });
      expect(oracle.status, oracle.stderr).toBe(0); expect(oracle.stdout.trim()).toBe("busy");
      const began = performance.now();
const control = { signal: originalSignal, remainingMilliseconds: () => Math.max(0, cases.admission.deadlineMs - (performance.now() - began)), publish: async () => { active = true; await Bun.sleep(cases.admission.publishDelayMs); published++; active = false; }, yieldContinuation: async () => { expect(active).toBe(false); await Bun.sleep(1); yielded++; } };
      const operation = stage === "core" ? acquireCompilerCapacityLeaseV1({ directory: leaseDirectory, control, environment: {}, requestedParallelism: 1 } as any).then(lease => { lease.release(); throw Error("Expired core admission entered"); }) : withCompilerState({ ...state, control, leaseDirectory, capacity: stage !== "state", environment: {}, files: [cases.admission.compilerLeaf] } as any, async () => { throw Error("Expired compiler admission entered"); });
      await expect(Promise.race([operation, Bun.sleep(cases.admission.deadlineMs + 400).then(() => { throw Error("Admission wait outlived original authority"); })])).rejects.toThrow(/deadline/i);
      expect(controller.signal.aborted).toBe(false);
      expect(control.signal).toBe(originalSignal);
      expect(published).toBeGreaterThan(0); expect(yielded).toBeGreaterThan(0); expect(active).toBe(false);
      console.log(`[DEBUG] Compiler admission ${stage} original finite authority refused with signal unchanged, awaited publish=${published}, yield=${yielded}`);
    } finally { held.forEach(lease => lease.release()); }
  }
}, 5000);

test("resolved compiler leaves and SQLite sidecars refuse oversized absolute and relative stores before creation", async () => {
  const { withCompilerState } = await import("../../🗃️state/🟦️.ts"), { acquireResourceLease } = await import("../../../🔒️leases/🟦️.ts");
  const short = await mkdtemp(join(output!, "p-"));
  const originalSignal = new AbortController().signal, control = { signal: originalSignal, remainingMilliseconds: () => 1000, publish: async () => {}, yieldContinuation: async () => {} };
  const absolute = join(short, "x".repeat(cases.admission.maximumPathUnits - short.length - 1 - 76));
  for (const root of [absolute, relative(process.cwd(), absolute)]) {
    expect(existsSync(absolute)).toBe(false);
    await expect(acquireResourceLease({ directory: root, resource: "oversized-sidecar", mode: "exclusive", signal: originalSignal })).rejects.toThrow(/path/i);
    expect(existsSync(absolute)).toBe(false);
  }
  const stateRoot = join(short, "s".repeat(cases.admission.maximumPathUnits - short.length - 1 - 58));
  for (const root of [stateRoot, relative(process.cwd(), stateRoot)]) {
    await expect(withCompilerState({ root, owner: "path-boundary", compiler: "typescript:test", configuration: "paths", mode: "prototype", control, leaseDirectory: join(short, "l"), files: [cases.admission.compilerLeaf], environment: { SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: "1" } } as any, async () => { throw Error("Oversized state admitted"); })).rejects.toThrow(/path/i);
    expect(existsSync(stateRoot)).toBe(false);
  }
  const accepted = join(short, "b".repeat(cases.admission.maximumPathUnits - short.length - 1 - 80)), { resourceLeasePathV1 } = await import("../../../🔒️leases/🟦️.ts"), leaf = resourceLeasePathV1(accepted, "boundary");
  const pathOracle = new Ajv({ strict: true }).compile({ type: "string", maxLength: cases.admission.maximumPathUnits });
  expect(pathOracle(leaf + "-journal")).toBe(true); expect((leaf + "-journal").length).toBe(cases.admission.maximumPathUnits);
  const valid = await acquireResourceLease({ directory: accepted, resource: "boundary", mode: "exclusive", signal: originalSignal });
  valid.release();
  console.log("[DEBUG] Resolved absolute/relative SQLite sidecars and inherited compiler leaf bounds reject before mkdir; exact 256 boundary admits with Ajv agreement");
});


test("actual TypeScript admission intersects requested time with un-aborted original authority and awaits expired publication", async () => {
  const { typecheckProject } = await import("../🟦️.ts"), { acquireResourceLease } = await import("../../../🔒️leases/🟦️.ts"), { acquireCompilerCapacityLeaseV1 } = await import("../../🎟️capacity/🟦️.ts");
  const directory = await mkdtemp(join(output!, "i-")), leaseDirectory = join(output!, "leases");
  await writeFile(join(directory, "package.json"), '{"type":"module"}');
  await writeFile(join(directory, "value.ts"), cases.source);
  await writeFile(join(directory, "tsconfig.json"), JSON.stringify({ compilerOptions: { types: [], strict: true, noEmit: true }, files: ["value.ts"] }));
  for (const row of cases.admission.intersections) {
    const signal = new AbortController().signal, held = await acquireResourceLease({ directory: leaseDirectory, resource: "semio.compiler.host-capacity/v1", mode: "exclusive", signal });
    const started = performance.now(), events: string[] = [];
    const invocation: ScriptInvocation = { policy: readScriptPolicy({ version: 1, owner: "typecheck-admission", maximumElapsedMilliseconds: row.originalMs }), control: { signal, remainingMilliseconds: () => Math.max(0, row.originalMs - (performance.now() - started)), publish: async progress => { await Bun.sleep(1); events.push(progress.stage); }, yieldContinuation: () => new Promise(done => setImmediate(done)) }, capabilities: {} };
    try {
      await expect(typecheckProject({ workspace: root, stateRoot: directory, leaseDirectory, cwd: directory, project: "tsconfig.json", owner: "typecheck-admission", mode: "prototype", timeoutMs: row.requestedMs, invocation })).rejects.toThrow(/deadline/i);
      expect(signal.aborted).toBe(false); expect(events.length).toBeGreaterThan(1);
      expect(performance.now() - started).toBeLessThan(Math.max(row.originalMs, row.requestedMs));
      console.log(`[DEBUG] Actual TypeScript admission original=${row.originalMs}ms requested=${row.requestedMs}ms exhausted intersection without aborting original signal`);
    } finally { held.release(); }
  }
  const signal = new AbortController().signal, held = await acquireResourceLease({ directory: leaseDirectory, resource: "semio.compiler.host-capacity/v1", mode: "exclusive", signal }), started = performance.now();
  let completed = false, yielded = false;
  const control = { signal, remainingMilliseconds: () => cases.admission.publicationExpiryMs - (performance.now() - started), publish: async () => { await Bun.sleep(cases.admission.publishDelayMs); completed = true; }, yieldContinuation: async () => { yielded = true; } };
  try {
    await expect(acquireCompilerCapacityLeaseV1({ directory: leaseDirectory, control, environment: {} })).rejects.toThrow(/deadline/i);
    expect(completed).toBe(true); expect(yielded).toBe(false); expect(signal.aborted).toBe(false);
    console.log("[DEBUG] Original asynchronous wait publisher completed before expiry refusal; continuation never ran after exhausted authority");
  } finally { held.release(); await rm(directory, { recursive: true, force: true }); }
}, 5000);
