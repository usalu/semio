import { existsSync, mkdirSync, mkdtempSync, realpathSync, rmdirSync, statSync, symlinkSync } from "node:fs";
import { isAbsolute, join, relative, sep } from "node:path";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

export type NativeTemporaryLeaseV1 = Readonly<{ path: string; close(): void }>;
export type NativeTemporaryPlanV1 = Readonly<{ kind: "direct"; root: string }> | Readonly<{ kind: "junction"; roots: readonly string[] }> | Readonly<{ kind: "drive" }>;
export interface NativeTemporaryPortV1 {
  realpath(path: string): string;
  sameDirectory(first: string, second: string): boolean;
  junction(root: string, target: string): NativeTemporaryLeaseV1 | undefined;
  drive(target: string): NativeTemporaryLeaseV1 | undefined;
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
export function withNativeTemporaryDirectoryV1<T>(request: Readonly<{ target: string; candidates: readonly string[]; ticket?: string }>, port: NativeTemporaryPortV1, body: (path: string) => T): T {
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
  try {
    if (!ascii(lease.path) || lease.path.length > policy.maximumRootLength || !port.sameDirectory(lease.path, target)) throw Error("Native temporary alias must name the exact bounded physical target");
    return body(lease.path);
  } finally { lease.close(); }
}

/** 🗝️ Uses native directory junctions and subst without copying or deleting the physical target. */
export function nativeTemporarySystemPortV1(runDosDevice: (args: string[]) => number): NativeTemporaryPortV1 {
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
    drive(target) {
      for (let code = 90; code >= 68; code--) {
        const device = String.fromCharCode(code) + ":", alias = device + "\\";
        if (existsSync(alias) || runDosDevice([device, target]) !== 0) continue;
        return { path: alias, close() {
          if (!sameDirectory(alias, target)) throw Error("Native temporary DOS-device ownership changed");
          if (runDosDevice([device, "/d"]) !== 0) throw Error("Native temporary DOS-device cleanup failed");
        } };
      }
      return undefined;
    },
  };
}
