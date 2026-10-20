import { dirname, resolve } from "node:path";

/** 🎛️ Resource lease admission control: the access descriptor, the awaited wait progress, the caller control and the materialized path bound every lease owner shares; a leaf, so leases, retirement and routing never import each other for these. */
export type ResourceAccess = { readonly resource: string; readonly mode: "shared" | "exclusive" };
export type LeaseWait = ResourceAccess & { readonly elapsedMs: number };
/** 🎛️ Preserves an admitted caller's existing signal, deadline and awaited wait transitions. */
export interface ResourceLeaseControl {
  readonly signal: AbortSignal;
  remainingMilliseconds(): number | null;
  publish(progress: LeaseWait): void | Promise<void>;
  yieldContinuation(): Promise<void>;
}

/** ⏳️ Refuses expired caller authority without replacing its cancellation signal. */
export function checkResourceLeaseControl(control: ResourceLeaseControl): number | null {
  if (!control || !(control.signal instanceof AbortSignal) || typeof control.remainingMilliseconds !== "function" || typeof control.publish !== "function" || typeof control.yieldContinuation !== "function") throw Error("Complete resource admission control required");
  control.signal.throwIfAborted();
  const remaining = control.remainingMilliseconds();
  if (remaining !== null && (!Number.isFinite(remaining) || remaining <= 0)) throw Error("Original compiler admission deadline exhausted");
  return remaining;
}

/** 📏️ Checks the complete resolved materialized path in both portable string measures. */
export function requireMaterializedPathV1(path: string): string {
  const absolute = resolve(path);
  if (absolute.includes("\0") || absolute.length > 256 || [...absolute].length > 256) throw Error("Materialized absolute path exceeds 256 characters");
  return absolute;
}

/** 🔗️ Whether an ancestor symlink belongs to the operating system rather than to the store: a link directly below the filesystem root that only root could create (macOS `/var`, `/tmp`, `/etc`), or on Windows any link directly below the drive root. Such a link cannot be substituted by the store's users, so the strict no-symlink rule stands for every other ancestor. */
export function isSystemLeaseLinkV1(current: string, stat: Readonly<{ uid: number }>): boolean {
  const parent = dirname(current);
  if (process.platform === "win32") return dirname(parent) === parent;
  return stat.uid === 0 && (process.getuid?.() !== 0 || dirname(parent) === parent);
}
