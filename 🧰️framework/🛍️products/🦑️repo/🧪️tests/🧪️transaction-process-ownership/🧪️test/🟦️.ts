import { afterAll, expect, test } from "bun:test";
import { EventEmitter } from "node:events";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { parse, type ParseError } from "jsonc-parser";
import isEqual from "lodash/isEqual";
import { SmartBuffer } from "smart-buffer";
import ts from "typescript";

const owner = resolve(import.meta.dir, ".."), path = join(owner, "🟦️.ts"), vectorText = readFileSync(join(owner, "🔣️.json"), "utf8"), vector = JSON.parse(vectorText), schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
const require = createRequire(import.meta.url), compilers = [{ id: "bun", compile: (code: string) => new Bun.Transpiler({ loader: "ts", target: "node" }).transformSync(code) }, { id: "typescript", compile: (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText }];
const clone = <T>(value: T): T => structuredClone(value);
const inputs = [path, join(owner, "🔣️.json"), join(owner, "🧬️schema/🔣️.json"), import.meta.filename].map((file) => ({ path: file, bytes: existsSync(file) ? readFileSync(file) : null }));

afterAll(() => { for (const input of inputs) expect(existsSync(input.path) ? readFileSync(input.path) : null).toEqual(input.bytes); console.info("[DEBUG] Process observer inert endpoints " + JSON.stringify({ pid: process.pid, inputs: inputs.map(({ path, bytes }) => ({ path, bytes: bytes?.length ?? null, sha256: bytes ? createHash("sha256").update(bytes).digest("hex") : null })) })); });

function compiled(compiler: typeof compilers[number], inertNativeApi?: unknown) {
  expect(existsSync(path), "Approved private observer must exist").toBe(true);
  const text = readFileSync(path, "utf8"), source = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), exports: string[] = [];
  const replacements: { start: number; end: number; text: string }[] = [];
  for (const node of source.statements) {
    if (ts.isImportDeclaration(node)) { const clause = node.importClause; expect(clause?.namedBindings && ts.isNamedImports(clause.namedBindings)).toBe(true); replacements.push({ start: node.getStart(source), end: node.end, text: `const { ${((clause!.namedBindings as ts.NamedImports).elements).filter((item) => !item.isTypeOnly).map((item) => item.propertyName ? `${item.propertyName.text}: ${item.name.text}` : item.name.text).join(", ")} } = require(${(node.moduleSpecifier as ts.StringLiteral).getText(source)});` }); }
    if (ts.canHaveModifiers(node)) for (const modifier of ts.getModifiers(node) ?? []) if (modifier.kind === ts.SyntaxKind.ExportKeyword) { replacements.push({ start: modifier.getStart(source), end: modifier.end, text: "" }); if (ts.isFunctionDeclaration(node) && node.name) exports.push(node.name.text); }
  }
  let code = text; for (const row of replacements.sort((a, b) => b.start - a.start)) code = code.slice(0, row.start) + row.text + code.slice(row.end);
  if (inertNativeApi) code += "\nprocessNativeApi = async () => inertNativeApi;";
  code += `\nreturn { ${exports.join(", ")}, createTransactionProcessObserver };`;
  return new Function("require", "process", "inertNativeApi", compiler.compile(code))((name: string) => name === "node:child_process" ? { ChildProcess: FakeChild } : require(name), { pid: 100, ppid: 1, platform: "darwin", arch: "arm64" }, inertNativeApi);
}

class FakeChild extends EventEmitter {
  pid: number | undefined; exitCode: number | null = null; signalCode: string | null = null;
  constructor(pid: number | undefined = 4101) { super(); this.pid = pid; }
  exit(): void { this.exitCode = 0; this.emit("exit", 0, null); }
  close(): void { this.emit("close", this.exitCode, this.signalCode); }
}

function changed(row: any): any {
  if (row.change === "absent") return null;
  const value = clone(vector.observations[row.base]);
  if (row.change === "birth") { const key = row.base === "darwin" ? "seconds" : row.base === "linux" ? "startTicks" : "filetime"; value.birth[key] = String(BigInt(value.birth[key]) + 1n); }
  if (row.change === "parent") value.parentPid++;
  if (row.change === "group") value.groupId++;
  if (row.change === "session") value.sessionId++;
  if (row.change === "pid") value.pid++;
  if (row.change === "zombie") value.state = "zombie";
  if (row.change === "extra") value.birth.extra = true;
  if (row.change === "leading-zero") value.birth.startTicks = "0" + value.birth.startTicks;
  return value;
}

function darwinBytes(): Buffer {
  const bytes = Buffer.alloc(136), value = vector.observations.darwin;
  bytes.writeUInt32LE(2, 4); bytes.writeUInt32LE(value.pid, 12); bytes.writeUInt32LE(value.parentPid, 16); bytes.writeUInt32LE(value.groupId, 100); bytes.writeBigUInt64LE(BigInt(value.birth.seconds), 120); bytes.writeBigUInt64LE(BigInt(value.birth.microseconds), 128);
  return bytes;
}

test("process observation has closed neutral schema and independent JSON authority", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema); expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const value of [{ ...vector, signals: true }, { ...vector, nativeProbe: { ...vector.nativeProbe, signals: true } }, { ...vector, darwinLayout: { ...vector.darwinLayout, size: 128 } }, { ...vector, scope: "pid-only" }]) expect(validate(value)).toBe(false);
  const errors: ParseError[] = []; expect(parse(vectorText, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector); expect(errors).toEqual([]);
  expect(new Set(vector.decisionCases.map((row: any) => row.id)).size).toBe(vector.decisionCases.length);
});

test("actual process decoder preserves 64-bit birth tokens through two compilers and SmartBuffer", () => {
  for (const compiler of compilers) {
    const api = compiled(compiler), bytes = darwinBytes(), layout = api.certifyTransactionProcessLayout(vector.darwinLayout, clone(vector.darwinLayout), "darwin", "arm64");
    const smart = SmartBuffer.fromBuffer(bytes), value = vector.observations.darwin;
    expect({ pid: smart.readUInt32LE(12), parentPid: smart.readUInt32LE(16), groupId: smart.readUInt32LE(100), sessionId: null, state: "live", birth: { platform: "darwin", seconds: smart.readBigUInt64LE(120).toString(), microseconds: smart.readBigUInt64LE(128).toString() } }).toEqual(value);
    expect(api.decodeDarwinTransactionObservation(bytes, 136, layout)).toEqual(value);
    expect(api.parseLinuxTransactionObservation(vector.linuxStat, vector.observations.linux.birth.bootId)).toEqual(vector.observations.linux);
    const times = Buffer.alloc(8); times.writeUInt32LE(vector.windowsFiletime.low, 0); times.writeUInt32LE(vector.windowsFiletime.high, 4);
    expect(api.decodeWindowsTransactionFiletime(times)).toBe(SmartBuffer.fromBuffer(times).readBigUInt64LE().toString());
    expect(api.decodeWindowsTransactionFiletime(times)).toBe(vector.windowsFiletime.value);
  }
});

test("layout agreement rejects 128, disagreement, wrong host and unsupported or short native records", () => {
  for (const compiler of compilers) {
    const api = compiled(compiler), good = vector.darwinLayout, layout = api.certifyTransactionProcessLayout(good, clone(good), "darwin", "arm64");
    for (const id of vector.darwinFailures) {
      const bytes = darwinBytes();
      const check = () => {
        if (id === "layout-128") return api.certifyTransactionProcessLayout({ ...good, size: 128 }, { ...good, size: 128 }, "darwin", "arm64");
        if (id === "different-oracles") return api.certifyTransactionProcessLayout(good, { ...good, offsets: { ...good.offsets, pid: 16 } }, "darwin", "arm64");
        if (id === "wrong-architecture") return api.certifyTransactionProcessLayout(good, clone(good), "darwin", "x64");
        if (id === "invalid-pid") bytes.writeUInt32LE(0, 12);
        if (id === "invalid-microseconds") bytes.writeBigUInt64LE(1000000n, 128);
        if (id === "unsupported-status") bytes.writeUInt32LE(999, 4);
        return api.decodeDarwinTransactionObservation(id === "short-buffer" ? bytes.subarray(0, 128) : bytes, id === "short-return" ? 128 : id === "long-return" ? 137 : 136, layout);
      };
      expect(check, `${compiler.id}:${id}`).toThrow();
    }
    for (const id of vector.linuxFailures) {
      const text = id === "missing-close" ? "4102 (missing" : id === "short-fields" ? "4102 (x) S 1" : id === "invalid-state" ? vector.linuxStat.replace(") S ", ") ? ") : id === "scientific-pid" ? vector.linuxStat.replace("4102 ", "4.102e3 ") : id === "negative-start" ? vector.linuxStat.replace("9007199254740993", "-1") : vector.linuxStat;
      expect(() => api.parseLinuxTransactionObservation(text, id === "invalid-boot" ? "unknown" : vector.observations.linux.birth.bootId), id).toThrow();
    }
    expect(() => api.decodeWindowsTransactionFiletime(Buffer.alloc(7))).toThrow();
    expect(() => api.certifyTransactionProcessLayout(good, clone(good), "freebsd", "arm64")).toThrow();
    expect(api.certifyTransactionProcessLayout(vector.windowsLayout, clone(vector.windowsLayout), "win32", "x64")).toEqual(vector.windowsLayout);
  }
});

test("pure decisions match independent Ajv and Lodash identity oracle without signal authority", () => {
  const validate = new Ajv({ strict: true }).compile({ ...schema, $ref: "#/definitions/observation", properties: undefined, required: undefined, additionalProperties: undefined });
  for (const compiler of compilers) {
    const api = compiled(compiler);
    for (const row of vector.decisionCases) {
      const expected = vector.observations[row.base], current = changed(row);
      const oracle = row.exited ? "terminal" : !current || !validate(current) || current.state !== "live" || !isEqual(current, expected) ? "reject" : row.base === "win32" ? "owned-handle" : current.groupId === current.pid ? "owned-leader" : "owned-process";
      expect(oracle, row.id).toBe(row.expected); expect(api.transactionProcessDecision(expected, current, row.exited).kind, `${compiler.id}:${row.id}`).toBe(oracle);
    }
  }
});

test("private observer retains pending identity and actual handle lifecycle without a public reader override", () => {
  for (const compiler of compilers) {
    const api = compiled(compiler), seen: string[] = [], self = { ...clone(vector.observations.darwin), pid: 100, parentPid: 1, groupId: 100 };
    const run = (id: string, exercise: (observer: any, child: FakeChild, state: { reads: number; hook?: () => void; fail: boolean; foreign: boolean; closed: boolean }) => void) => {
      const child = new FakeChild(), state = { reads: 0, fail: false, foreign: false, closed: false } as { reads: number; hook?: () => void; fail: boolean; foreign: boolean; closed: boolean };
      const reader = { self, read(pid: number) { state.reads++; expect(pid).toBe(child.pid); state.hook?.(); if (state.fail) throw new Error("inert read failure"); return { ...clone(vector.observations.darwin), parentPid: state.foreign ? 999 : 100 }; }, close() { state.closed = true; } };
      const observer = api.createTransactionProcessObserver("00000000-0000-4000-8000-000000000001", reader);
      exercise(observer, child, state); if (!child.exitCode && child.exitCode !== 0) child.exit(); child.close(); observer.close(); expect(state.closed).toBe(true); seen.push(id);
    };
    run("pending-before-read", (observer, child, state) => { state.hook = () => expect(observer.pending()).toHaveLength(1); const token = observer.observeFreshChild(child, "phase-child"); expect(observer.recheck(token).decision.kind).toBe("owned-leader"); });
    run("listeners-before-read", (observer, child, state) => { state.hook = () => { for (const name of ["error", "exit", "close"]) expect(child.listenerCount(name)).toBe(1); }; observer.observeFreshChild(child, "mixed-generator-child"); });
    run("stop-during-read", (observer, child, state) => { state.hook = () => observer.beginStop(); const token = observer.observeFreshChild(child, "failure-child"); expect(observer.pending()).toEqual([token]); expect(() => observer.observeFreshChild(new FakeChild(4102), "failure-child")).toThrow(); });
    run("exit-during-read", (observer, child, state) => { state.hook = () => child.exit(); const token = observer.observeFreshChild(child, "lease-contender"); const count = state.reads; expect(observer.recheck(token).decision.kind).toBe("terminal"); expect(state.reads).toBe(count); });
    run("exit-during-recheck", (observer, child, state) => { const token = observer.observeFreshChild(child, "phase-child"); state.hook = () => child.exit(); expect(observer.recheck(token).decision.kind).toBe("terminal"); });
    run("read-failure-retained", (observer, child, state) => { state.fail = true; const token = observer.observeFreshChild(child, "shard"); expect(observer.pending()).toEqual([token]); expect(observer.recheck(token).decision.kind).toBe("reject"); });
    run("foreign-parent-retained", (observer, child, state) => { state.foreign = true; const token = observer.observeFreshChild(child, "probe-subject"); expect(observer.recheck(token).decision.kind).toBe("reject"); expect(observer.pending()).toEqual([token]); });
    run("closed-handle-never-read", (observer, child, state) => { child.exit(); child.close(); const token = observer.observeFreshChild(child, "probe-decoy"); expect(observer.recheck(token).decision.kind).toBe("terminal"); expect(state.reads).toBe(0); });
    run("foreign-observer-token", (observer, child) => { observer.observeFreshChild(child, "phase-child"); expect(() => observer.recheck(Object.freeze({ id: "forged" }))).toThrow(); });
    run("duplicate-handle", (observer, child) => { observer.observeFreshChild(child, "phase-child"); expect(() => observer.observeFreshChild(child, "phase-child")).toThrow(); });
    run("arbitrary-pid-rejected", (observer) => { expect(() => observer.observeFreshChild({ pid: 4101 }, "phase-child")).toThrow(); });
    run("stop-before-admission", (observer, child) => { observer.beginStop(); expect(() => observer.observeFreshChild(child, "phase-child")).toThrow(); });
    run("close-with-live-child-rejected", (observer, child) => { observer.observeFreshChild(child, "phase-child"); expect(() => observer.close()).toThrow(); });
    run("failed-spawn-close", (observer, child) => { child.pid = undefined; const token = observer.observeFreshChild(child, "failure-child"); child.emit("error", new Error("spawn failed")); child.close(); expect(observer.recheck(token).decision.kind).toBe("terminal"); });
    run("close-before-exit-not-terminal", (observer, child) => { const token = observer.observeFreshChild(child, "phase-child"); child.close(); expect(observer.recheck(token).decision.kind).not.toBe("terminal"); expect(() => observer.close()).toThrow(); });
    expect(seen).toEqual(vector.orderingCases);
  }
});

test("actual module imports no third-party runtime or signal/command implementation", () => {
  expect(existsSync(path)).toBe(true); const text = readFileSync(path, "utf8"), source = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const imports = source.statements.filter(ts.isImportDeclaration).map((row) => (row.moduleSpecifier as ts.StringLiteral).text); expect(imports.every((name) => name.startsWith("node:"))).toBe(true);
  expect(text).not.toMatch(/\b(?:spawnSync|execSync|execFile|taskkill|TerminateProcess|process\.kill)\b/u);
  expect(text).not.toContain("export function createTransactionProcessObserver");
  console.info(`[DEBUG] Private process observation inert input ${createHash("sha256").update(text).digest("hex")}; no native calls or subjects`);
});

test("complete private helper passes actual strict TypeScript declaration checking", () => {
  const options: ts.CompilerOptions = { noEmit: true, strict: true, noUncheckedIndexedAccess: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, skipLibCheck: true, types: ["node"] };
  expect(ts.getPreEmitDiagnostics(ts.createProgram([path], options)).map((row) => `${row.file?.fileName}:${row.start} ${ts.flattenDiagnosticMessageText(row.messageText, "\n")}`)).toEqual([]);
});

test("actual Darwin binding is inertly scoped once and gated before every native record read", async () => {
  for (const compiler of compilers) {
    const buffers = new Map<number, Uint8Array>(), calls: unknown[] = []; let pointer = 0, returned = 136, opens = 0, closes = 0;
    const native = { ptr(bytes: Uint8Array) { buffers.set(++pointer, bytes); return pointer; }, dlopen(library: string, symbols: unknown) {
      opens++; expect(library).toBe("/usr/lib/libSystem.B.dylib"); expect(symbols).toEqual({ proc_pidinfo: { args: ["i32", "i32", "u64", "ptr", "i32"], returns: "i32" } });
      return { close() { closes++; }, symbols: { proc_pidinfo(pid: number, flavor: number, argument: bigint, pointer: number, size: number) {
        calls.push({ pid, flavor, argument: argument.toString(), size }); expect(flavor).toBe(3); expect(argument).toBe(0n); expect(size).toBe(136);
        const bytes = buffers.get(pointer)!; bytes.set(darwinBytes()); const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); view.setUint32(12, pid, true); view.setUint32(16, pid === 100 ? 1 : 100, true); view.setUint32(100, pid, true); return returned;
      } } };
    } };
    const api = compiled(compiler, native), evidence = { c: vector.darwinLayout, rust: clone(vector.darwinLayout) };
    await expect(api.openTransactionProcessObserver("00000000-0000-4000-8000-000000000001")).rejects.toThrow(); expect(opens).toBe(0);
    await expect(api.openTransactionProcessObserver("00000000-0000-4000-8000-000000000001", { ...evidence, rust: { ...evidence.rust, size: 128 } })).rejects.toThrow(); expect(opens).toBe(0);
    const observer = await api.openTransactionProcessObserver("00000000-0000-4000-8000-000000000001", evidence), child = new FakeChild(), token = observer.observeFreshChild(child, "probe-subject");
    expect(observer.recheck(token).decision.kind).toBe("owned-leader"); expect(opens).toBe(1); expect(calls).toHaveLength(3);
    returned = 128; expect(observer.recheck(token).decision.kind).toBe("reject"); returned = 136;
    child.exit(); child.close(); const count = calls.length; expect(observer.recheck(token).decision.kind).toBe("terminal"); expect(calls).toHaveLength(count); observer.close(); expect(closes).toBe(1);
    returned = 128; await expect(api.openTransactionProcessObserver("00000000-0000-4000-8000-000000000002", evidence)).rejects.toThrow(); expect(opens).toBe(2); expect(closes).toBe(2);
  }
});
