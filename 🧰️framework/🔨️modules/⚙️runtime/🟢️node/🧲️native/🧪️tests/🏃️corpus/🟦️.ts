import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { isMainModuleV1 } from "../../../../🏁️main/🟦️.ts";
import { runtimeNativeSystemV1 } from "../../../../🧲️native/🟦️.ts";
import { resolveRuntimeNativeLibcV1, runtimeNativeLibcV1 } from "../../../../🧲️native/📚️libc/🟦️.ts";
import type { RuntimeNativeSystemV1 } from "../../../../📋️contract/🟦️.ts";
import { loadNodeNativeCoreV1 } from "../../🏗️addon/🟦️.ts";
import { createNodeNativeSystemV1 } from "../../🧮️system/🟦️.ts";

type Json = null | boolean | number | string | readonly Json[] | { readonly [key: string]: Json };
type Corpus = { environment: Record<string, string>; calls: any[]; memory: any[]; view: any; refusals: any[]; host: string[] };

const darwin = process.platform === "darwin";
const errnoSymbol = darwin ? "__error" : "__errno_location";

function makeView(spec: { bytes: string; view?: string; offset?: number }): ArrayBufferView | ArrayBuffer {
  const copy = new Uint8Array(Buffer.from(spec.bytes, "hex")), offset = spec.offset ?? 0;
  switch (spec.view ?? "Buffer") {
    case "Uint8Array": return new Uint8Array(copy.buffer, offset);
    case "DataView": return new DataView(copy.buffer, offset);
    case "ArrayBuffer": return copy.buffer;
    default: return Buffer.from(copy.buffer, offset);
  }
}

function hex(view: ArrayBufferView | ArrayBuffer): string {
  return Buffer.from(ArrayBuffer.isView(view) ? new Uint8Array(view.buffer, view.byteOffset, view.byteLength) : new Uint8Array(view)).toString("hex");
}

function encode(value: unknown, ranges: readonly { id: string; start: number; length: number }[]): Json {
  if (value === undefined) return { void: true };
  if (typeof value === "bigint") return { bigint: value.toString() };
  if (typeof value === "string") return { string: value };
  if (typeof value === "number") {
    const range = ranges.find(candidate => value >= candidate.start && value < candidate.start + Math.max(candidate.length, 1));
    return range === undefined ? value : { pointer: range.id, offset: value - range.start };
  }
  return value as Json;
}

function runCall(system: RuntimeNativeSystemV1, libc: string, spec: any): Json {
  const buffers = new Map<string, ArrayBufferView>(), ranges: { id: string; start: number; length: number }[] = [];
  for (const row of spec.buffers ?? []) { const view = makeView(row) as Buffer; buffers.set(row.id, view); ranges.push({ id: row.id, start: system.ptr(view), length: view.byteLength }); }
  const values = (spec.args as [string, any][]).map(([, value]) => {
    if (value !== null && typeof value === "object" && !Array.isArray(value)) {
      if ("bigint" in value) return BigInt(value.bigint);
      if ("string" in value) return value.string;
      if ("ref" in value) return buffers.get(value.ref)!;
      if ("pointer" in value) { const view = buffers.get(value.pointer)!; return system.ptr(view) + (value.offset ?? 0); }
      if ("bytes" in value) return makeView(value);
    }
    return value;
  });
  const declared: Record<string, { args: any[]; returns: any }> = { [spec.symbol]: { args: (spec.args as [string, unknown][]).map(([type]) => type), returns: spec.returns } };
  if (spec.errno !== undefined) declared[errnoSymbol] = { args: [], returns: "ptr" };
  const library = system.dlopen(libc, declared as any);
  try {
    const result = encode((library.symbols as any)[spec.symbol](...values), ranges);
    const row: Record<string, Json> = { value: result };
    if (spec.errno !== undefined) row.errno = system.read.i32((library.symbols as any)[errnoSymbol]());
    if (spec.after !== undefined) row.after = Object.fromEntries(Object.keys(spec.after).map(id => [id, hex(buffers.get(id)!)]));
    return row;
  } finally { library.close(); }
}

function runHost(system: RuntimeNativeSystemV1, libc: string, scratch: string): Record<string, Json> {
  const out: Record<string, Json> = {};
  const call = (symbol: string, args: string[], returns: string, ...values: unknown[]): any => {
    const library = system.dlopen(libc, { [symbol]: { args, returns }, [errnoSymbol]: { args: [], returns: "ptr" } } as any);
    try {
      const value = (library.symbols as any)[symbol](...values);
      return { value, errno: Number(value) === -1 ? system.read.i32((library.symbols as any)[errnoSymbol]()) : 0 };
    } finally { library.close(); }
  };
  const swapRoot = join(scratch, "swap");
  mkdirSync(join(swapRoot, "a"), { recursive: true }); mkdirSync(join(swapRoot, "b"), { recursive: true });
  writeFileSync(join(swapRoot, "a", "x"), "A"); writeFileSync(join(swapRoot, "b", "y"), "B");
  const exchange = (first: string, second: string): { status: number; errno: number } => {
    const at = darwin ? -2 : -100, paths = [Buffer.from(`${first}\0`), Buffer.from(`${second}\0`)];
    if (darwin) { const row = call("renameatx_np", ["i32", "ptr", "i32", "ptr", "u32"], "i32", at, paths[0], at, paths[1], 2); return { status: Number(row.value), errno: row.errno }; }
    try {
      const row = call("renameat2", ["i32", "ptr", "i32", "ptr", "u32"], "i32", at, paths[0], at, paths[1], 2);
      return { status: Number(row.value), errno: row.errno };
    } catch {
      const row = call("syscall", ["i64", "i32", "ptr", "i32", "ptr", "u32"], "i64", process.arch === "arm64" ? 276 : 316, at, paths[0], at, paths[1], 2);
      return { status: Number(row.value), errno: row.errno };
    }
  };
  const swapped = exchange(join(swapRoot, "a"), join(swapRoot, "b"));
  const missing = exchange(join(swapRoot, "missing-one"), join(swapRoot, "missing-two"));
  out["rename-swap"] = { swapped: swapped.status, namesA: readdirSync(join(swapRoot, "a")), namesB: readdirSync(join(swapRoot, "b")), missing: { status: missing.status, errno: missing.errno } };

  const statBuffer = Buffer.alloc(4096), statSymbol = darwin && process.arch === "x64" ? "statfs$INODE64" : "statfs";
  const stat = call(statSymbol, ["cstring", "ptr"], "i32", scratch, statBuffer);
  const base = system.ptr(statBuffer);
  out.statfs = darwin
    ? { status: Number(stat.value), bsize: system.read.u32(base), blocks: Number(system.read.u64(base, 8)), bavailPositive: system.read.u64(base, 24) > 0n }
    : { status: Number(stat.value), bsize: Number(system.read.i64(base, 8)), blocks: Number(system.read.u64(base, 16)), bavailPositive: system.read.u64(base, 32) > 0n };

  const caseRoot = join(scratch, "case");
  mkdirSync(caseRoot, { recursive: true }); writeFileSync(join(caseRoot, "Probe-Case.TXT"), "x");
  const access = (path: string): number => Number(call("access", ["cstring", "i32"], "i32", path, 0).value);
  out["case-sensitivity"] = { exact: access(join(caseRoot, "Probe-Case.TXT")), folded: access(join(caseRoot, "probe-case.txt")) };

  out["process-tree"] = { pid: call("getpid", [], "i32").value, ppid: call("getppid", [], "i32").value, pgid: call("getpgid", ["i32"], "i32", 0).value, uid: call("getuid", [], "u32").value };
  return out;
}

/** 🧪️ Runs the whole shared corpus through one native system and returns plain JSON, so any runtime's result can be compared with any other and with an independent oracle. */
export function runNativeCorpusV1(system: RuntimeNativeSystemV1, libc: string, corpus: Corpus, scratch: string): Record<string, Json> {
  for (const [key, value] of Object.entries(corpus.environment)) process.env[key] = value;
  const calls: Record<string, Json> = {};
  for (const spec of corpus.calls) calls[spec.id] = runCall(system, libc, spec);
  const memory: Record<string, Json> = {};
  for (const spec of corpus.memory) {
    const buffer = Buffer.from(spec.bytes, "hex"), base = system.ptr(buffer);
    memory[spec.id] = (spec.reads as [string, number, unknown][]).map(([type, offset]) => encode((system.read as any)[type](base, offset), []));
  }
  const view = corpus.view, backing = new Uint8Array(Buffer.from(view.bytes, "hex")), address = system.ptr(backing) + view.offset;
  const before = hex(new Uint8Array(system.toArrayBuffer(address, 0, view.length)));
  const library = system.dlopen(libc, { memset: { args: ["ptr", "i32", "u64"], returns: "ptr" } });
  const window = system.toArrayBuffer(address, 0, view.length);
  try { (library.symbols as any).memset(address, view.fill, view.fillLength); } finally { library.close(); }
  const viewResult = { before, after: hex(new Uint8Array(window)) };
  const refusals: Record<string, Json> = {};
  for (const spec of corpus.refusals) {
    try { system.dlopen(spec.library === "libc" ? libc : spec.library, { [spec.symbol]: { args: spec.args, returns: spec.returns } } as any).close(); refusals[spec.id] = "opened"; } catch { refusals[spec.id] = "throws"; }
  }
  return { calls, memory, view: viewResult, refusals, host: runHost(system, libc, scratch) };
}

/** 🧭️ Builds the native system a corpus run uses: the host runtime's own binding, or the Node core projected in this process. */
export function corpusSystemV1(kind: string): { system: RuntimeNativeSystemV1; libc: string } {
  if (kind === "host") return { system: runtimeNativeSystemV1(), libc: runtimeNativeLibcV1() };
  const system = createNodeNativeSystemV1(loadNodeNativeCoreV1());
  return { system, libc: resolveRuntimeNativeLibcV1(process.platform, process.arch, name => { try { system.dlopen(name, { getpid: { args: [], returns: "i32" } }).close(); return true; } catch { return false; } }) };
}

if (isMainModuleV1(import.meta.url)) {
  const option = (name: string): string => process.argv.find(argument => argument.startsWith(`--${name}=`))!.slice(name.length + 3);
  const kind = option("system"), corpus = JSON.parse(readFileSync(option("fixture"), "utf8")) as Corpus, { system, libc } = corpusSystemV1(kind);
  const result = runNativeCorpusV1(system, libc, corpus, option("scratch"));
  process.stdout.write(JSON.stringify({ runtime: "bun" in process.versions ? "bun" : "node", system: kind, platform: process.platform, arch: process.arch, libc, ...result }));
}
