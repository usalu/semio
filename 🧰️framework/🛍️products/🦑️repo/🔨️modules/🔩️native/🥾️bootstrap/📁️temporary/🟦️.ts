import { mkdirSync, mkdtempSync, realpathSync, rmdirSync, statSync, symlinkSync } from "node:fs";
import { isAbsolute, join, relative, sep } from "node:path";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { runtimeNativeSystemV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";

export type NativeTemporaryLeaseV1 = Readonly<{ path: string; close(): void }>;
export type NativeTemporaryPlanV1 = Readonly<{ kind: "direct"; root: string }> | Readonly<{ kind: "junction"; roots: readonly string[] }> | Readonly<{ kind: "drive" }>;
export interface NativeTemporaryPortV1 {
  realpath(path: string): string;
  sameDirectory(first: string, second: string): boolean;
  junction(root: string, target: string): NativeTemporaryLeaseV1 | undefined;
  drive(target: string): NativeTemporaryLeaseV1 | undefined;
}

/** 🪟️ Acquires an exclusive cooperative Windows DOS-device lease without replacing existing mappings. */
export function nativeTemporaryWindowsDosDeviceV1(devices: readonly string[] = Array.from({ length: 23 }, (_, index) => String.fromCharCode(90 - index) + ":")): (target: string) => NativeTemporaryLeaseV1 | undefined {
  return target => {
    if (process.platform !== "win32" || !isAbsolute(target)) return undefined;
    const { dlopen, FFIType: f } = runtimeNativeSystemV1();
    const library = dlopen("kernel32.dll", {
      CreateMutexW: { args: [f.ptr, f.i32, f.ptr], returns: f.ptr },
      WaitForSingleObject: { args: [f.ptr, f.u32], returns: f.u32 },
      ReleaseMutex: { args: [f.ptr], returns: f.i32 },
      CloseHandle: { args: [f.ptr], returns: f.i32 },
      QueryDosDeviceW: { args: [f.ptr, f.ptr, f.u32], returns: f.u32 },
      DefineDosDeviceW: { args: [f.u32, f.ptr, f.ptr], returns: f.i32 },
      GetLastError: { args: [], returns: f.u32 },
    });
    const api = library.symbols, wide = (value: string) => Buffer.from(value + "\0", "utf16le");
    const canonical = target.replaceAll("/", "\\"), ntTarget = canonical.startsWith("\\\\") ? "\\??\\UNC\\" + canonical.slice(2) : "\\??\\" + canonical;
    for (const device of devices) {
      if (!/^[D-Z]:$/u.test(device)) continue;
      const name = wide(device), mutex = api.CreateMutexW(null, 0, wide("Local\\Semio.NativeTemporary." + device[0]));
      if (!mutex) continue;
      const wait = Number(api.WaitForSingleObject(mutex, 0));
      if (wait !== 0 && wait !== 128) { api.CloseHandle(mutex); continue; }
      const release = () => { api.ReleaseMutex(mutex); api.CloseHandle(mutex); };
      const buffer = Buffer.alloc(65536), length = Number(api.QueryDosDeviceW(name, buffer, buffer.length / 2));
      if (length !== 0 || Number(api.GetLastError()) !== 2 || Number(api.DefineDosDeviceW(1, name, wide(ntTarget))) !== 1) { release(); continue; }
      const alias = device + "\\";
      return { path: alias, close() {
        try {
          const current = Number(api.QueryDosDeviceW(name, buffer, buffer.length / 2));
          if (!current || buffer.toString("utf16le").split("\0", 1)[0] !== ntTarget) throw Error("Native temporary DOS-device ownership changed");
          if (Number(api.DefineDosDeviceW(7, name, wide(ntTarget))) !== 1) throw Error("Native temporary DOS-device cleanup failed");
        } finally { release(); library.close(); }
      } };
    }
    library.close();
    return undefined;
  };
}

const policy = schema.definitions.TemporaryPolicy.const;
const ascii = (path: string): boolean => /^[\x00-\x7f]+$/u.test(path);
const inside = (root: string, path: string): boolean => {
  const suffix = relative(root, path);
  return suffix === "" || (!isAbsolute(suffix) && suffix !== ".." && !suffix.startsWith(".." + sep));
};

/** 🪄️ Chooses existing ASCII paths, owned junctions, or an owned user DOS-device alias. */
export function nativeTemporaryPlanV1(target: string, candidates: readonly string[]): NativeTemporaryPlanV1 {
  if (!target || target.length > policy.maximumRootLength) throw Error("Native temporary targets need 1–128 UTF-16 characters to reserve room within 256-character paths");
  if (ascii(target)) return { kind: "direct", root: target };
  const roots = candidates.filter(root => ascii(root) && root.length + policy.junctionSuffixLength <= policy.maximumRootLength);
  return roots.length ? { kind: "junction", roots } : { kind: "drive" };
}

/** 🧵️ Retains an ASCII alias of the exact ticket-owned physical directory until its installer exits. */
function acquireNativeTemporaryDirectoryV1(request: Readonly<{ target: string; candidates: readonly string[]; ticket?: string; signal?: AbortSignal }>, port: NativeTemporaryPortV1): NativeTemporaryLeaseV1 {
  request.signal?.throwIfAborted();
  let target = port.realpath(request.target);
  if (request.ticket) {
    const ticket = port.realpath(request.ticket), generated = port.realpath(join(ticket, "🗑️generated"));
    if (!inside(ticket, generated)) throw Error("Native temporary files must remain physically inside the ticket generated directory");
    if (port.sameDirectory(request.target, generated)) target = generated;
    else if (!inside(generated, target)) throw Error("Native temporary files must remain physically inside the ticket generated directory");
  }
  if (target.length > policy.maximumRootLength) throw Error("Native physical temporary targets must reserve 128 characters within the 256-character path limit");
  const plan = nativeTemporaryPlanV1(request.target, request.candidates);
  const lease = plan.kind === "direct" ? { path: plan.root, close() {} } : (plan.kind === "junction" ? plan.roots.map(root => () => port.junction(root, target)).reduce<NativeTemporaryLeaseV1 | undefined>((held, open) => held ?? open(), undefined) : undefined) ?? port.drive(target);
  if (!lease) throw Error("No owned ASCII Windows temporary alias is available");
  try { if (!ascii(lease.path) || lease.path.length > policy.maximumRootLength || !port.sameDirectory(lease.path, target)) throw Error("Native temporary alias must name the exact bounded physical target"); }
  catch (error) { lease.close(); throw error; }
  return lease;
}

/** 🧵️ Retains an ASCII alias through a synchronous operation. */
export function withNativeTemporaryDirectoryV1<T>(request: Readonly<{ target: string; candidates: readonly string[]; ticket?: string; signal?: AbortSignal }>, port: NativeTemporaryPortV1, body: (path: string) => T): T {
  const lease = acquireNativeTemporaryDirectoryV1(request, port);
  try { return body(lease.path); } finally { lease.close(); }
}

/** ⏳️ Retains an ASCII alias until asynchronous work and its children settle. */
export async function withNativeTemporaryDirectoryAsyncV1<T>(request: Readonly<{ target: string; candidates: readonly string[]; ticket?: string; signal?: AbortSignal }>, port: NativeTemporaryPortV1, body: (path: string) => Promise<T>): Promise<T> {
  const lease = acquireNativeTemporaryDirectoryV1(request, port);
  try { return await body(lease.path); } finally { lease.close(); }
}

/** 🗝️ Uses native directory junctions and a supplied DOS-device lease without copying or deleting the physical target. */
export function nativeTemporarySystemPortV1(drive: (target: string) => NativeTemporaryLeaseV1 | undefined): NativeTemporaryPortV1 {
  const sameDirectory = (first: string, second: string): boolean => {
    const a = statSync(first, { bigint: true }), b = statSync(second, { bigint: true });
    return a.isDirectory() && b.isDirectory() && a.ino !== 0n && a.dev === b.dev && a.ino === b.ino;
  };
  return {
    realpath: realpathSync,
    sameDirectory,
    junction(root, target) {
      let owner: string;
      try { mkdirSync(root, { recursive: true }); owner = mkdtempSync(join(root, "nt-")); }
      catch { return undefined; }
      const alias = join(owner, "t");
      try { symlinkSync(target, alias, "junction"); }
      catch { rmdirSync(owner); return undefined; }
      return { path: alias, close() {
        if (!sameDirectory(alias, target)) throw Error("Native temporary junction ownership changed");
        rmdirSync(alias);
        rmdirSync(owner);
      } };
    },
    drive,
  };
}
