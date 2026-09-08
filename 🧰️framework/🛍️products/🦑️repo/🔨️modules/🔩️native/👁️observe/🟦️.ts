import { ChildProcess } from "node:child_process";
import { randomUUID } from "node:crypto";
import { closeSync, constants, openSync, readSync } from "node:fs";

export type TransactionProcessBirth = Readonly<{ platform: "darwin"; seconds: string; microseconds: string } | { platform: "linux"; bootId: string; startTicks: string } | { platform: "win32"; filetime: string }>;
export type TransactionProcessObservation = Readonly<{ pid: number; parentPid: number; groupId: number | null; sessionId: number | null; state: "live" | "zombie"; birth: TransactionProcessBirth }>;
export type TransactionProcessLayout = Readonly<{ platform: "darwin" | "win32"; architecture: "arm64" | "x64"; size: number; alignment: number; offsets: Readonly<Record<string, number>> }>;
export type TransactionProcessDecision = Readonly<{ kind: "owned-leader" | "owned-process" | "owned-handle" | "terminal" | "reject"; reason: string }>;
export type TransactionProcessRole = "phase-child" | "mixed-generator-child" | "failure-child" | "lease-contender" | "shard" | "probe-subject" | "probe-decoy";
export type TransactionProcessToken = Readonly<{ runId: string; spawnId: string; role: TransactionProcessRole }>;
export type TransactionProcessChild = Readonly<{ pid?: number; exitCode: number | null; signalCode: string | null }>;
export type TransactionProcessCheck = Readonly<{ token: TransactionProcessToken; initial: TransactionProcessObservation | null; current: TransactionProcessObservation | null; exited: boolean; closed: boolean; error: string | null; decision: TransactionProcessDecision }>;
export type TransactionProcessObserver = Readonly<{ self: TransactionProcessObservation; observeFreshChild(child: TransactionProcessChild, role: TransactionProcessRole): TransactionProcessToken; recheck(token: TransactionProcessToken): TransactionProcessCheck; pending(): readonly TransactionProcessToken[]; beginStop(): readonly TransactionProcessToken[]; close(): void }>;
type TransactionProcessReader = { self: TransactionProcessObservation; read(pid: number): TransactionProcessObservation; close(): void };
type TransactionProcessEntry = { token: TransactionProcessToken; child: ChildProcess; initial: TransactionProcessObservation | null; exited: boolean; closed: boolean; failedSpawn: boolean; error: string | null };
type TransactionNativeFunction = (...args: (number | bigint | null)[]) => number;
type TransactionNativeLibrary = { symbols: Record<string, TransactionNativeFunction>; close(): void };
type TransactionNativeApi = { dlopen(path: string, symbols: Record<string, { args: string[]; returns: string }>): TransactionNativeLibrary; ptr(bytes: Uint8Array): number };

function processRecord<const Keys extends readonly string[]>(value: unknown, names: Keys): value is Record<Keys[number], unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).length === names.length && names.every((name) => Object.hasOwn(value, name));
}

function processInteger(value: unknown, zero = false): value is number {
  return typeof value === "number" && Number.isInteger(value) && value >= (zero ? 0 : 1) && value <= 2147483647;
}

function processDecimal(value: unknown): value is string {
  return typeof value === "string" && /^(?:0|[1-9][0-9]{0,19})$/u.test(value) && BigInt(value) <= 18446744073709551615n;
}

function processBirth(value: unknown): value is TransactionProcessBirth {
  if (!value || typeof value !== "object" || !("platform" in value)) return false;
  if (value.platform === "darwin") return processRecord(value, ["platform", "seconds", "microseconds"]) && processDecimal(value.seconds) && processDecimal(value.microseconds) && BigInt(value.microseconds) <= 999999n;
  if (value.platform === "linux") return processRecord(value, ["platform", "bootId", "startTicks"]) && typeof value.bootId === "string" && /^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/u.test(value.bootId) && processDecimal(value.startTicks);
  return value.platform === "win32" && processRecord(value, ["platform", "filetime"]) && processDecimal(value.filetime);
}

function processObservation(value: unknown): value is TransactionProcessObservation {
  if (!processRecord(value, ["pid", "parentPid", "groupId", "sessionId", "state", "birth"]) || !processInteger(value.pid) || !processInteger(value.parentPid, true) || !processBirth(value.birth) || !["live", "zombie"].includes(String(value.state))) return false;
  if (value.birth.platform === "win32") return value.groupId === null && value.sessionId === null;
  return processInteger(value.groupId) && (value.birth.platform === "darwin" ? value.sessionId === null : processInteger(value.sessionId));
}

function immutableProcessObservation(value: TransactionProcessObservation): TransactionProcessObservation {
  if (!processObservation(value)) throw new Error("Invalid process observation");
  return Object.freeze({ ...value, birth: Object.freeze({ ...value.birth }) });
}

/** 🧮️ Compares independently produced native layouts before any native record is read. */
export function certifyTransactionProcessLayout(c: unknown, rust: unknown, platform: string, architecture: string): TransactionProcessLayout {
  if ((platform !== "darwin" && platform !== "win32") || (architecture !== "arm64" && architecture !== "x64")) throw new Error("Unsupported process layout host");
  const offsets: Record<string, number> = platform === "darwin" ? { status: 4, pid: 12, parentPid: 16, groupId: 100, seconds: 120, microseconds: 128 } : { size: 0, pid: 8, parentPid: 32 };
  for (const value of [c, rust]) {
    if (!processRecord(value, ["platform", "architecture", "size", "alignment", "offsets"]) || value.platform !== platform || value.architecture !== architecture || value.size !== (platform === "darwin" ? 136 : 568) || value.alignment !== 8 || !processRecord(value.offsets, Object.keys(offsets))) throw new Error("Unsupported or disagreeing process layout");
    for (const [field, offset] of Object.entries(offsets)) if (value.offsets[field] !== offset) throw new Error("Unsupported or disagreeing process layout offset");
  }
  return Object.freeze({ platform, architecture, size: platform === "darwin" ? 136 : 568, alignment: 8, offsets: Object.freeze(offsets) });
}

/** 🧾️ Decodes only a complete certified Darwin record, retaining both lossless start fields. */
export function decodeDarwinTransactionObservation(bytes: Uint8Array, returnedBytes: number, layout: TransactionProcessLayout): TransactionProcessObservation {
  certifyTransactionProcessLayout(layout, layout, "darwin", layout.architecture);
  if (returnedBytes !== layout.size || bytes.byteLength !== layout.size) throw new Error("Incomplete Darwin process record");
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength), offsets = layout.offsets, status = view.getUint32(offsets.status!, true);
  if (status < 1 || status > 5) throw new Error("Unsupported Darwin process status");
  return immutableProcessObservation({ pid: view.getUint32(offsets.pid!, true), parentPid: view.getUint32(offsets.parentPid!, true), groupId: view.getUint32(offsets.groupId!, true), sessionId: null, state: status === 5 ? "zombie" : "live", birth: { platform: "darwin", seconds: view.getBigUint64(offsets.seconds!, true).toString(), microseconds: view.getBigUint64(offsets.microseconds!, true).toString() } });
}

/** 🐧️ Parses the exact proc stat suffix after its command delimiter, without tick rounding. */
export function parseLinuxTransactionObservation(text: string, bootId: string): TransactionProcessObservation {
  const start = text.indexOf(" ("), end = text.lastIndexOf(") ");
  if (text.length > 8192 || start < 1 || end <= start || !/^[1-9][0-9]*$/u.test(text.slice(0, start))) throw new Error("Malformed Linux process record");
  const fields = text.slice(end + 2).trim().split(/\s+/u);
  if (fields.length < 20 || !/^[RSDZTtXxKWPI]$/u.test(fields[0]!) || !fields.slice(1, 4).every((value) => /^(?:0|[1-9][0-9]*)$/u.test(value)) || !processDecimal(fields[19])) throw new Error("Malformed Linux process fields");
  return immutableProcessObservation({ pid: Number(text.slice(0, start)), parentPid: Number(fields[1]), groupId: Number(fields[2]), sessionId: Number(fields[3]), state: /^[ZXx]$/u.test(fields[0]!) ? "zombie" : "live", birth: { platform: "linux", bootId, startTicks: fields[19]! } });
}

/** 🪟️ Preserves the complete native FILETIME instead of converting it through Date. */
export function decodeWindowsTransactionFiletime(bytes: Uint8Array): string {
  if (bytes.byteLength !== 8) throw new Error("Incomplete Windows creation time");
  return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).getBigUint64(0, true).toString();
}

/** 🪪️ Establishes identity only; an owned leader is not permission to signal unproved group members. */
export function transactionProcessDecision(initial: unknown, current: unknown, exited: boolean): TransactionProcessDecision {
  if (!processObservation(initial) || typeof exited !== "boolean") return { kind: "reject", reason: "invalid-initial-identity" };
  if (exited) return { kind: "terminal", reason: "owned-handle-exit" };
  if (!processObservation(current) || current.state !== "live") return { kind: "reject", reason: "unverifiable-current-identity" };
  if (initial.pid !== current.pid || initial.parentPid !== current.parentPid || initial.groupId !== current.groupId || initial.sessionId !== current.sessionId || initial.birth.platform !== current.birth.platform) return { kind: "reject", reason: "ownership-changed" };
  const left = initial.birth as unknown as Record<string, string>, right = current.birth as unknown as Record<string, string>;
  if (!Object.keys(left).every((key) => left[key] === right[key])) return { kind: "reject", reason: "creation-token-changed" };
  return { kind: current.birth.platform === "win32" ? "owned-handle" : current.groupId === current.pid ? "owned-leader" : "owned-process", reason: "cooperative-exact-identity" };
}

function readProcessText(path: string, limit: number): string {
  const fd = openSync(path, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0)), bytes = Buffer.alloc(limit + 1);
  try {
    let length = 0;
    while (length < bytes.length) { const count = readSync(fd, bytes, length, bytes.length - length, null); if (!count) break; length += count; }
    if (length > limit) throw new Error("Process record exceeds bound");
    return new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(0, length));
  } finally { closeSync(fd); }
}

function createTransactionProcessObserver(runId: string, reader: TransactionProcessReader): TransactionProcessObserver {
  if (!/^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/u.test(runId)) throw new Error("Invalid transaction run identity");
  const self = immutableProcessObservation(reader.self), entries = new Map<TransactionProcessToken, TransactionProcessEntry>(), children = new WeakSet<ChildProcess>();
  let stopped = false, closed = false;
  const terminal = (entry: TransactionProcessEntry): boolean => entry.exited || entry.child.exitCode !== null || entry.child.signalCode !== null || (entry.failedSpawn && entry.closed);
  const pending = (): readonly TransactionProcessToken[] => [...entries.values()].filter((entry) => !terminal(entry) || !entry.closed).map((entry) => entry.token);
  const checkOpen = (): void => { if (closed) throw new Error("Process observer is closed"); };
  return Object.freeze({
    self,
    observeFreshChild(child: TransactionProcessChild, role: TransactionProcessRole): TransactionProcessToken {
      checkOpen();
      if (stopped || !(child instanceof ChildProcess) || children.has(child) || !["phase-child", "mixed-generator-child", "failure-child", "lease-contender", "shard", "probe-subject", "probe-decoy"].includes(role)) throw new Error("Child admission rejected");
      const token = Object.freeze({ runId, spawnId: randomUUID(), role }), entry: TransactionProcessEntry = { token, child, initial: null, exited: child.exitCode !== null || child.signalCode !== null, closed: false, failedSpawn: false, error: null };
      entries.set(token, entry); children.add(child);
      child.on("error", (error: Error) => { entry.error = String(error).slice(0, 512); entry.failedSpawn = !processInteger(child.pid); });
      child.on("exit", () => { entry.exited = true; });
      child.on("close", () => { entry.closed = true; });
      if (!terminal(entry)) {
        try {
          if (!processInteger(child.pid)) throw new Error("Spawn has no assigned PID");
          const observed = immutableProcessObservation(reader.read(child.pid));
          if (observed.pid !== child.pid || observed.parentPid !== self.pid || observed.birth.platform !== self.birth.platform || observed.state !== "live") throw new Error("Fresh child ownership rejected");
          entry.initial = observed;
        } catch (error) { entry.error = String(error).slice(0, 512); }
      }
      return token;
    },
    recheck(token: TransactionProcessToken): TransactionProcessCheck {
      checkOpen(); const entry = entries.get(token); if (!entry) throw new Error("Foreign process token");
      let exited = terminal(entry), current: TransactionProcessObservation | null = null;
      if (!exited && entry.initial) {
        try { current = immutableProcessObservation(reader.read(entry.initial.pid)); } catch (error) { entry.error = String(error).slice(0, 512); }
      }
      exited = terminal(entry);
      const decision: TransactionProcessDecision = exited ? { kind: "terminal", reason: "owned-handle-exit" } : transactionProcessDecision(entry.initial, current, false);
      return Object.freeze({ token, initial: entry.initial, current, exited, closed: entry.closed, error: entry.error, decision: Object.freeze(decision) });
    },
    pending() { checkOpen(); return pending(); },
    beginStop() { checkOpen(); stopped = true; return pending(); },
    close() { if (closed) return; stopped = true; if (pending().length) throw new Error("Process observer still owns unsettled handles"); reader.close(); closed = true; },
  });
}

async function processNativeApi(): Promise<TransactionNativeApi> {
  const moduleName = "bun:ffi";
  return await import(moduleName) as TransactionNativeApi;
}

async function darwinProcessReader(layout: TransactionProcessLayout): Promise<TransactionProcessReader> {
  const ffi = await processNativeApi(), library = ffi.dlopen("/usr/lib/libSystem.B.dylib", { proc_pidinfo: { args: ["i32", "i32", "u64", "ptr", "i32"], returns: "i32" } }), bytes = new Uint8Array(layout.size);
  const read = (pid: number): TransactionProcessObservation => {
    bytes.fill(0); const count = library.symbols.proc_pidinfo!(pid, 3, 0n, ffi.ptr(bytes), layout.size), observation = decodeDarwinTransactionObservation(bytes, count, layout);
    if (observation.pid !== pid) throw new Error("Darwin process response PID mismatch");
    return observation;
  };
  try { return { self: read(process.pid), read, close: () => library.close() }; } catch (error) { library.close(); throw error; }
}

function linuxProcessReader(): TransactionProcessReader {
  const bootId = readProcessText("/proc/sys/kernel/random/boot_id", 64).trim();
  const read = (pid: number): TransactionProcessObservation => {
    const observation = parseLinuxTransactionObservation(readProcessText(`/proc/${pid}/stat`, 8192), bootId);
    if (observation.pid !== pid) throw new Error("Linux process response PID mismatch");
    return observation;
  };
  return { self: read(process.pid), read, close() {} };
}

async function windowsProcessReader(layout: TransactionProcessLayout): Promise<TransactionProcessReader> {
  const ffi = await processNativeApi(), library = ffi.dlopen("kernel32.dll", {
    OpenProcess: { args: ["u32", "i32", "u32"], returns: "ptr" }, GetProcessTimes: { args: ["ptr", "ptr", "ptr", "ptr", "ptr"], returns: "i32" }, WaitForSingleObject: { args: ["ptr", "u32"], returns: "u32" }, CloseHandle: { args: ["ptr"], returns: "i32" },
    CreateToolhelp32Snapshot: { args: ["u32", "u32"], returns: "ptr" }, Process32FirstW: { args: ["ptr", "ptr"], returns: "i32" }, Process32NextW: { args: ["ptr", "ptr"], returns: "i32" },
  }), handles = new Map<number, number>();
  const closeHandle = (handle: number): void => { if (!library.symbols.CloseHandle!(handle)) throw new Error("Windows process handle close failed"); };
  const parentPid = (pid: number): number => {
    const snapshot = library.symbols.CreateToolhelp32Snapshot!(2, 0);
    if (!snapshot || snapshot === -1 || snapshot === 18446744073709551615) throw new Error("Windows process snapshot failed");
    const bytes = new Uint8Array(layout.size), view = new DataView(bytes.buffer); view.setUint32(layout.offsets.size!, layout.size, true);
    try {
      let more = library.symbols.Process32FirstW!(snapshot, ffi.ptr(bytes)), count = 0;
      while (more && count++ < 65536) {
        if (view.getUint32(layout.offsets.size!, true) !== layout.size) throw new Error("Windows process entry size changed");
        if (view.getUint32(layout.offsets.pid!, true) === pid) return view.getUint32(layout.offsets.parentPid!, true);
        more = library.symbols.Process32NextW!(snapshot, ffi.ptr(bytes));
      }
      throw new Error("Windows parent observation unavailable");
    } finally { closeHandle(snapshot); }
  };
  const read = (pid: number): TransactionProcessObservation => {
    let handle = handles.get(pid);
    if (!handle) { handle = library.symbols.OpenProcess!(4096 | 1048576, 0, pid); if (!handle) throw new Error("Windows process open failed"); handles.set(pid, handle); }
    if (library.symbols.WaitForSingleObject!(handle, 0) !== 258) throw new Error("Windows process is not currently live");
    const times = new Uint8Array(32);
    if (!library.symbols.GetProcessTimes!(handle, ffi.ptr(times.subarray(0, 8)), ffi.ptr(times.subarray(8, 16)), ffi.ptr(times.subarray(16, 24)), ffi.ptr(times.subarray(24, 32)))) throw new Error("Windows creation time unavailable");
    const parent = parentPid(pid);
    if (library.symbols.WaitForSingleObject!(handle, 0) !== 258) throw new Error("Windows process exited during observation");
    return immutableProcessObservation({ pid, parentPid: parent, groupId: null, sessionId: null, state: "live", birth: { platform: "win32", filetime: decodeWindowsTransactionFiletime(times.subarray(0, 8)) } });
  };
  const close = (): void => { let failure: unknown; for (const handle of handles.values()) try { closeHandle(handle); } catch (error) { failure ??= error; } handles.clear(); library.close(); if (failure) throw failure; };
  try { return { self: read(process.pid), read, close }; } catch (error) { close(); throw error; }
}

/** 🔬️ Opens only the current platform's private reader; native layouts require two exact oracle records. */
export async function openTransactionProcessObserver(runId: string, layoutEvidence?: Readonly<{ c: TransactionProcessLayout; rust: TransactionProcessLayout }>): Promise<TransactionProcessObserver> {
  if (!/^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/u.test(runId)) throw new Error("Invalid transaction run identity");
  let reader: TransactionProcessReader;
  if (process.platform === "linux") { if (layoutEvidence) throw new Error("Linux process observer has no binary layout"); reader = linuxProcessReader(); }
  else if (process.platform === "darwin" || process.platform === "win32") {
    if (!layoutEvidence || !processRecord(layoutEvidence, ["c", "rust"])) throw new Error("Independent native layout evidence required");
    const layout = certifyTransactionProcessLayout(layoutEvidence.c, layoutEvidence.rust, process.platform, process.arch);
    reader = process.platform === "darwin" ? await darwinProcessReader(layout) : await windowsProcessReader(layout);
  } else throw new Error("Unsupported process observer platform");
  try { return createTransactionProcessObserver(runId, reader); } catch (error) { reader.close(); throw error; }
}
